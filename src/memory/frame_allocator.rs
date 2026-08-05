use limine::memmap::{Entry, MEMMAP_USABLE};
use x86_64::{
    PhysAddr,
    structures::paging::{FrameAllocator, PhysFrame, Size4KiB},
};

use crate::memory::memmap;

pub struct PhysFrameAllocator {
    entries: &'static [&'static Entry],
    current_entry: usize,
    current_addr: u64,
}

unsafe impl FrameAllocator<Size4KiB> for PhysFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        loop {
            let entry = self.entries.get(self.current_entry)?;
            if entry.type_ != MEMMAP_USABLE {
                self.current_entry += 1;
                continue;
            }

            if self.current_addr == 0 {
                self.current_addr = entry.base;
            }

            if self.current_addr < entry.base + entry.length {
                let addr = self.current_addr;
                self.current_addr += 4096;

                return Some(PhysFrame::containing_address(PhysAddr::new(addr)));
            }

            self.current_entry += 1;
            self.current_addr = 0;
        }
    }
}

pub fn init() -> PhysFrameAllocator {
    PhysFrameAllocator {
        entries: memmap::entries(),
        current_entry: 0,
        current_addr: 0,
    }
}

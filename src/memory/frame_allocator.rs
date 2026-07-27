use limine::{
    memmap::{Entry, MEMMAP_USABLE},
    request::MemmapRequest,
};
use x86_64::{
    PhysAddr,
    structures::paging::{FrameAllocator, PhysFrame, Size4KiB},
};

static MEMORY_MAP_REQUEST: MemmapRequest = MemmapRequest::new();

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
    let memory_map_response = MEMORY_MAP_REQUEST
        .response()
        .expect("Failed to receive memory map response from bootloader");

    PhysFrameAllocator {
        entries: memory_map_response.entries(),
        current_entry: 0,
        current_addr: 0,
    }
}

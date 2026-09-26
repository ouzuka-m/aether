//! Physical frame allocator.
//!
//! Implements a simple bump allocator that walks the bootloader-provided
//! memory map and hands out 4 KiB physical frames from usable regions.

use limine::memmap::{Entry, MEMMAP_USABLE};
use x86_64::{
    PhysAddr,
    structures::paging::{FrameAllocator, PhysFrame, Size4KiB},
};

use crate::boot::info::ENTRIES;

/// Bump-style physical frame allocator.
///
/// Iterates through the bootloader memory map entries, skipping non-usable
/// regions, and linearly allocates 4 KiB frames from each usable entry.
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

/// Creates a new [`PhysFrameAllocator`] seeded with the bootloader memory map.
pub fn init() -> PhysFrameAllocator {
    crate::info!("Physical frame allocator initialized");

    PhysFrameAllocator {
        entries: *ENTRIES,
        current_entry: 0,
        current_addr: 0,
    }
}

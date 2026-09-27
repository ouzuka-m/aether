//! Physical frame allocator.
//!
//! Implements a simple bump allocator that walks the bootloader-provided
//! memory map and hands out 4 KiB physical frames from usable regions.

use limine::memmap::{Entry, MEMMAP_USABLE};
use x86_64::{
    PhysAddr,
    structures::paging::{FrameAllocator, FrameDeallocator, PhysFrame, Size4KiB},
};

use crate::{boot::info::ENTRIES, memory::address::PhysExt};

const FRAME_SIZE: usize = 4096;

#[derive(Debug)]
pub struct PhysFrameAllocator {
    bitmap: &'static mut [u8],
}

impl PhysFrameAllocator {
    pub fn new(entries: &'static [&'static Entry]) -> Self {
        let (bitmap_addr, bitmap_base, bitmap_length) = {
            let entry = entries
                .iter()
                .find(|e| e.type_ == MEMMAP_USABLE)
                .expect("No usable memory region found in memory map");

            (
                PhysAddr::new(entry.base).to_virt(),
                entry.base,
                entry.length,
            )
        };

        let highest_addr = entries
            .iter()
            .map(|e| e.base + e.length)
            .max()
            .expect("Memory map is empty");
        let total_frames = (highest_addr as usize).div_ceil(FRAME_SIZE);

        let bitmap: &'static mut [u8] = unsafe {
            &mut *core::ptr::slice_from_raw_parts_mut(
                bitmap_addr.as_mut_ptr(),
                total_frames.div_ceil(8),
            )
        };

        bitmap.fill(0xFF);

        for entry in entries {
            if entry.type_ != MEMMAP_USABLE {
                continue;
            }

            let start_frame = entry.base as usize / FRAME_SIZE;
            let end_frame = (entry.base + entry.length) as usize / FRAME_SIZE;

            for frame in start_frame..end_frame {
                let byte = frame / 8;
                let bit = frame % 8;

                bitmap[byte] &= !(1 << bit);
            }
        }

        let start_frame = bitmap_base as usize / FRAME_SIZE;
        let end_frame = (bitmap_base + bitmap_length) as usize / FRAME_SIZE;

        for frame in start_frame..end_frame {
            let byte = frame / 8;
            let bit = frame % 8;

            bitmap[byte] |= 1 << bit;
        }

        Self { bitmap }
    }
}

unsafe impl FrameAllocator<Size4KiB> for PhysFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        for (idx, byte) in self.bitmap.iter_mut().enumerate() {
            if *byte == 0xFF {
                continue;
            }

            let bit = (!*byte).trailing_zeros() as usize;

            *byte |= 1 << bit;

            let frame = idx * 8 + bit;

            return Some(PhysFrame::containing_address(PhysAddr::new(
                (frame * FRAME_SIZE) as u64,
            )));
        }

        None
    }
}

impl FrameDeallocator<Size4KiB> for PhysFrameAllocator {
    unsafe fn deallocate_frame(&mut self, frame: PhysFrame<Size4KiB>) {
        let frame = frame.start_address().as_usize() / FRAME_SIZE;

        let byte = frame / 8;
        let bit = frame % 8;

        self.bitmap[byte] &= !(1 << bit);
    }
}

/// Creates a new [`PhysFrameAllocator`] seeded with the bootloader memory map.
pub fn init() -> PhysFrameAllocator {
    crate::info!("Physical frame allocator initialized");

    PhysFrameAllocator::new(*ENTRIES)
}

//! Global kernel heap allocator.
//!
//! Maps a contiguous range of virtual pages to physical frames and
//! initialises a buddy-system allocator as the `#[global_allocator]`.

use buddy_system_allocator::LockedHeap;
use x86_64::{
    VirtAddr,
    structures::paging::{FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB},
};

#[global_allocator]
static ALLOCATOR: LockedHeap<33> = LockedHeap::empty();

pub const HEAP_START: usize = 0xFFFF_9000_0000_0000;
pub const HEAP_SIZE: usize = 1024 * 1024; // 1 MB, 256 pages

pub fn init(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    let page_range = {
        let start = VirtAddr::new(HEAP_START as u64);

        // Subtract 1 to prevent use of page 257
        let end = start + HEAP_SIZE as u64 - 1u64;

        let start_page: Page<Size4KiB> = Page::containing_address(start);
        let end_page: Page<Size4KiB> = Page::containing_address(end);

        Page::range_inclusive(start_page, end_page)
    };

    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .expect("Failed to allocate frame");
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::NO_EXECUTE;
        // SAFETY: The page is in the dedicated heap virtual range and has
        // not been previously mapped. The frame is freshly allocated and
        // the flags grant read/write access with no-execute.
        unsafe {
            mapper
                .map_to(page, frame, flags, frame_allocator)
                .expect("Failed to map page & frame")
                .flush();
        }
    }

    // SAFETY: The virtual address range [HEAP_START .. HEAP_START + HEAP_SIZE)
    // has just been identity-mapped to valid physical frames above, so it is
    // safe to hand this region to the buddy allocator.
    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }

    crate::info!(
        "Heap allocator initialized at {:#X} ({} KB)",
        HEAP_START,
        HEAP_SIZE / 1024
    );
}

use buddy_system_allocator::LockedHeap;
use x86_64::{
    VirtAddr,
    structures::paging::{FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB},
};

#[global_allocator]
static ALLOCATOR: LockedHeap<33> = LockedHeap::empty();

pub const HEAP_START: usize = 0xFFFF_9000_0000_0000;
pub const HEAP_SIZE: usize = 1000 * 1024; // 1 MB, 250 pages

pub fn init(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    let page_range = {
        let start = VirtAddr::new(HEAP_START as u64);

        // Subtract 1 to prevent use of page 251
        let end = start + HEAP_SIZE as u64 - 1u64;

        let start_page: Page<Size4KiB> = Page::containing_address(start);
        let end_page: Page<Size4KiB> = Page::containing_address(end);

        Page::range_inclusive(start_page, end_page)
    };

    for page in page_range {
        let Some(frame) = frame_allocator.allocate_frame() else {
            panic!("frame allocation failed")
        };
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            mapper
                .map_to(page, frame, flags, frame_allocator)
                .expect("failed to map page & frame")
                .flush();
        }
    }

    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }
}

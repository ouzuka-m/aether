use x86_64::{
    VirtAddr,
    structures::paging::{
        FrameAllocator, FrameDeallocator, PageSize, PhysFrame, Size4KiB,
        frame::PhysFrameRangeInclusive,
    },
};

use crate::memory::{
    address::{PhysExt, VirtExt},
    frame_allocator::FRAME_ALLOCATOR,
};

#[derive(Debug)]
pub struct Stack {
    #[allow(dead_code)]
    pub bottom: VirtAddr,

    pub top: VirtAddr,
}

pub fn alloc_stack() -> Stack {
    let frame = FRAME_ALLOCATOR
        .lock()
        .allocate_frame()
        .expect("Failed to allocate stack frame");

    let bottom = frame.start_address().to_virt();
    let top = bottom + Size4KiB::SIZE;

    Stack { bottom, top }
}

#[allow(dead_code)]
pub fn deallocate_stack(stack: Stack) {
    let (start, end) = (
        PhysFrame::containing_address(stack.bottom.to_phys()),
        PhysFrame::containing_address(stack.bottom.to_phys()),
    );
    let frames: PhysFrameRangeInclusive = PhysFrame::range_inclusive(start, end);

    for frame in frames {
        unsafe { FRAME_ALLOCATOR.lock().deallocate_frame(frame) };
    }
}

use x86_64::{
    VirtAddr,
    structures::paging::{FrameAllocator, FrameDeallocator, PageSize, PhysFrame, Size4KiB},
};

use crate::memory::{address::PhysExt, frame_allocator::FRAME_ALLOCATOR};

#[derive(Debug)]
pub struct Stack {
    pub frame: PhysFrame,
    pub top: VirtAddr,
}

pub fn alloc_stack() -> Stack {
    let frame = FRAME_ALLOCATOR
        .lock()
        .allocate_frame()
        .expect("Failed to allocate stack frame");

    let top = frame.start_address().to_virt() + Size4KiB::SIZE;

    Stack { frame, top }
}

#[allow(dead_code)]
pub fn deallocate_stack(stack: Stack) {
    unsafe { FRAME_ALLOCATOR.lock().deallocate_frame(stack.frame) };
}

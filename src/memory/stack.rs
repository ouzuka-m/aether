use x86_64::{
    VirtAddr,
    structures::paging::{FrameAllocator, FrameDeallocator, PageSize, PhysFrame, Size4KiB},
};

use crate::memory::{address::PhysExt, frame_allocator::FRAME_ALLOCATOR};

#[derive(Debug)]
pub struct Stack {
    frame: PhysFrame,
    top: VirtAddr,
}

impl Stack {
    pub fn alloc() -> Self {
        let frame = FRAME_ALLOCATOR
            .lock()
            .allocate_frame()
            .expect("Failed to allocate stack frame");

        let top = frame.start_address().to_virt() + Size4KiB::SIZE;

        Self { frame, top }
    }

    #[allow(dead_code)]
    pub fn deallocate(self) {
        unsafe { FRAME_ALLOCATOR.lock().deallocate_frame(self.frame()) };
    }

    pub fn frame(&self) -> PhysFrame {
        self.frame
    }

    pub fn top(&self) -> VirtAddr {
        self.top
    }
}

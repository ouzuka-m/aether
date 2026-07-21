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
    next: usize,
}

unsafe impl FrameAllocator<Size4KiB> for PhysFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let mut frames = self
            .entries
            .iter()
            .filter(|entry| entry.type_ == MEMMAP_USABLE)
            .flat_map(|entry| (entry.base..(entry.base + entry.length)).step_by(4096))
            .map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)));

        let frame = frames.nth(self.next);
        self.next += 1;

        frame
    }
}

pub fn init() -> PhysFrameAllocator {
    let Some(response) = MEMORY_MAP_REQUEST.response() else {
        panic!("No memory map response");
    };

    PhysFrameAllocator {
        entries: response.entries(),
        next: 0,
    }
}

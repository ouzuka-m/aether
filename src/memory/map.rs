use limine::{memmap::Entry, request::MemmapRequest};

static MEMORY_MAP_REQUEST: MemmapRequest = MemmapRequest::new();

pub fn usable() -> usize {
    let mut total = 0usize;

    for entry in entries() {
        total += entry.length as usize;
    }

    total / 1024 / 1024
}

pub fn entries() -> &'static [&'static Entry] {
    MEMORY_MAP_REQUEST
        .response()
        .expect("Failed to receive memory map response from bootloader")
        .entries()
}

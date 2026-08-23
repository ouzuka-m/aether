use limine::{
    memmap::Entry,
    request::{ModulesRespData, Response},
};
use spin::{lazylock::LazyLock, mutex::Mutex};
use x86_64::VirtAddr;

use crate::{boot::requests, display::framebuffer::FrameBuffer};

/// Virtual base address offset of the Higher-Half Direct Mapping (HHDM).
///
/// The bootloader maps all physical memory directly at this virtual address offset.
///
/// # Panics
/// Panics if the bootloader fails to return a valid HHDM response.
pub static HHDM: LazyLock<VirtAddr> = LazyLock::new(|| {
    let hhdm_response = requests::HHDM_REQUEST
        .response()
        .expect("Failed to receive HHDM response from bootloader");

    VirtAddr::new(hhdm_response.offset)
});

pub static ENTRIES: LazyLock<&'static [&'static Entry]> = LazyLock::new(|| {
    requests::MEMMAP_REQUEST
        .response()
        .expect("Failed to receive memory map response from bootloader")
        .entries()
});

pub static RSDP: LazyLock<VirtAddr> = LazyLock::new(|| {
    let rsdp_address = requests::RSDP_REQUEST
        .response()
        .expect("Failed to receive RSDP response from bootloader")
        .address;

    VirtAddr::new(rsdp_address as u64)
});

pub static FRAMEBUFFER: LazyLock<Mutex<FrameBuffer>> = LazyLock::new(|| {
    let framebuffers = requests::FRAMEBUFFER_REQUEST
        .response()
        .expect("Failed to receive framebuffers from bootloader")
        .framebuffers();

    let framebuffer = framebuffers
        .first()
        .expect("No framebuffer provided by bootloader");

    Mutex::new(FrameBuffer::new(
        framebuffer.address() as *mut u32,
        framebuffer.width as usize,
        framebuffer.height as usize,
        (framebuffer.pitch / 4) as usize,
    ))
});

pub static MODULES: LazyLock<&'static Response<ModulesRespData>> = LazyLock::new(|| {
    requests::MODULES_REQUEST
        .response()
        .expect("Failed to get modules response from bootloader")
});

pub static CMDLINE: LazyLock<&'static str> =
    LazyLock::new(|| match requests::CMDLINE_REQUEST.response() {
        Some(response) => response.cmdline(),
        None => "",
    });

pub mod cell;
pub mod font;
pub mod framebuffer;
pub mod macros;

use limine::request::FramebufferRequest;
use spin::{mutex::Mutex, once::Once};

use crate::display::framebuffer::FrameBuffer;

static FRAMEBUFFER: Once<Mutex<FrameBuffer>> = Once::new();

static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

pub fn init() {
    let framebuffers = FRAMEBUFFER_REQUEST
        .response()
        .expect("Failed to receive framebuffers from bootloader")
        .framebuffers();

    let framebuffer = framebuffers
        .iter()
        .next()
        .expect("No framebuffer provided by bootloader");

    FRAMEBUFFER.call_once(|| {
        Mutex::new(FrameBuffer::new(
            framebuffer.address() as *mut u32,
            framebuffer.width as usize,
            framebuffer.height as usize,
            (framebuffer.pitch / 4) as usize,
        ))
    });
}

pub fn framebuffer() -> &'static Mutex<FrameBuffer> {
    FRAMEBUFFER
        .get()
        .expect("Framebuffer hasn't been initialized")
}

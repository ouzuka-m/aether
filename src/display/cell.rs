//! Character cell tracking for the framebuffer.
//!
//! Each [`Cell`] records the screen position and width of a rendered glyph,
//! enabling the backspace operation to erase the correct pixel region.

/// A rendered character cell in the framebuffer.
///
/// Stores the top-left origin and pixel width of a single glyph so that
/// the display subsystem can locate and clear it on backspace.
#[derive(Debug)]
pub struct Cell {
    start_x: usize,
    start_y: usize,
    width: usize,
}

impl Cell {
    pub fn new(start_x: usize, start_y: usize, width: usize) -> Self {
        Self {
            start_x,
            start_y,
            width,
        }
    }

    pub fn start_x(&self) -> usize {
        self.start_x
    }

    pub fn start_y(&self) -> usize {
        self.start_y
    }

    pub fn width(&self) -> usize {
        self.width
    }
}

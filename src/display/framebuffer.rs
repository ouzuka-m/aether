use core::fmt::{Result, Write};

use alloc::vec::Vec;

use crate::display::{cell::Cell, font};

pub struct FrameBuffer {
    address: *mut u32,
    width: usize,
    height: usize,
    stride: usize,
    cursor_x: usize,
    cursor_y: usize,
    cells: Vec<Cell>,
}

unsafe impl Sync for FrameBuffer {}
unsafe impl Send for FrameBuffer {}

impl FrameBuffer {
    pub fn new(address: *mut u32, width: usize, height: usize, stride: usize) -> Self {
        FrameBuffer {
            address,
            width,
            height,
            stride,
            cursor_x: 0,
            cursor_y: 0,
            cells: Vec::new(),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn newline(&mut self) {
        self.cells.clear();

        self.cursor_x = 0;
        self.cursor_y += font::SIZE.val();
    }

    fn backspace(&mut self) {
        let Some(cell) = self.cells.pop() else {
            return;
        };

        self.cursor_x = cell.start_x();
        self.cursor_y = cell.start_y();

        for y in cell.start_y()..cell.start_y() + font::SIZE.val() {
            for x in cell.start_x()..cell.start_x() + cell.width() {
                if x < self.width && y < self.height {
                    self.put_pixel(x, y, 0x000000);
                }
            }
        }
    }

    fn put_pixel(&self, x: usize, y: usize, color: u32) {
        unsafe { *self.address.add(y * self.stride + x) = color }
    }
}

impl Write for FrameBuffer {
    fn write_str(&mut self, s: &str) -> Result {
        for c in s.chars() {
            self.write_char(c)?;
        }

        Ok(())
    }

    fn write_char(&mut self, c: char) -> Result {
        if c == '\n' {
            self.newline();
            return Ok(());
        }

        if c == '\u{8}' {
            self.backspace();
            return Ok(());
        }

        let Some(glyph) = noto_sans_mono_bitmap::get_raster(c, font::WEIGHT, font::SIZE) else {
            return Ok(());
        };

        let start_x = self.cursor_x;
        let start_y = self.cursor_y;

        self.cells.push(Cell::new(start_x, start_y, glyph.width()));

        let raster = glyph.raster();

        for (row, pixels) in raster.iter().enumerate() {
            for (col, intensity) in pixels.iter().enumerate() {
                if *intensity == 0 {
                    continue;
                }

                let x = self.cursor_x + col;
                let y = self.cursor_y + row;

                if x >= self.width || y >= self.height {
                    continue;
                }

                let value = *intensity as u32;
                let rgba = (value << 16) | (value << 8) | value;

                self.put_pixel(x, y, rgba);
            }
        }

        self.cursor_x += glyph.width();

        if self.cursor_x + glyph.width() >= self.width {
            self.newline();
        }

        Ok(())
    }
}

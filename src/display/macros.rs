use core::fmt::{Arguments, Write};

use crate::display::{self};

pub fn _print(args: Arguments) {
    let framebuffer = display::framebuffer();

    if let Some(mut framebuffer) = framebuffer.try_lock() {
        let _ = framebuffer.write_fmt(args);
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::display::macros::_print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*))
    };
}

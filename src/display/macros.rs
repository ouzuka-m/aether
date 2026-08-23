use core::fmt::{Arguments, Write};

use crate::boot::info::FRAMEBUFFER;

pub fn _print(args: Arguments) {
    if let Some(mut framebuffer) = FRAMEBUFFER.try_lock() {
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

//! Display output macros.
//!
//! Provides [`print!`], [`println!`], and [`prompt!`] macros that write
//! formatted text to the global framebuffer.

use core::fmt::{Arguments, Write};

use crate::boot::info::FRAMEBUFFER;

pub fn _print(args: Arguments) {
    if let Some(mut framebuffer) = FRAMEBUFFER.try_lock() {
        let _ = framebuffer.write_fmt(args);
    }
}

#[macro_export]
macro_rules! print {
    ($val:ident) => {
        $crate::display::macros::_print(format_args!("{}", $val));
    };
    ($($arg:tt)*) => {
        $crate::display::macros::_print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! println {
    ($val:ident) => {
        $crate::print!("{}\n", $val);
    };
    ($($arg:tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! prompt {
    () => {
        $crate::print!("$ ");
    };
}

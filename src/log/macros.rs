use core::fmt::{Arguments, Write};
use x86_64::instructions::interrupts;

use crate::log::{
    level::{LOG_LEVEL, Level},
    uart::SERIAL,
};

/// Internal logging function used by the logging macros.
pub fn _log(level: Level, args: Arguments) {
    if level < LOG_LEVEL {
        return;
    }

    let timestamp = unsafe { core::arch::x86_64::_rdtsc() };
    let args = format_args!("[{}] [{}] {}\n", timestamp, level.as_str(), args);

    interrupts::without_interrupts(|| {
        if let Some(mut serial) = SERIAL.try_lock() {
            let _ = serial.write_fmt(args);
        }
    });
}

/// Logs a message at the debug level.
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::log::macros::_log($crate::log::level::Level::Debug, format_args!($($arg)*))
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {};
}

/// Logs a message at the info level.
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::log::macros::_log($crate::log::level::Level::Info, format_args!($($arg)*))
    };
}

/// Logs a message at the warn level.
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::log::macros::_log($crate::log::level::Level::Warn, format_args!($($arg)*))
    };
}

/// Logs a message at the error level.
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::log::macros::_log($crate::log::level::Level::Error, format_args!($($arg)*))
    };
}

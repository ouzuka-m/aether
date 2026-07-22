use core::fmt::Write;
use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::{Config, Uart16550Tty, backend::PioBackend};
use x86_64::instructions::interrupts;

lazy_static! {
    pub static ref SERIAL: Mutex<Uart16550Tty<PioBackend>> = Mutex::new(unsafe {
        Uart16550Tty::new_port(0x3F8, Config::default()).expect("failed to initialize UART")
    });
}

pub fn _print(args: ::core::fmt::Arguments) {
    interrupts::without_interrupts(|| {
        if let Some(mut serial) = SERIAL.try_lock() {
            let _ = serial.write_fmt(args);
        }
    });
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! println {
    () => (
        $crate::print!("\n");
    );
    ($fmt:expr) => (
        $crate::print!(concat!($fmt, "\n"));
    );
    ($fmt:expr, $($arg:tt)*) => (
        $crate::print!(
            concat!($fmt, "\n"),
            $($arg)*
        );
    );
}

use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::{Config, Uart16550Tty, backend::PioBackend};

lazy_static! {
    pub static ref SERIAL: Mutex<Uart16550Tty<PioBackend>> = Mutex::new(unsafe {
        Uart16550Tty::new_port(0x3F8, Config::default()).expect("Failed to initialize UART 16550")
    });
}

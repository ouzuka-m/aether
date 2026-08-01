use spin::{Mutex, lazylock::LazyLock};
use uart_16550::{Config, Uart16550Tty, backend::PioBackend};

pub static SERIAL: LazyLock<Mutex<Uart16550Tty<PioBackend>>> = LazyLock::new(|| unsafe {
    Mutex::new(
        Uart16550Tty::new_port(0x3F8, Config::default()).expect("Failed to initialize UART 16550"),
    )
});

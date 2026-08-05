use spin::{Mutex, lazylock::LazyLock};
use uart_16550::{Config, Uart16550Tty, backend::PioBackend};

const PORT: u16 = 0x3F8;

pub static SERIAL: LazyLock<Mutex<Uart16550Tty<PioBackend>>> = LazyLock::new(|| unsafe {
    Mutex::new(
        Uart16550Tty::new_port(PORT, Config::default()).expect("Failed to initialize UART 16550"),
    )
});

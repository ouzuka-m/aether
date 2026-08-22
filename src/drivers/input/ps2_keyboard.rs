use pc_keyboard::{DecodedKey, HandleControl, PS2Keyboard, ScancodeSet1, layouts::Us104Key};
use spin::{lazylock::LazyLock, mutex::Mutex};
use x86_64::instructions::port::Port;

/// Thread-safe PS/2 keyboard driver instance.
static KEYBOARD: LazyLock<Mutex<PS2Keyboard<Us104Key, ScancodeSet1>>> = LazyLock::new(|| {
    Mutex::new(PS2Keyboard::new(
        ScancodeSet1::new(),
        Us104Key,
        HandleControl::Ignore,
    ))
});

pub fn read() -> u8 {
    unsafe { Port::new(0x60).read() }
}

pub fn decode(scancode: u8) -> Option<DecodedKey> {
    let mut keyboard = KEYBOARD.lock();
    if let Ok(Some(event)) = keyboard.add_byte(scancode) {
        return keyboard.process_keyevent(event);
    }

    None
}

pub fn clear_buffer() {
    let mut status_port: Port<u8> = Port::new(0x64);
    let mut data_port: Port<u8> = Port::new(0x60);

    unsafe {
        while status_port.read() & 0x1 != 0 {
            data_port.read();
        }
    }
}

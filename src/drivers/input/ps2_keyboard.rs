//! PS/2 keyboard driver.
//!
//! Reads raw scancodes from the PS/2 data port (`0x60`), decodes them
//! into key events using the `pc-keyboard` crate, and provides a
//! helper to drain stale scancodes from the controller buffer.

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

/// Reads a single raw scancode byte from the PS/2 data port.
pub fn read() -> u8 {
    // SAFETY: Port 0x60 is the standard PS/2 data port. Reading it
    // returns the most recent scancode and has no side effects beyond
    // consuming the byte from the controller's output buffer.
    unsafe { Port::new(0x60).read() }
}

/// Attempts to decode a raw scancode into a [`DecodedKey`].
pub fn decode(scancode: u8) -> Option<DecodedKey> {
    let mut keyboard = KEYBOARD.lock();
    if let Ok(Some(event)) = keyboard.add_byte(scancode) {
        return keyboard.process_keyevent(event);
    }

    None
}

/// Drains any pending bytes from the PS/2 controller output buffer.
///
/// Reads and discards data from port `0x60` until the Output Buffer
/// Status bit (bit 0) in the status register (port `0x64`) is clear.
pub fn clear_buffer() {
    let mut status_port: Port<u8> = Port::new(0x64);
    let mut data_port: Port<u8> = Port::new(0x60);

    // SAFETY: Ports 0x64 (status) and 0x60 (data) are standard PS/2
    // controller ports. Reading them is non-destructive apart from
    // consuming buffered scancodes, which is the intended effect.
    unsafe {
        while status_port.read() & 0x1 != 0 {
            data_port.read();
        }
    }
}

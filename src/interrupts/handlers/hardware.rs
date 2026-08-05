//! Hardware interrupt handlers (IRQs).
//!
//! This module contains interrupt service routines (ISRs) for handling
//! hardware-level interrupts delivered via the APIC, such as spurious
//! interrupts, APIC timer ticks, and PS/2 keyboard inputs.

use core::sync::atomic::{AtomicU64, Ordering};

use pc_keyboard::{DecodedKey, HandleControl, PS2Keyboard, ScancodeSet1, layouts::Us104Key};
use spin::{lazylock::LazyLock, mutex::Mutex};
use x86_64::{instructions::port::Port, structures::idt::InterruptStackFrame};

use crate::{
    debug,
    drivers::{lapic, tsc_deadline},
    warn,
};

/// Interrupt vector index for Spurious Vector Interrupts (SVR).
pub const SVR_VECTOR: u8 = 0xFF; // 255

/// Interrupt vector index for APIC Timer interrupts.
pub const TIMER_VECTOR: u8 = 0x20; // 32

/// Interrupt vector index for PS/2 Keyboard interrupts.
pub const KEYBOARD_VECTOR: u8 = 0x21; // 33

/// Thread-safe PS/2 keyboard driver instance.
static KEYBOARD: LazyLock<Mutex<PS2Keyboard<Us104Key, ScancodeSet1>>> = LazyLock::new(|| {
    Mutex::new(PS2Keyboard::new(
        ScancodeSet1::new(),
        Us104Key,
        HandleControl::Ignore,
    ))
});

static TICK: AtomicU64 = AtomicU64::new(0);

/// Spurious vector interrupt handler (Vector 255 / 0xFF).
///
/// Spurious interrupts occur when an interrupt signal is dropped before the APIC
/// can process it. According to the Intel APIC specification, spurious interrupts
/// do not require an End of Interrupt (EOI) signal.
pub extern "x86-interrupt" fn spurious_vector_interrupt(_: InterruptStackFrame) {
    warn!("Spurious vector interrupt (SVR) triggered");
}

/// Timer interrupt handler (Vector 32 / 0x20).
///
/// Triggered periodically by the APIC timer or PIT to drive OS scheduling
/// and timekeeping tasks. Sends an EOI signal to the Local APIC upon completion.
pub extern "x86-interrupt" fn timer(_: InterruptStackFrame) {
    let count = TICK.fetch_add(1, Ordering::Relaxed);
    if count.is_multiple_of(1000) {
        debug!("Heartbeat: {} ticks", count);
    }

    // Fix keyboard sometimes "die" when you spamming keys on startup
    {
        let mut status_port: Port<u8> = Port::new(0x64);
        let mut data_port: Port<u8> = Port::new(0x60);
        unsafe {
            while status_port.read() & 0x1 != 0 {
                let scancode = data_port.read();
                process_scancode(scancode);
            }
        }
    }

    tsc_deadline::arm(1);
    lapic::eoi();
}

/// PS/2 Keyboard interrupt handler (Vector 33 / 0x21).
///
/// Reads raw scancodes from I/O port `0x60`, decodes keypress events using the
/// layout parser, logs the key, and issues an EOI to the Local APIC.
pub extern "x86-interrupt" fn keyboard(_: InterruptStackFrame) {
    let scancode: u8 = unsafe { Port::new(0x60).read() };
    process_scancode(scancode);

    lapic::eoi();
}

fn process_scancode(scancode: u8) {
    let mut keyboard = KEYBOARD.lock();
    if let Ok(Some(event)) = keyboard.add_byte(scancode)
        && let Some(key) = keyboard.process_keyevent(event)
    {
        match key {
            DecodedKey::Unicode(c) => debug!("Keyboard input char: {:?}", c),
            DecodedKey::RawKey(k) => debug!("Keyboard input raw key: {:?}", k),
        }
    }
}

//! Hardware interrupt handlers (IRQs).
//!
//! This module contains interrupt service routines (ISRs) for handling
//! hardware-level interrupts delivered via the APIC, such as spurious
//! interrupts, APIC timer ticks, and PS/2 keyboard inputs.

use core::sync::atomic::{AtomicU64, Ordering};

use pc_keyboard::{DecodedKey, HandleControl, PS2Keyboard, ScancodeSet1, layouts::Us104Key};
use spin::mutex::Mutex;
use x86_64::{instructions::port::Port, structures::idt::InterruptStackFrame};

use crate::{
    acpi::{lapic, lvt::timer},
    debug, warn,
};

/// Interrupt vector index for Spurious Vector Interrupts (SVR).
pub const SVR_IDX: u8 = 0xFF; // 255

/// Interrupt vector index for APIC Timer interrupts.
pub const TIMER_IDX: u8 = 0x20; // 32

/// Interrupt vector index for PS/2 Keyboard interrupts.
pub const KEYBOARD_IDX: u8 = 0x21; // 33

lazy_static::lazy_static! {
    /// Thread-safe PS/2 keyboard driver instance.
    static ref KEYBOARD: Mutex<PS2Keyboard<Us104Key, ScancodeSet1>> = {
        Mutex::new(PS2Keyboard::new(ScancodeSet1::new(), Us104Key, HandleControl::Ignore))
    };
}

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

    timer::arm_tsc_deadline(1);
    lapic::eoi();
}

/// PS/2 Keyboard interrupt handler (Vector 33 / 0x21).
///
/// Reads raw scancodes from I/O port `0x60`, decodes keypress events using the
/// layout parser, logs the key, and issues an EOI to the Local APIC.
pub extern "x86-interrupt" fn keyboard(_: InterruptStackFrame) {
    let scancode: u8 = unsafe { Port::new(0x60).read() };
    debug!("Keyboard scancode received: {:#04x}", scancode);

    let mut keyboard = KEYBOARD.lock();

    if let Ok(Some(event)) = keyboard.add_byte(scancode)
        && let Some(key) = keyboard.process_keyevent(event)
    {
        match key {
            DecodedKey::Unicode(c) => {
                debug!("Keyboard input char: {:?}", c);
            }
            DecodedKey::RawKey(k) => {
                debug!("Keyboard input raw key: {:?}", k);
            }
        }
    }

    lapic::eoi();
}

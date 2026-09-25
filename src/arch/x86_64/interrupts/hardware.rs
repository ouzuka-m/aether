//! Hardware interrupt handlers (IRQs).
//!
//! This module contains interrupt service routines (ISRs) for handling
//! hardware-level interrupts delivered via the APIC, such as spurious
//! interrupts, APIC timer ticks, and PS/2 keyboard inputs.

use core::sync::atomic::{AtomicU64, Ordering};

use alloc::string::String;
use pc_keyboard::DecodedKey;
use spin::mutex::Mutex;
use x86_64::structures::idt::InterruptStackFrame;

use crate::{
    arch::x86_64::idt::READ_COMMAND_VECTOR,
    debug,
    drivers::{apic::lapic, input::ps2_keyboard, tsc_deadline},
    print, warn,
};

pub static INPUT_BUFFER: Mutex<String> = Mutex::new(String::new());

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
    ps2_keyboard::clear_buffer();

    tsc_deadline::arm(1);

    lapic::eoi();
}

/// PS/2 Keyboard interrupt handler (Vector 33 / 0x21).
///
/// Reads raw scancodes from I/O port `0x60`, decodes keypress events using the
/// layout parser, logs the key, and issues an EOI to the Local APIC.
pub extern "x86-interrupt" fn keyboard(_: InterruptStackFrame) {
    let scancode = ps2_keyboard::read();
    if let Some(key) = ps2_keyboard::decode(scancode) {
        match key {
            DecodedKey::Unicode(c) => {
                debug!("Keyboard input char: {:?}", c);

                match c {
                    '\u{8}' => {
                        if INPUT_BUFFER.lock().pop().is_some() {
                            print!(c);
                        }
                    }

                    '\n' => {
                        print!(c);

                        unsafe {
                            core::arch::asm!(
                                "int {vector}",
                                vector = const READ_COMMAND_VECTOR
                            );
                        }

                        INPUT_BUFFER.lock().clear();
                    }

                    _ => {
                        print!(c);
                        INPUT_BUFFER.lock().push(c);
                    }
                }
            }
            DecodedKey::RawKey(k) => debug!("Keyboard input raw key: {:?}", k),
        }
    }

    lapic::eoi();
}

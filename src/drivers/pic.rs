//! Legacy Intel 8259 PIC (Programmable Interrupt Controller) control.
//!
//! Provides routines to disable the legacy 8259 dual PIC chips when transitioning
//! to the modern APIC (Local/IO APIC) interrupt routing model.

use x86_64::instructions::port::Port;

use crate::{debug, info};

/// I/O port address for the Master 8259 PIC interrupt mask register (IMR).
const MASTER_PIC_IMR: u16 = 0x21;

/// I/O port address for the Slave 8259 PIC interrupt mask register (IMR).
const SLAVE_PIC_IMR: u16 = 0xA1;

/// Disables the legacy 8259 PIC by masking all IRQ lines (IRQs 0–15).
///
/// Writes `0xFF` to both the Master PIC IMR (port `0x21`) and Slave PIC IMR (port `0xA1`).
/// This ensures legacy PIC interrupts will not trigger unexpectedly when APIC mode is active.
pub fn disable() {
    debug!(
        "Masking legacy 8259 PIC interrupt lines (ports {:#x} and {:#x})",
        MASTER_PIC_IMR, SLAVE_PIC_IMR
    );

    let mut master: Port<u8> = Port::new(MASTER_PIC_IMR);
    let mut slave: Port<u8> = Port::new(SLAVE_PIC_IMR);

    unsafe {
        master.write(0xFF);
        slave.write(0xFF);
    }

    info!("Legacy 8259 PIC disabled");
}

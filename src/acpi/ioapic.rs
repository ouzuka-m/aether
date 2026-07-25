//! I/O APIC (Input/Output Advanced Programmable Interrupt Controller) driver.
//!
//! Provides functions to query I/O APIC registers, map ISA hardware interrupt request (IRQ)
//! lines to Global System Interrupts (GSIs) via ACPI interrupt source overrides,
//! and program I/O Redirection Table entries (IOREDTBL).

use acpi::platform::interrupt::{InterruptSourceOverride, IoApic};
use core::ptr;
use x86_64::{PhysAddr, VirtAddr};

use crate::{address::ext::PhysExt, debug, info};

/// Register Select offset relative to I/O APIC MMIO base.
const IOREGSEL: u64 = 0x00;
/// I/O Window offset relative to I/O APIC MMIO base.
const IOWIN: u64 = 0x10;

/// Register index for I/O APIC ID.
const IOAPICID: u32 = 0x00;
/// Register index for I/O APIC Version and Max Redirection Entries.
const IOAPICVER: u32 = 0x01;
/// Base register index for I/O Redirection Table entries.
const IOREDTBL_BASE: u32 = 0x10;

/// ISA IRQ line for PIT/APIC timer.
const TIMER_ISA: u8 = 0;
/// IDT vector for timer interrupt (Vector 32 / 0x20).
const TIMER_GSI_INFO: u32 = 0x00000020;

/// ISA IRQ line for PS/2 Keyboard.
const KEYBOARD_ISA: u8 = 1;
/// IDT vector for keyboard interrupt (Vector 33 / 0x21).
const KEYBOARD_GSI_INFO: u32 = 0x00000021;

/// Initializes the I/O APIC by configuring Redirection Table entries.
///
/// Maps legacy ISA interrupts (Timer IRQ 0 and Keyboard IRQ 1) to their
/// respective Global System Interrupt (GSI) pins based on ACPI interrupt source
/// overrides, and routes them to the designated Local APIC destination ID.
///
/// # Parameters
/// - `ioapic`: Reference to the ACPI platform `IoApic` structure.
/// - `overrides`: Slice of ACPI `InterruptSourceOverride` entries.
/// - `lapic_id`: Destination Local APIC ID (shifted by 24 bits).
pub fn init(ioapic: &IoApic, overrides: &[InterruptSourceOverride], lapic_id: u32) {
    let ioapic_address = PhysAddr::new(ioapic.address as u64).to_virt();

    let id = ((read_from_ioapic(ioapic_address, IOAPICID) >> 24) & 0xFF) as u8;
    let (max_irqs, version) = {
        let value = read_from_ioapic(ioapic_address, IOAPICVER);

        ((value >> 16) as u8, value as u8)
    };

    info!(
        "I/O APIC initialized: ID = {}, Version = {:#x}, Max IRQs = {}",
        id,
        version,
        max_irqs + 1
    );

    let lapic_id = lapic_id << 24;

    let timer_gsi = isa_to_gsi(TIMER_ISA, overrides);
    let base_reg = IOREDTBL_BASE + timer_gsi * 2;

    debug!(
        "Mapping Timer IRQ (ISA {}) -> GSI {} (Vector {:#x})",
        TIMER_ISA, timer_gsi, TIMER_GSI_INFO
    );
    map_redtbl(ioapic_address, base_reg, TIMER_GSI_INFO, lapic_id);

    let keyboard_gsi = isa_to_gsi(KEYBOARD_ISA, overrides);
    let base_reg = IOREDTBL_BASE + keyboard_gsi * 2;

    debug!(
        "Mapping Keyboard IRQ (ISA {}) -> GSI {} (Vector {:#x})",
        KEYBOARD_ISA, keyboard_gsi, KEYBOARD_GSI_INFO
    );
    map_redtbl(ioapic_address, base_reg, KEYBOARD_GSI_INFO, lapic_id);
}

/// Reads a 32-bit register value from the I/O APIC.
///
/// Writes the register index to `IOREGSEL` and reads the data from `IOWIN`.
fn read_from_ioapic(ioapic_address: VirtAddr, reg: u32) -> u32 {
    let sel: *mut u32 = (ioapic_address + IOREGSEL).as_mut_ptr();
    let win: *const u32 = (ioapic_address + IOWIN).as_ptr();

    unsafe {
        ptr::write_volatile(sel, reg);
        ptr::read_volatile(win)
    }
}

/// Writes a 32-bit value to an I/O APIC register.
///
/// Writes the register index to `IOREGSEL` followed by the value to `IOWIN`.
fn write_to_ioapic(ioapic_address: VirtAddr, reg: u32, value: u32) {
    let sel: *mut u32 = (ioapic_address + IOREGSEL).as_mut_ptr();
    let win: *mut u32 = (ioapic_address + IOWIN).as_mut_ptr();

    unsafe {
        ptr::write_volatile(sel, reg);
        ptr::write_volatile(win, value);
    }
}

/// Configures a 64-bit Redirection Table entry (IOREDTBL) for an IRQ pin.
///
/// Write the destination field (`dst`) to the high 32-bit register (`base_reg + 1`),
/// and vector / delivery mode flags (`gsi_info`) to the low 32-bit register (`base_reg`).
fn map_redtbl(ioapic_address: VirtAddr, base_reg: u32, gsi_info: u32, dst: u32) {
    write_to_ioapic(ioapic_address, base_reg + 1, dst);
    write_to_ioapic(ioapic_address, base_reg, gsi_info);
}

/// Resolves an ISA IRQ line number to its corresponding Global System Interrupt (GSI).
///
/// Searches the ACPI interrupt source overrides slice. If an explicit override
/// exists for `isa_irq`, returns its `global_system_interrupt`; otherwise defaults to `isa_irq`.
fn isa_to_gsi(isa_irq: u8, overrides: &[InterruptSourceOverride]) -> u32 {
    overrides
        .iter()
        .find(|o| o.isa_source == isa_irq)
        .map(|o| o.global_system_interrupt)
        .unwrap_or(isa_irq as u32)
}

//! I/O APIC (Input/Output Advanced Programmable Interrupt Controller) driver.
//!
//! Provides functions to query I/O APIC registers, map ISA hardware interrupt request (IRQ)
//! lines to Global System Interrupts (GSIs) via ACPI interrupt source overrides,
//! and program I/O Redirection Table entries (IOREDTBL).

use acpi::platform::interrupt::{InterruptSourceOverride, IoApic};
use core::ptr;
use spin::once::Once;
use x86_64::{PhysAddr, VirtAddr};

use crate::{acpi::lapic, address::ext::PhysExt, debug, info, interrupts::handlers::KEYBOARD_IDX};

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

/// ISA IRQ line for PS/2 Keyboard.
const KEYBOARD_ISA: u8 = 1;

const KEYBOARD_GSI_INFO: u32 = KEYBOARD_IDX as u32;

static IOAPIC_BASE: Once<VirtAddr> = Once::new();

/// Initializes the I/O APIC by configuring Redirection Table entries.
pub fn init(ioapic: &IoApic, overrides: &[InterruptSourceOverride]) {
    let ioapic_base = PhysAddr::new(ioapic.address as u64).to_virt();

    IOAPIC_BASE.call_once(|| ioapic_base);

    let id = ((read_from_ioapic(IOAPICID) >> 24) & 0xFF) as u8;
    let (max_irqs, version) = {
        let value = read_from_ioapic(IOAPICVER);

        ((value >> 16) as u8, value as u8)
    };

    info!(
        "I/O APIC initialized: ID = {}, Version = {:#x}, Max IRQs = {}",
        id,
        version,
        max_irqs + 1
    );

    let lapic_id = lapic::id();

    let keyboard_gsi = isa_to_gsi(KEYBOARD_ISA, overrides);
    let low = IOREDTBL_BASE + keyboard_gsi * 2;
    let high = low + 1;

    debug!(
        "Mapping Keyboard IRQ (ISA {}) -> GSI {} (Vector {:#x})",
        KEYBOARD_ISA, keyboard_gsi, KEYBOARD_GSI_INFO
    );
    map_redtbl(low, high, KEYBOARD_GSI_INFO, lapic_id as u32);
}

/// Reads a 32-bit register value from te I/O APIC.
///
/// Writes the register index to `IOREGSEL` and reads the data from `IOWIN`.
pub fn read_from_ioapic(ioregsel_idx: u32) -> u32 {
    let ioapic_base = ioapic_base();

    let ioregsel: *mut u32 = (ioapic_base + IOREGSEL).as_mut_ptr();
    let iowin: *const u32 = (ioapic_base + IOWIN).as_ptr();

    unsafe {
        ptr::write_volatile(ioregsel, ioregsel_idx);
        ptr::read_volatile(iowin)
    }
}

/// Writes a 32-bit value to an I/O APIC register.
///
/// Writes the register index to `IOREGSEL` followed by the value to `IOWIN`.
pub fn write_to_ioapic(ioregsel_idx: u32, iowin_val: u32) {
    let ioapic_base = ioapic_base();

    let ioregsel: *mut u32 = (ioapic_base + IOREGSEL).as_mut_ptr();
    let iowin: *mut u32 = (ioapic_base + IOWIN).as_mut_ptr();

    unsafe {
        ptr::write_volatile(ioregsel, ioregsel_idx);
        ptr::write_volatile(iowin, iowin_val);
    }
}

/// Configures a 64-bit Redirection Table entry (IOREDTBL) for an IRQ pin.
fn map_redtbl(low: u32, high: u32, gsi_info: u32, dst: u32) {
    write_to_ioapic(high, dst);
    write_to_ioapic(low, gsi_info);
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

fn ioapic_base() -> VirtAddr {
    *IOAPIC_BASE
        .get()
        .expect("IOAPIC address hasn't been initialized")
}

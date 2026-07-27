//! Local APIC (Advanced Programmable Interrupt Controller) control module.
//!
//! Provides functionality to enable the Local APIC, send End of Interrupt (EOI)
//! signals, and query CPU APIC IDs via memory-mapped I/O (MMIO) registers.

use core::ptr;

use spin::once::Once;
use x86_64::{PhysAddr, VirtAddr};

use crate::{
    address::ext::{PhysExt, VirtExt},
    debug, info,
    interrupts::handlers::SVR_IDX,
};

/// Offset for the Local APIC ID register.
const LAPIC_ID_REG: u64 = 0x20;

/// Offset for the End of Interrupt (EOI) register.
const LAPIC_EOI_REG: u64 = 0xB0;

/// Offset for the Spurious Interrupt Vector Register (SVR).
const LAPIC_SVR_REG: u64 = 0xF0;

/// Bit flag in SVR to enable the Local APIC.
const SVR_APIC_ENABLE: u32 = 1 << 8;

/// Global thread-safe singleton holding the initialized LAPIC virtual address.
pub static LAPIC_ADDRESS: Once<VirtAddr> = Once::new();

/// Enables the Local APIC at the given physical base address.
///
/// Converts the physical address to a virtual address, writes to the Spurious
/// Interrupt Vector Register (SVR at offset `0xF0`) to set the APIC enable bit
/// and set the spurious interrupt vector index ([`SVR_IDX`]), and stores the
/// global [`LAPIC_ADDRESS`] singleton.
///
/// # Parameters
/// - `lapic_address`: Physical address of the Local APIC base registers.
pub fn enable(lapic_address: u64) {
    let virt_address = PhysAddr::new(lapic_address).to_virt();

    debug!(
        "Enabling Local APIC (Phys: {:#x}, Virt: {:#x})",
        lapic_address,
        virt_address.as_u64()
    );

    let svr: *mut u32 = virt_address.offset(LAPIC_SVR_REG).as_mut_ptr();

    unsafe {
        let mut svr_value = ptr::read_volatile(svr);

        svr_value |= SVR_APIC_ENABLE;
        svr_value |= SVR_IDX as u32;

        ptr::write_volatile(svr, svr_value);
    }

    LAPIC_ADDRESS.call_once(|| virt_address);

    info!("Local APIC enabled (ID: {})", id());
}

/// Sends an End of Interrupt (EOI) signal to the Local APIC.
///
/// Writing `0` to the EOI register (offset `0xB0`) signals to the Local APIC
/// that processing of the current interrupt has finished, allowing higher or
/// equal priority interrupts to be delivered.
pub fn eoi() {
    unsafe {
        let ptr: *mut u32 = LAPIC_ADDRESS
            .get_unchecked()
            .offset(LAPIC_EOI_REG)
            .as_mut_ptr();

        ptr::write_volatile(ptr, 0);
    }
}

/// Reads the Local APIC ID of the current processor.
///
/// Reads the value from the Local APIC ID register (offset `0x20`) and shifts
/// it appropriately.
///
/// # Returns
/// The APIC ID left-shifted by 24 bits as required by IOAPIC destination matching.
pub fn id() -> u32 {
    unsafe {
        let ptr: *const u32 = LAPIC_ADDRESS.get_unchecked().offset(LAPIC_ID_REG).as_ptr();
        let value = ptr::read_volatile(ptr);
        value << 24
    }
}

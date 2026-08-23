//! ACPI (Advanced Configuration and Power Interface) subsystem.
//!
//! Handles table parsing via RSDP, interrupt model identification,
//! legacy 8259 PIC disabling, and Local/IO APIC initialization.

pub mod handler;

use crate::{
    boot::info::RSDP,
    debug,
    memory::address::{PhysExt, VirtExt},
};

use self::handler::AcpiHandler;

use acpi::{AcpiTables, platform::AcpiPlatform};

/// Initializes the ACPI platform.
///
/// # Panics
/// Panics if the bootloader fails to return a valid RSDP response.
pub fn init() -> AcpiPlatform<AcpiHandler> {
    let rsdp_address = RSDP.to_phys();
    debug!("RSDP table address: {:#x}", rsdp_address.as_u64());

    let tables = unsafe {
        AcpiTables::from_rsdp(AcpiHandler, rsdp_address.as_usize())
            .expect("Failed to get ACPI tables from RSDP")
    };
    debug!("ACPI tables parsed successfully from RSDP");

    AcpiPlatform::new(tables, AcpiHandler).expect("Failed to initialize ACPI platform")
}

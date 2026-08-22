//! ACPI (Advanced Configuration and Power Interface) subsystem.
//!
//! Handles table parsing via RSDP, interrupt model identification,
//! legacy 8259 PIC disabling, and Local/IO APIC initialization.

pub mod handler;

use crate::{
    debug,
    memory::address::ext::{PhysExt, VirtExt},
};

use self::handler::AcpiHandler;

use acpi::{AcpiTables, platform::AcpiPlatform};
use limine::request::RsdpRequest;
use x86_64::VirtAddr;

static RSDP_REQUEST: RsdpRequest = RsdpRequest::new();

/// Initializes the ACPI platform.
///
/// # Panics
/// Panics if the bootloader fails to return a valid RSDP response.
pub fn init() -> AcpiPlatform<AcpiHandler> {
    let rsdp_response = RSDP_REQUEST
        .response()
        .expect("Failed to receive RSDP response from bootloader");

    // Get physical RSDP address
    let rsdp_address = VirtAddr::new(rsdp_response.address as u64).to_phys();
    debug!("RSDP table address: {:#x}", rsdp_address.as_u64());

    let tables = unsafe {
        AcpiTables::from_rsdp(AcpiHandler, rsdp_address.as_usize())
            .expect("Failed to get ACPI tables from RSDP")
    };
    debug!("ACPI tables parsed successfully from RSDP");

    AcpiPlatform::new(tables, AcpiHandler).expect("Failed to initialize ACPI platform")
}

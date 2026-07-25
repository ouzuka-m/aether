//! ACPI (Advanced Configuration and Power Interface) subsystem.
//!
//! Handles table parsing via RSDP, interrupt model identification,
//! legacy 8259 PIC disabling, and Local/IO APIC initialization.

pub mod handler;
pub mod ioapic;
pub mod lapic;
pub mod pic;

use crate::{
    address::ext::{PhysExt, VirtExt},
    debug, error, info,
};

use self::handler::AcpiHandler;

use acpi::{
    AcpiTables,
    platform::{AcpiPlatform, InterruptModel},
};
use limine::request::RsdpRequest;
use x86_64::VirtAddr;

static RSDP_REQUEST: RsdpRequest = RsdpRequest::new();

/// Initializes the ACPI subsystem and configures the hardware interrupt system.
///
/// This function queries the Limine bootloader for the RSDP (Root System Description Pointer)
/// physical address, parses all available ACPI system tables, checks the platform interrupt model,
/// disables the legacy 8259 PIC, and initializes the Local APIC and I/O APIC.
///
/// # Panics
/// Panics if:
/// - RSDP table is not provided by the bootloader.
/// - ACPI tables or platform cannot be initialized.
/// - The platform does not support the APIC interrupt model.
/// - No I/O APIC is found in the ACPI tables.
pub fn init() {
    info!("Initializing ACPI subsystem...");

    let Some(rsdp_response) = RSDP_REQUEST.response() else {
        error!("Failed to receive RSDP response from bootloader");
        panic!("failed to get RSDP response data");
    };

    // Get physical RSDP address
    let rsdp_address = VirtAddr::new(rsdp_response.address as u64).to_phys();
    debug!("RSDP table address: {:#x}", rsdp_address.as_u64());

    let tables = unsafe {
        AcpiTables::from_rsdp(AcpiHandler, rsdp_address.as_usize())
            .expect("failed to initialize ACPI tables")
    };
    debug!("ACPI tables parsed successfully from RSDP");

    let platform =
        AcpiPlatform::new(tables, AcpiHandler).expect("failed to initialize ACPI platform");
    debug!("ACPI platform initialized");

    match platform.interrupt_model {
        InterruptModel::Apic(apic) => {
            info!("APIC interrupt model detected");

            // Disable Intel PIC 8259
            pic::disable();

            lapic::enable(apic.local_apic_address);

            let Some(ioapic) = apic.io_apics.first() else {
                error!("No I/O APIC found in ACPI tables, cannot route IRQs");
                panic!("IOAPIC not found, can't handle IRQs");
            };
            let overrides = apic.interrupt_source_overrides.as_slice();
            let lapic_id = lapic::id();

            debug!(
                "Initializing I/O APIC at physical address {:#x} for Local APIC ID {}",
                ioapic.address, lapic_id
            );
            ioapic::init(ioapic, overrides, lapic_id);
        }
        _ => {
            error!("Unsupported interrupt model (legacy 8259 PIC only)");
            panic!("unknown interrupt model, no legacy support for I8259");
        }
    };

    info!("ACPI subsystem initialization complete");
}

//! ACPI (Advanced Configuration and Power Interface) subsystem.
//!
//! Handles table parsing via RSDP, interrupt model identification,
//! legacy 8259 PIC disabling, and Local/IO APIC initialization.

pub mod handler;
pub mod hpet;
pub mod ioapic;
pub mod lapic;
pub mod lvt;
pub mod pic;

use crate::{
    acpi::lvt::timer,
    address::ext::{PhysExt, VirtExt},
    debug, info,
};

use self::handler::AcpiHandler;

use acpi::{
    AcpiTables, HpetInfo,
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

    let hpet = HpetInfo::new(&tables).expect("Failed to get HPET info from ACPI tables");
    hpet::init(&hpet);
    debug!("HPET initialized");

    let platform =
        AcpiPlatform::new(tables, AcpiHandler).expect("Failed to initialize ACPI platform");
    debug!("ACPI platform initialized");

    match platform.interrupt_model {
        InterruptModel::Apic(apic) => {
            info!("APIC interrupt model detected");

            // Disable Intel PIC 8259
            pic::disable();

            lapic::enable(apic.local_apic_address);

            timer::init();

            let ioapic = apic
                .io_apics
                .first()
                .expect("No I/O APIC found in ACPI tables");
            let overrides = apic.interrupt_source_overrides.as_slice();

            debug!(
                "Initializing I/O APIC at physical address {:#x}",
                ioapic.address
            );
            ioapic::init(ioapic, overrides);
        }
        _ => {
            panic!("Unsupported interrupt model (legacy 8259 PIC only)");
        }
    };

    info!("ACPI subsystem initialization complete");
}

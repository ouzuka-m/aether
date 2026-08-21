pub mod hpet;
pub mod ioapic;
pub mod lapic;
pub mod pic;
pub mod tsc_deadline;
pub mod uart;

use acpi::{
    HpetInfo,
    platform::{AcpiPlatform, InterruptModel},
};

use crate::{acpi::handler::AcpiHandler, info};

pub fn init(platform: &AcpiPlatform<AcpiHandler>) {
    let InterruptModel::Apic(apic) = &platform.interrupt_model else {
        panic!("Death");
    };

    info!("APIC interrupt model detected");

    // Disable Intel PIC 8259
    pic::disable();

    let hpet_info = HpetInfo::new(&platform.tables).expect("HPET information not found");
    hpet::init(&hpet_info);

    lapic::init(apic.local_apic_address);

    tsc_deadline::init();

    let ioapics = &apic.io_apics;

    let ioapic = ioapics.first().expect("No I/O APIC found in ACPI tables");
    let overrides = apic.interrupt_source_overrides.as_slice();

    ioapic::init(ioapic, overrides);
}

pub mod acpi;
pub mod apic;
pub mod hpet;
pub mod input;
pub mod pic;
pub mod tsc_deadline;
pub mod uart;

use crate::{
    drivers::apic::{ioapic, lapic},
    info,
};
use ::acpi::{HpetInfo, platform::InterruptModel};

pub fn init() {
    let acpi = acpi::init();

    let InterruptModel::Apic(apic) = &acpi.interrupt_model else {
        panic!("Unsupported interrupt model, can't handle IRQs");
    };

    info!("APIC interrupt model detected");

    // Disable Intel PIC 8259
    pic::disable();

    let hpet_info = HpetInfo::new(&acpi.tables).expect("HPET information not found");
    hpet::init(&hpet_info);

    lapic::init(apic.local_apic_address);

    tsc_deadline::init();

    let ioapics = &apic.io_apics;

    let ioapic = ioapics.first().expect("No I/O APIC found in ACPI tables");
    let overrides = apic.interrupt_source_overrides.as_slice();

    ioapic::init(ioapic, overrides);
}

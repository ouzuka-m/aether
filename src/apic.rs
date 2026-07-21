use core::{pin::Pin, ptr};

use acpi::{AcpiTables, sdt::madt::Madt};

use crate::acpi::handler::AcpiHandler;

pub fn enable(acpi: &AcpiTables<AcpiHandler>, hhdm: u64) {
    let physical_mapping = acpi.find_table::<Madt>().expect("failed to get MADT table");
    let madt: Pin<&Madt> = physical_mapping.get();

    let lapic_virt_address = madt.local_apic_address as u64 + hhdm;
    let svr = (lapic_virt_address + 0xF0) as *mut u32;

    unsafe {
        // Set bit 8 & spurious vector interrupt
        ptr::write_volatile(svr, 0x1FF);
    }
}

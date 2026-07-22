pub mod handler;
pub mod lapic;

use crate::address::ext::{PhysExt, VirtExt};

use self::handler::AcpiHandler;

use acpi::{
    AcpiTables,
    platform::{AcpiPlatform, InterruptModel},
};
use limine::request::RsdpRequest;
use x86_64::{PhysAddr, VirtAddr};

static RSDP_REQUEST: RsdpRequest = RsdpRequest::new();

pub fn init() {
    let Some(rsdp_response) = RSDP_REQUEST.response() else {
        panic!("failed to get RSDP response data");
    };

    // Get physical RSDP address
    let rsdp_address = VirtAddr::new(rsdp_response.address as u64).to_phys();

    let tables = unsafe {
        AcpiTables::from_rsdp(AcpiHandler, rsdp_address.as_u64() as usize)
            .expect("failed to initialize ACPI tables")
    };

    let plaform =
        AcpiPlatform::new(tables, AcpiHandler).expect("failed to initialize ACPI platform");

    match plaform.interrupt_model {
        InterruptModel::Apic(apic) => {
            let lapic_address = PhysAddr::new(apic.local_apic_address).to_virt();
            lapic::enable(lapic_address);
        }
        _ => panic!("unknown interrupt model. Can't handle IRQs"),
    };
}

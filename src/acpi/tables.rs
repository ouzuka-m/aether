use acpi::AcpiTables;
use limine::request::RsdpRequest;

use crate::acpi::handler::AcpiHandler;

static RSDP_REQUEST: RsdpRequest = RsdpRequest::new();

pub fn init(hhdm: usize) -> AcpiTables<AcpiHandler> {
    let Some(rsdp_response) = RSDP_REQUEST.response() else {
        panic!("failed to get RSDP response data");
    };

    unsafe {
        // Get physical RSDP address
        let rsdp_address = rsdp_response.address as usize - hhdm;
        let handler = AcpiHandler { hhdm };

        AcpiTables::from_rsdp(handler, rsdp_address).expect("failed to initialize ACPI tables")
    }
}

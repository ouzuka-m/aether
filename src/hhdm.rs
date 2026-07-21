use limine::request::HhdmRequest;
use x86_64::VirtAddr;

static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

pub fn get() -> VirtAddr {
    let Some(hhdm_response) = HHDM_REQUEST.response() else {
        panic!("failed to get HHDM response data");
    };

    VirtAddr::new(hhdm_response.offset)
}

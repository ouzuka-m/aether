use limine::request::HhdmRequest;
use x86_64::VirtAddr;

static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

lazy_static::lazy_static! {
    pub static ref HHDM: VirtAddr = {
        let Some(hhdm_response) = HHDM_REQUEST.response() else {
            panic!("failed to get HHDM response data");
        };

        VirtAddr::new(hhdm_response.offset)
    };
}

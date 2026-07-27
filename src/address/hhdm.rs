//! # Higher-Half Direct Mapping (HHDM) Request
//!
//! Interfaces with the Limine bootloader to obtain the Higher-Half Direct Mapping (HHDM) virtual memory offset.

use limine::request::HhdmRequest;
use x86_64::VirtAddr;

/// Limine bootloader HHDM feature request structure.
static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

lazy_static::lazy_static! {
    /// Virtual base address offset of the Higher-Half Direct Mapping (HHDM).
    ///
    /// The bootloader maps all physical memory directly at this virtual address offset.
    ///
    /// # Panics
    /// Panics if the bootloader fails to return a valid HHDM response.
    pub static ref HHDM: VirtAddr = {
        let hhdm_response = HHDM_REQUEST.response().expect("Failed to receive HHDM response from bootloader");

        VirtAddr::new(hhdm_response.offset)
    };
}

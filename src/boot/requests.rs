//! Limine boot protocol requests.
//!
//! Declares the static request objects that the Limine bootloader scans for
//! during early boot. Each request is populated with a response by the
//! bootloader before control is transferred to the kernel entry point.

use limine::request::{
    ExecutableCmdlineRequest, FramebufferRequest, HhdmRequest, MemmapRequest, ModulesRequest,
    RsdpRequest,
};

pub static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

pub static MEMMAP_REQUEST: MemmapRequest = MemmapRequest::new();

pub static RSDP_REQUEST: RsdpRequest = RsdpRequest::new();

pub static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

pub static MODULES_REQUEST: ModulesRequest = ModulesRequest::new();

pub static CMDLINE_REQUEST: ExecutableCmdlineRequest = ExecutableCmdlineRequest::new();

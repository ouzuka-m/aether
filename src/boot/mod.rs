//! Boot subsystem.
//!
//! Provides static accessors for bootloader-supplied data (HHDM offset,
//! memory map, RSDP address, framebuffer, kernel modules, and command line)
//! and declares the Limine boot protocol requests.

pub mod info;
pub mod requests;

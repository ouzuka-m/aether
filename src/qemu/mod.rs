//! QEMU integration utilities.
//!
//! Provides helpers for communicating with the QEMU debug-exit device,
//! allowing the kernel to signal success or failure to an automated
//! test harness.

pub mod exit;

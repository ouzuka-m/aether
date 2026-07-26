//! # Memory Address Conversion & Higher-Half Direct Mapping (HHDM)
//!
//! Provides extension traits and utilities for converting between physical memory addresses
//! (`PhysAddr`) and higher-half virtual memory addresses (`VirtAddr`) based on the Limine HHDM offset.

pub mod ext;
pub mod hhdm;

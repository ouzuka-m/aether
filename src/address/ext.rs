//! # Address Extension Traits
//!
//! Provides convenience extension methods for [`x86_64::PhysAddr`] and [`x86_64::VirtAddr`]
//! to seamlessly translate addresses between physical and virtual address spaces using HHDM.

use x86_64::{PhysAddr, VirtAddr};

use crate::address::hhdm::HHDM;

/// Extension trait for physical memory addresses ([`PhysAddr`]).
pub trait PhysExt {
    /// Translates a physical address to its corresponding higher-half virtual address.
    fn to_virt(self) -> VirtAddr;

    /// Returns the physical address value as a `usize`.
    fn as_usize(&self) -> usize;
}

/// Extension trait for virtual memory addresses ([`VirtAddr`]).
pub trait VirtExt {
    /// Translates a higher-half virtual address back to its physical address.
    fn to_phys(self) -> PhysAddr;

    /// Returns a new virtual address offset by `offset` bytes.
    fn offset(&self, offset: u64) -> VirtAddr;
}

impl PhysExt for PhysAddr {
    fn to_virt(self) -> VirtAddr {
        VirtAddr::new((*HHDM).as_u64() + self.as_u64())
    }

    fn as_usize(&self) -> usize {
        self.as_u64() as usize
    }
}

impl VirtExt for VirtAddr {
    fn to_phys(self) -> PhysAddr {
        PhysAddr::new(self.as_u64() - (*HHDM).as_u64())
    }

    fn offset(&self, offset: u64) -> VirtAddr {
        VirtAddr::new(self.as_u64() + offset)
    }
}

//! ACPI table memory handler implementation.
//!
//! Provides the [`AcpiHandler`] type which implements the [`acpi::Handler`] trait,
//! allowing the `acpi` crate to map physical memory addresses into kernel virtual memory space.

use core::ptr::NonNull;

use acpi::{Handle, Handler, PciAddress, PhysicalMapping, aml::AmlError};
use x86_64::PhysAddr;

use crate::{address::ext::PhysExt, debug};

/// Kernel implementation of the `acpi::Handler` trait.
///
/// Responsible for translating physical memory addresses to virtual addresses
/// using higher-half kernel direct mapping during ACPI table discovery.
#[derive(Clone)]
pub struct AcpiHandler;

impl Handler for AcpiHandler {
    /// Maps a physical memory region into virtual address space for ACPI table parsing.
    ///
    /// # Safety
    /// Caller must ensure that `physical_address` points to valid physical memory
    /// mapped into kernel virtual space.
    unsafe fn map_physical_region<T>(
        &self,
        physical_address: usize,
        size: usize,
    ) -> PhysicalMapping<Self, T> {
        let virt_addr = PhysAddr::new(physical_address as u64).to_virt();
        debug!(
            "ACPI map_physical_region: Phys {:#x} -> Virt {:#x} (Size: {} bytes)",
            physical_address,
            virt_addr.as_u64(),
            size
        );

        let ptr: *mut T = virt_addr.as_mut_ptr();

        PhysicalMapping {
            physical_start: physical_address,
            virtual_start: NonNull::new(ptr).expect("virtual address pointer cannot be null"),
            region_length: size,
            mapped_length: size,
            handler: self.clone(),
        }
    }

    /// Unmaps a previously mapped physical region.
    ///
    /// In higher-half physical identity mapping, memory remains mapped, so this operation is a no-op.
    fn unmap_physical_region<T>(region: &PhysicalMapping<Self, T>) {
        debug!(
            "ACPI unmap_physical_region: Phys {:#x} (Size: {} bytes)",
            region.physical_start, region.region_length
        );
    }

    fn read_u8(&self, _: usize) -> u8 {
        unimplemented!()
    }

    fn read_u16(&self, _: usize) -> u16 {
        unimplemented!()
    }

    fn read_u32(&self, _: usize) -> u32 {
        unimplemented!()
    }

    fn read_u64(&self, _: usize) -> u64 {
        unimplemented!()
    }

    fn write_u8(&self, _: usize, _: u8) {
        unimplemented!()
    }

    fn write_u16(&self, _: usize, _: u16) {
        unimplemented!()
    }

    fn write_u32(&self, _: usize, _: u32) {
        unimplemented!()
    }

    fn write_u64(&self, _: usize, _: u64) {
        unimplemented!()
    }

    fn read_io_u8(&self, _: u16) -> u8 {
        unimplemented!()
    }

    fn read_io_u16(&self, _: u16) -> u16 {
        unimplemented!()
    }

    fn read_io_u32(&self, _: u16) -> u32 {
        unimplemented!()
    }

    fn write_io_u8(&self, _: u16, _: u8) {
        unimplemented!()
    }

    fn write_io_u16(&self, _: u16, _: u16) {
        unimplemented!()
    }

    fn write_io_u32(&self, _: u16, _: u32) {
        unimplemented!()
    }

    fn read_pci_u8(&self, _: PciAddress, _: u16) -> u8 {
        unimplemented!()
    }

    fn read_pci_u16(&self, _: PciAddress, _: u16) -> u16 {
        unimplemented!()
    }

    fn read_pci_u32(&self, _: PciAddress, _: u16) -> u32 {
        unimplemented!()
    }

    fn write_pci_u8(&self, _: PciAddress, _: u16, _: u8) {
        unimplemented!()
    }

    fn write_pci_u16(&self, _: PciAddress, _: u16, _: u16) {
        unimplemented!()
    }

    fn write_pci_u32(&self, _: PciAddress, _: u16, _: u32) {
        unimplemented!()
    }

    fn nanos_since_boot(&self) -> u64 {
        unimplemented!()
    }

    fn stall(&self, _: u64) {}

    fn sleep(&self, _: u64) {}

    fn create_mutex(&self) -> Handle {
        unimplemented!()
    }

    fn acquire(&self, _: Handle, _: u16) -> Result<(), AmlError> {
        unimplemented!()
    }

    fn release(&self, _: Handle) {}
}

use core::ptr::NonNull;

use acpi::{Handle, Handler, PciAddress, PhysicalMapping, aml::AmlError};
use x86_64::PhysAddr;

use crate::address::ext::PhysExt;

#[derive(Clone)]
pub struct AcpiHandler;

impl Handler for AcpiHandler {
    unsafe fn map_physical_region<T>(
        &self,
        physical_address: usize,
        size: usize,
    ) -> PhysicalMapping<Self, T> {
        let ptr: *mut T = PhysAddr::new(physical_address as u64)
            .to_virt()
            .as_mut_ptr();

        PhysicalMapping {
            physical_start: physical_address,
            virtual_start: NonNull::new(ptr).unwrap(),
            region_length: size,
            mapped_length: size,
            handler: self.clone(),
        }
    }

    fn unmap_physical_region<T>(_: &PhysicalMapping<Self, T>) {
        // Nothing
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

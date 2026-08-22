//! ACPI table memory handler implementation.
//!
//! Provides the [`AcpiHandler`] type which implements the [`acpi::Handler`] trait,
//! allowing the `acpi` crate to map physical memory addresses into kernel virtual memory space.

use core::ptr::NonNull;

use acpi::{Handle, Handler, PciAddress, PhysicalMapping, aml::AmlError};
use x86_64::PhysAddr;

use crate::{debug, memory::address::ext::PhysExt};

macro_rules! stub_handler {
    ($($fn_name:ident ($($_:ident : $ty:ty),*) $(-> $ret:ty)? ;)*) => {
        $(
            fn $fn_name(&self, $(_: $ty),*) $(-> $ret)? {
                unimplemented!()
            }
        )*
    };
}

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

    stub_handler! {
        read_u8(address: usize) -> u8;
        read_u16(address: usize) -> u16;
        read_u32(address: usize) -> u32;
        read_u64(address: usize) -> u64;

        write_u8(address: usize, value: u8);
        write_u16(address: usize, value: u16);
        write_u32(address: usize, value: u32);
        write_u64(address: usize, value: u64);

        read_io_u8(port: u16) -> u8;
        read_io_u16(port: u16) -> u16;
        read_io_u32(port: u16) -> u32;

        write_io_u8(port: u16, value: u8);
        write_io_u16(port: u16, value: u16);
        write_io_u32(port: u16, value: u32);

        read_pci_u8(address: PciAddress, offset: u16) -> u8;
        read_pci_u16(address: PciAddress, offset: u16) -> u16;
        read_pci_u32(address: PciAddress, offset: u16) -> u32;

        write_pci_u8(address: PciAddress, offset: u16, value: u8);
        write_pci_u16(address: PciAddress, offset: u16, value: u16);
        write_pci_u32(address: PciAddress, offset: u16, value: u32);

        nanos_since_boot() -> u64;
        stall(microseconds: u64);
        sleep(microseconds: u64);

        create_mutex() -> Handle;
        acquire(mutex: Handle, timeout: u16) -> Result<(), AmlError>;
        release(mutex: Handle);
    }
}

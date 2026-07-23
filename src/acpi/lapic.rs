use core::ptr;

use x86_64::VirtAddr;

use crate::address::ext::VirtExt;

pub const ENABLE_BIT: u32 = 1 << 8; // Enable the 8 bit
pub const SPURIUOUS_INTERRUPT_INDEX: u32 = 0xFF; // Set the SVR Interrupt to 255

pub fn enable(lapic_address: VirtAddr) {
    let svr: *mut u32 = lapic_address.offset(0xF0).as_mut_ptr();

    unsafe {
        let mut svr_value = ptr::read_volatile(svr);

        svr_value |= ENABLE_BIT;
        svr_value |= SPURIUOUS_INTERRUPT_INDEX;

        core::ptr::write_volatile(svr, svr_value);
    }

    crate::info!("Local APIC enabled");
}

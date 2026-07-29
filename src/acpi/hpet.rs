use core::ptr;

use acpi::HpetInfo;
use spin::once::Once;
use x86_64::{PhysAddr, VirtAddr};

use crate::address::ext::{PhysExt, VirtExt};

const CAP_ID: u64 = 0x000;
const CONFIG: u64 = 0x010;
const MAIN_COUNTER: u64 = 0x0F0;

#[derive(Debug)]
pub struct Hpet {
    base: VirtAddr,
    period_fs: u64,
}

pub static HPET: Once<Hpet> = Once::new();

pub fn init(hpet: &HpetInfo) {
    let base = PhysAddr::new(hpet.base_address as u64).to_virt();

    let period_fs = read_from_hpet(base, CAP_ID) >> 32;
    if period_fs == 0 || period_fs > 0x05F5E100 {
        panic!("Invalid HPET main counter tick");
    }

    // Enable CNF
    write_to_hpet(base, CONFIG, 0x1);

    HPET.call_once(|| Hpet { base, period_fs });
}

pub fn wait_ns(ns: u64) {
    let hpet = unsafe { HPET.get_unchecked() };

    let ticks = (ns * 1_000_000) / hpet.period_fs;
    let target = read_counter() + ticks;
    while read_counter() < target {
        core::hint::spin_loop();
    }
}

pub fn wait_ms(ms: u64) {
    wait_ns(ms * 1_000_000);
}

pub fn read_counter() -> u64 {
    let hpet_base = unsafe { HPET.get_unchecked().base };
    read_from_hpet(hpet_base, MAIN_COUNTER)
}

pub fn read_from_hpet(hpet_base: VirtAddr, offset: u64) -> u64 {
    unsafe { ptr::read_volatile(hpet_base.offset(offset).as_ptr::<u64>()) }
}

pub fn write_to_hpet(hpet_base: VirtAddr, offset: u64, value: u64) {
    unsafe { ptr::write_volatile(hpet_base.offset(offset).as_mut_ptr::<u64>(), value) }
}

use core::ptr;

use acpi::HpetInfo;
use spin::once::Once;
use x86_64::{PhysAddr, VirtAddr};

use crate::address::ext::{PhysExt, VirtExt};

const CAP_ID: u64 = 0x000;
const CONFIG: u64 = 0x010;
const MAIN_COUNTER: u64 = 0x0F0;

static HPET_BASE: Once<VirtAddr> = Once::new();
static PERIOD_FS: Once<u64> = Once::new();

pub fn init(hpet: &HpetInfo) {
    let hpet_base = PhysAddr::new(hpet.base_address as u64).to_virt();

    HPET_BASE.call_once(|| hpet_base);

    let period_fs = read_from_hpet(CAP_ID) >> 32;
    if period_fs == 0 || period_fs > 0x05F5E100 {
        panic!("Invalid HPET main counter tick");
    }

    PERIOD_FS.call_once(|| period_fs);

    // Enable CNF
    write_to_hpet(CONFIG, 0x1);
}

pub fn wait_ns(ns: u64) {
    let period_fs = period_fs();

    let ticks = ns.checked_mul(1_000_000).expect("wait_ns overflow") / period_fs;
    let target = read_counter() + ticks;
    while read_counter() < target {
        core::hint::spin_loop();
    }
}

pub fn wait_ms(ms: u64) {
    wait_ns(ms * 1_000_000);
}

pub fn read_counter() -> u64 {
    read_from_hpet(MAIN_COUNTER)
}

pub fn read_from_hpet(offset: u64) -> u64 {
    let hpet_base = hpet_address();
    unsafe { ptr::read_volatile(hpet_base.offset(offset).as_ptr::<u64>()) }
}

pub fn write_to_hpet(offset: u64, value: u64) {
    let hpet_base = hpet_address();
    unsafe { ptr::write_volatile(hpet_base.offset(offset).as_mut_ptr::<u64>(), value) }
}

fn hpet_address() -> VirtAddr {
    *HPET_BASE
        .get()
        .expect("HPET address hasn't bee initialized")
}

fn period_fs() -> u64 {
    *PERIOD_FS
        .get()
        .expect("Counter tick period hasn't been initialized")
}

use core::ptr;

use acpi::HpetInfo;
use spin::once::Once;
use x86_64::{PhysAddr, VirtAddr};

use crate::memory::address::ext::{PhysExt, VirtExt};

const CAP_ID: u64 = 0x000;
const CONFIG: u64 = 0x010;
const MAIN_COUNTER: u64 = 0x0F0;

static BASE_ADDRESS: Once<VirtAddr> = Once::new();
static PERIOD_FS: Once<u64> = Once::new();

pub fn init(hpet_info: &HpetInfo) {
    let base_address = PhysAddr::new(hpet_info.base_address as u64).to_virt();

    BASE_ADDRESS.call_once(|| base_address);

    let period_fs = read(CAP_ID) >> 32;
    if period_fs == 0 || period_fs > 0x05F5E100 {
        panic!("Invalid HPET main counter tick");
    }

    PERIOD_FS.call_once(|| period_fs);

    // Enable CNF
    write(CONFIG, 0x1);
}

pub fn wait_ns(ns: u64) {
    let period_fs = period_fs();

    let ticks = ns.checked_mul(1_000_000).expect("wait_ns overflow") / period_fs;
    let target = counter() + ticks;
    while counter() < target {
        core::hint::spin_loop();
    }
}

pub fn wait_ms(ms: u64) {
    wait_ns(ms * 1_000_000);
}

pub fn counter() -> u64 {
    read(MAIN_COUNTER)
}

fn read(offset: u64) -> u64 {
    let base_address = base_address();
    unsafe { ptr::read_volatile(base_address.offset(offset).as_ptr::<u64>()) }
}

fn write(offset: u64, value: u64) {
    let base_address = base_address();
    unsafe { ptr::write_volatile(base_address.offset(offset).as_mut_ptr::<u64>(), value) }
}

fn base_address() -> VirtAddr {
    *BASE_ADDRESS
        .get()
        .expect("HPET address hasn't bee initialized")
}

fn period_fs() -> u64 {
    *PERIOD_FS
        .get()
        .expect("Counter tick period hasn't been initialized")
}

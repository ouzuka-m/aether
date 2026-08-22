use core::arch::x86_64::__cpuid;

use spin::once::Once;
use x86_64::registers::model_specific::Msr;

use crate::{
    arch::x86_64::idt::TIMER_VECTOR,
    drivers::{hpet, lapic},
};

const LVT: u64 = 0x320;
const TSC_DEADLINE_MODE: u32 = 2 << 17;
const CALIBRATION_MS: u64 = 10;
const IA32: u32 = 0x6E0;

static TICKS_PER_MS: Once<u64> = Once::new();

pub fn init() {
    assert!(supports(), "CPU doesn't support TSC-Deadline");

    lapic::configure_lvt(LVT, TIMER_VECTOR, TSC_DEADLINE_MODE);

    let start = unsafe { core::arch::x86_64::_rdtsc() };
    hpet::wait_ms(CALIBRATION_MS);
    let end = unsafe { core::arch::x86_64::_rdtsc() };

    TICKS_PER_MS.call_once(|| (end - start) / CALIBRATION_MS);

    arm(1);
}

pub fn arm(ms: u64) {
    let now = unsafe { core::arch::x86_64::_rdtsc() };
    let ticks_per_ms = ticks_per_ms();

    let delta = ticks_per_ms.checked_mul(ms).expect("TSC delay overflow");
    let deadline = now.checked_add(delta).expect("TSC deadline overflow");

    wrmsr(deadline);
}

#[allow(unused)]
pub fn disarm() {
    wrmsr(0);
}

pub fn wrmsr(value: u64) {
    unsafe { Msr::new(IA32).write(value) };
}

pub fn supports() -> bool {
    let cpuid = __cpuid(1);
    (cpuid.ecx & (1 << 24)) != 0
}

pub fn ticks_per_ms() -> u64 {
    *TICKS_PER_MS.get().expect("Ticks per MS haven't calculated")
}

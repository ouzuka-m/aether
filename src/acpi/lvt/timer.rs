use core::arch::x86_64::__cpuid;

use spin::once::Once;
use x86_64::registers::model_specific::Msr;

use crate::{
    acpi::{hpet, lapic},
    interrupts::handlers::TIMER_VECTOR,
};

const LVT: u64 = 0x320;

const TSC_MODE: u32 = 2 << 17;

const CALIBRATION_MS: u64 = 10;

const IA32_TSC_DEADLINE: u32 = 0x6E0;

static TICKS_PER_MS: Once<u64> = Once::new();

pub fn init() {
    assert!(supports_tsc_deadline(), "CPU doesn't support TSC-Deadline");

    // Using TSC-Deadline mode
    lapic::lapic_write(LVT, TSC_MODE | TIMER_VECTOR as u32);

    let start = unsafe { core::arch::x86_64::_rdtsc() };
    hpet::wait_ms(CALIBRATION_MS);
    let end = unsafe { core::arch::x86_64::_rdtsc() };

    TICKS_PER_MS.call_once(|| (end - start) / CALIBRATION_MS);

    arm_tsc_deadline(1);
}

pub fn arm_tsc_deadline(ms: u64) {
    let now = unsafe { core::arch::x86_64::_rdtsc() };
    let ticks_per_ms = ticks_per_ms();

    let delta = ticks_per_ms.checked_mul(ms).expect("TSC delay overflow");
    let deadline = now.checked_add(delta).expect("TSC deadline overflow");

    wrmsr(deadline);
}

#[allow(unused)]
pub fn disarm_tsc_deadline() {
    wrmsr(0);
}

pub fn wrmsr(value: u64) {
    unsafe { Msr::new(IA32_TSC_DEADLINE).write(value) };
}

pub fn supports_tsc_deadline() -> bool {
    let cpuid = __cpuid(1);
    (cpuid.ecx & (1 << 24)) != 0
}

pub fn ticks_per_ms() -> u64 {
    *TICKS_PER_MS.get().expect("Ticks per MS haven't calculated")
}

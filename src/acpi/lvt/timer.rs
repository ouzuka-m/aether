use core::arch::x86_64::__cpuid;

use x86_64::registers::model_specific::Msr;

use crate::{
    acpi::{hpet, lapic},
    interrupts::handlers::TIMER_IDX,
};

const LVT: u64 = 0x320;

const TSC_MODE: u32 = 2 << 17;

const CALIBRATION_MS: u64 = 10;

const IA32_TSC_DEADLINE: u32 = 0x6E0;

static mut TICKS_PER_MS: u64 = 0;

pub fn init() {
    assert!(
        supports_tsc_deadline(),
        "This CPU doesn't support TSC-Deadline"
    );

    // Using TSC-Deadline mode
    lapic::write_to_lapic(LVT, TSC_MODE | TIMER_IDX as u32);

    let start = unsafe { core::arch::x86_64::_rdtsc() };
    hpet::wait_ms(CALIBRATION_MS);
    let end = unsafe { core::arch::x86_64::_rdtsc() };

    unsafe { TICKS_PER_MS = (end - start) / CALIBRATION_MS };

    arm_tsc_deadline(1);
}

pub fn arm_tsc_deadline(ms: u64) {
    let now = unsafe { core::arch::x86_64::_rdtsc() };
    let ticks_per_ms = unsafe { TICKS_PER_MS };

    let deadline = now.saturating_add(ticks_per_ms.saturating_mul(ms));

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

use crate::{drivers::apic::lapic, prompt};
use x86_64::structures::idt::InterruptStackFrame;

pub extern "x86-interrupt" fn read_command(_: InterruptStackFrame) {
    prompt!();
    lapic::eoi();
}

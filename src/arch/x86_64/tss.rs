//! # Task State Segment (TSS) and Interrupt Stacks
//!
//! Configures the x86_64 Task State Segment (TSS) and sets up the Interrupt Stack Table (IST).
//! Dedicated stacks are allocated for critical exceptions (Double Fault, Non-Maskable Interrupt,
//! and Machine Check Exception) to prevent stack overflow cascading into a hardware triple fault.

use spin::lazylock::LazyLock;
use x86_64::{VirtAddr, structures::tss::TaskStateSegment};

use crate::{
    arch::x86_64::stack::{self, Stack},
    memory::address::ext::VirtExt,
};

pub static TASK_STATE_SEGMENT: LazyLock<TaskStateSegment> = LazyLock::new(|| {
    let mut tss = TaskStateSegment::new();

    tss.interrupt_stack_table[stack::DF_INDEX as usize] = stack_end(&raw const stack::DF_STACK);
    tss.interrupt_stack_table[stack::NMI_INDEX as usize] = stack_end(&raw const stack::NMI_STACK);
    tss.interrupt_stack_table[stack::MCE_INDEX as usize] = stack_end(&raw const stack::MCE_STACK);

    tss
});

fn stack_end(stack_ptr: *const Stack) -> VirtAddr {
    VirtAddr::from_ptr(stack_ptr).offset(stack::STACK_SIZE as u64)
}

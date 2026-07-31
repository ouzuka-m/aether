//! # Task State Segment (TSS) and Interrupt Stacks
//!
//! Configures the x86_64 Task State Segment (TSS) and sets up the Interrupt Stack Table (IST).
//! Dedicated stacks are allocated for critical exceptions (Double Fault, Non-Maskable Interrupt,
//! and Machine Check Exception) to prevent stack overflow cascading into a hardware triple fault.

use lazy_static::lazy_static;
use x86_64::{VirtAddr, structures::tss::TaskStateSegment};

use crate::stacks::{self, Stack};

lazy_static! {
    /// Global lazy-initialized Task State Segment (TSS).
    ///
    /// Configures the Interrupt Stack Table (IST) entries for:
    /// - Double Fault ([`stacks::DF_INDEX`])
    /// - Non-Maskable Interrupt ([`stacks::NMI_INDEX`])
    /// - Machine Check Exception ([`stacks::MCE_INDEX`])
    pub static ref TASK_STATE_SEGMENT: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();

        tss.interrupt_stack_table[stacks::DF_INDEX as usize] = stack_end(&raw const stacks::DF_STACK);
        tss.interrupt_stack_table[stacks::NMI_INDEX as usize] = stack_end(&raw const stacks::NMI_STACK);
        tss.interrupt_stack_table[stacks::MCE_INDEX as usize] = stack_end(&raw const stacks::MCE_STACK);

        tss
    };
}

fn stack_end(stack_ptr: *const Stack) -> VirtAddr {
    VirtAddr::from_ptr(stack_ptr) + stacks::STACK_SIZE as u64
}

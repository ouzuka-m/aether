use lazy_static::lazy_static;
use x86_64::{VirtAddr, structures::tss::TaskStateSegment};

use crate::stacks;

#[repr(C, align(16))]
struct Stack([u8; stacks::STACK_SIZE]);

lazy_static! {
    pub static ref TASK_STATE_SEGMENT: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();

        tss.interrupt_stack_table[stacks::DF_INDEX as usize] = {
            static STACK: Stack = Stack([0u8; stacks::STACK_SIZE]);

            let stack_start = VirtAddr::from_ptr(&raw const STACK);
            stack_start + stacks::STACK_SIZE as u64
        };
        tss.interrupt_stack_table[stacks::NMI_INDEX as usize] = {
            static STACK: Stack = Stack([0; stacks::STACK_SIZE]);

            let stack_start = VirtAddr::from_ptr(&raw const STACK);
            stack_start + stacks::STACK_SIZE as u64
        };
        tss.interrupt_stack_table[stacks::MCE_INDEX as usize] = {
            static STACK: Stack = Stack([0; stacks::STACK_SIZE]);

            let stack_start = VirtAddr::from_ptr(&raw const STACK);
            stack_start + stacks::STACK_SIZE as u64
        };

        tss
    };
}

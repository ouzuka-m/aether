use spin::lazylock::LazyLock;
use x86_64::structures::idt::InterruptDescriptorTable;

use crate::arch::x86_64::{
    interrupts::{exceptions, hardware},
    stack,
};

/// Interrupt vector index for Spurious Vector Interrupts (SVR).
pub const SVR_VECTOR: u8 = 0xFF; // 255

/// Interrupt vector index for APIC Timer interrupts.
pub const TIMER_VECTOR: u8 = 0x20; // 32

/// Interrupt vector index for PS/2 Keyboard interrupts.
pub const KEYBOARD_VECTOR: u8 = 0x21; // 33

static INTERRUPT_DESCRIPTOR_TABLE: LazyLock<InterruptDescriptorTable> = LazyLock::new(|| {
    let mut idt = InterruptDescriptorTable::new();

    idt.divide_error.set_handler_fn(exceptions::divide_error); // Vector 0
    idt.debug.set_handler_fn(exceptions::debug); // Vector 1

    idt.breakpoint.set_handler_fn(exceptions::breakpoint); // Vector 3
    idt.overflow.set_handler_fn(exceptions::overflow); // Vector 4
    idt.bound_range_exceeded
        .set_handler_fn(exceptions::bound_range_exceeded); // Vector 5
    idt.invalid_opcode
        .set_handler_fn(exceptions::invalid_opcode); // Vector 6
    idt.device_not_available
        .set_handler_fn(exceptions::device_not_available); // Vector 7

    idt.invalid_tss.set_handler_fn(exceptions::invalid_tss); // Vector 10
    idt.segment_not_present
        .set_handler_fn(exceptions::segment_not_present); // Vector 11
    idt.stack_segment_fault
        .set_handler_fn(exceptions::stack_segment_fault); // Vector 12
    idt.general_protection_fault
        .set_handler_fn(exceptions::general_protection_fault); // Vector 13
    idt.page_fault.set_handler_fn(exceptions::page_fault); // Vector 14

    idt.x87_floating_point
        .set_handler_fn(exceptions::x87_floating_point); // Vector 16
    idt.alignment_check
        .set_handler_fn(exceptions::alignment_check); // Vector 17

    idt.simd_floating_point
        .set_handler_fn(exceptions::simd_floating_point); // Vector 19
    idt.virtualization
        .set_handler_fn(exceptions::virtualization_exception); // Vector 20
    idt.cp_protection_exception
        .set_handler_fn(exceptions::control_protection_exception); // Vector 21

    idt.hv_injection_exception
        .set_handler_fn(exceptions::hv_injection_exception); // Vector 28
    idt.vmm_communication_exception
        .set_handler_fn(exceptions::vmm_communication_exception); // Vector 29
    idt.security_exception
        .set_handler_fn(exceptions::security_exception); // Vector 30

    // Hardware interrupts
    idt[TIMER_VECTOR].set_handler_fn(hardware::timer);
    idt[KEYBOARD_VECTOR].set_handler_fn(hardware::keyboard);
    idt[SVR_VECTOR].set_handler_fn(hardware::spurious_vector_interrupt);

    // Need to switch to a different stack for some interrupts
    unsafe {
        idt.double_fault
            .set_handler_fn(exceptions::double_fault)
            .set_stack_index(stack::DF_INDEX); // Vector 8

        idt.non_maskable_interrupt
            .set_handler_fn(exceptions::non_maskable_interrupt)
            .set_stack_index(stack::NMI_INDEX); // Vector 2

        idt.machine_check
            .set_handler_fn(exceptions::machine_check_exception)
            .set_stack_index(stack::MCE_INDEX); // Vector 18
    }

    idt
});

pub fn init() {
    INTERRUPT_DESCRIPTOR_TABLE.load();
    crate::info!("IDT loaded");
}

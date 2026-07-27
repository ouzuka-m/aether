use x86_64::{
    registers::control::Cr2,
    structures::idt::{InterruptStackFrame, PageFaultErrorCode},
};

use crate::{debug, info, warn};

/// Vector 0: Divide Error (#DE)
///
/// Triggered when a division instruction (e.g., `DIV` or `IDIV`) is executed with a divisor of zero,
/// or when the quotient of the division is too large to fit in the destination register.
///
/// This exception does not push an error code onto the stack.
///
/// # Behavior
/// Currently, this handler panics with the interrupt stack frame.
pub extern "x86-interrupt" fn divide_error(stack_frame: InterruptStackFrame) {
    panic!("Divide Error\nRIP: {:#x}", stack_frame.instruction_pointer);
}

/// Vector 1: Debug (#DB)
///
/// Triggered by various debug conditions, such as single-step execution, instruction execution
/// breakpoints, data read/write breakpoints (watchpoints), or general detect conditions.
///
/// This exception does not push an error code onto the stack.
///
/// # Behavior
/// Prints the exception name along with the interrupt stack frame.
pub extern "x86-interrupt" fn debug(stack_frame: InterruptStackFrame) {
    debug!("Debug\nRIP: {:#x}", stack_frame.instruction_pointer);
}

/// Vector 2: Non-Maskable Interrupt (#NMI)
///
/// Handles NMIs, which are hardware interrupts that bypass the normal interrupt enable/disable
/// mechanism (e.g., `cli` / `sti`). NMIs typically signal critical hardware errors, such as memory parity
/// errors or chipset-level issues.
///
/// This exception does not push an error code onto the stack.
///
/// # Behavior
/// Currently, this handler panics to stop execution.
pub extern "x86-interrupt" fn non_maskable_interrupt(stack_frame: InterruptStackFrame) {
    panic!(
        "Non-Maskable Interrupt\nRIP: {:#x}",
        stack_frame.instruction_pointer
    );
}

/// Vector 3: Breakpoint (#BP)
///
/// Triggered by the `INT3` instruction (opcode `0xCC`), which is commonly inserted by debuggers
/// to pause execution and inspect code status.
///
/// This exception does not push an error code onto the stack.
///
/// # Behavior
/// Prints the exception name and the stack frame.
pub extern "x86-interrupt" fn breakpoint(stack_frame: InterruptStackFrame) {
    info!("Breakpoint\nRIP: {:#x}", stack_frame.instruction_pointer);
}

/// Vector 4: Overflow (#OF)
///
/// Triggered by the `INTO` instruction if the overflow flag (OF) in the `rFLAGS` register
/// is set, indicating a signed integer overflow.
///
/// This exception does not push an error code onto the stack.
///
/// # Behavior
/// Currently, this handler panics with the interrupt stack frame.
pub extern "x86-interrupt" fn overflow(stack_frame: InterruptStackFrame) {
    panic!("Overflow\nRIP: {:#x}", stack_frame.instruction_pointer);
}

/// Vector 5: Bound Range Exceeded (#BR)
///
/// Triggered by the `BOUND` instruction if the signed index operand is outside the specified bounds
/// in memory.
///
/// This exception does not push an error code onto the stack.
///
/// # Behavior
/// Currently, this handler panics with the interrupt stack frame.
pub extern "x86-interrupt" fn bound_range_exceeded(stack_frame: InterruptStackFrame) {
    panic!(
        "Bound Range Exceeded\nRIP: {:#x}",
        stack_frame.instruction_pointer
    );
}

/// Vector 6: Invalid Opcode (#UD)
///
/// Triggered when the CPU attempts to execute an invalid, undefined, or reserved opcode,
/// or when instructions have invalid operands or incorrect prefix combinations.
///
/// This exception does not push an error code onto the stack.
///
/// # Behavior
/// Currently, this handler panics with the interrupt stack frame.
pub extern "x86-interrupt" fn invalid_opcode(stack_frame: InterruptStackFrame) {
    panic!(
        "Invalid Opcode\nRIP: {:#x}",
        stack_frame.instruction_pointer
    );
}

/// Vector 7: Device Not Available (#NM)
///
/// Triggered when a floating-point (x87 FPU), MMX, or SSE instruction is executed while the EM (Emulation)
/// or TS (Task Switched) bits in the `CR0` control register are set.
///
/// This exception does not push an error code onto the stack.
///
/// # Behavior
/// Prints the exception details and the stack frame.
pub extern "x86-interrupt" fn device_not_available(stack_frame: InterruptStackFrame) {
    warn!(
        "Device Not Available\nRIP: {:#x}",
        stack_frame.instruction_pointer
    );
}

/// Vector 8: Double Fault (#DF)
///
/// Triggered when the CPU fails to call an exception handler because another exception occurred
/// during the initial delivery. This represents a critical kernel fault.
///
/// This exception pushes a dummy error code (always 0) onto the stack.
///
/// # Behavior
/// This handler diverges (`-> !`) and panics, halting the system. It runs on a dedicated stack
/// to prevent stack overflows from causing a triple fault (which would reboot the machine).
pub extern "x86-interrupt" fn double_fault(stack_frame: InterruptStackFrame, _: u64) -> ! {
    panic!("Double Fault\nRIP: {:#x}", stack_frame.instruction_pointer);
}

/// Vector 10: Invalid TSS (#TS)
///
/// Triggered when a task switch or segment register loading operation references an invalid Task State
/// Segment (TSS) or a segment descriptor with invalid properties.
///
/// This exception pushes an error code containing the selector index.
///
/// # Behavior
/// Currently, this handler panics with the error code and stack frame.
pub extern "x86-interrupt" fn invalid_tss(stack_frame: InterruptStackFrame, error_code: u64) {
    panic!(
        "Invalid TSS\nRIP: {:#x}\nError Code: {}",
        stack_frame.instruction_pointer, error_code
    );
}

/// Vector 11: Segment Not Present (#NP)
///
/// Triggered when the CPU attempts to load a segment register that references a descriptor
/// whose present bit (P) is set to 0 (indicating it is not present in memory).
///
/// This exception pushes an error code containing the selector index.
///
/// # Behavior
/// Prints the exception name, the selector/error code, and the stack frame.
pub extern "x86-interrupt" fn segment_not_present(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    warn!(
        "Segment Not Present\nRIP: {:#x}\nError Code: {}",
        stack_frame.instruction_pointer, error_code
    );
}

/// Vector 12: Stack-Segment Fault (#SS)
///
/// Triggered by stack operations (e.g., `PUSH`, `POP`, or loading `SS`) that violate segment limits,
/// or when the segment descriptor referenced by the stack segment register is marked as not present.
///
/// This exception pushes an error code containing the selector index.
///
/// # Behavior
/// Prints the exception name, the error code, and the stack frame.
pub extern "x86-interrupt" fn stack_segment_fault(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    warn!(
        "Stack Segment Fault\nRIP: {:#x}\nError Code: {}",
        stack_frame.instruction_pointer, error_code
    );
}

/// Vector 13: General Protection Fault (#GP)
///
/// Triggered by various protection violations that do not fall under other specific exceptions.
/// Examples include referencing non-canonical memory addresses, writing to read-only segments,
/// or executing privileged instructions in user mode.
///
/// This exception pushes an error code (often 0, or containing a selector index).
///
/// # Behavior
/// Currently, this handler panics with the error code and stack frame.
pub extern "x86-interrupt" fn general_protection_fault(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    panic!(
        "General Protection Fault\nRIP: {:#x}\nError Code: {}",
        stack_frame.instruction_pointer, error_code
    );
}

/// Vector 14: Page Fault (#PF)
///
/// Triggered when a virtual memory page translation is invalid or violates access permissions
/// (e.g., trying to write to a read-only page, or page not present in page tables).
///
/// This exception pushes a structured [`PageFaultErrorCode`] onto the stack.
///
/// # Behavior
/// Currently, this handler panics.
///
/// # TODO
/// Implement page fault resolution by dynamically allocating a new page.
pub extern "x86-interrupt" fn page_fault(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    // TODO: Handle page fault by allocating a new page.

    let faulting_address = Cr2::read_raw();

    panic!(
        "Page Fault\nFaulting Address: {:#x}\nRIP: {:#x}\nError Code: {:?}",
        faulting_address, stack_frame.instruction_pointer, error_code
    );
}

/// Vector 16: x87 Floating-Point Exception (#MF)
///
/// Triggered when the x87 FPU detects an unmasked floating-point error, such as numeric overflow,
/// underflow, zero division, or invalid operation.
///
/// This exception does not push an error code onto the stack.
///
/// # Behavior
/// Currently, this handler panics with the interrupt stack frame.
pub extern "x86-interrupt" fn x87_floating_point(stack_frame: InterruptStackFrame) {
    panic!(
        "X87 Floating Point\nRIP: {:#x}",
        stack_frame.instruction_pointer
    );
}

/// Vector 17: Alignment Check (#AC)
///
/// Triggered when alignment checking is enabled (in `CR0` and `rFLAGS`) and an unaligned memory access
/// occurs (e.g., reading a 4-byte value from an address that is not a multiple of 4).
///
/// This exception pushes a dummy error code (always 0) onto the stack.
///
/// # Behavior
/// Currently, this handler panics with the error code and stack frame.
pub extern "x86-interrupt" fn alignment_check(stack_frame: InterruptStackFrame, error_code: u64) {
    panic!(
        "Alignment Check\nRIP: {:#x}\nError Code: {}",
        stack_frame.instruction_pointer, error_code
    );
}

/// Vector 18: Machine Check (#MC)
///
/// Triggered by internal or bus hardware errors detected by the CPU's machine-check architecture.
///
/// This exception does not push an error code onto the stack.
///
/// # Behavior
/// This handler diverges (`-> !`) and panics, halting execution on a dedicated stack.
pub extern "x86-interrupt" fn machine_check_exception(stack_frame: InterruptStackFrame) -> ! {
    panic!(
        "Machine Check Exception\nRIP: {:#x}",
        stack_frame.instruction_pointer
    );
}

/// Vector 19: SIMD Floating-Point Exception (#XM)
///
/// Triggered when the processor detects an unmasked SSE/SIMD floating-point error (e.g., division by zero
/// or overflow).
///
/// This exception does not push an error code onto the stack.
///
/// # Behavior
/// Currently, this handler panics with the interrupt stack frame.
pub extern "x86-interrupt" fn simd_floating_point(stack_frame: InterruptStackFrame) {
    panic!(
        "SIMD Floating Point\nRIP: {:#x}",
        stack_frame.instruction_pointer
    );
}

/// Vector 20: Virtualization Exception (#VE)
///
/// Triggered by virtualization-related events, such as when EPT (Extended Page Tables) violations
/// are configured to deliver virtualization exceptions directly to the guest.
///
/// This exception does not push an error code onto the stack.
///
/// # Behavior
/// Currently, this handler panics with the interrupt stack frame.
pub extern "x86-interrupt" fn virtualization_exception(stack_frame: InterruptStackFrame) {
    panic!(
        "Virtualization Exception\nRIP: {:#x}",
        stack_frame.instruction_pointer
    );
}

/// Vector 21: Control Protection Exception (#CP)
///
/// Triggered by control-flow protection violations (associated with Intel/AMD Control-flow Enforcement
/// Technology - CET, such as shadow stack or indirect branch tracking faults).
///
/// This exception pushes an error code indicating the cause of the control protection fault.
///
/// # Behavior
/// Currently, this handler panics with the error code and stack frame.
pub extern "x86-interrupt" fn control_protection_exception(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    panic!(
        "Control Protection Exception\nRIP: {:#x}\nError Code: {}",
        stack_frame.instruction_pointer, error_code
    );
}

/// Vector 28: Hypervisor Injection Exception (#HV)
///
/// Triggered when events are injected into the guest operating system by a hypervisor.
///
/// This exception does not push an error code onto the stack.
///
/// # Behavior
/// Currently, this handler panics with the interrupt stack frame.
pub extern "x86-interrupt" fn hv_injection_exception(stack_frame: InterruptStackFrame) {
    panic!(
        "HV Injection Exception\nRIP: {:#x}",
        stack_frame.instruction_pointer
    );
}

/// Vector 29: VMM Communication Exception (#VC)
///
/// Triggered in secure nested paging (SEV-ES/SEV-SNP) guests to request services from the Hypervisor/VMM,
/// representing a non-automatic exit (NAE) event.
///
/// This exception pushes an error code.
///
/// # Behavior
/// Currently, this handler panics with the error code and stack frame.
pub extern "x86-interrupt" fn vmm_communication_exception(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    panic!(
        "VMM Communication Exception\nRIP: {:#x}\nError Code: {}",
        stack_frame.instruction_pointer, error_code
    );
}

/// Vector 30: Security Exception (#SX)
///
/// Triggered by security-related events (such as AMD Security Exception or Intel TXT events).
///
/// This exception pushes an error code.
///
/// # Behavior
/// Currently, this handler panics with the error code and stack frame.
pub extern "x86-interrupt" fn security_exception(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    panic!(
        "Security Exception\nRIP: {:#x}\nError Code: {}",
        stack_frame.instruction_pointer, error_code
    );
}

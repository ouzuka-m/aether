use x86_64::{
    registers::control::Cr2,
    structures::idt::{InterruptStackFrame, PageFaultErrorCode},
};

/// Vector 0: Divide Error (#DE)
///
/// Triggered when a division instruction (e.g., `DIV` or `IDIV`) is executed with a divisor of zero,
/// or when the quotient of the division is too large to fit in the destination register.
///
/// This exception does not push an error code onto the stack.
///
/// # Behavior
/// Currently, this handler panics.
pub extern "x86-interrupt" fn divide_error(_: InterruptStackFrame) {
    panic!("Divide Error (#DE)");
}

/// Vector 1: Debug (#DB)
///
/// Triggered by various debug conditions, such as single-step execution, instruction execution
/// breakpoints, data read/write breakpoints (watchpoints), or general detect conditions.
///
/// This exception does not push an error code onto the stack.
///
/// # Behavior
/// Currently, this handler panics.
pub extern "x86-interrupt" fn debug(_: InterruptStackFrame) {
    panic!("Debug (#DB)");
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
pub extern "x86-interrupt" fn non_maskable_interrupt(_: InterruptStackFrame) {
    panic!("Non-Maskable Interrupt (#NMI)");
}

/// Vector 3: Breakpoint (#BP)
///
/// Triggered by the `INT3` instruction (opcode `0xCC`), which is commonly inserted by debuggers
/// to pause execution and inspect code status.
///
/// This exception does not push an error code onto the stack.
///
/// # Behavior
/// Currently, this handler panics.
pub extern "x86-interrupt" fn breakpoint(_: InterruptStackFrame) {
    panic!("Breakpoint (#BP)");
}

/// Vector 4: Overflow (#OF)
///
/// Triggered by the `INTO` instruction if the overflow flag (OF) in the `rFLAGS` register
/// is set, indicating a signed integer overflow.
///
/// This exception does not push an error code onto the stack.
///
/// # Behavior
/// Currently, this handler panics.
pub extern "x86-interrupt" fn overflow(_: InterruptStackFrame) {
    panic!("Overflow (#OF)");
}

/// Vector 5: Bound Range Exceeded (#BR)
///
/// Triggered by the `BOUND` instruction if the signed index operand is outside the specified bounds
/// in memory.
///
/// This exception does not push an error code onto the stack.
///
/// # Behavior
/// Currently, this handler panics.
pub extern "x86-interrupt" fn bound_range_exceeded(_: InterruptStackFrame) {
    panic!("Bound Range Exceeded (#BR)");
}

/// Vector 6: Invalid Opcode (#UD)
///
/// Triggered when the CPU attempts to execute an invalid, undefined, or reserved opcode,
/// or when instructions have invalid operands or incorrect prefix combinations.
///
/// This exception does not push an error code onto the stack.
///
/// # Behavior
/// Currently, this handler panics.
pub extern "x86-interrupt" fn invalid_opcode(_: InterruptStackFrame) {
    panic!("Invalid Opcode (#UD)");
}

/// Vector 7: Device Not Available (#NM)
///
/// Triggered when a floating-point (x87 FPU), MMX, or SSE instruction is executed while the EM (Emulation)
/// or TS (Task Switched) bits in the `CR0` control register are set.
///
/// This exception does not push an error code onto the stack.
///
/// # Behavior
/// Currently, this handler panics.
pub extern "x86-interrupt" fn device_not_available(_: InterruptStackFrame) {
    panic!("Device Not Available (#NM)");
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
pub extern "x86-interrupt" fn double_fault(_: InterruptStackFrame, _: u64) -> ! {
    panic!("Double Fault (#DF)");
}

/// Vector 10: Invalid TSS (#TS)
///
/// Triggered when a task switch or segment register loading operation references an invalid Task State
/// Segment (TSS) or a segment descriptor with invalid properties.
///
/// This exception pushes an error code containing the selector index.
///
/// # Behavior
/// Currently, this handler panics.
pub extern "x86-interrupt" fn invalid_tss(_: InterruptStackFrame, _: u64) {
    panic!("Invalid TSS (#TS)");
}

/// Vector 11: Segment Not Present (#NP)
///
/// Triggered when the CPU attempts to load a segment register that references a descriptor
/// whose present bit (P) is set to 0 (indicating it is not present in memory).
///
/// This exception pushes an error code containing the selector index.
///
/// # Behavior
/// Currently, this handler panics.
pub extern "x86-interrupt" fn segment_not_present(_: InterruptStackFrame, _: u64) {
    panic!("Segment Not Present (#NP)");
}

/// Vector 12: Stack-Segment Fault (#SS)
///
/// Triggered by stack operations (e.g., `PUSH`, `POP`, or loading `SS`) that violate segment limits,
/// or when the segment descriptor referenced by the stack segment register is marked as not present.
///
/// This exception pushes an error code containing the selector index.
///
/// # Behavior
/// Currently, this handler panics.
pub extern "x86-interrupt" fn stack_segment_fault(_: InterruptStackFrame, _: u64) {
    panic!("Stack Segment Fault (#SS)");
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
/// Currently, this handler panics.
pub extern "x86-interrupt" fn general_protection_fault(_: InterruptStackFrame, _: u64) {
    panic!("General Protection Fault (#GP)");
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
pub extern "x86-interrupt" fn page_fault(_: InterruptStackFrame, error: PageFaultErrorCode) {
    // TODO: Handle page fault by allocating a new page.
    let faulting_address = Cr2::read_raw();

    panic!(
        "Page Fault (#PF) at {faulting_address:#x} with error code {:#x}",
        error.bits()
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
/// Currently, this handler panics.
pub extern "x86-interrupt" fn x87_floating_point(_: InterruptStackFrame) {
    panic!("x87 Floating-Point Exception (#MF)");
}

/// Vector 17: Alignment Check (#AC)
///
/// Triggered when alignment checking is enabled (in `CR0` and `rFLAGS`) and an unaligned memory access
/// occurs (e.g., reading a 4-byte value from an address that is not a multiple of 4).
///
/// This exception pushes a dummy error code (always 0) onto the stack.
///
/// # Behavior
/// Currently, this handler panics.
pub extern "x86-interrupt" fn alignment_check(_: InterruptStackFrame, _: u64) {
    panic!("Alignment Check (#AC)");
}

/// Vector 18: Machine Check (#MC)
///
/// Triggered by internal or bus hardware errors detected by the CPU's machine-check architecture.
///
/// This exception does not push an error code onto the stack.
///
/// # Behavior
/// This handler diverges (`-> !`) and panics, halting execution on a dedicated stack.
pub extern "x86-interrupt" fn machine_check_exception(_: InterruptStackFrame) -> ! {
    panic!("Machine Check Exception (#MC)");
}

/// Vector 19: SIMD Floating-Point Exception (#XM)
///
/// Triggered when the processor detects an unmasked SSE/SIMD floating-point error (e.g., division by zero
/// or overflow).
///
/// This exception does not push an error code onto the stack.
///
/// # Behavior
/// Currently, this handler panics.
pub extern "x86-interrupt" fn simd_floating_point(_: InterruptStackFrame) {
    panic!("SIMD Floating-Point Exception (#XM)");
}

/// Vector 20: Virtualization Exception (#VE)
///
/// Triggered by virtualization-related events, such as when EPT (Extended Page Tables) violations
/// are configured to deliver virtualization exceptions directly to the guest.
///
/// This exception does not push an error code onto the stack.
///
/// # Behavior
/// Currently, this handler panics.
pub extern "x86-interrupt" fn virtualization_exception(_: InterruptStackFrame) {
    panic!("Virtualization Exception (#VE)");
}

/// Vector 21: Control Protection Exception (#CP)
///
/// Triggered by control-flow protection violations (associated with Intel/AMD Control-flow Enforcement
/// Technology - CET, such as shadow stack or indirect branch tracking faults).
///
/// This exception pushes an error code indicating the cause of the control protection fault.
///
/// # Behavior
/// Currently, this handler panics.
pub extern "x86-interrupt" fn control_protection_exception(_: InterruptStackFrame, _: u64) {
    panic!("Control Protection Exception (#CP)");
}

/// Vector 28: Hypervisor Injection Exception (#HV)
///
/// Triggered when events are injected into the guest operating system by a hypervisor.
///
/// This exception does not push an error code onto the stack.
///
/// # Behavior
/// Currently, this handler panics.
pub extern "x86-interrupt" fn hv_injection_exception(_: InterruptStackFrame) {
    panic!("Hypervisor Injection Exception (#HV)");
}

/// Vector 29: VMM Communication Exception (#VC)
///
/// Triggered in secure nested paging (SEV-ES/SEV-SNP) guests to request services from the Hypervisor/VMM,
/// representing a non-automatic exit (NAE) event.
///
/// This exception pushes an error code.
///
/// # Behavior
/// Currently, this handler panics.
pub extern "x86-interrupt" fn vmm_communication_exception(_: InterruptStackFrame, _: u64) {
    panic!("VMM Communication Exception (#VC)");
}

/// Vector 30: Security Exception (#SX)
///
/// Triggered by security-related events (such as AMD Security Exception or Intel TXT events).
///
/// This exception pushes an error code.
///
/// # Behavior
/// Currently, this handler panics.
pub extern "x86-interrupt" fn security_exception(_: InterruptStackFrame, _: u64) {
    panic!("Security Exception (#SX)");
}

use x86_64::structures::idt::InterruptStackFrame;

pub extern "x86-interrupt" fn spurious_vector_interrupt(_: InterruptStackFrame) {
    // IGNORED
}

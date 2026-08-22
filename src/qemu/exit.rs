/// Exits the program successfully.
pub fn success() {
    unsafe {
        core::arch::asm!(
            "out dx, eax",
            in("dx") 0xF4u16,
            in("eax") 0x0u32
        )
    }
}

/// Exits the program with a failure code.
pub fn failure() {
    unsafe {
        core::arch::asm!(
            "out dx, eax",
            in("dx") 0xF4u16,
            in("eax") 0x1u32
        )
    }
}

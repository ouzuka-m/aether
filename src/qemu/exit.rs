//! QEMU debug-exit device interface.
//!
//! Writes to I/O port `0xF4` which is mapped to the `isa-debug-exit`
//! device in QEMU. The exit code seen by the host is `(value << 1) | 1`,
//! so writing `0x00` produces host exit code `1` (success by convention)
//! and writing `0x01` produces host exit code `3` (failure).
//!
//! These functions are no-ops on real hardware (port `0xF4` is unused).

/// Signals successful execution to the QEMU debug-exit device.
///
/// Writes `0x00` to I/O port `0xF4`, causing QEMU to exit with code `1`.
pub fn success() {
    // SAFETY: Writing to I/O port 0xF4 targets the QEMU isa-debug-exit
    // device. On real hardware this port is unused and the write is
    // harmless.
    unsafe {
        core::arch::asm!(
            "out dx, eax",
            in("dx") 0xF4u16,
            in("eax") 0x0u32
        )
    }
}

/// Signals a failure to the QEMU debug-exit device.
///
/// Writes `0x01` to I/O port `0xF4`, causing QEMU to exit with code `3`.
pub fn failure() {
    // SAFETY: Same as `success` — writing to the QEMU debug-exit port.
    unsafe {
        core::arch::asm!(
            "out dx, eax",
            in("dx") 0xF4u16,
            in("eax") 0x1u32
        )
    }
}

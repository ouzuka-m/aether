//! Default `RFLAGS` register value for new tasks/threads.

/// Default initial value for the x86_64 `RFLAGS` register.
///
/// Sets bit 1 (reserved, always 1) and bit 9 (Interrupt Flag / `IF`),
/// enabling hardware interrupts when the task starts.
pub const RFLAGS: u64 = 0x202;

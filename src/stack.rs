//! # Stack Constants and IST Indices
//!
//! Defines the default allocation size for kernel exception stacks and specifies
//! the Interrupt Stack Table (IST) index mappings used by the TSS and IDT.

/// Size of each dedicated exception stack in bytes (32 KiB).
pub const STACK_SIZE: usize = 4096 * 8; // 32 KiB

/// Internal wrapper representing a 16-byte aligned stack memory buffer.
#[repr(C, align(16))]
pub struct Stack([u8; STACK_SIZE]);

pub static DF_STACK: Stack = Stack([0; STACK_SIZE]);
pub static NMI_STACK: Stack = Stack([0; STACK_SIZE]);
pub static MCE_STACK: Stack = Stack([0; STACK_SIZE]);

/// IST index for the Double Fault (#DF) exception stack.
pub const DF_INDEX: u16 = 0;

/// IST index for the Non-Maskable Interrupt (#NMI) stack.
pub const NMI_INDEX: u16 = 1;

/// IST index for the Machine Check Exception (#MC) stack.
pub const MCE_INDEX: u16 = 2;

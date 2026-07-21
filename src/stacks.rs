#![allow(unused)]

pub const STACK_SIZE: usize = 4096 * 8; // 32 KiB

// Double Fault
pub const DF_INDEX: u16 = 0;

// Non-Maskable Interrupt
pub const NMI_INDEX: u16 = 1;

// Machine Check
pub const MCE_INDEX: u16 = 2;

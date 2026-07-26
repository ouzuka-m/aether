//! # Interrupt Handling Subsystem
//!
//! Provides the Interrupt Descriptor Table (IDT) configuration as well as exception
//! and hardware interrupt service routines (ISRs).

pub mod handlers;
pub mod idt;

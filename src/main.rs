//! # Aether Kernel Entry Point
//!
//! This module serves as the primary entry point for the Aether operating system kernel.
//! It disables interrupts during early boot, initializes essential core subsystems
//! (GDT, IDT, page frame allocation, virtual memory mapping, global heap allocator, and ACPI),
//! and transitions the CPU into an power-efficient idle loop waiting for interrupts.

#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;

mod acpi;
mod address;
mod gdt;
mod interrupts;
mod log;
mod memory;
mod stacks;
mod tss;

use core::panic::PanicInfo;
use x86_64::instructions::{self};

use interrupts::idt;
use memory::{frame_allocator, heap_allocator, mapper};

/// Kernel entry point called by the Limine bootloader.
///
/// This function performs early kernel setup in sequential phases:
/// 1. Disables CPU interrupts to ensure atomic subsystem initialization.
/// 2. Configures the Global Descriptor Table (GDT) and Task State Segment (TSS).
/// 3. Populates and loads the Interrupt Descriptor Table (IDT).
/// 4. Initializes page table mapping and physical frame allocation.
/// 5. Initializes the global dynamic heap allocator.
/// 6. Configures the ACPI subsystem, legacy PIC disabling, and APIC/IOAPIC routing.
/// 7. Enables CPU interrupts and enters an infinite `hlt` loop to wait for hardware events.
///
/// # Divergence
/// This function never returns (`-> !`).
#[unsafe(no_mangle)]
extern "C" fn _start() -> ! {
    // Step 1: Disable CPU hardware interrupts during critical boot sequence
    x86_64::instructions::interrupts::disable();

    info!("Aether Kernel starting...");

    // Step 2: Load Global Descriptor Table (GDT) and Task State Segment (TSS)
    gdt::init();

    // Step 3: Load Interrupt Descriptor Table (IDT) with exception & IRQ handlers
    idt::init();

    // Step 4: Initialize virtual memory mapper and physical frame allocator
    let mut mapper = mapper::init();
    let mut frame_allocator = frame_allocator::init();

    // Step 5: Set up the global buddy system heap allocator
    heap_allocator::init(&mut mapper, &mut frame_allocator);

    // Step 6: Parse ACPI tables, disable 8259 PIC, and enable Local/IO APIC
    acpi::init();

    info!("Kernel initialized successfully. Entering idle loop.");

    // Step 7: Enable CPU interrupts and enter low-power idle loop
    loop {
        // Re-enable interrupts and halt CPU until next hardware interrupt arrives
        x86_64::instructions::interrupts::enable_and_hlt();
    }
}

/// Custom panic handler for bare-metal `#![no_std]` environment.
///
/// Invoked automatically when a Rust `panic!` macro is triggered anywhere in the kernel.
/// It disables CPU interrupts, logs the panic information over the UART serial line,
/// and permanently halts the CPU in a `hlt` loop.
///
/// # Parameters
/// - `info`: Information regarding the location and message of the panic.
///
/// # Divergence
/// This function never returns (`-> !`).
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Disable interrupts to prevent further interrupt processing during a fatal panic
    x86_64::instructions::interrupts::disable();

    // Output formatted panic details to serial logger
    error!("{}", info);

    // Enter infinite halt loop
    loop {
        instructions::hlt();
    }
}


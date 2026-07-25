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

#[unsafe(no_mangle)]
extern "C" fn _start() -> ! {
    // Disable the CPU interrupts
    x86_64::instructions::interrupts::disable();

    info!("Aether Kernel starting...");

    // Load Global Descriptor Table (GDT)
    gdt::init();

    // Load Interrupt Descriptor Table (IDT)
    idt::init();

    // Set up the allocator
    let mut mapper = mapper::init();
    let mut frame_allocator = frame_allocator::init();

    heap_allocator::init(&mut mapper, &mut frame_allocator);

    acpi::init();

    info!("Kernel initialized successfully. Entering idle loop.");

    loop {
        // Ready to accept interrupts & quickly enter the sleep mode
        x86_64::instructions::interrupts::enable_and_hlt();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    x86_64::instructions::interrupts::disable();

    error!("{}", info);

    loop {
        instructions::hlt();
    }
}

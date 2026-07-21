#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;

mod acpi;
mod apic;
mod gdt;
mod hhdm;
mod interrupts;
mod memory;
mod pic8259;
mod serial;
mod stacks;
mod tss;

use core::panic::PanicInfo;
use x86_64::instructions;

use interrupts::idt;
use memory::{frame_allocator, heap_allocator, mapper};

#[unsafe(no_mangle)]
extern "C" fn _start() -> ! {
    // Disable the CPU interrupts
    x86_64::instructions::interrupts::disable();

    // Load Global Descriptor Table (GDT)
    gdt::init();

    // Load Interrupt Descriptor Table (IDT)
    idt::init();

    // Disable old Intel 8259 Programmable Interrupt Controller (PIC)
    pic8259::disable();

    // Loat the Higher-Half Direct Map (HHDM)
    let hhdm = hhdm::get();

    // Set up the allocator
    let mut mapper = mapper::init(hhdm);
    let mut frame_allocator = frame_allocator::init();
    heap_allocator::init(&mut mapper, &mut frame_allocator);

    let acpi = acpi::tables::init(hhdm.as_u64() as usize);

    apic::enable(&acpi, hhdm.as_u64());

    loop {
        // Ready to accept interrupts & quickly enter the sleep mode
        x86_64::instructions::interrupts::enable_and_hlt();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {
        instructions::hlt();
    }
}

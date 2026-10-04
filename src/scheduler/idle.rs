pub fn idle() -> ! {
    loop {
        x86_64::instructions::interrupts::enable_and_hlt();
    }
}

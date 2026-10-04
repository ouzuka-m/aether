use crate::scheduler::round_robin::SCHEDULER;

pub fn exit() -> ! {
    x86_64::instructions::interrupts::without_interrupts(|| {
        SCHEDULER.lock().retire_current();
    });

    loop {
        x86_64::instructions::interrupts::enable_and_hlt();
    }
}

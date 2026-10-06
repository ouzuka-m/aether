use x86_64::instructions::interrupts;

use crate::scheduler::round_robin::SCHEDULER;

pub fn exit() -> ! {
    interrupts::without_interrupts(|| {
        SCHEDULER.lock().retire_current();
    });

    loop {
        interrupts::enable_and_hlt();
    }
}

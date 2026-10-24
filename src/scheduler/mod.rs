pub mod context;
pub mod exit;
pub mod idle;
pub mod rflags;
pub mod round_robin;
pub mod state;
pub mod switch;
pub mod task;

use spin::{lazylock::LazyLock, mutex::Mutex};

use self::{round_robin::RoundRobin, switch::Switch};

const MAX_TASKS: usize = 10;

pub static SCHEDULER: LazyLock<Mutex<RoundRobin<MAX_TASKS>>> =
    LazyLock::new(|| Mutex::new(RoundRobin::new()));

pub fn schedule() {
    let switch = SCHEDULER.lock().prepare();

    unsafe {
        match switch {
            Switch::Launch(ctx) => context::launch(ctx),
            Switch::Swap(old, new) => context::switch(old, new),
            _ => {}
        }
    }
}

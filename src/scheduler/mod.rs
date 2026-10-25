pub mod context;
pub mod error;
pub mod exit;
pub mod idle;
pub mod rflags;
pub mod round_robin;
pub mod state;
pub mod switch;
pub mod task;

use spin::{lazylock::LazyLock, mutex::Mutex};

use self::{round_robin::RoundRobin, switch::Switch};

pub static SCHEDULER: LazyLock<Mutex<RoundRobin<255>>> =
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

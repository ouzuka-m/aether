use crate::scheduler::{round_robin::SCHEDULER, switch::Switch};

pub mod context;
pub mod exit;
pub mod idle;
pub mod round_robin;
pub mod state;
pub mod switch;
pub mod task;

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

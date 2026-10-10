pub mod context;
pub mod exit;
pub mod idle;
pub mod round_robin;
pub mod state;
pub mod switch;
pub mod task;

use alloc::collections::VecDeque;
use spin::{lazylock::LazyLock, mutex::Mutex};

use self::{context::Context, round_robin::RoundRobin, state::State, switch::Switch, task::Task};

use crate::memory;

pub static SCHEDULER: LazyLock<Mutex<RoundRobin>> = LazyLock::new(|| {
    let mut tasks = VecDeque::with_capacity(5);

    let stack = memory::stack::alloc_stack();
    let mut rsp = stack.top;

    rsp -= 8;
    unsafe {
        *(rsp.as_mut_ptr()) = (idle::idle as fn() -> !) as usize;
    }

    let context = Context::new_rsp(rsp.as_u64());

    let state = State::Ready;

    tasks.push_back(Task {
        state,
        context,
        stack,
    });

    Mutex::new(RoundRobin::new(tasks))
});

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

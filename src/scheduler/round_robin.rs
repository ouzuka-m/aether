use alloc::collections::VecDeque;

use crate::{
    memory::stack::Stack,
    scheduler::{self, context::Context, rflags::RFLAGS, state::State, switch::Switch, task::Task},
};

#[derive(Debug)]
pub struct RoundRobin {
    current: Option<usize>,
    tasks: VecDeque<Task>,
}

impl RoundRobin {
    pub fn new() -> Self {
        Self {
            current: None,
            tasks: VecDeque::with_capacity(5),
        }
    }

    pub fn retire_current(&mut self) {
        if let Some(current) = self.current {
            self.tasks[current].state = State::Dead;
        }
    }

    #[allow(dead_code)]
    pub fn spawn(&mut self, entry: fn()) {
        let stack = Stack::alloc();
        let mut rsp = stack.top();

        rsp -= 8;
        unsafe {
            *(rsp.as_mut_ptr()) = (scheduler::exit::exit as fn() -> !) as usize;
        }

        rsp -= 8;
        unsafe {
            *(rsp.as_mut_ptr()) = entry as usize;
        }

        rsp -= 8;
        unsafe {
            *(rsp.as_mut_ptr()) = RFLAGS as usize;
        }

        let context = Context::new_rsp(rsp.as_u64());

        let state = State::Ready;

        self.tasks.push_back(Task {
            state,
            context,
            stack,
        });
    }

    pub fn spawn_no_exit(&mut self, entry: fn() -> !) {
        let stack = Stack::alloc();
        let mut rsp = stack.top();

        rsp -= 8;
        unsafe {
            *(rsp.as_mut_ptr()) = entry as usize;
        }

        rsp -= 8;
        unsafe {
            *(rsp.as_mut_ptr()) = RFLAGS as usize;
        }

        let context = Context::new_rsp(rsp.as_u64());

        let state = State::Ready;

        self.tasks.push_back(Task {
            state,
            context,
            stack,
        });
    }

    pub fn prepare(&mut self) -> Switch {
        match self.current {
            None => {
                self.tasks[0].state = State::Running;
                self.current = Some(0);

                Switch::Launch(&self.tasks[0].context as *const Context)
            }

            Some(current) => {
                for i in 1..=self.tasks.len() {
                    let next = (current + i) % self.tasks.len();

                    if self.tasks[next].state == State::Dead {
                        continue;
                    }

                    if next == current {
                        return Switch::None;
                    }

                    if self.tasks[current].state == State::Running {
                        self.tasks[current].state = State::Ready;
                    }

                    self.tasks[next].state = State::Running;
                    self.current = Some(next);

                    let old_ctx = &mut self.tasks[current].context as *mut Context;

                    let new_ctx = &self.tasks[next].context as *const Context;

                    return Switch::Swap(old_ctx, new_ctx);
                }

                Switch::None
            }
        }
    }
}

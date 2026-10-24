use crate::{
    memory::stack::Stack,
    scheduler::{context::Context, state::State},
};

#[derive(Debug)]
pub struct Task {
    pub state: State,
    pub context: Context,
    pub stack: Stack,
}

impl Task {
    pub const fn new(context: Context, stack: Stack) -> Self {
        Self {
            state: State::Ready,
            context,
            stack,
        }
    }

    pub fn deallocate_stack(&self) {
        self.stack.deallocate();
    }
}

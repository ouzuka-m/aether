use crate::{
    memory::stack::Stack,
    scheduler::{context::Context, state::State},
};

#[derive(Debug)]
pub struct Task {
    pub state: State,
    pub context: Context,

    #[allow(dead_code)]
    pub stack: Stack,
}

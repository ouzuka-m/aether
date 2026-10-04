use crate::scheduler::context::Context;

#[derive(Debug)]
pub enum Switch {
    None,
    Launch(*const Context),
    Swap(*mut Context, *const Context),
}

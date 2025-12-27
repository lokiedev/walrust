use std::path::PathBuf;

pub enum Action {
    Start, // app start action
    Select(Option<PathBuf>),
    Next,
    Previous,
    Quit,
}

#[derive(PartialEq, Eq)]
pub enum ActionState {
    Consumed,
    NotConsumed,
}

impl ActionState {
    pub fn is_consumed(&self) -> bool {
        *self == ActionState::Consumed
    }
}

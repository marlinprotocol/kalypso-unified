use serde::{Deserialize, Serialize};

#[derive(Default, PartialEq, Eq, PartialOrd, Serialize, Deserialize, Hash, Copy, Clone)]
pub enum GeneratorState {
    #[default]
    Null,
    Joined,
    NoComputeAvailable,
    Wip,
    RequestedForExit,
    PendingConfirmation, // Not present in contracts
}

impl std::fmt::Debug for GeneratorState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value_str = match self {
            GeneratorState::Null => "This is a Null state",
            GeneratorState::Joined => "The generator has joined",
            GeneratorState::NoComputeAvailable => "The generator has no compute available",
            GeneratorState::Wip => "Work in progress state",
            GeneratorState::RequestedForExit => "The generator requested for exit",
            GeneratorState::PendingConfirmation => "The generator is selected for task",
        };
        write!(f, "{}", value_str)
    }
}

#[allow(unused)]
pub fn get_generator_state(state: u8) -> GeneratorState {
    match state {
        0 => GeneratorState::Null,
        1 => GeneratorState::Joined,
        2 => GeneratorState::NoComputeAvailable,
        3 => GeneratorState::Wip,
        4 => GeneratorState::RequestedForExit,
        _ => GeneratorState::Null,
    }
}

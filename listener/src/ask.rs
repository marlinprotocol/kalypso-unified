use serde::{Deserialize, Serialize};

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Serialize, Deserialize, Hash)]
pub enum AskState {
    #[default]
    Null,
    Create,
    UnAssigned,
    Assigned,
    Complete,
    DeadlineCrossed,
    // added latter and not in contract
    InvalidSecret,
}

impl std::fmt::Debug for AskState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value_str = match self {
            AskState::Null => "This is a Null state",
            AskState::Create => "The ask was created",
            AskState::UnAssigned => "The ask is unassigned",
            AskState::Assigned => "The ask is assigned",
            AskState::Complete => "The ask is complete",
            AskState::DeadlineCrossed => "The ask deadline has been crossed",
            AskState::InvalidSecret => "The secret for the ask is invalid",
        };
        write!(f, "{}", value_str)
    }
}

pub fn get_ask_state(state: u8) -> AskState {
    // enum values in contract

    // NULL,
    // CREATE,
    // UNASSIGNED,
    // ASSIGNED,
    // COMPLETE,
    // DEADLINE_CROSSED
    match state {
        0 => AskState::Null,
        1 => AskState::Create,
        2 => AskState::UnAssigned,
        3 => AskState::Assigned,
        4 => AskState::Complete,
        5 => AskState::DeadlineCrossed,
        // added latter and not in contract
        6 => AskState::InvalidSecret,
        _ => AskState::Null,
    }
}

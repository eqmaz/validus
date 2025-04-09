use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TradeAction {
    Submit,
    Approve,
    Cancel,
    Update,
    SendToExecute,
    Book,
}

impl TradeAction {
    pub fn is_irreversible(self) -> bool {
        matches!(self, TradeAction::SendToExecute | TradeAction::Book)
    }
}

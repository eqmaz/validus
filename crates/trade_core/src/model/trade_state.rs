use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TradeState {
    Draft,
    PendingApproval,
    NeedsReapproval,
    Approved,
    SentToCounterparty,
    Executed,
    Cancelled,
}

impl TradeState {
    pub fn is_final(self) -> bool {
        matches!(self, TradeState::Executed | TradeState::Cancelled)
    }
}

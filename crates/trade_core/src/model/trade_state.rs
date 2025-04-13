use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

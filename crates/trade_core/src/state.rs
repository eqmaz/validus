use crate::errors::ValidationError;
use crate::model::{TradeAction, TradeState};

use TradeAction::*;
use TradeState::*;

/// Simple state machine for trade transitions
/// TODO - Could potentially move the allowed transitions to a config file if it's likely to change
///  Probably best practice so it's not hard coded. Anyway it's here for now
#[derive(Default)]
pub struct StateMachine;

impl StateMachine {

    /// Checks if a transition is valid, returning a bool
    /// Do not confuse with next_state which actually returns the next state for a given action
    pub fn can_transition(&self, from: TradeState, to: TradeState) -> bool {
        match (from, to) {
            // New trade creation or "draft" = no state change
            (Draft, Draft) => true,

            // Submitting a draft goes to pending approval
            (Draft, PendingApproval) => true,

            // UPDATING a pending-approval trade = no stage change
            (PendingApproval, PendingApproval) => true,

            // Pending approval state can go to Approved, Cancelled or NeedsApproval
            (PendingApproval, Approved | Cancelled | NeedsReapproval) => true,

            // After update, original requester can re-approve or cancel it
            (NeedsReapproval, Approved | Cancelled) => true,

            // Approved trade can be sent to counterparty or cancelled
            (Approved, SentToCounterparty | Cancelled) => true,

            // After sending, trade can be executed or cancelled
            (SentToCounterparty, Executed | Cancelled) => true,

            // Anything else is not supported
            _ => false,
        }
    }

    /// Provides the next state for a given action and current state
    /// Responds with an error if the requested action is not valid for the current state
    pub fn next_state(&self, action: TradeAction, from_state: TradeState, ) -> Result<TradeState, ValidationError> {
        match (action, from_state) {
            // ----------------- Happy paths -------------------------------------------------------

            // User submits draft -> moves to "pending approval"
            (Submit, Draft) => Ok(PendingApproval),

            // Trade approved -> Moves to "Approved"
            (Approve, PendingApproval) => Ok(Approved),
            // Original REQUESTER re-approves trade -> moves to "approved"
            (Approve, NeedsReapproval) => Ok(Approved),

            // Trade gets updated -> needs re-approval
            // We allow updates from Draft, PendingApproval, and POSSIBLY also NeedsReapproval
            // Debatable whether update is allowed from "cancelled"
            (Update, Draft | PendingApproval | NeedsReapproval ) => Ok(NeedsReapproval),

            // Approved trade sent to counterparty -> "SentToCounterparty"
            (SendToExecute, Approved) => Ok(SentToCounterparty),

            // Trade executed (confirmation) -> book it
            (Book, SentToCounterparty) => Ok(Executed),

            // Cancel allowed from active states,
            // - possibly including SentToCounterparty (on a best-effort basis)
            //   but definitely not including Executed or Cancelled
            (Cancel, Draft | PendingApproval | NeedsReapproval | Approved) => Ok(Cancelled),
            (Cancel, SentToCounterparty) => Ok(Cancelled), // TODO - To be discussed

            // --------------- Unhappy paths / No-Ops ----------------------------------------------

            // Cannot update when sent to counterparty, executed - possibly or cancelled
            // Could be covered by fallback anyway, but just in case we need a different error
            (Update, SentToCounterparty | Executed | Cancelled) => {
                Err(ValidationError::InvalidTransition(from_state, from_state))
            }

            // No action allowed from "final" states
            (_, Executed | Cancelled) => Err(ValidationError::AlreadyFinal(from_state)),

            // Catch-all for anything not explicitly supported above
            _ => Err(ValidationError::InvalidTransition(from_state, Draft)),
        }
    }
}

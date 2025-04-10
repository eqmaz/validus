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

            // Updating a draft can go to needs re-approval
            (Draft, NeedsReapproval) => true,

            // UPDATING a pending-approval trade = no stage change
            (PendingApproval, PendingApproval) => true,

            // Pending approval state can go to Approved, Cancelled or NeedsApproval
            (PendingApproval, Approved | Cancelled | NeedsReapproval) => true,

            // After update, original requester can re-approve or cancel it
            (NeedsReapproval, Approved | Cancelled) => true,

            // Approved trade can be sent to counterparty, cancelled, or re-approved if updated
            (Approved, SentToCounterparty | Cancelled | NeedsReapproval) => true,

            // After sending, trade can be executed or cancelled
            (SentToCounterparty, Executed | Cancelled) => true,

            // Anything else is not supported
            _ => false,
        }
    }

    /// Provides the next state for a given action and current state
    /// Responds with an error if the requested action is not valid for the current state
    pub fn next_state(&self, action: TradeAction, from_state: TradeState) -> Result<TradeState, ValidationError> {
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
            (Update, Draft | PendingApproval | NeedsReapproval) => Ok(NeedsReapproval),

            // Approved trade sent to counterparty -> "SentToCounterparty"
            (SendToExecute, Approved) => Ok(SentToCounterparty),

            // Trade executed (confirmation) -> book it
            (Book, SentToCounterparty) => Ok(Executed),

            // Cancel allowed from active state,
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

            // Can't cancel a cancelled trade
            (Cancel, Cancelled) => Err(ValidationError::AlreadyFinal(from_state)),

            // No action allowed from "final" state
            (_, Executed | Cancelled) => Err(ValidationError::AlreadyFinal(from_state)),

            // Catch-all for anything not explicitly supported above
            _ => Err(ValidationError::InvalidTransition(from_state, from_state)),
        }
    }
}

// = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =
// Unit tests for state machine
// = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =
#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::ValidationError;

    fn sm() -> StateMachine {
        StateMachine::default()
    }

    // ---------------------------------------
    // Basic can_transition() sanity checks
    // ---------------------------------------

    #[test]
    fn test_can_transition_valid() {
        // These transitions are explicitly allowed by the state machine rules
        assert!(sm().can_transition(Draft, PendingApproval));
        assert!(sm().can_transition(PendingApproval, Approved));
        assert!(sm().can_transition(PendingApproval, PendingApproval)); // allowed if updating
        assert!(sm().can_transition(NeedsReapproval, Approved));
        assert!(sm().can_transition(Approved, SentToCounterparty));
        assert!(sm().can_transition(SentToCounterparty, Executed));
    }

    #[test]
    fn test_can_transition_invalid() {
        // These transitions are invalid and should be rejected
        assert!(!sm().can_transition(Executed, Draft)); // Final to Draft
        assert!(!sm().can_transition(Cancelled, Approved)); // Final to active

        // Same states
        assert!(!sm().can_transition(Cancelled, Cancelled)); // Can't cancel a cancelled trade
        assert!(!sm().can_transition(SentToCounterparty, SentToCounterparty)); // Final to active
        assert!(!sm().can_transition(Executed, Executed)); // Pending to final
    }

    // - - - - - - - - - - - - - - - - - - - - - - - -  - - - - - - - - - - - -  - - - - - - - - - -
    // HAPPY PATH — Valid transitions
    // - - - - - - - - - - - - - - - - - - - - - - - -  - - - - - - - - - - - -  - - - - - - - - - -

    #[test]
    fn test_next_state_submit_draft() {
        // Draft → Submit → PendingApproval
        let result = sm().next_state(Submit, Draft);
        assert_eq!(result.unwrap(), PendingApproval);
    }

    #[test]
    fn test_next_state_approve_pending() {
        // PendingApproval → Approve → Approved
        let result = sm().next_state(Approve, PendingApproval);
        assert_eq!(result.unwrap(), Approved);
    }

    #[test]
    fn test_next_state_update_pending() {
        // PendingApproval → Update → NeedsReapproval
        let result = sm().next_state(Update, PendingApproval);
        assert_eq!(result.unwrap(), NeedsReapproval);
    }

    #[test]
    fn test_next_state_send_to_execute() {
        // Approved → SendToExecute → SentToCounterparty
        let result = sm().next_state(SendToExecute, Approved);
        assert_eq!(result.unwrap(), SentToCounterparty);
    }

    #[test]
    fn test_next_state_book_trade() {
        // SentToCounterparty → Book → Executed
        let result = sm().next_state(Book, SentToCounterparty);
        assert_eq!(result.unwrap(), Executed);
    }

    #[test]
    fn test_next_state_cancel_pending() {
        // PendingApproval → Cancel → Cancelled
        let result = sm().next_state(Cancel, PendingApproval);
        assert_eq!(result.unwrap(), Cancelled);
    }

    #[test]
    fn test_approve_from_needs_reapproval() {
        // NeedsReapproval → Approve → Approved
        let result = sm().next_state(Approve, NeedsReapproval);
        assert_eq!(result.unwrap(), Approved);
    }

    #[test]
    fn test_update_from_draft() {
        // Draft → Update → NeedsReapproval
        let result = sm().next_state(Update, Draft);
        assert_eq!(result.unwrap(), NeedsReapproval);
    }

    #[test]
    fn test_cancel_from_needs_reapproval() {
        // NeedsReapproval → Cancel → Cancelled
        let result = sm().next_state(Cancel, NeedsReapproval);
        assert_eq!(result.unwrap(), Cancelled);
    }

    #[test]
    fn test_cancel_from_approved() {
        // Approved → Cancel → Cancelled
        let result = sm().next_state(Cancel, Approved);
        assert_eq!(result.unwrap(), Cancelled);
    }

    // - - - - - - - - - - - - - - - - - - - - - - - -  - - - - - - - - - - - -  - - - - - - - - - -
    // SAD PATH TESTS — Invalid / disallowed transitions
    // - - - - - - - - - - - - - - - - - - - - - - - -  - - - - - - - - - - - -  - - - - - - - - - -

    #[test]
    fn test_next_state_invalid_update_executed() {
        // Cannot update an already executed trade
        let result = sm().next_state(Update, Executed);
        assert_eq!(
            result.unwrap_err(),
            ValidationError::InvalidTransition(Executed, Executed)
        );
    }

    #[test]
    fn test_next_state_cancel_executed() {
        // Cannot cancel an already executed trade — it's final
        let result = sm().next_state(Cancel, Executed);
        assert_eq!(
            result.unwrap_err(),
            ValidationError::AlreadyFinal(Executed)
        );
    }

    #[test]
    fn test_next_state_unknown_transition() {
        // Trying to send a draft trade to execution is invalid
        let result = sm().next_state(SendToExecute, Draft);
        assert_eq!(
            result.unwrap_err(),
            ValidationError::InvalidTransition(Draft, Draft)
        );
    }

    #[test]
    fn test_cancel_cancel_not_allowed() {
        // Cannot cancel a cancelled trade
        let result = sm().next_state(Cancel, Cancelled);
        assert_eq!(
            result.unwrap_err(),
            ValidationError::AlreadyFinal(Cancelled)
        );
    }

    #[test]
    fn test_submit_not_allowed_from_needs_reapproval() {
        // Cannot submit from NeedsReapproval state
        let result = sm().next_state(Submit, NeedsReapproval);
        assert!(matches!(result, Err(ValidationError::InvalidTransition(NeedsReapproval, _))));
    }
}

use app_core::config::config_int;
use app_core::AppError;
use serde_json::json;
use std::sync::{Arc, Mutex};

use crate::errors::{ErrCodes, ValidationError};
use crate::model::TradeState::NeedsReapproval;
use crate::model::*;
use crate::snowflake::SnowflakeIdGenerator;
use crate::state::StateMachine;
use crate::store::{InMemoryStore, TradeStore};
use crate::util::{diff_details, TradeDiff};

pub struct TradeEngine {
    /// Snowflake generator encapsulated in the engine
    id_gen: SnowflakeIdGenerator,

    /// Shared, thread-safe, and mutable trade store:
    /// - `Arc<Mutex dyn`: shared ownership across threads with mutability supporting trait objects.
    /// - `Send + Sync + 'static`: safe cross-thread usage.
    store: Arc<Mutex<dyn TradeStore + Send + Sync + 'static>>,

    /// State machine logic can be updated without much touching engine code
    state_machine: StateMachine,
}

/// Meat and potatoes of the trade engine
impl<'a> TradeEngine {
    /// Helper method to access the store properly
    fn store_lock(&self) -> Result<std::sync::MutexGuard<'_, dyn TradeStore + Send + Sync + 'static>, ValidationError> {
        self.store
            .lock()
            .map_err(|_| ValidationError::Internal("Failed to acquire store lock".into()))
    }

    /// Internal function to fetch a trade by ID
    /// Returns a Result with the trade or an error
    /// ValidationError is an internal enum, we expose AppError to the outside world
    fn fetch_trade(&self, trade_id: TradeId) -> Result<Trade, ValidationError> {
        let store = self.store_lock()?;
        store.get(trade_id).ok_or(ValidationError::TradeNotFound(trade_id))
    }

    /// Creates a new instance of the TradeEngine
    /// The instance is thread safe and contains the storage (whether in-memory or other)
    pub fn new(store: InMemoryStore) -> Self {
        // For the snowflake ID generator, use a config-based machine ID
        let machine_id = config_int("engine.machine_id").unwrap_or(10) as u16;

        // wrap the store in an Arc<Mutex for thread safety
        let store: Arc<Mutex<dyn TradeStore>> = Arc::new(Mutex::new(store));

        Self {
            id_gen: SnowflakeIdGenerator::new(machine_id),
            store,
            state_machine: StateMachine::default(),
        }
    }

    /// Creates a DRAFT trade on the system and returns the trade ID.
    pub fn create(&self, user_id: &str, details: TradeDetails) -> Result<TradeId, AppError> {
        // Ensure the trade details are all present and correct
        details.validate()?; // Converts to AppError with "From"

        let trade_id = self.id_gen.generate(); // Snowflake ID generation
        let trade = Trade::new(trade_id, details, user_id.to_string());

        let mut store_guard = self.store.lock().map_err(
            // Should never happen
            |_| ValidationError::Internal("Failed to acquire store lock".into()),
        )?;

        store_guard.push(trade);
        Ok(trade_id)
    }

    /// Transition a draft trade to a pending approval state.
    pub fn submit(&self, user_id: &str, trade_id: TradeId) -> Result<(), AppError> {
        // Grab the trade from the trade id
        let mut trade = self.fetch_trade(trade_id)?; // ValidationError becomes AppError with "From"

        let state_now = trade.current_state();
        let state_new = self.state_machine.next_state(TradeAction::Submit, state_now)?; // PendingApproval

        // Check if the transition is allowed (we don't assume a submission from draft state)
        // Only DRAFT trades can be submitted
        if !self.state_machine.can_transition(state_now, state_new) {
            return Err(ValidationError::InvalidTransition(state_now, state_new).into());
            // Converts to AppError
        }

        // Get a copy of the latest details
        let details = trade.latest_details().cloned().ok_or_else(|| {
            // This should never happen, but if it does, we need to handle it
            ValidationError::Internal("Missing trade details during submit".into())
        })?;

        // Record the event snapshot, preserving all state and details
        // TODO :: NOTE:: details are entirely unchanged in this case
        //  There probably is no point duplicating the details here
        trade.add_snapshot(user_id, state_new, details);

        // put the modified trade back into the store
        // Later we'll come back and refactor to edit trade in place
        self.store_lock()?.update(trade)?;

        Ok(())
    }

    /// A user is approving a trade for execution
    /// Applies to trades in PendingApproval or NeedsReapproval
    /// Business rule: only the original requester can re-approve a trade
    pub fn approve(&self, user_id: &str, trade_id: TradeId) -> Result<(), AppError> {
        // Grab the trade from the trade id
        let mut trade = self.fetch_trade(trade_id).map_err(|err| {
            let app_err: AppError = err.into();
            app_err.with_tags(&["approve"])
        })?;

        // Determine the state transition
        let state_now = trade.current_state();
        let state_new = self.state_machine.next_state(TradeAction::Approve, state_now)?; // Expecting "Approved"

        // Bundle up some data for error reporting
        let err_data = json!({"user_id" : user_id, "trade_id": trade_id});

        // Check if the transition is allowed (don't assume submission from correct state)
        if !self.state_machine.can_transition(state_now, state_new) {
            let err: AppError = ValidationError::InvalidTransition(state_now, state_new).into();
            return Err(err.with_tags(&["approve"]).with_data("state", err_data));
        }

        // -----------------------------------------------------------------------------------------
        // Business rule:
        // -----------------------------------------------------------------------------------------
        // We do not allow the original requester to approve a trade (only re-approve)
        // In real life we'd hook into a proper authentication / user system
        if state_now != NeedsReapproval && trade.get_requester() == user_id {
            return Err(AppError::from_code(ErrCodes::TOR14, err_data).with_tags(&["approve", "requester"]));
        }

        // -----------------------------------------------------------------------------------------
        // Special business rule:
        // -----------------------------------------------------------------------------------------
        // We only allow the original requester to RE-approve a trade
        // (Original requester is not the first approver, but the user who created the trade)
        if trade.needs_re_approval() {
            if trade.get_requester() != user_id {
                return Err(AppError::from_code(ErrCodes::T0001, err_data).with_tags(&["approve", "re-approval"]));
            }
            // If we get here, the user is the original requester, so we're fine
        }
        // -----------------------------------------------------------------------------------------

        // Get a copy of the latest trade details
        let details = trade
            .latest_details()
            .cloned()
            .ok_or_else(|| ValidationError::Internal("Missing trade details on approve".into()))?;

        // Save the event snapshot
        // TODO :: NOTE:: details are entirely unchanged in this case
        //  There probably is no point duplicating the details here
        trade.add_snapshot(user_id, state_new, details);

        // put the modified trade back into the store
        // Later we'll come back and refactor to edit trade in place
        self.store_lock()?.update(trade)?;

        Ok(())
    }

    /// Cancel a trade
    /// Applies to trades in Draft, PendingApproval, NeedsReapproval, Approved
    /// and possibly SentToCounterparty, but not Executed or Cancelled
    pub fn cancel(&self, user_id: &str, trade_id: TradeId) -> Result<(), AppError> {
        // Grab the trade from the trade id
        let mut trade = self.fetch_trade(trade_id).map_err(|err| {
            let app_err: AppError = err.into();
            app_err.with_tags(&["cancel"])
        })?;

        let state_now = trade.current_state();
        let state_new = TradeState::Cancelled;

        // Check if the transition to cancelled is allowed
        if !self.state_machine.can_transition(state_now, state_new) {
            let err_data = json!({"user_id": user_id, "trade_id": trade_id});
            let err: AppError = ValidationError::InvalidTransition(state_now, state_new).into();

            // write that we are here
            println!("TradeEngine::cancel: trade_id: {}, state_now: {:?}, state_new: {:?} - failed", trade_id, state_now, state_new);

            return Err(err.with_tags(&["cancel"]).with_data("state", err_data));
        }

        let details = trade
            .latest_details()
            .cloned()
            .ok_or_else(|| ValidationError::Internal("Missing trade details on cancel".into()))?;

        // TODO :: NOTE:: details are entirely unchanged in this case
        //  There probably is no point duplicating the details here
        trade.add_snapshot(user_id, state_new, details);

        // put the modified trade back into the store
        // Later we'll come back and refactor to edit trade in place
        self.store_lock()?.update(trade)?;

        Ok(())
    }

    /// Update trade details
    /// Can only be done if trade has not been sent to counterparty and beyond
    pub fn update(&self, user_id: &str, trade_id: TradeId, details: TradeDetails) -> Result<(), AppError> {
        // Ensure the incoming trade details are all present and correct
        details.validate()?;

        // Grab the trade from the trade id
        let mut trade = self.fetch_trade(trade_id).map_err(|err| {
            let app_err: AppError = err.into();
            app_err.with_tags(&["update"])
        })?;

        // Figure out the current state, and the state we would transition to
        let state_now = trade.current_state();
        let state_new = self.state_machine.next_state(TradeAction::Update, state_now)?;

        // Validate the proposed state transition
        let err_data = json!({"user_id": user_id, "trade_id": trade_id});
        if !self.state_machine.can_transition(state_now, state_new) {
            let e: AppError = ValidationError::InvalidTransition(state_now, state_new).into();
            return Err(e.with_data("info", err_data).with_tags(&["update"]));
        }

        // No-op if details are identical
        if let Some(current) = trade.latest_details() {
            if current == &details {
                return Err(AppError::from_code(ErrCodes::TDI13, err_data)
                    .with_data("reason", json!("No change in trade details"))
                    .with_tags(&["update", "noop"]));
            }
        }

        // One or more within details have now definitely changed
        trade.add_snapshot(user_id, state_new, details);

        // put the modified trade back into the store
        // Later we'll come back and refactor to edit trade in place
        self.store_lock()?.update(trade)?;

        Ok(())
    }

    /// Send a trade to the counterparty for execution
    pub fn send_to_execute(&self, user_id: &str, trade_id: TradeId) -> Result<(), AppError> {
        // Grab the trade from the trade id
        let mut trade = self.fetch_trade(trade_id).map_err(|err| {
            let app_err: AppError = err.into();
            app_err.with_tags(&["send"])
        })?;

        let state_now = trade.current_state();
        let state_new = self.state_machine.next_state(TradeAction::SendToExecute, state_now)?;
        if !self
            .state_machine
            .can_transition(state_now, TradeState::SentToCounterparty)
        {
            let e: AppError = ValidationError::InvalidTransition(state_now, TradeState::SentToCounterparty).into();
            let err_data = json!({"user_id": user_id, "trade_id": trade_id});
            return Err(e.with_data("info", err_data).with_tags(&["send"]));
        }

        // Get a copy of the latest trade details
        let details = trade
            .latest_details()
            .cloned()
            .ok_or_else(|| ValidationError::Internal("Missing trade details on send_to_execute".into()))?;

        // TODO :: NOTE:: details are entirely unchanged in this case
        //  There probably is no point duplicating the details here
        trade.add_snapshot(user_id, state_new, details);

        // put the modified trade back into the store
        // Later we'll come back and refactor to edit trade in place
        self.store_lock()?.update(trade)?;

        Ok(())
    }

    /// Marks a trade as executed
    /// Applies to trades in SentToCounterparty only
    pub fn book(&self, user_id: &str, trade_id: TradeId) -> Result<(), AppError> {
        let mut trade = self.fetch_trade(trade_id).map_err(|err| {
            let app_err: AppError = err.into();
            app_err.with_tags(&["book"])
        })?;

        let state_now = trade.current_state();
        let state_new = self.state_machine.next_state(TradeAction::Book, state_now)?;
        if !self.state_machine.can_transition(state_now, TradeState::Executed) {
            let err_data = json!({ "user_id": user_id, "trade_id": trade_id });
            let err: AppError = ValidationError::InvalidTransition(state_now, TradeState::Executed).into();
            return Err(err.with_data("info", err_data).with_tags(&["book"]));
        }

        let details = trade
            .latest_details()
            .cloned()
            .ok_or_else(|| ValidationError::Internal("Missing trade details on book".into()))?;

        trade.add_snapshot(user_id, state_new, details);

        // put the modified trade back into the store
        // Later we'll come back and refactor to edit trade in place
        self.store_lock()?.update(trade)?;

        Ok(())
    }

    /// Gets the status of the given trade id
    pub fn trade_get_status(&self, trade_id: TradeId) -> Result<TradeState, AppError> {
        let trade = self.fetch_trade(trade_id).map_err(|err| {
            let app_err: AppError = err.into();
            app_err.with_tags(&["trade_get_status"])
        })?;

        Ok(trade.current_state())
    }

    /// Fetch a simple list of trade IDs
    pub fn trade_ids(&self, should_sort: bool) -> Result<Vec<TradeId>, AppError> {
        let store = self.store_lock()?;
        if should_sort {
            let mut keys = store.keys();
            keys.sort();
            return Ok(keys);
        }
        Ok(store.keys())
    }

    /// Fetch a vector of TradeEventSnapshot objects
    /// These include the state transitions and details for each state
    pub fn trade_history(&self, trade_id: TradeId) -> Result<Vec<TradeEventSnapshot>, AppError> {
        let trade = self.fetch_trade(trade_id).map_err(|err| {
            let app_err: AppError = err.into();
            app_err.with_tags(&["history"])
        })?;

        Ok(trade.history)
    }

    /// Fetch the latest (current) trade details for the given trade id
    pub fn trade_details(&self, trade_id: TradeId) -> Result<TradeDetails, AppError> {
        let trade = self.fetch_trade(trade_id).map_err(|err| {
            let app_err: AppError = err.into();
            app_err.with_tags(&["trade_details"])
        })?;

        // Get the latest details
        trade
            .latest_details()
            .cloned()
            .ok_or_else(|| ValidationError::Internal("Missing trade details".into()).into())
    }

    /// Returns a structure of differences between two snapshots of a trade
    ///
    /// # Arguments
    /// trade_id - The ID of the trade to compare
    /// v1 - The version of the first snapshot (0-indexed)
    /// v2 - The version of the second snapshot (0-indexed)
    pub fn diff(&self, trade_id: TradeId, v1: usize, v2: usize) -> Result<TradeDiff, AppError> {
        let trade = self.fetch_trade(trade_id).map_err(|err| {
            let app_err: AppError = err.into();
            app_err.with_tags(&["diff"])
        })?;

        let from = trade.history.get(v1).ok_or_else(|| {
            let e: AppError = ValidationError::Internal(format!("Snapshot v{} not found", v1)).into();
            return e.with_tags(&["diff", "from"]);
        })?;

        let to = trade.history.get(v2).ok_or_else(|| {
            let e: AppError = ValidationError::Internal(format!("Snapshot v{} not found", v2)).into();
            return e.with_tags(&["diff", "to"]);
        })?;

        // Using the diff_details helper to do the comparison
        let differences = diff_details(&from.details, &to.details);
        Ok(TradeDiff {
            trade_id,
            from_version: v1,
            to_version: v2,
            from_user: from.user_id.clone(),
            to_user: to.user_id.clone(),
            from_timestamp: from.timestamp,
            to_timestamp: to.timestamp,
            differences,
        })
    }
}

// = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =
// Basic unit tests for engine logic
// = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =
#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};
    use super::*;
    use crate::model::{Currency, Direction};
    use rust_decimal_macros::dec;

    fn sample_trade_details() -> TradeDetails {
        TradeDetails {
            trading_entity: "EntityA".into(),
            counterparty: "CounterpartyB".into(),
            direction: Direction::Buy,
            notional_currency: Currency::USD,
            notional_amount: dec!(1_000_000.00),
            underlying: vec![Currency::EUR, Currency::GBP, Currency::USD],
            trade_date: Utc.with_ymd_and_hms(2025, 4, 10, 0, 0, 0).unwrap(),
            value_date: Utc.with_ymd_and_hms(2025, 4, 12, 0, 0, 0).unwrap(),
            delivery_date: Utc.with_ymd_and_hms(2025, 4, 13, 0, 0, 0).unwrap(),
            strike: Some(dec!(1.2345)),
        }
    }

    fn new_engine() -> TradeEngine {
        TradeEngine::new(InMemoryStore::new())
    }

    #[test]
    fn test_submit_trade() {
        let engine = new_engine();
        let user_id = "alice";
        let details = sample_trade_details();

        // 1: Create trade
        let trade_id = engine.create(user_id, details).expect("Trade creation failed");

        // 2: Submit trade
        let result = engine.submit(user_id, trade_id);
        assert!(result.is_ok(), "Submit failed: {:?}", result);

        // 3: Check trade state is now PendingApproval
        let state = engine.trade_get_status(trade_id).expect("Failed to get trade state");
        assert_eq!(state, TradeState::PendingApproval, "Trade state should be PendingApproval");

        // 4: Try to submit again, should fail (InvalidTransition)
        let result_again = engine.submit(user_id, trade_id);
        assert!(result_again.is_err(), "Resubmitting should fail");

        let err = result_again.unwrap_err();
        assert_eq!(err.code(), "TST02", "Expected error code TST02 for invalid action");
    }

    #[test]
    fn test_create_trade() {
        let engine = new_engine();
        let user_id = "alice";
        let details = sample_trade_details();

        // Create trade
        let trade_id_result = engine.create(user_id, details.clone());
        assert!(trade_id_result.is_ok(), "Trade creation failed: {:?}", trade_id_result);

        let trade_id = trade_id_result.unwrap();

        // Fetch back the trade details
        let fetched = engine.trade_details(trade_id);
        assert!(fetched.is_ok(), "Fetching trade failed: {:?}", fetched);

        let fetched_details = fetched.unwrap();
        assert_eq!(fetched_details, details, "Trade details mismatch");
    }

    #[test]
    fn test_approve_trade_happy_path() {
        let engine = new_engine();
        let requester = "alice";
        let approver = "bob"; // not the requester
        let details = sample_trade_details();

        // Create trade
        let trade_id = engine.create(requester, details).expect("Trade creation failed");

        // Submit it
        engine.submit(requester, trade_id).expect("Submit failed");

        // Approver (not requester) approves it
        let result = engine.approve(approver, trade_id);
        assert!(result.is_ok(), "Approve failed: {:?}", result);

        // 4: Verify new state is Approved
        let state = engine.trade_get_status(trade_id).expect("Failed to get status");
        assert_eq!(state, TradeState::Approved, "Expected trade to be in Approved state");
    }

    #[test]
    fn test_approve_trade_rejected_for_requester() {
        let engine = new_engine();
        let requester = "alice";
        let details = sample_trade_details();

        // Create trade
        let trade_id = engine.create(requester, details).expect("Trade creation failed");

        // Submit trade
        engine.submit(requester, trade_id).expect("Submit failed");

        // Requester tries to approve — this should fail
        let result = engine.approve(requester, trade_id);
        assert!(result.is_err(), "Requester should not be allowed to approve");

        let err = result.unwrap_err();

        // Expect error code TOR14 with tags
        assert_eq!(err.code(), "TOR14", "Expected error code TOR14 for requester approval");
        assert!(err.tags().contains(&"approve".into()), "Expected tag 'approve'");
        assert!(err.tags().contains(&"requester".into()), "Expected tag 'requester'");
    }

    #[test]
    fn test_reapproval_by_requester_allowed() {
        let engine = new_engine();
        let requester = "alice";
        let approver = "bob";
        let details = sample_trade_details();

        // 1: Create + Submit
        let trade_id = engine.create(requester, details.clone()).expect("Trade creation failed");
        engine.submit(requester, trade_id).expect("Submit failed");

        // 2: Approver approves
        engine.approve(approver, trade_id).expect("Initial approval failed");

        // 3: Approver updates the trade (triggers NeedsReapproval)
        let mut new_details = details.clone();
        new_details.strike = Some(dec!(1.2500)); // small change
        engine.update(approver, trade_id, new_details).expect("Update failed");

        // 4: Now requester re-approves
        let result = engine.approve(requester, trade_id);
        assert!(result.is_ok(), "Re-approval by requester should succeed: {:?}", result);

        // 5: Check final state is Approved
        let state = engine.trade_get_status(trade_id).expect("Failed to get state");
        assert_eq!(state, TradeState::Approved, "Expected trade to be in Approved after re-approval");
    }

    #[test]
    fn test_reapproval_rejected_for_non_requester() {
        let engine = new_engine();
        let requester = "alice";
        let approver = "bob";
        let intruder = "charlie";
        let details = sample_trade_details();

        // 1: Create + Submit
        let trade_id = engine.create(requester, details.clone()).expect("Trade creation failed");
        engine.submit(requester, trade_id).expect("Submit failed");

        // 2: Approver approves
        engine.approve(approver, trade_id).expect("Approval by bob failed");

        // 3: Approver updates (triggers NeedsReapproval)
        let mut modified_details = details.clone();
        modified_details.strike = Some(dec!(1.3456));
        engine.update(approver, trade_id, modified_details).expect("Update failed");

        // 4: Non-requester (charlie) tries to re-approve — should be rejected
        let result = engine.approve(intruder, trade_id);
        assert!(result.is_err(), "Non-requester re-approval should fail");

        let err = result.unwrap_err();

        // Assert it's the correct code and tagging
        assert_eq!(err.code(), "T0001", "Expected error code T0001 for invalid re-approver");
        assert!(err.tags().contains(&"approve".into()), "Expected 'approve' tag");
        assert!(err.tags().contains(&"re-approval".into()), "Expected 're-approval' tag");
    }

    #[test]
    fn test_cancel_from_draft() {
        let engine = new_engine();
        let user = "alice";
        let details = sample_trade_details();

        // 1: Create trade (still in Draft)
        let trade_id = engine.create(user, details).expect("Create failed");

        // 2: Cancel it
        let result = engine.cancel(user, trade_id);
        if result.is_err() {
            result.as_ref().err().unwrap().display_with_trace();
        }
        assert!(result.is_ok(), "Cancel from Draft should succeed");

        // Step 3: Confirm state is Cancelled
        let state = engine.trade_get_status(trade_id).expect("State fetch failed");
        assert_eq!(state, TradeState::Cancelled, "Expected state to be Cancelled");
    }

    #[test]
    fn test_cancel_after_executed_should_fail() {
        let engine = new_engine();
        let requester = "alice";
        let approver = "bob";
        let details = sample_trade_details();

        // 1: Create → Submit → Approve
        let trade_id = engine.create(requester, details.clone()).expect("Create failed");
        engine.submit(requester, trade_id).expect("Submit failed");
        engine.approve(approver, trade_id).expect("Approval failed");

        // 2: Send to counterparty
        engine.send_to_execute(approver, trade_id).expect("Send to counterparty failed");

        // 3: Book (Executed)
        engine.book(approver, trade_id).expect("Booking failed");

        // 4: Attempt to cancel — should fail
        let result = engine.cancel(approver, trade_id);
        assert!(result.is_err(), "Cancel after execution should fail");

        let err = result.unwrap_err();

        // Assert error type
        assert_eq!(err.code(), "TST02", "Expected error code TST02 for invalid transition");
        assert!(err.tags().contains(&"state".into()), "Expected tag 'state'");
        assert!(err.tags().contains(&"validation".into()), "Expected tag 'validation'");
    }

    #[test]
    fn test_cancel_twice_should_fail() {
        let engine = new_engine();
        let user = "alice";
        let details = sample_trade_details();

        // Step 1: Create a trade in Draft state
        let trade_id = engine.create(user, details).expect("Create failed");

        // Step 2: Cancel it once (valid)
        engine.cancel(user, trade_id).expect("Initial cancel should succeed");

        // Step 3: Try cancel again — should fail
        let result = engine.cancel(user, trade_id);
        assert!(result.is_err(), "Second cancel should fail");

        let err = result.unwrap_err();
        assert_eq!(err.code(), "TST02", "Expected error code TST02 for invalid transition");
        assert!(err.tags().contains(&"state".into()), "Expected tag 'state'");
    }

    #[test]
    fn test_update_trade_details_success() {
        let engine = new_engine();
        let requester = "alice";
        let approver = "bob";
        let mut details = sample_trade_details();

        // 1: Create, submit, approve
        let trade_id = engine.create(requester, details.clone()).expect("Create failed");
        engine.submit(requester, trade_id).expect("Submit failed");
        engine.approve(approver, trade_id).expect("Approve failed");

        // 2: Modify details
        details.strike = Some(dec!(1.3333)); // small change

        // 3: Update trade
        let result = engine.update(approver, trade_id, details.clone());
        assert!(result.is_ok(), "Update failed: {:?}", result);

        // 4: Check state is now NeedsReapproval
        let state = engine.trade_get_status(trade_id).expect("Get state failed");
        assert_eq!(state, TradeState::NeedsReapproval, "Expected NeedsReapproval state");

        // 5: Confirm updated details are present
        let updated = engine.trade_details(trade_id).expect("Get details failed");
        assert_eq!(updated, details, "Trade details should match updated");
    }

    #[test]
    fn test_update_noop_should_fail() {
        let engine = new_engine();
        let requester = "alice";
        let approver = "bob";
        let details = sample_trade_details();

        // : Create, submit, approve
        let trade_id = engine.create(requester, details.clone()).expect("Create failed");
        engine.submit(requester, trade_id).expect("Submit failed");
        engine.approve(approver, trade_id).expect("Approve failed");

        // 2: Try to update with the *same* details
        let result = engine.update(approver, trade_id, details.clone());
        assert!(result.is_err(), "No-op update should fail");

        let err = result.unwrap_err();
        assert_eq!(err.code(), "TDI13", "Expected TDI13 for no-op update");
        assert!(err.tags().contains(&"update".into()), "Expected 'update' tag");
        assert!(err.tags().contains(&"noop".into()), "Expected 'noop' tag");
    }

    #[test]
    fn test_update_after_executed_should_fail() {
        let engine = new_engine();
        let requester = "alice";
        let approver = "bob";
        let mut details = sample_trade_details();

        // 1: Create, Submit, Approve, Send, Book
        let trade_id = engine.create(requester, details.clone()).expect("Create failed");
        engine.submit(requester, trade_id).expect("Submit failed");
        engine.approve(approver, trade_id).expect("Approve failed");
        engine.send_to_execute(approver, trade_id).expect("Send failed");
        engine.book(approver, trade_id).expect("Booking failed");

        // 2: Try to update (should fail)
        details.strike = Some(dec!(2.2222));
        let result = engine.update(approver, trade_id, details);

        assert!(result.is_err(), "Update after execution should fail");

        let err = result.unwrap_err();
        assert_eq!(err.code(), "TST02", "Expected error code TST02 for invalid transition");
        assert!(err.tags().contains(&"state".into()), "Expected 'state' tag");
    }

    #[test]
    fn test_send_to_execute_success() {
        let engine = new_engine();
        let requester = "alice";
        let approver = "bob";
        let details = sample_trade_details();

        // 1: Create ➡️ Submit ➡️ Approve
        let trade_id = engine.create(requester, details.clone()).expect("Create failed");
        engine.submit(requester, trade_id).expect("Submit failed");
        engine.approve(approver, trade_id).expect("Approve failed");

        // 2: Send to counterparty
        let result = engine.send_to_execute(approver, trade_id);
        assert!(result.is_ok(), "send_to_execute should succeed");

        // 3: Confirm state is now SentToCounterparty
        let state = engine.trade_get_status(trade_id).expect("Failed to get state");
        assert_eq!(state, TradeState::SentToCounterparty, "Expected SentToCounterparty state");
    }

    #[test]
    fn test_send_from_draft_should_fail() {
        let engine = new_engine();
        let user = "alice";
        let details = sample_trade_details();

        // 1: Create trade — still in Draft
        let trade_id = engine.create(user, details).expect("Create failed");

        // 2: Attempt to send to execute (invalid from Draft)
        let result = engine.send_to_execute(user, trade_id);
        assert!(result.is_err(), "Send from Draft should fail");

        let err = result.unwrap_err();
        assert_eq!(err.code(), "TST02", "Expected TST02 for invalid transition");
    }

    #[test]
    fn test_book_trade_success() {
        let engine = new_engine();
        let requester = "alice";
        let approver = "bob";
        let details = sample_trade_details();

        // 1: Create, Submit, Approve, Send
        let trade_id = engine.create(requester, details.clone()).expect("Create failed");
        engine.submit(requester, trade_id).expect("Submit failed");
        engine.approve(approver, trade_id).expect("Approve failed");
        engine.send_to_execute(approver, trade_id).expect("Send failed");

        // 2: Book the trade
        let result = engine.book(approver, trade_id);
        assert!(result.is_ok(), "Booking should succeed");

        // 3: Confirm final state
        let state = engine.trade_get_status(trade_id).expect("Get state failed");
        assert_eq!(state, TradeState::Executed, "Expected state to be Executed");
    }

    #[test]
    fn test_book_from_draft_should_fail() {
        let engine = new_engine();
        let user = "alice";
        let details = sample_trade_details();

        // 1: Create trade (still in Draft)
        let trade_id = engine.create(user, details).expect("Create failed");

        // 2: Try to book immediately — invalid
        let result = engine.book(user, trade_id);
        assert!(result.is_err(), "Booking from Draft should fail");

        let err = result.unwrap_err();
        assert_eq!(err.code(), "TST02", "Expected TST02 for invalid transition");
    }
}

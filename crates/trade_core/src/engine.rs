// CURRENTLY UNDER HEAVY DEVELOPMENT

use std::sync::Arc;
use serde_json::json;
use app_core::config::ConfigManager;
use app_core::{AppError};

use crate::errors::{ErrCodes, ValidationError};
use crate::model::*;
use crate::state::StateMachine;
use crate::store::TradeStore;
use crate::snowflake::SnowflakeIdGenerator;
use crate::util::{diff_details, TradeDiff};

pub struct TradeEngine {
    id_gen: SnowflakeIdGenerator,
    store: Arc<dyn TradeStore>,
    state_machine: StateMachine,
}

/// Meat and potatoes of the trade engine
impl TradeEngine {

    /// Internal function to fetch a trade by ID
    /// Returns a Result with the trade or an error
    /// ValidationError is an internal enum, we expose AppError to the outside world
    fn fetch_trade(&self, trade_id: TradeId) -> Result<Trade, ValidationError> {
        self.store
            .get(trade_id)
            .ok_or(ValidationError::TradeNotFound(trade_id))
    }

    /// Creates a new instance of the TradeEngine
    /// The instance is thread safe and contains the storage (whether in-memory or other)
    pub fn new(store: Arc<dyn TradeStore>) -> Self {
        // For the snowflake ID generator, use a config-based machine ID
        let machine_id = ConfigManager::get_int("engine.machine_id").unwrap_or(10) as u16;

        Self {
            id_gen: SnowflakeIdGenerator::new(machine_id),
            store,
            state_machine: StateMachine::default(),
        }
    }

    /// Creates a DRAFT trade on the system and returns the trade ID.
    pub fn create(&mut self, user_id: &str, details: TradeDetails, ) -> Result<TradeId, AppError> {
        // Ensure the trade details are all present and correct
        details.validate()?; // Converts to AppError with "From"

        let trade_id = self.id_gen.generate(); // Snowflake ID generation
        let trade = Trade::new(trade_id, details, user_id.to_string());

        self.store.create(trade)?;

        Ok(trade_id)
    }

    /// Transition a draft trade to a pending approval state.
    pub fn submit(&mut self, user_id: &str, trade_id: TradeId) -> Result<(), AppError> {
        // Grab the trade from the trade id
        let mut trade = self.fetch_trade(trade_id)?; // ValidationError becomes AppError with "From"

        let state_now = trade.current_state();
        let state_new = self.state_machine.next_state(TradeAction::Submit, state_now)?; // PendingApproval

        // Check if the transition is allowed (we don't assume a submission from draft state)
        // Only DRAFT trades can be submitted
        if !self.state_machine.can_transition(state_now, state_new) {
            ValidationError::InvalidTransition(state_now, state_new)? // Converts to AppError
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

        Ok(())
    }

    /// A user is approving a trade for execution
    /// Applies to trades in PendingApproval or NeedsReapproval
    /// Business rule: only the original requester can re-approve a trade
    pub fn approve(&mut self, user_id: &str, trade_id: TradeId) -> Result<(), AppError> {
        // Grab the trade from the trade id
        let mut trade = self.fetch_trade(trade_id).map_err(|err| {
            let app_err: AppError = err.into();
            app_err.with_tags(&["approve"])
        })?;

        // Determine the state transition
        let state_now = trade.current_state();
        let state_new = self
            .state_machine
            .next_state(TradeAction::Approve, state_now)?; // Expecting "Approved"

        // Bundle up some data for error reporting
        let err_data = json!({"user_id" : user_id, "trade_id": trade_id});

        // Check if the transition is allowed (don't assume submission from correct state)
        if !self.state_machine.can_transition(state_now, state_new) {
            let err: AppError = ValidationError::InvalidTransition(state_now, state_new)?;
            return Err(err.with_tags(&["approve"]).with_data("state", err_data));
        }

        // -----------------------------------------------------------------------------------------
        // Special business rule:
        // -----------------------------------------------------------------------------------------
        // We only allow the original requester to RE-approve a trade
        // (Original requester is not the first approver, but the user who created the trade)
        if trade.needs_re_approval() {
            if trade.get_requester() != user_id {
                return Err(AppError::from_code(ErrCodes::T0001, err_data)
                    .with_tags(&["approve", "re-approval"]));
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

        Ok(())
    }

    /// Cancel a trade
    /// Applies to trades in Draft, PendingApproval, NeedsReapproval, Approved
    /// and possibly SentToCounterparty, but not Executed or Cancelled
    pub fn cancel(&mut self, user_id: &str, trade_id: TradeId) -> Result<(), AppError> {
        // Grab the trade from the trade id
        let mut trade = self.fetch_trade(trade_id).map_err(|err| {
            let app_err: AppError = err.into();
            app_err.with_tags(&["cancel"])
        })?;

        let state_now  = trade.current_state();
        let state_new = TradeState::Cancelled;

        // Check if the transition to cancelled is allowed
        if !self.state_machine.can_transition(state_now, state_new) {
            let err_data = json!({"user_id": user_id, "trade_id": trade_id});
            let err: AppError = ValidationError::InvalidTransition(state_now, state_new)?;
            return Err(err.with_tags(&["cancel"]).with_data("state", err_data));
        }

        let details = trade
            .latest_details()
            .cloned()
            .ok_or_else(|| ValidationError::Internal("Missing trade details on cancel".into()))?;

        // TODO :: NOTE:: details are entirely unchanged in this case
        //  There probably is no point duplicating the details here
        trade.add_snapshot(user_id, state_new, details);

        Ok(())
    }

    /// Update trade details
    /// Can only be done if trade has not been sent to counterparty and beyond
    pub fn update(&mut self, user_id: &str, trade_id: TradeId, details: TradeDetails) -> Result<(), AppError> {
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
            let err = Err(ValidationError::InvalidTransition(state_now, state_new));
            return Err(AppError::from_error(err)
                .with_data("update", err_data)
                .with_tags(&["update"])
            );
        }

        // No-op if details are identical
        if let Some(current) = trade.latest_details() {
            if current == &details {
                return Err(
                    AppError::from_code(ErrCodes::TDI13, err_data)
                        .with_data("reason", json!("No change in trade details"))
                        .with_tags(&["update", "noop"])
                );
            }
        }

        // One or more within details have now definitely changed
        trade.add_snapshot(user_id, state_new, details);

        Ok(())
    }

    /// Send a trade to the counterparty for execution
    pub fn send_to_execute(&mut self, user_id: &str, trade_id: TradeId, ) -> Result<(), AppError> {
        // Grab the trade from the trade id
        let mut trade = self.fetch_trade(trade_id).map_err(|err| {
            let app_err: AppError = err.into();
            app_err.with_tags(&["send"])
        })?;

        let state_now = trade.current_state();
        let state_new = self.state_machine.next_state(TradeAction::SendToExecute, state_now)?;
        if !self.state_machine.can_transition(state_now, TradeState::SentToCounterparty) {
            let e: AppError = ValidationError::InvalidTransition(state_now, TradeState::SentToCounterparty).into();
            let err_data = json!({"user_id": user_id, "trade_id": trade_id});
            return Err(e.with_data("info", err_data).with_tags(&["send"]));
        }

        // Get a copy of the latest trade details
        let details = trade.latest_details().cloned().ok_or_else(|| {
            ValidationError::Internal("Missing trade details on send_to_execute".into())
        })?;

        // TODO :: NOTE:: details are entirely unchanged in this case
        //  There probably is no point duplicating the details here
        trade.add_snapshot(user_id, state_new, details);

        Ok(())
    }

    /// Marks a trade as executed
    /// Applies to trades in SentToCounterparty only
    pub fn book(&mut self, user_id: &str, trade_id: TradeId) -> Result<(), AppError>{
        let mut trade = self.fetch_trade(trade_id).map_err(|err| {
            let app_err: AppError = err.into();
            app_err.with_tags(&["book"])
        })?;

        let state_now = trade.current_state();
        let state_new = self.state_machine.next_state(TradeAction::Book, state_now)?;
        if !self.state_machine.can_transition(state_now, TradeState::Executed) {
            let err_data = json!({ "user_id": user_id, "trade_id": trade_id });
            let err = ValidationError::InvalidTransition(state_now, TradeState::Executed);
            return Err(AppError::from_error(err)
                .with_data("booking", err_data)
                .with_tags(&["book"]));
        }

        let details = trade.latest_details().cloned().ok_or_else(|| {
            ValidationError::Internal("Missing trade details on book".into())
        })?;

        trade.add_snapshot(user_id, state_new, details);

        Ok(())
    }

    /// Fetch a vector of TradeEventSnapshot objects
    /// These include the state transitions and details for each state
    pub fn history(&self, trade_id: TradeId) -> Result<Vec<TradeEventSnapshot>, AppError> {
        let mut trade = self.fetch_trade(trade_id).map_err(|err| {
            let app_err: AppError = err.into();
            app_err.with_tags(&["history"])
        })?;

        Ok(trade.history)
    }

    /// Returns a structure of differences between two snapshots of a trade
    ///
    pub fn diff(&self, trade_id: TradeId, v1: usize, v2: usize) -> Result<TradeDiff, AppError> {
        let mut trade = self.fetch_trade(trade_id).map_err(|err| {
            let app_err: AppError = err.into();
            app_err.with_tags(&["diff"])
        })?;

        let from = trade.history.get(v1).ok_or_else(|| {
            let e: AppError = ValidationError::Internal(format!("Snapshot v{} not found", v1)).into();
            return Err(e.with_tags(&["diff", "from"]))
        })?;

        let to = trade.history.get(v2).ok_or_else(|| {
            let e: AppError = ValidationError::Internal(format!("Snapshot v{} not found", v2)).into();
            return Err(e.with_tags(&["diff", "to"]))
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

// CURRENTLY UNDER HEAVY DEVELOPMENT

use std::sync::Arc;
use chrono::Utc;
use serde_json::json;
use app_core::config::ConfigManager;
use app_core::{app_err, AppError};

use crate::errors::{ErrCodes, ValidationError};
use crate::model::*;
use crate::state::StateMachine;
use crate::store::TradeStore;
use crate::snowflake::SnowflakeIdGenerator;
use crate::util::{diff_details, DiffMap};




pub struct TradeEngine {
    id_gen: SnowflakeIdGenerator,
    store: Arc<dyn TradeStore>,
    state_machine: StateMachine,
}

impl TradeEngine {


    /// Internal function to fetch a trade by ID
    /// Returns a Result with the trade or an error
    /// ValidationError is an internal enum, we expose AppError to the outside world
    fn fetch_trade(&self, trade_id: TradeId) -> Result<Trade, ValidationError> {
        self.store
            .get(trade_id)
            .ok_or(ValidationError::TradeNotFound(trade_id))
    }

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
    pub fn create(
        &mut self,
        user_id: &str,
        details: TradeDetails,
    ) -> Result<TradeId, ValidationError> {
        // Ensure the trade details are all present and correct
        details.validate()?;

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
        let state_next = self.state_machine.next_state(TradeAction::Submit, state_now)?; // PendingApproval

        // Check if the transition is allowed (we don't assume a submission from draft state)
        // Only DRAFT trades can be submitted
        if !self.state_machine.can_transition(state_now, state_next) {
            let err AppError = ValidationError::InvalidTransition(state_now, state_next).into()
                ;
            return Err(
                AppError::from_error(err)
                    .with_data("state", json!({"from": state_now, "to": state_next}))
                    .with_tags(&["submit"]),
            );
        }

        // Get a copy of the latest details
        let details = trade.latest_details().cloned().ok_or_else(|| {
            ValidationError::Internal("Missing trade details during submit".into())
        })?;

        // Record the event snapshot, preserving all state and details
        // TODO :: NOTE:: details are entirely unchanged in this case
        //  There probably is no point duplicating the details here
        trade.add_snapshot(user_id, state_next, details);

        self.store.update(trade)
        Ok(())
    }

    /// A user is approving a trade for execution
    /// Applies to trades in PendingApproval or NeedsReapproval
    /// Business rule: only the original requester can re-approve a trade
    pub fn approve(&mut self, user_id: &str, trade_id: TradeId) -> AppError {
        // Grab the trade from the trade id
        let mut trade = self.fetch_trade(trade_id).unwrap_or_else(|e| {
            AppError::from_code(ErrCodes::TNF01)

            AppError::from_error(e)
                .with_code(ErrCodes::T0001)
                .with_data("trade_id", trade_id)
                .with_tags(&["approve"])
        });

        let mut trade = self
            .store
            .get(trade_id)
            .ok_or_else(|| ValidationError::TradeNotFound(trade_id))?;

        // Determine the state transition
        let state_now = trade.current_state();
        let state_next = self
            .state_machine
            .next_state(TradeAction::Approve, state_now)?; // Expecting "Approved"

        // Bundle up some data for error reporting
        let err_data = json!({
            "user_id": user_id,
            "trade_id": trade_id,
            "state_now": state_now,
            "state_next": state_next
        });

        // Check if the transition is allowed (don't assume submission from correct state)
        if !self.state_machine.can_transition(state_now, state_next) {
            let err = Err(ValidationError::InvalidTransition(state_now, state_next));
            return AppError::from_error(err)
                .with_data("state", err_data)
                .with_tags(&["re-approval"])
        }

        // Special business rule:
        // ----------------------
        // We only allow the original requester to RE-approve a trade
        // (Original requester is not the first approver, but the user who created the trade)
        if trade.needs_re_approval() {
            if trade.get_requester() != user_id {
                return AppError::from_code(ErrCodes::T0001, err_data).with_tags(&["re-approval"]);
            }

            // If we get here, the user is the original requester, so we're fine
        }

        // Get a copy of the latest trade details
        let details = trade
            .latest_details()
            .cloned()
            .ok_or_else(|| ValidationError::Internal("Missing trade details on approve".into()))?;

        // Save the event snapshot
        // TODO :: NOTE:: details are entirely unchanged in this case
        //  There probably is no point duplicating the details here
        trade.add_snapshot(user_id, state_next, details);

        self.store.update_trade(trade);
        Ok(())
    }

    pub fn cancel(&mut self, user_id: &str, trade_id: TradeId) -> AppError {
        let mut trade = self
            .store
            .get(trade_id)
            .ok_or_else(|| ValidationError::TradeNotFound(trade_id))?;

        let state_now = trade.current_state();
        let state_next = TradeState::Cancelled;

        if !self.state_machine.can_transition(state_now, state_next) {
            return Err(ValidationError::InvalidTransition(state_now, state_next));
        }

        let details = trade
            .latest_details()
            .cloned()
            .ok_or_else(|| ValidationError::Internal("Missing trade details on cancel".into()))?;

        // TODO :: NOTE:: details are entirely unchanged in this case
        //  There probably is no point duplicating the details here
        trade.add_snapshot(user_id, state_next, details);

        self.store.update_trade(trade);
        Ok(())
    }

    // TODO - finish
    pub fn update(&mut self, user_id: &str, trade_id: TradeId, details: TradeDetails, ) -> Result<(), ValidationError> {
        // Ensure the trade details are all present and correct
        details.validate()?;

        let mut trade = self
            .store
            .get(trade_id)
            .ok_or(ValidationError::TradeNotFound(trade_id))?;

        let state_now = trade.current_state();
        let state_new = self
            .state_machine
            .next_state(TradeAction::Update, state_now)?; // TODO handle the error

        // If it's in draft state, it stays in draft state
        // if state_now == TradeState::Draft {
        //     return Err(ValidationError::InvalidTransition(state_now, TradeState::Draft));
        // }
        // // If it's in pending approval state, it stays in pending approval state
        // else if state_now == TradeState::PendingApproval {
        //     return Err(ValidationError::InvalidTransition(state_now, TradeState::PendingApproval));
        // }
        // // If it's in approve state, it goes to needs reapproval state
        // else if state_now == TradeState::Approved {
        //     return Err(ValidationError::InvalidTransition(state_now, TradeState::NeedsReapproval));
        // }

        // Ensure the trade is in a state that allows for updates
        // ie, it is not in a final state, or sent for execution, or cancelled
        self.state_machine
            .can_transition(state_now, TradeState::NeedsReapproval);

        // NOTE - one or more within details have now changed
        trade.add_snapshot(user_id, state_now, details);

        self.store.update_trade(trade);
        Ok(())
    }

    pub fn send_to_execute(&mut self, user_id: &str, trade_id: TradeId, ) -> Result<(), ValidationError> {
        let mut trade = self
            .store
            .get(trade_id)
            .ok_or(ValidationError::TradeNotFound(trade_id))?;

        let state_now = trade.current_state();
        if !self.state_machine.can_transition(state_now, TradeState::SentToCounterparty) {
            return Err(ValidationError::InvalidTransition(
                state_now,
                TradeState::SentToCounterparty,
            ));
        }

        let details = trade.latest_details().cloned().ok_or_else(|| {
            ValidationError::Internal("Missing trade details on send_to_execute".into())
        })?;

        // TODO :: NOTE:: details are entirely unchanged in this case
        //  There probably is no point duplicating the details here
        trade.add_snapshot(user_id, state_now, details);

        self.store.update_trade(trade);
        Ok(())
    }

    pub fn book(&mut self, user_id: &str, trade_id: TradeId) -> Result<(), ValidationError> {
        let mut trade = self
            .store
            .get(trade_id)
            .ok_or(ValidationError::TradeNotFound(trade_id))?;

        let state_now = trade.current_state();
        if !self.state_machine.can_transition(state_now, TradeState::Executed) {
            return Err(ValidationError::InvalidTransition(
                state_now,
                TradeState::Executed,
            ));
        }

        let details = trade
            .latest_details()
            .cloned()
            .ok_or_else(|| ValidationError::Internal("Missing trade details on book".into()))?;

        trade.add_snapshot(user_id, state_now, details);

        self.store.update_trade(trade);
        Ok(())
    }

    // TODO history
    pub fn history(&self, trade_id: TradeId) -> Result<Vec<TradeEventSnapshot>, ValidationError> {
        let trade = self
            .store
            .get(trade_id)
            .ok_or(ValidationError::TradeNotFound(trade_id))?;
        Ok(trade.history)
    }

    // TODO diff
    pub fn diff(&self, trade_id: TradeId, v1: usize, v2: usize) -> Result<DiffMap, ValidationError> {
        let trade = self
            .store
            .get(trade_id)
            .ok_or(ValidationError::TradeNotFound(trade_id))?;

        let from = trade
            .history
            .get(v1)
            .ok_or_else(|| ValidationError::Internal(format!("Snapshot v{} not found", v1)))?;

        let to = trade
            .history
            .get(v2)
            .ok_or_else(|| ValidationError::Internal(format!("Snapshot v{} not found", v2)))?;

        Ok(diff_details(&from.details, &to.details))
    }
}

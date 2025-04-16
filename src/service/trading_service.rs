//! # Service Layer: DDD Orchestration
//!
//! Domain-level orchestration logic for trade workflows.
//! It acts as the intermediary between external interfaces (REST/gRPC/FIX/etc.) and the
//! underlying `trade_core` library.
//!
//! Responsibilities include:
//! - App service boundary within DDD setup
//! - Validating any incoming requests
//! - Doing trade operations across the trade engine
//! - Enforcing higher-level business rules and process flows
//!
#[allow(dead_code)]
use app_core::AppError;
use rust_decimal::prelude::*;
use trade_core::model::{Currency, Direction, TradeDetails, TradeEventSnapshot};

use crate::service::trading_utils::history_to_table;
use crate::state::trading_state::engine;

const USER_TRADER_1: &str = "userTrader1";
const USER_ADMIN_1: &str = "userAdmin1";

pub fn create_trade(user_id: &str, details: TradeDetails) -> Result<String, AppError> {
    let trade_id = engine().create(user_id, details)?;
    Ok(trade_id.to_string())
}

pub fn trade_history(trade_id: u64) -> Result<Vec<TradeEventSnapshot>, AppError> {
    let history = engine().trade_history(trade_id)?;
    Ok(history)
}

pub(crate) fn trade_hello_world() -> Result<(), AppError> {
    let engine = engine();

    iout!("Hello world scenario");

    // Create a new trade
    let new_trade = TradeDetails {
        trading_entity: "foo".to_string(),
        counterparty: "bar".to_string(),
        direction: Direction::Buy,
        notional_currency: Currency::GBP,
        notional_amount: Decimal::from_str("100.1").unwrap(),
        underlying: vec![Currency::GBP, Currency::USD],
        trade_date: Default::default(),
        value_date: Default::default(),
        delivery_date: Default::default(),
        strike: None,
    };

    let trade_one = engine.create(USER_TRADER_1, new_trade)?;
    let trade_one_status = engine.trade_get_status(trade_one)?;
    sout!("\t -> First trade created with ID: {} and status {:?}", trade_one, trade_one_status);

    // there should be 1 item in the history
    let trade_one_hist = engine.trade_history(trade_one)?;
    sout!("\t -> Trade history count: {:?}", trade_one_hist.len());

    Ok(())
}

pub(crate) fn trade_scenario_1() -> Result<(), AppError> {
    iout!("Scenario 1 :: Submitting and Approving a Trade");

    let engine = engine();

    // Create a new trade
    let new_trade = TradeDetails {
        trading_entity: "foo".to_string(),
        counterparty: "bar".to_string(),
        direction: Direction::Buy,
        notional_currency: Currency::GBP,
        notional_amount: Decimal::from_str("55.6").unwrap(),
        underlying: vec![Currency::GBP, Currency::USD],
        trade_date: Default::default(),
        value_date: Default::default(),
        delivery_date: Default::default(),
        strike: None,
    };

    // Submit the trade
    let trade_id = engine.create(USER_TRADER_1, new_trade)?;
    let mut trade_status = engine.trade_get_status(trade_id)?;
    sout!("\t -> Trade created with ID: {} and status {:?}", trade_id, trade_status);

    // Submit the trade - status should transition to "PendingApproval"
    engine.submit(USER_TRADER_1, trade_id)?;
    trade_status = engine.trade_get_status(trade_id)?;
    sout!("\t -> Trade status after submission: {:?}", trade_status);

    // Obtain the trade details after submission
    let trade_details = engine.trade_details(trade_id)?;
    let amount = trade_details.notional_amount;
    sout!("\t -> Notional amount form trade details: {:?}", amount);

    // Admin approve the trade - status should transition to "Approved"
    engine.approve(USER_ADMIN_1, trade_id)?;
    trade_status = engine.trade_get_status(trade_id)?;
    sout!("\t -> Trade status after approval: {:?}", trade_status);

    Ok(())
}

pub(crate) fn trade_scenario_2() -> Result<(), AppError> {
    iout!("Scenario 2 :: An approver updates the trade details, requiring re-approval.");
    let engine = engine();

    // Create a new trade
    let new_trade = TradeDetails {
        trading_entity: "foo".to_string(),
        counterparty: "bar".to_string(),
        direction: Direction::Buy,
        notional_currency: Currency::GBP,
        notional_amount: Decimal::from_str("468.22").unwrap(),
        underlying: vec![Currency::GBP, Currency::USD],
        trade_date: Default::default(),
        value_date: Default::default(),
        delivery_date: Default::default(),
        strike: None,
    };

    // Create - Submit the trade into DRAFT
    let trade_id = engine.create(USER_TRADER_1, new_trade)?;
    let mut trade_status = engine.trade_get_status(trade_id)?;
    sout!("\t -> Trade created with ID: {} and status {:?}", trade_id, trade_status);

    // Get details of the trade just executed
    let mut trade_details = engine.trade_details(trade_id)?;

    // Second user updates the trade
    // Modify just the amount of the trade
    trade_details.notional_amount = Decimal::from_str("368.02").unwrap();

    engine.update(USER_ADMIN_1, trade_id, trade_details)?;
    trade_status = engine.trade_get_status(trade_id)?;
    sout!("\t -> Trade status after update: {:?}", trade_status);

    // user 1 Re-approves the trade
    engine.approve(USER_TRADER_1, trade_id)?;
    trade_status = engine.trade_get_status(trade_id)?;
    sout!("\t -> Trade status after re-approval: {:?}", trade_status);

    // Count the trade history
    let trade_history = engine.trade_history(trade_id).unwrap_or(vec![]).len();
    sout!("\t -> Trade history count: {:?}", trade_history);

    Ok(())
}

pub(crate) fn trade_scenario_3() -> Result<(), AppError> {
    iout!("Scenario 3 :: Approved trade sent to counterparty & marked as executed.");

    let engine = engine();

    // Create a new trade
    let new_trade = TradeDetails {
        trading_entity: "foo".to_string(),
        counterparty: "bar".to_string(),
        direction: Direction::Buy,
        notional_currency: Currency::GBP,
        notional_amount: Decimal::from_str("112.62").unwrap(),
        underlying: vec![Currency::GBP, Currency::USD],
        trade_date: Default::default(),
        value_date: Default::default(),
        delivery_date: Default::default(),
        strike: None,
    };

    // Submit the trade
    let trade_id = engine.create(USER_TRADER_1, new_trade)?;
    let mut trade_status = engine.trade_get_status(trade_id)?;
    sout!("\t -> Trade created with ID: {} and status {:?}", trade_id, trade_status);

    // Submit the trade - status should transition to "PendingApproval"
    engine.submit(USER_TRADER_1, trade_id)?;
    trade_status = engine.trade_get_status(trade_id)?;
    sout!("\t -> Trade status after submission: {:?}", trade_status);

    // Admin approve the trade - status should transition to "Approved"
    engine.approve(USER_ADMIN_1, trade_id)?;
    trade_status = engine.trade_get_status(trade_id)?;
    sout!("\t -> Trade status after approval: {:?}", trade_status);

    // Send the trade to the counterparty - status should transition to "SentToCounterparty"
    engine.send_to_execute(USER_ADMIN_1, trade_id)?;
    trade_status = engine.trade_get_status(trade_id)?;
    sout!("\t -> Trade status after sending to counterparty: {:?}", trade_status);

    // Execute the trade - status should transition to "Executed"
    engine.book(USER_TRADER_1, trade_id)?;
    trade_status = engine.trade_get_status(trade_id)?;
    sout!("\t -> Trade status after execution: {:?}", trade_status);

    Ok(())
}

pub(crate) fn trade_history_view() -> Result<(), AppError> {
    iout!("Viewing History :: A table of all actions with details");

    let engine = engine();

    // Get a list of all trade IDs
    // sorts by Snowflake ID which is time-based & monotonic
    let trade_ids = engine.trade_ids(true)?;
    if trade_ids.len() < 3 {
        sout!("Less than 3 trades available - run scenario 1, 2, and 3 first plz.");
        return Ok(());
    }

    let trade_id = trade_ids[2]; // third trade ID (index 2)
    let history = engine.trade_history(trade_id)?;

    sout!("\t -> Trade history table for id: {:?}", trade_id);

    let table = history_to_table(history)?; // uses the `prettytable` crate
    table.printstd();

    Ok(())
}

pub(crate) fn trade_hist_diff() -> Result<(), AppError> {
    iout!("Differences :: Show changes between versions");

    let engine = engine();

    // Get a list of all trade IDs
    let trade_ids = engine.trade_ids(true)?;
    if trade_ids.len() < 3 {
        sout!("Less than 3 trades available - run scenario 1, 2, and 3 first plz.");
        return Ok(());
    }

    // We'll use the third one for this example as it has amount that changes
    let trade_id = trade_ids[2]; // third trade ID (index 2)
    let history = engine.trade_history(trade_id)?;

    // Check that the history has at least 3 entries
    if history.len() < 3 {
        sout!("Not enough history entries to compare.");
        return Ok(());
    }
    sout!("\t -> This trade has {:?} history items (snapshots)", history.len());
    let diff = engine.diff(trade_id, 0, 2)?;

    // Display is implemented in the `Diff` struct
    sout!("\t -> Diff between snapshots 0 and 2:");
    sout!("\n{}", diff);

    Ok(())
}

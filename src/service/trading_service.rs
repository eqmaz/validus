// service layer
#[allow(dead_code)]
use app_core::AppError;
use rust_decimal::prelude::*;
use trade_core::model::{Currency, Direction, TradeDetails};

use crate::state::trading_state::engine;

const USER_TRADER_1: &str = "userTrader1";
const USER_ADMIN_1: &str = "userAdmin1";

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
    sout!(
        "\t -> First trade created with ID: {} and status {:?}",
        trade_one,
        trade_one_status
    );

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
    sout!(
        "\t -> Trade created with ID: {} and status {:?}",
        trade_id,
        trade_status
    );

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
    sout!(
        "\t -> Trade created with ID: {} and status {:?}",
        trade_id,
        trade_status
    );

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

    Ok(())
}

pub(crate) fn trade_scenario_3() -> Result<(), AppError> {
    iout!("Scenario 1 :: Approved trade sent to counterparty & marked as executed.");

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
    sout!(
        "\t -> Trade created with ID: {} and status {:?}",
        trade_id,
        trade_status
    );

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

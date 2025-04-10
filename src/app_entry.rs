use app_core::prelude::*;
//use serde_json::json;
use rust_decimal::prelude::*;
use trade_core::model::*;
use trade_core::*;

pub fn run(app: &mut AppContext) -> Result<(), AppError> {
    sout!("App Started!");
    if app.feature_enabled("dev_mode") {
        sout!("Developer mode is ON");
    }

    // Create a new instance of the trade engine
    let trade_store = store::InMemoryStore::new();
    let mut engine = TradeEngine::new(trade_store);

    // Create a new trade
    let details = TradeDetails {
        trading_entity: "foo".to_string(),
        counterparty: "bar".to_string(),
        direction: Direction::Buy,
        notional_currency: Currency::GBP,
        notional_amount: Decimal::from_str("22.5").unwrap(),
        //notional_amount: Decimal::new(-2, 0),
        underlying: vec![Currency::GBP, Currency::USD],
        trade_date: Default::default(),
        value_date: Default::default(),
        delivery_date: Default::default(),
        strike: None,
    };

    //let trade_id = engine.create("123", details).map_err(|err| err.log_and_display())?;
    let trade_one = engine.create("userTrader1", details)?;
    let mut trade_one_status = engine.trade_get_status(trade_one)?;
    iout!(
        "Trade one created with ID: {} and status {:?}",
        trade_one,
        trade_one_status
    );

    // there should be 1 item in the history
    let mut trade_one_hist = engine.history(trade_one)?;
    iout!("Trade one history count: {:?}", trade_one_hist.len());

    // Now submit the draft trade
    engine.submit("userTrader1", trade_one)?;
    trade_one_status = engine.trade_get_status(trade_one)?;
    iout!("Trade status after submission: {:?}", trade_one_status);

    trade_one_hist = engine.history(trade_one)?;
    iout!("Trade one history count: {:?}", trade_one_hist.len());

    // Now approve the trade
    engine.approve("userAdmin1", trade_one)?;
    trade_one_status = engine.trade_get_status(trade_one)?;
    iout!("Trade status after approval: {:?}", trade_one_status);

    Ok(())
}

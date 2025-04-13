use rust_decimal::Decimal;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use serde_json::json;
use openapi::{models as api, models};
use trade_core::model::{Currency, Direction, TradeDetails, TradeEventSnapshot};
use app_core::AppError;

pub fn to_trade_details(api: &api::TradeDetails) -> Result<TradeDetails, AppError> {
    let direction_raw = api.direction.clone().ok_or_else(|| AppError::new("100", "Missing direction"))?;
    let direction = Direction::from_str(&direction_raw)
        .ok_or_else(
            || AppError::new("100", "Invalid direction")
                .with_tag("trade_details")
                .with_data("direction", direction_raw.parse().unwrap())
        )?;

    let currency_raw = api.notional_currency
        .clone()
        .ok_or_else(|| AppError::new("100", "Missing currency"))?;
    let notional_currency = currency_raw.parse::<Currency>()
        .map_err(
            |_| AppError::new("100", "Invalid currency")
                .with_tag("trade_details")
                .with_data("currency", json!(currency_raw))
        )?;


    let notional_f64 = api.notional_amount.ok_or_else(|| AppError::new("100", "Missing notional_amount"))?;
    let notional_amount = Decimal::from_f64(notional_f64)
        .ok_or_else(
            || AppError::new("100", "Invalid notional amount")
                .with_tag("trade_details")
                .with_data("notional_amount", json!(notional_f64))
        )?;

    let underlying = api
        .underlying
        .clone()
        .unwrap_or_default()
        .into_iter()
        .map(
            |s| s.parse::<Currency>().map_err(
                |e| AppError::from_error(e)
                    .with_tag("trade_details")
                    .with_data("underlying", json!(s))
            )
        )
        .collect::<Result<Vec<_>, _>>()?;


    Ok(TradeDetails {
        trading_entity: api.trading_entity.clone().ok_or_else(|| AppError::new("100", "Missing trading_entity"))?,
        counterparty: api.counterparty.clone().ok_or_else(|| AppError::new("100", "Missing counterparty"))?,
        direction,
        notional_currency,
        notional_amount,
        underlying,
        trade_date: api.trade_date.unwrap_or_default(),
        value_date: api.value_date.unwrap_or_default(),
        delivery_date: api.delivery_date.unwrap_or_default(),
        strike: None,
    })
}

pub fn to_history_response(
    history: &[TradeEventSnapshot],
) -> Result<Vec<models::TradeEvent>, AppError> {
    Ok(history
        .iter()
        .map(|s| models::TradeEvent {
            user_id: Some(s.user_id.clone()),
            timestamp: Some(s.timestamp),
            state: Some(s.to_state.to_string()), // Ensure TradeState: Display
            details: Some(models::TradeDetails {
                trading_entity: Some(s.details.trading_entity.clone()),
                counterparty: Some(s.details.counterparty.clone()),
                direction: Some(s.details.direction.to_string()), // Ensure Direction: Display
                notional_currency: Some(s.details.notional_currency.clone().to_string()),
                notional_amount: Some(s.details.notional_amount.to_f64().unwrap()),
                underlying: Some(s.details.underlying
                        .iter()
                        .map(|c| c.to_string())
                        .collect()),
                trade_date: Some(s.details.trade_date),
                value_date: Some(s.details.value_date),
                delivery_date: Some(s.details.delivery_date),
                strike: s.details.strike.map(|d| d.to_f64().unwrap_or(0.0)),
            }),
        })
        .collect())
}
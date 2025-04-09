use crate::model::{Currency, Direction};
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TradeDetails {
    pub trading_entity: String,
    pub counterparty: String,
    pub direction: Direction,
    pub notional_currency: Currency, // Currency for better type safety
    pub notional_amount: Decimal,
    pub underlying: Vec<Currency>,
    pub trade_date: NaiveDate,
    pub value_date: NaiveDate,
    pub delivery_date: NaiveDate,
    pub strike: Option<Decimal>, // Decimal for guaranteed precision
}

impl TradeDetails {
    pub fn validate(&self) -> Result<(), String> {
        if let Some(amount) = &self.notional_amount {
            if amount <= &Decimal::ZERO {
                return Err("Notional amount must be positive".into());
            }
        } else {
            return Err("Notional amount must be provided".into());
        }

        if !self.underlying.contains(&self.notional_currency) {
            return Err("Notional currency must be included in underlying".into());
        }

        if self.trade_date > self.value_date {
            return Err("Trade date must be on or before value date".into());
        }

        if self.value_date > self.delivery_date {
            return Err("Value date must be on or before delivery date".into());
        }

        Ok(())
    }
}

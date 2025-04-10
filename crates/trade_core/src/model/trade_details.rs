use crate::errors::ValidationError;
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
    pub fn validate(&self) -> Result<(), ValidationError> {
        // Check that notional amount is present and positive
        if self.notional_amount < Decimal::ZERO {
            return Err(ValidationError::NegativeAmount(self.notional_amount));
        }

        if self.notional_amount == Decimal::ZERO {
            return Err(ValidationError::DetailsInvalid("Notional amount is required".into()));
        }

        // Check that underlying currency has at least one entry
        if self.underlying.is_empty() {
            return Err(ValidationError::EmptyUnderlying(
                "Underlying currency must be present".into(),
            ));
        }

        // Check that notional currency is present in underlying currency list
        if !self.underlying.contains(&self.notional_currency) {
            return Err(ValidationError::NoUnderlyingCcy(self.notional_currency));
        }

        // Check that trade date is on or before value date
        if self.trade_date > self.value_date {
            return Err(ValidationError::InvalidTradeDate(
                self.trade_date,
                "Trade date must be on or before value date".into(),
            ));
        }

        if self.value_date > self.delivery_date {
            return Err(ValidationError::InvalidValueDate(
                self.value_date,
                "Value date must be on or before delivery date".into(),
            ));
        }

        Ok(())
    }
}

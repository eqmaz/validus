use crate::model::{Currency, TradeId, TradeState};
use app_core::{AppError, ErrorCode};
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde_json::json;

#[derive(Debug)]
pub enum ErrCodes {
    T0001, // User for re-approvals must be original requester
    TNF01, // Trade not found
    TST02, // Invalid state transition
    TDI03, // Invalid trade details
    TUA04, // Unauthorized action
    TIN05, // Internal error
    TAF06, // Trade already final
    TAM07, // Negative amount
    TIC08, // Invalid currency
    TUE09, // Empty underlying
    TUC10, // No underlying currency
    TTD11, // Invalid trade date
    TVD12, // Invalid value date
    TDI13, // New details identical to existing
    TOR14, // Original requester cannot first-approve
}

impl ErrorCode for ErrCodes {
    fn code(&self) -> &'static str {
        match self {
            ErrCodes::T0001 => "T0001",
            ErrCodes::TNF01 => "TNF01",
            ErrCodes::TST02 => "TST02",
            ErrCodes::TDI03 => "TDI03",
            ErrCodes::TUA04 => "TUA04",
            ErrCodes::TIN05 => "TIN05",
            ErrCodes::TAF06 => "TAF06",
            ErrCodes::TAM07 => "TAM07",
            ErrCodes::TIC08 => "TIC08",
            ErrCodes::TUE09 => "TUE09",
            ErrCodes::TUC10 => "TUC10",
            ErrCodes::TTD11 => "TTD11",
            ErrCodes::TVD12 => "TVD12",
            ErrCodes::TDI13 => "TDI13",
            ErrCodes::TOR14 => "TOR14",
        }
    }

    fn format(&self) -> &'static str {
        match self {
            ErrCodes::T0001 => "User for re-approvals must be original requester",
            ErrCodes::TNF01 => "Trade not found",
            ErrCodes::TST02 => "Invalid state transition",
            ErrCodes::TDI03 => "Invalid trade details",
            ErrCodes::TUA04 => "Unauthorized action",
            ErrCodes::TIN05 => "Internal error",
            ErrCodes::TAF06 => "Trade already final",
            ErrCodes::TAM07 => "Amount cannot be negative",
            ErrCodes::TIC08 => "Unsupported or invalid currency",
            ErrCodes::TUE09 => "Underlying is empty",
            ErrCodes::TUC10 => "Underlying has no associated currency",
            ErrCodes::TTD11 => "Invalid trade date: {0}",
            ErrCodes::TVD12 => "Invalid value date: {0}",
            ErrCodes::TDI13 => "New trade details are identical to existing",
            ErrCodes::TOR14 => "Original requester cannot perform first-approval",
        }
    }

    fn kind(&self) -> &'static str {
        "engine"
    }
}

#[derive(Debug)]
pub enum ValidationError {
    TradeNotFound(TradeId),
    InvalidTransition(TradeState, TradeState),
    DetailsInvalid(String),
    Unauthorized(String),
    Internal(String),
    AlreadyFinal(TradeState),
    NegativeAmount(Decimal),
    InvalidCurrency(Currency),
    EmptyUnderlying(String),
    NoUnderlyingCcy(Currency),
    InvalidTradeDate(NaiveDate, String),
    InvalidValueDate(NaiveDate, String),
}

impl From<String> for ValidationError {
    fn from(e: String) -> Self {
        ValidationError::DetailsInvalid(e)
    }
}

// Automatic conversion from our internal ValidationError to AppError
impl From<ValidationError> for AppError {
    fn from(err: ValidationError) -> Self {
        match err {
            ValidationError::TradeNotFound(id) => {
                AppError::from_code(ErrCodes::TNF01, json!({ "trade_id": id })).with_tags(&["validation"])
            }
            ValidationError::InvalidTransition(from, to) => {
                let payload = json!({"from": from, "to": to});
                AppError::from_code(ErrCodes::TST02, payload).with_tags(&["validation", "state"])
            }
            ValidationError::DetailsInvalid(msg) => {
                AppError::from_code(ErrCodes::TDI03, json!({ "reason": msg })).with_tags(&["validation", "details"])
            }
            ValidationError::Unauthorized(reason) => {
                AppError::from_code(ErrCodes::TUA04, json!({ "reason": reason })).with_tags(&["auth"])
            }
            ValidationError::Internal(msg) => {
                AppError::from_code(ErrCodes::TIN05, json!({ "msg": msg })).with_tags(&["internal"])
            }
            ValidationError::AlreadyFinal(state) => {
                AppError::from_code(ErrCodes::TAF06, json!({ "state": state })).with_tags(&["state"])
            }
            ValidationError::NegativeAmount(amount) => {
                AppError::from_code(ErrCodes::TAM07, json!({ "amount": amount })).with_tags(&["validation", "amount"])
            }
            ValidationError::InvalidCurrency(ccy) => {
                AppError::from_code(ErrCodes::TIC08, json!({ "currency": ccy })).with_tags(&["validation", "currency"])
            }
            ValidationError::EmptyUnderlying(name) => {
                AppError::from_code(ErrCodes::TUE09, json!({ "underlying": name }))
                    .with_tags(&["validation", "underlying"])
            }
            ValidationError::NoUnderlyingCcy(ccy) => AppError::from_code(ErrCodes::TUC10, json!({ "currency": ccy }))
                .with_tags(&["validation", "underlying"]),
            ValidationError::InvalidTradeDate(date, reason) => {
                let payload = json!({"date": date, "reason": reason});
                AppError::from_code(ErrCodes::TTD11, payload).with_tags(&["validation", "dates"])
            }
            ValidationError::InvalidValueDate(date, reason) => {
                let payload = json!({"date": date, "reason": reason});
                AppError::from_code(ErrCodes::TVD12, payload).with_tags(&["validation", "dates"])
            }
        }
    }
}

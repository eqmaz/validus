// CURRENTLY UNDER HEAVY DEVELOPMENT

use serde_json::json;
use crate::model::{TradeId, TradeState};
use app_core::{app_err, AppError, ErrorCode};

// TODO use app_core error instead

#[derive(Debug)]
pub enum ValidationError {
    TradeNotFound(TradeId),
    InvalidTransition(TradeState, TradeState),
    DetailsInvalid(String),
    Unauthorized(String),
    Internal(String),
    AlreadyFinal(TradeState),
}

impl From<String> for ValidationError {
    fn from(e: String) -> Self {
        ValidationError::DetailsInvalid(e)
    }
}

impl From<ValidationError> for AppError {
    fn from(err: ValidationError) -> Self {
        match err {
            ValidationError::TradeNotFound(id) => {
                AppError::from_code(ErrCodes::TNF01, json!({ "trade_id": id }))
                    .with_tags(&["validation"])
            }
            ValidationError::InvalidTransition(from, to) => {
                let payload = json!({
                    "from": from,
                    "to": to
                });
                AppError::from_code(ErrCodes::TST02, payload)
                    .with_tags(&["validation", "state"])
            }
            ValidationError::DetailsInvalid(msg) => {
                app_err!("E400", msg).with_kind("validation")
            }
            ValidationError::Unauthorized(reason) => {
                app_err!("E401", reason).with_kind("auth")
            }
            ValidationError::Internal(msg) => {
                app_err!("E500", msg).with_kind("internal")
            }
            ValidationError::AlreadyFinal(state) => {
                app_err!("E409", "Trade is already finalized")
                    .with_kind("state")
                    .with_data("state", json!(state))
            }
        }
    }
}



pub mod err_kind {
    pub const ENGINE: &str = "engine";
}

#[derive(Debug)]
pub enum ErrCodes {
    T0001,
    TNF01,
    TST02
}

impl ErrorCode for ErrCodes {
    fn code(&self) -> &'static str {
        match self {
            ErrCodes::T0001 => "T0001",
            ErrCodes::TNF01 => "TNF01",
            ErrCodes::TST02 => "TST02"
        }
    }

    fn format(&self) -> &'static str {
        match self {
            ErrCodes::T0001 => "User for re-approvals must be original requester",
            ErrCodes::TNF01 => "Trade not found",
            ErrCodes::TST02 => "Invalid state transition"
        }
    }

    fn kind(&self) -> &'static str {
        err_kind::ENGINE
    }
}

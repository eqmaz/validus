use crate::model::TradeDetails;
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn current_timestamp_ms() -> u64 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO);
    now.as_millis() as u64
}

pub type DiffMap = HashMap<String, (String, String)>;

pub fn diff_details(from: &TradeDetails, to: &TradeDetails) -> DiffMap {
    let mut diffs = DiffMap::new();

    macro_rules! diff_field {
        ($field:ident) => {
            if from.$field != to.$field {
                diffs.insert(
                    stringify!($field).to_string(),
                    (format!("{:?}", from.$field), format!("{:?}", to.$field)),
                );
            }
        };
    }

    diff_field!(trading_entity);
    diff_field!(counterparty);
    diff_field!(direction);
    diff_field!(style);
    diff_field!(notional_currency);
    diff_field!(notional_amount);
    diff_field!(underlying);
    diff_field!(trade_date);
    diff_field!(value_date);
    diff_field!(delivery_date);
    diff_field!(strike);

    diffs
}

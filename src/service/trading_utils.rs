use app_core::AppError;
use chrono::{DateTime, Utc};
use prettytable::{row, Table};
use trade_core::model::TradeEventSnapshot;

/// Converts Trade history (vector of `TradeEventSnapshot`) into a `prettytable::Table`.
pub fn history_to_table(history: Vec<TradeEventSnapshot>) -> Result<Table, AppError> {
    let mut table = Table::new();
    table.add_row(row![
        //"Trade ID",
        "Snapshot",
        "User",
        "Timestamp",
        "From",
        "To",
        "Amount",
        "Ccy",
        "Entity",
        "Counterpty"
    ]);
    for event in history {
        let ts: DateTime<Utc> = DateTime::<Utc>::from(event.timestamp);
        table.add_row(row![
            //trade_id.to_string(),
            event.snapshot_id,
            event.user_id,
            ts.format("%Y-%m-%d %H:%M:%S"),
            format!("{:?}", event.from_state),
            format!("{:?}", event.to_state),
            event.details.notional_amount,
            format!("{:?}", event.details.notional_currency),
            event.details.trading_entity,
            event.details.counterparty,
        ]);
    }
    Ok(table)
}

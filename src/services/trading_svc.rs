use app_core::prelude::*;
use trade_core::runtime::TradeRuntime;
use crate::state::global_runtime;

/// Starts the trading runtime and stores it globally
pub fn start_trading_runtime(app: &AppContext) {
    let config = app.get_config();
    let mode = config.get("trade.mode").unwrap_or("default".into());

    sout!("Starting trading service in mode: {}", mode);

    let runtime = TradeRuntime::new(mode);
    global_runtime::set_global(runtime);
}


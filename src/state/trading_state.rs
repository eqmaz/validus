use std::sync::{Arc, Mutex};
use once_cell::sync::OnceCell;
use trade_core::runtime::Engine as TradeEngine;

static TRADE_ENGINE: OnceCell<Arc<Mutex<TradeEngine>>> = OnceCell::new();

/// Stores the runtime globally (should be called only once)
pub fn set_global(runtime: TradeEngine) {
    TRADE_ENGINE
        .set(Arc::new(Mutex::new(runtime)))
        .expect("TradeRuntime already initialized");
}

/// Accessor for the global trade runtime
pub fn get_global() -> Arc<Mutex<TradeEngine>> {
    TRADE_ENGINE
        .get()
        .expect("TradeRuntime not initialized")
        .clone()
}

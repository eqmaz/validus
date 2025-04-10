//! Global Trading Engine State
//!
//! This module defines the application-wide trade engine instance, built on top of
//! an `InMemoryStore` - for now - which uses fine(er)-grained concurrency (`DashMap`) to
//! allow multiple threads to access and mutate trades in parallel without locking
//! the entire engine.
//!
//! # Design
//! - The `TradeEngine` wrapped in an `Arc`, so it to be shared across threads.
//! - No `Mutex` is used at the engine level to avoid global lock bottlenecks.
//!
//! # Usage
//! - Use [`engine()`] to access the global singleton instance of the trade engine.
//!   It is lazily initialized on first use, and shared across the application.
//!
//! - Use [`engine_instance()`] if you need to create a separate instance (e.g. in unit tests)
//!   without interfering with the global state.
//!
//! # Example
//! ```rust
//! use state::trading_state::engine;
//!
//! let trade_id = engine().create("user1", trade_details)?;
//! ```

use once_cell::sync::Lazy;
use std::sync::Arc;
use trade_core::engine::TradeEngine;
use trade_core::store::InMemoryStore;

// The Mutex will go more granular at the trade/store level, to allow concurrent access
// So Arc<> will suffice here
pub type SharedTradeEngine = Arc<TradeEngine>;

static ENGINE: Lazy<SharedTradeEngine> = Lazy::new(|| {
    let store = InMemoryStore::new();
    Arc::new(TradeEngine::new(store))
});

/// Public access to the global trade engine
/// We only have one per application
pub fn engine() -> &'static SharedTradeEngine {
    &ENGINE
}

// For testing, when we need multiple instances
#[allow(dead_code)]
pub fn engine_instance() -> SharedTradeEngine {
    let store = InMemoryStore::new();
    Arc::new(TradeEngine::new(store))
}

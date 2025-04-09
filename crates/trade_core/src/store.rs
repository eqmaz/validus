use crate::model::{Trade, TradeId};
use std::collections::HashMap;
use app_core::AppError;

/// Just going with a simple HashMap for now, nothing too fancy
/// This is obviously not sustainable for a production system as we'd run out of memory!
pub struct InMemoryStore {
    trades: HashMap<TradeId, Trade>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        Self {
            trades: HashMap::new(),
        }
    }
}

/// TradeStore - the trait / interface for the trade store
/// Can be an in-memory or DB store etc
pub trait TradeStore: Send + Sync {
    fn push(&mut self, trade: Trade) -> TradeId;
    fn get(&self, trade_id: TradeId) -> Option<Trade>;
    fn has(&self, trade_id: TradeId) -> bool;
    fn update(&self, trade: Trade);
}

impl TradeStore for InMemoryStore {
    /// Push a trade to the store
    fn push(&mut self, trade: Trade) -> TradeId {
        let trade_id = trade.id;
        self.trades.insert(trade_id, trade);
        trade_id
    }

    /// Get a trade by ID
    fn get(&self, trade_id: TradeId) -> Option<Trade> {
        self.trades.get(&trade_id).cloned()
    }

    /// Check if the trade exists in the store
    fn has(&self, trade_id: TradeId) -> bool {
        self.trades.contains_key(&trade_id)
    }

    /// We would never really need update entries because
    /// The trade envelope is immutable. With this design
    /// we are just appending states to the trade history
    /// This is only if we want to completely replace the trade container
    fn update(&mut self, trade: Trade) -> Result<(), String> {
        // Just replace the trade found in the hashmap if found by id, with trade
        match self.trades.get_mut(&trade.id) {
            Some(trade_found) => {
                *trade_found = trade;
                Ok(())
            }
            // Trade not found - could handle, or just fail silently?
            None => Err(format!("Trade with ID {:?} not found", trade.id)),
        }
    }
}

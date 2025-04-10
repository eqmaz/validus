use crate::model::{Trade, TradeId};
use dashmap::DashMap;
//use std::collections::HashMap;

/// Just going with a simple HashMap for now, nothing too fancy
/// This is obviously not sustainable for a production system as we'd run out of memory!
pub struct InMemoryStore {
    trades: DashMap<TradeId, Trade>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        Self { trades: DashMap::new() }
    }
}

/// TradeStore - the trait / interface for the trade store
/// Can be an in-memory or DB store etc
pub trait TradeStore: Send + Sync {
    fn push(&mut self, trade: Trade) -> TradeId;
    fn get(&self, trade_id: TradeId) -> Option<Trade>;
    fn has(&self, trade_id: TradeId) -> bool;
    fn update(&mut self, trade: Trade) -> Result<(), String>;
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
        // self.trades.get(&trade_id).cloned() // Hashmap version
        self.trades.get(&trade_id).map(|entry| entry.clone()) // DashMap version
    }

    /// Check if the trade exists in the store
    fn has(&self, trade_id: TradeId) -> bool {
        self.trades.contains_key(&trade_id)
    }

    /// Update a trade in the store (replace it with a new one)
    ///
    /// We would never really need update entries with better thread safety optimization.
    /// But right now we are taking a COPY of the trade and then replacing it here.
    /// The trade envelope is basically immutable.
    /// With this design we are just appending state to the trade history
    fn update(&mut self, trade: Trade) -> Result<(), String> {
        // Just replace the trade found in the hashmap if found by id, with trade
        match self.trades.get_mut(&trade.id) {
            //Some(trade_found) => {
            Some(mut trade_found) => {
                *trade_found = trade;
                Ok(())
            }
            // Trade not found - could handle, or just fail silently?
            None => Err(format!("Trade with ID {:?} not found", trade.id)),
        }
    }
}

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
    fn keys(&self) -> Vec<TradeId>;
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

    /// Get a list of all trade IDs in the store
    /// They will be in order of insertion
    fn keys(&self) -> Vec<TradeId> {
        self.trades.iter().map(|entry| entry.key().clone()).collect()
    }
}

// = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =
// Unit tests for direction.rs
// = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =
#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Currency, Direction, TradeDetails, TradeState}; // adjust path if needed
    use chrono::{TimeZone, Utc};
    use rust_decimal::prelude::FromPrimitive;
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;

    fn trade_details(quantity: f32) -> TradeDetails {
        TradeDetails {
            trading_entity: "BigBank".to_string(),
            counterparty: "ClientCo".to_string(),
            direction: Direction::Buy,
            notional_currency: Currency::USD,
            notional_amount: Decimal::from_f32(quantity).expect("invalid float"),
            underlying: vec![Currency::EUR],
            trade_date: Utc.with_ymd_and_hms(2025, 4, 10, 0, 0, 0).unwrap(),
            value_date: Utc.with_ymd_and_hms(2025, 4, 12, 0, 0, 0).unwrap(),
            delivery_date: Utc.with_ymd_and_hms(2025, 4, 15, 0, 0, 0).unwrap(),
            strike: Some(dec!(1.25)),
        }
    }

    fn create_trade(id: TradeId, user_id: &str) -> Trade {
        Trade::new(id, trade_details(150.0), user_id.to_string())
    }

    #[test]
    fn test_push_and_has_trade() {
        let mut store = InMemoryStore::new();
        let trade = create_trade(1, "alice");

        assert!(!store.has(trade.id));
        store.push(trade.clone());
        assert!(store.has(trade.id));
    }

    #[test]
    fn test_get_trade_success() {
        let mut store = InMemoryStore::new();
        let trade = create_trade(2, "bob");

        store.push(trade.clone());
        let fetched = store.get(trade.id);
        assert!(fetched.is_some());
        assert_eq!(fetched.unwrap().id, trade.id);
    }

    #[test]
    fn test_get_trade_not_found() {
        let store = InMemoryStore::new();
        assert!(store.get(42).is_none());
    }

    #[test]
    fn test_update_trade_success() {
        let mut store = InMemoryStore::new();
        let mut trade = create_trade(3, "charlie");

        store.push(trade.clone());

        trade.add_snapshot("charlie", TradeState::PendingApproval, trade_details(160.0));
        let result = store.update(trade.clone());

        assert!(result.is_ok());

        let fetched = store.get(trade.id).unwrap();
        assert_eq!(fetched.latest_details().unwrap().notional_amount, dec!(160.0));
        assert_eq!(fetched.current_state(), TradeState::PendingApproval);
    }

    #[test]
    fn test_update_trade_not_found() {
        let mut store = InMemoryStore::new();
        let trade = create_trade(999, "ghost");

        let result = store.update(trade);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Trade with ID 999 not found");
    }

    #[test]
    fn test_keys_list() {
        let mut store = InMemoryStore::new();
        let trade1 = create_trade(100, "trader1");
        let trade2 = create_trade(200, "trader2");

        store.push(trade1.clone());
        store.push(trade2.clone());

        let keys = store.keys();
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&trade1.id));
        assert!(keys.contains(&trade2.id));
    }

    #[test]
    fn test_trade_lifecycle_and_approval() {
        let mut trade = create_trade(777, "origin");

        assert_eq!(trade.current_state(), TradeState::Draft);
        assert_eq!(trade.get_requester(), "origin".to_string());
        assert_eq!(trade.get_first_approver(), None);

        trade.add_snapshot("approver", TradeState::PendingApproval, trade_details(150.0));

        assert_eq!(trade.get_first_approver(), Some("approver".to_string()));
        assert_eq!(trade.current_state(), TradeState::PendingApproval);
    }

    #[test]
    fn test_trade_details_persistence_on_update() {
        let mut store = InMemoryStore::new();
        let mut trade = create_trade(42, "alice");

        store.push(trade.clone());

        // Update with new details
        let updated_details = TradeDetails {
            trading_entity: "BigBank".to_string(),
            counterparty: "AnotherCo".to_string(),
            direction: Direction::Sell,
            notional_currency: Currency::EUR,
            notional_amount: dec!(2_000_000),
            underlying: vec![Currency::USD, Currency::JPY],
            trade_date: Utc.with_ymd_and_hms(2025, 5, 1, 0, 0, 0).unwrap(),
            value_date: Utc.with_ymd_and_hms(2025, 5, 3, 0, 0, 0).unwrap(),
            delivery_date: Utc.with_ymd_and_hms(2025, 5, 10, 0, 0, 0).unwrap(),
            strike: None,
        };

        trade.add_snapshot("bob", TradeState::PendingApproval, updated_details.clone());
        store.update(trade.clone()).unwrap();

        let fetched = store.get(trade.id).unwrap();
        let current_details = fetched.latest_details().unwrap();

        assert_eq!(current_details, &updated_details);
    }
}

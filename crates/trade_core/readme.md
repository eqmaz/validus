# About Trade Core Crate

- Lives in /crates/trade_core


## Flows

### Submit Flow
1. Validate trade details (e.g., trade_date ≤ value_date ≤ delivery_date).
2. Generate new Trade Id
3. Construct TradeSnapshot version 1 (state = Draft).
4. Store it
5. Record initial history entry.
6. Return TradeId.


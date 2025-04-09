```text
trade_core/
└── src/
    ├── lib.rs                # Exposes modules and hello_trade()
    ├── workflow/             # Core API struct (TradeWorkflow)
    ├── model/                # Domain models (Trade, enums, etc.)
    ├── state_machine/        # State transitions, valid actions
    ├── validation/           # Date/order/field validators
    ├── store/                # In-memory, thread-safe store
    ├── history/              # History, versioning, diff engine
    ├── error/                # Custom error types
    └── util/                 # Optional helpers (e.g. time)
```


## Flows

### Submit Flow
1. Validate trade details (e.g., trade_date ≤ value_date ≤ delivery_date).
2. Generate new Trade Id
3. Construct TradeSnapshot version 1 (state = Draft).
4. Store it 
5. Record initial history entry.
6. Return TradeId.


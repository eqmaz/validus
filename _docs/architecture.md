# Project Structure

## Directories
- crates/app_core	✅ Logging, CLI/bootstrap config, env vars
- crates/trade_core	✅ Domain models (Trade, TradeDetails, State, etc.), validators, state machine logic, in-memory repository, version/diff tracking
- src/main.rs	    ✅ Business orchestration and testing ground (e.g., driving trade submissions, approvals, full scenario tests). Later: REST API surface


## Application layers
```text
    ┌────────────────────────────┐
    │    [ Some Public API ]     │  ← REST / gRPC / FIX / (bolt-on layer)
    └────────────▲───────────────┘
                 │
    ┌────────────────────────────┐
    │    [ Service layer ]       │  ← DDD orchestration layer with higher level business logic
    └────────────▲───────────────┘
                 │         
    ┌────────────┴───────────────┐
    │    Trade Core Library      │  ← Public API Surface (internal trade workflow library)
    ├────────────────────────────┤
    │ Models                     │
    │ State Machine Logic        │
    │ Validation Rules           │
    | In-memory Store / History  │
    └────────────────────────────┘
  └────────────────────────────────┘ <<-------->> all sits on top of app_core mini framework        
```


### Usage in service layer, for Trade Core library
```rust

    // Where engine is a globally accessible isntance

    let trade_id = engine.submit_trade("user1", details)?;
    engine.approve_trade("user2", trade_id)?;


```



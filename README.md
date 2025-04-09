# WIP (WORK IN PROGRESS)

## This solution is work in progress. 

### At the time of this post I have had less than a day on it.

### This will be complete before our next meeting. Feel free to track additional commits.

### Meanwhile this scaffold should give you a reasonable idea of my approach, patterns and direction

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


## Coming shortly:
 - Complete engine lib
 - Service layer to interface between trade_core and any public API
 - API layer to support REST or gRPC requests
 - Tests

## What's here now:
- Trade engine library guts, models, state machine, validations, general flow of logic
- A Snowflake based ID generator
- "Interface" based storage pattern for trades (TradeStore) - in memory for now, can be swapped for a DB etc
- Application micro framework (app_core) with config loading, logging, console, errors etc
- Application bootstrap with DDD pattern, global errors, configs etc
- Some docs, to be completed



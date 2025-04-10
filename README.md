# WIP (WORK IN PROGRESS)

## Solution is work in progress. It will be complete before our meeting / in the next few hours. Feel free to track additional commits.

### The scaffold should give you a reasonable idea of my approach, patterns and direction

### Getting started
See Makefile - ``make help``<br>
Just run ``make run``

### Example output (hello world)
```text
[2025-04-10 12:56:08.960] ✔ Config initialized from config.toml
[2025-04-10 12:56:08.960] ✔ Logger initialized to ./logs/app.log [debug]
[2025-04-10 12:56:08.960] ✔ App Started!
[2025-04-10 12:56:08.961] ℹ Trade one created with ID: 185749655617839104 and status Draft
[2025-04-10 12:56:08.961] ℹ Trade one history count: 1
[2025-04-10 12:56:08.961] ℹ Trade status after submission: PendingApproval
[2025-04-10 12:56:08.961] ℹ Trade one history count: 2
[2025-04-10 12:56:08.961] ℹ Trade status after approval: Approved
```


### Public API coming...

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
 - Better thread-safe performance in the engine (locking at more granular level etc)
 - Functions to demo the example scenarios (scenario1, scenario2 etc)
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


## Cheers, speak soon!

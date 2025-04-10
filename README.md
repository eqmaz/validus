# WIP (LIVE UPDATES COMING)

## Improvement in progress; App runs 3x example scenarios. 

### The scaffold here should give you a good idea of my approach, patterns and direction. Will be complete before our meeting / or in the next few hours. Feel free to track commits.

### With every new commit, it will still run without issue. Docs will be updated accordingly 

### Getting started
See Makefile - ``make help``<br>
Just run ``make run``

### console output (3x scenarios as per brief)
```text
[2025-04-10 17:16:05.010] ✔ Config initialized from config.toml
[2025-04-10 17:16:05.010] ✔ Logger initialized to ./logs/app.log [debug]
[2025-04-10 17:16:05.011] ✔ App Started!
[2025-04-10 17:16:05.011] ℹ Hello world scenario
[2025-04-10 17:16:05.011] ✔      -> First trade created with ID: 185815070192738304 and status Draft
[2025-04-10 17:16:05.011] ✔      -> Trade history count: 1

[2025-04-10 17:16:05.011] ℹ Scenario 1 :: Submitting and Approving a Trade
[2025-04-10 17:16:05.011] ✔      -> Trade created with ID: 185815070192738305 and status Draft
[2025-04-10 17:16:05.011] ✔      -> Trade status after submission: PendingApproval
[2025-04-10 17:16:05.011] ✔      -> Notional amount form trade details: 55.6
[2025-04-10 17:16:05.011] ✔      -> Trade status after approval: Approved
[2025-04-10 17:16:05.011] ✔      -> Trade status after approval: Approved

[2025-04-10 17:16:05.011] ℹ Scenario 2 :: An approver updates the trade details, requiring re-approval.
[2025-04-10 17:16:05.011] ✔      -> Trade created with ID: 185815070192738306 and status Draft
[2025-04-10 17:16:05.011] ✔      -> Trade status after update: NeedsReapproval
[2025-04-10 17:16:05.011] ✔      -> Trade status after re-approval: Approved

[2025-04-10 17:16:05.011] ℹ Scenario 3 :: Approved trade sent to counterparty & marked as executed.
[2025-04-10 17:16:05.011] ✔      -> Trade created with ID: 185815070192738307 and status Draft
[2025-04-10 17:16:05.011] ✔      -> Trade status after submission: PendingApproval
[2025-04-10 17:16:05.012] ✔      -> Trade status after approval: Approved
[2025-04-10 17:16:05.012] ✔      -> Trade status after sending to counterparty: SentToCounterparty
[2025-04-10 17:16:05.012] ✔      -> Trade status after execution: Executed
```
### history table output:
```text
[2025-04-10 18:43:07.822] ℹ Viewing History :: A table of all actions with details
[2025-04-10 18:43:07.822] ✔      -> Trade history table for id: 185836976245612546
+----------+-------------+---------------------+-----------------+-----------------+--------+-----+--------+------------+
| Snapshot | User        | Timestamp           | From            | To              | Amount | Ccy | Entity | Counterpty |
+----------+-------------+---------------------+-----------------+-----------------+--------+-----+--------+------------+
| 0        | userTrader1 | 2025-04-10 17:43:07 | Draft           | Draft           | 468.22 | GBP | foo    | bar        |
+----------+-------------+---------------------+-----------------+-----------------+--------+-----+--------+------------+
| 1        | userAdmin1  | 2025-04-10 17:43:07 | Draft           | NeedsReapproval | 368.02 | GBP | foo    | bar        |
+----------+-------------+---------------------+-----------------+-----------------+--------+-----+--------+------------+
| 2        | userTrader1 | 2025-04-10 17:43:07 | NeedsReapproval | Approved        | 368.02 | GBP | foo    | bar        |
+----------+-------------+---------------------+-----------------+-----------------+--------+-----+--------+------------+
```

### Difference API output
```text
[2025-04-10 21:59:10.501] ℹ Differences :: Show changes between versions
[2025-04-10 21:59:10.501] ✔      -> This trade has 3 history items (snapshots)
[2025-04-10 21:59:10.501] ✔      -> Diff between snapshots 0 and 2:
[2025-04-10 21:59:10.501] ✔ 
TradeDiff Report for Trade ID: 185886312488804353
Snapshot: 0 → 2
Changed by: userTrader1 → userTrader1
Timestamp: 2025-04-10 20:59:10.498492024 UTC → 2025-04-10 20:59:10.498676039 UTC
Changed fields:
  notional_amount: 468.22 → 368.02

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
  └────────────────────────────────┘ <<-------->> all sits on top of my own app_core mini framework
  
   !Everything built from scratch!        
```


## Coming shortly:
 - Better thread-safe performance in the engine (locking at more granular level etc)
 - API layer to support REST or gRPC requests
 - Few more tests

## Application includes:
- Functions to demo the example scenarios (scenario1, scenario2 etc)
- Trade engine library with models, state machine, validations, public method based API
  - create, submit, approve, reapprove, send-to-execute, book, history, diff etc
- A Snowflake based ID generator for trade IDs
- Abstracted TradeStore with in-memory implementation, interchangeable to DB etc 
- Service layer to interface between trade_core and any public API (REST, FIX, etc)
- Micro App framework (app_core) with config loading, logging, console, errors etc
- Clean main file just bootstraps dependencies and runs the app entry point
- Makefile for easy commands
- A few unit tests, more coming
- Docs


## Cheers, speak soon!

# CASE STUDY: TRADE APPROVAL SERVICE

Trades can be submitted for approval, following a structured workflow. The workflow is contained within an independently testable library (trade_core). 
A user can submit trade details for approval through an API layer - a basic REST scaffold is provided. 

[Supported actions](./_docs/trade_actions.md)

[Trade model](./_docs/trade_model.md)

[Trade states](./_docs/trade_states.md)

### Getting started
See Makefile for details ``make help`` or ``make run``
```shell
make help
make auto
```
To run the application you will **not need** to re-generate the REST API boilerplate code, 
as it's included in the repo. Note that ``make gen-api`` requires the openapi-generator-cli dependency. 
There is an installer in ``openapi/install-openapi-gen.sh``

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

## Test scenarios
The aplication will automatically run the test scenarios as per the brief, on start up, providing the config file has the following feature flag enabled:
```toml
[features]
dev_mode = true
```
These functions live in the application's service layer and are invoked once. Setting dev_mode to false will disable them from running.

### Console output for the scenarios as per brief
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

## Running unit and integration tests:
All test runners accessible from Make
```shell

make help | grep test

//  make test            - Run all unit tests
//  make test-app-core   - Unit tests for app-core framework
//  make test-trade-core - Unit tests for trade-core library
```


## App entry point:
/src/app_entry.rs

## What the app includes
- Functions to demo the example scenarios (scenario1, scenario2 etc)
- Trade engine library with models, state machine, validations, public method based API
  - create, submit, approve, reapprove, send-to-execute, book, history, diff etc
- Snowflake based ID generator for trade IDs
- Abstracted TradeStore with in-memory implementation, interchangeable to DB etc
- Service layer to interface between trade_core and any public API (REST, FIX, etc)
- Micro App framework (app_core) with config loading, logging, console, errors etc
- OpenAPI spec in yaml with code generator for RUST boilerplate code
- A simple REST API implementation with a couple of endpoints and stubs
- Clean main file just bootstraps dependencies and runs the app entry point
- Makefile for easy commands
- Unit tests for app_core and trade_core
- Docs

## Does not include:
- Authentication awareness (it's a hypothetical service)
- The REST API only has create and history endpoints + stubs for the rest

## What could be improved:
 - Better thread-safe performance in the engine (locking at more granular level etc)
 - Better memory efficiency in the in-memory store (not duplicating data)
 - User authentication stubs / awareness 
 - A better integration test suite
 - Nice validation layer for public API surface
 - Standardised REST response and error shapes
 - app_core framework is really basic
 - Versioning & version bump automation

## Cheers, speak soon!

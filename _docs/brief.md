# What are we building:
To implement:
 - a library 
 - with a suitable API 
 - to support a trade approval workflow.  
  
A **public API** should expose the core actions to consumers. Data can be stored in memory for this prototype

## The API must:
- Enforce validation rules for trade details
- Prevent invalid state transitions and unauthorized actions
- Allow users to view the history of a trade, including
  - Trade details at any previous state.
  - A tabular history of actions with user IDs, timestamps, and the state transitions.
  - Differences between two versions of trade details (e.g., updates to notional amounts).
- Allow trades to be sent to a counterparty and marked as executed

**Think beyond the given requirements and consider what extra value or features you could add to the case study**

## What the system needs to do:
- Allow users to:
  - submit
  - approve
  - cancel
  - update
  - send to counterparty
  - and book trades. 
- Expose a public API for:
    - Performing actions.
    - Viewing the history of each trade.
    - Comparing versions of a trade (showing diffs).

- Enforce state transitions (e.g., Draft → PendingApproval → Approved).
- Validate trade details (e.g., dates in correct order).
- Store trades and state history in memory.


## Definition of Success (test cases):
- The API enforces the correct workflow and state machine.
- Trade validations (e.g., TradeDate ≤ ValueDate ≤ DeliveryDate) are in place.
- Unauthorized actions and invalid transitions are blocked.
- Trade history and version diffs are viewable via the API.
- The solution is clean, modular, well-tested (unit + optionally integration tests).
- You document the API with clear examples.

## Test Matrix

| Success Item                                                                 | Type of Test        |
|------------------------------------------------------------------------------|----------------------|
| Can submit a valid trade → state becomes `PendingApproval`                  | Unit                |
| Cannot approve from `Draft` state                                            | Unit (Negative Case)|
| Cancelling a `SentToCounterparty` trade moves to `Cancelled`                | Unit                |
| Updated trade moves to `NeedsReapproval`, and original user must reapprove  | Integration         |
| API returns full history of state transitions and diffs                     | Integration         |

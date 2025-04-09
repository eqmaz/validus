# Project crates

For clean separation of concerns, the application scaffold (micro framework) and trade engine library 
are separate and outside the main business logic (main executable) in root /src. 

The trade engine library could be lifted out into a package such as another private repo if required. 

### app_core
Application micro framework. Config, logging, Outputs, CLI setup, etc.
- Lives in /crates/app_core
- Contains application framework
- common infra stff (config, logging, console, arguments parsing, container, etc.)

### trade_core
Trade workflow engine as described in the brief
- Lives in /crates/trade_core
- Domain types (Trade, TradeDetails, State, Direction, etc.)
- Validations (date ordering, field consistency)
- State machine logic (actions + transitions)
- In-memory store (HashMap for trades/history)
- Versioning + diff engine
# App Core Crate

Exposes:
- console colours
- console
- config wrapper
- app context / DI container / lifecycle manager
- logging wrapper
- a few macros
- a nice error system
- a few utils and traits for common things

These all work together harmoniously.

Because this is a tiny project, some are thread-safe singleton patterns however in 
larger projects or where testability is a big concern, they can be converted to instance-based.

Not really using app context as a DI container, but more as a lifecycle manager at this time.

## TODO / Could-do
- Event bus
- Timers (periodic and delayed)
 

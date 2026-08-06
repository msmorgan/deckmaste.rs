---
needs: []
---
**Engine: `sim.rs` has no mechanical strategy for the unbuilt decision kinds.**

`mechanical()` and `pending_player()` in `crates/deckmaste_engine/src/sim.rs`
both fall through to a loud arm for any `PendingDecision` kind outside the
wired subset. `crates/deckmaste_engine/src/strategy.rs` has the same shape at
its own fallback (`ChooseCostOptions`, `OrderReplacements`, `PreGame`,
`LegendRule`).

This is a *consequence* ticket, not a driver: each uncovered kind becomes
answerable once the mechanic that constructs it lands. Close it by sweeping
the fallbacks once the owning mechanic tickets
(`engine-order-replacements-decision`, `engine-pregame-procedure`,
`engine-voting-procedure`) are done, and by deciding whether the residual
fallback should stay loud or return a documented always-legal default.

Effort: **S** once its upstreams land.

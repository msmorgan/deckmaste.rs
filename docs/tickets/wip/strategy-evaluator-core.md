---
needs: [strategy-data-types, strategy-eval-context]
---
The `StrategyEvaluator`: turns a `Strategy` (data) + a `GameState` + a
`PendingDecision` into a `Decision`. Lives in `deckmaste_engine` (it calls
`pub(crate)` evaluators and emits `Decision`) and implements the EXISTING
`Strategy` trait (`crates/deckmaste_engine/src/sim.rs` —
`decide(&GameState, &PendingDecision) -> Decision`), so `play()`, the 50k
harness, and the TUI `Driver` consume it unchanged.

Core mechanics:
- Rule walk: the first `Rule` whose `when` (`condition_holds`) holds AND whose
  `prefer` resolves to a legal option wins.
- Selector engine: argmin/argmax of `by` (a `Count`, evaluated per candidate via
  a `This`-bound `Frame` from `strategy-eval-context`) over the `among`-filtered
  legal set; `Extremum::First` = enumeration order.
- **Totality**: never panic on an unhandled `PendingDecision`. Uncovered kinds —
  including the 8 shell kinds (ChooseModes / Division / Vote / YesNo /
  OrderReplacements / …) — fall back to the existing mechanical default (mirror
  today's `mechanical()` in `sim.rs`).

This ticket is the skeleton + dispatch + selector engine + totality. The
per-decision-kind handlers are split into `strategy-decision-handlers`.

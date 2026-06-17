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

## Status: DONE

`crates/deckmaste_engine/src/strategy.rs` (`StrategyEvaluator`; 5 module tests;
core/cards/engine full suites + clippy + `cargo build --workspace` all green).

- **Type relocation (supersedes `strategy-data-types`' placement).** The
  evaluator must live in `deckmaste_engine` (for the `pub(crate)` evaluators),
  but the engine only **dev**-depends on `deckmaste_cards`, so it can't import
  the strategy types from there in production. Resolved by moving
  `Strategy`/`Rule`/`Preference`/`Selector`/`Extremum`/`BlockPolicy` to
  **`deckmaste_core::strategy`** (a dedicated module, **not** re-exported at the
  core root — so `deckmaste_core::strategy::Strategy`, keeping the rules-
  primitive namespace clean). This is consistent with every other macroable type
  living in core; `Preference` is now registered in `deckmaste_core::ron::kinds()`
  and cards' `kinds()` is a thin delegation. Parse/round-trip tests moved to
  core; the macro-expansion tests stayed in `cards/macros.rs` (they need the
  cards macro registry).
- **`StrategyEvaluator { strategy, seat }`** implements the engine `Strategy`
  trait; public (re-exported from the engine), so `play()`/harness/TUI can adopt
  it.
- **Rule-walk** (`decide_priority`): first rule whose `when` holds (via
  `condition_holds` over a candidate-less `eval_frame`) AND whose `prefer`
  resolves to a legal action wins; falls through to `Pass`.
- **Selector engine** (`select`/`score`/`matches_among`): filter candidates by
  `among`, then `Min`/`Max` argmin/argmax of `by` (per-candidate `eval_frame`),
  `First` = enumeration order. Wired into the Priority handler (Pass / Play /
  Cast / Activate) — proving the machinery end-to-end.
- **Totality** (`fallback`): legal defaults for every kind that arises in v1
  decks plus the simple shells (ChooseModes/Division/Vote/YesNo/
  ChooseReplacement); declares no attackers / no blocks by default. The three
  deep shells with no trivial legal payload (ChooseCostOptions, OrderReplacements,
  PreGame) keep a documented `todo!` — none surface in v1 decks.

Deferred to `strategy-decision-handlers` (next): the smart per-kind handlers —
ChooseTargets via the selector, combat via `Attack`/`BlockPolicy`, `Discard`
via the selector, mana — replacing the fallback's legal defaults for those.

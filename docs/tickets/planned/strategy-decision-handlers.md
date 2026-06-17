---
needs: [strategy-evaluator-core]
---
Implement the per-`PendingDecision` handlers in the `StrategyEvaluator`, covering
exactly the decision kinds the hardcoded Greedy strategies make. See
`crates/deckmaste_engine/src/decide.rs` for the `PendingDecision` / `Decision` /
`Action` enums.

Handlers:
- `Priority` — cast-or-pass + which spell / land / ability (rules + selectors).
- `ChooseTargets` — extremum-selection among the already-legal targets.
- `DeclareAttackers` / `DeclareBlockers` — selected attacker set / coarse
  `BlockPolicy`.
- `DiscardToHandSize` / `DiscardCards` — `Discard(Selector)` (e.g. shed lands
  first).
- `ChooseManaColor` / `PayMana` — delegate to `state.auto_pay_pending()` /
  first option.
- `ChooseXValue` (X via a `Count`), `ChooseObjects` (take min count).
- `OrderTriggers`, `AssignCombatDamage` — mechanical default (identity / first).

Key invariant: **legality is free**. The engine only enumerates legal options,
so handlers never guard targeting legality (hexproof / protection / illegal
blocks) — only "which legal option is best." Effect-accomplishment guards ("is it
indestructible", "survives N damage") are the strategy author's deck-specific
predicate macros, not engine logic.

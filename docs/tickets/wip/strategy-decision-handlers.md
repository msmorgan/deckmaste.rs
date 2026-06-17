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

## Status: DONE

`crates/deckmaste_engine/src/strategy.rs` (5 new tests; engine suite + clippy
green). A shared rule-walk (`first_applicable`) drives all the new handlers; each
passes an extractor for the part of the (macro-resolved) preference it needs.

- **`ChooseTargets`** — applies the applicable `Cast`/`Activate` preference's
  `target` selector per spec slot (argmin/argmax over the slot's legal
  candidates), falling back to the first legal candidate when no rule supplies a
  selector.
- **`DeclareAttackers`** — the legal attackers matching the applicable `Attack`
  preference's `among` (the whole set when `among` is `None`); no `Attack` rule →
  none. (`pick`/`by` unused — attacking is a set decision.)
- **`DeclareBlockers`** — `Block` policy dispatch: `NoBlocks`/no rule → none;
  `BlockAll` pairs each legal blocker with a declared attacker (round-robin);
  `ChumpBiggest` sends all blockers at the highest-power attacker (via
  `state.combat.attackers()`). The engine re-validates each pair.
- **`DiscardToHandSize`/`DiscardCards`** — the `Discard` selector ranks the hand
  (`among`-matched first, whole hand if too few) and sheds the `count` cards at
  the `pick` end. `Discard(Min by ManaValue)` = "shed cheapest" ≈ lands first.
- `Priority` (evaluator-core), `ChooseManaColor`/`PayMana`/`OrderTriggers`/
  `AssignCombatDamage`/`ChooseXValue`/`ChooseObjects` keep the fallback's legal
  defaults — they need no per-strategy choice for v1.

Closes everything the greedy seats exercise; `strategy-greedy-port` (the winrate
-equivalence success criterion) is now unblocked.

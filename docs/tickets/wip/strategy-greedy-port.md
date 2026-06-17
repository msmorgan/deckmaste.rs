---
needs: [strategy-decision-handlers]
---
Prove the strategy language is expressive enough by reproducing known-good
behavior: re-express the three hardcoded Rust strategies — `GreedyCreatures`,
`GreedyRemoval`, `GreedyDemo` (`crates/deckmaste_engine/src/sim.rs`) — as RON
strategy files driving the `StrategyEvaluator`.

Includes:
- Builtin strategy-vocabulary macros these need: `Always`, plus the
  deck-specific predicate/selector macros — e.g. "cheapest creature" =
  `Cast(Min, by: StatOf(Candidate, ManaValue), among: Type(Creature))`,
  "biggest threat" target = `pick: Max, by: StatOf(Candidate, Power)`.
- **Success criterion**: run the ported strategies through the existing
  50,000-game Bears-vs-Bolts harness
  (`crates/deckmaste_engine/tests/full_game.rs`) and assert the winrate matches
  the Rust originals within statistical noise. Seeded determinism also allows
  per-seed game-trace equivalence for a handful of seeds as a stronger
  assertion (pick the tolerance, or prefer trace equivalence).

This closes v1 of the strategy-engine epic. Follow-ups (separate brainstorm +
spec, not yet ticketed): a generalized gauntlet (arbitrary deck-pair round-robin
+ winrate matrix) and a strategy optimizer (search/tune the numeric
`Count`/`Literal` weights to maximize winrate).

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

## Status: DONE (criterion revised — see finding)

`crates/deckmaste_engine/src/strategy.rs` (`from_ron` loader) +
`crates/deckmaste_engine/tests/full_game.rs` (2 tests; engine suite + clippy
green).

**Finding — exact greedy equivalence is not expressible today.** Greedy's
competitiveness is *entirely* its careful mana-ramp: tap *exactly enough* to
cast the cheapest creature, gated on `floated_mana + untapped_lands >= cost`.
A `CastSpell` only enters `legal` once mana is **already floated**
(`spendable_pool` is floated-only, `cast.rs`), so a strategy must ramp
explicitly — but the RON language has **no `Count` for floated pool mana**, so
that reachability gate is inexpressible. Empirically the gap is huge, not
"within noise": a naive RON ramp wins **97%** vs the Rust greedy's **49%** over
300 seeds. So the original winrate-equivalence criterion can't be met with the
current language.

**Revised criterion (per direction): a simple, fully-expressible tap-out line.**
Both matchup seats are authored as RON strategies (no hardcoded Rust policy):
- `RON_BEARS` (P0): in a main phase — play a land, tap out (`Activate`), cast
  every affordable creature; swing with everything; shed cheapest (lands first);
  pass.
- `RON_BOLTS` (P1): same shape; burn targets the biggest creature
  (`target: (pick: Max, by: StatOf(This, Power), among: Type(Creature))`),
  falling back to the face when there is none.

`ron_strategies_play_the_matchup_to_a_winner` asserts the data-driven matchup
reaches the *same sensible end-state shape* the Rust-greedy game does (real
loss, a Bear connects, a Bolt kills a Bear); `ron_strategies_are_deterministic`
pins reproducibility. This proves the data-driven engine drives a complete real
game end-to-end — the v1 goal — via `StrategyEvaluator::from_ron` (raw RON; the
macro-aware vocabulary loader and the `Always`/predicate macros are deferred).

**Follow-up to file:** `strategy-mana-count` — add a `Count` for the seat's
spendable (floated) mana so mana-aware ramp/sequencing (and true greedy
equivalence) become expressible. This is the one missing primitive the finding
isolates; it's also fundamental for any realistic deck strategy.

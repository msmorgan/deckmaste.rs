---
needs: []
design: true
---
Unlock the existential player-life board condition
`~ enters tapped unless a player has N or less life.` (~10 cards, flagged by the
parse-enters-tapped worker).

## Needs a design pass — the `Exists(<player-pred>)` shape is unsound (found 2026-07-08)

The predicate half is fine (`PlayerStatCmp` twin = `CharacteristicPredicate::Stat`
already exists, emitted at `idris_emit.rs`). The BLOCKER is the `Exists` wrapping
the ticket prescribes:

- Idris `exists` is object-scoped ONLY: `exists : Predicate b AnObject -> Condition b`
  (`idris/src/Core.idr`). `PlayerStatCmp` is `Predicate b APlayer`, so
  `exists (PlayerStatCmp …)` is an Idris TYPE ERROR (`APlayer` ≠ `AnObject`).
- The sound existential-over-players is the `Players` countable, not `exists`:
  `Compare (CountOf (Players (PlayerStatCmp Life AtMost (^N)))) Greater (Literal 0)`
  (cf. `CountOf (Players (Controls creature))` in `Spec.idr`). The Core.idr comment
  by `PlayerStatCmp` even shows it used as a FILTER, never under `exists`.
- **Rust can't emit `CountOf (Players <pred>)` for an arbitrary player predicate.**
  `Count::CountOf` unconditionally emits `Objects` (`idris_emit.rs`); the only
  `Players` emission is the bespoke `Count::Opponents(You)`. There is no general
  `Players`/`Countable` variant in Rust's `Count`.
- **The soundness trap**: `declines_existential_player_life` only asserts on the
  Rust RON string (never runs the Idris emitter), and the ~10 flagged cards live
  in `plugins/wizards/*.ron.todo`, not `plugins/canon` — so a naive
  `Exists(PlayerStat(…))` would flip the test green AND pass canon idris-check
  while emitting Idris that silently fails to typecheck. Latent unsoundness,
  contradicting the "idris-check count unchanged" intent.

Design decision required (RON authoring surface is user-ruled territory): either
(a) add a Rust `Count`/countable emitting `Players(<pred>)` and have the parser
build `Compare(CountOf-players, Greater, 0)` — NOT `Condition::Exists`; or (b)
make `Condition::Exists`'s emitter kind-aware, routing a player-scoped inner
predicate through the `Players` countable. Amend the ticket to pick one before
implementation. Surfaced by the batch executor.

Split out of `core-count-opponents-and-life`: that ticket assumed a life-total
`Count` primitive would unlock this form, but the grammar is **existential over
players** (∃ a player with life ≤ N), which is a *predicate*, not a
`Compare` over a single-player `Count`. `Count::PlayerStatOf(Reference,
PlayerAttr)` (a *specific* player's life) already landed there; this form needs
the player-attribute *comparison predicate*.

- Add a `Predicate` variant for "a player whose `PlayerAttr` compares `Cmp` to
  `Count`" — the 1:1 twin of the existing Idris
  `PlayerStatCmp : PlayerAttr -> Cmp -> Count -> Predicate APlayer`
  (`idris/src/Core.idr`). Emitter arm + structural renderer to match.
- Wire `parse_board_condition`
  (`crates/deckmaste_migrations/src/parsers/replacement.rs`) to build
  `Exists(<a player with life <= N>)` for the "a player has N or less life"
  shape. The sibling `unless_opponent_count` positive test is the pattern; there
  is a `declines_existential_player_life` test to flip to a positive assertion.

## Done
- The `declines_existential_player_life` test becomes a positive parse test; the
  ~10 flagged cards graduate. `idris-check` count unchanged; `cargo test
  --workspace` green.

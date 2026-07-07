---
needs: []
---
Unlock the existential player-life board condition
`~ enters tapped unless a player has N or less life.` (~10 cards, flagged by the
parse-enters-tapped worker).

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

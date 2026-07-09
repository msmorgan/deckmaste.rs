---
needs: []
---
Design and add the grammar needed to support Balance (and its family), which is currently
un-encodable. Two distinct gaps block it.

## Why

Balance ("Each player chooses a number of lands they control equal to the number of lands
controlled by the player who controls the fewest, then sacrifices the rest; then repeats for
creatures, then cards in hand") cannot be authored today. It is a real card, so its support is a
committed obligation, not a maybe. The player-fold `Aggregate` (min/max/sum over players) landed
already; these two residual pieces did not.

## The two gaps

1. **Choose-N / act-on-the-complement ("down-to").** There is no primitive for "keep N of a set,
   then act (sacrifice/discard) on the REST." Only `SeparatePiles` (label-divider piles) exists —
   it does not express "sacrifice down to K". Needed shape: choose a subset of size K from a set,
   apply an effect to the unchosen complement.
2. **Per-player "who controls the fewest" as a SELECTION, not just a number.** `Aggregate MinOf`
   yields the minimum count, but Balance needs each player to sacrifice down to that count — which
   requires selecting the complement per player. `Selection::Pick` is pinned to `AnObject`, so a
   player-indexed selection isn't representable. A player-kinded selection (or a per-player
   "reduce your set to N" effect) is needed.

## Scope

Design both primitives (brainstorm + spec first — this is grammar design, not a mechanical add),
then implement Idris grammar + Rust mirror + emit + engine + a Balance fixture that round-trips
and renders verbatim. Confirm with `cargo xtask idris-check plugins/canon "Balance"` +
`cargo xtask fidelity`.

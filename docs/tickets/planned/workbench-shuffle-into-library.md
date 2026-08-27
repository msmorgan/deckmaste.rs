---
needs: []
---
# The shuffle-into-library move

Routed from `workbench-event-zone-3-targeting-and-disjunction-arms` (close,
2026-08-26). "Shuffle [card] into its owner's library" is a move whose
destination takes a randomizing arrangement [CR#701.24a], and no move verb
spells it — `Move`'s library destinations are positional (`LibraryAt`), and
`Shuffle` is the bare whole-library act. Fblthp, the Lost's second ability
is the found carrier; MEASURE the "shuffle … into … library" surface before
shaping (it is a common printed verb — expect a large family; decide slot vs
row from the measured shapes).

## Consumption boundary

`idris/src/Experimental/Effect.idr` (or `Phrase.idr` if the destination
arrangement is the seat), `Macros.idr`, `Cards.idr` bench, `Proofs*.idr`.
No Rust crate.

## Acceptance

- Fblthp, the Lost benches whole; the verb is closed by design, not
  per-card.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

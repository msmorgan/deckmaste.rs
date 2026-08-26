---
needs: []
---
# The holder-sourced "same number of each kind of counter" read

Routed from `workbench-counter-distributive-residues` (close, 2026-08-26).
The announced-batch half of the "same number and kind" axis landed there
(`PutCountersOfThoseKinds ThatMuch`, Captain Marvel benched). The
holder-sourced half did not: Denry Klin's counters come from a HOLDER whose
mention is unwritten on the surface (named only in the intervening-if), so
the shape choice — a holder slot on the row vs an anaphor over the
condition's mention — is open, not mechanical. Measured size: 1 supported
line (Denry Klin); Dominion Saboteur additionally blocked on the copy-entry
frame (routed to workbench-copy-family-residues). [CR#122.8] performs the
same read for its leaves-the-battlefield case and marks it as not a move
[CR#122.5].

## Consumption boundary

`idris/src/Experimental/Effect.idr` (the row), `Cards.idr` (bench),
`Proofs*.idr` if a pin falls out. No Rust crate.

## Acceptance

- Denry Klin benches whole or the shape question ends in a written ruling.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

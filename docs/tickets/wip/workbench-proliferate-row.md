---
needs: []
---
# Proliferate's counter row — the self-reading kind-blind distributive

Routed from `workbench-verb-label-residues` (close, 2026-08-26). Proliferate's
chooser half spells today (verified there: `CountedGroup anyNumber` over a
joined counters-having description at `Object \/ Player`). The missing half
is the counter row: "one additional counter of each kind [it] already has",
per member — a distributive that reads the HOLDER'S OWN kinds, where
`PutCountersOfThoseKinds`/`GetsCountersOfThoseKinds` read an ANNOUNCED batch
and presuppose one, and `PutSameCounters` reads a different holder. Rule:
[CR#701.34a]. Unblocks Tromell (and the proliferate `verbFacts` row + macro,
one each, per the open-label mechanism).

## Consumption boundary

`idris/src/Experimental/Effect.idr` (the row + its nine tables),
`Macros.idr`, `Words.idr` (`verbFacts` row), `Cards.idr` bench,
`Proofs*.idr`. No Rust crate.

## Acceptance

- Tromell benches whole or is named at a non-counter blocker; proliferate
  expands in full under its label.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

---
needs: []
---
# The self-reading counter row's twins: remove, double, and the negated set

Routed from `workbench-proliferate-row` (close, 2026-08-27), which landed
`GiveCountersOfOwnKinds` (the self-reading kind-blind distributive at
`Object \/ Player`, [CR#701.34a]). Its measured siblings, one family:

1. **The remove twin** — "choose any number of permanents, then remove from
   each a counter of each kind already there" (Unclaimed Bird): same
   self-reading distributive, taking DIRECTION, at `Object` alone.
2. **The multiplicative twin** — "double the number of each kind of counter
   on [n]" (~10 cards: Gilder Bairn, Vorel, Deepglow Skate, Ferrafor,
   Arcade Cabinet, Arna Kennerüd, The First Tyrannic War, The Thing,
   Zimone, Miles Morales) plus the player seat ("… you have", Aetheric
   Amplifier). A multiplication over the holder's own kinds, not a per-kind
   increment — no row.
3. **The kind-blind move under a NEGATED kind set** (Goldberry,
   River-Daughter) — the self-reading move that excludes named kinds.

Weigh one parameterized row (direction/operation slot) against three
siblings before minting; `GiveCountersOfOwnKinds`' nine-table pattern is the
model either way.

## Consumption boundary

`idris/src/Experimental/Effect.idr` (+ its nine tables per row), `Words.idr`
if an operation word is needed, `Macros.idr`, `Cards.idr` bench,
`Proofs*.idr`. No Rust crate.

## Acceptance

- Each numbered item lands or ends in a written rule-backed verdict; at
  least Vorel or Deepglow Skate benches whole for item 2.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

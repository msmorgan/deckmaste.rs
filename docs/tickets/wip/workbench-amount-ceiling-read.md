---
needs: []
---
# "Up to [n]" as an Amount — the ceiling read off a described set

Routed from `workbench-payment-event-residues` (close, 2026-08-26). Shah of
Naar Isle's body "draw up to three cards" does not write: `UpToOf` is a
`Quantity` over a described set's mention, and `Draw`'s count is an
`Amount`, which has no ceiling arm. Decide whether the ceiling becomes an
`Amount` row (the drawer chooses a number up to the bound, [CR#608.2d]) or
the draw takes a `Quantity` — measure the "draw/mill/discard up to N" verb
surface in local card data first and shape for it, not one card.

## Consumption boundary

`idris/src/Experimental/Phrase.idr` (`Amount`/`Quantity`), `Effect.idr`
(the consuming verbs), `Cards.idr` bench, `Proofs*.idr`. No Rust crate.

## Acceptance

- Shah of Naar Isle benches whole; the shape is measured, not per-card.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

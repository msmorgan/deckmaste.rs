---
needs: []
---
# The distinct-kind count — an axis-keyed fold head

Routed from `workbench-counting-nouns-and-this-way` (which closed the other
two counting gaps): eight of the asymmetric definition's twelve lines
(Tarmogoyf among them) count distinct card TYPES among cards — "the number of
card types among cards in all graveyards". That counts distinct VALUES on a
characteristic axis, not members of a described set (`CountOf`) or of a named
mention (`CountOfGroup`), so neither existing head reaches it.

What it needs: an axis-keyed fold head over `Amount` — a domain (description
or mention) plus the axis whose distinct values are counted. Study the
existing `Aggregate`/`AggregateOf` op/axis split (`AggregateOp`, `ProjAxis`)
before minting new machinery: the natural shape may be a distinct-count op on
the existing axis vocabulary rather than a new head.

## Consumption boundary

`idris/src/Experimental/Phrase.idr` (the `Amount` head and its total-table
rows), evidence bench `idris/src/Experimental/Cards.idr`.
No Rust crate.

## Acceptance

- Tarmogoyf's definition line benches whole; the count is closed by design,
  not per-card.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

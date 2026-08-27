---
needs: []
---
# The distributive axis read and the two derived axes

Routed from `workbench-distinct-kind-count` (close, 2026-08-26), which landed
`DistinctCount`/`KindAxis`. Its measured remainders:

1. **The distributive twin** — "for each basic land type among lands you
   control" (33 supported), "for each color among …" (11), ~52 lines total:
   same `KindAxis`, same `Noun` domain, but a `ForEachOf`-side iteration
   head, not an `Amount` row. Study `ForEachOf`'s element binder and the
   landed `DistinctCount` before minting; the axis vocabulary must be
   SHARED, not duplicated.
2. **Two derived axes, measured and not built:** the colour pair with a
   domain restriction (Niv-Mizzet, Guildpact) and the subtype complement
   (Subgoyf; unsupported corpus line). Land each or name it at its measured
   size with the rule that bounds it.

## Consumption boundary

`idris/src/Experimental/Words.idr` (`KindAxis`), `Phrase.idr`, `Effect.idr`
(the iteration head), `Cards.idr` bench, `Proofs*.idr`. No Rust crate.

## Acceptance

- The distributive read lands over the shared axis vocabulary or ends in a
  written rule-backed verdict; witnesses benched per axis arm exercised.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

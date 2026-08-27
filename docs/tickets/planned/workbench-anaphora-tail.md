---
needs: []
---
# Anaphora tail: the plural provenance twin, and the condition's re-mark

The two actionable remainders of the anaphora split (parent umbrella:
workbench-anaphora-mentions-and-creation; sub-rounds A–E closed 2026-08-27).

1. **`ThemVerbed`** — `ItVerbed`'s plural twin (39 occ / 38 cards): a new
   mention constructor, deliberately left out of sub-round C's §5-only
   budget. Cost: the constructor + `countManyVerbedIt`-style counter +
   Words readers + 14 total-table rows + one ProofsAnaphora §2 identity and
   §3 pair (additive), on `ItVerbed`'s model exactly.
2. **The condition-level RE-MARK** — Bioplasm's "If it's a creature card"
   should re-mark the tested binding's type so later reads see it, but a
   `condDelta` row would break `condIntroIsDeltaThenPrefix`'s `Refl`; the
   shape is a `settleTargets`-like in-place re-mark at `condIntro` level
   (clause 2's licensed form). No settled pin covers it — design against
   the binder contract and state the §5 cost honestly. (Bioplasm's bare
   "it" already writes via `ItAt CardSlot`; this item is about the
   knowledge the test leaves behind.)

## Consumption boundary

`idris/src/Experimental/Words.idr`, `Phrase.idr`,
`ProofsAnaphora.idr`; `Cards.idr` bench; `Proofs*.idr`. No Rust crate.

## Acceptance

- Item 1 lands with its proof debt paid; Bioplasm whole is item 2's
  acceptance witness (its last blocker after the exile-stamp read landed).
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

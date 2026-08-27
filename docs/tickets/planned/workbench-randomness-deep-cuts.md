---
needs: []
---
# Randomness deep cuts: die kinds, chosen ignores, the flip instruction event

Routed from `workbench-randomness-tail` (close, 2026-08-26), the third
randomness round. Its supported unowned remainders:

1. **A die-kind slot on `RollsDice`** — 4 lines want the watched roll
   narrowed by die kind (the planar-header narrowing verdict from that
   round's item 2 rides this); completes none alone.
2. **The counted-and-chosen ignore arm** — "ignore any one of those results"
   style (3 lines): `IgnoreRolls` covers extremes (`IgnoreExtreme`/
   `IgnoreAllBut`), not a chooser-selected result.
3. **`RollPlanarDie`'s missing count** — the instruction takes no count;
   printed lines write one.
4. **The flip INSTRUCTION event** — `FlipEvent` is only [CR#705.2]'s called
   flip, so no replacement can reach the flipping act itself; Krark's Thumb
   ("If you would flip a coin, instead flip two coins and ignore one") is
   the carrier — verify its supported status and exact text from local data
   before building.

Named at zero in the parent close, not here: "Whenever chaos ensues" (172
lines, all on Plane cards, none supported — scope), the blank-face read,
exchange-a-result-with-base-P/T (unsupported carrier). Parent fence carries:
watch/read vocabulary only, no randomness engine.

## Consumption boundary

`idris/src/Experimental/Events.idr`, `Triggers.idr`, `Effect.idr`,
`Words.idr`; `Cards.idr` bench; `Proofs*.idr`. No Rust crate.

## Acceptance

- Each item lands or ends in a written rule-backed verdict; no silent drops.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

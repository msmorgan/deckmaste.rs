---
needs: []
---
# Count nouns and look back at "this way"

The letters round measured 67 supported "where X is … this way" lines and
benched one (Soul's Might). The other 66 are blocked on two independent
gaps, neither letter-specific:

1. **`CountOf`/`Aggregate` take a `Predicate`, not a `Noun`** — "the number
   of creatures destroyed this way" counts a *mention* (`ThoseVerbed
   Destroy …`), and no `Amount` head accepts one. Design the noun-counting
   `Amount` head (or widen `CountOf`) so a verbed anaphor can be counted.
2. **`Lookback` has no "this way" value** — `ThisTurn | ThisCombat |
   LastTurn | ThisGame` cannot scope an event to "by this spell/ability's
   own actions" [CR#701.x "this way" is defined per-action; check the rule
   that grounds the deixis]. Design the `ThisWay` lookback (distinct from
   `Effect.ThisWay`, which is the reflexive trigger).

Witness candidates once both land: Phyrexian Rebirth ("where X is the
number of creatures destroyed this way"), Hew the Entwood (also blocked on
`nounDelta (LibrarySlice …)` threading — see the letters ticket's ledger),
Astarion's Thirst (also blocked on `designationScope CommanderD`).

## Consumption boundary

`idris/src/Experimental.idr`, `Words.idr`, `Proofs*.idr`, `Cards.idr`.
No Rust crate.

## Acceptance

- At least Phyrexian Rebirth benches whole; the two gaps are closed by
  design, not per-card special cases; `idris/scripts/build` PASS, no
  witness lost, no pin silently passing.

Standard constraints apply.

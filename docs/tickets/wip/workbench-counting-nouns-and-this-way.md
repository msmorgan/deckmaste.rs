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

## Routed ledger items

Items from closed round tickets that this ticket owns. One line each, citing
the done ticket that recorded them.

- **The bare-participle half of the "this way" family.** Of 40 distinct "dealt
  damage this way" lines, 10 are the finite clause `DealtThisWay` spells and
  **30** are a bare participle modifying a noun ("each player dealt damage this
  way", "the number of opponents dealt damage this way") — a `Noun`-side
  surface, closest to the `TheVerbed`/`ThoseVerbed` marking, and explicitly not
  `DealtThisWay`'s to reach —
  `docs/tickets/done/workbench-joined-kind-binding.md`.
- **An `Amount` reading the SIZE of a NAMED participle group.** `GroupSize`
  reads the unique plural mention, not a named one, which is what blocks Burn at
  the Stake's "three times the number of creatures tapped this way" (its other
  blocker, the additional-cost frame, is routed to the cost ticket) —
  `docs/tickets/done/workbench-verb-labels-open.md`.
- **The distinct-KIND count.** Eight of the asymmetric definition's twelve lines
  (Tarmogoyf among them) count card TYPES among cards, which `CountOf` does not
  spell — `docs/tickets/done/workbench-amount-comparison-and-quantity.md`.

## Consumption boundary

`idris/src/Experimental.idr`, `Words.idr`, `Proofs*.idr`, `Cards.idr`.
No Rust crate.

## Acceptance

- At least Phyrexian Rebirth benches whole; the two gaps are closed by
  design, not per-card special cases; `idris/scripts/build` PASS, no
  witness lost, no pin silently passing.

Standard constraints apply.

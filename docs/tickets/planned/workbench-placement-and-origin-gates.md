---
needs: []
---
# The placement complement's subject row, and the zone gates' held questions

Routed from `workbench-event-zone-4-zone-catalog-and-reader-payload` (close,
2026-08-26), which landed the `Command` row and the `FromZones` origin
payload. Four remainders, one region:

1. **The placement zone complement — 61 measured lines** ("a creature card
   was put into your graveyard from anywhere this turn": ~35 relative-clause,
   ~13 condition, ~5 count). Structurally blocked, not deferred:
   `lookbackSubjectOk Placement` is `False` at both kinds, and the arrival
   gates (`putDestZoneOk`/`putSourceZoneOk`) live in `Triggers.idr`, which
   imports `Phrase.idr` — unreachable from `EventComplement`'s seat.
   `FromZones` is the mechanism it will use; the subject row and an
   event-keyed re-homing of the gates are the work.
2. **The negated origin** — "from anywhere other than your hand" (3 lines):
   a complement-side negation shape `FromZones` does not carry.
3. **`putDestZoneOk` drift (from the module split):** its docstring says the
   battlefield is excluded [CR#603.6a] while the cell reads
   `Battlefield = True`. Decide the direction and fix both to agree.
4. **The held `badCastFromBattlefield` pin:** no rule categorically refuses
   a battlefield cast, so `playableFrom Battlefield = False` may be a
   count-based refusal — a pins-doctrine defect if so. Settle it knowing the
   flip cascades through `complementLocates`' shared cells.

## Consumption boundary

`idris/src/Experimental/Events.idr`, `Triggers.idr`, `Phrase.idr`,
`Words.idr`; `Cards.idr` bench; `Proofs*.idr`. No Rust crate.

## Acceptance

- Each item lands or ends in a written rule-backed verdict; item 3's
  docstring and cell agree; item 4 ends with the pin retired or re-grounded
  on a rule.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

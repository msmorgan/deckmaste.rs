---
needs: []
---
# Per-type ability licensing for command-zone cards

Routed from `workbench-card-class-and-command-zone` (close, 2026-08-26).
`cardAbilityOk CommandZoneCard` licenses by CLASS, so it overgenerates
per-type refusals: an activated ability on a conspiracy card ([CR#315.5]
gives conspiracies statics), or anything beyond [CR#309.4c]'s room triggers
on a dungeon card, is admitted. Refusing those needs `cardAbilityOk` to see
the card TYPE, not the class — deliberately not taken by the parent round.
Weigh a per-type row against the tolerated-overgeneration doctrine before
building: each refusal needs its rule named.

## Consumption boundary

`idris/src/Experimental/Card.idr`, `Words.idr`; `Cards.idr` bench;
`Proofs*.idr`. No Rust crate.

## Acceptance

- Each per-type refusal lands with its rule or is named at its zero.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

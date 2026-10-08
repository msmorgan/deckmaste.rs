---
needs: []
---
**An object binding records no possessor, so the checker cannot enforce a
verb's own restriction on whose hand a card is in or who controls a
permanent.** Found while prototyping the actor handoff
(`semantics-v2-actor-handoff`, 2026-10-05). Standard constraints apply.

## The defect

Two shapes check clean in the prototype although the rules forbid them (seen
in the prototype, not pinned):

- `sacrifice (a creatureYouDontControl)`. [CR#701.21a]: "A player can’t
  sacrifice something that isn’t a permanent, or something that’s a permanent
  they don’t control."
- `[choose (a (.inZone yourHand)), act (target .opponent) (discard (that
  .card))]`: the discarded card is in the controller's hand, not the
  discarder's. [CR#701.9a]: "To discard a card, move it from its owner’s hand
  to that player’s graveyard."

## Cause

An object binding, `Payload.object ty zone prov orig size`
(`lean/Semantics/Check/Words.lean`), records the card types, the zone, the
provenance stamp, the origin and the size, but no possessor. The possessor
written in `.inZone (handOf p)` is dropped when the binding is made, and so
is a controller predicate such as `.hasPossessor .controller .you`. The
zone half of a verb's restriction is checked (an indefinite patient that
names no zone is selected where the deed's patient lives, 2026-10-05); the
possessor half has nothing to read.

## What a fix needs

1. A possessor or identity field on object bindings, seeded from
   zone-possessor predicates (`.inZone (handOf p)`) and controller predicates
   (`.hasPossessor .controller p`), and kept through moves.
2. Two rules, one per deed role: a sacrifice's patient is controlled by the
   performer [CR#701.21a], and a discard's patient is in the performer's own
   hand [CR#701.9a]. Each takes both forms §7 of
   `docs/decisions/semantics-v2.md` requires: a predicate conjunct on an
   indefinite patient and a demand on a definite referent.
3. Refusal pins for both shapes above, and the existing pins at their old
   outcomes.

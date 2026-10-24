---
needs: [lean-v3-choose-over-coordination]
---
# Reciprocal "each other" and "the other"

Standard constraints apply.

## Problem (probed 2026-10-23)

[CR#701.14a]: "A spell or ability may instruct a creature to fight another creature or it may
instruct two creatures to fight each other. Each of those creatures deals damage equal to its
power to the other creature." The `fight` Definition (`Definitions/Actions.lean`) has only the
first form: its actor fights `other`. The model has no reciprocal: the only reflexive noun
phrase is `themselves` (`.refl`). The done `workbench-coordination-family` ticket already lists
"the reciprocal fight node" as an open gap ("eleven printed lines"), and no open ticket owns it.

The distributive form does not check either. After choosing two creatures,
`instruct (eachOf (those creature)) (dealDamage (toughnessOf it) (theOther .object))` is refused
with `theRestFits .object`.

## Witnesses

- "Then those creatures fight each other": *Blizzard Brawl*, *Joust*, *Tail Swipe*, *Malamet
  Battle Glyph*. *Duel for Dominance* writes "the chosen creatures fight each other" (also
  `lean-v3-chosen-participial-anchor`).
- "Each of those creatures deals damage equal to its toughness to the other": *Grim Contest*.

## Design (proposed)

- A reciprocal argument read against a plural subject of exactly two members: "those creatures
  fight each other" means the two-creature form of [CR#701.14a]. It is not each creature
  fighting the other in turn, which would deal the damage twice.
- "the other", inside the scope of "each of those …", reads the group member that is not the
  current one.

Keep the self-fight rule [CR#701.14c] in view: two targets can be one object only when they come
from different instances of "target" [CR#601.2c]. "Target creature you control and target
creature you don't control" can never pick one creature twice.

## Tests

- *Joust* and *Blizzard Brawl* bench items through "those creatures fight each other", and
  *Grim Contest* whole.
- A refusal pin: "each other" with a singular subject.

---
needs: []
---
# A later spell ability that acts "before" an earlier one's action

Standard constraints apply. Found landing `lean-v3-postposed-doif-condition`, which left *Savage
Punch* unspelled.

## Problem

*Savage Punch*: "Target creature you control fights target creature you don't control. /
Ferocious — The creature you control gets +2/+2 until end of turn before it fights if you control
a creature with power 4 or greater." The second paragraph is its own spell ability [CR#113.2c],
yet its effect happens before the first paragraph's fight. Instructions are followed in the order
written, but later text may modify the meaning of earlier text [CR#608.2c]; nothing in Core lets
one spell ability place its action before an action of an earlier one. Spelling it as a single
sequence with the pump first erases the ability boundary; spelling it in written order makes the
pump come after the fight, which changes the outcome.

## Change

Decide the Core form for "<action> before <earlier action of this spell>": a temporal qualifier on
the later ability's action naming the earlier action, or the spell-level ordering the engine
applies when one spell ability says "before". Related: `lean-v3-cross-ability-self-replacement`
(a later ability's "instead" replacing an earlier one's effect [CR#614.15]); one design may serve
both. The design is proposed, not ruled.

## Tests

- *Savage Punch* spelled in full checks clean, with the pump ordered before the fight.
- Its "the creature you control" and "it" read the first paragraph's target (spell abilities share
  targets, landed).

---
needs: [plugins-v3-builtin-primitives-and-templates]
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

## Ruling (owner, 2026-10-24)

Kept separate from `lean-v3-cross-ability-self-replacement`; one design does not serve both.

## Change

- Add `Action.beforeEvent (action : Action) (event : GameEvent)` with template `beforeEvent`.
- Step it like `holdUntil` (`Action.holdUntil`, `Abilities.lean:1370`; checker arm
  `Check/Phrase.lean:4307`, which steps `inst` then `ev` in the output DRS): `action` first, then
  `event` in `action`'s output DRS [CR#608.2c]. The event is the earlier action the pump precedes.
- *Savage Punch*'s second paragraph is
  `italicHead ferocious (spell (doOnlyIf (beforeEvent (get …) (does it (fight any))) (exists_ …)))`
  (`doOnlyIf`, `Templates.lean:1520`; `italicHead`, `:2018`; `spell`, `:2101`).
- "It" reads back "the creature you control" through the landed
  [lean-v3-spell-abilities-share-targets](../done/lean-v3-spell-abilities-share-targets.md) and
  [lean-v3-anaphoric-definite-description](../done/lean-v3-anaphoric-definite-description.md)
  work.

## Tests

- *Savage Punch* spelled in full checks clean, with the pump ordered before the fight.
- Its "the creature you control" and "it" read the first paragraph's target (spell abilities share
  targets, landed).

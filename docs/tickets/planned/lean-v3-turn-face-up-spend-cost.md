---
needs: [plugins-v3-builtin-primitives-and-templates]
---
# Which cost a turn-face-up spend purpose pays

Standard constraints apply.

`SpendPurpose.toTurnFaceUp (subject : Predicate)` (`lean/Semantics/Abilities.lean`, added by
`lean-v3-rules-and-card-fidelity`, owner ruling 2026-10-25) says only that the mana turns a
permanent `subject` describes face up [CR#116.2b]. It does not say which cost it pays. A manifested
card is turned face up by paying its mana cost [CR#701.40b], or, if it has morph or disguise, by
paying that cost instead [CR#701.40c,701.40d].

*Qarsi Deceiver* prints "pay a mana cost to turn a manifested creature face up", so its mana cannot
pay a manifested card's disguise cost. Its fixture, `qarsiDeceiverMorphSpend`
(`lean/Semantics/Cards/Mana.lean`), writes `toTurnFaceUp (and [isCreature, was (manifest which)
thisObject])`, which admits that disguise cost. *Overgrown Zealot*'s "to turn permanents face up"
needs no qualifier.

Change: give the purpose an optional cost qualifier (the permanent's mana cost, as opposed to a
morph or disguise cost), spell Qarsi with it, and pin that the Qarsi purpose names the mana cost.
No existing `CostNamed` or `SpendPurpose` arm says "its mana cost".

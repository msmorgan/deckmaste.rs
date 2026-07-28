---
needs: []
---
**Mixed-introducer trigger conditions (`At the beginning of your upkeep and
whenever you cast a black spell, …`).** 39 groups / 39 occurrences / 829
words (Shrine cycle, Tombstone Stairwell; enumerated at
`recovery-harness/out/coordevent-A-mixed-introducer.txt`). The design
question is SETTLED (round coordevent, 2026-07-25): a triggered ability may
have more than one trigger condition [CR#603.1b], and the canonical shape
is one ability [CR#603.1] — so `TriggeredAbility` should carry a
coordinated LIST of `(introducer, event)` pairs, NOT be split into two
abilities, and NOT fold the second condition into the first event. Deferred
because `TriggeredAbility` is shared by thousands of ability-initial faces
(large blast radius for 39 groups) — the change needs a renderer inverse
for the coordination and selection-stat controls over ability-initial
triggers broadly. A named guard test
(`mixed_introducer_trigger_still_recovers`, grammar/ability.rs) pins the
shape as must-still-fail after coordevent's event widening; the round that
builds this retires that test deliberately.

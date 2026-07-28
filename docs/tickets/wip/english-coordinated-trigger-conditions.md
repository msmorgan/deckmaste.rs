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

## Completion

`TriggeredAbility` now carries a `TriggerConditionList`: one
`TriggerCondition { introducer, event }` plus coordinated remainder members.
The ability-frame parser recognizes top-level `and`/`or` followed by a fresh
trigger introducer, parses every condition independently, and rolls back any
declined coordinated probe before using the established single-condition
path. Modal headers and non-initial trigger sentences keep their existing
single-condition representation. The renderer and recovery walker traverse
the new list directly, so a mixed trigger remains one ability and reproduces
its conjunction byte exactly. A scalar `and at least`/`and at most` guard
prevents coordinated subjects from being mistaken for new `At` conditions.

The original must-recover guard is retired in favor of positive structural
tests for `At … and Whenever …` and `When … or When …`. A selection control
pins the exact event/effect selection spans of an ordinary ability-initial
trigger, and the full pre/post provenance for Keeper of Keys (initial `When`
and `At` triggers plus an intervening `if`) is identical down through selected
rules, costs, chart statistics, and forest statistics.

The live normalized unresolved-dump comparison contains only mixed-introducer
trigger movers. It removes 41 whole clause recoveries and two embedded-rules
recoveries; Ominous Roost and Minsc & Boo retain narrower, pre-existing effect
recoveries after their trigger frames become structural. Fifteen additional
faces that previously produced a false clean tree lose an opaque `when` or
`whenever`: the second introducer had been swallowed into the first event.
Cryptid Inspector newly exposes one existing opaque `face-down` noun inside
its now-structured event. Thus structural recovery moves from 3,521 spans /
65,182 source tokens to 3,480 / 64,217: clause recovery moves from 3,384 /
64,045 to 3,345 / 63,129, and embedded-rules recovery from 48 / 631 to 46 /
582; every other structural role is unchanged. Noun opacity moves from 806 to
792, while flavor-header opacity remains 622.

All 570 library tests and 111 public-API tests pass. The supported corpus
round-trips 31,685/31,685 clean with zero mismatches or render errors.

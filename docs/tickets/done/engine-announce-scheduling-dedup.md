---
needs: []
---
Cast and activate already share their announce LOGIC (`announce_targets`, `pay_cost`,
`targets_still_legal`) but copy the SCHEDULING shells: the
`[Begin*, AnnounceTargets, PayCost, Emit, CheckSbas, PlaceTriggers, OpenPriority]`
WorkItem list is written twice (`decide.rs:1138` vs `1162`, 5 of 7 items identical),
and the `StackEntry` announce-promotion push is near-identical between SpellCast
(`step.rs:291`) and AbilityActivated (`step.rs:317`). Add `announce_schedule(begin,
cast_event)`, a `priority_tail()` trailer (also reused by the PlayLand / mana-ability
arms), and a `promote_announce()` helper. Pure refactor.

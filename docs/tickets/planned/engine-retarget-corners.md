---
needs: [engine-copy-spells]
---
Two corners deferred from the `ChooseNewTargets` submit arm (`decide.rs`,
retargeting a COMMITTED stack entry, [CR#707.10c,115.7d]).

**Must(Target)/Flagbearer enforcement.** The `ChooseTargets` submit arm
enforces Must(Target) rows (the Flagbearer class — e.g. Coalition Honor
Guard: "that player must choose at least one Flagbearer on the battlefield
if able", [CR#601.2c]) by reading the in-flight announce/placing-trigger
state. A committed entry being retargeted has no such state, so that check
couldn't be ported verbatim — retargeting while a Flagbearer is in play
currently skips the must-target rule. Needs a must-row check keyed off the
entry being retargeted instead of the announce.

**`BecameTarget` re-fire on retarget.** The submit arm writes
`entry.targets = chosen` silently, emitting nothing. Whether choosing new
targets ([CR#115.7d]) should re-fire "becomes the target" triggers for a
slot whose target actually changed was open here. [CR#601.2c]: "Any
abilities that trigger when those objects and/or players become the target
of a spell trigger at this point" — and the official rulings on the sibling
"change the target" family settle the general principle for retargeting:
Deflection/Imp's Mischief/Misdirection/Shunt (2004-10-04) each read "Once
the spell resolves, the new target is considered to be targeted by the
[redirected] spell. This will trigger any effects which trigger on being
targeted." So a slot's NEW target must trigger becomes-the-target abilities
(Ward, etc.) same as at initial declare; an unchanged slot must not
re-trigger. Needs the submit arm to diff old vs. new target per slot and
emit `BecameTarget` only for slots that actually changed.

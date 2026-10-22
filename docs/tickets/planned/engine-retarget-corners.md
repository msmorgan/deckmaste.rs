---
needs: [engine-copy-spells]
---
Corners deferred from the `ChooseNewTargets` submit arm (`decide.rs`,
retargeting a COMMITTED stack entry, [CR#707.10c,115.7d]).

**No Flagbearer enforcement on retarget or copy (regression test).** Removed
2026-10-21: the earlier item proposed enforcing Must(Target)/Flagbearer rows when a
committed entry is retargeted. Coalition Honor Guard limits the requirement to
"While an opponent is choosing targets as part of casting a spell they control or
activating an ability they control"; changing or choosing new targets
([CR#115.7a,115.7d]) and a copy put onto the stack "with those targets"
([CR#707.10,707.10c]) are neither (ruling 2016-06-08: the changed target "doesn't
have to target a Flagbearer"). The engine is right today (must rows only on the
announce path, `cast.rs:71-100`). Add a regression test that retargeting and
copy-with-new-targets ignore a Flagbearer.
(`docs/evidence/mtg-rules-review-2026-10-21/departures-docs.md` #40.)

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

**Scope of "unchanged slot emits nothing" (added 2026-10-21).** That rule is for
redirecting an EXISTING stack entry ([CR#115.7d]) only. A copy is a new spell
([CR#707.10]) put on the stack with its targets ([CR#707.10c]), so every target of
a copy becomes a target, changed or not (Dormant Gomazoa ruling 2010-06-15: "You
become the target of a spell … when a spell that's targeting you is copied and
that target isn't changed"; Angelic Cub ruling 2024-11-08). That half is
`engine-spell-copy-stack-lifecycle`; do not route the copy path through this
diff.

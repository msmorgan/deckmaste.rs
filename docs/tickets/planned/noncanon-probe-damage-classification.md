---
needs: []
---
Fix the WC99 probe's damage classification for the blocking era. The
heuristic (crates/deckmaste_noncanon/src/probe.rs) classifies by
source-on-battlefield at post-batch observe time, guarded by a comment
("fine while the fixture has no combat trades; revisit with blockers") that
is stale: both pilots now declare `Block(BlockAll)`, so trades happen every
batch. Consequences: a dead-in-batch attacker's combat damage counts as
`spell_damage_to_creatures`; Mogg Fanatic's sacrifice-ability damage counts
toward `spell_damage_to_players`, so the "burn never went to the face" gate
(tests/wc99.rs) can be satisfied by ability damage; an on-battlefield Cursed
Scroll's ability damage counts as `creature_damage_to_players`. Classify off
the damage event's cause/source kind (spell vs ability vs combat) instead of
the source's zone at observe time, and re-derive the gate's expected
buckets.

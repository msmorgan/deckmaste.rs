---
needs: []
---
`Filter::Type`/`has_type` (`target.rs:99`) tests card types by reading the PRINTED
face via `derive::face(state.def(id)).types`, bypassing the layered view — so an
animated land or crewed Vehicle on the battlefield is mis-typed (the view says
Creature, the matcher says it isn't), and every external `candidates` /
`filter_matches_live` caller inherits the wrong answer. `layer.rs:374` already
documents the gap and patches around it inside `matches_derived`. Route the `Type`
arm (and the printed-type read at `legal.rs:215` play-land) through the derived view
for battlefield objects, exactly as the `Has`/`Subtype` arms already do. Spell/stack
reads (`cast.rs:154`, `resolve.rs:920` `is_permanent_spell`) may stay printed.
Correctness bug, release-blocker (publish-prep). Distinct from engine-filter-breadth
(which adds new filter KINDS); see engine-filter-walker for the follow-on cleanup.

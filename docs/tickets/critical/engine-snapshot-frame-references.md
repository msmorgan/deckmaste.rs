---
needs: [engine-reference-resolution-snapshot-channel, engine-snapshot-predicate-breadth]
---
**Engine: let snapshot predicate evaluation resolve frame-bound references.**
The snapshot matcher carries only a watcher plus the candidate snapshot. A
candidate-relative `Where(Happened(... Ref(It) ...))` can construct a real
`Frame`, but the nested event/snapshot comparison loses it and the non-`This`
`Predicate::Ref` reaches the catch-all.

Thread the resolving frame (or the unified live/LKI reference product from
`engine-reference-resolution-snapshot-channel`) through snapshot matching.
Compare identity using the bound snapshot's stale identity token without
looking it up as a live object. `This`/`You` must retain their watcher meaning;
`It` and event roles must retain the candidate/event bindings that were present
at the call site.

Foundations witness: Predator Ooze's “a creature dealt damage by this creature
this turn dies” history predicate. Pin a matching damaged creature, an
undamaged creature, and damage from another source; all three paths must be
ordinary booleans rather than a panic or release-only false fallback.

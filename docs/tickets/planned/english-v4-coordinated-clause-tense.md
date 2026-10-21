---
needs: []
---
# A coordination in a present-only Complement keeps preterite conjuncts

`english-v4-clause-tense-selection` requires `Inflection` `Present` on the
Clause Complement of *whenever*, *while*, *until* and supplementary *where*.
A Clause or Verb Phrase Coordination leaves `Inflection` unvalued
(`unshared`), so the requirement values the Coordination and leaves its
conjuncts free: *Whenever you cast a spell or put a counter on a creature,
draw a card* still has four Readings (present and preterite *cast* times
present and preterite *put*).

Sharing `Inflection` across conjuncts is wrong: it loses genuine mixed-tense
coordinations (*If you control a Squirrel or returned a Squirrel card to your
hand this way*, Cache Grab; *if this spell was cast from your hand and you've
cast another spell named …*, Approach of the Second Sun), seven faces in all.

## Change

- Let a Coordination resolve `Inflection` (move `"Inflection"` from `unshared`
  to `resolve: [("Inflection", "Preterite")]` on the `VerbPhrase` and `Clause`
  `Coordination` schemas) so any preterite conjunct makes the Coordination
  `Preterite` and a present-only Complement constrains every conjunct, without
  constraining mixed-tense coordinations elsewhere.
- In `deckmaste_syntax`, fold `nonlast` across serial tails rather than
  requiring pairwise equality between `nonlast` and `last` on non-ending serial
  tails, so 3+ conjunct coordinations resolve `Inflection` across all conjuncts
  as well.

## Done when

- A coordination in those Complements has present conjuncts only.
- No face is lost.

Standard constraints apply.

---
needs: [english-v2-scope-device-collapse]
---
**A frame-declared preposition has no Predicate-Adjunct derivation in that
frame's clause.** `english-v2-role-preemption-depth` (B6) eliminated only the
noun-Postmodifier derivation of a preposition the head verb's frame declares;
the Predicate-Adjunct derivation survives, so `Search your library for a card.`
still carries two candidates — the declared object/`for`/object frame and a
transitive Predicate plus a generic Prepositional Predicate Adjunct — and the
frame wins only by specificity (decisive position 2). Principle (ii) of the
scope-device design (right-periphery frame-role preemption) is meant to reach
this competitor too.

Pinned scope: widen B6's preemption so that, in a clause built by a Verb
Frame declaring role P with marker preposition p, a right-peripheral PP headed
by p has neither a Postmodifier nor a Predicate-Adjunct derivation. The
mechanism is B6's existing frame-role accessor and preemption site, read from
declared frame data — never a guard naming a preposition, verb, construction,
or card. A verb without a `for` role keeps its adjunct `for` (witness:
`Draw a card for each Island you control.`). Expect specificity-resolved →
unique movement on the search/for family; DISCLOSE every changed selection.

Authority: design doc `b7-scope-device-design.md` §C.4a (2026-09-05), which
routes this widening here rather than into phase 3 of the collapse. Runs after
`english-v2-scope-device-collapse` lands so the census diff is attributable.

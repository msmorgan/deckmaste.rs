---
needs: []
---
# Design relation queries with declared endpoints

Parked by the user on 2026-09-06. Assess sharing `NounPhrase.possessorOf`,
`attachHost`, and `designated` through typed relation queries. Their result
kinds and reference behavior differ; introducing general query machinery is
not justified by similar phrase shapes alone.

The candidate design declares endpoint kinds, directions, and cardinality.
Ownership/control yield players; attachment hosts can be objects or players;
designation reads follow the declared designation relation. Macros supply
currently implicit source attachments. Selection and singular-result
requirements remain distinct; do not impose one plurality rule on every read.

Preserve source and result mention ownership in the surrounding construction.
The controller-sacrifice expansion needs both the controller and the object;
a projection must not introduce an extra ambiguous "it" everywhere. Preserve
attachment-host type evidence through later movement, including multiple
known card types. Distinguish ordinary domain checking from proof of an extant
unique result. Generalizing designation reads beyond their present domain is
an explicit design change, not an already established equivalence.

The planned `lean-core-attachment-and-face-predicates` ticket can fold the
three attachment predicates locally without this mechanism. This design may
later reuse that family's direction and counterpart data; it does not block
its implementation.

Show representative relation reads, scope traces, retained evidence, and the
actual laws consolidated before promoting this ticket. Standard constraints apply.

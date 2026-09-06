---
needs: []
---
# Design shared action provenance descriptions

Parked by the user on 2026-09-06. Investigate sharing descriptors for `castBy`,
`castFrom`, `wasCast`, and corresponding activation reads without reducing
all of them to `happenedTo`.

The required distinctions are a currently proposed action, the action
associated with the current object incarnation, and a historical occurrence.
Cost modifiers can inspect a proposed cast; permitted cast provenance can
remain available after a permanent arrives; copied decisions are not new
actions. Preserve actor, origin, rank, reference scope, and uniqueness from
ranked reads. Rank the intended actor's action stream before restricting it
to the described subject; omitted actor constraints must not introduce
spurious references.

Before implementation, show concrete before/after terms and checker scope
traces for all three situations. Record an amendment to the
[binding and context-data contract](../../decisions/semantics-v2.md#2-binding-and-scope)
that explains what information is represented, which parts are projections,
and how the reads are licensed. The amendment must settle the necessary
distinctions, not assume a new fold-state class or state machine in advance.

Existing provenance constructors remain until the representation and its
benefit are agreed. The planned event-pattern folds do not depend on this
work. Standard constraints apply.

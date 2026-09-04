---
needs: [english-v2-require-through-optional-role]
---
Frame-role preemption must be structural at every depth (require-through
landing review HIGH-2). `right_edge_nominal_postmodifier_kind` in
`constructions.rs` bails on any non-`ObjectNominal` and inspects only the
outermost postmodifier, so a `from` PP nested under an outer `of` (or other
PP) survives beside the frame-role reading and the pair is settled by a
specificity weight. The amendment "selected roles preempt postmodifiers" says
this is elimination derived from the frame's declared Verb Frame, never a
preference weight.
Affected: ≤52 units (unstamped baseline; re-measure at claim and stamp the
tree) (All Suns' Dawn, Aphetto Dredging, Belbe's Portal,
Bloodline Bidding, Bone Harvest, Druidic Ritual, …); the frame-role reading
wins in every one today, so no misselection, but the invariant does not hold.

2026-09-04: superseded by ADR "Amendment: Verb Frame and execution-context
vocabulary (2026-09-04)" — was: "derived from declared valence"; and the
affected-unit count was an unstamped baseline.

Pin: within a frame's object position, a right-peripheral PP whose preposition
the frame declares as a role has no noun-postmodifier derivation at ANY depth
of the object's postmodifier chain — right-peripheral is the condition, because
only a right-peripheral PP can be the frame role. A PP that is not right-
peripheral to the object is untouched. Implement the walk over the general
`PostmodifiedReference` spine and any nominal wrapper, not `ObjectNominal`
only. Verify by census: the ≤52 units become unique (specificity-resolved
count falls by exactly that number), zero winner changes, coverage unchanged.
If any of those units has a reading where the nested PP is genuinely NOT the
frame role and the frame role is absent, that is a STOP with the unit named.
Standard constraints apply.

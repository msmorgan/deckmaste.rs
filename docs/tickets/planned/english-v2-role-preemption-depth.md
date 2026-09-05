---
needs: [english-v2-require-through-optional-role, english-v2-underspecified-adjunct-attachment]
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

2026-09-04 (second): extended from depth to **breadth** by coordinator ruling
after the fallout audit (F7) — was: depth only, "`right_edge_nominal_
postmodifier_kind` … inspects only the outermost postmodifier". The pin above
already reads "a right-peripheral PP **whose preposition the frame declares as a
role**"; the implementation does not. `object_has_no_selected_source_
postmodifier` (`crates/deckmaste_english_v2/src/constructions.rs:5075`) tests
`!= Some(PrepositionComplementKind::SourceComplement)` and nothing else, and is
attached at four sites (`:1885, 4425, 4452, 4472`), each paired with
`require source.preposition_complement_kind is SourceComplement`. So the general
amendment is implemented for exactly `from` — the one preposition where a tie was
observed. Frames declaring `to`, `into`, `onto`, `on` or `under` roles get no
preemption at all.

Breadth pin: the preemption reads the frame's declared role preposition through
the generated accessor and eliminates the noun-postmodifier derivation of **that**
preposition, whatever it is — one derivation-level rule over the declared Verb
Frame, not one checker per preposition and not a list of complement kinds. The
four call sites collapse to that rule. Report the census for every affected
preposition separately, and STOP on any unit whose nested PP is genuinely not the
frame role while the frame role is absent (unchanged from the depth pin).

2026-09-04 (third), by coordinator ruling Q6: this ticket is principle 2 of the
scope device's ordered principle set, so it lands as part of
`english-v2-underspecified-adjunct-attachment` (B7) or immediately after it —
was: "`english-v2-attachment-class-declared` lands first". R1 is parked behind
B7 for the same reason; widening attachment classes changes which prepositional
phrases are candidates at these sites, and the preemption is what keeps the
widened set from buying incoherent bracketings. The preemption reaches the whole
complement the frame governs, not the object position alone: the frame-complement
pair coordination the review found missing is eliminated at its incoherent
bracketing here, and minted by `english-v2-frame-complement-coordination`.

Standard constraints apply.

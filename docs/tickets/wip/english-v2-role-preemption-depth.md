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

## Landing record (2026-09-04 — STOP)

Measured on feature change `mzputsyo` after `kata refresh`, atop claim change
`slqupuzy`. The refresh completed without a conflict. Implementation and the
refreshed-tree gate run were performed from 20:12:59 to 21:27:12 PDT.

The implementation is the requested structural elimination. A checked field
argument can project the selected declaration verb's generated role-preposition
accessor; generation derives that accessor exhaustively from the frame's typed
and optional typed role atoms. One `GovernedMaterial` traversal applies the
same checker to a frame's object and selected Complements. The checker follows
the `PostmodifiedReference` structure and rejects a right-peripheral
Postmodifier when its preposition occurs in the frame accessor. No
per-preposition table, per-Complement-kind table, dominance edge, selection
exception, narrowed form, or word-naming guard was added.

### Refreshed-tree numbers

- Coverage: 17,601 -> 17,600 selected and covered identities; ordinary parse
  failures 15,040 -> 15,041; selected-uncovered identities remain 0. Report
  mode printed `no longer covered 1 corpus identity` and named
  `84d4266094836935360ad9e25a5c2acdeddaec5e84c7c2efbf77143096b85602`,
  Thelon of Havenwood. There are no newly covered identities. The lock delta
  is therefore +0/-1. Because the loss is a STOP below, `--bless` was not run
  and the checked-in lock remains at 17,601 identities.
- Selection census: 32,641 total; selected 17,601 -> 17,600; unique 13,759 ->
  13,776; specificity-resolved 3,842 -> 3,824; parse failures 15,040 ->
  15,041. Exception-resolved selections, exception uses, unresolved ties and
  internal failures remain 0. A reflink copy of the exact parent tree under
  `~/Dump/english-v2-role-preemption-depth-parent` was verified against
  `jj file show -r @-`; its complete 32,641-row ambiguity JSON was compared
  with the refreshed tree by status, selected construction path, resolution
  and candidate count.
- Construction declarations remain 387. Licensed vocabulary/lexicon
  homographs remain 2; form-literal/vocabulary overlaps remain 9. Licensing
  checkers are 20 permitted and 0 forbidden. Round trip is 17,600 clean of
  17,600 accepted, with 0 mismatches.
- Coverage provenance remains internally clean: 0 unresolved ties, internal
  failures, exception resolutions, round-trip mismatches, ownership failures,
  gap spans, overlap spans, traversal failures, synthetic claims, or
  provenance-plan mismatches.

### Per-preposition selection delta

`from`: 13 identities affected and 17 candidates eliminated.

- Specificity -> unique with the selected frame-source analysis unchanged:
  Druidic Ritual, Edgewall Inn, Eivor, Wolf-Kissed, Michelangelo, Improviser,
  Pull from the Deep, Reconstruct History, Restoration Specialist, Retrieve,
  Shreds of Sanity, and Sudden Reclamation.
- Selected analysis and resolution unchanged while losing only competing
  Postmodifier derivations: Rise from the Wreck and Sandman, Shifting
  Scoundrel.
- Mantle of the Ancients changes selected construction path. The parent
  selected a full-Noun-Phrase coordination in which `from your graveyard`
  occurred inside one coordinated arm; the measured tree selects the
  shared-head `Aura and/or Equipment cards` Nominal and realizes `from your
  graveyard` as the Return frame's source. This is the correct analysis and is
  decided by principle (ii).

`on`: 18 identities affected and 73 candidates eliminated.

- Specificity -> unique with the selected frame-destination analysis
  unchanged: Dueling Coach, Lifecrafter's Gift, Lockjaw Snapper, Oran-Rief
  Ooze, Patron of the Valiant, Tempered Veteran, and Twisted Spider-Clone.
- Selected analysis and resolution unchanged while losing only competing
  Postmodifier derivations: Ajani, the Greathearted; Brokers Ascendancy;
  Evolutionary Escalation; Filigree Vector; Juniper Order Ranger; River
  Heralds' Boon; Serrated Biskelion; Stand Together; and X-23, Deadly Weapon.
- Slurrk, All-Ingesting changes selected construction path to a wrong
  bracketing: the parent has `put [a +1/+1 counter] on [each creature you
  control that has [a +1/+1 counter on it]]`; the measured tree instead has
  `put [a +1/+1 counter on each creature you control that has a +1/+1
  counter] on [it]`. The first `on` is no longer right-peripheral to the new
  object, so the literal rule preserves this derivation and makes the final
  `on it` the frame destination. This is an `other` selection change and a
  STOP.
- Thelon of Havenwood loses its only correct selected analysis. The parent has
  `put [a spore counter] on [each Fungus on the battlefield]`: the first `on`
  realizes the frame destination and the second genuinely postmodifies
  `each Fungus`. Because both have the frame-declared preposition, the
  governed-Complement widening eliminates the second and leaves no complete
  derivation. This is not the absent-role case from the original depth pin and
  not argument-cluster Coordination. It has no authorized live-ticket route,
  so it is an `other` loss and a STOP.

The explicit witnesses behave as pinned. `Return target creature card from
your graveyard to your hand.` and the nested wrapper witness select the Return
frame's source uniquely; `Destroy target creature card from your graveyard.`
retains its noun Postmodifier uniquely. Forge Devil's `It deals 1 damage to
target creature and 1 damage to you.` has no derivation: the incoherent
bracketing is eliminated, and re-coverage is explicitly owed to
`english-v2-frame-complement-coordination`.

### Positive gate artifacts

- `cargo fmt --all` exited 0. Strict all-target Clippy for both touched crates
  finished successfully with `-D warnings`.
- `cargo test --workspace` exited 0. Representative suite lines were
  `test result: ok. 406 passed; 0 failed; 0 ignored` for construction core,
  `test result: ok. 108 passed; 0 failed; 0 ignored` for the predicate grammar,
  `test result: ok. 753 passed; 0 failed; 1 ignored` for the engine suite, and
  `test result: ok. 454 passed; 0 failed; 1 ignored` for xtask; all remaining
  unit and doc-test suites likewise reported `ok` with 0 failures.
- `DECKMASTE_COVERAGE_LOCK=report cargo xtask english_v2 coverage --check
  --workers 8` exited 0 and printed 17,600 selected/covered, 15,041 parse
  failures, and 0 unresolved ties, internal failures, round-trip mismatches,
  ownership failures, gaps, overlaps, or forbidden checkers, followed by the
  +0/-1 delta above.
- `cargo xtask english_v2 ambiguity --require-resolved --workers 8 --json`
  exited 0 with the selection census above. The corresponding parent command
  also exited 0 with 17,601 selected and 0 ties.
- `cargo xtask english_v2 roundtrip --require-clean --workers 8` exited 0 and
  printed `parse accepted 17600`, `clean 17600`, `mismatched 0`, and
  `not parse accepted 15041`.
- No citation-bearing source changed, so the cite gates were not in scope.

Performance advisory, all with 8 workers and the 16.26 s quiet-host ceiling:
coverage took 201.009839162 s at 229,734 integer ns/B under host load
25.56/32.04/31.58; ambiguity took 127.690250344 s at 156,202 ns/B under load
19.00/21.63/26.73; round trip took 119.560442710 s at 158,661 ns/B under load
21.49/18.92/23.21. All exceeded the ceiling under reported host contention.
The sandbox cannot observe sibling processes, so the concurrent-process count
is unavailable and remains for the reviewer to stamp.

### Deviations, additions and assurance

- No grammar Construction was added or deleted. The generated-accessor seam
  required minimal construction-core changes in `model.rs`, `parse.rs`,
  `semantic.rs`, `validate.rs`, `emit/build.rs`, `emit/runtime.rs`, and
  `emit/terminal.rs`. In particular, a checked argument can name the generated
  frame-role projection, and a checker on an optional category runs only when
  the field is present. These overlap the concurrently running phase-1 field
  work in `parse.rs`, `semantic.rs`, and `validate.rs`; the refresh found no
  conflict.
- Assurance counts: restored 0; re-spelled 1 existing witness test
  (`declared_optional_source_preempts_the_same_noun_postmodifier_derivation`)
  to cover the nested wrapper and selected construction path; ignored with
  blockers 0; added 5 tests
  (`parses_checked_verb_frame_role_preposition_projection`,
  `checked_field_reads_declared_verb_frame_role_prepositions`,
  `checked_optional_category_calls_the_guard_only_when_present`,
  `verb_frame_role_preposition_projection_requires_a_declared_verb_frame`,
  `frame_complement_preemption_removes_forge_devils_incoherent_bracketing`);
  removed 0.
- Glossary gap: right-peripheral.
- Glossary gap: bracketing.

### STOP and decision wanted

Thelon of Havenwood's correct loss and Slurrk, All-Ingesting's wrong selected
analysis demonstrate that principle (ii), as widened to every selected
Complement's interior without regard to an already-realized role, contradicts
correct English. Both are direct consequences of the required general rule;
an exception, named guard, per-preposition special case, or narrowed form would
violate this ticket's fences. The implementation, tests and record are
committed for review, but the coverage lock is deliberately not blessed and
this change must not be integrated without a coordinator ruling that revises
the principle or explicitly supplies a structural distinction for repeated
same-preposition material.

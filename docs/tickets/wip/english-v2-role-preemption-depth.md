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

2026-09-04 (fourth), by coordinator ruling on the landing review: right-
peripherality is the condition as the design writes it. Preemption removes the
Postmodifier derivation only of a role-preposition Prepositional Phrase on the
right periphery of the governed material — the object's or a selected
Complement's rightmost descent, in form order. A Prepositional Phrase inside a
non-final Conjunct is not right-peripheral and keeps its Postmodifier
derivation, so Rise // Fall keeps `[target creature card from a graveyard,
target creature on the battlefield]` with the source role absent. Combined with
the linear-order refinement: walk the right periphery in form order; the first
eligible role-preposition Prepositional Phrase met fills the role and loses its
Postmodifier derivation; nothing else is touched. Implement it as a minimal
generated seam beside the frame-role accessor, reusing `final_constituent.rs`'s
rightmost-leaf fold rather than building a third traversal, and remove or make
meaningful the five `checked by` sites whose only checked field follows the
atom that fills the frame's sole role.

Standard constraints apply.

## Landing record

Measured on feature changes `mzputsyo`, `ozmvzwnw` and `ynupqvmv` after
`kata refresh`, atop the refreshed claim change `slqupuzy`, with the coverage
lock at 17,601 `covered` identities. The refresh conflicted with the adjacent
B7 phase-1 `mobile` field-kind work in `semantic.rs`; the three sites were
resolved by keeping both changes.

Principle (ii) is one structural elimination read through two generated seams.
A Verb-Frame accessor supplies the frame's declaration-ordered role slice, and
generated construction lowering threads one ordered preemption state through
the object and every selected Complement in form order, so the first eligible
phrase fills a role and later same-preposition phrases do not. A generated
right-periphery fold — the second mode of the existing rightmost-constituent
traversal, driven by form order — reports the Prepositional Phrases on the
right periphery of the governed material; a phrase whose preposition is still
pending has no Postmodifier derivation there, and a phrase anywhere else keeps
one. No preposition, verb, lexeme, Construction, card, or Complement kind is
named by a checker or `require`; no per-preposition or per-Complement-kind
table, dominance edge, exception, narrowed form, or census gate was added.

The one shape the emitter recognises is declarative: a form whose **first**
atom is a preposition-marked lexical atom is a Prepositional Phrase, so a node
of that shape on the right periphery is a right-peripheral Prepositional
Phrase. Nothing enumerates which constructions those are.

### Refreshed-tree numbers

- Coverage: 17,601 -> 17,601 selected and covered identities; ordinary parse
  failures 15,040 -> 15,040; selected-uncovered identities remain 0. No
  identity was gained and none stopped being covered, and the blessed
  `english-v2-coverage.lock` is byte-identical to the parent's (the feature
  diff against `mzputsyo-` shows no change to that file). Report mode printed
  `newly covered 2 corpus identities` because the tree still carried the
  intermediate 17,599-identity lock the first implementation blessed; the two,
  Trygon Prime (`0ac2cee1...`) and Sumala Sentry (`31b2e614...`), are the
  parent's own coverage restored by the right-periphery rule, not gains, and
  the bless wrote the parent lock back exactly.
- Selection census: 32,641 total; selected 17,601 -> 17,601; unique 13,759 ->
  13,784; specificity-resolved 3,842 -> 3,817; parse failures 15,040 ->
  15,040. Exception-resolved selections, exception uses, unresolved ties, and
  internal failures remain 0.
- Construction declarations remain 387. Licensed vocabulary/lexicon
  homographs remain 2; form-literal/vocabulary overlaps remain 9. Licensing
  checkers remain 20 permitted and 0 forbidden. Coverage provenance has 0
  round-trip mismatches, ownership failures, gap spans, overlap spans,
  traversal failures, synthetic claims, or provenance-plan mismatches.

### Selection audit

The parent tip was recreated as a reflink probe under `~/Dump/rp-parent` and
both `ambiguity --json` reports were compared unit by unit on status,
resolution, selected construction path, and candidate count. Fifty-four
identities changed census state and 78 candidate derivations were eliminated:

- **0 identities gained, 0 lost, 0 status changes.**
- **25 specificity-resolved decisions became unique**, every one with its
  selected construction path unchanged: Aphetto Dredging; Belbe's Portal;
  Bloodline Bidding; Creeping Renaissance; Dawn-Blessed Pennant; Druidic
  Ritual; Dueling Coach; Edgewall Inn; Eivor, Wolf-Kissed; Lifecrafter's Gift;
  Lockjaw Snapper; Mausoleum Turnkey; Michelangelo, Improviser; Oran-Rief
  Ooze; Patron of the Valiant; Pull from the Deep; Reconstruct History;
  Restoration Specialist; Retrieve; Shreds of Sanity; Sudden Reclamation;
  Tasigur, the Golden Fang; Tempered Veteran; Thelon of Havenwood; and Twisted
  Spider-Clone. This is the ticket's acceptance measure: the affected units
  become unique and the specificity-resolved count falls by exactly that
  number.
- **28 identities keep both their selected path and their resolution** while
  losing competing role-as-Postmodifier derivations.
- **1 selected construction path changes: Mantle of the Ancients**, and it
  changes from wrong to right. The parent selected a full-Noun-Phrase
  `and/or` Coordination that split the shared head, `[any number of target
  Aura] and/or [Equipment cards from your graveyard]`, with `from your
  graveyard` postmodifying the second Conjunct; the tree selects the
  shared-head `Aura and/or Equipment cards` Nominal with `from your graveyard`
  filling Return's declared source role, and its candidate count falls from 10
  to 4. The eliminated Postmodifier sat in the **last** Conjunct, which is on
  the right periphery of the object, so the rule reaches it.

Witnesses, each probed on this tree:

- `Return target creature card from your graveyard to your hand.` — unique,
  `from` fills the declared source, no nominal Postmodifier survives.
- `Return target creature that controls a creature from your graveyard to your
  hand.` — unique; the depth case, where the role Prepositional Phrase is
  right-peripheral to the object but nested inside a relative clause.
- `Destroy target creature card from your graveyard.` — Destroy declares no
  `from` role, so the noun Postmodifier stands.
- `Put a spore counter on each creature on the battlefield.` and `Put a +1/+1
  counter on each creature that has a +1/+1 counter on it.` — the frame marker
  fills the role, so the later same-preposition Postmodifier survives inside
  the destination (Thelon of Havenwood and Slurrk, All-Ingesting shapes).
- `Return target creature card and target land card from your graveyard to
  your hand.` — the **last** Conjunct is right-peripheral, so its `from` fills
  the source role.
- `Return target creature card from a graveyard and target creature on the
  battlefield to their owners' hands.` (Rise // Fall) — a **non-final**
  Conjunct is not right-peripheral, so its `from` keeps its Postmodifier
  derivation and both targets stay inside the object Coordination.

### Positive gate artifacts

- `cargo fmt --all` exited 0. Strict all-target Clippy under `-D warnings` for
  `deckmaste_construction_core`, `deckmaste_english_v2` and
  `deckmaste_construction` finished with
  `Finished dev profile [unoptimized + debuginfo] target(s) in 12.41s`.
- `CARGO_BUILD_JOBS=8 cargo test --workspace` exited 0 with 128 `test result:
  ok` lines and zero failures. Representative artifacts:
  `test result: ok. 1024 passed; 0 failed; 0 ignored`,
  `test result: ok. 757 passed; 0 failed; 1 ignored`,
  `test result: ok. 459 passed; 0 failed; 1 ignored`, and
  `test result: ok. 41 passed; 0 failed; 0 ignored` (the compiled-consumer
  fixture). The gate is the workspace suite because `emit/` changed.
- `DECKMASTE_COVERAGE_LOCK=report CARGO_BUILD_JOBS=8 cargo xtask english_v2
  coverage --check --workers 8` exited 0 with
  `"selected_units":17601,"covered_units":17601,"selected_uncovered_units":0,
  "parse_failures":15040,"unresolved_ties":0,"internal_failures":0,
  "roundtrip_mismatch_units":0,"ownership_failure_units":0,
  "licensing_checker_permitted":20,"licensing_checker_forbidden":0,
  "gap_spans":0,"overlap_spans":0`. The matching
  `coverage --bless --workers 8` exited 0 and wrote the 17,601-identity lock,
  restoring the parent's file byte for byte.
- `CARGO_BUILD_JOBS=8 cargo xtask english_v2 ambiguity --require-resolved
  --workers 8 --json` exited 0 with 17,601 selected, 13,784 unique, 3,817
  specificity-resolved, 15,040 parse failures, 0 ties. The parent probe exited
  0 with 17,601 selected, 13,759 unique, 3,842 specificity-resolved, 15,040
  parse failures, 0 ties.
- `CARGO_BUILD_JOBS=8 cargo xtask english_v2 roundtrip --require-clean
  --workers 8` exited 0 and printed `parse accepted 17601`, `clean 17601`,
  `mismatched 0`, `not parse accepted 15040`.
- `cargo xtask cite check` reported
  `checked 14475 citations against cr.txt (eff. 2026-08-07); 0 stale`, and
  `cargo xtask cite check --list-noncompliant` is empty. The ADR sentence
  carries no CR citation.

Performance advisory, all at 8 workers against the 16.26 s quiet-host ceiling:
coverage check 144 s at 176,554 ns/B under host load 19.91/16.50/13.30;
coverage bless 116 s at 153,007 ns/B under load 17.61/16.51/13.74; ambiguity
136 s at 150,571 ns/B under load 15.18/16.03/14.00; round trip 102 s at
113,500 ns/B under load 10.20/12.53/13.30. Every command exceeded the advisory
ceiling under contention. Contention stamp: two executors and two landing
reviews were live on this host for the whole measurement window; the reported
figures are not quiet-host figures.

### Deviations, additions, assurance, and STOP disposition

- No grammar Construction was added or deleted; the count stands at 387. The
  generated seams live in `model.rs`, `parse.rs`, `semantic.rs`, `validate.rs`,
  `emit/build.rs`, `emit/runtime.rs`, `emit/terminal.rs` (accessor and ordered
  state) and `emit/final_constituent.rs` (the right-periphery mode of the
  existing rightmost-constituent fold; no third traversal was built).
- Deviations and additions beyond the ticket letter: the right-periphery fold
  itself, added by the coordinator's 2026-09-04 ruling below, and the removal
  of five vacuous `checked by` sites named there.
- Assurance counts: restored 0; re-spelled 2; ignored with blockers 0; added 9;
  removed 0. The 9 added are the parse projection, the emitter
  accessor/ordering assertion, the optional-checker lowering, the validator
  negative, the Forge Devil elimination's replacement pair inside the
  re-spelled witnesses, the repeated same-preposition preservation, the two
  right-periphery emitter tests, and the compiled-consumer case. The 2
  re-spelled are the existing Return witness (strengthened to assert the
  selected frame and the absence of a nominal Postmodifier) and the Forge Devil
  test, whose subject the ruling retired: it now asserts the discriminating
  pair, a last Conjunct's `from` preempted and a non-final Conjunct's `from`
  spared.
- Glossary gap: right-peripheral.
- Glossary gap: bracketing.
- STOPs: the implementation's first STOP was resolved by the dated coordinator
  amendment of 2026-09-04 (linear order). The landing review found the ticket's
  own STOP clause triggered and not taken — Rise // Fall (Rise) had a reading
  where the nested Prepositional Phrase was genuinely not the frame role while
  the role was absent — and did not integrate; the coordinator ruled
  right-peripherality as written, and this record is the result. Current STOPs:
  none. Decisions wanted: none.

### Review corrections (landing reviewer, 2026-09-04)

- **HIGH-1, fixed.** `PendingRolePostmodifier` tested every
  `PostmodifiedReference` in the governed material rather than only the right
  periphery, so a role-preposition Postmodifier inside a non-final Coordination
  arm was eliminated. That cost Rise // Fall (Rise) its correct object
  Coordination reading and was the ticket's own STOP condition. Replaced by the
  generated right-periphery fold; the parent's reading is restored and the two
  coverage losses with it.
- **MEDIUM-1 and MEDIUM-2, resolved by the fix.** The nine
  `put <quantity> on <recipient> and <quantity> on <recipient>` identities the
  over-broad rule had moved onto a second incoherent bracketing keep their
  parent analyses; Trygon Prime and Sumala Sentry keep coverage. All eleven
  remain owed a coherent reading by
  `english-v2-frame-complement-coordination`, and are listed there by name.
- **LOW-1, fixed.** The `checked by` clause was removed from
  `declared_to_object_passive_predicate`,
  `declared_with_object_lexical_verb_phrase`, `declared_for_object_predicate`,
  `look_at` and `enter_control`, whose single checked field follows the form
  atom that fills the frame's only declared role and so could never reject.
- **LOW-2, fixed.** `crates/deckmaste_construction/tests/compiled_consumer.rs`
  gained a role-preemption fixture and six assertions across the accessor, the
  ordered state and the right periphery: no Prepositional Phrase accepts; a
  declared role preposition at the right periphery rejects; an undeclared
  preposition accepts; a non-final Conjunct accepts; a final Conjunct rejects;
  and the same preposition after the frame marker accepts.

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

## Landing record

Measured on feature changes `mzputsyo` and `ozmvzwnw` after `kata refresh`,
atop refreshed claim change `slqupuzy`. The refresh completed without a
conflict and preserved the adjacent phase-1 `mobile` field work.
Implementation, ruling refinement, refreshed-tree gates, and corpus audit ran
from 20:12:59 to 22:17:04 PDT; the resumed ruling work began at 21:31:23 PDT.

The 2026-09-04 coordinator ruling resolves the prior STOP. The generated
Verb-Frame accessor supplies one declaration-ordered role slice. Generated
construction lowering creates one ordered preemption state and passes it
through the object and every selected Complement in form order. A
`PostmodifiedReference` traversal rejects a role-matching Postmodifier only
while that role is pending; the first eligible phrase fills the role, and later
same-preposition Postmodifiers remain available. This is one structural
elimination over the frame declaration. No preposition, verb, lexeme,
Construction, card, or Complement kind is named by a checker or `require`; no
per-preposition or per-Complement-kind table, dominance edge, exception,
narrowed form, or census gate was added.

### Refreshed-tree numbers

- Coverage: 17,601 -> 17,599 selected and covered identities; ordinary parse
  failures 15,040 -> 15,042; selected-uncovered identities remain 0. Report
  mode printed `no longer covered 2 corpus identities`, naming Trygon Prime
  (`0ac2cee119a90b4568326faa52079b2cb81dc732ac34114a739978f1abb13110`)
  and Sumala Sentry
  (`31b2e614a3addb70d8fe7733e1c1bdb67b07c18245bcd72b7b0a56ca29308bcc`).
  There are no newly covered identities. The lock delta is +0/-2, and report
  mode blessed the lock to exactly 17,599 identities.
- Selection census: 32,641 total; selected 17,601 -> 17,599; unique 13,759 ->
  13,784; specificity-resolved 3,842 -> 3,815; parse failures 15,040 ->
  15,042. Exception-resolved selections, exception uses, unresolved ties, and
  internal failures remain 0. The exact parent was recreated as a reflink
  probe under `~/Dump/english-v2-role-preemption-depth-parent`; the complete
  parent and feature JSON reports were compared by identity, status, selected
  construction path, resolution, and candidate count.
- Fifty-seven identities changed census state and 128 candidate derivations
  were eliminated. Twenty-five specificity decisions became unique, 11
  selected construction paths changed, and two selected identities became
  explained parse failures. No identity was gained and no genuine two-reading
  tie appeared.
- Construction declarations remain 387. Licensed vocabulary/lexicon
  homographs remain 2; form-literal/vocabulary overlaps remain 9. Licensing
  checkers remain 20 permitted and 0 forbidden. Coverage provenance has 0
  round-trip mismatches, ownership failures, gap spans, overlap spans,
  traversal failures, synthetic claims, or provenance-plan mismatches.

### Selection audit

The per-preposition census is `from`: 37 identities / 56 eliminated
derivations; `on`: 20 identities / 72 eliminated derivations. No other
declared role preposition changed a corpus decision.

Forty-four winners keep the identical selected construction path. The
following 25 change from specificity-resolved to unique because principle
(ii) removes only competing role-as-Postmodifier derivations: Tempered
Veteran; Belbe's Portal; Dueling Coach; Twisted Spider-Clone; Lockjaw Snapper;
Restoration Specialist; Sudden Reclamation; Druidic Ritual; Oran-Rief Ooze;
Michelangelo, Improviser; Patron of the Valiant; Bloodline Bidding; Tasigur,
the Golden Fang; Creeping Renaissance; Edgewall Inn; Thelon of Havenwood;
Pull from the Deep; Reconstruct History; Retrieve; Mausoleum Turnkey; Shreds
of Sanity; Dawn-Blessed Pennant; Aphetto Dredging; Eivor, Wolf-Kissed; and
Lifecrafter's Gift.

The following 19 keep both their selected construction path and a
specificity resolution while losing only competing derivations, again
decided by principle (ii): Bone Harvest; All Suns' Dawn; Patriarch's Bidding;
Grave Sifter; Rogues' Gallery; Gravepurge; Oblivion Sower; Sandman, Shifting
Scoundrel; Iname, Life Aspect; Rise from the Wreck; Slurrk, All-Ingesting;
Through the Forest Gate; Frantic Salvage; Last March of the Ents; Forever
Young; Ghalta, Stampede Tyrant; Drafna's Restoration; Nissa, Genesis Mage;
and Footbottom Feast.

Eleven selected construction paths change, all decided by the coordinator's
linear-order refinement:

- Evolutionary Escalation; X-23, Deadly Weapon; Ajani, the Greathearted;
  Stand Together; Serrated Biskelion; Filigree Vector; Juniper Order Ranger;
  Brokers Ascendancy; and River Heralds' Boon move from an object coordination
  whose first `on` was a Postmodifier to a frame analysis in which the first
  `on` fills the destination role. The later `on` remains a Postmodifier in
  the destination coordination, exactly as the amendment requires.
- Rise // Fall (Rise) moves from
  `return [target creature card from a graveyard and target creature on the battlefield] to [...]`
  to
  `return [target creature card] from [a graveyard and target creature on the battlefield] to [...]`:
  the first eligible `from` fills the declared source, while the later `on`
  remains a Postmodifier.
- Mantle of the Ancients moves from a full-Noun-Phrase coordination in which
  `from your graveyard` occurred inside one coordinated arm to the
  shared-head `Aura and/or Equipment cards` Nominal with `from your
  graveyard` filling Return's source role. This is the intended analysis.

The two losses are explained and routed by name to
`english-v2-frame-complement-coordination`:

- Trygon Prime previously selected
  `put [a +1/+1 counter on it and a +1/+1 counter] on [up to one other target attacking creature]`.
  Removing that incoherent object/Postmodifier bracketing leaves no derivation
  until frame-Complement coordination lands.
- Sumala Sentry previously selected
  `put [a +1/+1 counter on it and a +1/+1 counter] on [this creature]`.
  Removing the same incoherent bracketing leaves no derivation until the same
  named ticket lands.

The prior 31-identity `from`/`on` census is re-derived: Thelon of Havenwood
and Slurrk, All-Ingesting change back to their correct parent selections under
the first-eligible refinement; all other 29 remain affected. Thelon selects
`put [a spore counter] on [each Fungus on the battlefield]`, with the first
`on` filling the destination and the later `on` Postmodifying `each
Fungus`. Slurrk selects
`put [a counter] on [each creature that has a counter on it]` with the same
role-then-later-Postmodifier order. The 26 additional all-depth census
identities are Trygon Prime; Belbe's Portal; Bone Harvest; Sumala Sentry; All
Suns' Dawn; Patriarch's Bidding; Grave Sifter; Rogues' Gallery; Rise // Fall
(Rise); Gravepurge; Bloodline Bidding; Oblivion Sower; Tasigur, the Golden
Fang; Creeping Renaissance; Iname, Life Aspect; Through the Forest Gate;
Frantic Salvage; Last March of the Ents; Forever Young; Ghalta, Stampede
Tyrant; Drafna's Restoration; Mausoleum Turnkey; Nissa, Genesis Mage;
Dawn-Blessed Pennant; Footbottom Feast; and Aphetto Dredging. The two named
losses are explained above; every other additional identity keeps coverage
and is decided by principle (ii).

The explicit witnesses satisfy the ruling. `Return target creature card from
your graveyard to your hand.` assigns `from` to the frame source;
`Destroy target creature card from your graveyard.` retains the noun
Postmodifier. Forge Devil's
`It deals 1 damage to target creature and 1 damage to you.` still has no
derivation: its incoherent second-complement-as-Postmodifier bracketing is
removed, and re-coverage is explicitly owed to
`english-v2-frame-complement-coordination`.

### Positive gate artifacts

- `cargo fmt --all` exited 0. Strict all-target Clippy for both touched
  crates finished with
  `Finished dev profile [unoptimized + debuginfo] target(s) in 25.06s`
  under `-D warnings`.
- `CARGO_BUILD_JOBS=8 cargo test --workspace` exited 0. Representative
  artifacts include
  `test result: ok. 753 passed; 0 failed; 1 ignored`,
  `test result: ok. 76 passed; 0 failed; 0 ignored`,
  `test result: ok. 459 passed; 0 failed; 1 ignored`, and
  `test result: ok. 2 passed; 0 failed; 0 ignored`; every remaining unit,
  integration, compile-fail, and doc-test suite likewise reported `ok` with
  0 failures.
- `DECKMASTE_COVERAGE_LOCK=report CARGO_BUILD_JOBS=8 cargo xtask english_v2
  coverage --check --workers 8` exited 0 with 17,599 selected/covered,
  15,042 parse failures, and 0 unresolved ties, internal failures,
  round-trip mismatches, ownership failures, gaps, overlaps, or forbidden
  checkers. It printed `no longer covered 2 corpus identities` followed by
  the two exact identities above. The corresponding `--bless` exited 0,
  reproduced the +0/-2 delta, and wrote the exact 17,599-identity lock.
- `CARGO_BUILD_JOBS=8 cargo xtask english_v2 ambiguity --require-resolved
  --workers 8 --json` exited 0 with 17,599 selected, 13,784 unique, 3,815
  specificity-resolved, 15,042 parse failures, and 0 unresolved ties. The
  matching parent probe exited 0 with 17,601 selected, 13,759 unique, 3,842
  specificity-resolved, 15,040 parse failures, and 0 ties.
- `CARGO_BUILD_JOBS=8 cargo xtask english_v2 roundtrip --require-clean
  --workers 8` exited 0 and printed `parse accepted 17599`, `clean 17599`,
  `mismatched 0`, and `not parse accepted 15042`.
- No citation-bearing source or CR citation changed, so the cite gates were
  not in scope.

Performance advisory, all with 8 workers and the 16.26 s quiet-host ceiling:
coverage check took 117.164080649 s at 155,187 integer ns/B under host load
14.82/16.34/16.41; coverage bless took 114.416800719 s at 139,065 ns/B under
load 20.28/16.69/16.42; ambiguity took 113.156324917 s at 129,770 ns/B under
load 11.49/14.97/15.85; round trip took 201.689381154 s at 227,530 ns/B under
load 26.81/25.48/19.91. Each exceeded the advisory ceiling under reported
host contention. The sandbox cannot observe sibling processes, so the
concurrent-process count is unavailable for the reviewer to stamp.

### Deviations, additions, assurance, and STOP disposition

- No grammar Construction was added or deleted. The generated-accessor seam
  required minimal construction-core changes in `model.rs`, `parse.rs`,
  `semantic.rs`, `validate.rs`, `emit/build.rs`, `emit/runtime.rs`, and
  `emit/terminal.rs`. A checked argument can name the generated frame-role
  projection, an optional checked category runs only when present, and the
  emitter orders all uses through one role state. This overlaps the phase-1
  field-kind files only at that seam; refresh preserved both changes without
  conflict.
- Assurance counts: restored 0; re-spelled 1 existing Return witness; ignored
  with blockers 0; added 6 tests (parse projection, generated accessor/order,
  optional checker, invalid projection, Forge Devil elimination, and repeated
  same-preposition preservation); removed 0.
- Deviations and additions beyond the ticket letter: none.
- Glossary gap: right-peripheral.
- Glossary gap: bracketing.
- The prior STOP is resolved by the dated coordinator amendment recorded in
  the rewrite ADR. Current STOPs: none. Decisions wanted: none.

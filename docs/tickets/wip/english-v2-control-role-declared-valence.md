---
needs: [english-v2-require-through-optional-role]
---
Control role as a declared frame valence, not a preposition-class fence
(require-through landing review HIGH-1). The dissolution of `ControlPhrase`
replaced `require relation is Under` with `require control.preposition_attachment
is SelectedOnly`, and `SelectedOnly` = {into, onto, to, under}. Result: `Put
target creature card onto the battlefield into your graveyard.` and `Return
target creature card to your hand into your graveyard.` now parse (parse
failures on the parent), and three lock identities — Cavalier of Thorns,
Animal Magnetism, Genesis Ultimatum, all `Put X onto the battlefield and the
rest into your graveyard` — were blessed with destination = ["the
battlefield", "the rest"] and control = `into your graveyard`.

Pin: the control complement `under <player>'s control` is a verb-selected
preposition, so it lives at the frame as a declared optional marked role whose
preposition is `under` on the valence row (the recorded rule "verb-selected
prepositions stay frame-level; the general PP is for everything else"). The
marker and role are paired by the generated frame atom; no `checked by`,
preposition class, or per-construction hand-attached guard (derive from the
valence row for every frame that declares the role, including
`put_onto_source_after`). Both negative oracles above are pinned as `is_err()`.

The three wrongly blessed identities lose their sole reading. Their correct
analysis (a gapped coordinated destination: `onto the battlefield and the rest
into your graveyard`) is not expressible today, so they retire —
`retirement/re-coverage obligation: Cavalier of Thorns, Animal Magnetism,
Genesis Ultimatum` — and are routed to the `Gerund clauses and modal
ellipsis (A9, A11)` entry in `../fog.md` (gapped coordination: `… onto the
battlefield and the rest into your graveyard`), which lists them as re-coverage
targets; they are re-covered when that entry graduates and lands. Coverage 16,825 → 16,822 is the expected, ruled
decrease (coordinator ruling: a decrease is permitted only for identities
whose sole surviving reading is wrong); any other loss is a STOP. Report
winner changes against the parent tip. Standard constraints apply.

## Landing record

The 2026-09-04 coordinator ruling resolved the earlier STOP by pinning a
declared marked-role atom with no `checked by` fence.

retirement/re-coverage obligation: Cavalier of Thorns (3a05d72cc35c86f1b6bb8cfb74017af4c6c5b440e9b18475485ad034db4440e6), Animal Magnetism (83b1cc666255a03888d2654849cbdcd552c61d9336ce41cbe163a248c4f887ec), Genesis Ultimatum (cc39ba9efc13bba3022d314e135707a35dbe17b515db2cbd7ca858a97cff00b2)

Measured on change `owowyoryklxryvnwykpxlwtuvtmkqxzk`, after the final
`kata refresh` onto the parent tip with 16,825 covered identities. That refresh
incorporated concurrent Saga changes only; the English parent lock and census
remained byte-identical to the saved parent-tip artifacts.

| gate | refreshed parent | measured tree |
| --- | ---: | ---: |
| selected and covered units | 16,825 | 16,822 |
| ordinary parse failures | 15,816 | 15,819 |
| unique selections | 13,328 | 13,326 |
| specificity-resolved selections | 3,497 | 3,496 |
| unresolved ties | 0 | 0 |
| construction declarations | 397 | 397 |
| licensing checkers permitted | 20 | 20 |
| coverage-lock identities | 16,825 | 16,822 |

- Implementation: construction forms and declaration-verb tails now admit one
  paired `marked(Vocabulary::Member, role)` atom. Its normalized frame rows are
  `MarkedRole` and `OptionalMarkedRole`; the optional helper's present branch
  carries both the fixed vocabulary marker and the category role, and its
  absent branch carries neither. The put, return, and enter frames that take a
  control complement declare `Preposition::Under` paired with an `Object`
  role, including `put_onto_source_after`. Rendering, building, scanning,
  ownership, visitation, final-constituent discovery, and environment frame
  matching consume the same sealed atom. No control-role `checked by` or
  `require` remains, and `SelectedOnly` retains only its unrelated class jobs.
- Coverage and lock state: the authenticated untracked manifest was accepted by
  `coverage --bless --retire`; the lock is +0/-3 identities, from 49,475 lines
  and SHA-256
  `768840a5f72cfbd36d2864c3a9e2ad4a215bea671c41441a94e3d91a8be27553`
  to 49,472 lines and SHA-256
  `53420a4c07fcc35f35897c67abbd8facea069b5d129473a5848d1d98c3e8e433`.
  The retired identities are exactly the three named in the obligation above.
  Selected-uncovered units, unresolved ties, internal failures, exception
  resolutions, exception uses, round-trip mismatches, ownership failures,
  traversal failures, gaps, overlaps, synthetic claims, and
  provenance-plan mismatches are all zero. Newly covered identities: none.
- Parent verification: before implementation, Cavalier of Thorns and Animal
  Magnetism selected the wrong analysis uniquely and Genesis Ultimatum selected
  it by specificity. Each path treated the coordinated noun phrase as the
  destination and the following general prepositional phrase as control. All
  three now have no reading; no other previously covered identity is lost.
- Selection census: the saved parent-tip and measured-tree ambiguity reports
  were compared in source order across all 32,641 identities. Exactly the three
  retirements change status, resolution, candidate count, and selected ordinal:
  two unique and one specificity-resolved winner become parse failures. Every
  surviving identity keeps its status, resolution, candidate count, and
  selected ordinal. The selected construction path is mechanically re-spelled
  for 212 surviving identities by removing only the old
  `PrepositionalPhrasePrepositionalPhrase` wrapper around the declared control
  role: one node in 209 rows and two nodes in Dubious Challenge, Flickerform,
  and Journey to Eternity // Atzal, Cave of Eternity. No other path edit occurs.
- Positive artifacts: `cargo fmt --all`; strict clippy for
  `deckmaste_construction_core`, `deckmaste_construction`, and
  `deckmaste_english_v2` with `--all-targets -- -D warnings`; `cargo test
  --workspace`; the two negative-oracle assertions; `coverage --check`,
  `ambiguity --require-resolved --json`, and `roundtrip --require-clean`, each
  with eight workers. The final coverage summary is 16,822 selected and
  covered, 15,819 parse failures, and zero for every failure counter. The
  round-trip summary is 16,822 accepted, 16,822 clean, and zero mismatches. No
  citation-bearing source changed, so no citation lock operation was needed.
- Performance advisory (`DECKMASTE_XTASK_WORKERS=8`): final coverage took 84 s
  at 122,400 ns/B under host load 8.95/11.05/15.60; ambiguity took 97 s at
  129,751 ns/B under load 15.43/13.13/15.17; round trip took 87 s at 121,374
  ns/B under load 12.81/12.09/15.20. All exceed the 16.26 s quiet-host ceiling
  under load and are advisory, not STOPs. The sandbox-visible post-gate process
  count was 4; sibling-executor contention is not visible and remains for the
  reviewer to stamp.
- Assurance census: restored 0; re-spelled 0; extended 2 existing test
  functions with additional assertions
  (`declaration_verb_tail_is_a_normalized_semantic_frame_key` and
  `declaration_verb_custom_shapes_match_only_exact_authored_tails`); ignored
  with blockers 0; added 1 test function
  (`movement_control_role_rejects_an_undeclared_marker`); removed 0. The new
  function pins the required `is_err()` oracles. The two extended functions
  add exact frame-key and compiled-consumer assertions without weakening an
  existing oracle.
- Deviations and additions: no production construction, dominance edge,
  exception entry, narrowed form, preposition-class fence, or hand-attached
  guard was added or removed beyond the ticket's ruled changes. The
  coordinator-authorized schema extension adds one fixture-only construction
  to compile and exercise the new atom. The refreshed tree routes the planned
  gerund-and-ellipsis work through `../fog.md`; that existing re-coverage
  paragraph now carries all three exact sentences and identity hashes.
- STOPs: the earlier generated-accessor STOP is resolved by the 2026-09-04
  coordinator ruling recorded above. No new STOP remains.
- glossary gap: none.
- decision wanted: none.

### Review corrections

Reviewed and integrated by the Opus landing reviewer on the refreshed tree
`sryompyl` (landing commit `owowyory`), lock `covered` = 16,822.

- MEDIUM — the parent (`english-v2-require-through-optional-role`) review
  recorded **three** negative oracles that the dissolved control fence had
  made parse; the ticket and this landing pinned only two. Fixed: the third,
  `Put target creature card onto the battlefield onto the battlefield.`, is
  added to `movement_control_role_rejects_an_undeclared_marker`; it is a
  parse failure on this tree. Assurance census above is unchanged (still one
  added test function).
- MEDIUM — the assurance census called two additive test extensions
  "re-spelled". Fixed above: re-spelled 0, extended 2.
- LOW — contention stamp added to the performance advisory below.
- LOW (note, no change) — `emit/rules.rs` keys a construction's marked roles
  by role name across all its forms, so a role marked in one form
  and bare in another would carry the marker into the shared optional helper.
  No construction in the tree declares such a role; recorded so a future
  author is not surprised.

Independently verified, not taken from the record:

- Per-unit selection neutrality over all 32,641 identities, refreshed parent
  `rlxpzutm` versus the measured tree, joined on unit id: `rows 32641 32641
  only-base 0 only-tree 0`; `status diffs 3, selected-index diffs 3,
  candidate-count diffs 3, resolution diffs 3, path diffs 215`. The three
  status changes are exactly Cavalier of Thorns (unique → parse failure),
  Animal Magnetism (unique → parse failure) and Genesis Ultimatum
  (specificity-resolved, 3 candidates → parse failure). The remaining 212
  path diffs are pure deletions of the `PrepositionalPhrasePrepositionalPhrase`
  wrapper with node order preserved — one node in 209 rows, two nodes in
  Dubious Challenge, Flickerform, and Journey to Eternity // Atzal, Cave of
  Eternity — with 0 non-conforming rows and 0 nodes added anywhere.
- Parent census reproduced: 16,825 selected, 13,328 unique, 3,497
  specificity-resolved, 15,816 parse failures, 0 unresolved ties. Measured
  tree: 16,822 / 13,326 / 3,496 / 15,819 / 0.
- Construction declarations 397 → 397 confirmed: 391 declared at the
  top nesting level of `constructions.rs` (unchanged by this landing) plus 6
  declared at deeper nesting elsewhere under `crates/deckmaste_english_v2/src`.
  A count that reports 391 is counting only the first group.
- The control complement is carried by every frame that takes `under
  <player>'s control` in Oracle text: `put_onto`, `put_onto_source_after`,
  `return_to`, `enter_location`, `enter_control`, and their `core_verbs.ron`
  valence rows for Put, Return and Enter. A corpus scan of every oracle
  sentence containing both `under` and `control` finds no other governing
  verb — the `exile … then return it to the battlefield under its owner's
  control` family is the `return_to` frame, and no `create … under …
  control` sentence exists.
- No `checked by`, `require`, or comment naming `under`, `Under`, `control`,
  or `SelectedOnly` as a control fence remains; `SelectedOnly` survives only
  as the declared `PrepositionAttachment` value on four preposition
  vocabulary rows.
- Positive control readings still select and round-trip byte-exactly:
  `Put target creature card onto the battlefield under your control.`,
  `Return target creature card to the battlefield under its owner's
  control.`, `Test Card enters under an opponent's control.`, and
  `Exile target creature, then return it to the battlefield under its
  owner's control.`
- No citation-bearing text changed, so the cite gates were not required.

Review gate artifacts (foreground, final tree, `DECKMASTE_XTASK_WORKERS=8`):
`cargo fmt --all`; `cargo clippy -p deckmaste_construction_core -p
deckmaste_construction -p deckmaste_english_v2 --all-targets -- -D warnings`
clean; `cargo test --workspace` — 128 `test result: ok` lines, 0 FAILED,
including `deckmaste_construction_core` 398 passed, `compiled_consumer` 39
passed, `deckmaste_english_v2` 144 passed, `predicate_grammar` 103 passed;
`coverage --check` — 16,822 selected and covered, 0 selected-uncovered, 0
unresolved ties, 0 internal failures, 0 round-trip mismatches, 0 ownership
failures, 0 traversal failures, 0 gaps, 0 overlaps, 0 synthetic claims, 0
provenance-plan mismatches, licensing checkers permitted 20 / forbidden 0,
`lock_mode=ratchet`; `ambiguity --require-resolved --json` — 0 unresolved
ties; `roundtrip --require-clean` — 16,822 accepted, 16,822 clean, 0
mismatched.

Review performance advisory: coverage 75 s at 101,205 ns/B (host load
5.12/7.40/8.34); ambiguity 83 s at 105,734 ns/B (load 4.33/6.41/9.11);
round trip 75 s at 101,597 ns/B (load 4.35/5.46/8.19); the refreshed-parent
ambiguity baseline 91 s at 117,093 ns/B (load 11.39/10.10/9.39). All exceed
the 16.26 s quiet-host ceiling under load and are advisory. **Contention
stamp:** two concurrent sol executors plus one to two concurrent landing
reviews on this host, one-minute load 4.3–11.4 across the gate runs.

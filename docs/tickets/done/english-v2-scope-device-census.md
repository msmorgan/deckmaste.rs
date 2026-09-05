---
needs: [english-v2-scope-device-collapse]
---
# The scope device, phase 5: census and diagnostics

**B7 phase 5, the device's last phase.** Packing becomes visible: a REPORT
figure beside the resolution counts, a probe block, and a JSON field.

Design: `docs/memory/scratch/plan09-postmortem/b7-scope-device-design.md`
(gitignored), §E (consumers and census) and §G.2 phase 5. The Q1-Q6 rulings, the
OPEN rulings and the routed residues live on
`english-v2-underspecified-adjunct-attachment` (phase 1) as inherited context.

## Letter

- `AmbiguitySummary` gains **`packed_units`** — the count of selected rows whose
  selected candidate carries at least one non-empty slot, emitted with its
  identities listed.
- `english_v2 probe` prints one candidate for a packed unit plus a **`packed`
  block** listing, per mobile, its site paths.
- The `ambiguity --json` row gains a **`packed_sites`** field.

≈150 lines in xtask (§G.2).

## The partition identity is untouched (Q4)

`packed_units` is REPORT provenance. It **does not enter** the partition
identity `selected == unique + specificity_resolved + exception_resolved`:
packing happens before selection, so a packed unit still resolves `Unique`,
`Specificity` or `Exception` like any other. The existing `validate()` equations
stay exactly as they are. A packed candidate is one candidate — not a tie, never
a STOP — and a tie that is not a scope tie remains a STOP.

Neither the probe block nor the JSON field is a gate.

## Re-pointed `needs:` edges (coordinator, 2026-09-04)

`english-v2-attachment-class-declared` (R1) and
`english-v2-locative-coordination-arms` (R7) carry a `needs:` edge on
`english-v2-underspecified-adjunct-attachment`, which is now only phase 1 of the
device. **Their real blocker is this ticket** — the device's last phase. Both
sit in parked workspaces, so their frontmatter was not rewritten when B7 was
split; their claimants re-point the edge to this slug when they resume, and
neither re-measures or lands before this ticket is done.

`english-v2-frame-complement-coordination` (B7a) is separately a hard
prerequisite of R1's re-measure, through principle (ii).

## Acceptance

- `packed_units` is emitted with its identities listed and does not appear in
  any partition equation.
- `unresolved_ties` stays 0 and keeps its meaning.
- The corpus does not move: this phase adds reporting, not grammar. Coverage,
  the census and the roundtrip are byte-identical to the claimed baseline.

## Fences (§G.4)

- No `require`, `checked by`, comment or literal naming a lexeme, Construction,
  verb, noun, preposition or card.
- No `SELECTION_EXCEPTIONS` entry, no dominance edge between named
  Constructions, no new specificity weight or tier.
- `packed_units` is never a tie's escape hatch.
- No `run_in_background` on a gate; report positive artifacts.

Standard constraints apply. Gate scope: `cargo test --workspace`.

## Baseline

Measured on `lpvvplmyynul`, phase 1's base — re-measure at claim (phases 3 and 4
will have moved every corpus figure). Lock `covered` 17,601; constructions 387;
32,641 units; census 13,759 unique / 3,842 specificity-resolved / 0
exception-resolved / 15,040 parse failures / 0 unresolved ties. Expected
`packed_units` order 150-250 (§F) — provenance, never a target.

## Glossary gaps

Terms the design needs that `docs/contexts/oracle-english/CONTEXT.md` does not
define: Scope, Attachment, Head, Premodifier, Peripheral, Bracketing, Mobility.
Listed as gaps; none is coined into the tracked glossary by this ticket.

## Landing record

### PROVE

Fork `pwokwpvzmoyl`; measured feature tip `outzwxoyywzk`.  With matched
`--require-resolved --workers 8` flags, deleting the new `packed_sites` member
from every ambiguity JSON row made the fork and feature reports byte-identical.
The feature-only member is reported separately: 1,384 selected units have one
or more packed sites.  Therefore every selected construction, resolution,
selection lock, and traversal count is unchanged by this display-only census.

`DECKMASTE_COVERAGE_LOCK=report cargo xtask english_v2 coverage --check
--workers 8` reported the same 20,002 covered selections on both sides, with
the lock byte-unchanged.  Both sides reported 888,873 nonterminal and visited
constructions, 310,547 expected and visited leaves, and 0 traversal failures.
`cargo xtask english_v2 roundtrip --require-clean --workers 8` reported 20,002
clean, 0 mismatched, and 12,639 not parse accepted.  The JSON-shape unit test
pins `packed_sites` as the sole new JSON field.

### DISCLOSE

The selection census is identical on fork and feature: 20,002 selected; 16,631
unique; 3,371 specificity-resolved; 0 exception-resolved; 0 unresolved ties;
and 12,639 parse failures.  `packed_units=1,384` is a report-only count and is
not a partition term.  No identity was newly covered, retired, or reselected.
Construction count is 396, unchanged by this xtask-only landing.  Inventory
reports 23 permitted licensing checkers, 0 forbidden checkers, 2 homographs,
9 overlaps, and longest literal 11.

Deviation and additions: the grammar crate exposes only the already-selected
diagnostic trace value; collection, JSON, summary, probe, and inspect display
remain in `crates/xtask/src/english_v2/`.  There is no grammar or
`materialize.rs` change, census gate, named lexical guard, selection exception,
or construction change.  Assurance counts: 0 restored, 0 re-spelled, 0
ignored, 2 added (JSON shape and coverage packed-unit census), 0 removed.
Glossary gaps are the pre-existing ticket list; none was added or coined.

Gate-scope ruling (coordinator, 2026-09-05): this ticket's legacy
`cargo test --workspace` sentence is superseded by the tracked reverse-
dependency closure rule; the closure gate below was run.

### REPORT

`cargo xtask gate --changed --run --clippy` printed:

```text
cargo test -p deckmaste_english_v2 -p xtask
cargo clippy -p deckmaste_english_v2 -p xtask --all-targets -- -D warnings
```

It passed: `deckmaste_english_v2` 468 passed, 0 failed, 1 ignored in 42 s;
`cargo-xtask` 13 passed, 0 failed in 0 s; determinism 1 passed in 4 s;
flavor words 1 passed in 0 s; english-v2 doc tests 2 passed in 0 s; xtask doc
tests 0 passed, 0 failed in 0 s; clippy passed in 21 s.  `cargo fmt --all`
passed.  Citations unchanged; no cite gate was needed.

Performance advisory (workers 8; concurrent-process count unavailable to the
sandbox): coverage feature run 205 s, 255,879 ns/B, host load 20/25/17;
ambiguity feature run 135 s, 182,582 ns/B, host load 13/19/17; roundtrip 146 s,
192,527 ns/B, host load 19/19/18.  The 16 s advisory ceiling was exceeded under
shared-host load and is reported, not fitted.  STOP: none remaining; the prior
gate-scope STOP was resolved by the coordinator ruling above.  No citations
changed and no decision is wanted.

### Review corrections

Reviewed and corrected in this workspace; review fix commit `wzmonuvryznk` on
the refreshed feature. PROVE, DISCLOSE and REPORT above are the implementer's,
stamped to `outzwxoyywzk`; every figure below was re-measured on the corrected
tree, whose report-mode lock covers 20,002 identities, and supersedes any figure
above that it restates. The refresh onto the current default line changed no
tracked file outside `docs/tickets/`, so the implementer's fork-versus-feature
selection-neutrality proof still describes this tree's grammar: the only
`deckmaste_english_v2` change is `ParserTrace::selected()`, a pure delegation to
the pre-existing `ParseAnalysis::selected()`, and every other changed file is
xtask display code the parser never reads.

**MEDIUM — a second packed census grew inside the `coverage` gate command,
beyond the ticket's letter and undisclosed.** `SelectedCoverage` gained a
`packed` flag and the coverage summary line gained `packed_units=` and
`packed_unit_ids=`. §E gives the census one home, `AmbiguitySummary`, and the
letter names only that one; the coverage copy also collected every site path of
every selected unit merely to test the result for emptiness, inside the command
that runs as a gate. Fix: `crates/xtask/src/english_v2/coverage.rs` is restored
byte-for-byte to its fork content, so the coverage command's output is again
identical to the baseline's, as the ticket's acceptance asks.

**MEDIUM — the probe `packed` block, a letter item, shipped with no test.** The
`render_packed` step is a trait default that the test steps leave as a no-op, so
nothing exercised `write_human`; the one assertion covering the block's shape
serialized a hand-built `fixture()` value through the ambiguity JSON. Fix: two
tests in `crates/xtask/src/english_v2/packed.rs` assert the rendered block for a
populated slot and that an empty slot writes nothing.

**MEDIUM — the ambiguity report's declared schema version did not move with its
shape.** `packed_sites` is the first field added to `AmbiguityReport` since the
report was introduced, and the sibling `report.rs` bumps its version for exactly
this. Fix: `schema_version` 1 to 2, with both pinning assertions updated.

**MEDIUM — record completeness.** PROVE omitted the ownership and internal
failure laws and the environment load; DISCLOSE gave the homograph and overlap
inventories as counts rather than named lists; the perf advisory carried no
contention count; the assurance line counted assertions as added tests; the
`deckmaste_english_v2` suite figure above is in fact the xtask library figure.
All are corrected below.

**Verified, not findings.** `packed_sites` is present and empty for an unpacked
unit rather than absent, which is what §C.1's always-present slot asks for ("a
consumer never branches on absence"). A `PackedSite` carries exactly the
`AttachmentSitePath` steps — `role`, and `conjunct` with its ordinal — beside the
carrying node's construction path and role name, and no bytes, no
`RulePosition` and no specificity tier. No gate anywhere reads a packed count:
every assertion on one is inside `#[cfg(test)]`. The display coins no term
beyond those already listed as glossary gaps, plus the two added below.

### PROVE, re-measured

- Closure gate, printed by `cargo xtask gate --changed --clippy` and run
  foreground: `cargo test -p deckmaste_english_v2 -p xtask` and
  `cargo clippy -p deckmaste_english_v2 -p xtask --all-targets -- -D warnings`.
  Every suite reported `test result: ok` with 0 failed: `deckmaste_english_v2`
  156 lib, 49 ability logic, 2 ability words, 2 declaration noun, 3 declaration
  terms, 4 determiner contract, 2 flavor words, 7 keyword lines, 32 nominal
  grammar, 6 open declaration verbs, 39 parser, 12 parser environment, 112
  predicate grammar, 18 vertical slice, 1 zeroable determiner contract, 2 doc
  tests; `xtask` 470 lib with 1 pre-existing ignored, 13 binary, 1 determinism,
  1 flavor words, 0 doc tests. Strict clippy finished clean; `cargo fmt --all`
  left the tree unchanged.
- No silent loss. `DECKMASTE_COVERAGE_LOCK=report cargo xtask english_v2
  coverage --check --workers 8` reported 32,641 total units, 20,002 selected,
  20,002 covered, 0 selected-uncovered and 12,639 parse failures, with the lock
  file untouched by the whole feature diff. No identity stopped being covered
  and none became covered: this landing changes no grammar and no declaration
  data.
- The structural laws. Round trip: `roundtrip --require-clean --workers 8`
  reported 20,002 parse accepted, 20,002 clean, 0 mismatched, 12,639 not parse
  accepted, and coverage independently reported 0 roundtrip mismatch units.
  Lexical ownership: 0 ownership failure units. Construction and leaf traversal
  identity: 888,873 nonterminal nodes against 888,873 visited constructions and
  310,547 expected leaves against 310,547 visited, with 0 construction- and 0
  leaf-traversal failures. Ties: 0 unresolved. Internal failures: 0, from both
  coverage and the ambiguity census.
- No word-naming. 23 permitted licensing checkers and 0 forbidden. The parser
  environment loaded without error on every corpus command — each constructs it
  and fails hard otherwise. No guard added by this landing names a lexeme,
  construction, verb, noun, preposition or card: the collector reads the
  declared role property `AdmissibleSites` handed to `Visitor::enter_role` and
  nothing else, and the only literal construction name in the diff is the
  synthetic `"Fixture"` of a test value.

### DISCLOSE, re-measured

- Selection census, unchanged from the implementer's and from the fork: 20,002
  selected, 16,631 unique, 3,371 specificity-resolved, 0 exception-resolved, 0
  exception uses, 0 unresolved ties, 12,639 parse failures. The specificity
  share did not move, so no construction pair is named. 396 constructions.
- `packed_units=1,384`, emitted with its 1,384 identities listed, REPORT
  provenance only and no term of any partition equation. §F predicted 150-250
  and asked that a wildly different measurement be visible: it is, and the cause
  is definitional rather than a defect. §F's range enumerates the review-named
  defect families, while `packed_units` counts every selected unit carrying at
  least one populated slot, most of which were never in a defect list. The first
  census identity is Aang, Destined Savior — `At the beginning of combat on your
  turn, earthbend 2.` — whose determiner and modifier each keep one alternative
  site over the same qualified reference, an ordinary right-peripheral
  attachment. Phase 3's own landing measured 1,060 packed identities over its
  affected universe alone, so 1,384 is the first whole-corpus figure and is
  consistent with it, and phase 4
  (`english-v2-scope-device-distributive-measure`, live) still packs rather than
  decides the distributive-measure family. Nothing here is fitted to the range.
- Deviations and additions. (a) `inspect` gained the same `packed` block as
  `probe`, which the letter alone names; kept, because it is §E's one xtask
  diagnostic display reached by corpus identity instead of by text, and it is
  how the census identities above were read. (b) The coverage-side census was
  reverted, see above. (c) The ambiguity report schema version moved to 2. (d)
  `deckmaste_english_v2` gained `ParserTrace::selected()`, four lines delegating
  to the existing `ParseAnalysis::selected()`; no computation entered the leaf
  crate and no behaviour changed. (e) Two renderer tests were added at review.
  No grammar, `materialize.rs`, census gate, selection exception, dominance edge,
  specificity tier or named lexical guard anywhere.
- Assurance: 0 restored, 0 re-spelled, 0 ignored, 2 added, 0 removed. Both added
  tests are the review's renderer pair. The implementer added no test function;
  it extended assertions inside two existing ambiguity tests, which stand, and
  inside one coverage test, whose additions went back with their subject — no
  pre-existing assertion was touched, and `coverage.rs` is byte-identical to the
  fork.
- STOPs: one, the gate-scope contradiction recorded in `wlspxyzrvwnw`, resolved
  by the coordinator ruling that CLAUDE.md's reverse-dependency closure rule
  supersedes this ticket's `cargo test --workspace` sentence. None remaining.
- glossary gap: Packing / packed — `docs/contexts/oracle-english/CONTEXT.md`
  defines neither, and this landing prints `packed:` and reports `packed_units`
  and `packed_sites`. glossary gap: Attachment Site Path — the display's
  `site_path`. Both are additions to the ticket's list of Scope, Attachment,
  Head, Premodifier, Peripheral, Bracketing and Mobility; none is coined into
  the tracked glossary here.

### REPORT, re-measured

Provenance, never fitted to and never a gate. All figures below were measured on
the corrected tree at change `wzmonuvryznk`, whose report-mode lock covers
20,002 identities; the schema-version constant landed after the three corpus
passes and reaches no census input.

- Lock `covered` 20,002; constructions 396; permitted licensing checkers 23,
  forbidden 0; longest form literal 11 bytes; 456,485 claims over 1,938,031
  claimed bytes; 0 gap spans, 0 overlap spans, 0 synthetic claims, 0 provenance
  plan mismatches.
- Licensed vocabulary/lexicon homographs, 2, named: vocab
  `AttributiveAdjective::Untap` beside the Verb declaration keyword action
  `Untap`; vocab `TargetingMarker::Target` beside the Noun lexeme
  `CommonNoun::Target`.
- Form-literal/vocabulary overlaps, 9, named as surface at construction form
  atom: `additional` at `additional_cost` atom 2; `to` at
  `up_to_quantifying_determiner` atom 1; `the` at
  `definite_next_mass_quantity_reference` atom 0; `next` at
  `definite_next_mass_quantity_reference` atom 1; `to` at
  `scalar_less_than_or_equal_to` atom 4; `the` at `number_of_scalar_value` atom
  0; `the` at `greatest_scalar_value` atom 0; `other` at
  `other_than_qualified_reference` atom 1; `the` at `positional_partitive` atom
  0.
- `cargo xtask catalogs check` reported `catalogs are up to date`. No citation
  changed, so no cite gate was run.
- Performance advisory, 8 workers each, on a shared host running three
  concurrent codex executors and one other Opus reviewer besides this review
  (the implementer's advisory says the count was unavailable to its sandbox;
  this is the true one), 6 users: coverage 167 s at 212,227 ns/B with load
  25/24/21; ambiguity 243 s at 261,302 ns/B with load 32/30/24; roundtrip 161 s
  at 202,498 ns/B with load 19/27/24. The 16 s quiet-host ceiling is exceeded
  under that load and is reported, not fitted and not gated.

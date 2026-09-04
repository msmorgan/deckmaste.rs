---
needs: []
---
# Demote the six legacy pins to provenance

**R2 — Group R, off-grammar (xtask only; may run beside any grammar ticket).**
Authority: rewrite ADR "Amendment: what a landing proves, discloses, and reports
(2026-09-04)".

Defect. Six numbers designed for the retired corpus-driven plan are enforced as
gates, so the cheapest way past each is to make a grammar addition smaller. They
guard nothing the real invariants do not already guard:

| pin | site | note |
| --- | --- | --- |
| `LICENSED_VOCAB_LEXICON_HOMOGRAPHS == 2` | `crates/xtask/src/english_v2/coverage.rs` const + `--check` | an `!=` equality, so it also fires on a legitimate retirement; the ADR already names the pending third homograph (the Target Verb, `english-v2-target-verb-subject-selection`) |
| `FORM_LITERAL_VOCAB_OVERLAPS_CEILING <= 5` | same file | re-pinned onto the post-landing value (25 → 5), zero headroom |
| builtin noun-morphology census `491 / 165 / 26 / 300` | `crates/xtask/src/english_v2/report.rs`, two `assert_eq!` | adding one general noun the corpus does not attest breaks two tests |
| `roots == 8` in the escape-hatch vector | `report.rs` | root count is grammar shape, not an escape hatch |
| card-name catalog `assert_eq!(names.len(), 32_548)` | `crates/xtask/src/english_v2.rs` | hand-edited on every snapshot refresh |
| parenthetical census `17` / `136` | `crates/xtask/src/english_v2/corpus.rs` | attestation counts for a normalization decision |

Pinned shape. Each becomes a **reported** figure: emitted by `coverage`/`report`
with its named inventory where it has one (the licensed homograph owners, the
surviving overlap surfaces), and never a failure. Keep as gates the real
invariants beside them: the `environment.rs` load errors for an unlicensed
literal/vocabulary collision and for a licensed form literal governing nothing;
the card-name onset-override two-way closure `ensure!`; the noun-morphology
partition identity (`total == derived + explicit + unavailable`) and its
"no unclassifiable surface" error; the seven escape-hatch zeros in the
`report.rs` vector.

Also in scope, because landings treat them as binding and no code computes them:
give the **construction count** one canonical command so records stop disagreeing
(391 / 394 / 397 / 398 have all been quoted), and split the historical
"literal/lexicon collisions: N" figure permanently into its two named fields so
cross-landing comparisons stop being incommensurable.

Fences. Replacing a pin with a looser pin. Deleting an invariant along with its
pin. Adding a new count-shaped gate of any kind.

Baseline, measured on change `oulzkkoqmvuv` — re-measure at claim. Standard
constraints apply; `cargo test --workspace`.

## Landing record (2026-09-04)

Measured on change `mwzxqkyk` (the code tip; the commit carrying this record
changes only this file) after `kata refresh`, with 17,052 covered lock
identities. `DECKMASTE_COVERAGE_LOCK=report cargo xtask english_v2 coverage
--check --workers 8` exited 0 and printed no lock-delta lines: 0 identities
lost, 0 gained, and `english-v2-coverage.lock` is byte-identical to trunk's
(sha256 `ea381cd5ed53faea9be15a43f5f255c6961dabef525faeb9dac114b08dfd6d94`).

PROVE. No silent loss: nothing stopped being covered. Structural laws: 0
selected-uncovered units, 0 unresolved ties, 0 internal failures, 0 roundtrip
mismatches, 0 ownership failures, 713,353 visited constructions against 713,353
nonterminal nodes, 251,662 visited leaves against 251,662 expected, 0 traversal
and 0 leaf-traversal failures, 0 gap and 0 overlap spans, 0 synthetic claims, 0
provenance-plan mismatches. No word-naming: 0 forbidden licensing checkers; the
`environment.rs` load errors for an unlicensed literal/vocabulary collision and
for a licensed form literal governing nothing are untouched and still the only
gate on those surfaces. The other invariants the six demoted pins stood beside
also remain gates: the card-name onset-override two-way closure, the
noun-morphology partition identity with its unclassifiable-surface error, and
the seven escape-hatch zeros.

DISCLOSE. No newly covered identity, so no selected analysis to disclose.
Selection census unchanged: unique 13,453 / specificity-resolved 3,599 before
and after (an xtask-only change; the byte-identical lock and the identical
summary are the proof, and no construction pair moved). Permitted licensing
checkers: 20. Deviations and additions:

- The roots **identity list** assertion in
  `production_report_preserves_escape_hatch_invariants` was demoted alongside
  the ticket's `roots == 8`; the ticket's letter named only the escape-hatch
  vector. The list pinned the same eight roots by name, so a legitimate new
  parser entry point would have failed it — the same defect the ticket
  removes. The roots inventory is reported by `english_v2 report` instead, and
  the test now asserts the inventory is populated.
- `coverage` refuses a disagreement between the named form-literal/vocabulary
  overlap inventory and the environment's census of the same figure (review
  correction; see below). This is a relational identity between two
  derivations, not a pinned count.
- Out of scope, authorized by the coordinator: the `with`-preposition landing
  converted the Exchange keyword-action stub's tail literal to the declared
  `Preposition::With` member but left
  `deckmaste_construction_core/tests/builtin_v2_keyword_actions.rs`'s
  `exchange_has_every_attested_representable_tail_shape` asserting the old
  literal, so `cargo test --workspace` was red on trunk. The test now asserts
  the declared member, which is the correct shape per the frame-selected
  prepositions ruling.

STOP taken and resolved: the implementer stopped on that red workspace suite
rather than integrating around it; the coordinator authorized the repair above,
and `cargo test --workspace` is green.

Glossary gaps: none.

REPORT (provenance; not fitted to, not a gate). Canonical command for the
construction count: `cargo xtask english_v2 report`, which reports 388
construction declarations — every top-level `construction` in the declaration
source, which nests none. (`grep -c construction` over that file returns 391;
three matches are prose, which is where the 391 of earlier records came from.)
The same command reports 32,548 card-name catalog rows, noun morphology
491 / 165 / 26 / 300, 8 roots, and the parenthetical inventory: 5 rules-bearing
surfaces totalling 17 occurrences, 9 reminder-followed-by-text surfaces
totalling 136.

Licensed vocabulary/lexicon homograph owners (2), from `coverage`:

- ``vocab `AttributiveAdjective::Untap` beside Verb declaration keyword action
  `Untap` `` (`plugins/builtin_v2/macros/stubs/keyword_actions/Untap.ron`)
- ``vocab `TargetingMarker::Target` beside Noun lexeme CommonNoun::Target``

Form-literal/vocabulary overlap surfaces (5), from `coverage`:

- surface `additional` at construction `additional_cost` form `additional_cost` atom 2
- surface `to` at construction `up_to_quantifying_determiner` form `up_to_quantifying_determiner` atom 1
- surface `next` at construction `definite_next_mass_quantity_reference` form `definite_next_mass_quantity_reference` atom 1
- surface `to` at construction `scalar_less_than_or_equal_to` form `scalar_less_than_or_equal_to` atom 4
- surface `other` at construction `other_than_qualified_reference` form `other_than_qualified_reference` atom 1

Performance advisory (wall times as integer seconds; a three-digit decimal
reads as a rule number to the cite checker). `coverage --check --workers 8`:
101 s wall against the 16.26 s quiet-host ceiling, 114,190 ns/B thread CPU, host load
5.16 / 6.50 / 7.98. `ambiguity --require-resolved --workers 8`: 101 s,
120,019 ns/B, host load 4.83 / 6.06 / 7.62. Contention during these runs: 2
concurrent executors (english-v2-attachment-class-declared,
english-v2-clause-level-duration) plus this review — the ceiling comparison is
advisory at that load, not a breach claim. The implementer's own pre-review run
measured 118 s and 140,493 ns/B at host load 13.08 / 11.39 / 9.26.

Assurance counts: 0 restored, 4 re-spelled (the parenthetical census, the
collision census, the noun-morphology census, and the card-name row count that
lived as an assertion inside the onset-override inventory test), 0 ignored with
blockers, 1 added (the declaration-provenance fixture), 0 removed.

### Review corrections

- MEDIUM — the named form-literal/vocabulary overlap inventory was an
  independent xtask re-derivation of a figure the environment already computes,
  with nothing binding the two; the rewrite ADR requires that count to be
  reported *as* a named inventory. `coverage` now fails if the inventory and
  the environment census disagree, and both provenance fields document what
  they count. (They agree at 5 on this tree.)
- MEDIUM — the roots identity-list demotion was undisclosed and its replacement
  assertion was vacuous. Disclosed above; the assertion now also requires the
  inventory to be non-empty.
- MEDIUM — the record reported the homograph and overlap inventories as bare
  numbers with a pointer to command output. Both are now listed by name here,
  as the ADR requires.
- MEDIUM — the delivered record failed `cargo xtask cite check
  --list-noncompliant`: a wall time written as `117.704 s` reads as a CR rule
  number. Wall times are integer seconds here; the checker is clean.
- LOW — the canonical construction count had no one-line definition. Both the
  `DeclarationProvenance` and `CountedReport` fields now carry one, and the
  388-vs-391 discrepancy is explained above.
- Noted, not fixed: `validate_noun_morphology_census` cannot fire — the census
  loop increments the total and exactly one bucket per surface, so the
  partition holds by construction and its test proves the validator only
  against a hand-built impossible census. The ticket names that error as an
  invariant to keep, so it stays; a future change that widens the classifier
  is what makes it live.
- Noted, not fixed: `docs/decisions/english-v2-rewrite.md` still describes the
  homograph census as "both gated by `english_v2 coverage --check` ... pinned
  exactly" in the 2026-09-02 ruling text; the 2026-09-04 amendment above it
  supersedes that and names this ticket as the demotion. Editing the
  superseded passage is a coordinator call, not a landing's.

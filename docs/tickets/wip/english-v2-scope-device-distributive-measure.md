---
needs: [english-v2-scope-device-collapse]
---
# The scope device, phase 4: a distributive measure attaches to the Predicate

**B7 phase 4.** Principle (iv) of the ordered principle set, the last
elimination the device needs. A Prepositional Phrase whose Complement's
Determiner declares distributive quantification is a measure over the Predicate
— a multiplier — and has no Nominal-Postmodifier derivation.

Design: `docs/memory/scratch/plan09-postmortem/b7-scope-device-design.md`
(gitignored), §B principle (iv), OPEN-2 and §G.2 phase 4. The Q1-Q6 rulings, the
OPEN rulings and the routed residues live on
`english-v2-underspecified-adjunct-attachment` (phase 1) as inherited context.

Runs after the collapse so the corpus movement it causes is measured against a
tree that already packs: an elimination and a collapse landing together cannot
be told apart in the census.

## Letter

- A new sealed feature domain — a **quantification axis** — on the Determiner
  vocabulary, distinguishing distributive quantification from everything else.
  OPEN-2 is ruled (2026-09-04): the Determiner vocabulary carries the axis. The
  Prepositional-Complement side is **refused** — it re-creates the
  per-preposition switch the fallout audit recorded as F3.
- One requirement at the Nominal-Postmodifier site, reading that declared axis
  through the generated accessor. One general rule over declared features: never
  a per-preposition switch, never a list of Complement kinds, never a named
  Determiner.

≈80 lines plus witnesses (§G.2).

## Acceptance witnesses (§G.5)

Decided, not packed:

- `Draw a card for each Island you control.` — the measure attaches to the
  Predicate; the Nominal-Postmodifier derivation does not exist.
- `Draw a card for each creature you control.` — same.
- the two `put … on … for each …` units (Animal Friend, General Leo Cristophe),
  one of which today applies a restrictive Postmodifier to a rigid self-name
  with no head noun to restrict.

Expected disposition from §F, provenance and not a target: 153 R1
distributive-measure misselections (classes A, D, E) return to the Predicate
multiplier, plus the 2 degradations above.

Negative:

- the negative probes restored by R1's review still reject;
- `unresolved_ties == 0`;
- byte-exact roundtrip.

## Fences (§G.4)

- No `require`, `checked by`, comment or literal naming a lexeme, Construction,
  verb, noun, preposition or card. A permitted guard reads a declared feature or
  a declared role property.
- No `SELECTION_EXCEPTIONS` entry, no dominance edge between named
  Constructions, no new specificity weight or tier. This is an elimination, not
  an ordering: expressing it as a preference weight is the F12 defect.
- No new AST Category, and no field holding an alternative subtree.
- Attestation is provenance: the axis's values answer "which English fact
  excludes the missing values?", never a witness count.
- No `run_in_background` on a gate; report positive artifacts.

Standard constraints apply. Gate scope: `cargo test --workspace`.

## Baseline

Measured on `lpvvplmyynul`, phase 1's base — re-measure at claim (phase 3 will
have moved every corpus figure). Lock `covered` 17,601; constructions 387;
32,641 units; census 13,759 unique / 3,842 specificity-resolved / 0
exception-resolved / 15,040 parse failures / 0 unresolved ties.

## Glossary gaps

Terms the design needs that `docs/contexts/oracle-english/CONTEXT.md` does not
define: Scope, Attachment, Head, Premodifier, Peripheral, Bracketing, Mobility.
Listed as gaps; none is coined into the tracked glossary by this ticket.

## Landing record

Measured on feature change `xzxxqoztlqkxunxvqnxlnnyyosttnqtm` at lock
`covered` 20,002, against fork point `mrvowqkvwlnyskozzvuyutyxmvrqyols`. Wall
clock: 2026-09-05 10:51 PDT to 2026-09-05 12:18 PDT. **That tree is superseded:**
review refreshed the feature onto the scope-device-census landing before gating,
which moved every corpus figure. The change ids are unchanged by the rebase, so
`Review corrections` below carries the landed-tree numbers at lock `covered`
20,054 and supersedes every figure in this section that it restates.

### What landed and why

- Declaration Determinatives now carry a sealed `Quantification` feature with
  `NonDistributive` and `Distributive` values. The declaration parser,
  validator, semantic plan, generated AST/build/scan/render/visit boundaries,
  and zeroable Determiner default all carry that feature structurally. Only the
  distributive vocabulary member declares `Distributive`; every other member
  declares `NonDistributive`.
- The existing Determinative-to-Prepositional-Phrase feature chain relays the
  axis through generated category helpers. The Nominal-Postmodifier checker
  reads that helper and refuses a distributive Complement. The existing
  Predicate-site checker admits the same declared distributive value, so the
  measure is realized through the phase 1--3 mobile role and admissible-site
  machinery rather than a new syntax category or selection preference.
- No AST Category, construction, selection exception, specificity tier,
  dominance edge, per-preposition switch, census gate, or alternative-subtree
  field was added. `parser/materialize.rs` production behavior is unchanged;
  its new phase-4 witness test observes the attachment relation and retained
  mobile role without dispatching on host construction identity. The phase-3
  representative/nameable-host laws remain intact.
- Files touched are the 16 construction-core feature/compiler sources under
  `crates/deckmaste_construction_core/src`,
  `crates/deckmaste_english_v2/src/constructions.rs`,
  `crates/deckmaste_english_v2/src/parser/materialize.rs`, and this ticket.

### PROVE

- `cargo fmt --all` completed. The changed-path gate printed exactly
  `cargo test -p deckmaste_construction_core -p deckmaste_construction -p deckmaste_english_v2 -p xtask`
  and the matching strict-clippy line. Principal positive artifacts were
  `test result: ok. 44 passed`, `test result: ok. 422 passed`,
  `test result: ok. 157 passed`, `test result: ok. 49 passed`,
  `test result: ok. 32 passed`, `test result: ok. 112 passed`, and
  `test result: ok. 468 passed; 1 ignored`; every other emitted suite also
  reported `test result: ok`. Strict clippy completed with `-D warnings`.
- With `DECKMASTE_COVERAGE_LOCK=report`, both `coverage --check --workers 8`
  and `coverage --bless --workers 8` reported 32,641 total, 20,002 selected and
  covered, 0 selected-uncovered, 12,639 parse failures, 0 unresolved ties, 0
  internal failures, 0 exception uses, 0 roundtrip mismatches, and 0 ownership
  failures. The report-mode lock delta was `+0/-0`; bless produced no
  coverage-lock file diff. No identity stopped being covered, no identity was
  newly covered, and therefore no wrong analysis or negative oracle became
  covered.
- Exact fork/feature ambiguity measurements both used
  `--json --require-resolved --workers 8`. The fork reported 20,002 selected,
  16,631 unique, 3,371 specificity-resolved, 0 exception-resolved, 0 unresolved
  ties, and 12,639 parse failures. The feature reported 20,002 selected, 16,632
  unique, 3,370 specificity-resolved, 0 exception-resolved, 0 unresolved ties,
  and 12,639 parse failures. The temporary fork working copy was abandoned
  automatically after returning to the feature change.
- `roundtrip --require-clean --workers 8` reported 20,002 accepted, 20,002
  clean, 0 mismatched, and 12,639 not accepted. Coverage traversal was balanced:
  888,877 nonterminal nodes and visited constructions, 310,547 expected and
  visited leaves, 0 traversal failures, 0 gap or overlap spans, 0 synthetic
  claims, and 0 provenance-plan mismatches. `cargo xtask catalogs check`
  reported `catalogs are up to date`. No citation-bearing source changed, so no
  cite gate was applicable.
- Phase-4 witnesses `Draw a card for each Island you control.`, `Draw a card
  for each creature you control.`, Animal Friend, and General Leo Cristophe
  have only the Predicate attachment before scope collapse, retain the declared
  `adjunct` mobile role, and render byte-exactly. The added `Double the number
  of +1/+1 counters on each creature you control.` regression witness proves
  the same for the affected OnComplement family. The closure reran every
  phase-1--3 materialization witness and fence; their asserted attachment sites,
  packing, selected paths, leaf traversal, and rendered bytes did not move.
- Production coverage emitted 23 permitted licensing checkers and 0 forbidden
  checkers. The closure's production census test for word-naming checkers
  passed. The new condition compares only the declared feature value; no guard,
  comment, or branch names a word, lexeme, construction, preposition, or card.

### DISCLOSE

- Stable-identity comparison found exactly four changed selected paths. All
  four are **correct** semantic corrections from a
  `PrepositionalQualifiedReference` Nominal postmodifier to a
  `PrepositionalPredicateAdjunctPredicate` Predicate attachment; 0 are merely
  correct-but-recanonicalized, 0 are misselections, and 0 are wrong analyses:
  - `364ea613fadd7a4311aaa2b9a8f3c2b04ea7f4c56ad8a24bfd618c4a968ba44e`
    — Bristly Bill, Spine Sower; unique to unique.
  - `4674784e7169c846af1f52d487825e7a2835fd4485abf77c5cfd480b39c695b3`
    — Hulk, Strongest There Is; specificity-resolved to
    specificity-resolved.
  - `4aa5d66f3b1d0dad4f83e4628a6924f3a4cf46cc4ded06b53798a3d766d748c8`
    — Kalonian Hydra; unique to unique.
  - `a57f7c56e1477cba10157835e7c5591fae7acd249cfcc6d8ebac8ba9bcfb45f7`
    — The Three Seasons; specificity-resolved to unique.
- The first refreshed full ambiguity pass exposed three selected-to-failure
  regressions: Bristly Bill, Spine Sower; Hulk, Strongest There Is; and Kalonian
  Hydra. The existing Predicate-site license had excluded their distributive
  OnComplement measure. The generic declared-feature admission and the added
  regression witness resolved all three; the replacement exact comparison
  above has no selected loss. This was a failed gate followed by a code change,
  so the replacement full pass was authorized. A later behavior-preserving
  clippy refactor changed source, so the final matched pair was taken again on
  the exact recorded tree.
- The changed-path gate needed correction rounds. Its first run passed all tests
  and found `too_many_lines` in the extended Determinative validator; a helper
  extraction fixed it. Its second run passed all tests and found
  `too_many_arguments` in the extended Nominal-site checker; reading the
  generated category-feature helper from the already supplied checked value
  restored the original arity. An intermediate gate start then caught an
  incorrect guessed method name before tests; a targeted `cargo check` verified
  the generated free-helper name. The final complete changed-path test and
  clippy gate is green. No lint suppression was added.
- Deviations and additions: the Predicate-site checker now reads the declared
  axis in addition to the ticket-letter Nominal-site elimination. This was
  required to preserve the three correct distributive OnComplement analyses
  surfaced by the first full pass and is the positive half of principle (iv),
  not an ordering rule. The extra synthetic OnComplement row in the phase-4
  witness test is beyond the named §G.5 set and permanently guards that
  regression. No construction or test was deleted or ignored.
- Assurance: 0 restored, 13 re-spelled, 0 ignored, 2 added, 0 removed. The
  re-spellings add required declaration metadata to compiler fixtures and
  advance their exact generated-inventory assertions; one existing accessor
  test was sharpened to prove the new sealed axis. The two additions reject
  missing declaration metadata and prove the phase-4 attachment witnesses.
- The feature workspace was refreshed before final verification. The harmony
  helper reported a clean workspace and no textual conflict; concurrent changes
  were preserved. No STOP condition remains.
- Glossary gaps: Scope, Attachment, Head, Premodifier, Peripheral, Bracketing,
  and Mobility remain absent from the routed Oracle-English glossary. This
  ticket does not coin replacements.

### REPORT

- Inventory pins on `xzxxqoztlqkxunxvqnxlnnyyosttnqtm`, lock `covered`
  20,002: 397 constructions, 23 permitted licensing checkers, 0 forbidden
  checkers, 2 licensed vocabulary/lexicon homographs, 9 form-literal/vocabulary
  overlaps, and longest form literal 11 bytes. This ticket adds 0 constructions.
- Licensed homographs: `AttributiveAdjective::Untap` beside the declaration
  keyword action of the same surface, and `TargetingMarker::Target` beside
  `CommonNoun::Target`.
- Form-literal/vocabulary overlaps: `additional` in `additional_cost`; `to` in
  `up_to_quantifying_determiner`; `the` and `next` in
  `definite_next_mass_quantity_reference`; `to` in
  `scalar_less_than_or_equal_to`; `the` in `number_of_scalar_value`; `the` in
  `greatest_scalar_value`; `other` in `other_than_qualified_reference`; and
  `the` in `positional_partitive`.
- OPEN-4 performance advisory, exact refreshed-tree pair run back-to-back with
  8 workers: fork 138 s at 185,482 ns/B and host load 15/22/23; feature 189 s
  at 245,400 ns/B and host load 26/25/24. Launch-side cargo/rustc process counts
  were 11 and 2 respectively; the shared host was contended. The 16 s
  quiet-host ceiling was exceeded and is reported, not gated.
- Independent final report-mode coverage advisories, 8 workers: check 138 s at
  161,820 ns/B and host load 24/24/21; bless 261 s at 276,839 ns/B and host
  load 30/30/24. Final roundtrip was 137 s at 173,450 ns/B and host load
  19/26/24. These figures are REPORT-only and were not fitted to.

### Review corrections

Reviewed and gated on the refreshed tree: feature change
`xzxxqoztlqkxunxvqnxlnnyyosttnqtm` (rebased onto the scope-device-census
landing) at lock `covered` 20,054, against fork point
`mrvowqkvwlnyskozzvuyutyxmvrqyols` at the same lock. Both sides of every
before/after pair were measured in this workspace with identical flags
(`--json --require-resolved --workers 8`); the fork side was a temporary
`jj new` working copy, abandoned afterwards. No code finding was raised, so no
code change was made; the corrections below are to the record.

Findings and fixes:

- MEDIUM — the record was stamped on the pre-refresh tree. Restamped: the
  landed-tree figures are in this section, and the header now says so.
- MEDIUM — the contention stamp reported launch-side `cargo`/`rustc` process
  counts, which a sandboxed `pgrep` cannot see past. Replaced with the true
  count below.
- MEDIUM — the glossary-gap list omitted the terms this landing itself coins.
  `Quantification`, `Distributive` and `NonDistributive` are added to it below;
  `docs/contexts/oracle-english/CONTEXT.md` defines `Determinative` ("marks a
  Noun Phrase as definite, quantified, or otherwise determined") but has no
  entry for the quantification axis or its values. Listed as a gap; nothing is
  coined into the tracked glossary by this ticket.
- MEDIUM — the record ran the closure gate without citing the ruling that
  authorizes it. The Fences line "Gate scope: `cargo test --workspace`" is the
  same legacy sentence the coordinator ruled on for this family on 2026-09-05
  (recorded in `docs/tickets/done/english-v2-scope-device-census.md`): it is
  superseded by the tracked reverse-dependency closure rule. Cited here; the
  contradiction is ruled, not resolved by this landing.
- LOW, observed and not fixed — the "absent value" for a zeroable feature is
  now spelled twice, in `crates/deckmaste_construction_core/src/emit/ast.rs`
  and `crates/deckmaste_construction_core/src/emit/render.rs`, each with an
  `unreachable!` arm that a third zeroable feature would reach at codegen time
  rather than failing to compile. The duplication is the pre-existing shape
  (`BareDurationLicense` was already hardcoded in both); unifying it is
  emitter churn better spent when a third zeroable feature arrives.
- LOW, disclosed — the sharpened accessor re-spelling retired the test name
  `checked_fields_accept_feature_bearing_lexical_values`. Its subject (a
  build-only `checked by` on a feature-bearing lexical field) is preserved and
  strengthened inside
  `declaration_determinative_quantification_flows_through_the_generated_accessor`,
  which runs `generate` rather than `validate`. It is a re-spelling, not a
  removal; the assurance counts stand.

Landed-tree PROVE:

- `cargo fmt --all -- --check` clean. `cargo xtask gate --changed --clippy`
  printed exactly
  `cargo test -p deckmaste_construction_core -p deckmaste_construction -p deckmaste_english_v2 -p xtask`
  and the matching strict-clippy line; both ran green. Principal positive
  artifacts: `test result: ok. 44 passed`, `test result: ok. 424 passed`,
  `test result: ok. 158 passed`, `test result: ok. 49 passed`,
  `test result: ok. 32 passed`, `test result: ok. 112 passed`, and
  `test result: ok. 470 passed; 1 ignored` — the single ignored test is
  pre-existing and this diff adds no `#[ignore]`. Strict clippy finished with
  `-D warnings` and no diagnostic. The `environment.rs` load invariants and the
  production word-naming census run inside that closure and passed.
- `DECKMASTE_COVERAGE_LOCK=report cargo xtask english_v2 coverage --check
  --workers 8`: 32,641 total, 20,054 selected and 20,054 covered, 0
  selected-uncovered, 12,587 parse failures, 0 unresolved ties, 0 internal
  failures, 0 exception uses, 0 roundtrip mismatches, 0 ownership failures.
  The tracked lock's own `covered` is 20,054 and the working copy stayed
  clean, so the covered SET is unchanged: lock delta `+0/-0`, no identity
  newly covered and none dropped, and no bless was needed.
- Structural laws: 890,919 nonterminal nodes and 890,919 visited
  constructions, 311,251 expected and 311,251 visited leaves, 0 traversal and
  0 leaf-traversal failures, 0 gap and 0 overlap spans, 0 synthetic claims, 0
  provenance-plan mismatches. `roundtrip --require-clean --workers 8`: 20,054
  parse accepted, 20,054 clean, 0 mismatched, 12,587 not accepted.
  `cargo xtask catalogs check` reported `catalogs are up to date`. No
  citation-bearing source changed, so the cite gates are not applicable.
- No word-naming: 23 permitted licensing checkers, 0 forbidden. The two
  checkers this landing touched read only declared feature values —
  `nominal_nonrelational_preposition_is_licensed` compares the generated
  category helper against `Quantification::Distributive`, and
  `predicate_preposition_is_licensed` takes the declared value as a parameter.
  Neither names a lexeme, construction, verb, noun, preposition or card, and
  no comment or literal in the diff does either.

Landed-tree DISCLOSE:

- Selection census, matched flags: fork 20,054 selected / 16,680 unique /
  3,374 specificity-resolved / 0 exception-resolved / 0 unresolved ties /
  12,587 parse failures; feature 20,054 / 16,681 / 3,373 / 0 / 0 / 12,587. The
  specificity share fell by one unit, so no construction pair needs naming.
- Per-unit stable-identity comparison: 0 units went selected-to-failure, 0
  went failure-to-selected, and exactly four selected paths changed — the same
  four unit identities the implementer recorded. Each drops
  `PostmodifiedReferencePrepositionalQualifiedReference` and gains
  `PredicateAdjunctPredicatePrepositionalPredicateAdjunctPredicate` with
  `PredicateAdjunctPrepositionalPredicateAdjunct`; all four are correct
  corrections, read against the printed Oracle text:
  - Bristly Bill, Spine Sower (unique to unique) and Kalonian Hydra (unique to
    unique) — "double the number of +1/+1 counters on each creature you
    control"; the distributive measure multiplies the doubling per creature,
    which the Nominal reading (one summed count) could not express.
  - Hulk, Strongest There Is (specificity to specificity) — the same shape
    over "each Gamma creature you control".
  - The Three Seasons (specificity to unique) — "Choose three cards in each
    graveyard"; three per graveyard, not three cards standing in every
    graveyard.
- No surface lost the Nominal attachment it needed: the elimination is a
  declared-feature site rule, no construction arm was deleted, and the
  zero-loss per-unit comparison above is the corpus proof.
- Witnesses: the phase-4 test asserts that every raw derivation of "Draw a card
  for each Island you control.", "Draw a card for each creature you control.",
  the two `put ... for each ...` units, and the OnComplement regression row
  keeps the `adjunct` mobile and renders byte-exactly. The phase-3 witness
  `opacity_preserves_adjunct_and_frame_internal_scope_packing` is untouched by
  this diff and green in the closure gate: it still asserts one derivation for
  "Draw a card for each Island you control." with the Adjunct mobile intact.
- Deviations and additions are as recorded above and unchanged by review; no
  construction, test or gate was added or deleted during review.
- Glossary gaps: Scope, Attachment, Head, Premodifier, Peripheral, Bracketing,
  Mobility, and — added by review — Quantification, Distributive,
  NonDistributive.

Landed-tree REPORT (provenance, never fitted to):

- Lock `covered` 20,054; 397 constructions, of which this ticket adds 0; 23
  permitted licensing checkers, 0 forbidden; 2 licensed vocabulary/lexicon
  homographs; 9 form-literal/vocabulary overlaps; longest form literal 11
  bytes. The homograph and overlap inventories are exactly the named lists
  already recorded above and did not move.
- Performance advisory, 8 workers, 24-core shared host, true contention 1
  concurrent codex executor and 1 other Opus reviewer (the coordinator's
  count; the implementer's launch-side process counts are a sandboxed-`pgrep`
  artifact and are withdrawn): coverage check 117 s at 139,581 ns/B, host load
  6/9/14; ambiguity on the feature 115 s at 139,814 ns/B, host load 4/7/12;
  ambiguity on the fork 116 s at 146,662 ns/B, host load 4/6/11; roundtrip
  109 s at 133,501 ns/B, host load 4/5/10. The 16 s quiet-host ceiling was
  exceeded on every pass and is reported, not gated.
- Trunk advanced once more after these gates with a ticket-only change
  (`tickets: extend cross-host gates with the R7 witness-shape and
  coordination-family items`). It reaches no crate and no declaration data, so
  nothing was re-run for it.

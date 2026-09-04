---
needs: [english-v2-require-through-optional-role]
---
**Give lexical schemas, instantiated phrases, and evaluation contexts distinct
names.** Use Oracle English [`Verb Frame`, `Lexical Verb Phrase`, `Verb Phrase`,
and `Complement`](../../contexts/oracle-english/CONTEXT.md) alongside Game Model
[`Execution Frame`](../../contexts/game-model/CONTEXT.md). A Verb Frame is the lexical schema
that licenses ordered complements and fixed markers; a Lexical Verb Phrase is
an instantiation of that schema; an Execution Frame is the engine evaluation
context. Do not use `valency` as a catch-all for all three.

Apply this mapping after the optional-role WIP lands:

- rename the lexeme-owned `VerbValence`, whose custom case contains a set of
  ordered shapes, to `VerbFrameSet`; each member shape is one `VerbFrame`;
- keep or rename normalized `VerbFrameKey` explicitly as a compiler
  compatibility key, not as the lexical schema itself;
- rename realized AST `BaseVerbFrame` values to `LexicalVerbPhrase`;
- qualify engine evaluation-context `Frame` values as `ExecutionFrame`; and
- rename the current `Numerative` role to `MeasureComplement`.

Apply the mapping through English-v2/core/data types, constructors, providers,
diagnostics, and tests. Append dated superseding amendments to the affected
verb-valence sections of the builtin-v2 grammar and English-v2 rewrite
decisions; retain their historical text. Prefer the short `VerbFrame` family
over `VerbSubcategorization` or `ComplementationPattern`.

Acceptance makes schema-versus-instance types unambiguous at their API seams
and leaves generic `Frame` only where a qualification adds no information.

## Landing record

Measured after the required final refresh on the tree ending at change
`uqluoopr`, and re-measured by the reviewer after a second refresh that took in
the `effect-instruction-taxonomy` landing, on the tree ending at `wtyspyqp`.
Every figure below held across both. The vocabulary change is `qnnmmtzz`. The
frozen coverage lock has 49,421 lines and SHA-256
`2cf7f9b716e13b27d626c60638d1266ca6cdc083bbcca87cb1435b4b00cd65ca`, with
`covered` 16,771 on both trees.

- Vocabulary: lexeme declarations now expose `VerbFrameSet`, whose custom
  member list contains `VerbFrame` values. `VerbFrameKey` remains explicitly
  the compiler compatibility key between a declared Verb Frame and a realized
  Lexical Verb Phrase. The realized AST family uses `LexicalVerbPhrase`, the
  amount role uses `MeasureComplement`, and engine evaluation contexts use
  `ExecutionFrame`. Serialized declarations, generated constructors,
  providers, diagnostics, fixtures, and existing tests use the same names.
- Decisions: dated 2026-09-04 superseding amendments were appended to the
  builtin-v2 grammar and English-v2 rewrite decisions. Their historical text
  and historical vocabulary were retained.
- Coverage: 16,771 selected and covered units before and after; lock movement
  **+0/-0 rows** and byte movement **+0/-0**. Parse failures remain 15,870;
  selected-uncovered units, unresolved ties, internal failures, exception
  uses, round-trip mismatches, ownership failures, traversal failures, gaps,
  overlaps, synthetic claims, and provenance-plan mismatches remain zero.
  Newly covered identities: none, so there are no new selected analyses to
  enumerate.
- Selection census: 11,515 unique and 5,256 specificity-resolved selections
  before and after; exception-resolved selections and unresolved ties remain
  zero.
- Licensing checkers: 26 permitted and 5 forbidden identities, unchanged by
  this landing.
- Structural census: the ticket baseline had 397 construction declarations;
  the final refreshed parent and this feature both have 395. The intervening
  -2 belongs to the closed-class sibling landing; direct parent/feature
  measurement is **395 -> 395**, so this rename added or deleted no
  construction and did not change any `checked by` or `require` clause.
- Performance advisory: coverage took 27.204593310 s at 121,693 ns/B with
  host load 13.24/17.38/20.72; ambiguity took 27.032779599 s at 120,913 ns/B
  with host load 14.53/17.26/20.55. Each ran as one foreground gate process
  with 24 workers; sibling-process visibility is unavailable in the sandbox.
  Both exceeded the 16.26 s quiet-host ceiling while the host was loaded.
  Contention stamp (supplied by the reviewer, who can see the host): 4-6
  concurrent codex executors ran throughout these measurements, so neither
  figure is a quiet-host reading and neither is evidence of a regression.
  The reviewer re-measured on the corrected tree under the same contention:
  coverage 23.720765809 s at 113,266 ns/B with host load 31.29/25.11/20.48
  (a first run under heavier load read 68.292599007 s at 197,605 ns/B with
  host load 39.12/25.25/20.31), and ambiguity 26.103405518 s at 126,342 ns/B
  with host load 29.63/25.40/20.74. The 3x spread between two runs of the
  same command on the same tree is the contention, not the parser. After the
  second refresh the host was busier still: coverage 86.490157276 s at 219,727
  ns/B with host load 64.75/57.31/54.91, and ambiguity 3 m 55.92 s at 258,272
  ns/B with host load 86.54/76.43/63.39. After the third refresh: coverage
  95.443244134 s at 136,905 ns/B with host load 57.62/69.90/69.09, and
  ambiguity 90.468896018 s at 155,212 ns/B with host load 45.15/62.80/66.65.
  Seven readings of the same two commands over one unchanged parser span
  under 24 s to nearly four minutes, which is the measurement this host can
  offer, not a parse-time signal.
- Gates after refresh: `cargo fmt --all`; strict all-target Clippy for
  `deckmaste_construction_core`, `deckmaste_construction`, `deckmaste_engine`,
  `deckmaste_english_v2`, `xtask`, and the two gate-surfaced fixture crates
  `deckmaste_migrations` and `deckmaste_noncanon`; `cargo test --workspace`;
  `cargo xtask map enums crates/deckmaste_construction_core/src`; `cargo xtask
  english_v2 coverage --check`; `cargo xtask english_v2 ambiguity
  --require-resolved`; `cargo xtask cite check --list-noncompliant`; and
  `cargo xtask cite check` exited zero. Citation checks reported 0
  non-compliant strings and 0 stale among 18,102 citations. The corpus gates
  emitted only the loaded-host performance advisory.
- Refresh: harmony first resolved an overlapping import by retaining the
  refreshed parent's `DecisionPointKind` and this feature's `ExecutionFrame`.
  After the closed-class sibling landed, harmony retained its
  `licensed("for")` ownership annotation and specificity changes while applying
  `LexicalVerbPhrase` to the affected construction and diagnostic paths; both
  dated decision amendments were retained. The visitor-leaf sibling landing
  and the number-feature sibling's claim marker then refreshed in without
  conflicts; the number-feature sibling itself is still unintegrated, so none
  of its work is in this stack. The final conflict listing is empty, and every
  gate was rerun after the last changed refresh; the leaf census is 235,683
  expected and 235,683 visited with zero failures.
- Assurance census: restored 0; re-spelled 9 existing test names, with their
  affected assertions and fixtures migrated in place; ignored 0; added 0;
  removed 0. The `#[test]` total across the touched files is 1,307 before and
  1,307 after. The nine are
  `declaration_verb_valence_perturbation_moves_frame_availability`,
  `custom_valence_has_exact_finite_atom_shapes`,
  `declaration_verb_valences_require_a_construction_consumer`,
  `core_verb_declarations_have_exact_surfaces_and_deduplicated_valences`,
  `combat_frames_keep_active_valence_passive_agents_and_if_able_distinct`,
  `exchange_uses_declared_object_valence_and_a_typed_control_reference`,
  `declared_custom_valences_select_one_frame_per_linguistic_shape`,
  `shared_transitive_frame_preserves_visit_order_and_literal_claims`, and
  `object_gap_adjunct_attachment_follows_the_declared_valence_row`.
- STOPs: none. Glossary gaps: none. Decision wanted: none.
- Trunk state noted, not caused here: after the sibling landings refreshed in,
  `cargo xtask cite check --list-noncompliant` reports two loose rule numbers
  in `docs/tickets/done/ability-kind-taxonomy.md` (lines 165 and 168). This
  landing changes no citation and contributes none of them; `cargo xtask cite
  check` is 0 stale among 18,209. Repairing another landing's done-record
  prose is outside this ticket, so it is reported rather than edited here.

### Deviations and additions

- None from the ticket's letter. Intermediate refreshed-parent fixture failures
  were superseded by the final parent before measurement and do not remain in
  this feature diff.
- One incidental line: the mandated `cargo fmt --all` deleted a trailing blank
  line in `crates/deckmaste_lowering/tests/diagnostics.rs`, which a sibling
  landing left unformatted on the default line. It rides along rather than
  leaving the tree fmt-dirty.

### Review corrections

Applied by the landing reviewer on top of `uqluoopr`; gates rerun after them.

- Assurance count understated: the record said 7 re-spelled test names; the
  diff re-spells 9. Corrected, and each one is now named.
- Landing record omitted the permitted licensing-checker total the record
  contract requires. Added: 26 permitted, 5 forbidden.
- Refresh note claimed the number-feature sibling "refreshed in". Only its
  claim marker is on the default line; the sibling is unintegrated. Wording
  corrected. A whole-diff check confirms no Number sequence-propagation code,
  no collapsed singular/plural category, and no homogeneous-Number decision
  text entered this stack.
- Performance advisory carried no contention stamp and its "sibling-process
  visibility is unavailable" line left the reader without one. Stamped with
  the true concurrency and re-measured.
- `plan.rs`: the re-spelled test imported `VerbFrameSet as Set`, which erased
  the vocabulary the ticket buys and read against `HashSet` in the same file.
  Replaced with three named bindings (`intransitive`, `transitive`,
  `measure_complement`) over the full type name, which also keeps the test
  under the `clippy::too_many_lines` ceiling the alias was working around.
- `semantic.rs`, `emit/runtime.rs`: the new `VerbFrameKey` doc comment sat
  below `#[derive(...)]`, against the file convention. Moved above it.
- `emit/ast.rs`: the assertion message read "never frame or frame_set tags"
  after the mechanical substitution. Re-spelled as "never Verb Frame or Verb
  Frame Set tags".
- Six engine doc comments read "a `ExecutionFrame`" after the rename lengthened
  the noun. Corrected to "an".
- Second refresh: the `effect-instruction-taxonomy` landing
  (`OneShotEffect` -> `Instruction`, `StaticEffect` -> `StaticSpec`) reached
  the default line mid-review and collided with this rename on 18 lines across
  nine engine files (`agenda`, `cast`, `event`, `legal`, `payment`,
  `resolve/action`, `resolve/effect`, `sba`, `trigger`). Every one is two
  independent renames of the same signature, so each was resolved by taking
  the incoming side and re-applying `Frame` -> `ExecutionFrame` to it; no
  behaviour was chosen between. All gates were rerun afterwards: `cargo fmt
  --all`; strict all-target Clippy for the five touched crates; `cargo test
  --workspace` (127 suites, 6,089 passed, 0 failed, 6 pre-existing ignored);
  `cargo xtask english_v2 coverage --check`; `cargo xtask english_v2 ambiguity
  --require-resolved`; `cargo xtask cite check --list-noncompliant` (0) and
  `cargo xtask cite check` (0 stale among 18,165). Construction declarations
  are 395 on the new parent and 395 here.
- Third refresh: `type-def-permanent-type-flag` and
  `workbench-damage-recipient-disjunction` landed next and refreshed in without
  conflicts. All gates rerun on that tree: fmt; strict Clippy for the five
  crates; `cargo test --workspace` (127 suites, 6,091 passed, 0 failed, 6
  pre-existing ignored); coverage `--check` and ambiguity
  `--require-resolved` both exit zero with every figure above unchanged; cite
  0 non-compliant and 0 stale among 18,181; constructions 395 -> 395.

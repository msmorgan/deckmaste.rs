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

Measured before the required final refresh on change `qnnmmtzz`. The frozen
coverage lock has 49,421 lines and SHA-256
`2cf7f9b716e13b27d626c60638d1266ca6cdc083bbcca87cb1435b4b00cd65ca`.

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
- Structural census: 397 construction declarations before and after. No
  construction, `checked by` clause, or `require` clause was added, deleted,
  or semantically changed.
- Performance advisory: coverage took 39.367628832 s at 134,942 ns/B with
  host load 36.60/39.12/32.16; ambiguity took 37.726121501 s at 141,434 ns/B
  with host load 34.97/38.49/32.25. Each ran as one foreground gate process
  with 24 workers; sibling-process visibility is unavailable in the sandbox.
  Both exceeded the 16.26 s quiet-host ceiling while the host was loaded.
- Gates before refresh: `cargo fmt --all`; strict all-target Clippy for
  `deckmaste_construction_core`, `deckmaste_construction`, `deckmaste_engine`,
  `deckmaste_english_v2`, and `xtask`; `cargo xtask map enums
  crates/deckmaste_english_v2/src`; `cargo xtask english_v2 coverage --check`;
  `cargo xtask english_v2 ambiguity --require-resolved`; `cargo xtask cite
  check --list-noncompliant`; and `cargo xtask cite check` exited zero. The
  corpus gates emitted only the loaded-host performance advisory. The first
  `cargo test --workspace` run reached the unchanged
  `deckmaste_migrations::resolve::tests::ascend_gate_const_matches_canonical_condition`
  baseline assertion after all touched-crate suites passed; the exact rerun
  reproduced its `Candidate(Object)` versus `Candidate(Entity)` mismatch.
  Final workspace-gate disposition is pending the required concurrent refresh.
- Assurance census: restored 0; re-spelled 7 existing test names, with their
  affected assertions and fixtures migrated in place; ignored 0; added 0;
  removed 0.
- STOPs: none. Glossary gaps: none. Decision wanted: none.

### Deviations and additions

- None. The change is vocabulary-only and introduces no grammar, construction,
  selection, coverage, or execution-semantic movement.

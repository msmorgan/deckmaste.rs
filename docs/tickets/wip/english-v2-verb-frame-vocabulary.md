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
`uqluoopr`. The vocabulary change is `qnnmmtzz`. The frozen coverage lock has
49,421 lines and SHA-256
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
  dated decision amendments were retained. The number-feature and visitor-leaf
  siblings then refreshed in without conflicts. The final conflict listing is
  empty, and every gate was rerun after the last changed refresh; the leaf
  census is 235,683 expected and 235,683 visited with zero failures.
- Assurance census: restored 0; re-spelled 7 existing test names, with their
  affected assertions and fixtures migrated in place; ignored 0; added 0;
  removed 0.
- STOPs: none. Glossary gaps: none. Decision wanted: none.

### Deviations and additions

- None. Intermediate refreshed-parent fixture failures were superseded by the
  final parent before measurement and do not remain in this feature diff.

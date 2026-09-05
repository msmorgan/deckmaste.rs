---
needs: [english-v2-closed-class-single-owner]
---
**Rehome the closed-class vocabularies whose names still record the
construction they were born in.** Single ownership (2026-09-04) pointed
general constructions at vocabularies named for one earlier consumer, so the
declared name now misdescribes the word it spells:

- `ObjectOrder { Any, Random }` spells the determiner of `in any order`; the
  quantifying determiner `any number of` now consumes `ObjectOrder::Any`, and
  its element field is called `order`. Note the `Determiner` lexicon already
  carries a member realizing `any`, so the right owner may be the lexeme
  rather than a vocabulary at all.
- `CostComparisonDirection { More, Less }` is consumed by four scalar
  comparisons that are not cost comparisons.
- `DistributionReplacement { Instead }` is a distributed-measure verb tail
  slot; the general replacement predicate now consumes it.

Decide each word's real owner (vocabulary vs closed lexicon), rename the
vocabulary or move the member, and re-spell the element fields to name the
role rather than the vocabulary. Ownership provenance strings
(`vocab:<Vocabulary>/<Member>`) and their tests move with it; zero coverage
change and no new form literals. Standard constraints apply.

## Landing record

Measured on `zovmmrmxokyp` (the review-corrected feature tip, rebased onto
`default@` after the second `kata refresh`); lock `covered` = 17,601. This
record commit `lxovtwvoxwzz` adds only this file, and trunk advanced after the
gates with an idris-and-tickets-only delta that reaches no gated crate.

### Prove

- `ArbitraryDeterminer { Any, Random }` remains a vocabulary: its two members
  jointly fill the determiner function in both consumers, while moving only
  `Any` into the `DeterminativeHead` lexicon would split that slot and change
  its category path, and `a random` is not a lexical determinative at all. The
  name states what the words are — determiners of an unspecified value — and
  no longer records `ordered_predicate`, which is the point of this ticket:
  the second consumer, `any_number_quantifying_determiner` (`any number of`),
  has no order. `ComparisonDirection { More, Less }` and `ReplacementMarker
  { Instead }` likewise name their grammatical roles rather than their former
  first consumers.
- The vocabulary uses, generated AST and visitor exports, declared verb-frame
  roles in `core_verbs.ron` and `environment.rs`, and role fields now use
  those names. Provenance was re-spelled as `vocab:ComparisonDirection/Less`
  and `vocab:ReplacementMarker/Instead`; provenance remains attestation, not a
  selection filter.
- The coverage lock is byte-identical to `default@`'s (`cmp` exit 0, 50,251
  lines, `covered` 17,601): lock delta `+0/-0`, no gained, lost, or
  selection-changed identities, and therefore no selected analysis to name. No
  new form literal was added.
- Per-unit selection neutrality is proven, not asserted: `ambiguity --json`
  was run on `default@` in a reflinked scratch checkout and on this tree, and
  after normalizing only the three vocabulary names (including the lowercase
  spelling the parser prints inside `expected …` messages) the two 151,877,336
  and 151,875,554-byte reports are byte-identical — every one of the 32,641
  rows keeps its status, candidates, comparisons, survivors, selected ordinal
  and resolution. The only unnormalized differences were 2,621 diff lines, all
  inside `"message"` of `parse_failure` rows, all of the form `object order` →
  `arbitrary determiner`.
- Gated-tree coverage: 17,601 selected / 17,601 covered, 0 selected
  uncovered, 0 unresolved ties, 0 internal failures, 0 round-trip mismatches,
  0 ownership failures, 0 provenance-plan mismatches, and 0 forbidden
  licensing checkers. The required clean round trip is 17,601 / 17,601.

### Disclose

- Selection census before/after: 17,601 selected, 17,601 covered; unique
  13,759 / specificity-resolved 3,842, exception-resolved 0, unresolved ties
  0 — identical on both sides by the byte-identical normalized report above.
- Construction census before/after: 387 / 387; permitted licensing checkers
  20, forbidden 0; no `environment.rs` load errors.
- Assurance: restored 0; re-spelled 7 existing assertions, imports or visitor
  hooks (`nominal_grammar.rs` import, provenance string and `assert_eq`;
  `parser.rs` and `vertical_slice.rs` constructor arguments;
  `predicate_grammar.rs` visitor hook and provenance string); ignored 0; added
  0; removed 0. Every test whose subject survives still passes.
- Deviation and addition: updated the one live planned frame ticket that
  cites the renamed vocabulary (`english-v2-lexeme-owned-verb-frames`), a
  reference correction only; no construction, test, or literal was added
  beyond this ticket's nomenclature scope.
- glossary gap: arbitrary determiner — the role of *any* / *a random* marking
  an unspecified order or quantity has no glossary entry. `Determiner`
  (function) and `Determinative` (category) are both in
  `docs/contexts/oracle-english/CONTEXT.md`; the vocabulary is named for the
  function it fills, because `a random` is not a Determinative.
- glossary gap: comparison direction — the role of `more`/`less` has no
  glossary entry.
- glossary gap: replacement marker — the role of `instead` has no glossary
  entry.
- STOP: none. The implementer's first refresh reported a divergent sibling
  working copy (`fight`); following the harmony diagnostic it was left
  untouched, and both of the reviewer's refreshes reported the same workspace
  stale and untouched.

### Review corrections

- MEDIUM — incomplete letter. `OrderDeterminer` still recorded the
  construction the vocabulary was born in (`ordered_predicate`, `in any
  order`), which is the exact defect this ticket exists to remove: the
  vocabulary's other consumer, `any_number_quantifying_determiner`, spells
  `any number of` and involves no order. Fixed by renaming the vocabulary to
  `ArbitraryDeterminer` across `constructions.rs`, `ast.rs`, `visit.rs`,
  `environment.rs`, `core_verbs.ron`, `predicate_grammar.rs` and the planned
  R12 ticket, keeping the exports alphabetically placed
  (`review: name the arbitrary determiner vocabulary for its words, not the
  order construction`). `ComparisonDirection` and `ReplacementMarker` were
  read against the glossary and kept.
- MEDIUM — misleading record. The implementer's numbers carried no change-id
  or lock-`covered` stamp, offered no per-unit selection evidence ("not
  applicable"), and stamped a sandbox-visible contention count of 1. Fixed:
  the record above is stamped, the `ambiguity --json` trunk-vs-tree proof was
  run, and the true contention is recorded below.
- The re-run of `cargo test --workspace` confirms the gate scope
  `core_verbs.ron` requires; the implementer had recorded it as run.

### Report

Gates, all foreground, all on the gated tree after the final `kata refresh`:

- `cargo fmt --all`: no changes.
- `cargo clippy -p deckmaste_english_v2 --all-targets -- -D warnings`:
  `Finished dev profile ... in 17.42s`, zero warnings.
- `cargo test --workspace` (required: `core_verbs.ron` changed): 128
  `test result: ok` lines, 6,150 passed, 0 failed, 2 ignored (both
  unrelated to this diff, each carrying a named blocker in its attribute).
- `DECKMASTE_COVERAGE_LOCK=report cargo xtask english_v2 coverage --check
  --workers 8`: exit 0, no lock delta printed, working copy clean,
  `cmp english-v2-coverage.lock` against `default@`'s exit 0.
- `cargo xtask english_v2 ambiguity --require-resolved --json --workers 8`:
  exit 0, 0 unresolved ties.
- `cargo xtask english_v2 roundtrip --require-clean --workers 8`: clean
  17,601, mismatched 0.
- Citations were unchanged, so no cite gate applies.

Performance advisory (16.26 s quiet-host ceiling; every figure below is far
above it because the host was shared, not because the grammar slowed):

- coverage: 125 s wall, 141,143 ns/B, workers 8, host load
  18.57 / 18.15 / 21.22.
- ambiguity: 129 s wall, 175,090 ns/B, workers 8, host load
  17.66 / 18.87 / 21.13.
- roundtrip: 109 s wall, 140,279 ns/B, workers 8, host load
  11.47 / 16.84 / 20.18.
- trunk baseline ambiguity (reflinked scratch checkout): 118 s wall,
  165,441 ns/B, workers 8 — 165,441 vs 175,090 ns/B across the rename is host
  noise, not a parse-time regression; the two runs parse identical trees.

Inventories (provenance, never fitted to):

- Licensed vocab/lexicon homographs (2): `vocab AttributiveAdjective::Untap`
  beside the `Untap` keyword-action declaration; `vocab
  TargetingMarker::Target` beside `CommonNoun::Target`.
- Form-literal/vocabulary overlaps (9): `additional` at `additional_cost`;
  `to` at `up_to_quantifying_determiner`; `the` and `next` at
  `definite_next_mass_quantity_reference`; `to` at
  `scalar_less_than_or_equal_to`; `the` at `number_of_scalar_value`; `the` at
  `greatest_scalar_value`; `other` at `other_than_qualified_reference`; `the`
  at `positional_partitive`. Unchanged by this landing.
- Contention stamp: three concurrent kata executors and three concurrent
  landing reviews were live on this host for the duration of these gates. The
  implementer's "concurrent Codex process count: 1" was a sandbox artifact and
  is superseded by this line.

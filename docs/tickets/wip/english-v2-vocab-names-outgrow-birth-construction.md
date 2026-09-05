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

### Prove

- `OrderDeterminer { Any, Random }` remains a vocabulary: its two members
  jointly realize the determiner slot of the order construction, while moving
  only `Any` into `Determiner` would split that slot and change its category
  path. `ComparisonDirection { More, Less }` and `ReplacementMarker { Instead
  }` name their grammatical roles rather than their former first consumers.
- The vocabulary uses, generated AST and visitor exports, declared verb-frame
  roles, and role fields now use those names. Provenance was re-spelled as
  `vocab:ComparisonDirection/Less` and `vocab:ReplacementMarker/Instead`;
  provenance remains attestation, not a selection filter.
- The coverage lock is byte-identical to the parent (`cmp` exit 0): lock delta
  `+0/-0`, no gained, lost, or selection-changed identities, and therefore no
  selected analysis to name. No new form literal was added.
- Refreshed-tree coverage: 17,601 selected / 17,601 covered, 0 selected
  uncovered, 0 unresolved ties, 0 internal failures, 0 round-trip mismatches,
  0 ownership failures, 0 provenance-plan mismatches, and 0 forbidden
  licensing checkers. The required clean round trip is 17,601 / 17,601.

### Disclose

- Selection census before/after: 17,601 selected and 17,601 covered; the
  exact parent/current coverage-lock comparison is the per-unit evidence.
  The ambiguity before/after artifact is not applicable: this is a
  nomenclature-only change with identical forms and requirements, not a
  grammar change.
- Construction census before/after: 387 / 387. Licensed vocab/lexicon
  homographs: 2; form-literal/vocabulary overlaps: 9; permitted licensing
  checkers: 20; forbidden: 0.
- Assurance: restored 0; re-spelled 7 existing assertions or visitor hooks;
  ignored 0; added 0; removed 0.
- Deviation and addition: updated the one live planned frame ticket that
  cites the renamed vocabulary (`english-v2-lexeme-owned-verb-frames`); no
  construction, test, or literal was added beyond this ticket's nomenclature
  scope.
- glossary gap: comparison direction — the role of `more`/`less` has no
  glossary entry.
- glossary gap: replacement marker — the role of `instead` has no glossary
  entry.
- STOP: none. The first refresh reported a divergent sibling working copy;
  following the harmony diagnostic it was left untouched, and the prescribed
  retry was a no-op refresh.

### Report

- `DECKMASTE_COVERAGE_LOCK=report cargo xtask english_v2 coverage --check
  --workers 8`: 113.864409143 s, 137451 ns/B, host load 21.13 / 27.58 /
  20.03. `roundtrip --require-clean --workers 8`: 143.555832151 s, 176606
  ns/B, host load 21.07 / 25.55 / 20.50. Sandbox-visible concurrent Codex
  process count: 1; sibling-host contention is not observable here.
- Gates on the refreshed tree: `cargo fmt --all`; strict
  `cargo clippy -p deckmaste_english_v2 --all-targets -- -D warnings`;
  `cargo test --workspace`; coverage check above; and clean round trip above.
  Citations were unchanged, so no cite gate applies.

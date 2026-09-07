---
needs: []
---
# Analyze declared vocabulary independently of grammar

Implement the lexical interface in
[the accepted decision](../../decisions/english-lexical-analysis.md#lexical-analysis).
Reuse catalog identities, provider provenance, known forms/irregular data and
immutable indexes. Core and plugin vocabulary feed the same analysis interface;
their source formats need not be unified. Keep morphological generation shared
between analysis and declaration-driven rendering. No grammar/compiler dependency
may be required merely to inspect a word's analyses.

Pin token boundaries, separators, capitalization, bound/overlapping multiword
analyses, token occurrences and derived positions through supported corpus
examples. Preserve raw versus normalized input provenance. Analyses retain
lexeme/category/form identity and correlated feature bundles, with explicit
feature applicability. Lexemes supply grammatical properties and frames;
the tagger does not select a contextual reading.

Implement one default per applicable inflection and explicit replacing irregular
overrides. State whether each default is suffix append or a deterministic
orthographic algorithm, test its actual cases, and list valid explicit
alternatives. No unknown-word/POS/derivation speculation or inferred paradigm
classes. Preserve noun/verb homographs and distinct forms with the same spelling.

Reuse the handwritten numeral parse/format codecs as nonexclusive lexical
analyses. Retain noun and determinative readings of `one` alongside its numeric
reading, including `one or two targets`; preserve numeric value and notation.
Reject constructed numeric values whose notation cannot roundtrip losslessly.

Run lexical analysis over the entire supported corpus independently of parsing.
Produce the unique-word/remainder inventory requested by the user, accounting
separately for symbols, keywords and open catalogs. Unknown forms are explicit
gaps with source occurrences; this ticket establishes accounting and the shared
interface, not exhaustive vocabulary completion. The lexical-source ticket
owns the remaining inventory.

Acceptance: direct analysis and generation fixtures cover homographs, default
and irregular inflections, replacement rather than additive override behavior,
explicit variants, multiword overlap, sentence-initial forms and unknown-word
exclusions. Preserve exact surfaces through the declared lexical realization
interface; report the corpus remainder and runtime. No full-card coverage gain
is claimed before chart integration. Standard constraints apply.

## Landing record

Measured tree: `pznusnuuvxowusltwvptxqklxksoylsz`; unchanged parser lock
`covered`: 20,254. This landing establishes lexical analysis, not card parsing.

### PROVE

`deckmaste_lexical` freezes declared forms into shared analysis/realization
indexes without construction, compiler, catalog-I/O or game-model dependencies.
The standalone inspector consumes exported RON. Existing catalog and plugin
identities, grammatical properties and frames survive the export bridge.
`counters` retains plural Noun and finite third-singular present Verb readings;
`one` retains Noun, Determinative and numeric readings, including in
`one or two targets`. Defaults and replacing irregular variants share one
implementation. Unknown vocabulary and applicability remain explicit gaps.

The complete supported-corpus pass checked exact realization for 1,728,377
lexical occurrences and independent realization/reanalysis for 38,648 indexed
values. Numeric property tests additionally construct values independently.
Occurrence positions are derived from the lossless scalar sequence and do not
enter lexical identity. Raw text and optional normalized provenance are distinct.
The [interface documentation](../../../crates/deckmaste_lexical/README.md)
pins separators, capitalization, binding, applicability and spelling algorithms.

Production grammar, parser admission, generated construction traversal and
coverage lock are unchanged. No covered card identity was added or removed;
there is no new construction-selection or preference census. Grammatical
composition and both full-reading laws belong to
`english-v2-feature-chart-integration`. No word-naming admission guards were
added: lexical exceptions are declared form data. The existing environment
and downstream test suites passed. No internal lexical failures occurred in
the final full census.

### DISCLOSE

The census has 32,641 supported faces (32,285 with raw text), 914,801 word
occurrences and 6,824 distinct words. It reports 167,609 unknown occurrences,
representing 2,581 distinct remainders, with raw face identities and byte
occurrences. These are inventory gaps, not failed card parses. Frequent gaps
include `a`, `or`, `and`, `if`, `may`, `until`, `as`, `an`, `then`, `any`, `is`
and `can't`; many currently live in construction literals or unmapped auxiliary
inventories. `english-v2-grammar-lexical-source` owns their reviewed declarations,
modal applicability, derived adjectives, genitives/contractions and symbol gaps.

The report names 242 source issues/policy notes: 6 auxiliary inventories,
18 construction affixes, 122 construction literals, 42 plugin entries,
16 retired nonattestation policies, 4 supplemental adjectives and 34 vocabulary
inventories. These are not 242 missing words. It retains overlapping ordinary,
catalog, keyword, numeral and symbol classifications rather than subtracting a
catalog's internal words globally. Both `one` and `counters` have zero unknown
occurrences. Named source and overlap inventories are in the reproducible JSON
artifact described below; they are not copied into tracked evidence fixtures.

Deviations and additions:

- Added a standalone crate and xtask export bridge so lexical inspection requires
  no grammar/compiler runtime. Source-format consolidation remains with the
  lexical-source ticket.
- Added owner-preserving supplemental irregular data and Noun/Determinative
  declarations for `one`. Existing active WIP sources and tickets were untouched;
  the inventory owner will consolidate supplemental overrides as authoring
  sources adopt shared morphology.
- Restored all 19 handwritten numeral tests with their codecs; added 31 tests
  (5 numeral, 16 lexical integration, 10 source/accounting). Re-spelled 0,
  newly ignored 0, removed 0. No construction was added or deleted.
- Review exposed combining-mark boundaries, crossing-overlap accounting,
  inapplicable feature bundles and comma-separated cardinal scanning; the code
  was repaired and the witnesses retained. Checked numeric realization also
  rejects values whose legacy Roman formatter saturates lossily.

No unresolved STOP or glossary gap remains. Rust LSP was unavailable in this
session's exposed tool catalog (only Lean LSP was available); compiler diagnostics
and source inspection supplied validation. No CR citation sites were changed.

### REPORT

The exported inventory contains 35,077 lexemes: 33,358 catalog, 227 core,
938 keyword and 554 plugin source owners. Production construction counts,
permitted licensing-checker totals and selection shares were not remeasured:
their code/data is unchanged, and this pass is explicitly lexical.

The corpus command took 17,285 ms wall time; analysis/accounting took 12,630 ms,
loading 428 ms, indexing 164 ms and the independent-value law 250 ms.
One worker processed 5,170,266 raw bytes at 3,265 ns/B of process CPU.
Host load was 12.64 (one-minute), with the test gate running concurrently.
This is process telemetry including export/report work, not the parser's
per-accepted-byte thread metric or a quiet-host comparison to its 16,260 ms
ceiling.

Validation: `cargo xtask gate --changed --clippy --run` derived
`cargo test -p deckmaste -p deckmaste_english_v2 -p deckmaste_lexical -p xtask`:
1,017 passed, 0 failed, 1 preexisting ignored census test
(`macro_schema_census_count_matches_21`, explicitly on demand).
After fixing a semicolon lint, the exact derived command
`cargo clippy -p deckmaste -p deckmaste_english_v2 -p deckmaste_lexical -p xtask --all-targets -- -D warnings`
passed. The semicolon-only fix did not require repeating successful tests.
The standalone exported-RON inspector and dependency inspection also passed.

Reproduce the corpus inventory with:

```sh
cargo xtask english_v2 lexical --data data/mtgjson/AtomicCards.json \
  --output /tmp/english-lexical-corpus.json \
  --export /tmp/english-lexical-final-export.ron
```

Measured input SHA-256:
`e85359d7b8c578df13dff2fdf7c743a520a5b367d5ed25ab0a5f03cb8b3637dd`.
Session artifacts: `/tmp/english-lexical-corpus.json`,
`/tmp/english-lexical-unknown-words.txt`, `/tmp/english-lexical-words.txt`,
`/tmp/english-lexical-final-gate.log`, `/tmp/english-lexical-final-clippy.log`.
The JSON retains the full named source/overlap inventories and input provenance;
these artifacts are local evidence, not durable prerequisites for later tickets.

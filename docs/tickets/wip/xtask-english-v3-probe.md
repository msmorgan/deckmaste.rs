---
needs: []
---
# Probe English v3 fragment parsing across specified categories in xtask

Expose direct free-text and category-targeted parsing in `cargo xtask english-v3`.

Currently, `cargo xtask english-v3` only executes full card-face corpus evaluations (`--field text` mapped to `Category::Document` or `--field type-line` mapped to `Category::TypeLine`) sourced from card JSONL snapshots. The older `cargo xtask english probe` command operates exclusively against the retired v1 `deckmaste_english` parser. There is no CLI interface to test or inspect how arbitrary free-text fragments (e.g. `"Vampire"`, `"nonland"`, `"Draw a card."`, `"target creature you control"`, `"equipped creature has flying"`) parse directly into a targeted grammatical category.

The underlying engine `deckmaste_english_v3::parse` already natively accepts any `&G::Category` as its `start` nonterminal, and unit tests in `crates/deckmaste_english_v3/tests/grammar.rs` already exercise this. `xtask` already loads `Lexicon` and `Grammar`.

## Actionable changes

1. **Add `probe` functionality to `cargo xtask english-v3`:**
   - `--text <STRING>`: arbitrary input text string to analyze and parse.
   - `--category <CATEGORY>`: the start Category to parse against (e.g. `document`, `sentence`, `clause`, `imperative`, `finite-predicate`, `noun-phrase`, `nominal`, `preposition-phrase`, `cost`, etc.), supporting case-insensitive kebab-case names. Defaults to `document` or `sentence`.
   - `--readings <N>`: maximum distinct materialized readings to display (default 4; 0 inspects recognition / forest roots only).
   - `--packed`: print chart metrics, completed families, and packed nodes from `Forest::metrics()`.
   - `--json`: emit structured machine-readable JSON containing lexical alternatives, vocabulary gaps, admitted root count, reading count, and materialized readings.

2. **Lexical and realization reporting:**
   - Print lexical analysis results: recognized token matches, categories, and any vocabulary gaps / unknown words.
   - For each admitted reading, verify byte-exact realization against the input text (or report any mismatch).
   - Display formatted reading trees or debug representations for inspected readings.

3. **Batch / manifest support (optional flag):**
   - Accept `--manifest <PATH>` pointing to a JSON list of tagged test fragments (`{"text": "...", "category": "...", "expected_readings": N}`) to validate a suite of fragments in a single run.

## Acceptance criteria

- `cargo xtask english-v3 probe --text "Draw a card." --category sentence` runs cleanly, prints admitted readings, and validates byte-exact realization.
- `cargo xtask english-v3 probe --text "target tapped creature" --category noun-phrase` parses and outputs the structural reading.
- Missing vocabulary reports unknown-word diagnostics clearly.
- `cargo xtask gate --changed --run`, `cargo fmt --all --check`, and `cargo xtask cite check` pass.

## Landing record

Change `oksmympqtpnsotlyvzrynurnuxnqlqzq`: direct fragment inspection is
implemented in xtask, using the unchanged lexical inventory, generated grammar,
packed recognizer and existing admission/roundtrip/traversal validator.

PROVE: no grammar, lexical declaration, construction or coverage identity changed;
no coverage losses or re-coverage obligations arise. Both required fragments
produce one admitted reading with byte-exact realization and matching lexical
and construction traversal identities. Unknown words retain byte-offset diagnostics,
including after a multibyte character. Recognition-only inspection materializes
zero readings. Internal/materialization/validation issues are reported and fail
the command. No licensing checks or word-naming guards were introduced.

DISCLOSE: no new corpus coverage or selection-policy change is claimed. Forest
root counts describe admitted summary groups; inspected-reading counts are
explicitly capped, and reports state whether enumeration exhausted the forest.
No preference ranking or pruning was added. Deviations and additions: the
optional manifest flag is omitted; two focused tests cover CLI compatibility,
case-insensitive category resolution, required fragments, unknown vocabulary,
packed diagnostics and recognition-only operation. Tests restored 0, re-spelled
0, newly ignored 0, added 2, removed 0. Existing census tests retain their
assertions with the optional CLI output representation. README usage was added.
No STOPs or glossary gaps arose.

REPORT: this is CLI/reporting work, so corpus coverage, lock covered count,
construction census, homograph/overlap inventories, licensing totals and
coverage performance telemetry were not remeasured. Their underlying grammar
and lexicon are unchanged; no corpus performance or coverage claim is made.

Validation: both acceptance CLI examples pass; `cargo fmt --all --check` passes;
`cargo xtask cite check` reports 0 stale citations. The changed gate selects
`cargo test -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask`.
The changed gate passes, including 358 xtask unit tests (1 existing ignored
test), the construction/runtime suites, integration tests and doc tests.

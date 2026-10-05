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

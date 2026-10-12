---
needs: [grammar-registered-labels-not-flavor-words]
---
# Size report, grammar line total and census check mode

Build the size instrument, record today's total, fail the check when the total rises, and give the census tool an enforcing check mode. A cap without enforcement was the 2026-10-04 mistake: the 2,800-line ceiling was crossed on 6 October and only disclosed, and `declarations.rs` grew from 4,339 to 7,702 lines in four days. 2,400 lines at 98% coverage existed before the 2026-09 loss, so 7,000 lines is not acceptable. Likewise `diff` is a report, never a gate: it exits 0 whatever it finds (`crates/xtask/src/english_v3/diff.rs:3`), so no landing gate has had an enforcing command.

## Size report (visibility only)

- A command under `cargo xtask english-v3` prints, over the whole grammar, a total for every declaration kind: lines, features, categories, category interface entries, constructions, policies, tables, schemas, instance blocks, schema instances, frames, `frame_category` mappings, separators, forms, and the `export`/`require`/`agree`/`use` lines. Counts of statements are line-leading statement counts. Measured at the start: 7,702 lines; 184 features; 198 categories; 714 interface entries; 255 constructions; 135 policies; 216 tables; 71 schemas; 83 instance blocks; 415 schema instances; 41 frames; 9 frame categories; 10 separators; 2,082 `export` lines (838 forwards, 993 constants), 716 `require`, 157 `use`, about 370 forms.
- Authored counts are what the source declares; expanded counts are what the compiler generates from it (inherited exports, generated coordination forms). Both are reported, so a compiler capability cannot hide growth and the expanded counts show what the chart sees.
- Additions and removals are reported separately as well as net, so deleting one declaration cannot conceal an unrelated addition.
- Each landing's note prints the report. Nothing in it is enforced except the line total below. The report goes under `target/english-v3/`.

## Grammar line total (enforced)

- One number: total authored grammar lines. It lives in one small tracked file, checked by the census check mode.
- Source boundary, defined once: grammar lines are those of `crates/deckmaste_english_v3/src/declarations.rs` plus every other file that declares grammar, and from the start the two lexicon files whose contents later move into the grammar crate, `crates/deckmaste_lexical_source/lexicon/vocabulary.ron` and `core.ron`, so that moving them is not counted as growth. Rust helper code for the grammar, the closed-class declarations (when `grammar-closed-class-words` lands) and the analysis-key mapping annotations (when `grammar-analysis-key` lands) all count. The tracked file names the files counted.
- During the consolidation the number may not rise, and each landing lowers it to the new total. The one approved exception is the `grammar-analysis-key` landing, which may raise it by its mapping annotations and reports by how much.
- At the coverage-resume step the number is replaced by a marginal rate (see `grammar-coverage-resume`).

## Census check mode

A mode of the census tool that exits non-zero on any of: a lost face; an incomplete enumeration; a baseline whose report or digest version is incompatible; a difference prohibited for the landing type; the grammar line total rising. Two landing types: identical-digest (every per-face reading digest equal) and analysis-key (no face lost, and per line the set of analysis keys unchanged while tree count falls, or, where the ticket allows, only reviewed key additions). The analysis-key type becomes available when `grammar-analysis-key` lands. Every later ticket's gate names this mode and its landing type. Its output stays under `target/english-v3/`. The landing updates the census description in `crates/deckmaste_english_v3/README.md`.

## Gate

Reading digests identical on every face, checked with the new mode as an identical-digest landing. Note: census delta (none); the size report is printed and the tracked line total is created at today's total; the analysis-key and judgment-set lines do not apply yet.

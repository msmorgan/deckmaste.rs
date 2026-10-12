---
needs: [grammar-registered-labels-not-flavor-words]
---
# Size report and census check mode

Build a size report anyone can run, and give the census tool an enforcing check mode. The grammar grew from 4,339 to 7,702 lines in four days without anyone looking; the report makes looking cheap. Separately, `diff` is a report, never a gate: it exits 0 whatever it finds (`crates/xtask/src/english_v3/diff.rs:3`), so no landing gate has had an enforcing command.

## Size report

A tool command. Nothing is tracked and nothing is enforced; the output goes under `target/english-v3/`.

- A command under `cargo xtask english-v3` prints, over the whole grammar, a total for every declaration kind: lines, features, categories, category interface entries, constructions, policies, tables, schemas, instance blocks, schema instances, frames, `frame_category` mappings, separators, forms, and the `export`/`require`/`agree`/`use` lines. Counts of statements are line-leading statement counts. Measured at the start: 7,702 lines; 184 features; 198 categories; 714 interface entries; 255 constructions; 135 policies; 216 tables; 71 schemas; 83 instance blocks; 415 schema instances; 41 frames; 9 frame categories; 10 separators; 2,082 `export` lines (838 forwards, 993 constants), 716 `require`, 157 `use`, about 370 forms.
- Authored counts are what the source declares; expanded counts are what the compiler generates from it (inherited exports, generated coordination forms). Both are reported.
- Source boundary, defined once: grammar lines are those of `crates/deckmaste_english_v3/src/declarations.rs` plus every other file that declares grammar, and from the start the two lexicon files whose contents later move into the grammar crate, `crates/deckmaste_lexical_source/lexicon/vocabulary.ron` and `core.ron`, so that moving them does not change the count. Rust helper code for the grammar, the closed-class declarations (when `grammar-closed-class-words` lands) and the analysis-key mapping annotations (when `grammar-analysis-key` lands) all count.

## Census check mode

A mode of the census tool that exits non-zero on any of: a lost face; an incomplete enumeration; a baseline whose report or digest version is incompatible; a difference prohibited for the landing type. It checks nothing about size. Two landing types: identical-digest (every per-face reading digest equal) and analysis-key (no face lost, and per line the set of analysis keys unchanged while tree count falls, or, where the ticket allows, only reviewed key additions). The analysis-key type becomes available when `grammar-analysis-key` lands. Every later ticket's gate names this mode and its landing type. Its output stays under `target/english-v3/`. The landing updates the census description in `crates/deckmaste_english_v3/README.md`.

## Gate

Reading digests identical on every face, checked with the new mode as an identical-digest landing. Note: census delta (none); the judgment-set line does not apply yet.

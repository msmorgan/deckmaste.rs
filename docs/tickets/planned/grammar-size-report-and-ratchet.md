---
needs: [grammar-per-sentence-enumeration]
---
# Size report, ratchet test and census check mode

Build the size instrument, record today's totals, fail the test run when any total rises, and give the census tool an enforcing check mode. A cap without enforcement was the 2026-10-04 mistake: the 2,800-line ceiling was crossed on 6 October and only disclosed, and `declarations.rs` grew from 4,339 to 7,702 lines in four days. 2,400 lines at 98% coverage existed before the 2026-09 loss, so 7,000 lines is not acceptable. Likewise `diff` is a report, never a gate: it exits 0 whatever it finds (`crates/xtask/src/english_v3/diff.rs:3`), so no landing gate has had an enforcing command.

## Size report

- A command under `cargo xtask english-v3` prints, over the whole grammar, a total for every declaration kind: lines, features, categories, category interface entries, constructions, policies, tables, schemas, instance blocks, schema instances, frames, `frame_category` mappings, separators, forms, and the `export`/`require`/`agree`/`use` lines. Counts of statements are line-leading statement counts. Measured at the start: 7,702 lines; 184 features; 198 categories; 714 interface entries; 255 constructions; 135 policies; 216 tables; 71 schemas; 83 instance blocks; 415 schema instances; 41 frames; 9 frame categories; 10 separators; 2,082 `export` lines (838 forwards, 993 constants), 716 `require`, 157 `use`, about 370 forms.
- Authored counts are what the source declares; expanded counts are what the compiler generates from it (inherited exports, generated coordination forms). Both are reported; the ratchet applies to the authored counts, so a compiler capability cannot hide growth, and the expanded counts show what the chart sees.
- Source boundary: grammar lines are those of `crates/deckmaste_english_v3/src/declarations.rs` plus every other file that declares grammar, including Rust helpers and the closed-class declarations when `grammar-closed-class-words` lands. The tracked file names the files counted; adding a file to the list is an owner-approved change.
- It prints the delta against the tracked totals, with additions and removals reported separately as well as net, so deleting one declaration cannot conceal an unrelated addition. The report goes under `target/english-v3/`.

## Ratchet

- One small tracked file holds the current totals per kind. A test fails if any total rises above it. Totals move only down; lowering the file is part of the landing that lowers them. Raising a number is a change only the owner approves.
- A new declaration kind (typed classes, memberships, coordination declarations) enters the baseline at its first measured total only with the owner's approval, recorded in the landing note.
- The ratchet covers steps through the nominal, coordination, frame and preposition rework. At the coverage-resume step it is replaced by a marginal rate (see `grammar-coverage-resume`).

## Census check mode

A mode of the census tool that exits non-zero on any of: a lost face; an incomplete enumeration; a baseline whose report or digest version is incompatible; or a difference prohibited for the landing type. Two landing types: identical-digest (every per-face reading digest equal) and analysis-key (no face lost, and per line the set of analysis keys unchanged while tree count falls, or, where the ticket allows, only reviewed key additions). The key landing type becomes available when `grammar-analysis-key` lands. Every later ticket's gate names this mode and its landing type. Its output stays under `target/english-v3/`. The landing updates the census description in `crates/deckmaste_english_v3/README.md`.

## Gate

Reading digests identical on every face, checked with the new mode as an identical-digest landing. Report: census delta (none); the size line is the first baseline; the judgment-set line does not apply yet.

---
needs: [grammar-per-sentence-enumeration]
---
# Size report and ratchet test

Build the size instrument, record today's totals, and fail the test run when any total rises. A cap without enforcement was the 2026-10-04 mistake: the 2,800-line ceiling was crossed on 6 October and only disclosed, and `declarations.rs` grew from 4,339 to 7,702 lines in four days. 2,400 lines at 98% coverage existed before the 2026-09 loss, so 7,000 lines is not acceptable.

## Change

- A tool command (under `cargo xtask english-v3`) prints, over the whole grammar, a total for every declaration kind: lines, features, categories, category interface entries, constructions, policies, tables, schemas, instance blocks and schema instances, `export`/`require`/`agree`/`use` lines, forms. Measured at the start: 7,702 lines; 184 features; 198 categories; 714 interface entries; 255 constructions; 135 policies; 216 tables; 71 schemas; 415 schema instances; 2,082 `export` lines (838 forwards, 993 constants), 716 `require`, 157 `use`, about 370 forms.
- It prints the delta against the tracked totals. The report goes under `target/english-v3/`.
- One small tracked file holds the current totals per kind. A test fails if any total rises above it. Totals move only down; lowering the file is part of the landing that lowers them. Raising a number is a change only the owner approves.
- The ratchet covers steps through the nominal/coordination/frame/preposition rework. At the coverage-resume step it is replaced by a marginal rate (see `grammar-coverage-resume`).

## Gate

Reading digests identical on every face. Report: census delta (none); the size line is the first baseline; the judgment-set line does not apply yet.

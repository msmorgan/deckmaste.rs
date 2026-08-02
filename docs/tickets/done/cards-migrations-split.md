---
needs: []
---
Refocus `deckmaste_plugin` on its two real jobs — reading cards from files and writing
card text (the `render` module) — by moving migration/graduation file-management out of
`cards` into `deckmaste_migrations`, which then calls back into `cards` for
validation/graduation. Untangles the two crates' responsibilities (per the
card-text-render brainstorm): `cards` = card data ↔ text, `migrations` = the
extract→resolve→graduate pipeline orchestration. Scope: inventory what in `cards` is
migration-file-mgmt vs card-read/validate/render, move the former, keep `cards` lean.
No behavior change to the pipeline.

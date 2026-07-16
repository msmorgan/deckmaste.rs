---
needs: []
---
CI runs zero Idris↔Rust lockstep coverage: the old elaborator/twin-gate and
resolution-fixture tests were deleted, and their replacement — `cargo xtask
idris-check`, which re-emits each Rust card as a raw Core.idr term and
typechecks it — needs a live `idris2` binary, absent on the runner and invoked
nowhere in `.github/workflows/ci.yml`. Wire the gate in: install (or cache)
idris2 in a scheduled or optional job, run `cargo xtask idris-check`, and diff
a regenerated `crates/deckmaste_cards/tables/entailments.ron` against the
committed copy. Until then every mirror drift (see `idris-mirror-enum-gaps`)
rots silently — the 2026-07-16 drift review found the gap only by hand.

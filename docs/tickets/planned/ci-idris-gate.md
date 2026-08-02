---
needs: []
---
CI runs zero Idris↔Rust lockstep coverage: the old elaborator/twin-gate and
resolution-fixture tests were deleted, and their replacement — `cargo xtask
idris-check`, which re-emits each Rust card as a raw Core.idr term and
typechecks it — needs a live `idris2` binary, absent on the runner and invoked
nowhere in `.github/workflows/ci.yml`. Wire the gate in: install (or cache)
idris2 in a scheduled or optional job, run `cargo xtask idris-check`, and diff
a regenerated `crates/deckmaste_plugin/tables/entailments.ron` against the
committed copy. Until then every mirror drift (see `idris-mirror-enum-gaps`)
rots silently — the 2026-07-16 drift review found the gap only by hand.

**Baseline requirement (2026-08-02, from the authoring-program review):**
batch `idris-check` currently PRINTS emitter/proof failures and still exits
0, so "no regressions" stage gates in the authoring/spelling/lowering
program (`docs/decisions/authoring-spelling-lowering.md` §16.1) are
unauditable without this ticket: check in a pass/gap baseline and make the
batch command FAIL on lost passes, new gaps, or proof failures.

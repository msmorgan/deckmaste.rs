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

**Seed baseline (2026-08-03, measured on `plugin-repoint` at integration).**
`cargo xtask idris-check plugins/canon` — 68/79 cards emit and typecheck; 11
emitter gaps, every one an unmapped Rust grammar shape (no provenance-shaped
failure among them):

- `Anje's Ravager`, `Chandra, Torch of Defiance` — `Action::Cast`
  (resolution-time cast-as-effect, [CR#608.2g]) has no Idris `OneShotEffect`
  counterpart; Idris casts via the 601 permission pipeline
- `Cursed Scroll` — `ChooseValue(You, CardName, Ident(…))` not mapped
- `Deepwood Tantiv` — `BlockDeclared{of}` not mapped
- `Delver of Secrets` — `Card::TwoFaced` not mapped
- `Do or Die` — `OneShotEffect::SeparatePiles` not mapped (Idris's
  `DivideAndChoose` has a different two-pile shape)
- `Falkenrath Gorger` — unmapped keyword name `Madness`
- `Otherworldly Journey` — `EnterRider` list has no Idris `Move`/`MoveGroup`
  counterpart beyond a lone `Attacking(Some(_))`
- `Phantasmal Bear` — `EventFilter::BecomesTarget` has no Idris `EventKind`
- `Weather the Storm` — `Count::EventCount` (history lookback) not mapped
- `Wild Dogs` — `Action::GainControl` has no Idris one-shot `Action` (only the
  continuous `Modification`)

This is a session measurement, recorded so the checked-in baseline has a
starting artifact to diff against; re-measure before committing it.

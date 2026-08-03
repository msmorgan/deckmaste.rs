---
needs: []
---
**Stage 0 of the authoring/spelling/lowering program: rename the
card-loader crate to `deckmaste_plugin`, freeing the card-name space before
the authoring-era crates arrive.** Design:
`docs/decisions/authoring-spelling-lowering.md`
(§1, §11).

The crate is the plugin loader plus riders: `plugin.rs` (per-directory
loaders, `Plugin::card`/`token`), the Idris emitter, validation, fidelity,
and the deprecated legacy renderer + template index. The rename is the
required core; splitting the riders out (`plugin` + separate homes for
emit/validate — the "misc") is at the claimant's discretion, but the legacy
renderer dies in place later and should NOT get a new home.

## Scope

- Cargo rename + workspace/imports sweep (mechanical; the OLD crate name
  must not survive anywhere, including docs and xtask references).
- No semantic change of any kind.

## Gates

Standard constraints apply. Zero behavior change: full workspace suites,
`cargo xtask idris-check plugins/canon` no regressions, `cargo xtask
fidelity` PASS, wizards regen unaffected.

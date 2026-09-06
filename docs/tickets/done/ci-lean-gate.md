---
needs: []
---
**Build the Lean workbench in CI.** `.github/workflows/ci.yml` has no Lean job:
`lean/` is never built or checked, while the `idris` job still builds the
superseded Idris mirror. Add a `lean` job that

- installs elan and the toolchain pinned by `lean/lean-toolchain`
  (`leanprover/lean4:v4.33.1`; read the file, never hard-code the version);
- caches `lean/.lake` keyed on `lean-toolchain` and `lake-manifest.json`;
- runs `lean/scripts/build` (`lake build --wfail`: syntax, checker, every pin
  suite, warnings are errors) as the gate. The `decide` pins are evaluated by
  the kernel; the full build takes about a minute warm.

The citation gates already cover Lean (`cite-config.json` lists `lean`, the
coverage scan counts `Proofs/` as the tested tier since 2026-09-05), so no
extra step is needed there. Required-check activation is deferred by the user
(2026-09-06); local validation completes this ticket’s current scope.
Decision recorded: the Lean workbench is the successor of the Idris one
(`idris-retirement`), so CI gates Lean unconditionally and keeps the Idris job
only until its two remaining duties have Lean or Rust successors.

## Confirmed scope (2026-09-06)

The user accepted local validation for this round. The job will run on push and
pull request when published, but canary publication, hosted-runner verification,
and required-check activation are deferred. No push or GitHub settings change
is authorized or performed. The existing Idris job remains while its duties
await successors; the card-soundness emitter waits for `semantics_v2`.

## Landing record

Feature change `pmxqmzxlpnvnkrpuovklllylrluypkzk`; final validation recorded on
`prmmxlktzqyywrpxypsqkvsuzpqnsuvn`, lock `covered` 20,254 (2026-09-06).

### PROVE

The job reads `lean/lean-toolchain`, installs elan without modifying shell
profiles, caches `lean/.lake` by platform, toolchain, manifest, and Lean source,
and runs the existing warnings-as-errors build. The restore prefix permits
reuse only within the same toolchain and manifest. It builds the separate
semantics project through its declared default targets.

`/tmp/lean-ci-tools/actionlint .github/workflows/ci.yml` passes. The official
elan installer and the pinned toolchain download were validated earlier in an
isolated temporary directory on `pmxqmzxlpnvnkrpuovklllylrluypkzk`, lock
`covered` 20,254. After refreshing through the completed lookback refactor,
`lean/scripts/build` passes all 68 jobs with warnings as errors.

No card identity, test assertion, English construction, or licensing guard
changed in this feature. Tests restored 0, re-spelled 0, ignored 0, added 0,
removed 0. The code diff adds only the Lean workflow job.

### DISCLOSE

The original request to activate a required hosted check is deferred by the
user's explicit decision. Local validation establishes the workflow's syntax
and local command behavior; it does not establish hosted runner success. `act`
and a working Docker daemon are unavailable on this host. No checks were
removed or weakened to make the workflow pass.

No new coverage identity or glossary term is introduced. There are no code or
test additions beyond the requested Lean job. English structural, selection,
and licensing inventories were not remeasured for this workflow-only change.

### REPORT

Provenance: `prmmxlktzqyywrpxypsqkvsuzpqnsuvn`, lock `covered` 20,254.
Construction count, homograph and form-literal/vocabulary overlap inventories,
coverage wall time, host-load/worker telemetry, and ns/B were not remeasured.
The coverage lock and declaration data are unchanged. Hosted-runner success
and required-check activation remain explicitly unverified and deferred.

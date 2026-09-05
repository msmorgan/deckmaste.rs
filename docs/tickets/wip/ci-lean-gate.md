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
extra step is needed there. Make the job required alongside `clippy & test`.
Decision recorded: the Lean workbench is the successor of the Idris one
(`idris-retirement`), so CI gates Lean unconditionally and keeps the Idris job
only until its two remaining duties have Lean or Rust successors.

## Implementation and activation

The Lean job is implemented locally. Per the user's 2026-09-05 direction,
do not publish to `main` yet. Validate on a `canary` branch when authorized;
no such branch currently exists locally or on GitHub. Required-check
activation remains pending: `main` currently has neither branch protection
nor a ruleset, so there is no existing required `clippy & test` setting to
extend. No GitHub settings were changed.

The job reads `lean/lean-toolchain`, installs elan without modifying shell
profiles, restores a Lake cache partitioned by toolchain and manifest, and
runs the full warnings-as-errors gate on every push and pull request. The
source digest permits saving refreshed build outputs while restoring prior
outputs only from the same toolchain and manifest.

Local validation on change `pmxqmzxlpnvnkrpuovklllylrluypkzk`, English lock
`covered` 20,254: `actionlint` passed; the official elan installer completed in
an isolated temporary directory; `lean/scripts/build` passed all 61 jobs
without warnings. The pinned Lean release was verified as downloadable.
No test assertions or English declarations changed. Hosted runner execution
and required-check activation have not been performed; this ticket remains
in progress.

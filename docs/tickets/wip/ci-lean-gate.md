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

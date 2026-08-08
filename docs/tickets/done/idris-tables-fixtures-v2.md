---
needs: [idris-core-v2, cards-fidelity-target-sunset]
---
**Idris v2 becomes the one table source and fixture exporter; `Cards.idr`
re-encoded; `Ron.idr` deleted.** Completes the Idris role-change begun in
[[idris-core-v2]]: the reference model drives the Rust elaborator through
generated data and exported fixtures, and stops being a serialization peer.

## Table emission (the full catalog)

`idris2 --exec emitTables` regenerates `data/grammar-tables/*.ron` from the v2
total functions — now the complete set: sort-compat, intro rows, caps rows,
fact signatures (per-verb: fact kind, guaranteed fields, product arity,
cost-eligibility), entailments, lanes (per-consumer atom support), scopes
(counter/designation/subtype/patient), zone-sort, the part-wise
fixed-vs-live class column [CR#611.2c], and the delayed-trigger decision rows
[CR#603.7c] (same object in expected zone → affect, tracked live; not in
expected zone → doesn't affect, dependents skip; left-and-returned → new
object, not affected [CR#400.7]). Every row carries its bracketed CR cite; `cargo
xtask cite audit --diff` reads generated rows like prose. The interim rows
emitted from the v1 model ([[cards-elaborator-tables]]) are superseded — after
this ticket there is exactly one emission path.

## Fixture export, CI-wired

- Every `failing "<err>"` block in the v2 proof suite exports a RON reject
  fixture under `crates/deckmaste_plugin/tests/reject/` that the Rust
  elaborator must refuse with the matching error code — the twin discipline
  becomes generated, not hand-maintained.
- Every `Cards.idr` card exports its RESOLUTION TABLE (reference → bound
  antecedent) as a fixture the Rust elaborator must reproduce byte-for-byte.
- CI runs: `idris2 --build` → `emitTables` + fixture export → fail if the
  committed tables/fixtures differ from the freshly emitted ones (drift =
  red).

## Cards.idr and Ron.idr

- `Cards.idr` survives as the pathological acceptance corpus, RE-ENCODED in
  the v2 grammar; each card's rendered English must pass the fidelity gate
  ([[cards-fidelity-target-sunset]]) — this kills the known hand-drift between
  the Idris encodings and the oracle text.
- `Ron.idr` is DELETED. macro_ron is the declared wire dialect; Idris emits
  tables and fixtures, never card serializations.

## Done

- One emission path; committed tables regenerate byte-identical from v2.
- Reject fixtures + resolution tables generated and green on the Rust side; a
  Rust rule without an Idris twin (or vice versa) fails CI.
- `Cards.idr` re-encoded and fidelity-green; `Ron.idr` gone, along with any
  build plumbing that referenced it.

## Verification

- `idris2 --build mtg.ipkg` + `idris2 --exec emitTables` (in `idris/`);
  `git`-visible diff of `data/grammar-tables/` empty on a second run
  (check via `jj st`).
- `cargo test -p deckmaste_plugin` and `cargo test --workspace` green.
- `rg 'Ron.idr' idris/ crates/` — empty.
- `cargo xtask fidelity` green; `cargo xtask cite check` — 0 stale,
  `--list-noncompliant` empty; `cite audit --diff` over regenerated rows.

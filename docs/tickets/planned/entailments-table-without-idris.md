---
needs: []
---
**Emit `crates/deckmaste_plugin/tables/entailments.ron` without Idris.** The
engine reads it at build time (`deckmaste_engine/src/entail.rs`,
`include_str!`), and today `idris/scripts/emit-tables` regenerates it from
`idris/src/EmitTables.idr`; the CI `idris` job diffs the committed file against
that output. It is the one runtime artifact the Idris tree still produces.
Move the emission to the successor: a Lean program under `lean/` that derives
the same table from the Lean model (`lake exe`, output byte-identical to the
committed file on first landing), or a Rust derivation in xtask if the table's
inputs are all Rust-side. Record which and why. CI diffs the committed file
against the new emitter; `idris/scripts/emit-tables` and `EmitTables.idr` are
deleted in the same landing. Related: `idris-emittables-drift-control`
becomes moot and is closed by this ticket.

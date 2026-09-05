---
needs: [ci-lean-gate, lean-card-soundness-gate, entailments-table-without-idris, docs-lean-is-the-workbench, tickets-idris-sweep, idris-retire-cards-idr]
---
**Retire the Idris workbench.** Decision (2026-09-05): the Lean 4 workbench
under `lean/` is the successor of the Idris 2 workbench under `idris/`; the
Idris tree is a reference only, reread when a Lean port needs its evidence,
and is deleted once nothing else depends on it. This ticket is the
coordinator; its `needs` are the dependencies, in the order they unblock:

1. `ci-lean-gate`: Lean is built in CI at all.
2. `lean-card-soundness-gate`: the card-data gate (`idris-check`,
   `idris_emit.rs`, per-plugin baselines) has a Lean successor.
3. `entailments-table-without-idris`: the engine's entailment table is emitted
   without `EmitTables.idr`.
4. `docs-lean-is-the-workbench`: prose names the right workbench; the ADR is
   recorded.
5. `tickets-idris-sweep`: live tickets no longer gate on or deliver into Idris.
6. `idris-retire-cards-idr` (existing): the hand-authored Idris card list is
   migrated to a RON corpus.

Then, in this ticket's own landing: delete `idris/` (48 `.idr` files, 47,518
lines, two `.ipkg`, six scripts, `VERIFY.md`), the CI `idris` job, `cargo xtask
idris-check`, `cargo xtask facts generate/check/labels` in their Idris form
(`lean-facts-tables-from-registries` re-homes the generator; if it lands
first, only the `.idr` output path goes), `cargo xtask map idris` and
`map idris-dead` (add `map lean` for the Lean constructors first, or fold it
into the same landing), the `idr` entry in `cite-config.json` and
`crates/xtask/src/coverage.rs`, and `docs/idris-workbench-closure-tables.md`.
The `rs`-side doc comments that cite `idris/src/Semantics.idr` constructors
(`deckmaste_core` and `deckmaste_semantics` `action.rs`, `continuous.rs`,
`stat_value.rs`; `deckmaste_engine` `activate.rs`, `trigger.rs`) are
re-pointed at the Lean declarations; `core-idris-prose-sweep` covers the
larger `deckmaste_core` set and should land before or with this.

Landing record must list every CR citation that lived only in `.idr` files
(the coverage ratchet will otherwise drop them silently) and where each now
lives in Lean.

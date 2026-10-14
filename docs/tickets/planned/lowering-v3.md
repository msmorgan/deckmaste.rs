---
needs: [plugins-v3-canon]
---
**Create `crates/deckmaste_lowering_v3`: `semantics_v3` to `core` (updated
2026-10-14 per [`docs/decisions/semantics-v3.md`](../../decisions/semantics-v3.md)).**
Sibling of `deckmaste_lowering`; v1 lowering deletes at cutover. Same contract
as `semantics-spelling-lowering.md` §9 and §17: one-way compile, the crate is
the divergence ledger, rearrangement, hoisting, and indexing happen here. May
fail on a card that breaks a Lean law; does not validate. Standard constraints
apply.

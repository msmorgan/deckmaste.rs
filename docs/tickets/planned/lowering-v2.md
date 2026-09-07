---
needs: [plugins-v2-canon]
---
**Create `crates/deckmaste_lowering_v2`: semantics_v2 to core.** Sibling of
`deckmaste_lowering` under the `_v2` rule; v1 lowering deletes at cutover.
Same contract as `semantics-spelling-lowering.md` §9 and §17: one-way
compile, the crate is the divergence ledger, rearrangement, hoisting, and
indexing happen here. May fail on a card that breaks a Lean law; does not
validate. Standard constraints apply.

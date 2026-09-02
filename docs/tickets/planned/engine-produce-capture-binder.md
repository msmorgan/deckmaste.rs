---
needs: []
---
**Engine: a cost that moves an object to a public zone must be decidable as
payable, and its product readable.** [CR#400.7j]'s second sentence is
explicit that a cost may move an object to a public zone and the spell's
effects can then find it.

Rewritten 2026-09-02: the cost landing of
[Core is explicit regions](../../decisions/core-explicit-regions.md) deleted
both functions this ticket used to name. `with_cost_feasible` became
`cost_step_feasible` in `crates/deckmaste_engine/src/activate.rs`, and
`resolve_binder`'s read-only spine is gone with the `Binder` enum. Costs are
now a block of instructions with destinations, and `CostComponent::Act {
dest, .. }` captures a paid product's register.

What survives as a real question: whether `cost_step_feasible` can decide
payability for every producing cost shape, and whether a product moved by a
cost is chased to its new incarnation on the read the body makes. Re-assess
against the landed shape before sizing; the original two-consumer diagnosis
no longer describes the code. `Binder::Produce` still exists on the
semantics side, so lowering and the legacy renderer are unaffected.

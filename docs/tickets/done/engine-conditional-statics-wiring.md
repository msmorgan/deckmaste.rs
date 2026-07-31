---
needs: []
---
Wire `StaticEffect::Conditionally(Condition, StaticEffect)` into the layers
engine. The variant exists in the core grammar
(`crates/deckmaste_core/src/continuous.rs`) with semantics documented on the
variant (condition re-checked continuously, never locked in), but its doc
comment notes it is *currently unwired* — a `Conditionally` static is not yet
applied during layer evaluation.

Scope: evaluate the wrapped condition inside the whole-pass layers fixpoint
(the condition must see the current in-progress view per the established
layers design — one component, whole-pass fixpoint), apply the inner
`StaticEffect` only while it holds, and cover flap/ordering cases in
`deckmaste_engine` tests (condition flips mid-pass via another static;
condition references the modified object itself).

Pairs with `parse-as-long-as-statics` (the parse half); together they make
the ~642-card "as long as" one-away family both graduate AND execute.

Verify: engine test suite; a noncanon scenario where the condition toggles
(e.g. equip/unequip) and the granted ability appears/disappears.

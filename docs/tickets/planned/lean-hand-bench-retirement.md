---
needs: [plugins-v3-canon]
---
**[port-required: semantics-v3] Shrink the hand-written `lean/Semantics/Cards/`
bench to zero.** Each `Spelled` card there is a stand-in for a canon card not
yet authored in `plugins_v3/` (`docs/decisions/semantics-v3.md` §4); once its
DSL file lands and the emitter produces the term, the hand spelling goes. Pins
in `Proofs/` that are theorems about constructions rather than cards stay. Any
hand card with no canon counterpart gets one authored, never kept as a
Lean-only authoring surface. Standard constraints apply.

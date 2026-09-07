---
needs: [plugins-v2-canon]
---
**Shrink the hand-written `lean/Semantics/Cards/` bench to zero.** Each
`Spelled` card there is a stand-in for a canon card not yet authored in
`plugins_v2`; once its RON lands and the emitter produces the term, the hand
spelling goes. Pins in `Proofs/` that are theorems about constructions
rather than cards stay. Any hand card with no canon counterpart gets one
authored, never kept as a Lean-only authoring surface. Standard constraints
apply.

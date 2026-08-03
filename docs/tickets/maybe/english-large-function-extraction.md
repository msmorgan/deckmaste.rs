---
needs: [ci-cite-gate]
---
**[design] Decide whether three large English-pipeline functions have stable
phase boundaries worth extracting.** `Chart::scan` in `chart.rs`, the
`NominalModifier` builder in `grammar/mod.rs`, and `lower_composed_clause` in
`grammar/clause/lowering.rs` each combine multiple recognizable phases, but
function length alone is not evidence that helpers will improve the grammar.

At design time, map each function's phases, shared mutable state,
short-circuits, and invariants. Approve extraction only where a phase has a
clear input/output contract and a name more informative than the code it
wraps. Do not introduce a framework, visitor abstraction, or shared helper
merely because two phases have similar syntax.

If approved, treat each original function as a separate checkpoint and land
one commit per function. Pin outcome/count fixtures before movement. Gates:
exact test-count parity, unchanged corpus summaries and fingerprints,
workspace clippy/fmt, and a comment/citation multiset audit. Engine
interpreter work is explicitly out of scope.

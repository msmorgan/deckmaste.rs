---
needs: []
---
**Ruling: is the elaboration cost of the obligation cycles worth a
constructor-level change?** Residue of `workbench-mutual-split`
(2026-09-04), which split both whole-module `mutual` blocks and measured
the result time-neutral: `Phrase.idr` 29.5s → 28.8s, `Effect.idr` 5.3s →
4.3s. `idris2 --timing` puts 25.9s of `Phrase`'s 28s in totality checking
and 1.6s in declaration processing; the checker walks the call graph, not
the block, and the graph has two strongly connected components of 176 and
143 units (`Predicate ↔ Noun ↔ Amount` and `Cost ↔ Instruction ↔
AbilityAt`) because every constructor carries an erased obligation naming a
function that matches on the same data.

Options: (a) accept the cost (one module at 29s; the full gate is 65s);
(b) move the obligations out of the constructors into a separate checked
layer so the data types stop being cyclic with their own gates — a
design-bearing change to every core row and to the pin idiom; (c) mark the
obligation functions `partial`/`assert_total` selectively — refused on
sight, it retires the soundness the workbench exists for.

Size: ruling; (b) is L. Reopen when a build-time regression or a new
module pushes the gate past the point where the round loop suffers.

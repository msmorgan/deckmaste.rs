---
needs: [english-derived-family-inventory]
---
**Derive the fan-out-one sentence wrapper.**

Migrate S01 from `docs/english-derived-family-inventory.md`: `sentence`.

This is the capability-independent whole-subtree control: one `Clause` hole on
the existing chart, with both period-consuming forms mapping to one `Sentence`
AST. Keep the terminal period derived from sentence and terminal-quote
structure; do not add a stored punctuation witness.

Land declarations and generated parse/reduction/lowering, total render, and
validated build projections; Sentence/paragraph/cost and spelling/view
consumer changes; direct-AST and `inspect` fixtures; exact period and terminal-
quote replay; dependent-root/doubled-punctuation negatives; the registry flip;
and deletion of both handwritten productions and every reducer/lowerer/
renderer/constructor mirror in one change. Update the inventory. No consumer
or deletion work is deferred.

Standard constraints apply.

---
needs: [english-derived-family-inventory, english-derived-noun-lexeme-family]
---
**Derive prepositional phrases and typed objects.**

Migrate P02 from `docs/english-derived-family-inventory.md`:
`prepositional_phrase` and `prepositional_object`.

Use identity holes for the preposition and whole typed subtree alternatives
for noun-phrase, PP, gerund, and adverb objects. Add object-category,
selected-complement versus adjunct, and attachment-role constraints; preserve
legitimate packed PP attachment rather than selecting by registration order.

Land declarations and generated parse/reduction/lowering, total render, and
validated build projections; all nominal/predicate/cost/spelling/view consumer
changes; direct-AST and `inspect` attachment fixtures; nested exact replay and
invalid-role negatives; both registry flips; and deletion of every handwritten
mirror and constructor bypass together. Update the inventory. No consumer or
deletion work is deferred.

Standard constraints apply.

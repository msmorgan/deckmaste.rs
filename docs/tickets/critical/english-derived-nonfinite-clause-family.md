---
needs: [english-derived-family-inventory, english-derived-predicate-family]
---
**Derive infinitive and gerund clauses.**

Migrate F01 from `docs/english-derived-family-inventory.md`: `infinitive_to`,
`infinitive_not_to`, `gerund_clause_base`, and
`gerund_clause_subordinate_after`.

Use the predicate family's valency/form constraints with whole predicate and
clause subtrees. Preserve infinitive negation and subordinate attachment as
typed distinctions; fixed `to` spelling and spacing remain derived.

Land declarations and generated parse/reduction/lowering, total render, and
validated build projections; all nominal/predicate/PP/complex-clause and
spelling/view consumer changes; direct-AST and `inspect` form fixtures;
exactness and wrong-form/root negatives; all four registry flips; and deletion
of handwritten mirrors and bypassing constructors together. Update the
inventory. No consumer or deletion work is deferred.

Standard constraints apply.

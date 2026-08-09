---
needs: [english-derived-family-inventory, english-derived-quantity-family, english-derived-noun-lexeme-family, english-derived-nominal-family]
---
**Derive determiners and possessive noun phrases.**

Migrate D01 from `docs/english-derived-family-inventory.md`:
`determiner_closed`, `determiner_target`, `determiner_quantified_target`,
`determiner_quantity`, `determiner_possessive_this_card`,
`possessive_noun_base`, `possessive_noun_determined`,
`determiner_possessive_noun`, and `possessive_noun_adjective`.

Use the landed scalar, identity, and lens holes; add the first required
article/onset, cardinality, demonstrative-agreement, and already-determined
selection constraints. Keep meaning-bearing possessor/demonstrative identity
and exact self-reference form while deriving `a`/`an` only under the measured
exactness law.

Land declarations and generated parse/reduction/lowering, total render, and
validated build projections; all spelling and serialized-view consumer
changes; direct-AST/`inspect`/exactness/negative fixtures; all nine registry
flips; and deletion of handwritten mirrors and raw constructor paths in one
change. Update the inventory. No consumer or deletion work is deferred.

Standard constraints apply.

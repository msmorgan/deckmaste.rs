---
needs: [english-derived-family-inventory, english-derived-predicate-family]
---
**Derive finite, existential, and copular clauses.**

Migrate F02 from `docs/english-derived-family-inventory.md`:
`simple_clause_subject`, `simple_clause_subject_distributive_each`,
`simple_clause_contracted_subject`, `simple_clause_subjectless`,
`clause_simple`, `clause_elliptical`, `clause_existential`,
`copular_remainder_noun`, `copular_remainder_adjective`,
`copular_remainder_prepositional`, `copular_remainder_power_toughness`,
`copular_remainder_prepositional_adjunct`, `copular_remainder_adverb`,
`copular_remainder_negated`, `copular_remainder_distributive_each`,
`clause_copular`, `clause_contracted_copular`, and
`clause_variable_value_constraint`.

Use the predicate valency machinery and add agreement, distributive-each,
contraction, existential-number, copular-complement, and variable-value
constraints. Preserve the `clause_simple` dominance edge and every exact
contraction/existential distinction.

Land declarations and generated parse/reduction/lowering, total render, and
validated build projections; all Sentence/Ability/Cost and spelling/view
consumers; direct-AST and `inspect` agreement/dominance fixtures; exactness and
invalid-combination negatives; all 18 registry flips; and deletion of
handwritten mirrors and raw constructors together. Update the inventory. No
consumer or deletion work is deferred.

Standard constraints apply.

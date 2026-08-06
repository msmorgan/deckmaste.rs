---
needs: [english-derived-family-inventory, english-derived-finite-clause-family, english-derived-phrase-coordination-family]
---
**Derive clause coordination and shared continuations.**

Migrate F04 from `docs/english-derived-family-inventory.md`:
`clause_coordination`, `clause_coordination_comma`,
`clause_coordination_asyndetic`,
`clause_coordination_copular_noun_prepositional`,
`clause_coordination_copular_noun_prepositional_comma`, and
`clause_coordination_copular_noun_prepositional_asyndetic`.

Use the phrase-coordination C4 backend for the tuple-valued boundary created
by flattened shared-subject predicates and shared-copular continuations.
Preserve finite agreement, imperative adoption, conjunction class, member
scope, `ClauseCoordination`'s stored comma, and every legitimate packed
complete-member versus shared-predicate alternative. The structural-design
per-member trailing conditions and shared-subject `A, B, then C` chains land
here, not as residue.

Land declarations and generated parse/reduction/lowering, total render, and
validated build projections; all Sentence/Ability/spelling/view consumers;
direct-AST and verbose-`inspect` scope/agreement fixtures; exact punctuation
replay and invalid-agreement/conjunction/shared-subject/copula negatives; all
six registry flips; and deletion of CR's general registrations, NR's late
shared-copular registrations, CR's substantive reducer/lowerer arms, every
renderer mirror, and every constructor bypass together. Update the inventory.
No consumer or deletion work is deferred.

Standard constraints apply.

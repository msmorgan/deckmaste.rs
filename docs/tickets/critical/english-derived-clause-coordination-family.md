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

Reuse the phrase-coordination sequence, junction, feature-flow, and projection
machinery on the ordinary fan-out-one Chart backend. There is no tuple-valued
boundary: `FiniteClause` owns a shared subject and agreement once while
`PredicateExpression::Coordinated` owns contiguous predicate members;
`IndependentClause::Coordinated` owns complete members. Shared-copular
continuations are ordinary copular predicate members. Preserve finite
agreement, imperative adoption, conjunction class, recursive grouping, member
scope, `ClauseCoordination`'s stored comma, and every legitimate packed
complete-member versus shared-predicate alternative.

Land the structural-design scope here. Tek is one n-ary shared-subject
predicate coordination whose five predicates each own their trailing `as long
as` attachment; the parallel all-members-attached form locally dominates a
reading that moves only the final condition outside. Chaos Mutation is an
asyndetic/`then` shared-subject predicate chain with the `until` dependent on
its reveal member. Sycorax Commander is an outer complete-clause `or` whose
first member contains the inner shared-subject `then` chain. Do not flatten
that grouping. Dominaria's Judgment remains separate keyword-argument/verb
ellipsis work.

Land declarations and generated parse/reduction/lowering, total render, and
validated build projections; all Sentence/Ability/spelling/view consumers;
direct-AST and verbose-`inspect` scope/agreement fixtures; exact punctuation
replay and invalid-agreement/conjunction/shared-subject/copula negatives; all
six registry flips; and deletion of CR's general registrations, NR's late
shared-copular registrations, CR's substantive reducer/lowerer arms, every
renderer mirror, and every constructor bypass together. Update the inventory.
No consumer or deletion work is deferred.

Use exact supported fixtures for Tek, Chaos Mutation, and Sycorax Commander,
plus direct checked-AST fixtures for both nested associations and normal/
reversed construction registration. A trailing condition that occurs before a
junction must stay member-local; a lone final trailing condition without the
parallel-member discriminator may remain packed as local versus group scope.

Standard constraints apply.

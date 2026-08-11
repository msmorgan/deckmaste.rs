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
The Chart/backend conclusion in the structural decision remains applicable;
the scope and grouping rules below supersede its earlier final-condition
discriminator where they conflict.

Land the structural-design scope here. Match the corpus-derived surface
convention recorded under
[`Conditions`](../../oracle-style-guide.md#conditions): Tek is one n-ary
shared-subject predicate coordination whose five predicates each own their
repeated postpositive condition. A single postpositive condition after two
coordinated predicates is group-wide, as on Backwoods Survivalists and Tuinvale
Guide. When a shared condition governs three or more predicates, Oracle fronts
it before the serial-comma list, as on Dragon's Rage Channeler; do not
manufacture a tied member-local reading for the final predicate. Sentence and
quoted-ability boundaries prevent attachment across them.

Chaos Mutation is one n-ary asyndetic/`then` predicate chain because its three
predicate members share the one overt subject; the `until` dependent remains
inside its reveal member. Sycorax Commander is an outer complete-clause `or`:
the overt subject `this creature` starts the second clause, while the first
clause contains the inner shared-subject `then` chain. Preserve every junction
inside its grammatical layer and do not flatten either structure.
Dominaria's Judgment remains separate keyword-argument/verb ellipsis work.

Land declarations and generated parse/reduction/lowering, total render, and
validated build projections; all Sentence/Ability/spelling/view consumers;
direct-AST and verbose-`inspect` scope/agreement fixtures; exact punctuation
replay and invalid-agreement/conjunction/shared-subject/copula negatives; all
six registry flips; and deletion of CR's general registrations, NR's late
shared-copular registrations, CR's substantive reducer/lowerer arms, every
renderer mirror, and every constructor bypass together. Update the inventory.
No consumer or deletion work is deferred.

Use exact supported fixtures for Tek, Tribal Golem, Backwoods Survivalists,
Tuinvale Guide, Dragon's Rage Channeler, Chaos Mutation, and Sycorax Commander,
plus direct checked-AST fixtures for both nested associations and normal/
reversed construction registration. Repeated postpositive conditions stay
member-local; a lone postpositive `as long as` after two coordinated predicates
is group-wide; a three-member shared condition is preposed. Add quotation-
boundary coverage with Giant's Amulet and reject any flattening that assigns
Sycorax Commander's explicit second subject to the first predicate layer.

Standard constraints apply.

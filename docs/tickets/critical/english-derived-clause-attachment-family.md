---
needs: [english-derived-family-inventory, english-derived-finite-clause-family, english-derived-nonfinite-clause-family, english-derived-phrase-coordination-family]
---
**Derive complex clause coordination and attachment.**

Migrate F03 from `docs/english-derived-family-inventory.md`:
`clause_coordination`, `clause_coordination_comma`,
`clause_coordination_asyndetic`,
`clause_coordination_copular_noun_prepositional`,
`clause_coordination_copular_noun_prepositional_comma`,
`clause_coordination_copular_noun_prepositional_asyndetic`,
`clause_adverb_before`, `clause_sentence_adverbial_before`,
`clause_prepositional_before`, `clause_subordinate_before`,
`clause_subordinate_gerund_before`, `clause_subordinate_after_elliptical`,
`clause_subordinate_after`, `clause_subordinate_after_comma`,
`clause_subordinate_after_infinitive`, `exception_rider_single`,
`exception_rider_conjoined`, `exception_rider_comma`,
`exception_rider_oxford`, `clause_excepted`, `clause_restriction_run`, and
`clause_restriction_member`.

Use the phrase-coordination C4 backend for shared and member-scoped yields and
the finite/nonfinite constraints for attachment owners, positions, forms,
subordinators, exception hosts, and restriction roles. The structural-design
per-member trailing conditions and shared-subject `A, B, then C` chains land
here, not as residue. Preserve `ClauseCoordination`'s stored comma and every
legitimate packed scope alternative.

Land declarations and generated parse/reduction/lowering, total render, and
validated build projections; all Sentence/Ability/spelling/view consumers;
direct-AST and verbose-`inspect` scope fixtures; exact punctuation replay and
invalid-host/arity/subordinator negatives; all 22 registry flips; and deletion
of every handwritten mirror and constructor bypass together. Update the
inventory. No consumer or deletion work is deferred.

Standard constraints apply.

---
needs: [english-derived-family-inventory, english-ability-construction-backend, english-derived-adjective-comparison-family, english-derived-clause-attachment-family, english-derived-clause-coordination-family, english-derived-determiner-possession-family, english-derived-finite-clause-family, english-derived-nominal-family, english-derived-nonfinite-clause-family, english-derived-noun-lexeme-family, english-derived-noun-phrase-family, english-derived-phrase-coordination-family, english-derived-predicate-family, english-derived-prepositional-family, english-derived-quantity-family, english-derived-relative-clause-family, english-derived-sentence-family]
---
**Retire the English derived-grammar migration ratchet.**

Final audit for `docs/decisions/english-grammar-is-derived.md`. The family
inventory adds every minted chart-family migration to this ticket's `needs:`,
so this node cannot become ready while any supported family remains assigned
to handwritten ownership.

- Prove every supported registry family is generated and delete the
  handwritten-owner branch, mixed-registry migration switches, duplicate
  registration paths, and temporary boundary conversions.
- Prove every invariant-bearing migrated AST has sealed generated construction
  and validated deserialization, with no raw construction in active consumers.
- Confirm every discontinuous family records and tests either its PMCFG backend
  or measured CFG approximation boundary.
- Run corpus-wide round-trip, recovery and ambiguity census review, direct-AST
  and `inspect` suites, registration permutation, frame compile/unify/render,
  and the structural no-mirror audit.

Completion means no paired handwritten grammar/renderer authority remains and
the registry is ordinary generated construction metadata rather than a
migration ratchet.

Standard constraints apply.

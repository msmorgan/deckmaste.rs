---
needs: [english-derived-family-inventory, english-derived-predicate-family]
---
**Derive subject-, object-, and copular-relative clauses.**

Migrate R01 from `docs/english-derived-family-inventory.md`:
`relative_object`, `relative_object_contracted_subject`,
`relative_subject_contracted_auxiliary`, `relative_subject`,
`relative_subject_distributive_each`, `relative_contracted_copular_noun`,
`relative_contracted_copular_adjective`, and
`relative_contracted_copular_prepositional`.

Use predicate valency and add explicit subject/object gap, relativizer,
agreement, contraction, and distributive-each constraints. Preserve the
`relative_subject` over `relative_object` dominance edge; zero relativizers
and demonstrative `that` remain structurally distinct.

Land declarations and generated parse/reduction/lowering, total render, and
validated build projections; all nominal/clause/spelling/view consumers;
direct-AST and verbose-`inspect` gap fixtures; exact zero/explicit/contraction
replay; impossible-gap/agreement negatives; all eight registry flips; and
deletion of handwritten mirrors and bypassing constructors together. Update
the inventory. No consumer or deletion work is deferred.

Standard constraints apply.

---
needs: [builtin-v2-grammar-consumer]
---
Author and commit the noncreature subtype records under
`plugins/builtin_v2/macros/stubs/subtypes`, one category-scoped directory and
one record per entry in each canonical catalog the semantic model can
currently represent: artifact, battle, enchantment, land, planeswalker, and
instant/sorcery spell subtypes [CR#205.3]. Follow the ratified
[builtin-v2 spelling/grammar ADR](../../decisions/builtin-v2-macro-spelling-and-grammar.md).

Each record declares its catalog identity, exact subtype spelling, supported
semantic category, and only attested noun morphology. Category eligibility is
semantic registry data: artifact, battle, enchantment, land, and planeswalker
subtypes name their corresponding card type; spell subtypes admit Instant and
Sorcery. Grammar data never decides category legality. Existing rules-defined
conferrals remain on the same declaration; do not infer new behavior from a
name.

Check casing, count/mass behavior, and number forms against the CR and
supported Oracle corpus, following style guide §7, "Types, subtypes, and
supertypes as nouns and modifiers." In particular, artifact subtypes are not
uniformly count nouns, proper-name planeswalker subtypes are not ordinary
common nouns, and hyphenated or possessive catalog spellings are retained.
Omit unattested morphology rather than deriving it mechanically from title
case.

Planar and dungeon subtypes are named semantic-model gaps, not covered
inventory: the current subtype category axis cannot represent them. Do not
silently treat the six directories here plus the creature sibling as complete
CR subtype coverage. The files are committed source code and graduate in
place; do not generate them or create per-subtype parser constructions.
Acceptance audits at least one record from every supported category plus the
mass/count, hyphen, and multi-type spell boundaries, and loads every record
through the v2 declaration reader. Standard constraints apply.

---
needs: [builtin-v2-spelling-stub-design]
---
Author and commit the noncreature subtype records under
`plugins/builtin_v2/macros/stubs/subtypes`, one category-scoped directory and
one record per entry in each canonical catalog the semantic model can
currently represent: artifact, battle, enchantment, land, planeswalker, and
instant/sorcery spell subtypes [CR#205.3]. Follow the ratified
[builtin-v2 spelling/grammar ADR](../../decisions/builtin-v2-macro-spelling-and-grammar.md).

These files are source code and graduate in place; do not generate them. Check
their lexical, casing, and number facts against authoritative grammar evidence
rather than deriving them mechanically from catalog title case.

Planar and dungeon subtypes are named semantic-model gaps, not covered
inventory: the current subtype category axis cannot represent them. Do not
silently treat the six directories here plus the creature sibling as complete
CR subtype coverage. Standard constraints apply.

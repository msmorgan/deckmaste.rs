---
needs: [builtin-v2-spelling-stub-design]
---
Author and commit the subtype records under
`plugins/builtin_v2/macros/stubs/subtypes`, one category-scoped directory and
one record per entry in every canonical subtype catalog the semantic model can
currently represent: artifact, battle, creature/kindred, enchantment, land,
planeswalker, and instant/sorcery spell subtypes [CR#205.3]. Follow the
ratified [builtin-v2 spelling/grammar ADR](../../decisions/builtin-v2-macro-spelling-and-grammar.md).

These files are source code and graduate in place; do not generate them. Verify
number morphology exhaustively against authoritative grammar evidence—catalog
membership is not evidence that `+s` is valid (`Merfolk` is plural `Merfolk`,
not `Merfolks`). Do not guess unattested forms.

Planar and dungeon subtypes are named semantic-model gaps, not covered
inventory: the current subtype category axis cannot represent them. Do not
silently treat the seven supported category directories as complete CR subtype
coverage. Standard constraints apply.

---
needs: [builtin-v2-spelling-stub-design]
---
Author and commit the creature/kindred subtype records under
`plugins/builtin_v2/macros/stubs/subtypes/creature`, one for each canonical
`data/gen/catalogs/creature-types.txt` entry, following the ratified
[builtin-v2 spelling/grammar ADR](../../decisions/builtin-v2-macro-spelling-and-grammar.md).

This is one category-scoped part of the subtype inventory, not the boundary of
the open-registry design. The sibling `builtin-v2-noncreature-subtype-stubs`
ticket owns every other subtype category the semantic model can currently
represent.

These files are source code and graduate in place; do not generate them. Verify
number morphology exhaustively against authoritative grammar evidence—catalog
membership is not evidence that `+s` is valid (`Merfolk` is plural `Merfolk`,
not `Merfolks`). Do not guess unattested forms. Standard constraints apply.

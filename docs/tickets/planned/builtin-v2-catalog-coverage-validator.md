---
needs: [builtin-v2-keyword-action-stubs, builtin-v2-keyword-ability-stubs, builtin-v2-subtype-stubs]
---
Add a read-only completeness gate for every catalog-backed `builtin_v2`
inventory covered by the ratified
[builtin-v2 spelling/grammar ADR](../../decisions/builtin-v2-macro-spelling-and-grammar.md):
keyword actions, keyword abilities, and all seven currently representable
subtype categories.

The gate checks only the catalog-line ↔ category-scoped filename bijection,
including collisions and the ADR's reviewed exceptional character mappings.
It must never write files, generate records, replace authored contents,
byte-compare record bodies, or prescribe morphology. It must not treat
`counter-kind-phrases.txt` as a counter declaration inventory, and it must not
invent closed catalogs for types or designations. Standard constraints apply.

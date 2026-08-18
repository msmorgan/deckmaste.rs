---
needs: [builtin-v2-keyword-action-stubs, builtin-v2-keyword-ability-stubs, builtin-v2-creature-type-stubs, builtin-v2-noncreature-subtype-stubs]
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
invent closed catalogs for types or designations.

Implement the comparison as a reusable read-only library check over
`deckmaste_catalogs::CatalogSet` plus an explicit table mapping each covered
`CatalogKind` to its category-scoped source directory. The identifier mapping
is the ADR's one total function: report unsupported punctuation, collisions,
missing files, extra `.ron` files, non-file entries, and a filename whose stem
does not equal the mapped identifier. Sort diagnostics by catalog kind and
source spelling so filesystem iteration order cannot affect output.

The checker inspects paths only. It does not deserialize a record or decide
whether its spelling, grammar, morphology, parameters, or body are correct.
Tests pin a clean miniature tree and one failure each for missing, extra,
collision, unsupported punctuation, category isolation, and the reviewed
`∞`/apostrophe/exclamation mappings. Wire the read-only check into the ordinary
repository validation gate after the inventories exist; expose no write or
"fix" mode. Standard constraints apply.

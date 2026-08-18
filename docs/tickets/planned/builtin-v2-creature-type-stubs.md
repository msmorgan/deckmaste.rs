---
needs: [builtin-v2-declaration-schema]
---
Author and commit the creature/kindred subtype records under
`plugins/builtin_v2/macros/stubs/subtypes/creature`, one for each canonical
`data/gen/catalogs/creature-types.txt` entry, following the ratified
[builtin-v2 spelling/grammar ADR](../../decisions/builtin-v2-macro-spelling-and-grammar.md).

This is one category-scoped part of the subtype inventory, not the boundary of
the open-registry design. The sibling `builtin-v2-noncreature-subtype-stubs`
ticket owns every other subtype category the semantic model can currently
represent.

Each record declares the catalog identity, the Creature/Kindred subtype
category, the exact singular subtype spelling, and only attested noun
morphology. The identity and category are semantic registry data; `grammar`
owns realization facts such as plural or invariant number. Empty conferrals
remain empty. A subtype with rules-defined conferrals keeps those on the same
declaration rather than acquiring a second registry row.

Verify morphology against the CR and supported Oracle corpus, following style
guide §7, "Types, subtypes, and supertypes as nouns and modifiers."
Catalog membership is not evidence that `+s` is attested: `Merfolk` is
plural `Merfolk`, not `Merfolks`. Omission deliberately selects the dumb
`singular + "s"` default. Cover the two-word `Time Lord`, apostrophe mapping,
regular plurals, irregular plurals, invariants, and a genuinely unattested
plural. Use an explicit replacement for an attested exception and the explicit
unavailable state plus an evidence note for an unattested form; do not guess.

These files are committed source code and graduate in place; do not generate
them or create predicate twins per subtype. Acceptance loads every authored
record through the v2 declaration reader and audits the representative
morphology cases above, including rejection of a redundant default-equal
override; the catalog coverage ticket separately owns only the filename
bijection. Standard constraints apply.

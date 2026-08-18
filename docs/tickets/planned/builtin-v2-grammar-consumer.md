---
needs: [builtin-v2-english-card-consumer, builtin-v2-catalog-coverage-validator, builtin-v2-type-declarations, builtin-v2-counter-kind-declarations, builtin-v2-designation-declarations]
---
Perform the final builtin-v2 grammar cutover after the declaration-backed
inventories and each independently reviewable consumer layer are complete.
This is a narrow integration gate for the
[builtin-v2 spelling/grammar ADR](../../decisions/builtin-v2-macro-spelling-and-grammar.md),
not the former all-in-one schema/loader/parser/frame/CardSource epic.

Load the completed `builtin_v2` source tree through the ordinary
dependency-closure provider, build the immutable normalized grammar
environment, and make that environment the builtin parser's default source.
Remove the narrow builtin-only bootstrap reader and every remaining
official-corpus static identity or surface table. Closed structural grammar and
static semantic constructions may remain, but they must request open
declaration IDs and must not own duplicate registry membership, morphology,
valence, casing, or render surfaces.

Run the read-only catalog coverage gate for keyword actions, keyword abilities,
and the seven supported subtype categories, plus manifest/loader coverage for
types, counter kinds, and designations. These checks certify committed source
completeness only. Do not enrich the environment from a catalog, add a
generated stub manifest, or reject an open plugin declaration because it is
absent from official data.

Exercise the complete runtime path over the builtin plugin and representative
Wizards corpus: provider merge, environment construction, English parsing and
rendering, positional frame selection, semantic assembly, and Idris
validation. Nursery-only or semantically unsupported records may contribute
grammar but must remain unselectable as semantic frames. Preserve every
surface collision for ordinary ambiguity handling.

Acceptance proves there is exactly one provider path and one normalized
environment ABI; no runtime consumer reads `ParserCatalogs` or plugin files;
no public registry owner ID requires `&'static str`; the full committed
builtin inventory loads deterministically; representative official text still
round-trips; and a synthetic noncatalog plugin declaration still parses under
the same default path. Standard constraints apply.

---
needs: [english-v2-number-feature-unification]
---
**Collapse the six `genitive_determiner_*` constructions, whose discriminating
guards are inert.** `genitive_determiner_{singular,plural,mass}_reference` and
their `plural_genitive_determiner_*` twins share the form `possessor nominal`
and byte-identical `derive` blocks; each pair differs only by
`require possessor.number is Singular` versus `… is Plural`. `possessor.number`
feeds no `derive`, and `{Singular, Plural}` is the whole `Number` domain, so
each pair is an exhaustive partition with no observable consequence: a
possessive of any number followed by a singular nominal parses once either
way.

Pinned shape: after number unification there is one
`genitive_determiner_reference: UnqualifiedReference { possessor: Possessive,
nominal: Nominal }` with the existing derives, plus
`genitive_determiner_coordination_reference` unchanged. If this lands before
the unification, collapse to three (singular / plural / mass nominal) by
deleting the `possessor.number` requires; never re-encode possessor number as
an AST product distinction. The `'s`/`'` choice stays in `construction
possessive`. Audit `possessed_{singular,plural,mass}_reference` (form
`lex(possessor) nominal`) for the same fold under the same test: a guard that
reads a feature no derive consumes and partitions its whole domain is inert.

Fence: a possessor-number distinction retained "for the downstream layer" is a
STOP; the downstream layer reads `possessor.number` from the `Possessive` node.

Acceptance: construction count down by the folds, selection census unchanged
for every possessive identity, byte-exact laws green. Standard constraints
apply.

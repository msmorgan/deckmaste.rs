---
needs: [english-v2-concord-class, english-v2-grammatical-relations, english-v2-number-feature-unification]
---
**Strengthen English agreement to explicit Person and Number features.** Build
on the weak Concord Class split using the [`Person`, `Number`, `Agreement`, and
`Concord Class` definitions](../../contexts/oracle-english/CONTEXT.md). Carry underlying Person and
Number on the agreeing nominal/pronominal values and derive the morphological
Concord Class used by verb realization.

Update construction constraints, coordination behavior, pronoun realization,
provider rows, parser consistency checks, and direct AST tests. Cover at least
*you cast*, *a player casts*, *players/they cast*, and the supported *was/were*
contrasts. Keep Person and Number available for later pronoun and possessive
agreement rather than collapsing them back into the two-way derived class.

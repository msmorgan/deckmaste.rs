---
needs: []
---
[design] **Stop silently choosing a referent's remembered type by the order of
type conjuncts.** In `lean/Semantics/Check/Phrase.lean`, `Predicate.seedTyAll`
keeps the first type found; `Payload.object` stores one `Option CardType`.
Lean LSP confirmed that after `target (.and [artifact, creature])`,
`that (.type .creature)` refuses with zero antecedents, but after
`target (.and [creature, artifact])` it accepts. This is inherited from Idris,
not solely a port regression.

The semantics_v2 contract describes predicates as flat sibling modifier sets.
Decide whether bindings preserve all relevant written type facts or whether a
distinguished grammatical head must be explicit in syntax. Do not introduce an
undocumented first-conjunct convention. The selected representation must also
account for disjunctions and the current carrier after movement.

Done when the two pure type conjunctions support the intended same reads, or
an explicit head distinction explains their difference; the choice is recorded
in the contract; and positive/negative anaphora pins cover both type words,
movement, and joined kinds. Prove order invariance for pure type modifiers if
the set interpretation is retained. Do not assume arbitrary modifiers with
binding effects commute. Preserve the existing card and pin assertions.

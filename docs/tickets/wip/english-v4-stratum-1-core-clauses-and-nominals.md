---
needs: [english-v4-stratum-0-card-frame-and-headers]
---
# Stratum 1: Core imperative/declarative clauses and NP/PP structure

Implement **Stratum 1** in `crates/deckmaste_english_v4` per
[`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§4, §6.1):

- **Layered `Nominal` and `NounPhrase`:** Core nouns, compound nouns, card/token
  names (`~`, self-reference shortened names `[CR#201.5,201.5c]`, and
  `named <CardName>`), layered premodification (adjectives, color words,
  card/subtype nouns, participial premodifiers), determiner selection (articles,
  demonstratives, possessives, basic quantifiers, numerals, `target`,
  `another`), and PP postmodifiers.
- **Frame-Driven `PrepositionPhrase`:** Built from `prepositions.ron` complement
  frames and unselected host-category distribution (`Nominal` postmodifier,
  `Predicate`/`Clause` adjunct, or `ComplementOnly`), with zero bespoke
  PP-licence tables.
- **Core Active Clauses:** Single-clause imperative and finite present/past
  declarative clauses driven by active verb frames (`Intransitive`,
  `Monotransitive`, `Prepositional`, `TransitiveWithPP`), admitting all
  `[CR#701.1]` keyword-action verbs purely through `keyword_actions.ron`.
- **Same-Head Complement Priority & Free Adjuncts:** Frame-selected complements
  take priority over unselected adjuncts on the same verb instance, while
  cross-head attachment alternatives are preserved in the packed forest.

## Gate

- 100% of corpus units in Strata 0–1 parse as **read** (0 `known-wrong-only`)
  and satisfy both round-trip laws (`v is an admitted Reading of s => render(v) = s`
  and `v is independently constructed and grammatical => v is a Reading of parse(render(v))`),
  including activated/labelled/modal abilities from Stratum 0 whose costs and
  effects are Stratum 1 clauses.
- Stratum 0–1 judgment suite passes with zero regressions.

Standard constraints apply.

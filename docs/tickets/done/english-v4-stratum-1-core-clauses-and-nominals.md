---
needs: [english-v4-stratum-0-card-frame-and-headers]
---
# Stratum 1: Core imperative/declarative clauses and NP/PP structure

Implement **Stratum 1** in `crates/deckmaste_english_v4` per
[`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§4, §6.1):

- **Layered `Nominal` and `NounPhrase`:** Core nouns, compound nouns, card/token
  names (`~` and self-reference shortened names `[CR#201.5,201.5c]`),
  layered premodification (adjectives, color words,
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

## Owner ruling

`target`, `enchanted`, and `equipped` have explicitly declared Determinative
Lexemes in Oracle English. Keyword-associated determinatives belong to their
catalog items; other lexical analyses remain available in their own uses.
This supersedes the Modifier-only assertions for `equipped creature` and
`enchanted land`. It does not license arbitrary participles as determiners.
Additional determinatives require evidence and an explicit declaration.

## Gate

`named <CardName>` belongs to Stratum 3 with the other reduced participial
postmodifiers, as confirmed by the owner; it is not a Stratum 1 exception.

- 100% of corpus units in Strata 0–1 parse as **read** (0 `known-wrong-only`)
  and satisfy both round-trip laws (`v is an admitted Reading of s => render(v) = s`
  and `v is independently constructed and grammatical => v is a Reading of parse(render(v))`),
  including activated/labelled/modal abilities from Stratum 0 whose costs and
  effects are Stratum 1 clauses.
- Stratum 0–1 judgment suite passes with zero regressions.

Standard constraints apply.

Census delta (oxukttzv): faces read 2,363 → 8,233; total Readings 52,591 → 102,767; lost faces: none. All 56,490 Strata 0–1 units read, zero known-wrong-only or law failures; full census followed by reprofiling all units and reparsing all 76 units affected by the final transformed Lexeme addition.
Judgment set (oxukttzv): cumulative Strata 0–1 and seed judgments pass; both independent judges report no remaining HIGH findings. Tests and clippy pass for deckmaste_english_v4, deckmaste_lexical_source_v4, and deckmaste_syntax; citation checks pass.

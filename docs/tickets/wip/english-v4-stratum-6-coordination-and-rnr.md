---
needs: [english-v4-stratum-5-comparatives-measures-and-partitives, english-v4-stratum-5-residual, english-v4-intern-declaration-identities, english-v4-zero-alloc-chart-state, english-v3-disable-frozen-tests]
---
# Stratum 6: Coordination and Right-Node Raising

Implement **Stratum 6** in `crates/deckmaste_english_v4` per
[`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§3, §4.1, §6.1):

- **Binary and Oxford-Comma Serial Coordination:** `A and B`, `A or B`, `A
  and/or B`, and 3+-member Oxford-comma chains (`A, B, and C`; `A, B, or C`;
  `A, B, and/or C`) across `Clause`, `Predicate`, `NounPhrase`, `Nominal`,
  `PrepositionPhrase`, `AdjectivePhrase`, `AdverbPhrase`, `DeterminerPhrase`,
  and `Cost`, plus comma/semicolon-coordinated sequences where licensed by
  Oracle punctuation rules.
- **Correlative Coordination:** `both A and B`, `either A or B`, `neither A nor
  B`, and `not only A, but also B`.
- **Compiler-Derived Right-Node Raising (Shared Complements):** Shared right-edge
  complement satisfaction across coordinated verbs, prepositions, adjectives,
  and nouns (`enters or leaves [the battlefield]`, `put into your hand or exile
  [target card]`, `greater than or equal to [X]`).

## Gate

- 100% of corpus units in Strata 0–6 parse as **read** (0 `known-wrong-only`)
  and satisfy both round-trip laws (`v is an admitted Reading of s => render(v) = s`
  and `v is independently constructed and grammatical => v is a Reading of parse(render(v))`).
- Stratum 0–6 judgment suite passes with zero regressions.

Standard constraints apply.

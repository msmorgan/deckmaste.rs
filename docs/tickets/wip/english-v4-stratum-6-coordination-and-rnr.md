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

## Owner ruling: accepted wrong Readings

Against `docs/decisions/english-lexical-analysis.md` (line 97: a newly
admitted wrong Reading is a defect, never a gain), the owner explicitly
accepted the attachment ambiguities that serial unlike Object lists add. In
each, a first Noun Phrase conjunct takes a keyword through its own *with* or
*from among* phrase, which syntax cannot settle. Derivations before → after:
Riftmarked Knight 36 → 72; Liberated Livestock 12 → 36; Odric, Blood-Cursed
36 → 52; Mascot Exhibition 8 → 24; Aragorn, Company Leader 9 → 18; Grimdancer
5 → 9; Triplicate Titan 1 → 4; Trostani's Summoner 1 → 4; Hungry for More
2 → 4; Greymond, Avacyn's Stalwart 2 → 3. The owner likewise accepted twelve
units that keep a narrow Targeting Marker reading beside the wide one, owned
by `english-v4-stratum-7-discourse-and-long-tail`.

1. **Census delta** (measured on `yqpnnypk`): faces read 20,391 → 28,709;
   total Readings 173,844 → 399,377; lost faces: Bladehold War-Whip
   (deferred to Stratum 7 by the owner).
2. **Judgment set**: 2,981 enforced at `COMPLETED_STRATUM` 6, all hold; 189
   pending.


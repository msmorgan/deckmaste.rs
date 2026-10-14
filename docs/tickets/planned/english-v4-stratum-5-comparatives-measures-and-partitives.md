---
needs: [english-v4-stratum-4-subordinate-clauses-and-triggers]
---
# Stratum 5: Comparatives, measures, partitives, and arithmetic modifiers

Implement **Stratum 5** in `crates/deckmaste_english_v4` per
[`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§6.1):

- **Partitives and Number-Transparent Quantificational Nouns:** `one of`, `all
  of`, `each of`, `both of`, `any of`, `none of`, `half of`, `the number of`,
  `any number of`, `a pair of`, `a pile of`, with CGEL number transparency and
  countability agreement.
- **Comparatives, Superlatives, and Equatives:** `equal to`, `greater than`,
  `less than`, `more ... than`, `fewer ... than`, `or greater`, `or less`, `or
  more`, `or fewer`, `at least`, `at most`, `up to`, `as much/many ... as`,
  `the most`, `the least`, `the greatest`, `the fewest`, `the highest/lowest`.
- **Measure Phrases, Arithmetic Modifiers, and Distribution:** `plus`, `minus`,
  `times`, `divided by`, `rounded up/down`, `for each ...`, quantificational
  adjuncts (`each`), `apiece`, and `divided as you choose among ...`.

## Gate

- 100% of corpus units in Strata 0–5 parse as **read** (0 `known-wrong-only`)
  and satisfy both round-trip laws (`v is an admitted Reading of s => render(v) = s`
  and `v is independently constructed and grammatical => v is a Reading of parse(render(v))`).
- Stratum 0–5 judgment suite passes with zero regressions.

Standard constraints apply.

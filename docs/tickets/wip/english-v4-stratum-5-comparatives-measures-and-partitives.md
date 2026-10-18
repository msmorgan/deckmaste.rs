---
needs: [english-v4-stratum-4-residual]
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


Adjectival *less* (CGEL Ch. 13 §4.1.1, p. 1126) is missing from the lexicon:
`closed_class.ron` declares *less* only as the determinative (the comparative
of *little*, *less money*, *ten mistakes or less*). Oracle's scalar *power
less than X* and *mana value 3 or less* contrast with adjectival *greater*
(*greater than X*, *3 or greater*), so they need the adjective, as the
judgments in `postpositive_comparative_property.txt` and
`spelled_cardinal_scalar_measure.txt` assert.

Standard constraints apply.


Census delta (`npnwlulw` → `pmvxkuzp`): faces read 18,805 → 20,352; total Readings 146,555 → 181,454; lost faces: none. Strata 0–5: 75,014 units read, 33 deferred to `english-v4-stratum-5-residual`, 0 known-wrong-only, 0 unread, 0 law failures.
Judgment set: seed and cumulative Strata 0–5 pass (2,362 enforced, 709 pending, one of them pending for a deferral); tests and clippy pass for deckmaste_syntax, deckmaste_lexical_source_v4 and deckmaste_english_v4; citation checks pass.

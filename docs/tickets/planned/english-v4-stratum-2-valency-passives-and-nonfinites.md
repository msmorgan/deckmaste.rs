---
needs: [english-v4-stratum-1-core-clauses-and-nominals]
---
# Stratum 2: Auxiliaries, copulas, passives, and non-finite complements

Implement **Stratum 2** in `crates/deckmaste_english_v4` per
[`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§4.1, §6.1):

- **Auxiliaries, Modals, and Negation:** `may`, `can`, `can't`, `must`, `will`,
  `would`, `do`/`does`/`did`/`don't`/`doesn't`/`didn't`, perfect `have`/`has`/`had`,
  progressive/passive `be`, and primary/secondary negation (`not`, `never`).
- **Copular, Predicative, and Existential Clauses:** `be`, `become`, `get`,
  `have`, `base power and toughness`, slash-measure predicatives (`+2/+2`,
  `2/2`), adjectival/nominal/PP predicative complements, and existential
  `there is/are`.
- **Ditransitive and Complex-Transitive Complements:** Resultative and
  predicative `as`/`into` complements (`becomes a copy of`, `enters tapped`).
- **Compiler-Derived Passives:** Finite passives (`is/are/was/were + V-en`) and
  reduced postnominal passive clauses (`each card exiled this way`, `damage
  dealt by ~`) derived automatically from active verb frames in `verbs.ron` and
  `keyword_actions.ron`.
- **Non-Finite Complements and Depictives:** Infinitival (`to + VP`, bare
  infinitival) and gerund-participial (`V-ing`) catenative complements and
  depictive adjuncts.

## Gate

- 100% of corpus units in Strata 0–2 parse as **read** (0 `known-wrong-only`)
  and satisfy both round-trip laws (`v is an admitted Reading of s => render(v) = s`
  and `v is independently constructed and grammatical => v is a Reading of parse(render(v))`).
- Stratum 0–2 judgment suite passes with zero regressions.

Standard constraints apply.

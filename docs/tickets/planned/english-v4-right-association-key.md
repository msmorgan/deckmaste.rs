---
needs: [english-v4-ranking-key-engine, english-v4-comma-adjunct-island, english-v4-relational-noun-of-complement, english-v4-fronted-temporal-pp-clause, english-v4-bare-relative-fronted-adjunct, english-v4-cost-not-passivizable, english-v4-keyword-phrase-adjunct-island, english-v4-auxiliary-adjunct-canonicalisation]
---
# RightAssociationWords: dependency length to the host

Fill the Ranking Key's **RightAssociationWords** component
(`docs/decisions/english-v4.md` §4.3).

## Change

- For every unselected dependent (every Adjunct and Modifier schema or
  production), charge the length in words of the host constituent it
  attaches to.
  - The engine computes this from spans; no production list is kept.
  - This is the distance from the dependent back to its host, which is the
    host's left edge for a head-initial Verb Phrase or Nominal.
- Coordinations: charge the difference in length between conjuncts (a
  conjunct-balance preference; CGEL, Ch. 15 §3.1, pp. 1323–1324, likeness of
  function).
- No charge when the host is itself a coordination.
- Dependents whose class or frame attaches high count their length reversed.
  The class is set by `english-v4-attachment-classes`, the frame by
  `english-v4-catalog-attachment-preferences`.

## Evidence from the cost study

- Right association alone (dependency length) raised single-cheapest units
  from 33.7% to 78.9%.
- It also moved a few equivalent-meaning ties: *the first time each turn*,
  *explores X times*. Adjudicate each judgment entry that changes tag. Rewrite
  one that pins an incidental choice to assert what is wanted, and say so.

## Done when

- Readings and Surplus are unchanged.
- Every single-group judgment entry whose top tier changes from all-endorsed
  to not endorsed is listed and adjudicated in the landing.

Standard constraints apply.

1. **Census delta**: measured at landing.
2. **Judgment set**: measured at landing.

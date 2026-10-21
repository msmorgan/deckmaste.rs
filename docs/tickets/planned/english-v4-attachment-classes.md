---
needs: [english-v4-ranking-key-engine, english-v4-right-association-key, english-v4-comma-adjunct-island, english-v4-relational-noun-of-complement, english-v4-fronted-temporal-pp-clause, english-v4-bare-relative-fronted-adjunct, english-v4-cost-not-passivizable, english-v4-keyword-phrase-adjunct-island, english-v4-auxiliary-adjunct-canonicalisation]
---
# AttachmentClass: structural attachment preferences

Fill the Ranking Key's **AttachmentClass** component
(`docs/decisions/english-v4.md` §4.3). It counts attachments that violate a
structural class, keyed on host kind and grammatical function, never on words.

## Classes

1. **A Supplement anchors to its clause.**
   - A `host_kind: Supplement` dependent (*, where X is …*) attaches to the
     outermost eligible clause and reverses its Right Association.
   - CGEL, Ch. 15 §5.1, p. 1351.
   - Fixes the *where*-clause entries in `where_variable_clause.txt`, which
     Right Association alone puts inside the Verb Phrase.
2. **A dependent after a coordination attaches to the whole coordination.**
   - This applies to a postmodifier after coordinated nominals (*a creature
     or planeswalker you control*, *Elves and Faeries you control*) and to an
     adjunct after coordinated predicates (*cast … or activate … this turn*).
   - It does not attach to the last conjunct alone.
   - Authority: `docs/oracle-style-guide.md` §6, Determiners and modifier
     order: "Put ownership and control after the full noun phrase".

## Out of scope

The blanket split between Nominal-hosted PP Adjuncts (cost 1) and
Predicate-hosted ones (cost 2) stays as ProductionCost.
- Under the ruled order the study measured no difference with it equalised:
  221 of 310 either way.
- The split is now subordinate to Right Association and the classes, so it no
  longer decides attachments.

## Acceptance

Measured against the judgment set (310 single-group entries with 2–4,000
Readings, at least one unendorsed), not the ten tuning units.

- With this ticket and its two predecessors, the ruled order simulated:
  - **≥ 219** entries with an all-endorsed top tier (today 94);
  - single-cheapest units **≥ 84%** (today 33.7%).
- Readings and Surplus are unchanged, and no face is lost.

Standard constraints apply.

1. **Census delta**: measured at landing.
2. **Judgment set**: measured at landing.

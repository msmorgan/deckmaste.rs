---
needs: [english-v4-ranking-key-engine, english-v4-right-association-key, english-v4-comma-adjunct-island, english-v4-omitted-idiom-complement-split, english-v4-fronted-temporal-pp-clause, english-v4-bare-relative-fronted-adjunct, english-v4-cost-not-passivizable, english-v4-keyword-phrase-adjunct-island, english-v4-auxiliary-adjunct-canonicalisation, english-v4-in-addition-to-predicate-host]
---
# AttachmentClass, and one base cost for Nominal and Predicate adjuncts

This ticket completes the structural Ranking Key
(`docs/decisions/english-v4.md` §4.3):

AttachmentClass → RightAssociationWords → ProductionCost → FinalTiebreak

It does two things:
- it fills **AttachmentClass**;
- it removes the blanket Nominal-vs-Predicate bias from ProductionCost.

## Owner's ruling (round 2)

The key is purely structural. Lexical and domain plausibility preferences
are dropped; they are soft policing. That includes:
- *cast* liking origins;
- *spend* liking purpose clauses;
- zone nouns disliking postmodifiers.

What remains is:
- equal Nominal and Predicate adjunct base cost;
- Right Association, with comma-set supplements attaching high;
- Production Cost;
- a deterministic tiebreak.

Caveat 4 (owner-approved) keeps the coordination rule.

## Classes

Each class is keyed on host kind, grammatical function and existing features,
never on words:

1. **High-attaching dependents.**
   - A comma-set Supplement (`host_kind: Supplement`, *, where X is …*)
     anchors to the outermost eligible clause (CGEL, Ch. 15 §5.1, p. 1351).
   - So does a distributive adjunct, a Preposition Phrase whose complement has
     a `Distributive` determinative (*for each …*).
   - These count their Right Association reversed.
   - This fixes the *where*-clause entries in `where_variable_clause.txt`, and
     *for each …* in `syncretic_word_forms.txt:3` and
     `predicative_gap_relatives.txt:5`.
2. **A dependent after a coordination attaches to the whole coordination.**
   - This applies to a postmodifier after coordinated nominals (*a creature or
     planeswalker you control*, *Elves and Faeries you control*) and to an
     adjunct after coordinated predicates.
   - It does not attach to the last conjunct alone.
   - Conjunct balance (the difference in length between conjuncts) is part of
     the same rule, charged in RightAssociationWords.
   - Authority: `docs/oracle-style-guide.md` §6, Determiners and modifier
     order: "Put ownership and control after the full noun phrase"; CGEL,
     Ch. 15 §3.1, pp. 1323–1324 (likeness of function).

## Equal base cost

- The PP Adjunct Schemas cost 1 when Nominal-hosted and 2 when
  Predicate-hosted (`english.ron`, the `Adjunct(…)` block).
- Give them one cost. ProductionCost then encodes no blanket preference
  between the hosts; Right Association decides.

## No new features

No new features in `english.ron`. If a class cannot be stated with existing
features, STOP and return to the owner.

## Acceptance

Re-simulated for this round on the current line (`pzrkntzo`), for the full
structural key. Measured against the judgment set (310 single-group entries
with 2–4,000 Readings, at least one unendorsed), not the ten tuning units.

The simulation assumes these defect fixes have landed:
- `english-v4-comma-adjunct-island`, both nominal-internal and non-finite;
- `english-v4-fronted-temporal-pp-clause`;
- `english-v4-bare-relative-fronted-adjunct`;
- `english-v4-cost-not-passivizable` (*cost* only);
- `english-v4-in-addition-to-predicate-host`.

It does **not** model:
- `english-v4-omitted-idiom-complement-split`;
- `english-v4-keyword-phrase-adjunct-island`;
- `english-v4-auxiliary-adjunct-canonicalisation`;
- *have*'s passive.

Classes 1–2 were simulated as word-weighted terms inside the Right
Association tier, not as a separate higher tier.

Targets:
- **≥ 218 of 310** entries with an all-endorsed top tier (today 94).
  - The same key without assuming any fix gave 219.
- Single-cheapest units **≥ 84.6%** of the 17,890 ambiguous units (today
  33.7%). The largest cheapest tier is 8 Readings (today 6,354).
- Every entry whose top tier moves into "no endorsed Reading" is adjudicated.
  The simulation had 25 such moves against 134 improvements.
  - Wrong judgments are rewritten (`english-v4-wrong-judgments-and-np-and-dup`).
  - Equivalent-meaning ties settle by the FinalTiebreak, and their entries
    assert what is wanted.
- Readings and Surplus are unchanged, and no face is lost.
- For reference only: 2 of the 10 tuning units rank the intended Reading
  uniquely first (Spark Double, Moonhold).

Standard constraints apply.

1. **Census delta**: measured at landing (Readings and Surplus unchanged).
2. **Judgment set**: measured at landing.

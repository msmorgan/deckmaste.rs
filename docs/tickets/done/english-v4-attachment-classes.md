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

## Landing

What landed:
- **High-attaching dependents.** A Supplement, and a Preposition Phrase whose
  `DeterminativeDistribution` is `Distributive` (*for each …*), count their
  Right Association reversed. The reversal covers the host's words and the
  words between the host and the dependent.
- **Coordination class.** The Attachment Class counts a coordination's last
  conjunct that ends in a dependent while its first conjunct ends in its
  head. Conjunct balance and the free coordination host are unchanged from
  `english-v4-right-association-key`, pending the owner's ruling.
- **Equal base cost.** Every Predicate-hosted PP Adjunct Schema now costs 1,
  as the Nominal-hosted ones do.
- **Existing feature carried further (`english.ron`):**
  - `DeterminativeDistribution` is added to the feature lists of
    `PrepositionPhrase` and `ComplementNounPhrase`.
  - It is added to the Preposition valence's `agree` list.
  - It is unshared in Preposition Phrase coordination, as Noun Phrase
    coordination already unshares it. Without that, PP coordinations such as
    *on the battlefield or in the command zone* lost Readings.
  - It is an existing feature, carried so that a forest node knows whether it
    is distributive. Reading the complement through one family made the
    forest's key disagree with the tree's.
  - Without the unsharing, the census lost 72 Readings in 14 units; with it,
    nothing changes.
- **Chart (`deckmaste_syntax`).** A coordinator leaf keeps the first
  conjunct's `head_final` in the item state, so the last conjunct's item
  records it for every family. Readings are unchanged.
- **Agreement check.** `ranking_keys` checks the forest's keys against the
  trees' over the judgment set. A probe also confirmed agreement on the 8 best
  Readings of every ambiguous corpus unit.

Judgment-set ranking:
- Measured over the 301 single-group entries with 2–4,000 Readings and at
  least one unendorsed Reading.
- Tags: S, the cheapest tier is one endorsed Reading; A, all endorsed; P,
  partly endorsed; F, none.
- Under Right Association alone: S 202, A 17, P 13, F 69 (S+A 219).
- Now: S 203, A 18, P 13, F 67 (S+A 221), against the target of 222.
- Changes from Right Association alone:
  - `kind_of_counter.txt:9` F → S, through the coordination class;
  - `syncretic_word_forms.txt:3` F → A, through the distributive reversal.
- No entry moves from S/A to P/F.

Of the 24 entries that Right Association moved from P to F, one is
recovered (`syncretic_word_forms.txt:3`). The partitive *for each of the
chosen colors* (`predicative_gap_relatives.txt:5`) is not: its fused
partitive Noun Phrase does not carry `DeterminativeDistribution`.

1. **Census delta** (measured on `qttprlkr`): faces read 28,706 → 28,706;
   total Readings (product) 218,547 → 218,547; Surplus 91,328 → 91,328.
   Every unit's verdict, Derivations and Segments are identical; lost faces:
   none. Cheapest tier: singleton 14,526 → 14,544 of 17,220 ambiguous units
   (84.5%); summed 20,674 → 20,646; largest 8 → 8.
2. **Judgment set** (`qttprlkr`): 3,009 enforced at `COMPLETED_STRATUM` 6,
   all hold; 200 pending.

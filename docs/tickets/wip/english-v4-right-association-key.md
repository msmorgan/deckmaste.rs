---
needs: [english-v4-ranking-key-engine, english-v4-comma-adjunct-island, english-v4-omitted-idiom-complement-split, english-v4-fronted-temporal-pp-clause, english-v4-bare-relative-fronted-adjunct, english-v4-cost-not-passivizable, english-v4-keyword-phrase-adjunct-island, english-v4-auxiliary-adjunct-canonicalisation, english-v4-in-addition-to-predicate-host]
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
- Dependents whose class attaches high count their length reversed. The
  class is set by `english-v4-attachment-classes`.

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

## Landing

What landed:
- **Host words.** Each unselected dependent (a child named `adjunct`,
  `modifier` or `supplement` after its phrase's `head`) adds the words of its
  host constituent, unless the host is a coordination. It also adds the words
  between the host and itself, for example a selected Object before a
  postposed relative or an interleaved PP.
- **Conjunct balance.** The last conjunct of a coordination adds the
  difference between its length and its first conjunct's.
- **Two ways of computing the key.** The forest computes the key per edge
  during extraction, and `CompiledGrammar::ranking_key` computes it from a
  Reading's tree. The new `ranking_keys` test checks that the two agree over
  the judgment set. A probe confirmed they also agree on the 8 best Readings
  of every ambiguous corpus unit.
- **Not yet:** high-attaching classes count reversed only once
  `english-v4-attachment-classes` lands.

Judgment-set ranking:
- Measured over the 301 single-group entries with 2–4,000 Readings and at
  least one unendorsed Reading.
- Tags: S, the cheapest tier is one endorsed Reading; A, all endorsed; P,
  partly endorsed; F, none.
- Before: S 69, A 25, P 149, F 58 (S+A 94).
- After: S 202, A 17, P 13, F 69 (S+A 219).
- 134 entries move from P/F to S/A; 24 move from P to F.

Nine entries move from S/A to P/F:

- **Grammar defect: keyword-line misanalysis.**
  - Entries:
    - `keyword_mention_without_parameter.txt:23` (*Equip abilities you
      activate …*);
    - `keyword_mention_without_parameter.txt:24` (*Blitz costs you pay …*);
    - `stratum_zero.txt:47` (*Dash costs you pay …*).
  - The cheapest Reading is a Keyword Line whose cost parameter is the whole
    declarative clause (*Equip [abilities you activate cost {2} less to
    activate]*).
  - It wins because the relative clause's host is *abilities* rather than
    *Equip abilities*.
  - The judgments are right. The keyword-cost parameter should not admit a
    declarative Clause; ticket that defect.
- **Grammar defect, likely: frequency NP as Object.**
  - Entry: `no_more_than.txt:13` (*Activate no more than three times each
    turn*).
  - The cheapest Reading takes *no more than three times each turn* as the
    Object of *activate*, with *each turn* on *times*.
  - The judgment is right. A serial temporal Noun Phrase as a direct Object
    should be checked against `english-v4-temporal-np-as-direct-object`.
- **Wrong ranking: conjunct balance.**
  - Entry: `enter_as_copy_complement.txt:11`.
  - The intended reading coordinates a long *with*-PP with a short *as*-PP.
    Balance penalises that, so the cheapest Reading coordinates *equal to
    this creature's power* with *as a Mutant* instead.
  - The judgment is right.
- **Wrong ranking: coordination host plus balance.**
  - Entry: `remove_source_attachment.txt:6`.
  - In *untap it and remove it from combat*, the cheapest Reading makes *from
    combat* an Adjunct of the coordination *[untap it] and [remove it]*. That
    host is free, and the conjuncts balance.
  - The intended *from combat* is *remove*'s selected Complement.
  - The judgment is right.
- **Wrong ranking: the nearest Nominal wins.**
  - Entries:
    - `gapped_coordination_shared_adjunct.txt:11`: *in any order* goes on
      *library* rather than *put*;
    - `kind_of_counter.txt:9`: *on each Spider …* goes on *counter*, and
      *counter on each Spider* is coordinated with *legendary creature you
      control*.
  - Both judgments are right.
- **Wrong judgment, for the owner to decide.**
  - Entry: `vocabulary_batch_1.txt:61` (*a spell was warped this turn*).
  - The cheapest Reading is the verbal passive *was [warped this turn]*, the
    canonical site for the adjunct.
  - The entry endorses only the adjectival passive. ADR §2 retains both, and
    *this turn* describes an event, so the entry looks over-specific. It is
    not rewritten here.

1. **Census delta** (measured on `xmsxwnxt`): faces read 28,706 → 28,706;
   total Readings (product) 218,547 → 218,547; Surplus 91,328 → 91,328.
   Every unit's verdict, Derivations and Segments are identical; lost faces:
   none. Cheapest tier: singleton 5,509 → 14,526 of 17,220 ambiguous units
   (32.0% → 84.4%); summed 56,052 → 20,674; largest 366 → 8 (Mockingbird's
   702 Readings now have a single cheapest).
2. **Judgment set** (`xmsxwnxt`): 3,009 enforced at `COMPLETED_STRATUM` 6,
   all hold; 200 pending.

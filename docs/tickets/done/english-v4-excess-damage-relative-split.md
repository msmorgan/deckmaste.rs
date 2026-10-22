---
needs: []
---
# Withdrawn: *the excess damage dealt …* is split into *the excess* and a Bare Relative Clause

Owner ruling (cost-audit smaller defect S11): mint this as a defect.

## The defect

Archaic's Agony: *Exile cards from the top of your library equal to the excess
damage dealt to that creature this way.* This Segment has 132 Readings, and
88 of them read *the excess* as a complete Noun Phrase (the noun *excess*).
They then read *damage dealt to that creature* as a Bare Relative Clause with
*damage* as its Subject. That Relative Clause postmodifies *top* (or *cards*):

- In one family, the Object of *deal* is the Gap: "the top that damage dealt
  to that creature".
- In the other family (44 Readings), *to* is stranded: "the top that damage
  dealt to __ that creature", with *deal* in its two-object frame.

Both families make *damage* the Subject of active *deal*.

## Authority

- [CR#120.1]: an object deals damage and is its source. Damage never deals
  anything.
  - In Oracle text, *damage dealt* is always the passive participle: 152
    occurrences in the snapshot.
  - Active *damage deals* never occurs in rules text.
- The noun *excess* occurs in Oracle text in only two ways (snapshot counts):
  - *the excess*, ending its Noun Phrase: Toralf, God of Fury, *deals damage
    equal to the excess to any target* (Judgment `excess_count_noun.txt:5`);
  - the fossilised *in excess of*.

  Before a noun, *excess* is the attributive Adjective (*excess damage*, 42
  occurrences). `nouns.ron` already records this in the comment on its
  `excess` entry.
- `docs/decisions/english-lexical-analysis.md` §2: "a newly admitted wrong
  Reading is a defect".

## Withdrawn

The owner withdrew this ticket. The split stays in the forest; nothing in the
grammar or the engine removes it.

History:

- Round 5, item 7c: no one-word `NounDistribution` value. Either a general
  `Conversion: Converted` twin closure, modelled on `fused_head`, or withdraw
  the ticket and leave the split to ranking.
- The closure was built and landed in `vxouvotv`:
  - `ProductionDecl::converted_head`, set on `Determined`;
  - a Derivation closed where its `head` began with a lone converted noun,
    while the same Production over the same earlier children had a `head`
    that the noun's adjective twin began as a premodifier.
- That landing deferred Nameless Race and minted
  `english-v4-fused-comparative-predicative`.
- The owner then reversed the ruling and chose to withdraw the ticket. This
  change backs `vxouvotv` out completely, and Nameless Race is not deferred.

The owner's reasons:

- `converted_head` puts grammar knowledge into the syntax engine. `compile.rs`
  hardcoded the feature and value names `Conversion` and `Converted`, and the
  closure was bespoke.
- It broke a real card, Nameless Race. Every Reading of that card was a
  known-wrong split of the same shape. Removing them left the face with no
  Reading, because *be more than NP* has no analysis yet. That gap is
  `english-v4-fused-comparative-predicative`, still planned.
- It broke that card for about 4 units whose intended Readings were already
  present.

Evidence left for ranking:

- In both witnesses, the Ranking Key puts the wrong Reading first. Its
  cheapest tier is the noun *excess* postmodified by the Bare Relative
  *[the [excess [damage dealt …]]]*:
  - Archaic's Agony: tier of 2, at key 0/5/89;
  - Nahiri's Warcrafting: tier of 2, at key 0/0/94.
- The intended *[the [excess(Adj) damage dealt …]]* Readings are present but
  ranked lower.
- The closure removed only wrong Readings in 16 units: *the excess damage*,
  *the total …*, *the bottom card* and *all white Auras*.
- The two `+` Judgments it added for Archaic's Agony and Nahiri's Warcrafting
  are removed with it.

Standard constraints apply.

Census delta (measured on `vqmvnoor` → this back-out): faces read 28,798 → 28,799 (Nameless Race reads again), product 213,193 → 214,300, Surplus 88,433 → 88,560, cheapest tier singleton 14,801/16,560 → 14,800/16,562 (summed 18,924 → 18,929, largest 8 → 8); exactly the 16 units the closure reduced return to their counts, nothing else changes; lost faces: none.
Judgment set: 3,028 enforced, 207 pending, all holding.

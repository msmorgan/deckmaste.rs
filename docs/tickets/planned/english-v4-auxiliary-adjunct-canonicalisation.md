---
needs: []
---
# Attach post-verbal adjuncts below perfect *have* and passive or progressive *be*

`Progressive`, `Perfect`, `PassivePerfect` and `Passive` (`english.ron`,
around lines 583–601) set `VerbKind: Auxiliary` and wrap a participial Verb
Phrase through `AuxiliaryParticiple`. A trailing adjunct can attach either to
that lexical participial Verb Phrase or to the auxiliary Verb Phrase above
it:
- *was [put there from the battlefield] this turn* vs *was [put there from
  the battlefield this turn]*;
- *have [lost] this turn* vs *have [lost this turn]*.

The two sites differ in no meaning, and k trailing adjuncts make 2^k trees.

## Owner's ruling

This is the fifth structural canonicalization in
`docs/decisions/english-lexical-analysis.md` §2 (amended with this ticket).
The canonical site is the lower, lexical participial Verb Phrase: an
uncoordinated `VerbKind: Auxiliary` Verb Phrase is never a post-VP adjunct
host. Build the canonical site structurally; do not merge afterwards. As a
result:
- the host head is the lexical verb (*put*, *spend*, *block*, *lose*), which
  matches the existing *as Adjunct of put*, *of spend* and *of block*
  judgments and Right Association distance;
- the duplicate trees are never constructed in the chart.

## Change

- Exclude an uncoordinated Verb Phrase formed by those four productions as the
  host of every post-VP Adjunct schema, as the Gerund adjunct already does with
  `Exclude("head.VerbKind", "Auxiliary")`.
- A coordinated auxiliary Verb Phrase still hosts an adjunct its conjuncts
  share.
- Modal Verb Phrases are out of scope: an adjunct above or below *may* is a
  retained scope ambiguity (Vizier of Deferment). Check that the exclusion
  does not catch them, whatever their `VerbKind`.
- Gate: per unit, the set of Reading Normal Forms is unchanged under exactly
  this equivalence while the number of trees falls (§2).

## Judgment updates (owner)

In `clitic_have_object_relative.txt`:
- line 7: `VerbPhrase "lost"` as Complement of have becomes
  `VerbPhrase "lost this turn" as Complement of have`;
- line 8: `VerbPhrase "cycled or discarded"` as Complement of have becomes
  `VerbPhrase "cycled or discarded this turn" as Complement of have`.

`clitic_contractions.txt:12-13` already assert the canonical site
(`VerbPhrase "cast a creature spell this turn" as Complement of have`).

## Expected reductions

- The cost study's text proxy for this gate also caught causative *have*, so it
  gave no reliable total; measure at landing.
- Units where it shows: Moonhold (*was spent to cast this spell*), Activated
  Sleeper (*was put there from the battlefield this turn*), and every *you've
  cast/drawn/lost … this turn* relative.

## Done when

- No adjunct is hosted by an uncoordinated perfect, passive or progressive
  auxiliary Verb Phrase.
- The normal-form gate holds.
- The two judgments are updated and hold.
- 0 faces lost.

## Constraint: no new features in `english.ron`

Owner's ruling (round 2): the canonicalisation uses the existing
`VerbKind: Auxiliary` feature on `Progressive`, `Perfect`, `PassivePerfect`
and `Passive`. It adds **zero new
features to `english.ron`**. It uses existing features, lexicon or catalog
frames, or an engine mechanism, and names no word in the grammar. If it
cannot be done that way, STOP and return to the owner.

Standard constraints apply.

1. **Census delta**: measured at landing.
2. **Judgment set**: measured at landing.

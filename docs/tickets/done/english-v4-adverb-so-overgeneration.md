---
needs: []
---
# Restrict `adv:so` to its Oracle constructions (*so far*, *do so*)

`crates/deckmaste_lexical_source_v4/lexicon/english/adverbs.ron:53` declares
`(lemma: "so", positions: [End, Phrasal])`:

- `Phrasal` (`PhrasalAdverb`) was added for *so far* (*so far this turn*), but
  feeds `AdverbModifiedNounPhrase` (`english.ron:626`),
  `AdverbModifiedPrepositionPhrase` (`627`), `AdverbModifiedAdjectivePhrase`
  (`630`), `AdverbModifiedAdverbPhrase` (`633`), `ModifiedInfinitival` (`544`),
  and `AdverbModifiedTemporal` (`649`), allowing *so* to premodify any Noun
  Phrase, Preposition Phrase, Predicative Adjective, Temporal Noun Phrase, or
  *to*-infinitival (*Destroy so target creature.*, *So you draw a card.*), and
  giving *so far this turn* two attachments (`[so far] this turn` and
  `so [far this turn]`).
- `End` (`EndAdverb`) feeds `EndAdverbial` (`english.ron:515`) and
  `InterleavedAdverb` (`585`), allowing *so* as an unselected end/manner
  Adjunct on any Verb Phrase (*Draw a card so.*) and misanalyzing pro-VP *do
  so* as intransitive *do* plus an unselected Adjunct—which will double-parse
  every Stratum 7 *do so* clause (*If you do so, …*).

## Change

- Remove `End` and `Phrasal` from `adv:so` in
  `crates/deckmaste_lexical_source_v4/lexicon/english/adverbs.ron`.
- License *so* in its attested Oracle constructions: as a degree modifier of
  *far* (*so far*) and in the pro-form construction *do so*, without making it
  a general `PhrasalAdverb` or `EndAdverb`.

## Done when

- `*Clause Destroy so target creature.` and `*Clause Draw a card so.` hold in
  judgments, and *so far this turn* has one internal attachment of *so*.
- Every unit that reads before the change still reads, with no face lost.

Standard constraints apply.

## Record

- `adv:so` is gone. Oracle uses *so* only in *so far* and the pro-form *do so*,
  so both are Lexemes of their own: the End adverb *so far* (`adverbs.ron`) and
  the Intransitive verb *do so*, inflected on *do* (*does so*, *did so*, *done
  so*, *doing so*; `verbs.ron`). Neither is a `PhrasalAdverb` or `EndAdverb`
  *so*. The profiler places *do so* in the pro-form family, at Stratum 7, as
  before.
- The Done-when's `*` Judgments are on strings no card contains. The intent is
  covered by a `+` Judgment on Reverse Polarity (`[so far] this turn`, never
  `so [far this turn]`) and by `+VerbPhrase doing so`. The *do so* Judgments now
  name the head `"do so"`.
- *and so on* (Selective Adaptation, Akroma, Vision of Ixidor) was already
  deferred to `english-v4-stratum-7-discourse-and-long-tail` and stays there.
  Its *so* is now an unknown word, which places those units at Stratum 7.

1. **Census delta** (measured on `tsppnvrp`): faces read 28,709 → 28,709;
   total Readings 382,154 → 382,152; lost faces: none.
2. **Judgment set**: 2,899 enforced at `COMPLETED_STRATUM` 6, all hold; 190
   pending.

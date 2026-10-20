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

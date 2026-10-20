---
needs: []
---
# A slash measure is no free noun phrase

*Draw a card 3/3.* is not English, yet it reads twice: as the Object *a card
3/3*, with *card* an attributive noun before the measure nominal *3/3*, and as
*Draw a card* with *3/3* a Manner Adjunct, through a bare singular Noun Phrase
built on the measure nominal (`MeasureNominal`, `BareSingularTerm`,
`MannerAdjunct` in `crates/deckmaste_english_v4/grammar/english.ron`). Both
analyses predate Stratum 6, which found them and left them alone.

## Change

- A slash measure (*3/3*, *X/X*) occurs where its head licenses one: a
  creature token's or a creature's size before its noun (*a 3/3 green Beast
  creature token*), the value of *base power and toughness*, and a
  predicative of *is* or *becomes*. Find each licensing site in the corpus
  before restricting it.
- It is no Manner Adjunct and no bare singular Noun Phrase on its own, and it
  does not head a nominal that an attributive noun premodifies.
- Fix the same unset-feature leak on `LoyaltyNominal`
  (`crates/deckmaste_english_v4/grammar/english.ron:406`), which sets only
  `Number` and `Countability` while leaving `Determination`, `NounKind`, and
  `NounDistribution` unset (unlike `SymbolNominal`, `NumeralNominal`, and
  `CardinalNominal`), allowing `[+1]` to project through `BareSingularTerm` to
  `MannerAdjunct` (`*Draw a card [+1].`), `TemporalLocation`,
  `BareSingularPossessor`, and `NominalPremodifier`.

## Done when

- `*Clause Draw a card 3/3.` and `*Clause Draw a card [+1].` hold in a judgment
  file.
- Every unit that reads before the change still reads, with no face lost, and
  the census reports the derivation change on previously read units.

Standard constraints apply.

Census delta (`rutnpyrp`): faces read 28,705 → 28,705; total Readings 390,052 → 384,270 (495 units fewer, none more: wrong measure Manner Adjuncts, measured comparatives, subjects, *of*-complements and relative antecedents, and duplicate bare-measure coordinations); lost faces: none. Strata 0–6 unchanged, 0 known-wrong-only, 0 law failures.
Judgment set (`rutnpyrp`): 3,013 enforced, 189 pending, all hold, including `slash_measure_selection.txt`; tests and clippy pass for deckmaste_syntax, deckmaste_lexical_source_v4 and deckmaste_english_v4.

---
needs: []
---
# A bare idiom noun that may omit its Complement must not shed a following one

*top* and *addition* may stand bare after a preposition without their
Complement (*put that card on top*, *in addition*; CGEL, Ch. 7 §3.1, p. 620),
so `nouns.ron` declares them `Intransitive` beside their `PP` frame and the
`obligatory` `PrepositionalNominal` admits them alone. Where the Complement
follows, the bare noun can still leave it to another host: Starving Revenant
(*for each card you put on top of your library*) has 8 Readings, 4 of them
*on [top]* with *of your library* attached elsewhere. Other *on top of* units
(Nael, Avizoa Aeronaut; Cream of the Crop) are candidates to check. These
Readings predate
[`english-v4-amount-of-excess-bare-nominal`](../done/english-v4-amount-of-excess-bare-nominal.md),
which removed the same split for every noun that may not omit its Complement
(*the amount of [excess] damage dealt this way*, *in [excess] of*).

## Change

- When an idiom noun's selected Complement immediately follows it, the bare
  noun does not also form a Preposition Phrase that leaves that phrase to
  another host: one analysis, as same-head complement priority gives a verb
  (`english-lexical-analysis.md` §2). Amend `english-lexical-analysis.md` §2
  and propagate the omitted-complement preposition mask from
  `Test::FreeOptional(Noun)` along the right edge of ancestor constituents so
  neither an outer `Nominal+PrepositionPhrase` adjunct (*Starving Revenant*)
  nor an `Interleaved` complement-cluster PP (*Cream of the Crop*) can attach a
  PP headed by the omitted preposition immediately after the bare idiom noun;
  no word named in the grammar.

## Done when

- *on top of your library* has no Reading with bare *top*, covered by a `+`
  Judgment on real text (Starving Revenant).
- *Put that card on top.* (Mwonvuli Beast Tracker) still reads.
- No face is lost, and no unit gains a Reading.

Standard constraints apply.

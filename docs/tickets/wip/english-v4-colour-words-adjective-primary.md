---
needs: []
---

# english-v4: colour words as adjectives that head a bare fused-head NP

*black, blue, green, red, white* each have an adjective entry
(`adjectives.ron`) and a HeadOnly mass noun entry (`nouns.ron`). A
reviewer proposes making them adjective-only, the adjective heading a bare
abstract fused-head NP for the colour concept (*devotion to blue*, *a color
other than white*, *green is the most common color*), citing CGEL Ch. 5
§9.3(c), p. 417, and Ch. 19 §3.3, p. 1642 (conversion). Verify both.

Proposed steps (a hypothesis; measure each):

1. Delete the five noun entries.
2. Add `Bare` to `FusedHeadDetermination` and set `fused_head: Some(Bare)`
   on the five adjectives.
3. Add a `BareFusedAdjectiveHead` production (FusedNounPhrase, Plain,
   Third, Singular, Bare, Mass, Ordinary) beside FusedAdjectiveHead and
   DefiniteFusedAdjectiveHead. It bypasses Nominal: no determiner, no
   postmodifier, no Nominal coordination, no WordMention.

Duplicate trees the reviewer claims this removes; measure each before and
after:

- (a) Quality, *protection from white*. `xvworrvr` resolved this the other
  way (noun only, `QualityHead`/`fused_quality` limiting
  AdjectivalQuality). Reconcile to one coherent analysis. The reviewer's
  Option B (`ExcludeTogether` on NominalPhraseQuality) must be measured for
  faces lost.
- (b) copular predicative, *becomes green*. The proposal assumes an
  `ExcludeTogether(Bare, Mass)` on NominalPredicative; that was measured and
  rejected (76 faces lost; bare mass predicatives are English, CGEL
  pp. 536, 538), and the bare-predicative ticket landed as a FixedFrame
  (`onmrzunv`). Decide on CGEL grounds whether any second tree is one
  analysis twice (canonicalize structurally) or a real specifying versus
  ascriptive pair, and bring `english-lexical-analysis.md` §2 into line.
- (c) *devotion to white and black*: NP versus Nominal coordination.
- (d) *Choose a color other than white.*: Mention versus NP (reported 2
  Readings).

## Outcome

- **Proposal not adopted.** CGEL lists colour terms among adjective-to-noun
  conversions (*His tie is brown* (adj), *a dark shade of brown* (noun)) and
  distinguishes conversion from the fused modifier-head (Ch. 19 §3.3,
  p. 1642). The colour fused heads of Ch. 5 §9.3(d) (not (c)), p. 417, [25]
  are elliptical (*I like [blue]*, "blue shirts"), not the colour itself.
  The nouns stay.
- **(a) no change.** *protection from white* has 1 Reading, the noun
  (`xvworrvr`). Option B (exclude bare mass NP Qualities, colours made
  `fused_quality`) loses 2 faces (Reaver Titan, Mistmeadow Skulk): rejected.
- **(b) fixed.** A bare colour word after a copula is the adjective only: the
  noun would ascribe the same property (p. 1642), and an AdjP specifies too
  (Ch. 4 §5.5.1, p. 267, [48ii]); no corpus face has a specifying colour
  Predicative. A bare non-count `HeadOnly` Noun Phrase is no Predicative
  (NominalPredicative; NounPhrase now carries NounDistribution, from the
  last conjunct of a coordination). §2 rewritten. 127 units fell (585
  Readings): 126 copular colour duplicates, and Temple of Power's
  non-English *if [red that sources … dealt]*.
- **(c) not colour-specific.** *devotion to white and black* keeps 3:
  Nominal versus NP coordination of bare NPs is general (*creatures and
  lands*, *damage and life*); ticketed as
  `english-v4-bare-np-coordination-level`. The third Reading is a real
  attachment (*[devotion to white] and [black]*).
- **(d) two analyses, kept.** *a color other than white*: NP (the colour)
  and Mention (the cited word; CGEL Ch. 5 §8.2, p. 401). Excluding mass
  nouns from WordMention loses Prisoner's Dilemma (*silence or snitch*):
  rejected.

## Done when

- No face is lost; every removed Reading is a duplicate or non-English.
- Judgments asserting noun *green* etc. are rewritten to the adopted
  analysis, each named.

Standard constraints apply.

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

## Done when

- No face is lost; every removed Reading is a duplicate or non-English.
- Judgments asserting noun *green* etc. are rewritten to the adopted
  analysis, each named.

Standard constraints apply.

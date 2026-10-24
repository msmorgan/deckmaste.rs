---
needs: []
---
# english-v4: oncology equation fixes (F4, F5, F7, F9, F10)

Grammar and lexicon equation fixes found by the post-Stratum-7 oncology
re-scan. No engine (`deckmaste_syntax`) change. One commit per finding; each
mechanism is a hypothesis verified on its witness, then measured.

1. **F4, colour Quality read twice.** `NominalPhraseQuality` lacks the
   `HeadOnly` exclusion that `SingularNominalQuality` has, so *protection from
   red* reads as AdjectivalQuality and as NominalPhraseQuality. Decide whether
   the Quality slot is one analysis twice and update
   `english-lexical-analysis.md` §2 consistently.
2. **F5, comparative bounded.** `ComparativeBoundedAlternative` and
   `ComparativeBoundedAmount` lack the `last.Grade Comparative` and
   `last.ComplementKind Intransitive` requirements of
   `ComparativeBoundedNumeralName`: *if X is 2 or tapped* is admitted and
   *is 13 or less* has two routes.
3. **F7, *round*.** `Prepositional(["down","up"])` makes *rounded down* an
   active finite preterite. Make *round* transitive with a particle (CGEL
   Ch. 14 §9.1).
4. **F9, *total* / *this way*.** *the total mana value of the cards exiled
   this way* reads with a bare relative whose *exiled* is active with an
   object gap. Pick the more principled of: excluding an NP gap on
   MannerAdjunct (OSG §5), or requiring noun *total* to take its *of*
   complement.
5. **F10, SubjectRemnant with a central *then*.** Exclude an occupied central
   position on SubjectRemnant, as Imperative does.

Standard constraints apply. No face lost; every removed Reading is a duplicate
or non-English.

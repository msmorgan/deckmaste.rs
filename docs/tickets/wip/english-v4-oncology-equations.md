---
needs: []
---
# english-v4: oncology equation fixes (F4, F5, F10)

Grammar and lexicon equation fixes found by the post-Stratum-7 oncology
re-scan. No engine (`deckmaste_syntax`) change. One commit per finding; each
mechanism was verified on its witness, then measured.

1. **F4, colour Quality read twice.** *protection from red* read as the colour
   noun and as a fused-head adjective. The Quality is one analysis, the noun's:
   a fused-head adjective needs the lexicon's `fused_quality` declaration (the
   `QualityHead` feature; *multicolored*, *monocolored*, *colorless*, *worthy*,
   and every supertype word), which a colour word lacks.
   `english-lexical-analysis.md` §2 says why this is one analysis in a Quality
   and two after a copula. The finding's literal fix (an `Exclude` on
   `NominalPhraseQuality`) cannot be written: Noun Phrase carries no
   `NounDistribution`.
2. **F5, comparative bounded.** `ComparativeBoundedAlternative` and
   `ComparativeBoundedAmount` take a comparative `last` with an Intransitive
   Complement, as `ComparativeBoundedNumeralName` does. *If X is 2 or tapped*
   and *power 2 or green* are no longer admitted. The Amount is also an
   Amount-kind Complement that `NumericNounPhrase` excludes, so *is 13 or
   less* has the adjectival route alone. (`PP` would not do: the nouns' Amount
   Complement frame would reject it.)
3. **F10, SubjectRemnant with a central *then*.** A Subject remnant's Verb
   Phrase has no occupied central position, as Imperative's has not.

F7 (*round*) and F9 (noun *total* with *this way*) did not hold up and moved to
`english-v4-oncology-round-total`.

Standard constraints apply. No face lost; every removed Reading is a duplicate
or non-English.

1. **Census delta** (`mspqyqss` -> `tloqulks`, full corpus): faces read 32,825 -> 32,825; total Readings 340,496 -> 339,709 (249 units fewer, none more). After the refresh over `english-v4-oncology-determinatives-names` the tip reads 327,836 (196 more units fewer, none more). Lost faces: none.
2. **Judgment set** (`tloqulks`): 3,361 enforced, 0 pending; all hold.

---
needs: []
---
# Color words in `Quality` and `Predicative`: noun vs. adjective

The five color words (*black*, *blue*, *green*, *red*, *white*) are declared
both as mass nouns (`nouns.ron`, `countability: [Mass], distribution:
HeadOnly`) and as unrestricted adjectives (`adjectives.ron`). Wherever a
construction accepts both a bare mass `NounPhrase` (`BareMass`) and an
`AdjectivePhrase`, every bare color word is read twice:

- In `Quality` (`english.ron`), *protection from red* and *hexproof from white*
  read *red* and *white* as both `NominalPhraseQuality` (`NounPhrase`) and
  `AdjectivalQuality` (`AdjectivePhrase`), producing 409 surplus Readings
  across 182 Corpus Units (177 card faces). `AdjectivalQuality` is only needed
  for adjectival or supertype qualities that are not color nouns
  (*multicolored*, *monocolored*, *colorless*, *snow*, *worthy*).
- In `Predicative` (`english.ron`), *Target creature becomes red.* and *If it's
  red, draw a card.* read *red* as both `AdjectivalPredicative`
  (`AdjectivePhrase`) and `NominalPredicative` (`BareMass` of `noun:red`),
  producing 267+ surplus Readings across 111+ Corpus Units.
- Similarly, the `HeadOnly` count nouns *bottom*, *excess*, *maximum*, *top*
  and *total* are declared in `adjectives.ron` for attributive use (*the bottom
  card*, *excess damage*, *maximum hand size*, *the top card*, *total mana
  value*, as their comments in `nouns.ron` state) without
  `position: Some(Attributive)` (unlike *left*), and `SingularNominalQuality`
  does not exclude `NounDistribution` `HeadOnly`.

## Change

- In `Quality`, read the five color words only as color nouns
  (`NominalPhraseQuality`), keeping `AdjectivalQuality` for non-noun
  adjectival/supertype qualities (*multicolored*, *monocolored*, *colorless*,
  *snow*, *worthy*). Update `judgments/keyword_quality_phrases.txt` where it
  currently names `AdjectivePhrase "black"` / `AdjectivePhrase "red"` on
  *protection from black and from red*.
- In `Predicative`, prevent bare `HeadOnly` color mass nouns from duplicating
  `AdjectivalPredicative` through `NominalPredicative`.
- Restrict the attributive-only `adjectives.ron` entries (*bottom*, *excess*,
  *maximum*, *top*, *total*) to `position: Some(Attributive)` and exclude
  `HeadOnly` from `SingularNominalQuality` if no corpus unit needs them in
  non-attributive adjective position.

## Done when

- *Protection from red* and *Target creature becomes red.* each have one
  Reading for *red* (noun in `Quality`, adjective in `Predicative`).
- The 409 surplus `Quality` Readings and 267+ surplus `Predicative` Readings
  are gone.
- No face is lost.

Standard constraints apply.

Census delta (sqtnrpxr): faces read 28,709 → 28,709; Readings 331,257 → 330,062; lost faces: none.
Judgment set (sqtnrpxr): `cargo test -p deckmaste_english_v4` passes (2,950 enforced, 194 pending), including the new `color_word_categories.txt` and `amount_of_excess_damage.txt`.

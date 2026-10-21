---
needs: []
---
# Number agreement of *target* and *all* with zero-plural nouns

A noun whose plural Word Form is spelled like its singular (*Equipment*,
*Merfolk*, *Kavu*, *Kithkin*, *Samurai*, *Myr*, *Treefolk*, *Moonfolk*, *Plains*,
*d6*) gets two Readings wherever nothing fixes its Number. In 146 surplus
Readings across 55 Corpus Units, something should fix it:

- Bare *target* has no Number (`closed_class.ron`), so *Target Merfolk can't
  be blocked this turn* and *Destroy target Equipment* also read the noun as
  plural. In Oracle, *target* on its own determines a singular noun. A plural
  noun needs an outer determiner that marks it and gives it its Number (*two
  target creatures*, *up to X target creatures*) or an adjective (*other target
  creatures*), as the `MarkedDeterminer` comment in `english.ron` says.
- *all* has no Number or Countability, so *destroy all Equipment attached to
  that creature* also reads *Equipment* as a singular count noun. *all* with a
  singular head needs a mass noun.
- An attributive noun (*all Merfolk cards*, *all Kavu cards*) reads as both
  singular and plural.

Found by `english-v4-byte-identical-duplicate-readings`. The parse dump now
shows each head's Word Form, so these pairs no longer print identically. Each
pair is still two analyses, and one of them is wrong.

## Change

- Give bare *target* and *all* the agreement their heads need, and settle what
  Number an attributive noun takes. Do this in the grammar or the lexicon, not
  with a flag specific to a word.
- Use `*` Judgments only for inflection that agreement selects (*target*
  with a plural head is the case here).

## Done when

- Each of the cited units has one Reading per Number that agreement allows.
- No face is lost.

Standard constraints apply.

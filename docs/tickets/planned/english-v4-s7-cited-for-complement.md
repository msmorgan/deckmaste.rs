---
needs: []
---
# Stratum 7: cited words as the complement of *for*

Residue of `english-v4-s7-long-tail-quotes-and-mentions`.

## Scope

**Grim Captain's Call**: *then do the same for Vampire, Dinosaur, and Merfolk*. `Mention` as the complement of *for* is the right analysis (the subtype names are cited, and none is a bare Noun Phrase), and it parses, but no way of licensing it is both duplicate-free and head-selected without an engine change.

- A `for` frame `(complement: Mention, hosts: Predicate)` makes every bare singular noun after *for* a second Reading beside its bare Noun Phrase: *tapped for mana*, *votes for blue, black, red, or green*, *tied for greatest power*, *except for Merfolk, Krakens*. Measured: more than 100 units grew, as duplicate trees.
- Hosts `ComplementOnly` plus `do: TransitiveWithPP(["for"])` is selected by lemma alone, so every *for* phrase after *do* (including `anaphoric_same.txt:9`, *Do the same for flying, …*, endorsed as an Adjunct) turns into a Complement, and any other head that selects `PP(["for"])` (*tied*, *vote*) takes the Mention too.

## Engine change

A Frame Slot `SlotFill::PP(lemmas)` selects a Preposition Phrase by its preposition only. Let it also name the complement's Slot Kind, for example `PP(lemmas, Some(Mention))` (lexical `FillSpec::PPWith(["for"], Mention)`). Then `do` selects only *for* + Mention (`TransitiveWithPP` with that fill), the Mention frame of *for* is `hosts: ComplementOnly`, the NP-complement *for* phrases keep their Adjunct analysis, and the Mention cannot duplicate a Noun Phrase. The witness is Grim Captain's Call; `deferred.txt` keeps its line until then.

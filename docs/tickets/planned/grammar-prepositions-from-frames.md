---
needs: [grammar-frame-derived-gaps]
---
# Prepositions from frames, case as a feature

**Placeholder: not claimable.** The README has no marker for this, so the `needs:` chain is what blocks it, and it must not be claimed or worked until it has been specified. It is specified only after its predecessor lands, following a short design pass the owner approves; no rulings are pre-written here.

Goal: build preposition phrases from preposition frames and make case a feature, removing the case wrappers (`CasePhrase` is the most-built construction, 7,744,012 nodes; `Noun` follows at 6,480,818). 18 constructions produce `PrepositionPhrase`. A complement the verb's frame selects should not also be analysed as an adjunct or postmodifier: "Search your library for a card." has 3 readings (frame complement, verb-phrase adjunct, postmodifier of "library"); the `search` plugin frame `[ObjectNounPhrase, Lex("Preposition","For"), ObjectNounPhrase]` is used by only one of them.

Gate decided so far: no face lost; ratchet passes; three-line note.

Open, owner has not decided: whether dropping the adjunct and postmodifier analyses of a preposition phrase the verb's frame selects (and "Destroy target creature with flying." keeps both attachments, which are different heads) is an approved removal. The owner decides this inside this ticket once the analysis key shows what the removal takes away; until then the gate is that every removed key has a surviving twin differing only in that label.

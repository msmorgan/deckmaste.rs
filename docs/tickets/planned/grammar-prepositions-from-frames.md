---
needs: [grammar-frame-derived-gaps]
---
# Prepositions from frames, case as a feature

**Placeholder: not claimable.** The README has no marker for this, so the `needs:` chain is what blocks it, and it must not be claimed or worked until it has been specified. It is specified only after its predecessor lands, following a short design pass the owner approves; no rulings are pre-written here.

Goal: build preposition phrases from preposition frames and make case a feature, removing the case wrappers (`CasePhrase` is the most-built construction, 7,744,012 nodes; `Noun` follows at 6,480,818). 18 constructions produce `PrepositionPhrase`. "Search your library for a card." has 3 readings (frame complement, verb-phrase adjunct, postmodifier of "library"); the `search` plugin frame `[ObjectNounPhrase, Lex("Preposition","For"), ObjectNounPhrase]` is used by only one of them.

Gate decided so far: no face lost; the census check mode passes for the landing type the owner assigns; two-line note.

The specification must include an inventory of the existing tests that pin the affected tree shapes (for example the case-wrapper constructions in `crates/deckmaste_english_v3/tests/suite/relative_clause_adjuncts.rs`) and re-spell each against the new shape for the same witnesses; tests are never deleted to pass, and the landing reports restored, re-spelled, added and removed counts. The landing updates the affected crate README.

Open, owner has not decided: whether dropping the adjunct and postmodifier analyses of a preposition phrase the verb's frame selects is an approved removal ("Destroy target creature with flying." keeps both attachments, which are different heads). The owner decides it on concrete before and after analysis keys, which must distinguish a same-head relabelling (complement versus adjunct of the same verb) from a changed attachment (the postmodifier of "library" has a different head). Until the owner decides, every existing analysis key is preserved (a frame-selected preposition phrase already has its own host and complement-slot identity in the key, so it never shares a key with an adjunct or postmodifier analysis). Also to decide: how a preposition frame is represented, how case propagates and agrees, and how every consumer of the case wrappers migrates.

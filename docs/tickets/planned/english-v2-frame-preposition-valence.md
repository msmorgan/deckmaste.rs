---
needs: [english-v2-closed-class-single-owner]
---
Verb-frame selected prepositions into declared valence data (homograph
review Class E, 20 rows): `to`/`from`/`on`/`into`/`for`/`onto`/`at` are
hardwired as form literals in frame constructions (constructions.rs
~:3446 already calls it a workaround). The declared valence (core_verbs
seed and KeywordAction grammar contributions) carries the selected
preposition as data; the frame construction consumes it as a vocab
Preposition claim in selected-only position — no literal. Also close the
optional-slot workaround if `english-v2-require-through-optional-role`
has landed. `form_literal_vocab_overlaps` decreases by these 20. Zero
coverage change; standard constraints apply.

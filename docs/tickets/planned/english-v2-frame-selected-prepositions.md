---
needs: [english-v2-closed-class-single-owner, english-v2-require-through-optional-role, english-v2-verb-frame-vocabulary]
---
**Move verb-selected prepositions into declared Verb Frame data.** Under the
[`Verb Frame` and `Complement` definitions](../../contexts/oracle-english/CONTEXT.md), the selected
preposition is part of what the lexical schema licenses; `valency` is not a
catch-all name for the schema, its instantiated phrase, and its realization.

Replace the 20 hardwired `to`/`from`/`on`/`into`/`for`/`onto`/`at` form
literals with vocabulary `Preposition` claims in selected-complement positions.
The core-verb seed and Keyword Action grammar contributions declare the marker,
and the frame construction consumes it. Close the optional-slot workaround at
the same seam. The `form_literal_vocab_overlaps` count decreases by these 20
with zero coverage change; standard constraints apply.

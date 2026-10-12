---
needs: []
---
# A registered label is not also an open flavor-word label

Parked on 2026-10-12 so the authoring-layer refactor goes first. A reviewed, gate-passing attempt is archived as `archive-grammar-registered-labels-not-flavor-words` (72,456 flavor-word Readings of registered labels removed, no face lost); it stores a `label_registration` flag on the lexeme, which is not wanted, and the rework to derive it from the existing lexeme source was interrupted. Flavor words themselves are registered nowhere: any text may stand in that position.

A line that begins with a registered ability word or keyword is read twice today: once as that registered label, and once as an open flavor-word label (an arbitrary word before the dash). Reading *Landfall* as an arbitrary flavor word is not a different grammatical analysis of the card, so the second Reading goes. (A flavor-word label is the labelled-ability form, as on Dungeons & Dragons cards; it is not flavor text.)

Measured after `grammar-per-sentence-enumeration`: 1,239 label-led ability units have both `AbilityWordHead` and `FlavorWordHead` wrappers (142,904 Readings), and 150 have `FlavorWordHead` beside `KeywordLine → ClausalKeyword` (2,008 Readings). Because their roots differ, these units cannot be decomposed and are kept whole; they hold about 30% of all enumerated Readings. 63 of the 64 distinct leading labels also have more than one declared lexical owner (for example a catalog entry and a plugin lexeme for *parley*, where the plugin lexeme lacks the property that marks it as an ability word).

## Change

- Lexical analysis produces an open flavor-word label only for a word that is not already registered as an ability word or a keyword. This is decided in the lexical layer, from registration, not by a grammar guard.
- One lexical owner per label. Where both a plugin declaration and a catalog entry exist, state which is the owner and make the other a membership authority only, the way subtypes already work; report every label that had more than one owner and what was done.
- No other grammar change. The 60 modal-list paragraphs kept whole for a trailing-sentence attachment ambiguity are not part of this ticket.

## Gate

- No face lost; read and unread sets unchanged.
- Every removed Reading is a flavor-word-label Reading of a registered label, or a duplicate-owner twin. Units with an unregistered label keep their Readings. Report the before and after counts for the 1,389 affected units and how many now decompose.
- A real flavor-word-labelled card (an unregistered label) still reads, with a test from the corpus.
- Three-line note: census delta; the size and judgment-set lines do not apply yet.

---
needs: []
---
# Four one-line narrowings in forms and roles

**R5 — Group R.** Four independent defects, one ticket because each is a single
line and none needs a design decision. Each is a value-keyed form or a role
optionality that froze one attested spelling as a rule.

1. **`edge_of_phrase` loses `the` on `Top`**
   (`crates/deckmaste_english_v2/src/constructions.rs:3615`):
   `form top when position is Top = lex(position) lex(relation) whole;` beside
   `form bottom otherwise = "the" lex(position) lex(relation) whole;`. So
   `from top of your library` parses, **`from the top of your library` does
   not**, and `on the bottom of your library` does. This is a transcription
   error frozen by a value-keyed form, not a grammar rule.
2. **`passive_movement_predicate` makes the source mandatory** (`:1785`):
   `source: FrameComplement` is required, so
   `Whenever a permanent an opponent controls is put into a graveyard, …` fails
   and only the attested `put into X from Y` shape is admitted. The role is
   `opt`.
3. **`bare_and_predicate_coordination` refuses a two-member `, and`** (`:1662`):
   the positional separator table sets `pair = " and "`, so
   `…exile them in a face-down pile, and shuffle that pile.` is refused. Oracle
   spells both; the pair position admits both surfaces.
4. **`codec EnterWithCountersVerb` confines a general frame to one verb**
   (`:715`): the `V with ‹obj› ‹prep› ‹obj›` tail is *enter*-only, so
   `…exile that card with a dream counter on it…` fails for every counter kind.
   The frame belongs to any verb that declares it.

Pinned shape. Fix each in place; each is independently landable and independently
measurable. Item 4 is the smallest instance of the pattern
`english-v2-lexeme-owned-verb-frames` generalizes — do it here only as far as
letting the existing codec serve more than one verb; do not start the frame
redesign in this ticket.

Fences. Adding a fifth form arm instead of removing a value key. A `checked by`
naming `top`, `bottom`, `enter`, or any card. Any census used as a gate.

Glossary: Form, Linearization, Complement, Verb Frame, Coordination. Record any
gap.

Baseline, measured on change `oulzkkoqmvuv` — re-measure at claim. Standard
constraints apply.

---
needs: []
---
# *The next time …, instead put …* reads only as a wrong Subject

*{5}, {T}, Exile this artifact: The next time you would draw a card this
turn, instead put a card you own from outside the game into your hand.*
(Ring of Ma'rûf) and *{2}: The next time you would draw a card this turn,
instead put the top card of the exiled pile into its owner's hand.*
(Mangara's Tome) have Readings today, but every one makes *The next time you
would draw a card this turn, instead* the Subject of a preterite *put*. The
intended analysis, a preposed temporal Noun Phrase Adjunct (*the next time
…*, with its adverbial relative clause) before the imperative *instead put
…*, has no Reading: even *The next time you draw a card this turn, instead
put a card into your hand.* has none.

## Change

- Fix the three interacting grammar rules that prevent the preposed temporal
  Adjunct + imperative Reading and admit the spurious Subject + preterite
  Reading:
  1. Add a pre-`VerbPhrase` `Adjunct` schema for intransitive prepositions
     (`Require("adjunct.ComplementKind", "Intransitive")`) so clause-initial
     `instead` can modify imperative `put …`.
  2. Exclude intransitive prepositions (`Exclude("adjunct.ComplementKind",
     "Intransitive")`) from the comma-delimited post-`VerbPhrase` `Adjunct`
     schema so `, instead` cannot attach inside the preceding adverbial
     relative clause (*you would draw a card this turn, instead*).
  3. Exclude adverbial relative clauses on temporal nominals
     (`ExcludeTogether([("head.NounKind", "Temporal"), ("modifier.RelativeKind",
     "Adverbial")])`) on `NominalPostmodifier` so *The next time `<Clause>`*
     must project through `RelativeTime` (`RelativeTemporalNominal`) rather
     than `Determined` as an ordinary `NounPhrase` Subject.

## Done when

- Both faces read with the temporal Adjunct over the imperative.
- `english-v4-case-solve-condition` can exclude the preterite main clause
  without losing them.

Standard constraints apply.

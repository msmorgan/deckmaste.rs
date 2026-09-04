---
needs: []
---
**Add `Noun.OneEachOf` for party and spell outlaw as a macro.** Fresh
workbench review 2026-09-03, R4, resolved by ruling.

**Ruling (settled 2026-09-03): outlaw is a macro, party is a constructor.**
"Outlaw" is a plain disjunction over creature types and needs no grammar: a
macro spelling that disjunction is the whole of it, the way
`Phrase.Predicate.IsHistoric` (`Phrase.idr:289`) is a leaf for its own defined
term. 20 supported cards say it (e.g. At Knifepoint).

Party is not a disjunction. `Noun.OneEachOf (roles : List (Predicate bs
Object)) (pool : Noun bs Object)` denotes **the joint maximal sub-group of
`pool` with at most one member per role and each member counted once**,
computed by the game per [CR#700.8a..700.8b] — not by the term. `party`,
`partySize`, `fullParty` and "a creature in your party" are macros over it.
31 supported cards read a party (e.g. Archpriest of Iona).

**Banned compositions**, both of which get the wrong answer and neither of
which may be substituted for `OneEachOf`:

- Independent per-role finds. Four separate "a Cleric you control", "a Rogue
  you control", … double-count a creature that is more than one of those
  types, so a lone Cleric Rogue would be a party of two.
- Greedy consuming finds. Taking each role in turn and removing the creature
  it matched is not maximal: an early role can consume the only creature a
  later role could have used.

Pin the small case in both directions: a single Cleric Rogue is a party of
**one**, and the term must not admit a reading that makes it two.

Stick Together is unaffected and keeps its `Choose (UpTo 1)` per role
[CR#700.8d] — that card chooses creatures, it does not read the party.

Size: M.

Done when: Archpriest of Iona and a `fullParty` card are typechecking bench
witnesses, and At Knifepoint reads through the outlaw macro; `partySize` of a
lone Cleric Rogue is one, recorded as a bench witness rather than a comment;
a pin refuses the double-counting reading, probed non-vacuous; Stick Together
still typechecks unchanged; the build is 44/44 with 0 errors and 0 warnings.
Standard constraints apply, plus the RON-shaped constraint: a core constructor
is admissible only if the RON re-emitter can produce it from a RON node, and a
macro only if it names a RON macro
(`docs/decisions/workbench-ron-shaped-and-label-rulings.md`).

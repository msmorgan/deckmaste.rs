---
needs: []
---
**Give "tapped for mana" its own `EventName` and delete the `forMana` axis.**
Ruling 2026-09-04 (cleanroom review 3, D-Q5).

- `Triggers.VerbedEvent` carries `forMana : Bool` on every verbed event, and
  `Words.actFacts`'s `actForMana` column is `True` for `Tap` alone — a lexeme
  guard written as data, consulted by `Events.verbForManaOk`. An ability that
  triggers whenever a permanent "is tapped for mana" triggers whenever such a
  mana ability resolves and produces mana [CR#106.12a], which is an event of
  its own rather than a flag on tapping.
- Add `Events.EventName.TappedForMana`, and either carry the specified-mana-type
  variant [CR#106.12a] or record the decision to leave it out. Delete
  `forMana`, `actForMana` and `verbForManaOk`.
- Re-spell the witnesses that carry `forMana = True` as the new event, and pin
  a plain tap read as a mana event.

Size: S. Done when: `grep` finds no `forMana` or `actForMana`; every
tapped-for-mana witness reads the new event; the pin probes non-vacuous; build
at its module count. Standard constraints apply, including the RON-shaped
constraint.

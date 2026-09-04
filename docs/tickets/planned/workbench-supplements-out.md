---
needs: []
---
**Delete the casual-supplement card types and the command-zone card class;
keep the planar-die effects.** Fresh workbench review 2026-09-03, R1,
resolved by ruling.

**Ruling (settled 2026-09-03): the workbench aims at objects that can be in a
Vintage deck.** `Words.CardType` loses `Conspiracy`, `Phenomenon`, `Plane`,
`Scheme`, `Vanguard` **and `Dungeon`**, and with them go
`Card.CardClass.CommandZoneCard`, `Card.commandZoneType`,
`Card.commandZoneTypeAbilityOk` (`Card.idr:95–102`),
`Card.anyCommandZoneType`, the `CommandZoneCard` clauses of
`Card.keywordCardOk` and `Card.classAbilityOk` (`Card.idr:52, 85–92`), the
`Words.KeywordFacts.onCommandZoneCard` column and its per-row value, and
anything whose only remaining dependency is one of those. Emblems are the
future home of dungeon semantics; venturing is not modelled by a `Dungeon`
card type.

The planar-die **effects** stay: `RollPlanarDie`, `ChaosEnsues` and
`PlanarDie` are printed on Vintage-legal cards (Fractured Powerstone, Missy,
Centaur of Attention), so the die machinery is card-backed even though the
plane card type is not.

This reverses parts of `done/workbench-card-class-and-command-zone` and
`done/workbench-command-zone-per-type-abilities`; both landed before the
scope ruling and are superseded by it, not contradicted on their own terms.
Note that the bench's one `Conspiracy` mention is the *enchantment* named
Conspiracy, which is unaffected.

Any pin whose subject is a supplement type goes with it — a pin refusing an
ability on a `Scheme` has no subject once `Scheme` is gone. A pin that
refuses something for a reason surviving the deletion is re-spelled, not
deleted; the landing record reports the split.

Size: S–M (~70 lines of deletion plus the column).

Done when: `CardType` holds the fifteen types minus the six named above and
`CardClass` has two constructors; `onCommandZoneCard` is gone from
`KeywordFacts` and from every row (including `Storied`'s); `grep -rn
'CommandZoneCard\|commandZoneType' idris/src` is empty; Fractured Powerstone
still typechecks as a bench witness; the deleted pins are accounted for by
name in the landing record with the reason each had no surviving subject; the
build is 44/44 with 0 errors and 0 warnings. Standard constraints apply, plus
the RON-shaped constraint: a core constructor is admissible only if the RON
re-emitter can produce it from a RON node, and a macro only if it names a RON
macro (`docs/decisions/workbench-ron-shaped-and-label-rulings.md`).

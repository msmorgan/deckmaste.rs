---
needs: []
---
**Add the combat steps and cleanup to `Words.TurnPart` [CR#506.1,514.1].**
Fresh workbench review 2026-09-03, F9.

`Words.TurnPart` has no `DeclareAttackers`, `DeclareBlockers`, `CombatDamage`
or `Cleanup`, so a whole family of printed trigger headers has no spelling:
"at the beginning of the declare blockers step" (34 supported cards, e.g.
Dazzling Beauty), "declare attackers step" (26), "combat damage step" (3),
"cleanup step" (15).

The four names are already RON: `plugins/builtin_v2/macros/stubs/turn_parts/`
holds `DeclareAttackersStep.ron`, `DeclareBlockersStep.ron`,
`CombatDamageStep.ron` and `CleanupStep.ron`, so the constructors are
re-emittable the day they land.

Fix: four constructors plus their rows in `Words.turnPartIx`,
`Words.partTriggerOk` and `Words.partAddable`. The gates are already there;
this is rows, not mechanism. `Words.PartQuant` (`workbench-possessor-residues`)
applies to the new parts unchanged — a turn can hold more than one combat
phase, so "each combat damage step" is spellable without a new slot.

Size: S.

Done when: a "beginning of the declare blockers step" trigger (Dazzling
Beauty) and a cleanup-step trigger are typechecking bench witnesses; the four
new parts each have their `partTriggerOk`/`partAddable` rows; a pin refuses a
part that the CR makes untriggerable, probed non-vacuous; the build is 44/44
with 0 errors and 0 warnings. Standard constraints apply, plus the RON-shaped
constraint: a core constructor is admissible only if the RON re-emitter can
produce it from a RON node, and a macro only if it names a RON macro
(`docs/decisions/workbench-ron-shaped-and-label-rulings.md`).

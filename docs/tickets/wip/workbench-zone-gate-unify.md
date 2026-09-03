---
needs: []
---
**Unify the two battlefield-zone gates into one strict `ZoneIs`, make
`deedHeadTysOk` compatibility-based over conjunctive heads, and key the bare
admission on the noun rather than the deed row.** Cleanroom review 2026-09-03,
F4 + F5, and audit-2 N6 / regression R-B. Three defects in one gate family.

## One zone gate, strictly (F4)

`Words.zoneFits:2218` has `zoneFits Nothing _ = True` (admits an unzoned
noun); `Words.OnBattlefield:2139` has no inhabitant for `Nothing`. The
permissive spelling guards 8 `StaticEffect` constructors (`Effect.idr:332,
341, 396, 399, 403, 455, 459, 463`), 13 `GameEvent` headers in `Triggers` and
15 macros; the strict one guards 15 `Effect` constructors (`Effect.idr:1046–
1304`) and 8 macros — inverted between modules. On `target source` (unzoned:
`Phrase.IsSource` is placeless, `Phrase.idr:1103–1110`), `gets (target source)
(PtUp 1) (PtUp 1) (Just untilEndOfTurn)` is accepted while `destroy (target
source)` and `tap (target source)` are refused. "Target source gets +1/+1" is
not rules-meaningful — [CR#609.7a] lets a chosen source be a spell on the
stack or a face-up command-zone object.

`done/workbench-battlefield-gate-defect` audited all 53 sites and *tightened
two rows* (`Regenerate`, `DoorOf`) with pins, per-row, instead of unifying the
mechanism — which is why the divergence survives. Fix: one strict `ZoneIs :
Maybe Zone -> Zone -> Type` (`So (zoneIsB mz z)`) at every site. Auditor C
retyped `destroy` this way in a scratch copy and rebuilt core plus the whole
bench (passes, 13.5 s). The five one-inhabitant witnesses `OnBattlefield`,
`OnStack`, `AtLeastTwo`, `ChoiceStands`, `Ascribable` have constructors that
occur only in 29 pin bodies; fold them into the same idiom in the same pass,
rewriting those pin bodies.

## Conjunctive head types (F5)

`Events.idr:360–362`: `deedHeadTysOk v r ts = all (deedTypeOk v r) ts`, so
every head type must be a deed role type and `And [land, creature, …]` fails
`DeedFits ["Attack"]` while `Gets` on the same noun is fine. Probe:
`Continuously (deontic (allOf (And [land, creature, HasPossessor ControllerAx
You])) Require ["Attack"] Agent …) (Just thisTurn)` → `Can't find an
implementation for DeedFits ["Attack"] Agent Object (nounHeadTys …) …`;
dropping `land` typechecks. Printed: Blossoming Tortoise "Land creatures you
control get +1/+1", Aang, at the Crossroads, Awaken the Woods' land-creature
tokens; [CR#508.1a] asks only for an untapped creature. Fix: compatibility
over the conjunction — some head type is a deed type and none is excluded —
not `all`. `ProofsB:172 badCantAttackLand` and `ProofsD:352 badMustAttackLand`
carry a lone `land` head and stay refused.

## Bare admission belongs to the noun (N6, regression R-B)

`Fights (target Permanent) (target creature)` typechecks because
`Events.deedHeadTysOk v r [] = deedBareOk v r` and the `"Attack"` agent row is
`roleBare = True` (there for `This` and pronoun reads). A [CR#701.12a] fight
needs a creature. The gate-dedup round deleted `FightParticipant (Just t)`,
which refused it, and disclosed that no pin covered it. Fix: `So
(deedHeadTysOk … (nounHeadTys n))` becomes `So (deedNounOk "Attack" Agent n)`,
where a `Described` noun with empty head types is refused (`isJust (nounDet n)
→ not bare`) and pronouns / `This` / `AsType` keep the bare path.

Size: M.

Done when: the build is 23/23 with 0 errors and 0 warnings; `zoneFits`'s
permissive branch and the five one-inhabitant zone witnesses are gone and
every former site reads through `ZoneIs`; `gets (target source) …` is refused
and a pin records it, non-vacuous; a land-creature deed restriction
(Blossoming Tortoise) is a typechecking bench witness while
`badCantAttackLand`/`badMustAttackLand` still refute; `badFightPermanent` sits
beside `badFightLand` and is non-vacuous. Standard constraints apply.

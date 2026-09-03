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

## As landed

- F4 mechanism: one `Words.zoneIsB : Maybe Zone -> Zone -> Bool` plus
  `ZoneIs mz z = So (zoneIsB mz z)`. `onFieldZone` and `onStackZone` (the
  third and fourth spellings of the same comparison, 8 written-out clauses
  each) are now one-liners over it, and `zoneFits`'s `Just` case reads
  `zoneIsB`.
- F4 sites: 65 use sites read `ZoneIs` (Phrase 6, Triggers 12, Effect 24,
  Macros 21, Words 1 — `DesignationHolder.HolderOnField`); 14 sites keep
  `ZoneFits` because their requirement side is a `Maybe Zone` that is
  genuinely absent at some of them (see Deviations); `Blocks`/`BecomesBlocked`
  patients moved to a new `Triggers.PatientZoneIs` that distinguishes "no
  patient" from "unzoned patient".
- F4 witnesses: `OnBattlefield`/`OnField`, `OnStack`/`OnTheStack`,
  `AtLeastTwo`/`TwoUp`, `ChoiceStands`/`ChoiceMade`, `Ascribable`/`AscribeThis`
  are gone; all five are `So` synonyms (`atLeastTwo`, `isSucc`, `ascribable`)
  and their 34 pin-body constructors are `Oh`.
- F4 probe pair pinned: `ProofsF.badGetsSource` refuses "target source gets
  +1/+1 until end of turn" [CR#609.7a] beside the existing
  `ProofsF.badDestroySource`; both non-vacuous.
- F5: `Events.deedHeadTysOk` is compatibility over the conjunction, but the
  flat `List CardType` could not tell `And` from `Or` (both flatten to one
  list), so `nounHeadTys`/`deedFits`/`DeedFits` now carry
  `List (List CardType)` — one entry per disjunctive alternative, each the
  union of that alternative's conjunct head types (`Phrase.headTyAlts`).
  `deedHeadTysOk = all deedAltOk` over the alternatives; `deedAltOk` is `any`
  over one alternative's types. `And [land, creature, …]` fits `["Attack"]`;
  `Or [creature, land]` still does not.
- F5 witness: `Cards.auriokSiegeSledDenial` — Auriok Siege Sled, "{1}: Target
  artifact creature can't block this creature this turn" — typechecks, and
  fails when `deedAltOk` is put back to `all`. `badCantAttackLand`,
  `badMustAttackLand` and `badCantDisjunctSubject` still refute.
- N6: `Phrase.deedNounOk v r n` keys the bare admission on the noun — a noun
  with no head types is admitted only when `nounDet n` is `Nothing`
  (pronouns / `This` / `AsType`). The seven `So (deedHeadTysOk …
  (nounHeadTys …))` gates on `Fights`, `BecomesBlocking`, `StopsBlocking` and
  `BecomesAttacking` read it. `Proofs.badFightPermanent` refuses
  `Fights (target Permanent) (target creature)` [CR#701.14a], non-vacuous,
  beside `badFightLand`.
- Nothing was left undone except the `zoneFits` permissive clause, which the
  bench refutes; see the STOP.

## Landing record

- Numbers before/after. Gate spellings of "is this noun on the battlefield":
  3 before (`OnBattlefield` data, `ZoneFits … (Just Battlefield)`,
  `onFieldZone`) → 1 (`zoneIsB`). One-inhabitant witness types: 5 → 0.
  `ZoneIs`/`ZoneFits` use sites: 0/54 → 65/14 (+2 `PatientZoneIs`).
  `nounHeadTys` result type `List CardType` → `List (List CardType)`.
  Construction count: 5 data types with 1 constructor each removed, 0 added.
  `cr-citations.lock` unchanged (both cited rules already registered).
- Gate lines. `cd idris && rm -rf build && ./scripts/build`: `23/23: Building
  Cards (src/Cards.idr)`, exit 0, 23 module lines, 0 Error and 0 Warning
  lines, `real 0m40.9s` (pre-change clean build on the same tree: 23/23,
  `real 1m1.7s`). `cargo xtask cite check --list-noncompliant`: `0
  non-compliant citation-looking string(s)`. `cargo xtask cite check`:
  `checked 17721 citations against cr.txt (eff. 2026-08-07); 0 stale`.
  `cargo xtask cite audit --diff`: `audited 2 citation site(s)` — both read
  against their rule text; `[CR#701.12a]` as given in the ticket is
  *Exchange*, not Fight, and was corrected to `[CR#701.14a]`.
- Assurance: restored 0; re-spelled 38 (34 pin bodies whose witness
  constructor became `Oh`; `badSharedSubjectEmptyDelta` retargeted to the
  `ownSubject` noun, and `badThatCreatureIsSelf`,
  `badThatCreatureIsCondSubject`, `badAltHeaderMixedReadback` given paired
  holes, because the now-strict `Gets` zone gate made their *types* fail to
  elaborate); ignored 0; added 2 pins (`badGetsSource`, `badFightPermanent`)
  and 1 bench witness (`auriokSiegeSledDenial`); removed 0.
- Non-vacuity probed by mis-stating once, each then refused as "not a valid
  impossible case": `badGetsSource`, `badFightPermanent`, `badFightLand`,
  `badTapGraveyard`, `badUntapGraveyard`, `badRegenerateInGraveyard`,
  `badRegenerateBareThis`, `badDoorOfBareThis`, `badResolvedOnBattlefield`
  (stack). The F5 witness was probed the same way against `all`.
- Deviations and additions:
  1. `zoneFits` keeps its unknown-subject clause — the STOP below.
  2. `Triggers.Enters` keeps `ZoneFits … (Just Battlefield)`. The event is
     what puts the subject on the battlefield, and 7 bench sites spell it
     `Enters This Nothing` (Fblthp and six others); the strict gate refused
     all of them.
  3. `Macros.leavesBattlefield`, `leavesZone` and `dealsCombatDamage` keep
     `ZoneFits` because they forward their hole into a constructor whose
     requirement side is a variable `Maybe Zone` (`sourceZone from`,
     `damageSourceZone kind`); a `ZoneIs` hole there does not convert.
  4. `Blocks`/`BecomesBlocked` gained `Triggers.PatientZoneIs`, so an absent
     patient (`Blocks n Nothing`) is admitted while a present one is strict —
     previously both rode `zoneFits`'s permissive clause.
  5. F5's stated fix ("some head type is a deed type") alone regressed
     `badCantDisjunctSubject` ("target creature or land can't block"), so the
     head-type projection became alternatives. The "none is excluded" half is
     *not* implemented: `DeedRole` has no exclusion set, and deriving one from
     `permanentType` misfires on `Kindred` (Tribal Enchantment is a permanent
     type combination `permanentType` calls False).
  6. `onFieldZone`/`onStackZone` folded onto `zoneIsB` — same comparison, two
     more spellings, not named by the ticket.
  7. `Phrase.deedAltOk` is exported so `attackableKind` keeps a flat-list
     entry point (`SoleTy t` has no alternatives).
  8. `Phrase.ascribable`/`Ascribable` had to move above `LinkSource` in the
     `mutual` block: a `So` synonym only reduces once its body is elaborated,
     and `SortedSelfLinked` needs it to reduce. `SortedSelfLinked` now passes
     `{asc = Oh}` rather than binding the witness.
- STOP: the ticket's "Done when" requires `zoneFits`'s permissive branch to be
  gone. Making it strict (`zoneFits _ Nothing = True; zoneFits subj (Just b) =
  zoneIsB subj b`) refuses printed sentences whose subject's zone is exactly
  what the sentence asks about, because `nounZone This = Nothing` is the same
  `Nothing` as an unzoned description. Measured casualties: `Macros.monstrosity`
  (`Matches This (HasDesignation Monstrous)`, [CR#701.37a]) fails to elaborate,
  and `Matches`'s `zc` gate then refuses Quakebringer (`Matches This (InZone
  battlefieldZ)` / `(graveyardOf You)`, Cards:9719-9720), Skyblade's Boon
  (:9752-9753), Veiling Oddity (`InZone exileZ`, :13079), Haakon, Stromgald
  Scourge (:13340) and Conqueror's Flail (`AttachedTo`, :15334) — "as long as
  this card is exiled" cannot require that the card is already exiled.
  Resolution: `ZoneIs` is the gate wherever a definite zone is *required*;
  `zoneFits` stays the two-`Maybe` compatibility relation, now with exactly
  one permissive clause (`zoneFits Nothing (Just _) = True`) and its
  comparison delegated to `zoneIsB`, so there is still one comparison and one
  strict gate. The F4 defect itself is closed: `gets (target source)` and
  `destroy (target source)` are both refused, and both are pinned.

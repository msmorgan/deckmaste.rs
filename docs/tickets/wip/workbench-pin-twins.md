---
needs: []
---
**Give every pin its positive twin in the same module.** Cleanroom review 3,
2026-09-04, §2d census: 84 of 628 pins are twinless.

- `VERIFY.md` requires each `Unspellable` pin to carry its positive twin in
  the same module. 84 pins carry their evidence in another module, in a bench
  file, or nowhere. None is vacuous — every twinless pin's positive form was
  probed and admitted — so this is a discipline defect, not a soundness one.
- Distribution over the pin families: Description and Anaphora 6; Trigger,
  Damage and Zone 4; Keyword 9; Counters and Mana 10; Deontic, Choice and
  Static 25; Cost, Faces, Turn, Copy and Piles 30. `ProofsStatic` is fully
  twinned and is the reference shape — adjacent pairs, every refusal on the
  named implicit.
- Write each twin in the pin's own module next to it: the same sentence with
  the refused part corrected, and a synthetic rules-meaningful sentence where
  no printed card supplies one. Pin modules import no other `Proofs*` module,
  so a twin is never borrowed across modules.
- Wire the check into `idris/scripts/build` beside the bench brace lint, so a
  new twinless pin fails the build.

Size: M. Done when: every `Unspellable` declaration has an adjacent positive
twin in its own module; the check is wired and green; the added count is
reported; build at its module count. Standard constraints apply, including the
RON-shaped constraint.

## As landed

- Wired `idris/scripts/check-pin-twins` into `idris/scripts/build` beside the
  bench brace lint: for every `Unspellable` pin, the nearest preceding
  declaration — walking back over the pins that share it and over the
  `Bindings` values that stage their contexts — must be a positive definition
  at the same type head. `ProofsStatic` passes untouched (the reference shape);
  the check reported 126 twinless pins in 13 modules before the round.
- `ProofsTrigger` (2 pins, 2 twins): `afterLifePayment` +
  `okLifePaymentPayerReadback` ("Whenever a player pays life, that player draws
  a card") twins `badPassivePayerReadback`; `okLifePaymentThatMuch` ("you gain
  that much life") twins `badKeywordCostPaymentThatMuch`.
- `ProofsPiles` (4 pins, 3 twins): `okPileWordAfterPartition` twins
  `badCardWordReadsPiles`/`badPileWordWithoutAPartition`;
  `okStatusOnPermanentAfterPartition` twins `badPileFaceAsAStatus`;
  `okDoorHeaderOnSharedLine` (a `SharedLineSplit` Room) twins
  `badDelayedDoorDeixis`.
- `ProofsMana` (3 pins, 3 twins): `okDiscardedCardWord` twins
  `badDiscardedCreatureWord`; `okIfDoneReadsDoneBody` twins
  `badIfNotReadsMandatoryBody`; `okContinuousPumpClause` twins
  `badAltCostClause`.
- `ProofsDescription` (5 pins, 4 twins): `okWholeZoneJoin` twins
  `badPartialZoneJoin`; `okDistinctStructuredDisjuncts` twins
  `badRepeatedStructuredDisjunct`; `afterTopLook` +
  `okReadsLookedAtLibraryCard` twin `badReadsShuffledIntoLibraryCard`;
  `okPaidCostOnKeywordWithACost` twins `badPaidCostOnCostlessKeyword` and
  `badTimesPaidUnknownKeyword`.
- `ProofsCounters` (6 pins, 3 twins): `okConsistentConjunction` twins
  `badControlContradiction`/`badAttackingNoncreature` (shared `cf`);
  `okAltHeaderAgreeingReadback` twins the three header-readback pins
  `badAltHeaderMixedReadback`/`badThreeArmHeaderReadback`/`badJoinedHeaderReadback`
  (shared counted-uniqueness read over a multi-arm header);
  `okSameScopeCounterMenu` twins `badMixedScopeCounterMenu`.
- `ProofsTurn` (6 pins, 4 twins): `okActivatedCostAndEffect` twins
  `badHiddenCost`/`badTwoCostMentions`; `okTapOnBattlefield` twins
  `badTapGraveyard`; `okCastSpellClass` twins
  `badActivatedSpellClass`/`badCastAbilityClass`; `oneExtraTurn` +
  `okThatTurnAfterASingleTurn` twin `badThatTurnAfterTwoTurns`.
- `ProofsKeyword` (8 pins, 4 twins): `okQuotedGrantOnPermanent` twins
  `badQuotedGrantOnSpell`; `okReaderAfterChooser` twins the five
  chosen-quality-read pins `badReaderBeforeChooser`/`badTwoChoosersOneSortRead`/
  `badChosenReadWrongSort`/`badChosenProtectionBeforeChoice`/
  `badAscribedQualityBeforeChoice`; `okParameterisedEcho` twins `badBareEcho`;
  `okSharedSubjectDelta` twins `badSharedSubjectEmptyDelta`.
- `ProofsChoice` (9 pins, 6 twins): `okChosenBasicTypeOnLand` twins
  `badChosenBasicTypeOnCreature`; `okLandsAreMountains` twins
  `badCreaturesAreMountains`; `okAddsAColor` twins
  `badAddsNoColor`/`badLosesNoColor`/`badLosesPt`/`badStillOnAddition`;
  `afterChoiceMade` + `okChoiceRestStands` twin `badChoiceRestDisposedTwice`;
  `afterDistributedChoice` + `okDistributedRestStands` twin
  `badDistributedRestDisposedTwice`; `okComparisonBoundOutsideTheMember` twins
  `badMemberInComparisonBound`.
- `ProofsZone` (10 pins, 8 twins): `okPositiveAntecedent` twins
  `badNegatedAntecedent`; `okSingleZoneOnTarget` twins `badConflictingZones`;
  `okConsistentZoneConjunct` twins `badNotOnBattlefield`/`badAttackingInHand`;
  `okConsistentQualityConjunct` twins
  `badQualityContradiction`/`badNestedContradiction`; `okDiscardACardFromHand`
  twins `badDiscardThisCreature`; `okDifferenceUnderComparisonTrigger` twins
  `badNonComparisonDifference`; `okAgentChoosesSomeOf` twins
  `badAgentChooseTheRest`; `okRegenerationBanOnBattlefield` twins
  `badRegeneratedInGraveyard`.
- `ProofsDeontic` (16 pins, 6 twins): `okCreaturePower` twins
  `badNoncreaturePower`; `okDistinctComparisonDisjuncts` twins
  `badRepeatedComparisonDisjunct`/`badMixedCharacteristicDisjunct`;
  `okDeedAsCost` twins the ten written-as-a-cost pins
  (`badContinuousAsCost`..`badRepeatAsCost`, shared `costActionOk`);
  `okCompoundCost` twins `badEmptyCompound`; `okRingBearerOnBattlefield` twins
  `badRingBearerInGraveyard`; `afterDamageDealt` + `okTheDamageAnnounced` twin
  `badTheDamageUnannounced`/`badTheDamageAfterLifeGain`.
- Follow-on twins forced by the check once the twins above split the runs:
  `ProofsTurn.okSacrificeOnBattlefield` (twins
  `badSacrificeExiled`/`badDeadCreatureRead`),
  `ProofsChoice.okSharedGroupWithoutARest` (twins
  `badDistributedRestOfSharedGroup`/`badDistributedRestOfSingularChoice`), and
  in `ProofsDeontic` `badTheDamageAfterLifeGain` moved above the
  `afterDamageDealt` twin so both pins keep an adjacent twin at their own type.
- `ProofsFaces` (19 pins, 13 twins + 2 reorders): `okProtectionOnPermanentCard`,
  `okProtectionFromAColor`, `okEquipOnEquipment`, `okEachOfATargetGroup`,
  `okGainsIndestructible` (also twins `badSpellIndestructible`),
  `okKindredWithAnotherType` (also twins `badKindredAlone`),
  `okEnchantedCreatureCantAttack`, `okLoyaltyOnPlaneswalker`,
  `okEscalateWithModes` (also twins `badEntwineWithoutModes`),
  `okCostedUnearth` (also twins `badCostedRetrace`), `okUnearthOnPermanentCard`
  (also twins `badFlashbackOnPermanentCard`), `okSingularDevotion`,
  `okRoomDoorHeaderOnSharedLine`; the existing `okPlaneswalkerLoyaltyBox` moved
  above `badPlaneswalkerNoLoyalty` (it was the twin, below its pin), and the
  door-header twin/pin pair moved below the two `Unlock` pins so both keep
  their `Instruction` twin.
- `ProofsDamage` (20 pins, 12 twins + 2 reorders): `okFightControlledCreatures`
  (twins the three `Fights` pins), `okAttackingOrBlockingOnBattlefield`,
  `okComplementSameHead` (twins the three complement pins),
  `okDisjunctWithoutAComplement`, `okPreventedFromSourceInAClause`,
  `okPreventedThisWayInAClause` (twins both prevented-this-way pins),
  `okRedirectToOne`, `okScaleDoubled` (twins both `Scale` pins),
  `okThatMuchAfterDamageEvent` (twins both `ThatMuch` trigger pins),
  `okThatCreatureAfterTargetedDamage`, `okLastChosenColorRead`,
  `okLiteralRollRow` (twins both roll-table pins);
  `badShieldSizedByItsOwnPrevention` moved below its `okShieldRepeatedly` twin,
  and the `okLastChosenColorRead`/`badLastChosenColorNoChooser` pair moved below
  the two `Card` last-chosen pins so those keep `okLastChosenAfterChooser`.

## Landing record

**Census (scripted, `idris/scripts/check-pin-twins` over `Proofs*.idr`).**

| | before | after |
|---|---:|---:|
| pins (`… : Unspellable …`) | 631 | 631 |
| positive declarations beside them | 560 | 654 |
| pins with no adjacent twin at their own type head | 126 | 0 |

Per module, twinless before → after: Anaphora 18→0, Choice 9→0, Counters 6→0,
Damage 20→0, Deontic 16→0, Description 5→0, Faces 19→0, Keyword 8→0, Mana 3→0,
Piles 4→0, Static 0→0 (the reference shape, untouched), Trigger 2→0, Turn 6→0,
Zone 10→0.

**Gates.**

- `cd idris && ./scripts/build` (after `rm -rf build`): `46/46: Building Cards
  (src/Cards.idr)`, exit 0, 0 `Error` lines, 0 `Warning` lines, 1m40s.
- The wired check probed non-vacuous: deleting `ProofsStatic.okSingleStaticXRider`
  makes `./scripts/build` exit 1 with
  `src/Experimental/ProofsStatic.idr:59: badDoubleStaticRider has no positive
  twin at StaticSpec` before `idris2` runs; restored, exit 0.
- `cargo xtask cite check --list-noncompliant`: `0 non-compliant
  citation-looking string(s)`.
- `cargo xtask cite check`: `checked 14342 citations against cr.txt
  (eff. 2026-08-07); 0 stale`.
- `jj --no-pager diff --git > /tmp/round.diff && cargo xtask cite audit --diff <
  /tmp/round.diff`: `audited 0 citation site(s) — nothing selected` — the round
  adds no CR citation (the twins restate their pins' sentences; the pins keep
  their own cites unchanged). `cite bless` not run: no new rule cited.

**Assurance counts.** restored 0; re-spelled 0; ignored 0; added 94
declarations (86 positive twins + 8 `Bindings` context stages); removed 0. No
pin was deleted, weakened, or re-spelled — pin count is 631 before and after,
and every pin body (`Oh impossible` / `Refl impossible` / named witnesses) is
byte-identical.

**Deviations and additions.**

- **Census differs from the ticket's 84.** The ticket quotes review 3's
  hand count of pins "carrying their evidence in another module, in a bench
  file, or nowhere". The check the ticket also asks for — *adjacent* positive
  twin in the same module — is stricter: it counts 126, a superset that
  includes pins whose same-typed positive exists in the module but sits behind
  an unrelated declaration or below the pin. All 126 are twinned; 86 twins
  cover them because one twin serves the pins that share its obligation (e.g.
  `ProofsDeontic.okDeedAsCost` covers ten written-as-a-cost pins).
- **The check skips `Bindings` declarations** when walking back from a pin: a
  `Bindings` value stages a pin's context and belongs between the twin and the
  pin (`ProofsTrigger.afterPassivePayment` is the pattern). It matches on the
  type *head*, not the full type, because a context-indexed pin and its twin
  differ in exactly the index or sort the pin refuses.
- **Six declaration moves, no content change:**
  `ProofsFaces.okPlaneswalkerLoyaltyBox` (its pin sat above its twin);
  the `ProofsFaces` door-header twin/pin pair, below the two `Unlock` pins;
  `ProofsDamage.badShieldSizedByItsOwnPrevention`, below `okShieldRepeatedly`;
  the `ProofsDamage` last-chosen `Predicate` twin/pin pair, below the two
  `Card` last-chosen pins; `ProofsDeontic.badTheDamageAfterLifeGain`, above the
  `afterDamageDealt` twin; `ProofsAnaphora.youPaidLifeThisTurn`, down beside
  `badBarePaymentLookback` (it was already that pin's twin).
- **Two name collisions surfaced the defect directly:** `okFightCreatures`
  (`ProofsDamage`) and `okDiscardHandCard` (`ProofsZone`) already existed in
  their modules — a same-shaped positive was present but not adjacent. The new
  twins were named `okFightControlledCreatures` and `okDiscardACardFromHand`.
- Nothing outside `idris/` and this ticket was touched; `cr-citations.lock` is
  unchanged.

**STOP:** none. Every twin elaborated; where a first spelling was refused
(`Macros.aAtRandom (And [IsCard, …])`, `MkRollRow (UpToOf (Lit 20))`,
`That (TypeW Creature)` under a bare `Dies`, `Noun … Object` for a union read)
the fix was in the twin's own spelling, and no pin was loosened to admit one.

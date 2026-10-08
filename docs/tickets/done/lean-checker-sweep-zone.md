---
needs: []
---
**Retire the checker's zone and object-state refusals (33 constructors).**
Per the checker's charter (`docs/decisions/semantics-v2.md` §7, ruling
2026-10-06; the `lean/CONTRACTS.md` header). The checker admits a written term as a faithful reading of the
card's English: references resolve and terms are well-kinded. It is not a
rules engine, and an instruction the game cannot carry out is still a
faithful reading [CR#101.3,609.3]. Standard constraints apply.

## The bucket

| Constructor | `Refusal.lean` line | Pin lines | Candidate emission sites (`lean/Semantics/Check/`) |
|---|---|---|---|
| `zoneIs` | 53 | 54 (15 files) | AbilityRules.lean:13,68,69; PhraseRules.lean:120,277,539,589,642,643,650,655,660,676,686,708 |
| `zoneFits` | 54 | 28 (9 files) | AbilityRules.lean:335; PhraseRules.lean:530,631,666,724,735 |
| `zoneCoherent` | 55 | 20 (8 files) | AbilityRules.lean:259; PhraseRules.lean:192,632 |
| `possessable` | 56 | 1 (1 file) | Events.lean:151; PhraseRules.lean:455 |
| `playableFrom` | 57 | 5 (2 files) | PhraseRules.lean:138,678 |
| `placeArrangementFits` | 58 | 2 (1 file) | PhraseRules.lean:474 |
| `placeOrdinalFits` | 59 | 1 (1 file) | PhraseRules.lean:475 |
| `exposableZone` | 61 | 1 (1 file) | PhraseRules.lean:818 |
| `costSubject` | 62 | 1 (1 file) | AbilityRules.lean:647,650 |
| `paidSubject` | 63 | 1 (1 file) | PhraseRules.lean:408 |
| `stackActOn` | 64 | 1 (1 file) | AbilityRules.lean:239,243; Phrase.lean:1847,1852,1853 |
| `copySourceOk` | 65 | 2 (2 files) | AbilityRules.lean:238 |
| `movable` | 66 | 2 (2 files) | AbilityRules.lean:228; Phrase.lean:1861 |
| `destOk` | 67 | 3 (2 files) | AbilityRules.lean:228; Phrase.lean:1868,1869 |
| `arrangementOk` | 68 | 1 (1 file) | AbilityRules.lean:229; Phrase.lean:205,1872; PhraseRules.lean:474 |
| `placeable` | 69 | 1 (1 file) | AbilityRules.lean:230 |
| `ridersFit` | 70 | 5 (4 files) | AbilityRules.lean:231 |
| `discardOk` | 71 | 0 (0 files) | Phrase.lean:1880 |
| `moveDestination` | 72 | 1 (1 file) | AbilityRules.lean:325 |
| `attachHeadOk` | 74 | 4 (2 files) | PhraseRules.lean:312 |
| `attachFits` | 75 | 5 (2 files) | AbilityRules.lean:186; PhraseRules.lean:89,161,531 |
| `combatRelOk` | 76 | 3 (1 file) | PhraseRules.lean:139,142,644 |
| `damageRecipient` | 77 | 17 (6 files) | AbilityRules.lean:93,163,308,505; Phrase.lean:1817; PhraseRules.lean:583,663; Rules.lean:49 |
| `attackable` | 78 | 3 (3 files) | AbilityRules.lean:83; Phrase.lean:1824; PhraseRules.lean:578,579 |
| `statusHolder` | 79 | 1 (1 file) | AbilityRules.lean:166 |
| `statusMarkable` | 80 | 2 (2 files) | AbilityRules.lean:167,511; PhraseRules.lean:687; Triggers.lean:45 |
| `visibilityOk` | 137 | 1 (1 file) | AbilityRules.lean:713 |
| `attacker` | 184 | 1 (1 file) | PhraseRules.lean:637 |
| `modifyStat` | 193 | 2 (1 file) | AbilityRules.lean:634 |
| `ctrlOverride` | 206 | 1 (1 file) | AbilityRules.lean:516 |
| `controlExchangeZone` | 226 | 1 (1 file) | AbilityRules.lean:50,51 |
| `cardSwapZones` | 227 | 1 (1 file) | AbilityRules.lean:55 |
| `zoneSwap` | 228 | 2 (1 file) | AbilityRules.lean:60 |

Pin lines in the bucket: 174.

The emission-site column is a grep for `.<name>` under `lean/Semantics/Check/`
(Refusal.lean excluded), 2026-10-06. Some matches are a same-named predicate
or another type's constructor (`NounPhrase.discardOk` is a predicate, not an
emission); confirm each before removing it. Pin lines count matching lines
under `lean/Semantics/Proofs/`.

## The work

- Remove each constructor's emission sites and the constructor itself, or
  keep a constructor with a one-line reason where this ticket allows it.
- Re-spell or retire each pin that asserted a retired refusal. A pin whose
  subject still exists is re-spelled against what the checker now says (the
  same term, its new outcome); a pin is removed only when the refusal was its
  whole subject, and the landing record names it.
- Update the facts table and `lean/CONTRACTS.md`: each section this sweep
  touches says what the checker no longer checks.

## Proof

- No reference-resolution or term-shape pin changes outcome.
- `cargo xtask lean check` over `plugins_v2/canon` and `plugins_v2/testing`
  stays all-pass, and `cargo xtask lean definitions` stays all-pass (or
  their regrouped names, `cargo xtask lean check` and `cargo xtask lean
  definitions`, once `semantics-v2-chooser-and-taker-fields` lands).
- `lean/scripts/build` passes.

## Did any retired pin encode a real card defect?

Mandatory. The landing fills this in, one line per retired or re-spelled pin:
**yes**, naming the card and the defect the refusal caught, or **no**. A yes
is reported to the owner with the landing record; it may argue for keeping
the refusal, which needs a dated ruling.

Every one of the 166 pins below is a hand-written sentence or a machinery
fixture; none is a printed card, and no canon or testing card failed a retired
refusal before this landing (the gate's baseline was canon 138/138, testing 7/7). No pin
encoded a real card defect: **0 yes**.

| Pin | File | Retired refusal(s) asserted | Fate | Real card defect? |
|---|---|---|---|---|
| `badAttachmentWithoutHost` | ActionFamilies.lean | `attachFits` | deleted | no: a hand-written sentence, a twin of the rule |
| `exiledAbilityCannotBeCountered` | ActionFamilies.lean | `zoneFits` | deleted | no: a hand-written sentence, a twin of the rule |
| `graveyardDestinationRejectsEntryRiders` | ActionFamilies.lean | `ridersFit` | deleted | no: a hand-written sentence, a twin of the rule |
| `clearDamageRequiresBattlefield` | ActionFamilies.lean | `zoneIs` | deleted | no: a hand-written sentence, a twin of the rule |
| `clearDamageRejectsAPlayer` | ActionFamilies.lean | `zoneIs` | re-spelled to `kindMismatch` kept | no: a hand-written sentence, a twin of the rule |
| `exiledAbilityCannotBeCopiedOnStack` | ActionFamilies.lean | `copySourceOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badMoveToBareLibrary` | Anaphora.lean | `destOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badMoveToStack` | Anaphora.lean | `destOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badStackAnaphorAfterCounteringAbility` | Anaphora.lean | `zoneFits` | re-spelled to the remaining reference refusal | no: a hand-written sentence, a twin of the rule |
| `badAttackerComplementOnObject` | Anaphora.lean | `attackable` | deleted | no: a hand-written sentence, a twin of the rule |
| `badPlayerCastComplement` | Anaphora.lean | `zoneIs` | re-spelled to the remaining `kindMismatch` | no: a hand-written sentence, a twin of the rule |
| `badCastFromStack` | Anaphora.lean | `playableFrom` | deleted | no: a hand-written sentence, a twin of the rule |
| `badCastOriginFromStack` | Anaphora.lean | `playableFrom` | deleted | no: a hand-written sentence, a twin of the rule |
| `badEmptyOriginCoordination` | Anaphora.lean | `playableFrom` | deleted | no: a hand-written sentence, a twin of the rule |
| `badEmptyOriginExclusion` | Anaphora.lean | `playableFrom` | deleted | no: a hand-written sentence, a twin of the rule |
| `badResolvedOnBattlefield` | Anaphora.lean | `zoneIs` | deleted | no: a hand-written sentence, a twin of the rule |
| `badEntryOriginBattlefield` | Anaphora.lean | `zoneCoherent` | deleted | no: a hand-written sentence, a twin of the rule |
| `badDisjunctionOrdered` | Anaphora.lean | `placeArrangementFits` | deleted | no: a hand-written sentence, a twin of the rule |
| `badShuffledArranged` | Anaphora.lean | `placeArrangementFits` | deleted | no: a hand-written sentence, a twin of the rule |
| `badShuffledOrdinal` | Anaphora.lean | `placeOrdinalFits` | deleted | no: a hand-written sentence, a twin of the rule |
| `anyTargetTakesDamage` | Anaphora.lean | `damageRecipient` | re-spelled to `objectOrPlayerCheck` = `[]` | no: a hand-written sentence, a twin of the rule |
| `badDealtThisWayAbility` | Anaphora.lean | `zoneIs` | deleted | no: a hand-written sentence, a twin of the rule |
| `badDestroyInGraveyard` | Anaphora.lean | `zoneFits` | deleted | no: a hand-written sentence, a twin of the rule |
| `badDiscardFromBattlefield` | Anaphora.lean | `zoneFits` | deleted | no: a hand-written sentence, a twin of the rule |
| `badTapLibraryTop` | Anaphora.lean | `zoneIs` | deleted | no: a hand-written sentence, a twin of the rule |
| `badEquippedPlayer` | Attachments.lean | `attachFits` | re-spelled to `kindLte` | no: a hand-written sentence, a twin of the rule |
| `badFortifiedPlayer` | Attachments.lean | `attachFits` | re-spelled to `kindLte` | no: a hand-written sentence, a twin of the rule |
| `badEquipmentAttachedToPlayer` | Attachments.lean | `attachFits` | re-spelled to `kindLte` | no: a hand-written sentence, a twin of the rule |
| `badEquipmentAttachedToLand` | Attachments.lean | `attachFits` | deleted | no: a hand-written sentence, a twin of the rule |
| `badAbsentCounterpartTarget` | Attachments.lean | `zoneIs` | re-spelled to the remaining reference refusal | no: a hand-written sentence, a twin of the rule |
| `badModalReadsAcrossModes` | Choice.lean | `zoneIs` | re-spelled to the remaining reference refusal | no: a hand-written sentence, a twin of the rule |
| `badReadsAfterModal` | Choice.lean | `zoneIs` | re-spelled to the remaining reference refusal | no: a hand-written sentence, a twin of the rule |
| `badRetargetPermanent` | Choice.lean | `stackActOn` | deleted | no: a hand-written sentence, a twin of the rule |
| `badGetsGraveyard` | Choice.lean | `zoneIs` | deleted | no: a hand-written sentence, a twin of the rule |
| `badThirdDamageRecipientAlternative` | Composition.lean | `damageRecipient` | deleted | no: a hand-written sentence, a twin of the rule |
| `badAltHeaderMixedReadback` | Counters.lean | `zoneIs` | re-spelled to the remaining reference refusal | no: a hand-written sentence, a twin of the rule |
| `badMoveCountersSelf` | Counters.lean | `moveDestination` | deleted | no: a hand-written sentence, a twin of the rule |
| `badExileTapped` | Counters.lean | `ridersFit` | deleted | no: a hand-written sentence, a twin of the rule |
| `badOtherwiseReadsIfArm` | Counters.lean | `zoneIs` | re-spelled to the remaining reference refusal | no: a hand-written sentence, a twin of the rule |
| `badTheyIt` | Damage.lean | `zoneIs` | re-spelled to the remaining reference refusal | no: a hand-written sentence, a twin of the rule |
| `badDamageGraveyardCard` | Damage.lean | `damageRecipient` | deleted | no: a hand-written sentence, a twin of the rule |
| `badDamageToColor` | Damage.lean | `damageRecipient` | re-spelled to `kindLte` | no: a hand-written sentence, a twin of the rule |
| `badDamageArtifact` | Damage.lean | `damageRecipient` | deleted | no: a hand-written sentence, a twin of the rule |
| `badDestroyAnyTargetRemention` | Damage.lean | `zoneFits` | deleted | no: a hand-written sentence, a twin of the rule |
| `badDamageThis` | Damage.lean | `damageRecipient` | deleted | no: a hand-written sentence, a twin of the rule |
| `badDamageDisjunctHead` | Damage.lean | `damageRecipient` | deleted | no: a hand-written sentence, a twin of the rule |
| `badAttackingOrBlockingInGraveyard` | Damage.lean | `zoneCoherent` | deleted | no: a hand-written sentence, a twin of the rule |
| `badConditionAntecedent` | Damage.lean | `zoneIs` | re-spelled to the remaining reference refusal | no: a hand-written sentence, a twin of the rule |
| `badPreventedBareThis` | Damage.lean | `damageRecipient` | deleted | no: a hand-written sentence, a twin of the rule |
| `badPreventDealtToArtifact` | Damage.lean | `damageRecipient` | deleted | no: a hand-written sentence, a twin of the rule |
| `badRedirectToArtifact` | Damage.lean | `damageRecipient` | deleted | no: a hand-written sentence, a twin of the rule |
| `badDamageToSource` | Damage.lean | `damageRecipient` | deleted | no: a hand-written sentence, a twin of the rule |
| `badScaleToArtifact` | Damage.lean | `damageRecipient` | deleted | no: a hand-written sentence, a twin of the rule |
| `badSameKindJoinDamage` | Damage.lean | `damageRecipient` | deleted | no: a hand-written sentence, a twin of the rule |
| `badDestroySource` | Damage.lean | `zoneFits` | deleted | no: a hand-written sentence, a twin of the rule |
| `badGetsSource` | Damage.lean | `zoneIs` | deleted | no: a hand-written sentence, a twin of the rule |
| `badUnflipEvent` | Deontic.lean | `statusMarkable` | deleted | no: a hand-written sentence, a twin of the rule |
| `badRingBearerInGraveyard` | Deontic.lean | `zoneCoherent` | deleted | no: a hand-written sentence, a twin of the rule |
| `badOtherwiseReadsLeadingArm` | Description.lean | `zoneIs` | re-spelled to the remaining reference refusal | no: a hand-written sentence, a twin of the rule |
| `badPaidReadOffStack` | Description.lean | `paidSubject` | deleted | no: a hand-written sentence, a twin of the rule |
| `badEmblemPermanent` | Description.lean | `zoneCoherent` | deleted | no: a hand-written sentence, a twin of the rule |
| `absentSourceCannotSupplyReference` | EventPatterns.lean | `damageRecipient` | re-spelled to the remaining reference refusal | no: a hand-written sentence, a twin of the rule |
| `excludedDepartureIsRefused` | EventPatterns.lean | `zoneFits` | deleted | no: a hand-written sentence, a twin of the rule |
| `arrivalCannotBeDescribedInTheOrigin` | EventPatterns.lean | `zoneFits` | deleted | no: a hand-written sentence, a twin of the rule |
| `sharedZoneCannotBeItsOwnDestination` | EventPatterns.lean | `zoneCoherent` | deleted | no: a hand-written sentence, a twin of the rule |
| `controlledGraveyardCardsAreRefused` | EventPatterns.lean | `zoneCoherent` | deleted | no: a hand-written sentence, a twin of the rule |
| `emptyOriginIsNotAZoneChange` | EventPatterns.lean | `zoneCoherent` | deleted | no: a hand-written sentence, a twin of the rule |
| `excludingEveryOriginIsImpossible` | EventPatterns.lean | `zoneCoherent` | deleted | no: a hand-written sentence, a twin of the rule |
| `identicalQualifiedZoneAndItsNegationConflict` | EventPatterns.lean | `zoneCoherent` | deleted | no: a hand-written sentence, a twin of the rule |
| `badUntapNextGraveyard` | Faces.lean | `zoneIs` | deleted | no: a hand-written sentence, a twin of the rule |
| `badTurnFaceDownGraveyard` | Faces.lean | `zoneIs` | deleted | no: a hand-written sentence, a twin of the rule |
| `badPhasesOutInHand` | Faces.lean | `zoneIs` | deleted | no: a hand-written sentence, a twin of the rule |
| `badEquippedPlayer` | Faces.lean | `attachHeadOk` | re-spelled to `kindLte` | no: a hand-written sentence, a twin of the rule |
| `badFortifiedCreatureNoun` | Faces.lean | `attachHeadOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badCopyPermanent` | Faces.lean | `copySourceOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badStackCopyAsToken` | Faces.lean | `zoneIs` | re-spelled to the remaining reference refusal | no: a hand-written sentence, a twin of the rule |
| `badTokenCopyAsCopyMention` | Faces.lean | `zoneIs` | re-spelled to the remaining reference refusal | no: a hand-written sentence, a twin of the rule |
| `badTurnOverOffField` | Faces.lean | `zoneIs` | deleted | no: a hand-written sentence, a twin of the rule |
| `badUnlockDoorOffBattlefield` | Faces.lean | `zoneIs` | deleted | no: a hand-written sentence, a twin of the rule |
| `badDoorOfBareThis` | Faces.lean | `zoneIs` | deleted | no: a hand-written sentence, a twin of the rule |
| `badGetsGraveyardTwoHalves` | GetsBothDeltas.lean | `zoneIs` | deleted | no: a hand-written sentence, a twin of the rule |
| `badAltHeaderMixedReadbackTwoHalves` | GetsBothDeltas.lean | `zoneIs` | re-spelled to the remaining reference refusal | no: a hand-written sentence, a twin of the rule |
| `badGetsSourceTwoHalves` | GetsBothDeltas.lean | `zoneIs` | deleted | no: a hand-written sentence, a twin of the rule |
| `badThatCreatureIsCondSubjectTwoHalves` | GetsBothDeltas.lean | `zoneIs` | re-spelled to the remaining reference refusal | no: a hand-written sentence, a twin of the rule |
| `badCostSubjectOnBattlefield` | Keyword.lean | `costSubject` | deleted | no: a hand-written sentence, a twin of the rule |
| `badCounterPermanent` | Keyword.lean | `zoneFits` | deleted | no: a hand-written sentence, a twin of the rule |
| `badCounterJoinedPlayer` | Keyword.lean | `movable`, `zoneFits` | re-spelled to `kindLte` | no: a hand-written sentence, a twin of the rule |
| `castRejectsEmptyOriginExclusion` | Lookback.lean | `playableFrom` | deleted | no: a hand-written sentence, a twin of the rule |
| `predicateScopesDoNotConcealNegatedZones` | MacroParameters.lean | `zoneCoherent` | deleted | no: a hand-written sentence, a twin of the rule |
| `nestedNounResultsKeepOuterInlineAliasesLive` | MacroParameters.lean | `zoneIs` | re-spelled to `anaphor` ("that permanent" after the move, with a resolving control) | no: a hand-written sentence, a twin of the rule |
| `dependentCapturesFollowDeclarationOrderRatherThanBodyUse` | MacroParameters.lean | `zoneIs` | re-spelled to `anaphor` ("that permanent" after the move, with a resolving control) | no: a hand-written sentence, a twin of the rule |
| `scopedEnactedMoveRetainsSourceZoneCheck` | MacroParameters.lean | `zoneFits` | deleted | no: a hand-written sentence, a twin of the rule |
| `badOwnedBattlefield` | Mana.lean | `possessable` | deleted | no: a hand-written sentence, a twin of the rule |
| `badTappedForManaOffField` | Mana.lean | `zoneFits` | deleted | no: a hand-written sentence, a twin of the rule |
| `badTransformedArrivalOffField` | Piles.lean | `ridersFit` | deleted | no: a hand-written sentence, a twin of the rule |
| `badPlayerMovedToAZone` | Piles.lean | `movable` | re-spelled to `kindLte` | no: a hand-written sentence, a twin of the rule |
| `badPileFaceAsAStatus` | Piles.lean | `statusHolder` | re-spelled to `kindMismatch` | no: a hand-written sentence, a twin of the rule |
| `badDamageResultRecipient` | Rules.lean | `damageRecipient` | deleted | no: a hand-written sentence, a twin of the rule |
| `cardStatChangeOffBattlefieldZone` | Rules.lean | `zoneIs` | deleted | no: a hand-written sentence, a twin of the rule |
| `p1p1CounterCountMultipliedTwin` | Rules.lean | `zoneIs` | re-spelled to `[]`, its no-count-read claim kept | no: a hand-written sentence, a twin of the rule |
| `badThatCreatureIsCondSubject` | Static.lean | `zoneIs` | re-spelled to the remaining reference refusal | no: a hand-written sentence, a twin of the rule |
| `badEquippedLand` | Static.lean | `attachHeadOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badFortifiedCreature` | Static.lean | `attachHeadOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badContradictoryOutsideSelection` | Static.lean | `zoneCoherent` | deleted | no: a hand-written sentence, a twin of the rule |
| `badModifyManaValue` | Static.lean | `modifyStat` | deleted | no: a hand-written sentence, a twin of the rule |
| `badSetLoyalty` | Static.lean | `modifyStat` | deleted | no: a hand-written sentence, a twin of the rule |
| `badThatCreatureIsSelf` | Trigger.lean | `zoneIs` | re-spelled to the remaining reference refusal | no: a hand-written sentence, a twin of the rule |
| `badGraveyardAttacker` | Trigger.lean | `attacker` | deleted | no: a hand-written sentence, a twin of the rule |
| `badMixedAttackDefenderHalves` | Trigger.lean | `attackable` | deleted | no: a hand-written sentence, a twin of the rule |
| `badStale` | Turn.lean | `zoneFits` | deleted | no: a hand-written sentence, a twin of the rule |
| `badHiddenCost` | Turn.lean | `zoneIs` | re-spelled to the remaining reference refusal | no: a hand-written sentence, a twin of the rule |
| `badSacrificeExiled` | Turn.lean | `zoneFits` | deleted | no: a hand-written sentence, a twin of the rule |
| `badTapGraveyard` | Turn.lean | `zoneIs` | deleted | no: a hand-written sentence, a twin of the rule |
| `badTriggerTapsDeadCreature` | Turn.lean | `zoneIs` | deleted | no: a hand-written sentence, a twin of the rule |
| `badLeavesThenTap` | Turn.lean | `zoneIs` | deleted | no: a hand-written sentence, a twin of the rule |
| `badAfterReflexiveReadsTrigger` | Turn.lean | `zoneIs` | re-spelled to the remaining reference refusal | no: a hand-written sentence, a twin of the rule |
| `badReflexiveTapsSacrificed` | Turn.lean | `zoneIs` | deleted | no: a hand-written sentence, a twin of the rule |
| `badBlockingGraveyardRelatum` | Turn.lean | `combatRelOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badAttackedByGraveyardRelatum` | Turn.lean | `combatRelOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badCouldBlockGraveyardRelatum` | Turn.lean | `combatRelOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `damageRejectsArtifactAlternative` | TypeEvidence.lean | `damageRecipient` | deleted | no: a hand-written sentence, a twin of the rule |
| `badTappedGraveyard` | Zone.lean | `zoneCoherent` | deleted | no: a hand-written sentence, a twin of the rule |
| `badUntapGraveyard` | Zone.lean | `zoneIs` | deleted | no: a hand-written sentence, a twin of the rule |
| `badThatPermanentDeparted` | Zone.lean | `zoneIs` | deleted | no: a hand-written sentence, a twin of the rule |
| `badTokenGraveyard` | Zone.lean | `zoneCoherent` | deleted | no: a hand-written sentence, a twin of the rule |
| `badThatTokenOfCard` | Zone.lean | `zoneIs` | re-spelled to the remaining reference refusal | no: a hand-written sentence, a twin of the rule |
| `badUnloopedZoneMoveRead` | Zone.lean | `zoneIs` | deleted | no: a hand-written sentence, a twin of the rule |
| `badDiesInGraveyard` | Zone.lean | `zoneFits` | deleted | no: a hand-written sentence, a twin of the rule |
| `badConflictingZones` | Zone.lean | `zoneCoherent` | deleted | no: a hand-written sentence, a twin of the rule |
| `badMoveToTargetsHand` | Zone.lean | `destOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badDestroyGraveyard` | Zone.lean | `zoneFits` | deleted | no: a hand-written sentence, a twin of the rule |
| `badDiscardBattlefield` | Zone.lean | `zoneFits` | deleted | no: a hand-written sentence, a twin of the rule |
| `badDestroyGraveyardCard` | Zone.lean | `zoneFits` | deleted | no: a hand-written sentence, a twin of the rule |
| `badNotOnBattlefield` | Zone.lean | `zoneCoherent` | deleted | no: a hand-written sentence, a twin of the rule |
| `badAttackingInHand` | Zone.lean | `zoneCoherent` | deleted | no: a hand-written sentence, a twin of the rule |
| `badDiscardThisCreature` | Zone.lean | `zoneFits` | deleted | no: a hand-written sentence, a twin of the rule |
| `badDiscardIt` | Zone.lean | `zoneFits` | deleted | no: a hand-written sentence, a twin of the rule |
| `badControlledInGraveyard` | Zone.lean | `zoneCoherent` | deleted | no: a hand-written sentence, a twin of the rule |
| `badTrailingPostStateZone` | Zone.lean | `zoneFits` | deleted | no: a hand-written sentence, a twin of the rule |
| `badSimultaneousReadsRetag` | Zone.lean | `zoneFits` | deleted | no: a hand-written sentence, a twin of the rule |
| `badTapAfterSimultaneousExile` | Zone.lean | `zoneIs` | deleted | no: a hand-written sentence, a twin of the rule |
| `badTapAfterSequentialExile` | Zone.lean | `zoneIs` | deleted | no: a hand-written sentence, a twin of the rule |
| `badGainControlGraveyard` | Zone.lean | `zoneIs` | deleted | no: a hand-written sentence, a twin of the rule |
| `badSingularOrderRider` | Zone.lean | `arrangementOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badSearchZonedDescription` | Zone.lean | `zoneCoherent` | deleted | no: a hand-written sentence, a twin of the rule |
| `badWouldDieInGraveyard` | Zone.lean | `zoneFits` | deleted | no: a hand-written sentence, a twin of the rule |
| `badMoveRidersToGraveyard` | Zone.lean | `ridersFit` | deleted | no: a hand-written sentence, a twin of the rule |
| `badMoveControlToHand` | Zone.lean | `ridersFit` | deleted | no: a hand-written sentence, a twin of the rule |
| `badMoveRidersPluralController` | Zone.lean | `ctrlOverride` | deleted | no: a hand-written sentence, a twin of the rule |
| `badExiledWithControlled` | Zone.lean | `zoneCoherent` | deleted | no: a hand-written sentence, a twin of the rule |
| `badExiledWithAttacking` | Zone.lean | `zoneCoherent` | deleted | no: a hand-written sentence, a twin of the rule |
| `badInstantOntoBattlefield` | Zone.lean | `placeable` | deleted | no: a hand-written sentence, a twin of the rule |
| `badRemoveFromCombatGraveyard` | Zone.lean | `zoneIs` | deleted | no: a hand-written sentence, a twin of the rule |
| `badExileCheckOnSortedSelf` | Zone.lean | `zoneFits` | deleted | no: a hand-written sentence, a twin of the rule |
| `badCreatureAttackDefender` | Zone.lean | `attackable` | deleted | no: a hand-written sentence, a twin of the rule |
| `badLookAtHandRider` | Zone.lean | `visibilityOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badRegenerateInGraveyard` | Zone.lean | `zoneFits`, `zoneIs` | deleted | no: a hand-written sentence, a twin of the rule |
| `badRegenerateBareThis` | Zone.lean | `zoneIs` | deleted | no: a hand-written sentence, a twin of the rule |
| `badRevealGraveyard` | Zone.lean | `exposableZone` | deleted | no: a hand-written sentence, a twin of the rule |
| `badInterceptReplacementAntecedent` | Zone.lean | `zoneFits` | re-spelled to the remaining reference refusal | no: a hand-written sentence, a twin of the rule |
| `badUnflipInstruction` | Zone.lean | `statusMarkable` | deleted | no: a hand-written sentence, a twin of the rule |
| `badExchangeControlInGraveyard` | Zone.lean | `controlExchangeZone` | deleted | no: a hand-written sentence, a twin of the rule |
| `badExchangeCardsSameZone` | Zone.lean | `cardSwapZones` | deleted | no: a hand-written sentence, a twin of the rule |
| `badExchangeZoneWithItself` | Zone.lean | `zoneSwap` | deleted | no: a hand-written sentence, a twin of the rule |
| `badExchangeBattlefieldZone` | Zone.lean | `zoneSwap` | deleted | no: a hand-written sentence, a twin of the rule |
| `badExchangeTextBoxInGraveyard` | Zone.lean | `zoneIs` | deleted | no: a hand-written sentence, a twin of the rule |

## Also in this sweep

- **The bearer exemption goes.** `Conferral.checkProperty`
  (`lean/Semantics/Check/Rules.lean:74-78`) exempts a two-stat
  `ptModification` on `this` from the battlefield demand; with the demand
  retired the exemption has nothing to exempt and is deleted (§7 ruling
  2026-10-06, "The bearer exemption stays narrow until the zone sweep").
- **The twin goes with it.** `p1p1CounterCountMultipliedTwin`
  (`lean/Semantics/Proofs/Rules.lean:178-191`, cited at
  `lean/CONTRACTS.md` in the counter-definition section) pins two
  `.zoneIs .battlefield` refusals; re-spell it against what the checker now
  says, and update the CONTRACTS paragraphs that cite it.
- **Zone state on bindings stays** (§7 ruling 2026-10-06, clause (c)):
  `that(Permanent)` against `that(Card)` resolves on it. This sweep removes
  refusals, not fold-state; a change that strips zone from a binding is a
  STOP.
- `zoneIs` alone is 54 pin lines in 15 files.
- `discardOk` has no emission site today (the match in `Check/Phrase.lean` is
  the predicate `NounPhrase.discardOk`); retire the constructor and decide
  whether the predicate has another user.

## Landing record

Measured on the pins commits `kxnvryrllmqy` and the liveness re-spelling above
`nnzuqsrypstr` (which is documentation only), after `kata refresh` onto trunk `krnpolwy`,
2026-10-08. No coverage lock, grammar construction or Rust crate is in play:
the change touches `lean/` and `docs/` only.

Commits: `kmxszwsy` emission sites and the bearer exemption; `mzytuysl` the
33 `Refusal` constructors; `kxnvryrl` the pins; `nnzuqsry` CONTRACTS, the ADR
note, this record.

### PROVE

- **No silent loss.** Every pin that asserted a retired refusal is named in
  the table above with its fate: 132 deleted (the refusal was its whole
  claim, retired by the §7 charter ruling of 2026-10-06), 34 re-spelled.
  `cargo xtask lean check`: canon 138/138, testing 8/8 (the eighth card,
  Airbend Probe, arrived with the refresh, not with this change; no card
  newly fails and none newly passes, since every card passed before).
  `cargo xtask lean definitions`: 553/553 (73 counters, 461 subtypes,
  19 designations). `cargo xtask facts check`: up to date.
- **Structural laws.** `lean/scripts/build` (`lake build --wfail`, 88 jobs):
  clean at the tip. No reference-resolution or term-shape pin changed
  outcome: the 22 pins that asserted a zone refusal beside an anaphor or kind
  refusal keep exactly those refusals. Zone state on bindings is untouched
  (`Binding`, `Payload`, `moveIntro`, `setZone` unchanged).
- **No word-naming.** No guard names a lexeme, construction or card.
- **Cites.** `cite check --list-noncompliant` empty; `cite check` 0 stale
  (16,365 citations); `cite audit --diff` from the claim read 16 sites, each
  against its claim. `cite bless` would only drop one unrelated lock entry (a gift rule cited only
  in a done ticket), so the lock is unchanged.
- **Board.** `kata kanban check`: OK.
- **Expansions and lexicon export.** No file under `plugins_v2/`, `crates/`
  or the lexicon changed between the claim and the tip, so both are
  byte-identical by construction; neither was re-run.
- **Rust.** No crate's code or consumed data changed (there is no Rust
  mirror of `Refusal`; the facts generator names none of the 33), so no
  `cargo test -p` or clippy run is owed.

### DISCLOSE

- **Constructors removed (33):** `zoneIs`, `zoneFits`, `zoneCoherent`,
  `possessable`, `playableFrom`, `placeArrangementFits`, `placeOrdinalFits`,
  `exposableZone`, `costSubject`, `paidSubject`, `stackActOn`,
  `copySourceOk`, `movable`, `destOk`, `arrangementOk`, `placeable`,
  `ridersFit`, `discardOk`, `moveDestination`, `attachHeadOk`, `attachFits`,
  `combatRelOk`, `damageRecipient`, `attackable`, `statusHolder`,
  `statusMarkable`, `visibilityOk`, `attacker`, `modifyStat`, `ctrlOverride`,
  `controlExchangeZone`, `cardSwapZones`, `zoneSwap`. None kept.
- **Predicates deleted** (each fed only a retired refusal): in
  `Check/Phrase.lean` `zonesOk` with `zonesAgree`, `negZones`,
  `Predicate.negZonesOf`, `zoneAdmits`, `zoneAdmitsAll`, `Predicate.zoneAdmit`,
  `ZoneExpr.sameKnownZone`, `EventSource.zonesOk`; `LibraryPlace.arrangementOk`,
  `LibraryPlace.ordinalOk`, `ZoneExpr.destOk`, `orderOk`,
  `NounPhrase.moveDestOk`, `paidSubjectOk`, `costSubjectOk`,
  `attackableKind`, `combatPartyKind`, `combatRelOk`, `damageableKind`,
  `NounPhrase.damageRecipient`, `NounPhrase.attackable`, `stackActOn`,
  `counterable`, `copiable`, `counterKind`, `copyKind`, `copySourceOk`,
  `NounPhrase.movable`, `NounPhrase.discardOk` (no other user),
  `NounPhrase.attackerOk`, `visibilityOk`; in `Check/Abilities.lean`
  `controlExchangeZone`, `cardSwapZonesOk`, `zoneSwapOk`,
  `enactPatientZoneOk`, `ctrlOverrideOk`, `ridersZoneFree`, `ridersFitZone`,
  `Stat.modifyOk`; in `Check/Words.lean` `Zone.exposable`, `destTypeOk`,
  `placeableTy`, `Status.markable`, `CombatRelation.bare`,
  `damageableHeadTysOk`; in `Check/EventContext.lean` `EventSource.admits`,
  `transitionSubjectOk`, `transitionEndpointsOk`, `castSourceOk`,
  `CombatRelation.eventRelation`, `patientZone`, `patientZoneIsB`,
  `DamageKind.sourceZone`, `GameEvent.headerStatusOk`. Kept because another
  rule reads them: `zoneIsB`, `zoneFits` and `playableFrom` (`deedFits`,
  `playSourceOk`), `Zone.possessable` (`keptPossessor`, the binding
  fold-state), `attachWordsOk`/`attachHeadOk` (`noAttachHeadClash` under
  `contradictionFree`), `isCardZone`.
- **The bearer exemption** in `Conferral.checkProperty` is deleted with the
  function: `Conferral.check` reads a property through `StaticSpec.check`,
  which is now byte-for-byte what the exemption computed.
  `p1p1CounterCountMultipliedTwin` is re-spelled to `[]` (its remaining claim:
  nothing is refused for reading its own count);
  `cardStatChangeOffBattlefieldZone` is deleted. CONTRACTS and the 2026-10-07
  ruling are annotated (the ruling kept, with a dated superseded note).
- **Borderline decisions, one line each.** The ruling's census counts all 33
  as zone and object state, and each is retired. Where a retired refusal also
  fixed a *kind*, that kind is kept under an existing term-shape constructor,
  so no ill-kinded term newly passes (charter: "terms are well-kinded"):
  - `statusHolder` (`k == object`): `setStatus` checks its subject at kind
    `object` (`kindMismatch`); the battlefield demand and `statusMarkable` go.
  - `movable` (object or pile): `move` refuses other kinds with
    `kindLte k (object ∨ pile)`.
  - `damageRecipient`, `attackable`, `attacker`: the recipient, the attacked
    and the attacker are checked with `objectOrPlayerCheck`
    (`kindLte k (object ∨ player)`); which objects can be dealt damage or
    attacked [CR#120.1a,506.2] and where they are go.
  - `attachFits` (the attachment word's host kind) and `attachHeadOk` (an
    `equipped`/`fortified` player): an Equipment's or Fortification's host
    must be an object (`kindLte k object`); host card types and the
    refusal of a hostless `attach` go.
  - `combatRelOk`: the relation's kinds are kept (`kindMismatch`,
    `objectOrPlayerCheck`); the battlefield demand, the bare-participle list
    and the event-relation list go.
  - `ctrlOverride` (one controller per member), `arrangementOk` (an order
    for a single object), `placeArrangementFits`/`placeOrdinalFits`,
    `modifyStat`, `visibilityOk`, `statusMarkable`, `zoneCoherent`: retired
    whole; each is a rule about what the game can do, not a reference or a
    kind.
  - `designationScope`/`designationChecked`: untouched (game-rules sweep).
- **Deviations and additions.** New emission sites of existing term-shape
  constructors, beyond the ticket's letter, so the kind content of retired
  refusals survives: `objectOrPlayerCheck` (renamed and widened from
  `attachmentHostKindCheck`, `Check/PhraseRules.lean`) at `dealDamage`,
  `distribute` (damage), `DamageScope.toRecipient`, `DamageOp.redirect`,
  `DamagePatient`, the damage event's recipient, `DamageResultRule`,
  `AttackDefender`, `DeonticPatient.defendingPlayer`, the attacker of
  `combat .attackerOf`, and `inCombat`; `kindLte (object ∨ pile)` at `move`;
  `setStatus` at kind `object`; `kindLte object` for equipped/fortified hosts.
  `ZoneScope.checkIn` loses its unused zone parameter. The two
  `MacroParameters` liveness fixtures read back through `that(Permanent)`
  instead of tapping the moved subject, and each gains a control fixture
  (`tapThroughNestedAlias`, `dependentSubjectsAsCard`).
- **Liveness fixtures.** `nestedNounResultsKeepOuterInlineAliasesLive` and
  `dependentCapturesFollowDeclarationOrderRatherThanBodyUse` (alias and capture
  liveness, zone state on bindings, ruling (c)) are re-spelled in their own
  commit: after the move a "that permanent" read of the moved object is
  refused `anaphor`, while the unmoved control (tap instead of move) or a
  "that card" read resolves. The nested pin's subject is `target creature`
  instead of a bare `this`, which introduces nothing for "that" to read.
- **STOPs.** None.
- **Glossary.** No new term needed.

### REPORT

- Counts (Assurance): restored 0, re-spelled 34, ignored 0, added 0,
  removed 132 (each named above with the constructor it asserted; all retired
  by the §7 charter ruling, 2026-10-06, consequence (b)).
- Builds: the first build of the claim was not fully warm (6m26s, the card
  bench rebuilt); `lake build Semantics SemanticsCards --wfail` passed at
  stages (a) and (b), where the pin suites cannot yet pass; the full
  `lake build --wfail` passed from stage (c). `cargo xtask lean check`
  took 115 seconds (lean check's own timer) on a quiet host.

import Semantics.Macros
import Semantics.Check.Card

open Semantics Semantics.Macros

namespace Semantics.Proofs.ActionFamilies

theorem okBlockednessWithoutChangingBlockingRelations :
    Instruction.check [] (.combat thisCreature (.participation (.blocked false))) = [] := by decide

theorem okRemoveAttackedPlaneswalkerFromCombat :
    Instruction.check [] (.combat (.asType .planeswalker .this none)
      (.participation .outsideCombat)) = [] := by decide

theorem okRemoveAttackedBattleFromCombat :
    Instruction.check [] (.combat (.asType .battle .this none)
      (.participation .outsideCombat)) = [] := by decide

theorem okAttachToPlayer :
    Instruction.check [] (.attachment .attached thisAura (some .you)) = [] := by decide

theorem badAttachmentWithoutHost :
    Instruction.check [] (.attachment .attached thisAura none) = [.attachFits] := by decide

theorem okDetachWithoutKnowingOldHost :
    Instruction.check [] (.attachment .unattached thisArtifact none) = [] := by decide

theorem okExtraTurnPublishesTurnReference :
    NounPhrase.check none
      (Instruction.intro [] (.insertPart .turn none (.lit 1) none (some .you))) thatTurn = [] := by decide

theorem badExtraTurnWithoutPlayer :
    Instruction.check [] (.insertPart .turn none (.lit 1) none none) = [.windowOk] := by decide

def emblemAbilities : List Ability := [.static (.abilityGrant (allOf creatureYouControl) (keyword "Haste"))]

theorem okCreateSeveralEmblems :
    Instruction.check [] (.createObject (.lit 2) (.emblem emblemAbilities) .you) = [] := by decide

theorem okEmblemPublication :
    Instruction.introducedDeeds [] (.createObject (.lit 1) (.emblem emblemAbilities) .you) =
      [⟨.a, .one, .object [] (some .command) none none none⟩] := by rfl

theorem badEmblemWithSpellAbility :
    Instruction.check [] (.createObject (.lit 1) (.emblem [.spell none (draw (.lit 1))]) .you) =
      [.emblemAbilities] := by decide


private def exiledAbility : Bindings :=
  Instruction.intro [] (.move (target (.abilityHead .anyActivated)) exileZone [])

private def graveyardAbility : Bindings :=
  Instruction.intro [] (.move (target (.abilityHead .anyActivated)) graveyard [])

theorem exiledAbilityKeepsItsLocation :
    NounPhrase.zone exiledAbility (that .ability) = some .exile := by decide

theorem exiledAbilityRemainsAnAbilityReference :
    NounPhrase.check none exiledAbility (that .ability) = [] := by decide

theorem exiledAbilityIsNotAStackReference :
    NounPhrase.check none exiledAbility (that .stack) = [.anaphor (.word .stack) .one 0] := by decide

theorem exiledAbilityIsNotASpellOrCard :
    NounPhrase.check none exiledAbility (that .spell) = [.anaphor (.word .spell) .one 0] ∧
    NounPhrase.check none exiledAbility (that .card) = [.anaphor (.word .card) .one 0] := by decide

theorem exiledAbilityCannotBeCountered :
    Instruction.check exiledAbility (Primitives.Instruction.counterSpell (that .ability)) = [.zoneFits] := by decide

theorem movementPublishesItsRequiredDestination :
    NounPhrase.zone graveyardAbility (that .ability) = some .graveyard := by decide

theorem movementAcceptsAnAbilitySubject :
    Instruction.check [] (.move (target (.abilityHead .anyActivated)) graveyard []) = [] := by decide

theorem graveyardDestinationRejectsEntryRiders :
    Instruction.check [] (.move (target (.abilityHead .anyActivated)) graveyard [.entersAs .tapped]) =
      [.ridersFit] := by decide

theorem graveyardDestinationAcceptsCounterRiders :
    Instruction.check [] (.move (target (.abilityHead .anyActivated)) graveyard
      [.withCounters (.lit 1) (.printed plusOnePlusOne) .fresh]) = [] := by decide

theorem pronounMovePreservesCopyOriginAndOuterBindings :
    Instruction.intro [⟨.the, .one, .ability (some .copy)⟩, ⟨.the, .one, .player false⟩]
      (.move (that .ability) graveyard []) =
      [⟨.the, .one, .ability (some .copy) (some .graveyard)⟩, ⟨.the, .one, .player false⟩] := by rfl

theorem abilityUnionRetainsCommonLocation :
    (unionPayload (.ability none (some .exile)) (.ability none (some .exile))).map
      (fun pair => pair.2.zone) = some (some .exile) := by decide

theorem abilityUnionDoesNotInventACommonLocation :
    (unionPayload (.ability none (some .exile)) (.ability none (some .stack))).map
      (fun pair => pair.2.zone) = some none := by decide

theorem mixedUnionRetainsCommonLocation :
    (unionPayload (.ability none (some .exile))
      (.object [] (some .exile) none none none)).map
      (fun pair => pair.2.zone) = some (some .exile) := by decide

theorem abilityElementRetainsItsLocation :
    (elemPayload .object true [] (some .exile) none).zone = some .exile := by decide


/-- Marked damage remains even when a permanent stops being a creature [CR#120.6]. -/
theorem clearDamageAdmitsALand :
    Instruction.check [] (.clearDamage (target land)) = [] := by decide

theorem clearDamageRequiresBattlefield :
    Instruction.check [] (.clearDamage (target (.and [creature, .inZone graveyard]))) =
      [.zoneIs .battlefield] := by decide

theorem clearDamageRejectsAPlayer :
    Instruction.check [] (.clearDamage .you) =
      [.kindMismatch .object .player, .zoneIs .battlefield] := by decide

theorem clearDamagePublishesItsSubject :
    NounPhrase.check (some .object) (Instruction.intro [] (.clearDamage (target creature))) it =
      [] := by decide


theorem exiledAbilityCannotBeCopiedOnStack :
    Instruction.check exiledAbility
      (.copy .fromStack (that .ability) (.lit 1) [] .you) = [.copySourceOk] := by decide

theorem clearDamageDoesNotPublishDamageDealt :
    Instruction.introducedDeeds [] (.clearDamage (target creature)) = [] := by rfl


theorem movingThisAbilityRetainsAbilityIdentity :
    Instruction.intro [] (.move thisAbility exileZone []) =
      [⟨.self, .one, .ability none (some .exile)⟩] := by rfl


private def operand (i : Nat) : NounPhrase := .pro .bare .one (.parameter 0 i)

/-- Equal-looking target phrases name two objects, rather than one shared payload. -/
theorem distinctOperandsHaveDistinctAddresses :
    let bs := captureOperands [] [target creature, target creature]
    (operandAddress bs 0, operandAddress bs 1) = (some [2], some [1]) := by decide

theorem capturesDoNotDuplicateOrdinaryPronounCandidates :
    countReach .bare .one (captureOperands [] [target creature]) = 1 := by decide

theorem captureUpdatesTheOriginalReference :
    let bs := captureOperands [] [target creature, target creature]
    let moved := moveIntro bs none (operand 0) (some .exile)
    ((operand 0).zone moved, (operand 1).zone moved,
      (closeOperands moved).map Binding.zone) =
      (some .exile, some .battlefield, [some .battlefield, some .exile]) := by decide

theorem laterMentionsDoNotShiftCapturedOperands :
    let bs := captureOperands [] [target creature]
    let more := selfSubjIntro bs (target land)
    ((operand 0).ty more, (operand 0).zone more) = ([.creature], some .battlefield) := by decide

/-- X already exists: a later target's X does not create a second numeric definition. -/
theorem captureUsesActualNumericIntroductions :
    let bs : Bindings := [letterB .x, ⟨.the, .one,
      .object [.creature] (some .battlefield) none none none⟩]
    let old : NounPhrase := .pro .bare .one (.below 1)
    let second := target (.and [creature, .compare [.stat .power] .eq (.letter .x)])
    let bound := captureOperands bs [old, second]
    (operandAddress bound 0, operandAddress bound 1) = (some [3], some [1]) := by decide

theorem nestedCaptureUpdatesTheOuterOperand :
    let outer := captureOperands [] [target creature]
    let inner := captureOperands outer [operand 0]
    let moved := moveIntro inner none (operand 0) (some .exile)
    (operand 0).zone (closeOperands moved) = some .exile := by decide

theorem nestedCaptureUpdatesAnInlineOuterOperand :
    let outer := captureOperands [] [.asMarker .ability .this]
    let inner := captureOperands outer [operand 0]
    let moved := moveIntro inner none (operand 0) none
    (operand 0).zone (closeOperands moved) = none := by decide

theorem fightAcceptsAnExistingReference :
    Instruction.check (nomIntro [] (target creature))
      (fight it (target creature)) = [] := by decide

theorem fightAcceptsItsOwnSource :
    Instruction.check [] (fight thisCreature thisCreature) = [] := by decide

theorem regenerationApplicationChecks :
    Instruction.check [] (regenerationApplication thisCreature) = [] := by decide

theorem regenerationInstallationPublishesNoImmediateOutcome :
    (regenerate thisCreature).introducedDeeds [] = [] := by decide

theorem counterDoesNotLeaveAnAbilityOnTheStack :
    NounPhrase.zone (Instruction.intro []
      (Primitives.Instruction.counterSpell (target (.abilityHead .anyActivated))))
      (that .ability) = some .graveyard := by decide

theorem counterPreservesTheAbilityCategory :
    NounPhrase.check none (Instruction.intro []
      (Primitives.Instruction.counterSpell (target (.abilityHead .anyActivated))))
      (that .ability) = [] := by decide

theorem counterIsSingleEnactedMove (subject : NounPhrase) :
    Primitives.Instruction.counterSpell subject =
      .enact (.action "Counter") (.move subject graveyard []) := by rfl

theorem losingCountersPublishesTheSharedRemovalOutcome :
    countOutcomes .countersRemoved (Instruction.intro []
      (loseCounters (some (.printed (.named "Poison")))
        (some (.lit 1)) .you)) = 1 := by decide

theorem ownPowerCanReadAnExistingReference :
    Instruction.check (nomIntro [] (target creature)) (dealDamageOwnPower it .you) = [] := by decide


/-- Forgetting a library mention cannot redirect a captured operand to its neighbour. -/
theorem capturedOperandSurvivesOrdinaryShuffleForgetting :
    let bound := captureOperands [] [a (.inZone yourLibrary), target creature]
    let shuffled := afterShuffle bound
    let moved := moveIntro shuffled none (operand 0) (some .exile)
    ((operand 0).zone moved, (operand 1).zone moved,
      (closeOperands moved).map Binding.zone) =
      (some .exile, some .battlefield, [some .battlefield]) := by decide

theorem postposedCounterConditionReadsTheUnmovedSpell :
    Instruction.check [] (.doOnlyIf (Primitives.Instruction.counterSpell (target spell))
      (costWasPaid (.byKeyword "Kicker") none it) none) = [] := by decide


theorem losingCountersAmountReadsItsPlayer :
    Instruction.check [] (loseCounters
      (some (.printed (.named "Poison"))) (some (lifeTotalOf they)) (a .opponent)) = [] := by decide

theorem regenerationCanProtectANoncreaturePermanent :
    Instruction.check [] (regenerate (target artifact)) = [] := by decide


theorem alternativeDoesNotObserveThePrimaryBranchesMove :
    Instruction.check [] (.doOnlyIf (move (target spell) exileZone)
      (.matches it (.colorIs .red)) (some (Primitives.Instruction.counterSpell it))) = [] := by decide


theorem aliasesRemainOneObjectAfterShuffle :
    let bound := captureOperands [] [a (.inZone yourLibrary), it]
    let moved := moveIntro (afterShuffle bound) none (operand 0) (some .graveyard)
    ((operand 0).zone moved, (operand 1).zone moved) =
      (some .graveyard, some .graveyard) := by decide

theorem aliasesAcrossNestedFramesRemainOneObjectAfterShuffle :
    let outer := captureOperands [] [a (.inZone yourLibrary)]
    let inner := captureOperands outer [operand 0]
    let moved := moveIntro (afterShuffle inner) none (operand 0) (some .graveyard)
    (operand 0).zone (closeOperands moved) = some .graveyard := by decide

theorem conditionalForgettingKeepsTheOperandScope :
    Instruction.check [] (.withBindings 0 [.subject (a (.inZone yourLibrary))] (.sequentially [
      .doIf (.matches .you .anyPlayer) (.shuffle .you) none,
      .move (operand 0) graveyard []])) = [] := by decide

theorem capturePreservesTheResolvedPermanentView :
    Instruction.check [] (fight
      (.resolvedPermanent (target (.and [spell, creature]))) (target creature)) = [] := by decide


theorem alternativeReadsTheAmountWithoutObservingTheUntakenDamage :
    let context := Instruction.otherwiseCtx [] (.dealDamage .this (.lit 3) .you)
    Amount.check context .thatMuch = [] ∧
      Condition.check context (.dealtThisWay .anyPlayer) = [.damageDealtInScope] := by decide

end Semantics.Proofs.ActionFamilies

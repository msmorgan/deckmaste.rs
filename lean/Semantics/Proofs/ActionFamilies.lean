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
  Instruction.intro [] (.move (target (.abilityHead .anyActivated)) (some exileZone) [])

private def removedAbility : Bindings :=
  Instruction.intro [] (.move (target (.abilityHead .anyActivated)) none [])

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
    Instruction.check exiledAbility (.counterSpell (that .ability)) = [.stackActOn] := by decide

theorem absentDestinationHasNoArrivalZone :
    NounPhrase.zone removedAbility (that .ability) = none := by decide

theorem absentDestinationAcceptsStackExit :
    Instruction.check [] (.move (target (.abilityHead .anyActivated)) none []) = [] := by decide

theorem absentDestinationRejectsEntryRiders :
    Instruction.check [] (.move (target (.abilityHead .anyActivated)) none [.entersAs .tapped]) =
      [.ridersFit] := by decide

theorem absentDestinationRejectsCounterRiders :
    Instruction.check [] (.move (target (.abilityHead .anyActivated)) none
      [.withCounters (.lit 1) (.printed plusOnePlusOne) .fresh]) = [.ridersFit] := by decide

theorem pronounExitPreservesCopyOriginAndOuterBindings :
    Instruction.intro [⟨.the, .one, .ability (some .copy)⟩, ⟨.the, .one, .player false⟩]
      (.move (that .ability) none []) =
      [⟨.the, .one, .ability (some .copy) none⟩, ⟨.the, .one, .player false⟩] := by rfl

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
    Instruction.intro [] (.move thisAbility (some exileZone) []) =
      [⟨.self, .one, .ability none (some .exile)⟩] := by rfl

end Semantics.Proofs.ActionFamilies

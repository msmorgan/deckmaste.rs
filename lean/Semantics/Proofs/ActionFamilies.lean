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

end Semantics.Proofs.ActionFamilies

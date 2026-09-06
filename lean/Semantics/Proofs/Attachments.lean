import Semantics.Macros
import Semantics.Check.Card

open Semantics Semantics.Macros

namespace Semantics.Proofs.Attachments

/-- The enchanted-player domain used by Curse of Exhaustion is also a valid host predicate. -/
theorem okEnchantedPlayer :
    Condition.check [] (.matches .you (.attachment .host (some .enchanted) none)) = [] := by decide

theorem okEnchantedPlayerWithModifierFirst :
    NounPhrase.check (some .player) []
      (a (.and [.attachment .host (some .enchanted) none, .anyPlayer])) = [] := by decide

theorem okEnchantedPlayerHasNoZone :
    NounPhrase.zone [] (a (.and [.anyPlayer, .attachment .host (some .enchanted) none])) = none := by rfl

theorem badEquippedPlayer :
    Condition.check [] (.matches .you (.attachment .host (some .equipped) none)) =
      [.attachFits] := by decide

theorem badFortifiedPlayer :
    Predicate.check .player [] (.attachment .host (some .fortified) none) = [.attachFits] := by decide

theorem okPlayerHostWithNoWord :
    Predicate.check .player [] (.attachment .host none none) = [] := by decide

theorem badPlayerAsAttachment :
    Predicate.check .player [] (.attachment .attachment none none) =
      [.kindMismatch .player .object] := by decide

theorem okAuraAttachedToPlayer :
    Condition.check [] (.matches thisAura (.attachment .attachment (some .enchanted) (some .you))) =
      [] := by decide

theorem badEquipmentAttachedToPlayer :
    Predicate.check .object [] (.attachment .attachment (some .equipped) (some .you)) =
      [.attachFits] := by decide

theorem badEquipmentAttachedToLand :
    Predicate.check .object [] (.attachment .attachment (some .equipped) (some thisLand)) =
      [.attachFits] := by decide

theorem okCountedAuraAttachments :
    Condition.check [] (.matches thisCreature (.attachment .host (some .enchanted)
      (some (counted (atLeast 2) (.hasSubtype (enchantmentType "Aura")))))) = [] := by decide

theorem badPlayerCounterpartOnHostSide :
    Predicate.check .object [] (.attachment .host none (some .you)) =
      [.kindMismatch .object .player] := by decide

theorem okAttachedHostRetainsCreatureType :
    NounPhrase.ty [] (a (.and [creature, .attachment .host (some .equipped) none])) =
      [.creature] := by rfl

theorem okNestedCounterpartTargetRemainsAvailable :
    Instruction.check [] (.sequentially [
      exile (target (.and [artifact, .attachment .attachment none (some (target creature))])),
      .setStatus .tapped (that (.type .creature))]) = [] := by decide

theorem badAbsentCounterpartTarget :
    Instruction.check [] (.sequentially [
      exile (target (.and [artifact, .attachment .attachment none (some thisCreature)])),
      .setStatus .tapped (that (.type .creature))]) =
      [.anaphor (.word (.type .creature)) .one 0, .zoneIs .battlefield] := by decide

theorem transformedMacroReadsWholeBackFace :
    Primitives.Predicate.isTransformed = Predicate.currentFace .back := by rfl

theorem frontAndBackAreDistinct :
    Predicate.currentFace .front ≠ Predicate.currentFace .back := by
  intro equal
  cases equal

theorem currentFaceIsNotFaceDownStatus :
    Predicate.currentFace .back ≠ Predicate.hasStatus .faceDown := by
  intro equal
  cases equal

theorem badCurrentFaceOnPlayer :
    Predicate.check .player [] (.currentFace .back) = [.kindMismatch .player .object] := by decide

theorem currentFaceDoesNotSupplyTokenEvidence :
    Predicate.seedsToken (.currentFace .back) = false := by rfl

end Semantics.Proofs.Attachments

import Semantics.Macros
import Semantics.Macros.Declared
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
      [.kindLte .player .object] := by decide

theorem badFortifiedPlayer :
    Predicate.check .player [] (.attachment .host (some .fortified) none) =
      [.kindLte .player .object] := by decide

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
      [.kindLte .player .object] := by decide

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
      [.anaphor (.word (.type .creature)) .one 0] := by decide

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

/-! ## The attach deed's host role (ruling 2026-10-10) -/

/-- "This Equipment may be attached to a creature" [CR#301.5,701.3a]. -/
theorem okEquipmentMayBeAttachedToACreature :
    StaticSpec.check [] (mayBeAttachedTo thisEquipment (allOf creature)) = [] := by decide

/-- An Aura may be attached to a player [CR#702.5d]: the host role admits either kind. -/
theorem okAuraMayBeAttachedToAPlayer :
    StaticSpec.check [] (mayBeAttachedTo thisAura (allOf .anyPlayer)) = [] := by decide

/-- A deed with no host role takes no host. -/
theorem badHostOfAHostlessDeed :
    StaticSpec.check []
      (.deonticRule thisPermanent .permit [.action "destroy"] .patient none (.host (allOf creature))
        none .noRider none) = [.deonticPatientOk] := by decide

/-! ## The chapter axis (ruling 2026-10-10) -/

/-- "This Saga's final chapter number" folds one object's chapter numbers [CR#714.2d]. -/
theorem okFinalChapterNumber :
    Amount.check [] (finalChapterNumber thisSaga) = [] := by decide

/-- A single-valued axis still folds only a group. -/
theorem badGreatestPowerOfOne :
    Amount.check [] (.aggregate .max (.stat .power) thisCreature) = [.plural] := by decide

/-- "A chapter ability of this Saga whose chapter number is its lore counter count"
[CR#702.155a]. -/
theorem okChapterAbilityAtLoreCount :
    NounPhrase.check none []
      (allOf (.and [chapterAbilityOf thisSaga,
        .compare [.chapter] .eq (countersOn (.named "loreCounter") thisSaga)])) = [] := by decide

/-- Read ahead's body as its declaration writes it: chapter abilities of this Saga whose chapter
number is not its lore counter count can't trigger the turn it entered [CR#702.155a]. -/
theorem okReadAhead : Ability.check [] Macros.Declared.readAhead = [] := by decide

/-- Enchant's body as its declaration writes it, here over "enchant creature": the Aura spell's
target, the legal-host permission, and the entry clause [CR#702.5a,303.4a,303.4c,303.4f]. -/
theorem okEnchantCreature : Ability.check [] (Macros.Declared.enchant creature) = [] := by decide

/-- An Aura may enchant a player [CR#702.5d]: "enchant player". -/
theorem okEnchantPlayer : Ability.check [] (Macros.Declared.enchant .anyPlayer) = [] := by decide

/-! ## Protection's attach rows -/

/-- Protection from black, its five rows as its declaration writes them: can't be targeted by
black spells or abilities from black sources, can't be enchanted by black Auras, equipped by black
Equipment or fortified by black Fortifications, damage from black sources prevented, and can't be
blocked by black creatures [CR#702.16b..702.16f]. The attach rows name this permanent in the
attach deed's host role. -/
theorem okProtectionFromBlack :
    Ability.check [] (Macros.Declared.protection (.colorIs .black)) = [] := by decide

/-- "Black Auras can't be attached to this permanent": the attachment in the patient role, this
permanent in the host role [CR#701.3a,702.16c]. -/
theorem okCantBeAttachedTo :
    StaticSpec.check []
      (Macros.Declared.cantBeAttachedTo (allOf (.and [enchantment, .colorIs .black])) thisPermanent)
      = [] := by decide

end Semantics.Proofs.Attachments

import Semantics.Definitions
import Semantics.Check.Card

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.KeywordArguments

/-- "Cycling {2}": a keyword's arguments are its typed Definition's, so their number and order
are fixed by its signature. -/
theorem unqualifiedCostVariantRemainsAvailable :
    Ability.check (DRS.ofList []) (cycling (.mana [generic 2])) = [] := by decide

/-- "Playercycling" describes no card: typecycling's body searches for a card with the quality,
so the quality is an object's [CR#702.29e]. -/
theorem qualifiedCostRequiresObjectQuality :
    Ability.check (DRS.ofList []) (typecycling .isPlayer (.mana [generic 2])) =
      [.kindMismatch .object .player] := by decide

/-- "Equip player" and "splice onto player" describe no creature and no spell: equip's body
targets a creature with the quality [CR#702.6c] and splice's reveals a spell with it
[CR#702.47a], and a conjunction whose conjuncts name kinds that do not overlap describes
nothing. -/
theorem equipAndSpliceRequireObjectQuality :
    Ability.check (DRS.ofList []) (equipQuality .isPlayer (.mana [generic 1])) =
        [.kindMismatch .player .object] ∧
      Ability.check (DRS.ofList []) (Ability.splice .isPlayer (.mana [generic 1])) =
        [.kindMismatch .object .player] := by decide

/-- "Plainscycling {2}" through the `typecycling` Definition. -/
theorem okTypecycling :
    Ability.check (DRS.ofList []) (typecycling (.hasSubtype (plains)) (.mana [generic 2]))
      = [] := by decide

/-- A typecycling ability is a cycling ability [CR#702.29f]: it carries the cycling label that
an "abilityHead (.keyword (.named "cycling"))" class names, as plain cycling does. -/
theorem typecyclingIsCycling :
    keywordOf (typecycling (.hasSubtype (plains)) (.mana [generic 2])) =
      .the (.named "cycling") ∧
    keywordOf (cycling (.mana [generic 2])) = .the (.named "cycling") := by decide

theorem qualityCanReadOuterChoice :
    Ability.check [(ChoiceSort.quality .color).binding]
      (keywordQuality protection (ofChosen .color)) = [] := by decide

theorem subjectCannotReadOuterChoice :
    Ability.check [(ChoiceSort.quality .color).binding]
      (keywordSubject enchant (ofChosen .color)) =
      [.choiceRef .theChoice (.quality .color) 0] := by decide

theorem amountCannotReadOuterChoice :
    Ability.check [(ChoiceSort.quality .number).binding]
      (keywordNumber bushido (.chosenNumber .theChoice)) =
      [.choiceRef .theChoice (.quality .number) 0] := by decide

theorem costCannotReadOuterChoice :
    Ability.check [(ChoiceSort.quality .number).binding]
      (keywordCosting ward (payLife (.chosenNumber .theChoice))) =
      [.choiceRef .theChoice (.quality .number) 0] := by decide

end Semantics.Proofs.KeywordArguments

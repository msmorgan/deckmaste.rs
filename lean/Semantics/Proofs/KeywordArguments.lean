import Semantics.Definitions
import Semantics.Check.Card

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.KeywordArguments

theorem qualifiedCostUsesDeclaredOrder :
    Ability.check (DRS.ofList []) (keywordQualityCosting cycling
      (.hasSubtype (landType "Plains")) (.mana [generic 2])) = [] := by decide

theorem reversedQualifiedCostIsRefused :
    Ability.check (DRS.ofList []) (.keyword (.named "cycling")
      [.cost (.mana [generic 2]), .quality (.hasSubtype (landType "Plains"))] []) =
      [.keywordParamFits (.named "cycling")] := by decide

theorem unqualifiedCostVariantRemainsAvailable :
    Ability.check (DRS.ofList []) (keywordCosting cycling (.mana [generic 2])) = [] := by decide

theorem numberedCostNeedsBothArguments :
    Ability.check (DRS.ofList []) (keywordCosting suspend (.mana [generic 2])) =
      [.keywordParamFits (.named "suspend")] := by decide

theorem extraArgumentIsRefused :
    Ability.check (DRS.ofList []) (.keyword (.named "suspend")
      [.number (.lit 2), .cost (.mana [generic 2]), .number (.lit 3)] []) =
      [.keywordParamFits (.named "suspend")] := by decide

theorem qualifiedCostRequiresObjectQuality :
    Ability.check (DRS.ofList []) (keywordQualityCosting cycling .anyPlayer (.mana [generic 2])) =
      [.kindMismatch .object .player] := by decide

/-- "Plainscycling {2}" through the `typecycling` Definition. -/
theorem okTypecycling :
    Ability.check (DRS.ofList []) (typecycling (.hasSubtype (landType "Plains")) (.mana [generic 2]))
      = [] := by decide

/-- A typecycling ability is a cycling ability [CR#702.29f]: it carries the cycling label that
an "abilityHead (.keyword (.named "cycling"))" class names, as plain cycling does. -/
theorem typecyclingIsCycling :
    keywordOf (typecycling (.hasSubtype (landType "Plains")) (.mana [generic 2])) =
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
      (keywordCosting ward (.perform (loseLife (.chosenNumber .theChoice)))) =
      [.choiceRef .theChoice (.quality .number) 0] := by decide

theorem emptyAndMissingSchemaAreDistinct :
    keywordParamFits (.named "flying") [] = true ∧ keywordParamFits (.named "flyign") [] = false := by decide

end Semantics.Proofs.KeywordArguments

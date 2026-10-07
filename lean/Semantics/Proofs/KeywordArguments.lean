import Semantics.Macros
import Semantics.Check.Card

open Semantics Semantics.Macros

namespace Semantics.Proofs.KeywordArguments

theorem qualifiedCostUsesDeclaredOrder :
    Ability.check [] (.keyword (.named "cycling")
      [.quality (.hasSubtype (landType "Plains")), .cost (.mana [generic 2])] []) = [] := by decide

theorem reversedQualifiedCostIsRefused :
    Ability.check [] (.keyword (.named "cycling")
      [.cost (.mana [generic 2]), .quality (.hasSubtype (landType "Plains"))] []) =
      [.keywordParamFits (.named "cycling")] := by decide

theorem unqualifiedCostVariantRemainsAvailable :
    Ability.check [] (.keyword (.named "cycling") [.cost (.mana [generic 2])] []) = [] := by decide

theorem numberedCostNeedsBothArguments :
    Ability.check [] (.keyword (.named "suspend") [.cost (.mana [generic 2])] []) =
      [.keywordParamFits (.named "suspend")] := by decide

theorem extraArgumentIsRefused :
    Ability.check [] (.keyword (.named "suspend")
      [.number (.lit 2), .cost (.mana [generic 2]), .number (.lit 3)] []) =
      [.keywordParamFits (.named "suspend")] := by decide

theorem qualifiedCostRequiresObjectQuality :
    Ability.check [] (.keyword (.named "cycling") [.quality .anyPlayer, .cost (.mana [generic 2])] []) =
      [.kindMismatch .object .player] := by decide

theorem qualityCanReadOuterChoice :
    Ability.check [(ChoiceSort.quality .color).binding]
      (.keyword (.named "protection") [.quality (ofChosen .color)] []) = [] := by decide

theorem subjectCannotReadOuterChoice :
    Ability.check [(ChoiceSort.quality .color).binding]
      (.keyword (.named "enchant") [.subject (ofChosen .color)] []) =
      [.choiceRef .theChoice (.quality .color) 0] := by decide

theorem amountCannotReadOuterChoice :
    Ability.check [(ChoiceSort.quality .number).binding]
      (.keyword (.named "bushido") [.number (.chosenNumber .theChoice)] []) =
      [.choiceRef .theChoice (.quality .number) 0] := by decide

theorem costCannotReadOuterChoice :
    Ability.check [(ChoiceSort.quality .number).binding]
      (.keyword (.named "ward") [.cost (.perform (loseLife (.chosenNumber .theChoice)))] []) =
      [.choiceRef .theChoice (.quality .number) 0] := by decide

theorem emptyAndMissingSchemaAreDistinct :
    keywordParamFits (.named "flying") [] = true ∧ keywordParamFits (.named "flyign") [] = false := by decide

end Semantics.Proofs.KeywordArguments

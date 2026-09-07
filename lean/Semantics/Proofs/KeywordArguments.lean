import Semantics.Macros
import Semantics.Check.Card

open Semantics Semantics.Macros

namespace Semantics.Proofs.KeywordArguments

theorem qualifiedCostUsesDeclaredOrder :
    Ability.check [] (.keyword "Cycling"
      [.quality (.hasSubtype (landType "Plains")), .cost (.mana [generic 2])] []) = [] := by decide

theorem reversedQualifiedCostIsRefused :
    Ability.check [] (.keyword "Cycling"
      [.cost (.mana [generic 2]), .quality (.hasSubtype (landType "Plains"))] []) =
      [.keywordParamFits "Cycling"] := by decide

theorem unqualifiedCostVariantRemainsAvailable :
    Ability.check [] (.keyword "Cycling" [.cost (.mana [generic 2])] []) = [] := by decide

theorem numberedCostNeedsBothArguments :
    Ability.check [] (.keyword "Suspend" [.cost (.mana [generic 2])] []) =
      [.keywordParamFits "Suspend"] := by decide

theorem extraArgumentIsRefused :
    Ability.check [] (.keyword "Suspend"
      [.number (.lit 2), .cost (.mana [generic 2]), .number (.lit 3)] []) =
      [.keywordParamFits "Suspend"] := by decide

theorem qualifiedCostRequiresObjectQuality :
    Ability.check [] (.keyword "Cycling" [.quality .anyPlayer, .cost (.mana [generic 2])] []) =
      [.kindMismatch .object .player] := by decide

theorem qualityCanReadOuterChoice :
    Ability.check [(ChoiceSort.quality .color).binding]
      (.keyword "Protection" [.quality (ofChosen .color)] []) = [] := by decide

theorem subjectCannotReadOuterChoice :
    Ability.check [(ChoiceSort.quality .color).binding]
      (.keyword "Enchant" [.subject (ofChosen .color)] []) =
      [.choiceRef .theChoice (.quality .color) 0] := by decide

theorem amountCannotReadOuterChoice :
    Ability.check [(ChoiceSort.quality .number).binding]
      (.keyword "Bushido" [.number (.chosenNumber .theChoice)] []) =
      [.choiceRef .theChoice (.quality .number) 0] := by decide

theorem costCannotReadOuterChoice :
    Ability.check [(ChoiceSort.quality .number).binding]
      (.keyword "Ward" [.cost (.perform (loseLife (.chosenNumber .theChoice)))] []) =
      [.choiceRef .theChoice (.quality .number) 0] := by decide

theorem emptyAndMissingSchemaAreDistinct :
    keywordParamFits "Flying" [] = true ∧ keywordParamFits "Flyign" [] = false := by decide

end Semantics.Proofs.KeywordArguments

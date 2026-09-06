import Semantics.Macros
import Semantics.Check.Card

open Semantics Semantics.Macros

namespace Semantics.Proofs.Composition

theorem okThreeConjunctsShareEarlierMention :
    NounPhrase.check (some .object) [] (.and [target creature, it, it]) = [] := by decide

theorem badAlternativesCannotReadSibling :
    NounPhrase.check (some .object) [] (.or [target creature, it, thisCreature]) =
      [.anaphor .bare .one 0] := by decide

theorem okAlternativesReadOuterMention :
    NounPhrase.check (some .object) (NounPhrase.introduced [] (target creature))
      (.or [it, thisCreature, it]) = [] := by decide

theorem badEmptyConjunction :
    NounPhrase.check none [] (.and []) = [.atLeastTwo] := by decide

theorem badSingletonConjunction :
    NounPhrase.check none [] (.and [thisCreature]) = [.atLeastTwo] := by decide

theorem badEmptyAlternatives :
    NounPhrase.check none [] (.or []) = [.atLeastTwo] := by decide

theorem badSingletonAlternatives :
    NounPhrase.check none [] (.or [thisCreature]) = [.atLeastTwo] := by decide

theorem okThreeCosts :
    Cost.check [] (.or [.mana [generic 1], .mana [generic 2], .mana [generic 3]]) = [] := by decide

theorem badEmptyCostAlternatives :
    Cost.check [] (.or []) = [.atLeastTwo] := by decide

theorem badSingletonCostAlternative :
    Cost.check [] (.or [.mana [generic 1]]) = [.atLeastTwo] := by decide

theorem badThirdCompoundCostAlternative :
    Cost.check [] (.or [.mana [generic 1], .mana [generic 2], .compound [.mana [generic 3]]]) =
      [.notCompound] := by decide

theorem okCostAlternativeKeepsSymbolProperties :
    Cost.payable (.or [.mana [generic 1], .mana [generic 2], .tapSymbol]) = false ∧
    Cost.offBattlefield (.or [.mana [generic 1], .mana [generic 2], .untapSymbol]) = false ∧
    Cost.selfTapUses (.or [.tapSymbol, .untapSymbol, .mana [generic 1]]) = 1 := by decide

theorem okOptionalSuccessReadsBody :
    Instruction.check [] (.withContinuation (.optional .you) (exile (a creatureYouControl))
      (some (move it battlefield)) none) = [] := by decide

theorem badOptionalFailureReadsBody :
    Instruction.check [] (.withContinuation (.optional .you) (exile (a creatureYouControl))
      none (some (exile it))) = [.anaphor .bare .one 0] := by decide

theorem okRequiredSuccessReadsBody :
    Instruction.check [] (.withContinuation .required (exile (a creatureYouControl))
      (some (move it battlefield)) none) = [] := by decide

theorem badRequiredFailureReadsBody :
    Instruction.check [] (.withContinuation .required (exile (a creatureYouControl))
      none (some (exile it))) = [.anaphor .bare .one 0] := by decide

theorem okNumericDefinitionAsCost :
    Cost.check [letterB .x] (.perform (.establish (.letterDefinition .x (.lit 2)) none)) = [] := by decide

theorem badContinuousSpecAsCost :
    Cost.check [] (.perform (.establish (.modification thisCreature .power (.up (.lit 1))) none)) =
      [.costAction] := by decide

theorem badDefinitionWithoutEarlierUse :
    Instruction.check [] (.establish (.letterDefinition .x (.lit 2)) none) = [.openLetter .x] := by decide

theorem okFixedRepetitionKeepsOuterBindings :
    Instruction.check [letterB .x] (.repeat_ (.fixed (.letter .x) (draw (.lit 1)))) = [] := by decide

theorem okFixedRepetitionPublishesCount :
    Instruction.introducedDeeds [] (.repeat_ (.fixed (.lit 3) (draw (.lit 1)))) =
      [outcomeB .repeatCount] := by rfl

theorem badEmptySequentialComposition :
    Instruction.check [] (.sequentially []) = [.nonEmpty] := by decide

theorem badEmptySimultaneousComposition :
    Instruction.check [] (.simultaneously []) = [.nonEmpty] := by decide

theorem badDurationBearingDefinitionAsCost :
    Cost.check [letterB .x]
      (.perform (.establish (.letterDefinition .x (.lit 2)) (some untilEndOfTurn))) =
      [.costAction] := by decide

theorem okThreeDamageRecipientAlternatives :
    Instruction.check [] (.dealDamage .this (.lit 1) (.or [thisCreature, .you, thisCreature])) =
      [] := by decide

theorem badThirdDamageRecipientAlternative :
    Instruction.check [] (.dealDamage .this (.lit 1) (.or [thisCreature, .you, thisLand])) =
      [.damageRecipient] := by decide

end Semantics.Proofs.Composition

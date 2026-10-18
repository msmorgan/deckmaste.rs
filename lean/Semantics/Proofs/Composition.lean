import Semantics.Definitions
import Semantics.Check.Card

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.Composition

private def nounResult (drs : DRS) (noun : NounPhrase) : DRS :=
  (stepNounPhrase 64 drs .other false noun).1

theorem okThreeConjunctsShareEarlierMention :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (.and [target creature, it, it]) = [] := by decide

theorem badAlternativesCannotReadSibling :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (.or [target creature, it, thisCreature]) =
      [.anaphor .bare .one 0] := by decide

theorem okAlternativesReadOuterMention :
    NounPhrase.checkAt (some .object) (nounResult (DRS.ofList []) (target creature))
      (.or [it, thisCreature, it]) = [] := by decide

theorem badEmptyConjunction :
    NounPhrase.checkAt none (DRS.ofList []) (.and []) = [.atLeastTwo] := by decide

theorem badSingletonConjunction :
    NounPhrase.checkAt none (DRS.ofList []) (.and [thisCreature]) = [.atLeastTwo] := by decide

theorem badEmptyAlternatives :
    NounPhrase.checkAt none (DRS.ofList []) (.or []) = [.atLeastTwo] := by decide

theorem badSingletonAlternatives :
    NounPhrase.checkAt none (DRS.ofList []) (.or [thisCreature]) = [.atLeastTwo] := by decide

theorem okThreeCosts :
    Cost.check (DRS.ofList []) (.or [.mana [generic 1], .mana [generic 2], .mana [generic 3]]) = [] := by decide

theorem badEmptyCostAlternatives :
    Cost.check (DRS.ofList []) (.or []) = [.atLeastTwo] := by decide

theorem badSingletonCostAlternative :
    Cost.check (DRS.ofList []) (.or [.mana [generic 1]]) = [.atLeastTwo] := by decide

theorem badThirdCompoundCostAlternative :
    Cost.check (DRS.ofList []) (.or [.mana [generic 1], .mana [generic 2], .compound [.mana [generic 3]]]) =
      [.notCompound] := by decide

theorem okCostAlternativeKeepsSymbolProperties :
    Cost.offBattlefield (.or [.mana [generic 1], .mana [generic 2], .untapSymbol]) = false := by
  decide

theorem okOptionalSuccessReadsBody :
    Action.check (DRS.ofList []) (.withContinuation .optional (exile (a creatureYouControl))
      (some (move it battlefield)) none) = [] := by decide

theorem badOptionalFailureReadsBody :
    Action.check (DRS.ofList []) (.withContinuation .optional (exile (a creatureYouControl))
      none (some (exile it))) = [.anaphor .bare .one 0] := by decide

theorem okRequiredSuccessReadsBody :
    Action.check (DRS.ofList []) (.withContinuation .required (exile (a creatureYouControl))
      (some (move it battlefield)) none) = [] := by decide

theorem badRequiredFailureReadsBody :
    Action.check (DRS.ofList []) (.withContinuation .required (exile (a creatureYouControl))
      none (some (exile it))) = [.anaphor .bare .one 0] := by decide

theorem okNumericDefinitionAsCost :
    Cost.check [letterB .x] (.perform (.establish (.letterDefinition .x (.lit 2)) none)) = [] := by decide

theorem badDefinitionWithoutEarlierUse :
    Action.check (DRS.ofList []) (.establish (.letterDefinition .x (.lit 2)) none) = [.openLetter .x] := by decide

theorem okFixedRepetitionKeepsOuterBindings :
    Action.check [letterB .x] (.repeat_ (.fixed (.letter .x) (draw (.lit 1)))) = [] := by decide

theorem okFixedRepetitionPublishesCount :
    ((stepAction 64 .postEvent .obj (DRS.ofList [])
      (.repeat_ (.fixed (.lit 3) (draw (.lit 1))))).1.visibleReferents.filter
        (fun ref => match ref.payload with | .outcome .repeatCount => true | _ => false)).length = 1 := by decide

theorem badEmptySequentialComposition :
    Action.check (DRS.ofList []) (.sequentially []) = [.nonEmpty] := by decide

theorem badEmptySimultaneousComposition :
    Action.check (DRS.ofList []) (.simultaneously []) = [.nonEmpty] := by decide

theorem okThreeDamageRecipientAlternatives :
    Action.check (DRS.ofList []) (Action.instruct .this (.dealDamage (.lit 1) (.or [thisCreature, .you, thisCreature]) .any)) =
      [] := by decide

end Semantics.Proofs.Composition

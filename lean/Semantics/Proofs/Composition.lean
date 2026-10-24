import Semantics.Definitions
import Semantics.Check.Card

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.Composition

private def nounResult (drs : DRS) (noun : NounPhrase) : DRS :=
  (stepNounPhrase 64 drs .other false noun).1

theorem okThreeConjunctsShareEarlierMention :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (.and [target isCreature, it, it]) = [] := by decide

theorem badAlternativesCannotReadSibling :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (.or [target isCreature, it, thisCreature]) =
      [.anaphor .bare .one 0] := by decide

theorem okAlternativesReadOuterMention :
    NounPhrase.checkAt (some .object) (nounResult (DRS.ofList []) (target isCreature))
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

theorem okOptionalSuccessReadsBody :
    Action.check (DRS.ofList []) (.withContinuation .optional (exile (a (and [isCreature, youControl])))
      (some (putInto it battlefield)) none) = [] := by decide

theorem badOptionalFailureReadsBody :
    Action.check (DRS.ofList []) (.withContinuation .optional (exile (a (and [isCreature, youControl])))
      none (some (exile it))) = [.anaphor .bare .one 0] := by decide

theorem okRequiredSuccessReadsBody :
    Action.check (DRS.ofList []) (.withContinuation .required (exile (a (and [isCreature, youControl])))
      (some (putInto it battlefield)) none) = [] := by decide

theorem badRequiredFailureReadsBody :
    Action.check (DRS.ofList []) (.withContinuation .required (exile (a (and [isCreature, youControl])))
      none (some (exile it))) = [.anaphor .bare .one 0] := by decide

theorem okNumericDefinitionAsCost :
    Cost.check [letterB .x] (.perform (.establish (.letterDefinition .x 2) none)) = [] := by decide

theorem badDefinitionWithoutEarlierUse :
    Action.check (DRS.ofList []) (.establish (.letterDefinition .x 2) none) = [.openLetter .x] := by decide

theorem okFixedRepetitionKeepsOuterBindings :
    Action.check [letterB .x] (.repeat_ (.fixed (.letter .x) (draw 1))) = [] := by decide

theorem okFixedRepetitionPublishesCount :
    ((stepAction 64 .postEvent .obj (DRS.ofList [])
      (.repeat_ (.fixed 3 (draw 1)))).1.visibleReferents.filter
        (fun ref => match ref.payload with | .outcome .repeatCount => true | _ => false)).length = 1 := by decide

theorem badEmptySequentialComposition :
    Action.check (DRS.ofList []) (.sequentially []) = [.nonEmpty] := by decide

theorem badEmptySimultaneousComposition :
    Action.check (DRS.ofList []) (.simultaneously []) = [.nonEmpty] := by decide

theorem okThreeDamageRecipientAlternatives :
    Action.check (DRS.ofList []) (Action.instruct .this (.dealDamage 1 (.or [thisCreature, .you, thisCreature]))) =
      [] := by decide

/-! ## Disjunction and negation scoping (`semantics-v3.md` §1.2, §1.3) -/

private def kinds (drs : DRS) : List (ReferentId × Kind) := drs.referents.map (fun r => (r.id, r.kind))

/-- "you or a white creature you control": neither alternative's participant outlives the
disjunction, which an earlier mention is read through as any mention reads it. -/
theorem nounAlternativesKeepNoBranchParticipant :
    kinds (nounResult (DRS.ofList []) (.or [.you, a (.and [isCreature, .colorIs .white])])) = [] ∧
      kinds (nounResult (nounResult (DRS.ofList []) (target isCreature)) (.or [it, .you])) =
        [(0, .object)] := by
  decide

/-- Alternative locations ("from your hand or graveyard") keep only what both introduce, the
one "you"; joined ones are stepped in order and keep both owners. -/
theorem locationAlternativesKeepOnlyWhatEveryAlternativeIntroduces :
    let step := fun (loc : Location) => kinds (stepLocationWith (stepOps 64) (DRS.ofList []) loc).1
    step (.coord .or [.zone .hand (some .you), .zone .graveyard (some .you)]) = [(0, .player)] ∧
      step (.coord .or [.zone .hand (some .you), .zone .graveyard (some anOpponent)]) = [] ∧
      step (.coord .and [.zone .hand (some .you), .zone .graveyard (some anOpponent)]) =
        [(1, .player), (0, .player)] := by
  decide

/-- A negated location ("anywhere other than your graveyard") introduces nothing the text after
it reads, as a negated Timing or Condition does. -/
theorem negatedLocationIntroducesNothing :
    kinds (stepLocationWith (stepOps 64) (DRS.ofList []) (.not (.zone .graveyard (some .you)))).1
      = [] := by
  decide

/-- "during your turn or an opponent's turn": the alternatives' owners differ, so neither
outlives the Timing; where both name "you", the one "you" does. -/
theorem timingAlternativesKeepOnlyWhatEveryAlternativeIntroduces :
    let step := fun (tm : Timing) => kinds (stepTimingWith (stepOps 64) (DRS.ofList []) tm).1
    step (.or [.inPart .turn (some .you) .current, .inPart .turn (some anOpponent) .current]) =
        [] ∧
      step (.or [.inPart .turn (some .you) .current, .inPart .combat (some .you) .current]) =
        [(0, .player)] := by
  decide

end Semantics.Proofs.Composition

import Semantics.Check.NumberSlots
import Semantics.Macros
import Semantics.Check.Card

open Semantics Semantics.Macros

namespace Semantics.Proofs.GetsBothDeltas

private def instructionResult (drs : DRS) (instruction : Instruction) : DRS :=
  (stepInstruction 64 .postEvent .obj drs instruction).1

theorem getsPtIsOneNode (subject : NounPhrase) (power toughness : Delta Amount) :
    getsPt subject power toughness = .ptModification subject power toughness := rfl

theorem okTargetGetsIntroducesOneTarget :
    (instructionResult (DRS.ofList []) (get (target creature) (.up (.lit 2)) (.up (.lit 2)) (some untilEndOfTurn))).referents.countP
      (fun b => b.det == .target) = 1 := by
  decide

theorem subjectWrittenTwiceIntroducesTwoTargets :
    (instructionResult (DRS.ofList [])
      (.establish
        (.conjunction none
          [ .modification (target creature) .power (.up (.lit 2)),
            .modification (target creature) .toughness (.up (.lit 2)) ])
        (some untilEndOfTurn))).referents.countP (fun b => b.det == .target) = 2 := by
  decide

theorem ptModificationSlots :
    (StaticSpec.ptModification thisCreature (.up (.lit 2)) (.set (.lit (-1)))).numberSlots =
      [(.lit 2, .clamped), (.lit (-1), .signed)] := by
  rfl

theorem okDeltasReadTheirSubject :
    Instruction.check (DRS.ofList [])
      (get (target creature) (.up (.statOf (.stat .power) it)) (.up (.statOf (.stat .power) it))
        (some untilEndOfTurn)) = [] := by
  decide

theorem okGetsInsideCoordination :
    StaticSpec.check (DRS.ofList [])
      (.conjunction none
        [getsPt (target creature) (.up (.lit 1)) (.up (.lit 1)), .abilityGrant it (keyword (.named "flying"))])
      = [] := by
  decide

private def selectedSubjectContext : DRS :=
  instructionResult (DRS.ofList []) (.sequentially
    [.setStatus .tapped (target artifact),
      get (.firstMention "pump.subject" (target creature))
        (.up (powerOf it)) (.up (powerOf it)) (some untilEndOfTurn)])

theorem selectedPtSubjectHasStableIdentity :
    (identityBinding "pump.subject" selectedSubjectContext).map (·.id) = some 1 ∧
      (firstReach .bare .one selectedSubjectContext).map (·.id) = some 1 ∧
      (selectedSubjectContext.findById? 1).map (·.ty) = some [.creature] := by decide

theorem deltasReadSelectedSubjectAfterUnrelatedObject :
    Instruction.check (DRS.ofList []) (.sequentially
      [.setStatus .tapped (target artifact),
        get (.firstMention "pump.subject" (target creature))
          (.up (powerOf it)) (.up (powerOf it)) (some untilEndOfTurn)]) = [] := by decide

theorem pluralPtSubjectIsIntroducedOnce :
    let after := instructionResult (DRS.ofList [])
      (get (counted (exactly 2) creature) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn))
    after.visibleReferents.length = 1 ∧
      (firstReach .bare .many after).map (·.id) = some 0 ∧
      countReach .bare .one after = 0 := by decide

theorem ambiguousSingularPtSubjectIsRefused :
    let before := (stepNounPhrase 64 (DRS.ofList []) .subj true
      (.and [target creature, target artifact])).1
    StaticSpec.check (nextClause before)
      (.ptModification it (.up (.lit 1)) (.up (.lit 1))) =
      [.anaphor .bare .one 2] := by decide

theorem nestedModernCoordinationIsRefused :
    StaticSpec.check (DRS.ofList [])
      (.conjunction none
        [.conjunction none
          [.ptModification thisCreature (.up (.lit 1)) (.up (.lit 1)),
            .abilityGrant thisCreature (keyword (.named "flying"))],
          .abilityGrant thisCreature (keyword (.named "trample"))]) = [.notCoord] := by decide

end Semantics.Proofs.GetsBothDeltas

import Semantics.Macros

open Semantics Semantics.Macros

namespace Semantics.Proofs.ControllerSacrifice

private def subject : NounPhrase := target (.and [artifact, creature])
private def instruction : Instruction := controllerSacrifices subject
private def afterSacrifice : DRS :=
  (stepInstruction 64 .postEvent .obj (DRS.ofList []) instruction).1
private def nestedSubject : NounPhrase := target (.and [creature,
  .hasPossessor .controller (controllerOf (target artifact))])
private def nestedInstruction : Instruction := controllerSacrifices nestedSubject

-- Parameter identity selects the target before its zone change.
theorem expansionIntroducesTheTargetOnce :
    instruction = .instruct
      (.possessorOf .controller (.firstMention "controllerSacrifices.subject" subject))
      (sacrifice (.laterMention "controllerSacrifices.subject" subject)) := by rfl

theorem targetMultiplicity :
    afterSacrifice.referents.countP (·.det == .target) = 1 := by decide

theorem instructionIsAdmitted :
    Instruction.check (DRS.ofList []) instruction = [] := by decide

theorem laterCardRetainsBothTypes :
    (firstReach (.word .card) .one afterSacrifice).map (·.ty) =
      some [.creature, .artifact] := by decide

theorem laterCardRetainsGraveyardCarrier :
    (firstReach (.word .card) .one afterSacrifice).map (·.zone) =
      some (some .graveyard) := by decide

theorem laterPlayerReadIsAdmitted :
    NounPhrase.checkAt (some .player) afterSacrifice (that .player) = [] := by decide

theorem laterCardReadIsAdmitted :
    NounPhrase.checkAt (some .object) afterSacrifice (that .card) = [] := by decide

theorem nestedDescriptionIsAdmitted :
    Instruction.check (DRS.ofList []) nestedInstruction = [] := by decide

theorem nestedDescriptionDeclaresExactlyItsTwoTargets :
    (stepInstruction 64 .postEvent .obj (DRS.ofList []) nestedInstruction).1.referents.countP
      (·.det == .target) = 2 := by decide

-- P08: replacing the explicit parameter read with an ordinary pronoun leaves a true tie.
theorem wholeOwnedScopeWouldBeAmbiguous :
    Instruction.check (DRS.ofList [])
      (instruct (controllerOf nestedSubject) (sacrifice it)) =
        [.anaphor .bare .one 2] := by decide

theorem sacrificeStampSelectsTheSameObject :
    (firstReach (.verbed (.action "sacrifice") none .attributive) .one afterSacrifice).map (·.id) =
      (firstReach (.word .card) .one afterSacrifice).map (·.id) := by decide

theorem formerPermanentReadIsAdmitted :
    NounPhrase.checkAt (some .object) afterSacrifice
      (theVerbed (.action "sacrifice") .one (some .permanent) .thisWay) = [] := by decide

theorem thatPlayerCanTakeTheFollowingAction :
    Instruction.check (DRS.ofList [])
      (.sequentially [instruction, instruct (that .player) (draw (.lit 1))]) = [] := by decide

private def nestedAfter : DRS :=
  (stepInstruction 64 .postEvent .obj (DRS.ofList []) nestedInstruction).1

theorem nestedDescriptionMovesOnlyItsOwnReferent :
    (firstReach (.word (.type .artifact)) .one nestedAfter).map (·.zone) = some (some .battlefield) ∧
    (firstReach (.word .card) .one nestedAfter).map (·.zone) = some (some .graveyard) := by decide

private def authoringWitness : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Controller sacrifice witness", types := [.sorcery], cost := some [generic 1],
      text := [Primitives.Ability.spell none
        (controllerSacrifices (target (Primitives.Predicate.and [artifact, creature])))] } }

theorem compositionPassesTheAuthoringBoundary :
    authoringWitness.authoring.onlyMacros = true := by decide

end Semantics.Proofs.ControllerSacrifice

import Semantics.Macros

open Semantics Semantics.Macros

namespace Semantics.Proofs.ControllerSacrifice

private def subject : NounPhrase := target (.and [artifact, creature])
private def instruction : Instruction := controllerSacrifices subject
private def controllerBinding : Binding := ⟨.the, .one, .player false⟩
private def targetBinding : Binding := ⟨.target, .one,
  .object [.creature, .artifact] (some .battlefield) none none (some 1)⟩
private def sacrificedBinding : Binding := ⟨.target, .one,
  .object [.creature, .artifact] (some .graveyard)
    (some ⟨.action "Sacrifice", true, true⟩) none (some 1)⟩
private def riderBinding : Binding := ⟨.target, .one,
  .object [.creature, .artifact] (some .battlefield)
    (some ⟨.action "Sacrifice", true, false⟩) none (some 1)⟩
private def afterSacrifice : Bindings := Instruction.intro [] instruction

/-- The target occurs only in the relational subject. The body is a scoped re-read. -/
theorem expansionIntroducesTheTargetOnce :
    instruction = .enact (.action "Sacrifice")
      (.move (.pro .bare .one (.introduced [.player, .object])) graveyard [])
      (agent := some (controllerOf subject)) := by rfl

/-- Full profile, including the general deed machinery's explicit rider context. -/
theorem exactProfileForEveryOuterContext (bs : Bindings) :
    Instruction.profile bs instruction =
      ⟨controllerBinding :: targetBinding :: bs,
       controllerBinding :: sacrificedBinding :: bs,
       some (controllerBinding :: riderBinding :: bs), []⟩ := by rfl

theorem targetMultiplicity :
    ((Instruction.profile [] instruction).pre.countP (fun b => b.det == .target)) = 1 := by decide

theorem duplicatingThePatientWouldIntroduceTwoTargets :
    ((Instruction.profile [] (sacrifice subject (agent := controllerOf subject))).pre.countP
      (fun b => b.det == .target)) = 2 := by decide

theorem instructionIsAdmitted : Instruction.check [] instruction = [] := by decide

theorem laterPlayerIsTheController :
    firstReach (.word .player) .one afterSacrifice = some controllerBinding := by rfl

theorem laterCardIsTheSacrificedObject :
    firstReach (.word .card) .one afterSacrifice = some sacrificedBinding := by rfl

theorem sacrificeStampSelectsTheSameObject :
    firstReach (.stamped (.action "Sacrifice")) .one afterSacrifice =
      some sacrificedBinding := by rfl

theorem laterPlayerReadIsAdmitted :
    NounPhrase.check (some .player) afterSacrifice (that .player) = [] := by decide

theorem laterCardReadIsAdmitted :
    NounPhrase.check (some .object) afterSacrifice (that .card) = [] := by decide

theorem formerPermanentReadIsAdmitted :
    NounPhrase.check (some .object) afterSacrifice
      (theVerbed (.action "Sacrifice") .permanent .thisWay .one) = [] := by decide

theorem laterCardRetainsBothTypes :
    NounPhrase.ty afterSacrifice (that .card) = [.creature, .artifact] := by decide

theorem laterCardRetainsGraveyardCarrier :
    NounPhrase.zone afterSacrifice (that .card) = some .graveyard := by decide

theorem riderReadsTheStampedPatient :
    firstReach (.stamped (.action "Sacrifice")) .one
      (Instruction.profile [] instruction).riderCtx = some riderBinding := by rfl

theorem reflexiveFollowUpRemainsAvailable :
    instruction.reflexEncloseUse = .reflexive := by rfl

theorem existingReferenceIsNotIntroducedAgain :
    Instruction.profile [targetBinding] (controllerSacrifices it) =
      ⟨[controllerBinding, targetBinding], [controllerBinding, sacrificedBinding],
       some [controllerBinding, riderBinding], []⟩ := by rfl

theorem existingReferenceIsAdmitted :
    Instruction.check [targetBinding] (controllerSacrifices it) = [] := by decide

theorem outerArtifactDoesNotStealThePatient :
    Instruction.check [⟨.target, .one, .object [.artifact] (some .battlefield) none none (some 1)⟩]
      instruction = [] := by decide

theorem thatPlayerCanTakeTheFollowingAction :
    Instruction.check [] (.sequentially [instruction, draw (.lit 1) (agent := that .player)]) = [] := by decide

private def nestedSubject : NounPhrase := target (.and [creature,
  .hasPossessor .controller (controllerOf (target artifact))])
private def nestedInstruction : Instruction := controllerSacrifices nestedSubject
private def nestedArtifact : Binding :=
  ⟨.target, .one, .object [.artifact] (some .battlefield) none none (some 1)⟩
private def nestedCreature : Binding :=
  ⟨.target, .one, .object [.creature] (some .battlefield) none none (some 1)⟩
private def nestedSacrificed : Binding :=
  ⟨.target, .one, .object [.creature] (some .graveyard)
    (some ⟨.action "Sacrifice", true, true⟩) none (some 1)⟩

theorem nestedDescriptionIsAdmitted :
    Instruction.check [] nestedInstruction = [] := by decide

theorem wholeOwnedScopeWouldBeAmbiguous :
    Instruction.check []
      (sacrifice (ownSubject (controllerOf nestedSubject)) (agent := controllerOf nestedSubject)) =
      [.anaphor .bare .one 2] := by decide

theorem nestedDescriptionMovesOnlyItsOwnReferent (bs : Bindings) :
    Instruction.intro bs nestedInstruction =
      controllerBinding :: nestedSacrificed :: controllerBinding :: nestedArtifact :: bs := by rfl

theorem nestedDescriptionDeclaresExactlyItsTwoTargets :
    (Instruction.profile [] nestedInstruction).pre =
      [controllerBinding, nestedCreature, controllerBinding, nestedArtifact] := by rfl

theorem coordinatedReferencesHaveNoSingleIntroducedReferent :
    (Primitives.NounPhrase.eitherOf (target creature) (target artifact)).introducesOwnReferent = false ∧
    (Primitives.NounPhrase.both (target creature) (target artifact)).introducesOwnReferent = false := by decide

theorem typedDescriptionRetainsItsOwnReferent :
    (NounPhrase.resolvedPermanent (.asType .creature nestedSubject none)).introducesOwnReferent =
      true := by decide

theorem wrappedSelfDoesNotInventAnIntroduction :
    (NounPhrase.resolvedPermanent (.asType .creature .this none)).introducesOwnReferent =
      false := by decide

private def authoringWitness : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Controller sacrifice witness", types := [.sorcery], cost := some [generic 1],
      text := [Primitives.Ability.spell none
        (controllerSacrifices (target (Primitives.Predicate.and [artifact, creature])))] } }

theorem compositionPassesTheAuthoringBoundary :
    authoringWitness.authoring.onlyMacros = true := authoringWitness.onlyMacros

end Semantics.Proofs.ControllerSacrifice

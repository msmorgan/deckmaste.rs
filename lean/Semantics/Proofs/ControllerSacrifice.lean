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

theorem controlGrantRecordsItsPatientIdentity :
    let (after, errors) := stepInstruction 64 .postEvent .obj (DRS.ofList [])
      (gainControl (target creature) (some untilEndOfTurn))
    let refs := resolveCandidates (.verbed (.core .gainControl) none .attributive)
      .one (nextClause after)
    errors = [] ∧ refs.length = 1 ∧ refs.all (fun r => r.det == .target) ∧
      refs.map (·.id) = (after.referents.filter (fun r => r.det == .target)).map (·.id) := by decide

private def existingTarget : Referent :=
  { det := .target, plur := .one,
    payload := .object [.creature, .artifact] (some .battlefield) none none (some 1) none }
private def existingTargetContext : DRS := DRS.ofList [existingTarget]

theorem existingReferenceIsAdmitted :
    Instruction.check existingTargetContext (controllerSacrifices it) = [] := by decide

theorem existingReferenceIsNotIntroducedAgain :
    let after := (stepInstruction 64 .postEvent .obj existingTargetContext
      (controllerSacrifices it)).1
    after.visibleReferents.countP (fun ref => ref.kind == .object) = 1 ∧
      after.visibleReferents.countP (fun ref => ref.det == .target) = 1 ∧
      (after.findById? 0).isNone = true ∧
      (firstReach (.word .card) .one (nextClause after)).map (·.id) = some 3 ∧
      (after.findById? 3).bind (·.zone) = some .graveyard := by decide

theorem reflexiveFollowUpRemainsAvailable :
    instruction.reflexEncloseUse = .reflexive := by decide

theorem riderReadsTheStampedPatient :
    let patient := theVerbed (.action "sacrifice") .one (some .card) .thisWay
    let (after, ids, errors) := stepNounPhrase 64 (nextClause afterSacrifice) .obj true patient
    Instruction.check (DRS.ofList [])
      (.withContinuation .required instruction (some (.expose .reveal (.cards patient))) none) = [] ∧
      errors = [] ∧ ids = [3] ∧
      (ids.filterMap after.findById?).map (fun ref => (ref.zone, ref.ty)) =
        [(some .graveyard, [.creature, .artifact])] := by decide

theorem controllerSacrificesExpansionForEverySubject (chosen : NounPhrase) :
    controllerSacrifices chosen =
      .instruct (.possessorOf .controller
        (.firstMention "controllerSacrifices.subject" chosen))
        (sacrifice (.laterMention "controllerSacrifices.subject" chosen)) := by rfl

theorem repeatedControllerSacrificesKeepDistinctPatients :
    let (after, errors) := stepInstruction 64 .postEvent .obj (DRS.ofList [])
      (.sequentially [instruction, instruction])
    errors = [] ∧
      (after.referents.filter (fun ref => ref.kind == .object)).map
        (fun ref => (ref.id, ref.zone, ref.idents)) =
        [(7, some .graveyard, []), (3, some .graveyard, [])] := by decide

theorem duplicatingThePatientWouldIntroduceTwoTargets :
    (stepInstruction 64 .postEvent .obj (DRS.ofList [])
      (instruct (controllerOf subject) (sacrifice subject))).1.referents.countP
        (·.det == .target) = 2 := by decide

theorem laterPlayerIsTheController :
    (firstReach (.word .player) .one afterSacrifice).map
      (fun ref => (ref.id, ref.det, ref.plur, ref.payload)) =
      some (1, .the, .one, .player false) := by decide

theorem laterCardIsTheSacrificedObject :
    (firstReach (.word .card) .one afterSacrifice).map
      (fun ref => (ref.id, ref.det, ref.plur, ref.payload)) =
      some (3, .target, .one,
        .object [.creature, .artifact] (some .graveyard)
          (some ⟨.action "sacrifice", true, true⟩) none none none) := by decide

theorem outerArtifactDoesNotStealThePatient :
    let outer : DRS := DRS.ofList
      [{ det := .target, plur := .one,
         payload := .object [.artifact] (some .battlefield) none none (some 1) none }]
    let (after, errors) := stepInstruction 64 .postEvent .obj outer instruction
    errors = [] ∧ after.findById? 0 = outer.findById? 0 ∧
      (firstReach (.word .card) .one (nextClause after)).map (·.id) = some 4 := by decide

private def universallySelectedPatient (id : ReferentId) : Referent :=
  { id, det := .target, plur := .one,
    payload := .object [.creature] (some .battlefield) none none none none }

/-- Moving the selected patient preserves every unrelated outer referent. Actor frames are
excluded because their links to the moved patient must follow its fresh identity. -/
theorem sacrificeMovePreservesEveryUnrelatedOuterReferent
    (rs : List Referent) (id fresh : ReferentId)
    (distinct : ∀ r ∈ rs, r.id ≠ id)
    (notActor : ∀ r ∈ rs, ∀ shape, r.payload ≠ .actor shape) :
    let before : DRS :=
      { referents := universallySelectedPatient id :: rs, nextId := fresh }
    (applyZoneMove before [id] (some (.action "sacrifice"))
      (some .graveyard)).referents.drop 1 = rs := by
  dsimp only
  simp only [applyZoneMove, List.eraseDups_cons, List.foldl_cons]
  simp only [DRS.findById?, List.find?_cons, universallySelectedPatient, BEq.rfl]
  simp
  induction rs with
  | nil => rfl
  | cons r rs ih =>
    have h := distinct r (by simp)
    have ha := notActor r (by simp)
    simp only [List.map_cons]
    cases hp : r.payload <;> simp_all

end Semantics.Proofs.ControllerSacrifice

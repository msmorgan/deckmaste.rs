import Semantics.Check
import Semantics.Check.Card
import Semantics.Definitions

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.ControllerSacrifice

private def subject : NounPhrase := target (.and [isArtifact, isCreature])
private def action : Action := controllerSacrifices subject
private def afterSacrifice : DRS :=
  (stepAction 64 .postEvent .obj (DRS.ofList []) action).1
private def nestedSubject : NounPhrase := target (.and [isCreature,
  .hasPossessor .controller (controllerOf (target isArtifact))])
private def nestedAction : Action := controllerSacrifices nestedSubject

theorem targetMultiplicity :
    afterSacrifice.referents.countP (·.det == .target) = 1 := by decide

theorem actionIsAdmitted :
    Action.check (DRS.ofList []) action = [] := by decide

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
    Action.check (DRS.ofList []) nestedAction = [] := by decide

theorem nestedDescriptionPreservesEveryOuterContextBeforeSacrifice
    (fuel : Nat) (drs : DRS) :
    (stepNounPhrase (fuel + 10) drs .subj true
      (controllerOf nestedSubject)).1.referents.drop 4 = drs.referents := by rfl

theorem nestedSubjectPublishesExactlyItsFourParticipants :
    (stepNounPhrase 64 (DRS.ofList []) .subj true (controllerOf nestedSubject)).1.referents.map
      (fun ref => (ref.kind, ref.zone, ref.ty)) =
      [(.player, none, []), (.object, some .battlefield, [.creature]),
        (.player, none, []), (.object, some .battlefield, [.artifact])] := by decide

theorem nestedDescriptionDeclaresExactlyItsTwoTargets :
    (stepAction 64 .postEvent .obj (DRS.ofList []) nestedAction).1.referents.countP
      (·.det == .target) = 2 := by decide

-- P08: replacing the explicit parameter read with an ordinary pronoun leaves a true tie.
theorem wholeOwnedScopeWouldBeAmbiguous :
    Action.check (DRS.ofList [])
      (instruct (controllerOf nestedSubject) (sacrifice it)) =
        [.anaphor .bare .one 2] := by decide

theorem sacrificeStampSelectsTheSameObject :
    (participialCandidates (.deed (.action "sacrifice")) (some .one) none false afterSacrifice).head?.map
        (·.id) =
      (firstReach (.word .card) .one afterSacrifice).map (·.id) := by decide

theorem formerPermanentReadIsAdmitted :
    NounPhrase.checkAt (some .object) afterSacrifice
      (the (and [isPermanent, was (sacrifice which) thisWay])) = [] := by decide

theorem thatPlayerCanTakeTheFollowingAction :
    Action.check (DRS.ofList [])
      (.sequentially [action, instruct (that .player) (draw 1)]) = [] := by decide

private def nestedAfter : DRS :=
  (stepAction 64 .postEvent .obj (DRS.ofList []) nestedAction).1

theorem nestedDescriptionMovesOnlyItsOwnReferentWithoutOuterContext :
    (firstReach (.word (.type .artifact)) .one nestedAfter).map (·.zone) = some (some .battlefield) ∧
    (firstReach (.word .card) .one nestedAfter).map (·.zone) = some (some .graveyard) := by decide

private def authoringWitness : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Controller sacrifice witness", types := [.sorcery], cost := some [generic 1],
      text := [spell
        (controllerSacrifices (target (and [isArtifact, isCreature])))] } }

theorem compositionPassesTheAuthoringBoundary :
    authoringWitness.authoring.onlyAuthored = true := by decide

theorem controlGrantRecordsItsPatientIdentity :
    let (after, errors) := stepAction 64 .postEvent .obj (DRS.ofList [])
      (gainControl (target isCreature) (some untilEndOfTurn))
    let refs := participialCandidates (.deed (.core .gainControl)) (some .one) none false (nextClause after)
    errors = [] ∧ refs.length = 1 ∧ refs.all (fun r => r.det == .target) ∧
      refs.map (·.id) = (after.referents.filter (fun r => r.det == .target)).map (·.id) := by decide

private def existingTarget : Referent :=
  { det := .target, plur := .one,
    payload := .object [.creature, .artifact] (some .battlefield) none none (some 1) }
private def existingTargetContext : DRS := DRS.ofList [existingTarget]

theorem existingReferenceIsAdmitted :
    Action.check existingTargetContext (controllerSacrifices it) = [] := by decide

theorem existingReferenceIsNotIntroducedAgain :
    let after := (stepAction 64 .postEvent .obj existingTargetContext
      (controllerSacrifices it)).1
    after.visibleReferents.countP (fun ref => ref.kind == .object) = 1 ∧
      after.visibleReferents.countP (fun ref => ref.det == .target) = 1 ∧
      (after.findById? 0).isNone = true ∧
      (firstReach (.word .card) .one (nextClause after)).map (fun ref => (ref.movedFrom, ref.zone)) =
        some ([0], some .graveyard) := by decide

theorem reflexiveFollowUpRemainsAvailable :
    action.reflexEncloseUse = .reflexive := by decide

theorem riderReadsTheStampedPatient :
    let patient := the (and [isCard, was (sacrifice which) thisWay])
    let (after, ids, errors) := stepNounPhrase 64 (nextClause afterSacrifice) .obj true patient
    Action.check (DRS.ofList [])
      (.withContinuation .required action (some (.expose .reveal (.cards patient))) none) = [] ∧
      errors = [] ∧ ids = (afterSacrifice.referents.filter (·.det == .target)).map (·.id) ∧
      (ids.filterMap after.findById?).map (fun ref => (ref.zone, ref.ty)) =
        [(some .graveyard, [.creature, .artifact])] := by decide

theorem repeatedControllerSacrificesKeepDistinctPatients :
    let (after, errors) := stepAction 64 .postEvent .obj (DRS.ofList [])
      (.sequentially [action, action])
    let patients := after.referents.filter (fun ref => ref.kind == .object)
    errors = [] ∧
      patients.map (fun ref => (ref.zone, ref.idents)) =
        [(some .graveyard, []), (some .graveyard, [])] ∧
      (patients.map (·.id)).Nodup := by decide

theorem duplicatingThePatientWouldIntroduceTwoTargets :
    (stepAction 64 .postEvent .obj (DRS.ofList [])
      (instruct (controllerOf subject) (sacrifice subject))).1.referents.countP
        (·.det == .target) = 2 := by decide

theorem laterPlayerIsTheController :
    (firstReach (.word .player) .one afterSacrifice).map
      (fun ref => (ref.det, ref.plur, ref.payload, ref.role)) =
      some (.the, .one, .player false, .subj) := by decide

theorem laterCardIsTheSacrificedObject :
    (firstReach (.word .card) .one afterSacrifice).map
      (fun ref => (ref.det, ref.plur, ref.payload)) =
      some (.target, .one,
        .object [.creature, .artifact] (some .graveyard)
          (some ⟨[.action "sacrifice"], true, some ⟨some .battlefield, some .graveyard⟩, false⟩)
          none none) := by decide

theorem outerArtifactDoesNotStealThePatient :
    let outer : DRS := DRS.ofList
      [{ det := .target, plur := .one,
         payload := .object [.artifact] (some .battlefield) none none (some 1) }]
    let (after, errors) := stepAction 64 .postEvent .obj outer action
    errors = [] ∧ after.findById? 0 = outer.findById? 0 ∧
      (firstReach (.word .card) .one (nextClause after)).map (·.id) =
        ((headerFreshReferents outer after).filter (·.kind == .object)).head?.map (·.id) := by decide

private def universallySelectedPatient (id : ReferentId) : Referent :=
  { id, det := .target, plur := .one,
    payload := .object [.creature] (some .battlefield) none none none }

/-- Moving the selected patient preserves every unrelated outer referent. (A handoff's performer
is no referent, `DRS.performers`, so none of them links to the moved patient.) -/
theorem sacrificeMovePreservesEveryUnrelatedOuterReferent
    (rs : List Referent) (id fresh : ReferentId)
    (distinct : ∀ r ∈ rs, r.id ≠ id) :
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
    simp only [List.map_cons]
    simp_all

open Semantics
namespace ControllerSuffixLocality

private theorem updateById_append (drs : DRS) (suffix : List Referent) (id : ReferentId)
    (f : Referent → Referent) (distinct : ∀ ref ∈ suffix, ref.id ≠ id) :
    ({ drs with referents := drs.referents ++ suffix }).updateById id f =
      { drs.updateById id f with referents := (drs.updateById id f).referents ++ suffix } := by
  have fixed : suffix.map (fun r => if r.id == id then
      let updated := f r
      { updated with id := r.id, idents := mergeIdents r.idents updated.idents }
    else r) = suffix := by
    induction suffix with
    | nil => rfl
    | cons ref suffix ih =>
      have h := distinct ref (by simp)
      have hb : (ref.id == id) = false := by simp [h]
      simp only [List.map_cons, hb, Bool.false_eq_true, ↓reduceIte]
      congr 1
      exact ih (by
        intro ref member
        exact distinct ref (by simp [member]))
  simp only [DRS.updateById, List.map_append, fixed]

private theorem updateFold_append (drs : DRS) (suffix : List Referent) (ids : List ReferentId)
    (change : ReferentId → Referent → Referent)
    (distinct : ∀ ref ∈ suffix, ∀ id ∈ ids, ref.id ≠ id) :
    ids.foldl (fun state id => state.updateById id (change id))
      { drs with referents := drs.referents ++ suffix } =
      { ids.foldl (fun state id => state.updateById id (change id)) drs with
        referents := (ids.foldl (fun state id => state.updateById id (change id)) drs).referents ++ suffix } := by
  induction ids generalizing drs with
  | nil => rfl
  | cons id ids ih =>
    simp only [List.foldl_cons]
    rw [updateById_append drs suffix id (change id) (by
      intro ref member
      exact distinct ref member id (by simp))]
    exact ih _ (by
      intro ref member id selected
      exact distinct ref member id (by simp [selected]))

private theorem applyInPlaceStamp_append (drs : DRS) (suffix : List Referent)
    (ids : List ReferentId) (deed : Option Deed)
    (distinct : ∀ ref ∈ suffix, ∀ id ∈ ids, ref.id ≠ id) :
    applyInPlaceStamp { drs with referents := drs.referents ++ suffix } ids deed =
      { applyInPlaceStamp drs ids deed with
        referents := (applyInPlaceStamp drs ids deed).referents ++ suffix } := by
  cases deed with
  | none => rfl
  | some deed =>
    cases ids with
    | nil => rfl
    | cons id ids =>
      unfold applyInPlaceStamp
      exact updateFold_append drs suffix (id :: ids) _ distinct

private theorem laterMention_step (fuel : Nat) (drs : DRS)
    (role : GramRole) (arg : Bool) (name : BindingId) (fallback : NounPhrase) :
    stepNounPhrase (fuel + 1) drs role arg (.laterMention name fallback) =
      let matching := drs.visibleReferents.filter (fun ref => ref.idents.elem name)
      match matching with
      | [] =>
        match drs.delayedTriggers.find? (·.idents.elem name) with
        | some delayed => (drs, [delayed.id], [])
        | none =>
          (drs, [], if drs.unboundIdentities.contains name then [] else [.bindingIdentity name])
      | refs =>
        let ids := refs.map (·.id)
        let out := if arg then ids.foldl (fun state id => state.recordMention id role true) drs
          else drs
        (out, ids, []) := by rfl

end ControllerSuffixLocality

open Semantics Semantics.Templates Semantics.Definitions
private def selectedContext (drs : DRS) : DRS :=
  let seen := { drs with characteristicRead := true, relativeKind := some .object }
  let artifactContext := seen.push
    { det := .target, plur := .one,
      payload := .object [.artifact] (some .battlefield) none none none } .other false
  let controllerContext := artifactContext.push
    { det := .the, plur := .one, payload := .player false } .other false
  let creatureContext := controllerContext.push
    { det := .target, plur := .one,
      payload := .object [.creature] (some .battlefield) none none none } .other false
  let named := recordIdentityById "controllerSacrifices.subject" creatureContext
    (drs.nextId + 2)
  { named with
      characteristicRead := drs.characteristicRead
      unboundIdentities := named.unboundIdentities.filter (· != "controllerSacrifices.subject") }.push
    { det := .the, plur := .one, payload := .player false } .subj true

private def sacrificeStampState (fuel : Nat) (drs : DRS) (body : Action) : DRS :=
 let (after, _) := stepAction fuel .postEvent .obj drs body
 let mentions := if after.currentClause == drs.currentClause then
    after.pendingRoles.filter (!drs.pendingRoles.contains ·)
    else after.pendingRoles
 let actedIds := mentions.filterMap fun (id, role) => if role == .obj then some id else none
 applyInPlaceStamp after actedIds (some (.action "sacrifice"))
private theorem sacrificeScopeStep (fuel : Nat) (drs : DRS) (body : Action) :
 (stepAction (fuel + 1) .postEvent .obj drs (.ownScope (some (.action "sacrifice")) body)).1 =
 (leaveOwnScope (sacrificeStampState fuel (enterOwnScope drs (some (.action "sacrifice"))) body)).withCaused drs .postEvent
   (.ownScope (some (.action "sacrifice")) body) := by rfl

private theorem sacrificeStampProjected (fuel : Nat) (drs : DRS) (body : Action) :
 sacrificeStampState fuel drs body =
 let after := (stepAction fuel .postEvent .obj drs body).1
 let mentions := if after.currentClause == drs.currentClause then
   after.pendingRoles.filter (!drs.pendingRoles.contains ·)
    else after.pendingRoles
 let actedIds := mentions.filterMap fun (id, role) => if role == .obj then some id else none
 applyInPlaceStamp after actedIds (some (.action "sacrifice")) := by
 unfold sacrificeStampState
 cases stepAction fuel .postEvent .obj drs body
 rfl
private def prepared (fresh : Nat) (rs : List Referent) : DRS :=
 let tracked : DRS := { referents := rs, nextId := fresh, outerReferentIds := [rs.map (·.id)]}
 let source := selectedContext tracked
 let frame : ActorShape := {
   kind := .player, isYou := false, ascribable := false,
   refIds := [fresh + 3] }
 source.pushPerformer frame
private def moveArg : NounPhrase :=
 .firstMention "sacrifice.subject"
   (.inCaller (.laterMention "controllerSacrifices.subject" nestedSubject))
private def moveBody : Action :=
 .move moveArg (.zone .battlefield none) (some graveyard) .departure []
private theorem moveBodyStep (fuel : Nat) (drs : DRS) (ordinary : drs.actionPattern = false) :
 (stepAction (fuel + 1) .postEvent .obj drs moveBody).1 =
 let selected := stepNounPhrase fuel drs .obj true moveArg
 (applyZoneMove { selected.1 with readsLastKnown := drs.readsLastKnown, handedOrigin := drs.handedOrigin }
   selected.2.1 drs.scopeDeed (some .graveyard)).withCaused drs .postEvent moveBody := by
 cases drs with
 | mk => simp only at ordinary; subst ordinary; rfl

private def argumentPrefix (fresh : Nat) (names : List BindingId) : List Referent :=
 [ { id := fresh + 3, det := .the, plur := .one, payload := .player false, role := .subj },
   { id := fresh + 2, det := .target, plur := .one, idents := names,
     payload := .object [.creature] (some .battlefield) none none none },
   { id := fresh + 1, det := .the, plur := .one, payload := .player false },
   { id := fresh, det := .target, plur := .one,
     payload := .object [.artifact] (some .battlefield) none none none } ]
private def argumentBase (fresh : Nat) (names : List BindingId) (data : DRS) : DRS :=
 { data with referents := argumentPrefix fresh names, nextId := fresh + 4, historicalReferentIds := [] }
private def afterPrefix (fresh : Nat) : List Referent :=
 [ { id := fresh + 3, det := .the, plur := .one, payload := .player false, role := .subj },
   { id := fresh + 4, det := .target, plur := .one,
     payload := .object [.creature] (some .graveyard)
       (some ⟨[.action "sacrifice"], true, some ⟨some .battlefield, some .graveyard⟩, false⟩)
       none none, movedFrom := [fresh + 2] },
   { id := fresh + 1, det := .the, plur := .one, payload := .player false },
   { id := fresh, det := .target, plur := .one,
     payload := .object [.artifact] (some .battlefield) none none none } ]
private theorem isolatedMoveStamp (fresh : Nat) (names : List BindingId) (data : DRS) :
 (dropActorFrame (applyInPlaceStamp
   (applyZoneMove (argumentBase fresh names data) [fresh + 2] (some (.action "sacrifice")) (some .graveyard))
   [fresh + 4] (some (.action "sacrifice")))).referents = afterPrefix fresh := by
 simp only [applyZoneMove, show [fresh + 2].eraseDups = [fresh + 2] from rfl,
   List.foldl_cons, List.foldl_nil]
 simp [argumentBase, argumentPrefix, afterPrefix, DRS.findById?,
   applyInPlaceStamp, DRS.updateById, dropActorFrame,
   moveStamp, onFieldZone, zoneIsB, mergeIdents, List.foldl_cons, List.foldl_nil]

private theorem argumentStep (fuel : Nat) (drs : DRS) :
    stepNounPhrase (fuel + 3) drs .obj true moveArg =
      let caller := enterCaller drs
      let selected := stepNounPhrase (fuel + 1) caller .obj true
        (.laterMention "controllerSacrifices.subject" nestedSubject)
      let left := leaveCaller drs selected.1 selected.2.1
      let named := selected.2.1.foldl
        (fun state id => recordIdentityById "sacrifice.subject" state id) left
      let unbound := named.unboundIdentities.filter (· != "sacrifice.subject")
      ({ named with
          unboundIdentities :=
            if selected.2.1.isEmpty then "sacrifice.subject" :: unbound else unbound },
        selected.2.1, selected.2.2) := by rfl

private theorem preparedSuffix (fresh : Nat) (rs : List Referent)
    (older : ∀ r ∈ rs, r.id < fresh) :
    (prepared fresh rs).referents = (prepared fresh []).referents ++ rs := by
  simp [prepared, selectedContext, DRS.pushPerformer, DRS.push, DRS.prepend, recordIdentityById, DRS.updateById]
  induction rs with
  | nil => rfl
  | cons ref rs ih =>
    have h := older ref (by simp)
    have distinct : ref.id ≠ fresh + 2 :=
      Nat.ne_of_lt (Nat.lt_of_lt_of_le h (Nat.le_add_right fresh 2))
    simp only [List.map_cons, distinct, ↓reduceIte]
    congr 1
    exact ih (by intro r member; exact older r (by simp [member]))

private def namedPatient (fresh : Nat) : Referent := {
  det := .target, plur := .one, id := fresh + 2,
  idents := ["controllerSacrifices.subject"],
  payload := .object [.creature] (some .battlefield) none none none }

private theorem argumentNamedPatient (fresh : Nat) (rs : List Referent)
    (older : ∀ r ∈ rs, r.id < fresh)
    (noNames : ∀ r ∈ rs, r.idents.elem "controllerSacrifices.subject" = false) :
    (enterCaller (enterOwnScope (prepared fresh rs) (some (.action "sacrifice")))).visibleReferents.filter
      (fun ref => ref.idents.elem "controllerSacrifices.subject") =
      [namedPatient fresh] := by
  have noSuffix : rs.filter (fun ref => ref.idents.elem "controllerSacrifices.subject") = [] := by
    apply List.filter_eq_nil_iff.mpr
    intro ref member
    simp only [noNames ref member, Bool.false_eq_true, not_false_eq_true]
  have refs : (enterCaller (enterOwnScope (prepared fresh rs) (some (.action "sacrifice")))).referents =
      (prepared fresh rs).referents := by rfl
  have visible : (enterCaller (enterOwnScope (prepared fresh rs) (some (.action "sacrifice")))).visibleReferents =
      (enterCaller (enterOwnScope (prepared fresh rs) (some (.action "sacrifice")))).referents := by
    apply List.filter_eq_self.mpr
    intro ref _
    rfl
  rw [visible, refs, preparedSuffix fresh rs older, List.filter_append, noSuffix,
    List.append_nil]
  simp [prepared, selectedContext, DRS.pushPerformer, DRS.push, DRS.prepend, recordIdentityById,
    DRS.updateById, mergeIdents, namedPatient]

private theorem argumentResult (fuel fresh : Nat) (rs : List Referent)
    (older : ∀ r ∈ rs, r.id < fresh)
    (noNames : ∀ r ∈ rs, r.idents.elem "controllerSacrifices.subject" = false) :
    stepNounPhrase (fuel + 3) (enterOwnScope (prepared fresh rs) (some (.action "sacrifice"))) .obj true moveArg =
      let inner := enterOwnScope (prepared fresh rs) (some (.action "sacrifice"))
      let caller := enterCaller inner
      let mentioned := caller.recordMention (fresh + 2) .obj true
      (recordIdentityById "sacrifice.subject" (leaveCaller inner mentioned [fresh + 2]) (fresh + 2),
        [fresh + 2], []) := by
  rw [argumentStep]
  dsimp only
  rw [ControllerSuffixLocality.laterMention_step,
    argumentNamedPatient fresh rs older noNames]
  rfl

private theorem argumentReferents (fuel fresh : Nat) (rs : List Referent)
    (older : ∀ r ∈ rs, r.id < fresh)
    (noNames : ∀ r ∈ rs, r.idents.elem "controllerSacrifices.subject" = false) :
    (stepNounPhrase (fuel + 3) (enterOwnScope (prepared fresh rs) (some (.action "sacrifice"))) .obj true moveArg).1.referents =
      (recordIdentityById "sacrifice.subject" (prepared fresh []) (fresh + 2)).referents ++ rs := by
  rw [argumentResult fuel fresh rs older noNames]
  change (recordIdentityById "sacrifice.subject" (prepared fresh rs) (fresh + 2)).referents = _
  simp only [recordIdentityById, DRS.updateById]
  rw [preparedSuffix fresh rs older, List.map_append]
  congr 1
  induction rs with
  | nil => rfl
  | cons ref rs ih =>
    have h := older ref (by simp)
    have distinct : ref.id ≠ fresh + 2 :=
      Nat.ne_of_lt (Nat.lt_of_lt_of_le h (Nat.le_add_right fresh 2))
    simp only [List.map_cons, show (ref.id == fresh + 2) = false by simp [distinct],
      Bool.false_eq_true, ↓reduceIte]
    congr 1
    exact ih (by intro r member; exact older r (by simp [member]))
      (by intro r member; exact noNames r (by simp [member]))

private theorem argumentCallerOwnership (fuel fresh : Nat) (rs : List Referent)
    (older : ∀ r ∈ rs, r.id < fresh)
    (noNames : ∀ r ∈ rs, r.idents.elem "controllerSacrifices.subject" = false) :
    let out := (stepNounPhrase (fuel + 3) (enterOwnScope (prepared fresh rs) (some (.action "sacrifice"))) .obj true moveArg).1
    out.scopeStack.head?.any (fun frame => frame.callerReferentIds.contains (fresh + 2)) = true := by
  rw [argumentResult fuel fresh rs older noNames]
  change ((([] ++ ([fresh + 2] ++ [fresh + 2] ++ _).eraseDups).eraseDups).contains (fresh + 2)) = true
  simp

namespace MoveActors
private theorem movingMapSuffix (suffix : List Referent) (id fresh : ReferentId)
    (payload : Payload)
    (distinct : ∀ ref ∈ suffix, ref.id ≠ id) :
    suffix.map (fun r =>
      if r.id == id then
        { r with
          id := fresh
          idents := []
          payload := payload
          movedFrom := id :: r.movedFrom }
      else r) = suffix := by
  induction suffix with
  | nil => rfl
  | cons ref suffix ih =>
    have different : (ref.id == id) = false := by simp [distinct ref (by simp)]
    simp only [List.map_cons, different, Bool.false_eq_true, ↓reduceIte]
    exact congrArg (List.cons ref) (ih (by intro r h; exact distinct r (by simp [h])))

private theorem applyZoneMove_append (drs : DRS) (suffix : List Referent)
    (id : ReferentId) (patient : Referent) (deed : Option Deed) (destination : Option Zone)
    (found : drs.findById? id = some patient)
    (distinct : ∀ ref ∈ suffix, ref.id ≠ id) :
    applyZoneMove { drs with referents := drs.referents ++ suffix } [id] deed destination =
      { applyZoneMove drs [id] deed destination with
        referents := (applyZoneMove drs [id] deed destination).referents ++ suffix } := by
  have lookup : ({ drs with referents := drs.referents ++ suffix } : DRS).findById? id =
      some patient := by
    change (drs.referents ++ suffix).find? (·.id == id) = some patient
    rw [List.find?_append, show drs.referents.find? (·.id == id) = some patient from found]
    rfl
  simp only [applyZoneMove, show [id].eraseDups = [id] from rfl,
    List.foldl_cons, List.foldl_nil, lookup, found]
  split
  · rfl
  · cases patient.payload <;> dsimp only
    all_goals try rfl
    all_goals
      simp only [List.map_append]
      congr 1
      exact congrArg (List.append _)
        (movingMapSuffix suffix id drs.nextId _ distinct)

end MoveActors

private theorem movedStampedPrefixAndSuffix (fresh : Nat) (names : List BindingId)
    (data : DRS) (rs : List Referent)
    (older : ∀ ref ∈ rs, ref.id < fresh) :
    let base := argumentBase fresh names data
    (dropActorFrame (applyInPlaceStamp
      (applyZoneMove { base with referents := base.referents++rs }
        [fresh + 2] (some (.action "sacrifice")) (some .graveyard))
      [fresh + 4] (some (.action "sacrifice")))).referents = afterPrefix fresh ++rs := by
  dsimp only
  have found : (argumentBase fresh names data).findById? (fresh + 2) =
      some { id := fresh + 2, det := .target, plur := .one, idents := names,
             payload := .object [.creature] (some .battlefield) none none none } := by
    simp [argumentBase, argumentPrefix, DRS.findById?]
  rw [MoveActors.applyZoneMove_append _ rs _ _ _ _ found (by
    intro r hr; have h := older r hr; exact Nat.ne_of_lt (Nat.lt_add_right 2 h))]
  rw [ControllerSuffixLocality.applyInPlaceStamp_append _ rs _ _ (by
    intro r hr id hid
    simp only [List.mem_cons, List.not_mem_nil, or_false] at hid
    subst id
    have h := older r hr
    exact Nat.ne_of_lt (Nat.lt_add_right 4 h))]
  change (applyInPlaceStamp (applyZoneMove (argumentBase fresh names data) [fresh + 2] (some (.action "sacrifice"))
    (some .graveyard)) [fresh + 4] (some (.action "sacrifice"))).referents ++ rs = _
  rw [← isolatedMoveStamp fresh names data]
  rfl

private def argumentOut (fresh : Nat) (rs : List Referent) : DRS :=
 let inner := enterOwnScope (prepared fresh rs) (some (.action "sacrifice"))
 recordIdentityById "sacrifice.subject"
   (leaveCaller inner ((enterCaller inner).recordMention (fresh + 2) .obj true) [fresh + 2])
   (fresh + 2)

private theorem argumentOutPrefix (fresh : Nat) (rs : List Referent)
    (older : ∀ r ∈ rs, r.id < fresh)
    (noNames : ∀ r ∈ rs, r.idents.elem "controllerSacrifices.subject" = false) :
    (argumentOut fresh rs).referents =
      argumentPrefix fresh ["controllerSacrifices.subject", "sacrifice.subject"] ++rs := by
  have h := argumentReferents 0 fresh rs older noNames
  rw [argumentResult 0 fresh rs older noNames] at h
  change (argumentOut fresh rs).referents = _ at h
  rw [h]
  congr 1
  simp [argumentPrefix, prepared, selectedContext, DRS.pushPerformer, DRS.push, DRS.prepend,
    recordIdentityById, DRS.updateById, mergeIdents]

private theorem argumentOutBase (fresh : Nat) (rs : List Referent)
    (older : ∀ r ∈ rs, r.id < fresh)
    (noNames : ∀ r ∈ rs, r.idents.elem "controllerSacrifices.subject" = false) :
    argumentOut fresh rs =
      { (argumentBase fresh ["controllerSacrifices.subject", "sacrifice.subject"]
          (argumentOut fresh rs)) with
        referents := argumentPrefix fresh
          ["controllerSacrifices.subject", "sacrifice.subject"] ++ rs } := by
  have refs := argumentOutPrefix fresh rs older noNames
  have next : (argumentOut fresh rs).nextId = fresh + 4 := rfl
  have hist : (argumentOut fresh rs).historicalReferentIds = [] := rfl
  cases h : argumentOut fresh rs
  simp only [h] at refs next hist
  simp_all [argumentBase]

private def movedOut (fresh : Nat) (rs : List Referent) : DRS :=
 applyZoneMove (argumentOut fresh rs) [fresh + 2] (some (.action "sacrifice")) (some .graveyard)
private theorem movedOutMetadata (fresh : Nat) (rs : List Referent)
    (older : ∀ r ∈ rs, r.id < fresh)
    (noNames : ∀ r ∈ rs, r.idents.elem "controllerSacrifices.subject" = false) :
    (movedOut fresh rs).currentClause = 1 ∧
    (movedOut fresh rs).pendingRoles = [(fresh + 4, .obj)] := by
  unfold movedOut
  rw [argumentOutBase fresh rs older noNames]
  let base := argumentBase fresh ["controllerSacrifices.subject", "sacrifice.subject"]
     (argumentOut fresh rs)
  have found : base.findById? (fresh + 2) = some {
     id := fresh + 2, det := .target, plur := .one
     idents := ["controllerSacrifices.subject", "sacrifice.subject"]
     payload := .object [.creature] (some .battlefield) none none none } := by
    simp [base, argumentBase, argumentPrefix, DRS.findById?]
  change (applyZoneMove { base with referents := base.referents ++ rs } [fresh + 2]
       (some (.action "sacrifice")) (some .graveyard)).currentClause = 1 ∧
     (applyZoneMove { base with referents := base.referents ++ rs } [fresh + 2]
       (some (.action "sacrifice")) (some .graveyard)).pendingRoles = [(fresh + 4, .obj)]
  rw [MoveActors.applyZoneMove_append base rs _ _ _ _ found (by
    intro r hr; exact Nat.ne_of_lt (Nat.lt_add_right 2 (older r hr)))]
  simp only [applyZoneMove, show [fresh + 2].eraseDups = [fresh + 2] from rfl, List.foldl_cons, List.foldl_nil]
  simp [base, argumentBase, argumentPrefix, DRS.findById?, argumentOut,
    prepared, selectedContext, DRS.pushPerformer, DRS.push, DRS.prepend, recordIdentityById, DRS.updateById,
    DRS.recordMention, enterOwnScope, enterCaller, leaveCaller,
    mergeIdents]

private theorem movedOutStampedReferents (fresh : Nat) (rs : List Referent)
    (older : ∀ r ∈ rs, r.id < fresh)
    (noNames : ∀ r ∈ rs, r.idents.elem "controllerSacrifices.subject" = false) :
    (dropActorFrame (applyInPlaceStamp (movedOut fresh rs)
      [fresh + 4] (some (.action "sacrifice")))).referents = afterPrefix fresh++rs := by
  unfold movedOut
  rw [argumentOutBase fresh rs older noNames]
  exact movedStampedPrefixAndSuffix fresh _ (argumentOut fresh rs) rs older

/-- A context with its recorded events dropped: the events a step records
(`DRS.withCaused`) are read by no referent computation. -/
private def sans (drs : DRS) : DRS := { drs with causedEvents := [] }

private theorem referentsSans (drs : DRS) : drs.referents = (sans drs).referents := rfl

private theorem sansWithCaused (drs before : DRS) (phase : StepPhase) (action : Action) :
    sans (drs.withCaused before phase action) = sans drs := by
  unfold DRS.withCaused
  split <;> rfl

private theorem withCausedCurrentClause (drs before : DRS) (phase : StepPhase) (action : Action) :
    (drs.withCaused before phase action).currentClause = drs.currentClause := by
  unfold DRS.withCaused
  split <;> rfl

private theorem withCausedPendingRoles (drs before : DRS) (phase : StepPhase) (action : Action) :
    (drs.withCaused before phase action).pendingRoles = drs.pendingRoles := by
  unfold DRS.withCaused
  split <;> rfl

private theorem sansDropActorFrame (drs : DRS) :
    sans (dropActorFrame drs) = dropActorFrame (sans drs) := rfl

private theorem sansLeaveOwnScope (drs : DRS) :
    sans (leaveOwnScope drs) = leaveOwnScope (sans drs) := by
  unfold leaveOwnScope
  cases drs
  simp only [sans]
  split <;> rfl

private theorem sansFold (step : DRS → ReferentId → DRS)
    (commutes : ∀ drs id, sans (step drs id) = step (sans drs) id) (ids : List ReferentId) :
    ∀ drs : DRS, sans (ids.foldl step drs) = ids.foldl step (sans drs) := by
  induction ids with
  | nil => intro drs; rfl
  | cons id rest ih =>
    intro drs
    simp only [List.foldl_cons]
    rw [ih, commutes]

private theorem sansApplyInPlaceStamp (drs : DRS) (ids : List ReferentId) (deed : Option Deed) :
    sans (applyInPlaceStamp drs ids deed) = applyInPlaceStamp (sans drs) ids deed := by
  unfold applyInPlaceStamp
  split
  · rfl
  · rfl
  · exact sansFold _ (fun _ _ => rfl) ids drs

private theorem actualTailReduction (fuel fresh : Nat) (rs : List Referent)
 (older : ∀ r ∈ rs, r.id < fresh)
 (noNames : ∀ r ∈ rs, r.idents.elem "controllerSacrifices.subject" = false) :
 (dropActorFrame (stepAction (fuel + 10) .postEvent .obj (prepared fresh rs)
   (sacrifice (.laterMention "controllerSacrifices.subject" nestedSubject))).1).referents =
 (dropActorFrame (leaveOwnScope (applyInPlaceStamp (movedOut fresh rs)
   [fresh + 4] (some (.action "sacrifice"))))).referents := by
 change (dropActorFrame (stepAction (fuel + 10) .postEvent .obj (prepared fresh rs)
   (.ownScope (some (.action "sacrifice")) moveBody)).1).referents =
     (dropActorFrame (leaveOwnScope (applyInPlaceStamp (movedOut fresh rs)
       [fresh + 4] (some (.action "sacrifice"))))).referents
 rw [show fuel + 10 = (fuel + 9)+1 by omega, sacrificeScopeStep]
 rw [sacrificeStampProjected]
 rw [show fuel + 9 = (fuel + 8)+1 by omega, moveBodyStep (fuel + 8) _ (by rfl)]
 rw [show fuel + 8 = (fuel + 5)+3 by omega, argumentResult (fuel + 5) fresh rs older noNames]
 dsimp only
 conv => lhs; rw [referentsSans, sansDropActorFrame, sansWithCaused, sansLeaveOwnScope,
   sansApplyInPlaceStamp, sansWithCaused]
 conv => rhs; rw [referentsSans, sansDropActorFrame, sansLeaveOwnScope, sansApplyInPlaceStamp]
 simp only [withCausedCurrentClause, withCausedPendingRoles]
 change (dropActorFrame (leaveOwnScope (applyInPlaceStamp (sans (movedOut fresh rs))
   (List.filterMap (fun x : ReferentId × GramRole => if x.2 == .obj then some x.1 else none)
     (if (movedOut fresh rs).currentClause == 1 then
       ((movedOut fresh rs).pendingRoles.filter (!([] : List (ReferentId × GramRole)).contains ·))
      else (movedOut fresh rs).pendingRoles)) (some (.action "sacrifice"))))).referents =
     (dropActorFrame (leaveOwnScope (applyInPlaceStamp (sans (movedOut fresh rs))
       [fresh + 4] (some (.action "sacrifice"))))).referents
 rw [(movedOutMetadata fresh rs older noNames).1,
     (movedOutMetadata fresh rs older noNames).2]
 change (dropActorFrame (leaveOwnScope (applyInPlaceStamp (sans (movedOut fresh rs))
   [fresh + 4] (some (.action "sacrifice"))))).referents =
     (dropActorFrame (leaveOwnScope (applyInPlaceStamp (sans (movedOut fresh rs))
       [fresh + 4] (some (.action "sacrifice"))))).referents
 rfl

private theorem leaveOwnScopeOwned (drs : DRS) (frame : OwnScopeFrame) (outer : List OwnScopeFrame)
    (stack : drs.scopeStack = frame :: outer)
    (owned : ∀ r ∈ drs.referents,
      (frame.preEntryIds.elem r.id || frame.callerReferentIds.elem r.id) = true) :
    (leaveOwnScope drs).referents = drs.referents := by
  simp only [leaveOwnScope, stack]
  have fixed (rs : List Referent) (h : ∀ r ∈ rs,
      (frame.preEntryIds.elem r.id || frame.callerReferentIds.elem r.id) = true) :
      rs.filterMap (fun r =>
        if frame.preEntryIds.elem r.id || frame.callerReferentIds.elem r.id then some r
        else if r.det == .part || (match r.payload with | .player _ => true | _ => false) then none
        else some { r with clause := frame.callerClause, role := .obj, pronounResolved := false }) = rs := by
    induction rs with
    | nil => rfl
    | cons r rs ih =>
      simp only [List.filterMap_cons, h r (by simp), ↓reduceIte]
      congr 1
      exact ih (by intro x hx; exact h x (by simp [hx]))
  exact fixed _ owned

private theorem updateByIdPreservesIds (drs : DRS) (id : ReferentId) (f : Referent → Referent) :
    (drs.updateById id f).referents.map (·.id) = drs.referents.map (·.id) := by
  simp only [DRS.updateById, List.map_map]
  apply List.map_congr_left
  intro r _
  dsimp only [Function.comp_def]
  split <;> rfl

private theorem updateFoldPreservesIds (ids : List ReferentId) (drs : DRS)
    (f : ReferentId → Referent → Referent) :
    (ids.foldl (fun d id => d.updateById id (f id)) drs).referents.map (·.id) =
      drs.referents.map (·.id) := by
  induction ids generalizing drs with
  | nil => rfl
  | cons id ids ih => simp only [List.foldl_cons, ih, updateByIdPreservesIds]

private theorem stampPreservesIds (drs : DRS) (ids : List ReferentId) (deed : Option Deed) :
    (applyInPlaceStamp drs ids deed).referents.map (·.id) = drs.referents.map (·.id) := by
  cases deed with
  | none => rfl
  | some deed =>
    cases ids with
    | nil => rfl
    | cons id ids =>
      unfold applyInPlaceStamp
      exact updateFoldPreservesIds _ _ _

private theorem updateFoldPreservesStack (ids : List ReferentId) (drs : DRS)
    (f : ReferentId → Referent → Referent) :
    (ids.foldl (fun d id => d.updateById id (f id)) drs).scopeStack = drs.scopeStack := by
  induction ids generalizing drs with
  | nil => rfl
  | cons id ids ih =>
    rw [List.foldl_cons, ih]
    rfl

private theorem stampPreservesStack (drs : DRS) (ids : List ReferentId) (deed : Option Deed) :
    (applyInPlaceStamp drs ids deed).scopeStack = drs.scopeStack := by
  cases deed with
  | none => rfl
  | some deed =>
    cases ids with
    | nil => rfl
    | cons id ids =>
      unfold applyInPlaceStamp
      exact updateFoldPreservesStack _ _ _

private theorem leaveStampedOwned (drs : DRS) (frame : OwnScopeFrame) (outer : List OwnScopeFrame)
    (ids : List ReferentId) (deed : Option Deed)
    (stack : drs.scopeStack = frame :: outer)
    (owned : ∀ r ∈ drs.referents,
      (frame.preEntryIds.elem r.id || frame.callerReferentIds.elem r.id) = true) :
    (leaveOwnScope (applyInPlaceStamp drs ids deed)).referents =
      (applyInPlaceStamp drs ids deed).referents := by
  apply leaveOwnScopeOwned _ frame outer (by rw [stampPreservesStack, stack])
  intro r hr
  have hid : r.id ∈ (applyInPlaceStamp drs ids deed).referents.map (·.id) :=
    List.mem_map.mpr ⟨r, hr, rfl⟩
  rw [stampPreservesIds] at hid
  obtain ⟨old, hold, same⟩ := List.mem_map.mp hid
  rw [← same]
  exact owned old hold

private theorem movedPreowned (drs : DRS) (frame : OwnScopeFrame) (outer : List OwnScopeFrame)
    (old : Referent) (ty : List CardType) (zn : Option Zone) (stamp : Option Stamp)
    (orig : Option Origin) (size : Option Nat)
    (deed : Option Deed) (dest : Option Zone)
    (found : drs.findById? old.id = some old)
    (payload : old.payload = .object ty zn stamp orig size)
    (historical : drs.historicalReferentIds.contains old.id = false)
    (stack : drs.scopeStack = frame :: outer)
    (preowned : ∀ r ∈ drs.referents, frame.preEntryIds.elem r.id = true)
    (selected : (frame.preEntryIds.elem old.id || frame.callerReferentIds.elem old.id) = true) :
    let after := applyZoneMove drs [old.id] deed dest
    ∃ next outer', after.scopeStack = next :: outer' ∧
      ∀ r ∈ after.referents,
        (next.preEntryIds.elem r.id || next.callerReferentIds.elem r.id) = true := by
  dsimp only
  simp only [applyZoneMove, show [old.id].eraseDups = [old.id] from rfl,
    List.foldl_cons, List.foldl_nil, found, historical, Bool.false_eq_true, ↓reduceIte,
    payload, stack, List.map_cons, selected]
  refine ⟨_, _, rfl, ?_⟩
  intro r hr
  obtain ⟨before, hb, same⟩ := List.mem_map.mp hr
  subst r
  split
  · simp
  · cases before.payload <;> simp only [preowned before hb, Bool.true_or]

private theorem actualMovedScopeClosure (fresh : Nat) (rs : List Referent)
    (older : ∀ r ∈ rs, r.id < fresh)
    (noNames : ∀ r ∈ rs, r.idents.elem "controllerSacrifices.subject" = false) :
    (leaveOwnScope (applyInPlaceStamp (movedOut fresh rs)
      [fresh + 4] (some (.action "sacrifice")))).referents =
    (applyInPlaceStamp (movedOut fresh rs) [fresh + 4] (some (.action "sacrifice"))).referents := by
  let input := argumentOut fresh rs
  let frame := input.scopeStack.headD { preEntryIds := [], maskedIds := [], callerClause := 0 }
  have stack : input.scopeStack = [frame] := by rfl
  have ids : input.referents.map (·.id) = (prepared fresh rs).referents.map (·.id) := by
    change (recordIdentityById "sacrifice.subject" (prepared fresh rs) (fresh + 2)).referents.map (·.id) = _
    exact updateByIdPreservesIds _ _ _
  have pre : frame.preEntryIds = (prepared fresh rs).referents.map (·.id) := by rfl
  have owned : ∀ r ∈ input.referents, frame.preEntryIds.elem r.id = true := by
    intro r hr
    rw [pre, ← ids]
    simpa using (List.mem_map.mpr ⟨r, hr, rfl⟩ : r.id ∈ input.referents.map (·.id))
  have caller := argumentCallerOwnership 0 fresh rs older noNames
  rw [argumentResult 0 fresh rs older noNames] at caller
  change frame.callerReferentIds.contains (fresh + 2) = true at caller
  let old : Referent := {
    id := fresh + 2, det := .target, plur := .one,
    idents := ["controllerSacrifices.subject", "sacrifice.subject"],
    payload := .object [.creature] (some .battlefield) none none none }
  have found : input.findById? old.id = some old := by
    unfold DRS.findById?
    change (argumentOut fresh rs).referents.find? _ = _
    rw [argumentOutPrefix fresh rs older noNames]
    simp [argumentPrefix, old]
  have selected : (frame.preEntryIds.elem old.id || frame.callerReferentIds.elem old.id) = true := by
    change (frame.preEntryIds.elem (fresh + 2) || frame.callerReferentIds.elem (fresh + 2)) = true
    change frame.callerReferentIds.elem (fresh + 2) = true at caller
    rw [caller, Bool.or_true]
  obtain ⟨next, outer, hstack, howned⟩ := movedPreowned input frame [] old
    [.creature] (some .battlefield) none none none
    (some (.action "sacrifice")) (some .graveyard) found rfl (by rfl) stack owned selected
  exact leaveStampedOwned (movedOut fresh rs) next outer [fresh + 4]
    (some (.action "sacrifice")) hstack howned

private theorem actualFullTail (fuel fresh : Nat) (rs : List Referent)
 (older : ∀ r ∈ rs, r.id < fresh)
    (noNames : ∀ r ∈ rs, r.idents.elem "controllerSacrifices.subject" = false) :
 (dropActorFrame (stepAction (fuel + 10) .postEvent .obj (prepared fresh rs)
   (sacrifice (.laterMention "controllerSacrifices.subject" nestedSubject))).1).referents =
 afterPrefix fresh++rs := by
 rw [actualTailReduction fuel fresh rs older noNames]
 change (leaveOwnScope (applyInPlaceStamp (movedOut fresh rs)
   [fresh + 4] (some (.action "sacrifice")))).referents = _
 rw [actualMovedScopeClosure fresh rs older noNames]
 exact movedOutStampedReferents fresh rs older noNames

private theorem sourceCanonical (fuel : Nat) (drs : DRS) :
    (stepNounPhrase (fuel + 10) drs .subj true
      (controllerOf (.firstMention "controllerSacrifices.subject" nestedSubject))).1 =
      selectedContext drs := by rfl

private def controllerInstructState (fuel : Nat) (drs : DRS)
    (patient : NounPhrase) (body : Action) : DRS :=
  let subject := controllerOf patient
  let tracked := { drs with
    outerReferentIds := drs.referents.map (·.id) :: drs.outerReferentIds }
  let (selected, ids, _) := stepNounPhrase fuel tracked .subj true subject
  let shape : ActorShape := {
    kind := (ids.filterMap selected.findById?).foldl
      (fun kind ref => match kind with
        | none => some ref.kind
        | some prior => some (joinKinds prior ref.kind)) none
        |>.getD (subject.kindIn selected .player)
    isYou := subject.isYou
    ascribable := false
    refIds := ids }
  let selves := selected.handedSelves ids
  let framed := selected.pushPerformer { shape with selves }
  let (after, _) := withDistributiveSubject subject ids framed (stepAction fuel .postEvent .obj · body)
  let out := performerOutOfView (dropActorFrame after) selves
  { out with outerReferentIds := out.outerReferentIds.drop 1 }

private theorem controllerInstructStep (fuel : Nat) (drs : DRS)
    (patient : NounPhrase) (body : Action) :
    (stepAction (fuel + 1) .postEvent .obj drs
      (.instruct (controllerOf patient) body)).1 =
      controllerInstructState fuel drs patient body := by rfl

private def controllerRecord (drs : DRS) : Referent :=
  { id := drs.nextId + 3, clause := drs.currentClause, role := .subj,
    det := .the, plur := .one, payload := .player false }

private theorem selectedController (drs : DRS) :
    (selectedContext drs).findById? (drs.nextId + 3) = some (controllerRecord drs) := by
  unfold DRS.findById?
  change (controllerRecord drs :: _).find? (fun r => r.id == drs.nextId + 3) = _
  simp only [List.find?_cons, controllerRecord, BEq.rfl]

private theorem sourceIds (fuel : Nat) (drs : DRS) :
    (stepNounPhrase (fuel + 10) drs .subj true
      (controllerOf (.firstMention "controllerSacrifices.subject" nestedSubject))).2.1 =
      [drs.nextId + 3] := by rfl

/-- "Its controller" names no distributive group, so the body's "another"/"other" has no group
member to contrast with. -/
private theorem controllerNotDistributive (ids : List ReferentId) :
    (controllerOf (.firstMention "controllerSacrifices.subject" nestedSubject)).distributiveGroup?
      ids = none := rfl

/-- A clause whose subject names no distributive group steps its rest with no group in view,
and the referents it leaves are those of that step. -/
private theorem nonDistributiveReferents (subject : NounPhrase) (ids : List ReferentId)
    (drs : DRS) (rest : DRS → DRS × List Refusal) (plain : subject.distributiveGroup? ids = none) :
    (dropActorFrame (withDistributiveSubject subject ids drs rest).1).referents =
      (dropActorFrame (rest { drs with distributiveSubject := none }).1).referents := by
  unfold withDistributiveSubject
  rw [plain]
  rfl

/-- The controller is no self-referent, so the handoff has nothing to put in view. -/
private theorem controllerAlreadyInView (drs : DRS) :
    (selectedContext { drs with
      outerReferentIds := drs.referents.map (·.id) :: drs.outerReferentIds }).handedSelves
        [drs.nextId + 3] = [] := by
  have h := selectedController { drs with
    outerReferentIds := drs.referents.map (·.id) :: drs.outerReferentIds }
  simp only [DRS.handedSelves, List.filter_cons, List.filter_nil, h, controllerRecord,
    Option.any_some]
  rfl

private theorem controllerSourceAndBody (fuel : Nat) (drs : DRS) :
    let tracked := { drs with
      outerReferentIds := drs.referents.map (·.id) :: drs.outerReferentIds }
    let source := selectedContext tracked
    let frame : ActorShape := {
      kind := .player, isYou := false, ascribable := false,
      refIds := [drs.nextId + 3] }
    let prepared := { source.pushPerformer frame with distributiveSubject := none }
    (stepAction (fuel + 11) .postEvent .obj drs
      (controllerSacrifices nestedSubject)).1.referents =
      (dropActorFrame (stepAction (fuel + 10) .postEvent .obj prepared
        (sacrifice (.laterMention "controllerSacrifices.subject" nestedSubject))).1).referents := by
  dsimp only
  -- Unfold the template in the action alone: a `change` of the whole step would compare the two
  -- steps by unfolding them.
  rw [show controllerSacrifices nestedSubject =
      .instruct (controllerOf (.firstMention "controllerSacrifices.subject" nestedSubject))
        (sacrifice (.laterMention "controllerSacrifices.subject" nestedSubject)) from rfl]
  rw [show fuel + 11 = (fuel + 10) + 1 by omega, controllerInstructStep]
  unfold controllerInstructState
  simp only [nonDistributiveReferents _ _ _ _ (controllerNotDistributive _), sourceCanonical,
    sourceIds, controllerAlreadyInView, performerOutOfView, List.filterMap_cons, List.filterMap_nil,
    selectedController, List.foldl_cons, List.foldl_nil, controllerRecord]
  rfl

/-- The complete action moves only its selected new creature and preserves every outer
referent, unrelated named referents included. -/
private theorem nestedDescriptionReferents (rs : List Referent) (fresh : ReferentId)
    (older : ∀ ref ∈ rs, ref.id < fresh)
    (nameFresh : ∀ ref ∈ rs, ref.idents.elem "controllerSacrifices.subject" = false) :
    (stepAction 64 .postEvent .obj { referents := rs, nextId := fresh }
      nestedAction).1.referents = afterPrefix fresh ++ rs := by
  change (stepAction 64 .postEvent .obj { referents := rs, nextId := fresh }
    (controllerSacrifices nestedSubject)).1.referents = _
  rw [controllerSourceAndBody 53]
  change (dropActorFrame (stepAction 63 .postEvent .obj (prepared fresh rs)
    (sacrifice (.laterMention "controllerSacrifices.subject" nestedSubject))).1).referents = _
  exact actualFullTail 53 fresh rs older nameFresh

/-- The complete action moves only its selected new creature and preserves every outer
referent, unrelated named referents included: of its four participants only the creature target
moves, to the graveyard, and the creature it was before is gone; the artifact target stays on
the battlefield. -/
theorem nestedDescriptionMovesOnlyItsOwnReferent (rs : List Referent) (fresh : ReferentId)
    (older : ∀ ref ∈ rs, ref.id < fresh)
    (nameFresh : ∀ ref ∈ rs, ref.idents.elem "controllerSacrifices.subject" = false) :
    let after := (stepAction 64 .postEvent .obj { referents := rs, nextId := fresh }
      nestedAction).1
    let published := after.referents.take 4
    after.referents.drop 4 = rs ∧
      published.map (fun ref => (ref.kind, ref.det, ref.zone, ref.ty)) =
        [(.player, .the, none, []), (.object, .target, some .graveyard, [.creature]),
          (.player, .the, none, []), (.object, .target, some .battlefield, [.artifact])] ∧
      (published.filter (·.zone == some .graveyard)).all (fun moved =>
        moved.movedFrom.all (fun id => (after.referents.find? (·.id == id)).isNone)) := by
  intro after published
  have h := nestedDescriptionReferents rs fresh older nameFresh
  simp only [after, published, h, afterPrefix]
  refine ⟨rfl, rfl, ?_⟩
  have absent : ∀ ref ∈ rs, ref.id ≠ fresh + 2 := fun ref mem => by
    exact Nat.ne_of_lt (Nat.lt_of_lt_of_le (older ref mem) (Nat.le_add_right _ _))
  simp
  exact Or.inr absent

end Semantics.Proofs.ControllerSacrifice

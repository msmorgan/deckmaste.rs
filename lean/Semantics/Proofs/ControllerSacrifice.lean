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

-- Parameter identity selects the target before its zone change.
theorem expansionIntroducesTheTargetOnce :
    action = .instruct
      (.possessorOf .controller (.firstMention "controllerSacrifices.subject" subject))
      (sacrifice (.laterMention "controllerSacrifices.subject" subject)) := by rfl

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
      (fun ref => (ref.id, ref.kind, ref.zone, ref.ty)) =
      [(3, .player, none, []), (2, .object, some .battlefield, [.creature]),
        (1, .player, none, []), (0, .object, some .battlefield, [.artifact])] := by decide

theorem nestedDescriptionDeclaresExactlyItsTwoTargets :
    (stepAction 64 .postEvent .obj (DRS.ofList []) nestedAction).1.referents.countP
      (·.det == .target) = 2 := by decide

-- P08: replacing the explicit parameter read with an ordinary pronoun leaves a true tie.
theorem wholeOwnedScopeWouldBeAmbiguous :
    Action.check (DRS.ofList [])
      (instruct (controllerOf nestedSubject) (sacrifice it)) =
        [.anaphor .bare .one 2] := by decide

theorem sacrificeStampSelectsTheSameObject :
    (firstReach (.verbed (.action "sacrifice") none .attributive) .one afterSacrifice).map (·.id) =
      (firstReach (.word .card) .one afterSacrifice).map (·.id) := by decide

theorem formerPermanentReadIsAdmitted :
    NounPhrase.checkAt (some .object) afterSacrifice
      (theVerbed (.action "sacrifice") .one (some .permanent) .thisWay) = [] := by decide

theorem thatPlayerCanTakeTheFollowingAction :
    Action.check (DRS.ofList [])
      (.sequentially [action, instruct (that .player) (draw (.lit 1))]) = [] := by decide

private def nestedAfter : DRS :=
  (stepAction 64 .postEvent .obj (DRS.ofList []) nestedAction).1

theorem nestedDescriptionMovesOnlyItsOwnReferentWithoutOuterContext :
    (firstReach (.word (.type .artifact)) .one nestedAfter).map (·.zone) = some (some .battlefield) ∧
    (firstReach (.word .card) .one nestedAfter).map (·.zone) = some (some .graveyard) := by decide

private def authoringWitness : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Controller sacrifice witness", types := [.sorcery], cost := some [generic 1],
      text := [Primitives.Ability.spell none
        (controllerSacrifices (target (Primitives.Predicate.and [isArtifact, isCreature])))] } }

theorem compositionPassesTheAuthoringBoundary :
    authoringWitness.authoring.onlyAuthored = true := by decide

theorem controlGrantRecordsItsPatientIdentity :
    let (after, errors) := stepAction 64 .postEvent .obj (DRS.ofList [])
      (gainControl (target isCreature) (some untilEndOfTurn))
    let refs := resolveCandidates (.verbed (.core .gainControl) none .attributive)
      .one (nextClause after)
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
      (firstReach (.word .card) .one (nextClause after)).map (·.id) = some 3 ∧
      (after.findById? 3).bind (·.zone) = some .graveyard := by decide

theorem reflexiveFollowUpRemainsAvailable :
    action.reflexEncloseUse = .reflexive := by decide

theorem riderReadsTheStampedPatient :
    let patient := theVerbed (.action "sacrifice") .one (some .card) .thisWay
    let (after, ids, errors) := stepNounPhrase 64 (nextClause afterSacrifice) .obj true patient
    Action.check (DRS.ofList [])
      (.withContinuation .required action (some (.expose .reveal (.cards patient))) none) = [] ∧
      errors = [] ∧ ids = [3] ∧
      (ids.filterMap after.findById?).map (fun ref => (ref.zone, ref.ty)) =
        [(some .graveyard, [.creature, .artifact])] := by decide

theorem controllerSacrificesExpansionForEverySubject (chosen : NounPhrase) :
    controllerSacrifices chosen =
      .instruct (.possessorOf .controller
        (.firstMention "controllerSacrifices.subject" chosen))
        (sacrifice (.laterMention "controllerSacrifices.subject" chosen)) := by rfl

theorem repeatedControllerSacrificesKeepDistinctPatients :
    let (after, errors) := stepAction 64 .postEvent .obj (DRS.ofList [])
      (.sequentially [action, action])
    errors = [] ∧
      (after.referents.filter (fun ref => ref.kind == .object)).map
        (fun ref => (ref.id, ref.zone, ref.idents)) =
        [(7, some .graveyard, []), (3, some .graveyard, [])] := by decide

theorem duplicatingThePatientWouldIntroduceTwoTargets :
    (stepAction 64 .postEvent .obj (DRS.ofList [])
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
          (some ⟨.action "sacrifice", true, true⟩) none none) := by decide

theorem outerArtifactDoesNotStealThePatient :
    let outer : DRS := DRS.ofList
      [{ det := .target, plur := .one,
         payload := .object [.artifact] (some .battlefield) none none (some 1) }]
    let (after, errors) := stepAction 64 .postEvent .obj outer action
    errors = [] ∧ after.findById? 0 = outer.findById? 0 ∧
      (firstReach (.word .card) .one (nextClause after)).map (·.id) = some 4 := by decide

private def universallySelectedPatient (id : ReferentId) : Referent :=
  { id, det := .target, plur := .one,
    payload := .object [.creature] (some .battlefield) none none none }

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

private theorem dropFirst_append (before suffix : List Referent)
    (present : ∃ ref ∈ before, ref.isActorFrame = true) :
    dropActorFrame.dropFirst (before ++ suffix) =
      dropActorFrame.dropFirst before ++ suffix := by
  induction before with
  | nil => simp at present
  | cons ref before ih =>
    by_cases isActor : ref.isActorFrame = true
    · simp only [List.cons_append, dropActorFrame.dropFirst, isActor, ↓reduceIte]
    · have inTail : ∃ ref ∈ before, ref.isActorFrame = true := by
        obtain ⟨candidate, member, actor⟩ := present
        rcases List.mem_cons.mp member with equal | inside
        · subst candidate
          contradiction
        · exact ⟨candidate, inside, actor⟩
      simp only [List.cons_append, dropActorFrame.dropFirst, isActor]
      exact congrArg (List.cons ref) (ih inTail)

private theorem dropActorFrame_append (drs : DRS) (suffix : List Referent)
    (present : ∃ ref ∈ drs.referents, ref.isActorFrame = true) :
    dropActorFrame { drs with referents := drs.referents ++ suffix } =
      { dropActorFrame drs with referents := (dropActorFrame drs).referents ++ suffix } := by
  exact congrArg (fun refs => { drs with referents := refs })
    (dropFirst_append drs.referents suffix present)

private theorem laterMention_step (fuel : Nat) (drs : DRS)
    (role : GramRole) (arg : Bool) (name : BindingId) (fallback : NounPhrase) :
    stepNounPhrase (fuel + 1) drs role arg (.laterMention name fallback) =
      let matching := drs.visibleReferents.filter (fun ref => ref.idents.elem name)
      match matching with
      | [] => (drs, [], [.bindingIdentity name])
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
  { named with characteristicRead := drs.characteristicRead }.push
    { det := .the, plur := .one, payload := .player false } .subj true

private def sacrificeEnactState (fuel : Nat) (drs : DRS) (body : Action) : DRS :=
 let priorMentions := drs.pendingRoles.length
 let (after, _) := stepAction fuel .postEvent .obj drs body
 let mentions := if after.currentClause == drs.currentClause then
    after.pendingRoles.take (after.pendingRoles.length - priorMentions)
    else after.pendingRoles
 let actedIds := mentions.filterMap fun (id, role) => if role == .obj then some id else none
 applyInPlaceStamp after actedIds (some (.action "sacrifice"))
private theorem sacrificeEnactStep (fuel : Nat) (drs : DRS) (body : Action) :
 (stepAction (fuel + 1) .postEvent .obj drs (.enact (.action "sacrifice") body none none)).1 =
 sacrificeEnactState fuel drs body := by rfl

private theorem sacrificeEnactProjected (fuel : Nat) (drs : DRS) (body : Action) :
 sacrificeEnactState fuel drs body =
 let after := (stepAction fuel .postEvent .obj drs body).1
 let mentions := if after.currentClause == drs.currentClause then
   after.pendingRoles.take (after.pendingRoles.length - drs.pendingRoles.length)
    else after.pendingRoles
 let actedIds := mentions.filterMap fun (id, role) => if role == .obj then some id else none
 applyInPlaceStamp after actedIds (some (.action "sacrifice")) := by
 unfold sacrificeEnactState
 cases stepAction fuel .postEvent .obj drs body
 rfl
private theorem ownScopeStep (fuel : Nat) (drs : DRS) (body : Action) :
 (stepAction (fuel + 1) .postEvent .obj drs (.ownScope (some (.action "sacrifice")) body)).1 =
 leaveOwnScope (stepAction fuel .postEvent .obj (enterOwnScope drs) body).1 := by rfl
private def prepared (fresh : Nat) (rs : List Referent) : DRS :=
 let tracked : DRS := { referents := rs, nextId := fresh, outerReferentIds := [rs.map (·.id)]}
 let source := selectedContext tracked
 let frame : ActorShape := {
   kind := .player, isYou := false, ascribable := false,
   refIds := [fresh + 3] }
 source.push { det := .the, plur := .one, payload := .actor frame } .other false
private def moveArg : NounPhrase :=
 .inCaller (.firstMention "sacrifice.subject"
   (.laterMention "controllerSacrifices.subject" nestedSubject))
private def moveBody : Action :=
 .move moveArg (.zone .battlefield none) (some graveyard) .departure none []
private theorem moveBodyStep (fuel : Nat) (drs : DRS) (ordinary : drs.actionPattern = false) :
 (stepAction (fuel + 1) .postEvent .obj drs moveBody).1 =
 let selected := stepNounPhrase fuel drs .obj true moveArg
 applyZoneMove { selected.1 with historicalMoveFrom := drs.historicalMoveFrom }
   selected.2.1 (some (.core .put)) (some .graveyard) := by
 cases drs with
 | mk => simp only at ordinary; subst ordinary; rfl

private def argumentPrefix (fresh : Nat) (names : List BindingId) : List Referent :=
 [ { id := fresh + 4, det := .the, plur := .one, payload := .actor {
       kind := .player, isYou := false, ascribable := false,
       refIds := [fresh + 3]} },
   { id := fresh + 3, det := .the, plur := .one, payload := .player false, role := .subj },
   { id := fresh + 2, det := .target, plur := .one, idents := names,
     payload := .object [.creature] (some .battlefield) none none none },
   { id := fresh + 1, det := .the, plur := .one, payload := .player false },
   { id := fresh, det := .target, plur := .one,
     payload := .object [.artifact] (some .battlefield) none none none } ]
private def argumentBase (fresh : Nat) (names : List BindingId) (data : DRS) : DRS :=
 { data with referents := argumentPrefix fresh names, nextId := fresh + 5, historicalReferentIds := [] }
private def afterPrefix (fresh : Nat) : List Referent :=
 [ { id := fresh + 3, det := .the, plur := .one, payload := .player false, role := .subj },
   { id := fresh + 5, det := .target, plur := .one,
     payload := .object [.creature] (some .graveyard)
       (some ⟨.action "sacrifice", true, true⟩) none none },
   { id := fresh + 1, det := .the, plur := .one, payload := .player false },
   { id := fresh, det := .target, plur := .one,
     payload := .object [.artifact] (some .battlefield) none none none } ]
private theorem isolatedMoveStamp (fresh : Nat) (names : List BindingId) (data : DRS) :
 (dropActorFrame (applyInPlaceStamp
   (applyZoneMove (argumentBase fresh names data) [fresh + 2] (some (.core .put)) (some .graveyard))
   [fresh + 5] (some (.action "sacrifice")))).referents = afterPrefix fresh := by
 simp only [applyZoneMove, show [fresh + 2].eraseDups = [fresh + 2] from rfl,
   List.foldl_cons, List.foldl_nil]
 simp [argumentBase, argumentPrefix, afterPrefix, DRS.findById?,
   applyInPlaceStamp, DRS.updateById, dropActorFrame, dropActorFrame.dropFirst,
   Referent.isActorFrame, mkStamp, onFieldZone, zoneIsB, mergeIdents, List.foldl_cons, List.foldl_nil]

private theorem argumentStep (fuel : Nat) (drs : DRS) :
    stepNounPhrase (fuel + 3) drs .obj true moveArg =
      let caller := enterCaller drs
      let selected := stepNounPhrase (fuel + 1) caller .obj true
        (.laterMention "controllerSacrifices.subject" nestedSubject)
      let named := selected.2.1.foldl
        (fun state id => recordIdentityById "sacrifice.subject" state id) selected.1
      (leaveCaller drs named selected.2.1, selected.2.1, selected.2.2) := by rfl

private theorem preparedSuffix (fresh : Nat) (rs : List Referent)
    (older : ∀ r ∈ rs, r.id < fresh) :
    (prepared fresh rs).referents = (prepared fresh []).referents ++ rs := by
  simp [prepared, selectedContext, DRS.push, DRS.prepend, recordIdentityById, DRS.updateById]
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
    (enterCaller (enterOwnScope (prepared fresh rs))).visibleReferents.filter
      (fun ref => ref.idents.elem "controllerSacrifices.subject") =
      [namedPatient fresh] := by
  have noSuffix : rs.filter (fun ref => ref.idents.elem "controllerSacrifices.subject") = [] := by
    apply List.filter_eq_nil_iff.mpr
    intro ref member
    simp only [noNames ref member, Bool.false_eq_true, not_false_eq_true]
  have refs : (enterCaller (enterOwnScope (prepared fresh rs))).referents =
      (prepared fresh rs).referents := by rfl
  have visible : (enterCaller (enterOwnScope (prepared fresh rs))).visibleReferents =
      (enterCaller (enterOwnScope (prepared fresh rs))).referents := by
    apply List.filter_eq_self.mpr
    intro ref _
    rfl
  rw [visible, refs, preparedSuffix fresh rs older, List.filter_append, noSuffix,
    List.append_nil]
  simp [prepared, selectedContext, DRS.push, DRS.prepend, recordIdentityById,
    DRS.updateById, mergeIdents, namedPatient]

private theorem argumentResult (fuel fresh : Nat) (rs : List Referent)
    (older : ∀ r ∈ rs, r.id < fresh)
    (noNames : ∀ r ∈ rs, r.idents.elem "controllerSacrifices.subject" = false) :
    stepNounPhrase (fuel + 3) (enterOwnScope (prepared fresh rs)) .obj true moveArg =
      let inner := enterOwnScope (prepared fresh rs)
      let caller := enterCaller inner
      let mentioned := caller.recordMention (fresh + 2) .obj true
      let named := recordIdentityById "sacrifice.subject" mentioned (fresh + 2)
      (leaveCaller inner named [fresh + 2], [fresh + 2], []) := by
  rw [argumentStep]
  dsimp only
  rw [ControllerSuffixLocality.laterMention_step,
    argumentNamedPatient fresh rs older noNames]
  rfl

private theorem argumentReferents (fuel fresh : Nat) (rs : List Referent)
    (older : ∀ r ∈ rs, r.id < fresh)
    (noNames : ∀ r ∈ rs, r.idents.elem "controllerSacrifices.subject" = false) :
    (stepNounPhrase (fuel + 3) (enterOwnScope (prepared fresh rs)) .obj true moveArg).1.referents =
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
    let out := (stepNounPhrase (fuel + 3) (enterOwnScope (prepared fresh rs)) .obj true moveArg).1
    out.scopeStack.head?.any (fun frame => frame.callerReferentIds.contains (fresh + 2)) = true := by
  rw [argumentResult fuel fresh rs older noNames]
  change ((([] ++ ([fresh + 2] ++ [fresh + 2] ++ _).eraseDups).eraseDups).contains (fresh + 2)) = true
  simp

namespace MoveActors
private theorem remapFixed (ids : List ReferentId) (id fresh : ReferentId)
    (different : ∀ value ∈ ids, value ≠ id) :
    ids.map (fun value => if value == id then fresh else value) = ids := by
  induction ids with
  | nil => rfl
  | cons value ids ih =>
    have h := different value (by simp)
    simp only [List.map_cons, show (value == id) = false by simp [h], Bool.false_eq_true, ↓reduceIte]
    congr 1
    exact ih (by intro value member; exact different value (by simp [member]))
private theorem movingMapSuffixWithActors (suffix : List Referent) (id fresh : ReferentId)
    (payload : Payload)
    (distinct : ∀ ref ∈ suffix, ref.id ≠ id)
    (actorLinks : ∀ ref ∈ suffix, ∀ shape, ref.payload = .actor shape →
      ∀ value ∈ shape.refIds, value ≠ id) :
    suffix.map (fun r =>
      if r.id == id then
        { r with
          id := fresh
          idents := []
          payload := payload
          librarySearchResult := false }
      else match r.payload with
        | .actor shape =>
          { r with payload := .actor { shape with
              refIds := shape.refIds.map (fun value => if value == id then fresh else value) } }
        | _ => r) = suffix := by
  induction suffix with
  | nil => rfl
  | cons ref suffix ih =>
    have different : (ref.id == id) = false := by simp [distinct ref (by simp)]
    have links := actorLinks ref (by simp)
    simp only [List.map_cons, different, Bool.false_eq_true, ↓reduceIte]
    have tail := ih (by intro r h; exact distinct r (by simp [h]))
      (by intro r h; exact actorLinks r (by simp [h]))
    cases hp : ref.payload <;> try simp_all
    case actor shape =>
      have fixed := remapFixed shape.refIds id fresh (links)
      simp only [beq_iff_eq] at fixed
      cases ref <;> simp_all

private theorem applyZoneMove_appendWithActors (drs : DRS) (suffix : List Referent)
    (id : ReferentId) (patient : Referent) (deed : Option Deed) (destination : Option Zone)
    (found : drs.findById? id = some patient)
    (distinct : ∀ ref ∈ suffix, ref.id ≠ id)
    (actorLinks : ∀ ref ∈ suffix, ∀ shape, ref.payload = .actor shape →
      ∀ value ∈ shape.refIds, value ≠ id) :
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
        (movingMapSuffixWithActors suffix id drs.nextId _ distinct actorLinks)

end MoveActors

private theorem movedStampedPrefixAndSuffixWithActors (fresh : Nat) (names : List BindingId)
    (data : DRS) (rs : List Referent)
    (older : ∀ ref ∈ rs, ref.id < fresh)
    (actorLinks : ∀ ref ∈ rs, ∀ shape, ref.payload = .actor shape →
      ∀ value ∈ shape.refIds, value ≠ fresh + 2) :
    let base := argumentBase fresh names data
    (dropActorFrame (applyInPlaceStamp
      (applyZoneMove { base with referents := base.referents++rs }
        [fresh + 2] (some (.core .put)) (some .graveyard))
      [fresh + 5] (some (.action "sacrifice")))).referents = afterPrefix fresh ++rs := by
  dsimp only
  have found : (argumentBase fresh names data).findById? (fresh + 2) =
      some { id := fresh + 2, det := .target, plur := .one, idents := names,
             payload := .object [.creature] (some .battlefield) none none none } := by
    simp [argumentBase, argumentPrefix, DRS.findById?]
  rw [MoveActors.applyZoneMove_appendWithActors _ rs _ _ _ _ found (by
    intro r hr; have h := older r hr; exact Nat.ne_of_lt (Nat.lt_add_right 2 h)) actorLinks]
  rw [ControllerSuffixLocality.applyInPlaceStamp_append _ rs _ _ (by
    intro r hr id hid
    simp only [List.mem_cons, List.not_mem_nil, or_false] at hid
    subst id
    have h := older r hr
    exact Nat.ne_of_lt (Nat.lt_add_right 5 h))]
  have active : ∃ ref ∈ (applyInPlaceStamp
      (applyZoneMove (argumentBase fresh names data) [fresh + 2]
        (some (.core .put)) (some .graveyard)) [fresh + 5]
      (some (.action "sacrifice"))).referents, ref.isActorFrame = true := by
    have head : ((applyInPlaceStamp
        (applyZoneMove (argumentBase fresh names data) [fresh + 2]
          (some (.core .put)) (some .graveyard)) [fresh + 5]
        (some (.action "sacrifice"))).referents.head?).map (·.isActorFrame) = some true := by
      simp only [applyZoneMove, show [fresh + 2].eraseDups = [fresh + 2] from rfl,
        List.foldl_cons, List.foldl_nil]
      simp [argumentBase, argumentPrefix, DRS.findById?,
        applyInPlaceStamp, DRS.updateById, Referent.isActorFrame, mergeIdents]
    cases h : (applyInPlaceStamp
        (applyZoneMove (argumentBase fresh names data) [fresh + 2]
          (some (.core .put)) (some .graveyard)) [fresh + 5]
        (some (.action "sacrifice"))).referents with
    | nil => simp [h] at head
    | cons ref refs =>
      rw [h] at head
      exact ⟨ref, by simp, by simpa only [List.head?_cons, Option.map_some, Option.some.injEq] using head⟩
  rw [ControllerSuffixLocality.dropActorFrame_append _ rs active]
  simp only [isolatedMoveStamp]

private def argumentOut (fresh : Nat) (rs : List Referent) : DRS :=
 let inner := enterOwnScope (prepared fresh rs)
 leaveCaller inner (recordIdentityById "sacrifice.subject"
   ((enterCaller inner).recordMention (fresh + 2) .obj true) (fresh + 2)) [fresh + 2]

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
  simp [argumentPrefix, prepared, selectedContext, DRS.push, DRS.prepend,
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
  have next : (argumentOut fresh rs).nextId = fresh + 5 := rfl
  have hist : (argumentOut fresh rs).historicalReferentIds = [] := rfl
  cases h : argumentOut fresh rs
  simp only [h] at refs next hist
  simp_all [argumentBase]

private def movedOut (fresh : Nat) (rs : List Referent) : DRS :=
 applyZoneMove (argumentOut fresh rs) [fresh + 2] (some (.core .put)) (some .graveyard)
private theorem movedOutMetadata (fresh : Nat) (rs : List Referent)
    (older : ∀ r ∈ rs, r.id < fresh)
    (noNames : ∀ r ∈ rs, r.idents.elem "controllerSacrifices.subject" = false)
    (actorLinks : ∀ r ∈ rs, ∀ shape, r.payload = .actor shape →
      ∀ value ∈ shape.refIds, value ≠ fresh + 2) :
    (movedOut fresh rs).currentClause = 1 ∧
    (movedOut fresh rs).pendingRoles = [(fresh + 5, .obj)] := by
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
       (some (.core .put)) (some .graveyard)).currentClause = 1 ∧
     (applyZoneMove { base with referents := base.referents ++ rs } [fresh + 2]
       (some (.core .put)) (some .graveyard)).pendingRoles = [(fresh + 5, .obj)]
  rw [MoveActors.applyZoneMove_appendWithActors base rs _ _ _ _ found (by
    intro r hr; exact Nat.ne_of_lt (Nat.lt_add_right 2 (older r hr))) actorLinks]
  simp only [applyZoneMove, show [fresh + 2].eraseDups = [fresh + 2] from rfl, List.foldl_cons, List.foldl_nil]
  simp [base, argumentBase, argumentPrefix, DRS.findById?, argumentOut,
    prepared, selectedContext, DRS.push, DRS.prepend, recordIdentityById, DRS.updateById,
    DRS.recordMention, enterOwnScope, enterCaller, leaveCaller,
    mergeIdents]

private theorem movedOutStampedReferents (fresh : Nat) (rs : List Referent)
    (older : ∀ r ∈ rs, r.id < fresh)
    (noNames : ∀ r ∈ rs, r.idents.elem "controllerSacrifices.subject" = false)
    (actorLinks : ∀ r ∈ rs, ∀ shape, r.payload = .actor shape →
      ∀ value ∈ shape.refIds, value ≠ fresh + 2) :
    (dropActorFrame (applyInPlaceStamp (movedOut fresh rs)
      [fresh + 5] (some (.action "sacrifice")))).referents = afterPrefix fresh++rs := by
  unfold movedOut
  rw [argumentOutBase fresh rs older noNames]
  exact movedStampedPrefixAndSuffixWithActors fresh _ (argumentOut fresh rs) rs older actorLinks

private theorem actualTailReduction (fuel fresh : Nat) (rs : List Referent)
 (older : ∀ r ∈ rs, r.id < fresh)
 (noNames : ∀ r ∈ rs, r.idents.elem "controllerSacrifices.subject" = false)
 (actorLinks : ∀ r ∈ rs, ∀ shape, r.payload = .actor shape →
      ∀ value ∈ shape.refIds, value ≠ fresh + 2) :
 (dropActorFrame (stepAction (fuel + 10) .postEvent .obj (prepared fresh rs)
   (sacrifice (.laterMention "controllerSacrifices.subject" nestedSubject))).1).referents =
 (dropActorFrame (leaveOwnScope (applyInPlaceStamp (movedOut fresh rs)
   [fresh + 5] (some (.action "sacrifice"))))).referents := by
 change (dropActorFrame (stepAction (fuel + 10) .postEvent .obj (prepared fresh rs)
   (.ownScope (some (.action "sacrifice")) (.enact (.action "sacrifice") moveBody none none))).1).referents =
     (dropActorFrame (leaveOwnScope (applyInPlaceStamp (movedOut fresh rs)
       [fresh + 5] (some (.action "sacrifice"))))).referents
 rw [show fuel + 10 = (fuel + 9)+1 by omega, ownScopeStep]
 rw [show fuel + 9 = (fuel + 8)+1 by omega, sacrificeEnactStep]
 rw [sacrificeEnactProjected]
 rw [show fuel + 8 = (fuel + 7)+1 by omega, moveBodyStep (fuel + 7) _ (by rfl)]
 rw [show fuel + 7 = (fuel + 4)+3 by omega, argumentResult (fuel + 4) fresh rs older noNames]
 dsimp only
 change (dropActorFrame (leaveOwnScope (applyInPlaceStamp (movedOut fresh rs)
   (List.filterMap (fun x : ReferentId × GramRole => if x.2 == .obj then some x.1 else none)
     (if (movedOut fresh rs).currentClause == 1 then
       ((movedOut fresh rs).pendingRoles.take ((movedOut fresh rs).pendingRoles.length - 0))
      else (movedOut fresh rs).pendingRoles)) (some (.action "sacrifice"))))).referents =
     (dropActorFrame (leaveOwnScope (applyInPlaceStamp (movedOut fresh rs)
       [fresh + 5] (some (.action "sacrifice"))))).referents
 rw [(movedOutMetadata fresh rs older noNames actorLinks).1,
     (movedOutMetadata fresh rs older noNames actorLinks).2]
 change (dropActorFrame (leaveOwnScope (applyInPlaceStamp (movedOut fresh rs)
   [fresh + 5] (some (.action "sacrifice"))))).referents =
     (dropActorFrame (leaveOwnScope (applyInPlaceStamp (movedOut fresh rs)
       [fresh + 5] (some (.action "sacrifice"))))).referents
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
      [fresh + 5] (some (.action "sacrifice")))).referents =
    (applyInPlaceStamp (movedOut fresh rs) [fresh + 5] (some (.action "sacrifice"))).referents := by
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
    (some (.core .put)) (some .graveyard) found rfl (by rfl) stack owned selected
  exact leaveStampedOwned (movedOut fresh rs) next outer [fresh + 5]
    (some (.action "sacrifice")) hstack howned

private theorem actualFullTail (fuel fresh : Nat) (rs : List Referent)
 (older : ∀ r ∈ rs, r.id < fresh)
    (noNames : ∀ r ∈ rs, r.idents.elem "controllerSacrifices.subject" = false)
 (actorLinks : ∀ r ∈ rs, ∀ shape, r.payload = .actor shape →
      ∀ value ∈ shape.refIds, value ≠ fresh + 2) :
 (dropActorFrame (stepAction (fuel + 10) .postEvent .obj (prepared fresh rs)
   (sacrifice (.laterMention "controllerSacrifices.subject" nestedSubject))).1).referents =
 afterPrefix fresh++rs := by
 rw [actualTailReduction fuel fresh rs older noNames actorLinks]
 change dropActorFrame.dropFirst (leaveOwnScope (applyInPlaceStamp (movedOut fresh rs)
   [fresh + 5] (some (.action "sacrifice")))).referents = _
 rw [actualMovedScopeClosure fresh rs older noNames]
 exact movedOutStampedReferents fresh rs older noNames actorLinks

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
  let framed := selected.push
    { det := .the, plur := .one, payload := .actor shape } .other false
  let (after, _) := stepAction fuel .postEvent .obj framed body
  let out := dropActorFrame after
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

private theorem controllerSourceAndBody (fuel : Nat) (drs : DRS) :
    let tracked := { drs with
      outerReferentIds := drs.referents.map (·.id) :: drs.outerReferentIds }
    let source := selectedContext tracked
    let frame : ActorShape := {
      kind := .player, isYou := false, ascribable := false,
      refIds := [drs.nextId + 3] }
    let prepared := source.push
      { det := .the, plur := .one, payload := .actor frame } .other false
    (stepAction (fuel + 11) .postEvent .obj drs
      (controllerSacrifices nestedSubject)).1.referents =
      (dropActorFrame (stepAction (fuel + 10) .postEvent .obj prepared
        (sacrifice (.laterMention "controllerSacrifices.subject" nestedSubject))).1).referents := by
  dsimp only
  change (stepAction (fuel + 11) .postEvent .obj drs
    (.instruct (controllerOf (.firstMention "controllerSacrifices.subject" nestedSubject))
      (sacrifice (.laterMention "controllerSacrifices.subject" nestedSubject)))).1.referents = _
  rw [show fuel + 11 = (fuel + 10) + 1 by omega, controllerInstructStep]
  unfold controllerInstructState
  simp only [sourceCanonical, sourceIds, List.filterMap_cons, List.filterMap_nil,
    selectedController, List.foldl_cons, List.foldl_nil, controllerRecord]
  rfl

/-- The complete action moves only its selected new creature and preserves every outer
referent, including unrelated named referents and actor frames without a link to the newly
selected target. -/
theorem nestedDescriptionMovesOnlyItsOwnReferent (rs : List Referent) (fresh : ReferentId)
    (older : ∀ ref ∈ rs, ref.id < fresh)
    (nameFresh : ∀ ref ∈ rs, ref.idents.elem "controllerSacrifices.subject" = false)
    (actorLinks : ∀ ref ∈ rs, ∀ shape, ref.payload = .actor shape →
      ∀ linked ∈ shape.refIds, linked ≠ fresh + 2) :
    (stepAction 64 .postEvent .obj { referents := rs, nextId := fresh }
      nestedAction).1.referents = afterPrefix fresh ++ rs := by
  change (stepAction 64 .postEvent .obj { referents := rs, nextId := fresh }
    (controllerSacrifices nestedSubject)).1.referents = _
  rw [controllerSourceAndBody 53]
  change (dropActorFrame (stepAction 63 .postEvent .obj (prepared fresh rs)
    (sacrifice (.laterMention "controllerSacrifices.subject" nestedSubject))).1).referents = _
  exact actualFullTail 53 fresh rs older nameFresh actorLinks

end Semantics.Proofs.ControllerSacrifice

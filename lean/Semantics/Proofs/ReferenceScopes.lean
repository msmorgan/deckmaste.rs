import Semantics.Definitions

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.ReferenceScopes

private def creatureBinding : Referent :=
  { det := .target, plur := .one,
    payload := .object [.creature] (some .battlefield) none none (some 1) }
private def artifactBinding : Referent :=
  { det := .target, plur := .one,
    payload := .object [.artifact] (some .battlefield) none none (some 1) }
private def playerBinding : Referent :=
  { det := .the, plur := .one, payload := .player false }
private def outer : DRS := DRS.ofList [artifactBinding, letterB .x]
private def creatureX : NounPhrase :=
  target (.and [isCreature, .compare [.stat .power] .eq (.letter .x)])
private def artifactX : NounPhrase :=
  target (.and [isArtifact, .compare [.stat .manaValue] .eq (.letter .x)])
private def powerAfter (drs : DRS) : DRS :=
  (stepAction 64 .postEvent .obj drs
    (.establish (Templates.modification (target isCreature) .power (.up (.letter .x))) none)).1

private theorem olderClauseMaximum (rs : List Referent) (clause : Nat)
    (older : ∀ r ∈ rs, r.clause < clause) :
    rs.foldl (fun acc r => Nat.max acc r.clause) clause = clause := by
  induction rs with
  | nil => rfl
  | cons r rs ih =>
    have hr := older r (by simp)
    have ht : ∀ r ∈ rs, r.clause < clause := by
      intro r h
      exact older r (by simp [h])
    simp only [List.foldl_cons, Nat.max_eq_left (Nat.le_of_lt hr)]
    exact ih ht

private theorem olderClauseFilter (rs : List Referent) (clause : Nat)
    (older : ∀ r ∈ rs, r.clause < clause) :
    rs.filter (fun r => r.clause == clause) = [] := by
  induction rs with
  | nil => rfl
  | cons r rs ih =>
    have hr := older r (by simp)
    have ht : ∀ r ∈ rs, r.clause < clause := by
      intro r h
      exact older r (by simp [h])
    simp [Nat.ne_of_lt hr, ih ht]

/-- A newly introduced subject wins over every referent from an earlier clause. -/
theorem ownStatIdentityStableForEveryOuterContext (rs : List Referent) (clause id : Nat)
    (older : ∀ r ∈ rs, r.clause < clause) :
    let subject := { creatureBinding with clause := clause, role := .subj, id := id }
    resolveCandidates .bare .one { referents := subject :: rs } = [subject] := by
  have filteredOlder : ∀ (p : Referent → Bool), ∀ r ∈ rs.filter p, r.clause < clause := by
    intro p r h
    exact older r (List.mem_filter.mp h).1
  let subject := { creatureBinding with clause := clause, role := .subj, id := id }
  change resolveCandidates .bare .one { referents := subject :: rs } = [subject]
  have visible : (DRS.visibleReferents { referents := subject :: rs }) = subject :: rs := by
    apply List.filter_eq_self.mpr
    intro r _
    rfl
  have eligible : reaches .bare .one subject = true := by rfl
  unfold resolveCandidates
  dsimp only
  rw [visible]
  simp only [Bool.false_eq_true, ↓reduceIte,
    show ((none : Option Zone) == some .battlefield) = false from rfl,
    Bool.false_and, Bool.or_false, show subject.linked = false from rfl, Bool.not_false,
    Bool.true_and, List.filter_cons, eligible]
  simp only [List.foldl_cons, show subject.clause = clause from rfl, Nat.zero_max,
    olderClauseMaximum _ _ (filteredOlder _), BEq.rfl, ↓reduceIte,
    olderClauseFilter _ _ (filteredOlder _), List.foldl_nil]
  rfl

/-- The same universally quantified subject read passes the actual noun-phrase checker. -/
theorem ownStatStableForEveryOuterContext (rs : List Referent) (clause id : Nat)
    (older : ∀ r ∈ rs, r.clause < clause) :
    let subject := { creatureBinding with clause := clause, role := .subj, id := id }
    NounPhrase.checkAt (some .object) { referents := subject :: rs } it = [] := by
  let subject := { creatureBinding with clause := clause, role := .subj, id := id }
  let drs : DRS := { referents := subject :: rs }
  change (let winners := resolveCandidates .bare .one drs
    (match winners with
    | [w] => (drs.recordMention w.id .other false true, [w.id], ([] : List Refusal))
    | _ => (drs, [], [Refusal.anaphor .bare .one winners.length])).2.2) ++ [] = []
  rw [ownStatIdentityStableForEveryOuterContext rs clause id older]
  rfl

-- P16: a true tie in grammatical role and clause remains a refusal.
theorem ordinaryItRemainsAmbiguous :
    NounPhrase.checkAt (some .object) (DRS.ofList [creatureBinding, artifactBinding]) it =
      [.anaphor .bare .one 2] := by decide

theorem ownStatWithoutOuter :
    NounPhrase.checkAt (some .object) (powerAfter (DRS.ofList [])) it = [] := by decide

theorem ownStatWithOuter :
    NounPhrase.checkAt (some .object) (powerAfter outer) it = [] := by decide

theorem ownStatKeepsCreature :
    (firstReach .bare .one (powerAfter outer)).map (·.ty) = some [.creature] ∧
      (firstReach .bare .one (powerAfter outer)).map (·.id) = some 2 := by decide

private def sharedAfter (drs : DRS) : DRS :=
  (stepNounPhrase 64 drs .subj true creatureX).1
private def previousAfter (drs : DRS) : DRS :=
  (stepAction 64 .postEvent .obj drs
    (.putCounters (.lit 1) (.printed p1p1Counter) creatureX .one false)).1
private def conditionAfter (drs : DRS) : DRS :=
  (stepCondition 64 drs (.matches creatureX isCreature)).1

theorem sharedSubjectWithoutOuter :
    NounPhrase.checkAt (some .object) (sharedAfter (DRS.ofList [])) it = [] := by decide

theorem sharedSubjectWithOuter :
    NounPhrase.checkAt (some .object) (sharedAfter outer) it = [] := by decide

theorem sharedSubjectKeepsCreature :
    (firstReach .bare .one (sharedAfter outer)).map (·.ty) = some [.creature] ∧
      (firstReach .bare .one (sharedAfter outer)).map (·.id) = some 2 := by decide

theorem previousActionWithoutOuter :
    NounPhrase.checkAt (some .object) (previousAfter (DRS.ofList [])) it = [] := by decide

theorem previousActionWithOuter :
    NounPhrase.checkAt (some .object) (previousAfter outer) it = [] := by decide

theorem previousActionKeepsCreature :
    (firstReach .bare .one (previousAfter outer)).map (·.ty) = some [.creature] ∧
      (firstReach .bare .one (previousAfter outer)).map (·.id) = some 2 := by decide

theorem conditionSubjectWithoutOuter :
    NounPhrase.checkAt (some .object) (conditionAfter (DRS.ofList [])) it = [] := by decide

theorem conditionSubjectWithOuter :
    NounPhrase.checkAt (some .object) (conditionAfter outer) it = [] := by decide

theorem conditionSubjectKeepsCreature :
    (firstReach .bare .one (conditionAfter outer)).map (·.ty) = some [.creature] ∧
      (firstReach .bare .one (conditionAfter outer)).map (·.id) = some 2 := by decide

private def playerX : NounPhrase :=
  each (.and [.anyPlayer, .compare [.playerStat .lifeTotal] .eq (.letter .x)])
private def libraryCards : NounPhrase := .slice .top (.lit 2) (libraryOf playerX)
private def libraryAfter (drs : DRS) : DRS :=
  (stepNounPhrase 64 drs .obj true (.firstMention "scope.library" libraryCards)).1
private def libraryRead : NounPhrase := .laterMention "scope.library" libraryCards

theorem librarySliceWithoutOuter :
    NounPhrase.checkAt (some .object) (libraryAfter (DRS.ofList [])) libraryRead = [] := by
  decide

theorem librarySliceWithOuter :
    NounPhrase.checkAt (some .object) (libraryAfter outer) libraryRead = [] := by decide

theorem librarySliceKeepsLibraryCarrier :
    let (after, ids, errors) := stepNounPhrase 64 (libraryAfter outer) .other false libraryRead
    errors = [] ∧ ids.length = 1 ∧
      (ids.filterMap after.findById?).map (·.zone) = [some .library] := by decide

/-- Without a named performer, two players do not determine whose choice this is. -/
theorem implicitChooserDoesNotIgnoreAnotherPlayer :
    NounPhrase.checkAt (some .object) (DRS.ofList [playerBinding, playerBinding])
      (aTheirChoice isCreature) = [.chooserInScope 2] := by decide

/-- An explicit performer determines the chooser despite other players in discourse. -/
theorem explicitChooserWinsOverOuterPlayers :
    Action.check (DRS.ofList [playerBinding, playerBinding])
      (instruct (target .anyPlayer) (choose (aTheirChoice isCreature))) = [] := by decide

theorem agentWithoutOuter :
    Action.check (DRS.ofList [])
      (instruct playerX (instruct .actor (gainLife (.lit 1)))) = [] := by decide

theorem agentWithOuter :
    Action.check (DRS.ofList [playerBinding, letterB .x])
      (instruct playerX (instruct .actor (gainLife (.lit 1)))) = [] := by decide

theorem scopedMovePreservesTheOuterBindings :
    let before := nextClause (powerAfter outer)
    let after := (stepAction 64 .postEvent .obj before (exile it)).1
    Action.check before (exile it) = [] ∧
      (after.findById? 0) = (before.findById? 0) ∧
      (after.findById? 1) = (before.findById? 1) ∧
      (after.findById? 2) = none ∧
      (firstReach (.word .card) .one (nextClause after)).map (fun r => (r.ty, r.zone)) =
        some ([.creature], some .exile) := by decide

theorem amountLetterChangesIntroductionCount :
    (sharedAfter (DRS.ofList [])).referents.length = 2 ∧
      (sharedAfter outer).referents.length = outer.referents.length + 1 := by decide

/-- A second subject in the same clause remains a true tie; ordinary pronouns are not captures. -/
theorem ownStatSameClauseSubjectsStillTie :
    let before := DRS.ofList [{ creatureBinding with role := .subj }]
    NounPhrase.checkAt (some .object) (powerAfter before) it =
      [.anaphor .bare .one 2] := by decide

theorem attachWithoutOuterLetter :
    Action.check (DRS.ofList [creatureBinding]) (attachToIt artifactX) = [] := by decide

theorem attachWithOuterLetter :
    Action.check (DRS.ofList [creatureBinding, playerBinding, letterB .x])
      (attachToIt artifactX) = [] := by decide

-- P15: the new attachment's clause-mate exclusion preserves the outer tie.
theorem attachKeepsOuterAmbiguity :
    Action.check (DRS.ofList [creatureBinding, creatureBinding, letterB .x])
      (attachToIt artifactX) = [.anaphor .bare .one 2] := by decide

theorem blockWithoutOuterLetter :
    Action.check (DRS.ofList [creatureBinding])
      (establish (must creatureX (block it)) (some untilEndOfTurn)) = [] := by decide

theorem blockWithOuterLetter :
    Action.check (DRS.ofList [creatureBinding, playerBinding, letterB .x])
      (establish (must creatureX (block it)) (some untilEndOfTurn)) = [] := by decide

theorem damageOwnPowerWithoutOuter :
    Action.check (DRS.ofList []) (dealDamageOwnPower creatureX (target isCreature)) = [] := by decide

theorem damageOwnPowerWithOuter :
    Action.check outer (dealDamageOwnPower creatureX (target isCreature)) = [] := by decide

private def playerLoop : Action :=
  .doForEach (each .anyPlayer)
    (choose (target (.and [isPermanent, .hasPossessor .controller they])))

theorem loopPlayerRetainsPlayerKind :
    Action.check (DRS.ofList []) playerLoop = [] := by decide

theorem loopExportsPluralGroupWithoutSingularMember :
    let after := (stepAction 64 .postEvent .obj (DRS.ofList []) playerLoop).1
    (resolveCandidates (.word .player) .many (nextClause after)).length = 1 ∧
      resolveCandidates (.word .player) .one (nextClause after) = [] := by decide

theorem optionalBranchesDoNotExportCreatedObjects :
    let branch := may (gainLife (.lit 1))
      (some (create (.lit 1) (.written (creatureToken 1 1 [.white] []))))
      (some (create (.lit 2) (.written (creatureToken 1 1 [.white] []))))
    let after := (stepAction 64 .postEvent .obj (DRS.ofList []) branch).1
    resolveCandidates (.word (.type .creature)) .one (nextClause after) = [] := by decide

theorem sourceTypeRefinesOneIdentity :
    let after := (stepNounPhrase 64 (DRS.ofList []) .subj true thisCreature).1
    after.referents.map (fun ref => (ref.id, ref.ty)) = [(0, [.creature])] ∧
      after.nextId = 1 := by decide

theorem negatedSourcePropertyKeepsSourceWithoutPositiveRefinement :
    let before := (stepAction 64 .postEvent .obj (DRS.ofList [])
      (destroy (target isCreature))).1
    let after := (stepCondition 64 (nextClause before)
      (.not (.matches thisEnchantment isCreature))).1
    let selected := resolveCandidates .bare .one (nextClause after)
    selected.map (·.ty) = [[.enchantment]] ∧
      selected.map (·.id) = [after.nextId - 1] := by decide

theorem optionalBodyReadsItsEnclosingPlayer :
    Action.check (DRS.ofList [])
      (instruct (target (.isOpponentOf .you))
        (may (instruct they (gainLife (.lit 1))))) = [] := by
  decide

private def powerSubject : Referent := {
  det := .target, plur := .one,
  payload := .object [.creature] (some .battlefield) none none none }

private theorem actualPowerPipeline (n : Nat) (drs : DRS) :
    ((stepOps (n + 6)).inst .postEvent .obj drs
      (.establish (Templates.modification (target isCreature) .power (.up (.letter .x))) none)).1 =
      let selected := ({ drs with relativeKind := some .object } : DRS).push powerSubject .subj true
      (introducedLetters .x selected).foldl
        (fun d ref => d.push ref .other false) selected := by rfl

private theorem powerSubjectWins (rs : List Referent) (clause id : Nat)
    (older : ∀ r ∈ rs, r.clause < clause) :
    let subject := { powerSubject with clause := clause, role := .subj, id := id }
    resolveCandidates .bare .one { referents := subject :: rs } = [subject] := by
  have filteredOlder : ∀ (p : Referent → Bool), ∀ r ∈ rs.filter p, r.clause < clause := by
    intro p r h
    exact older r (List.mem_filter.mp h).1
  let subject := { powerSubject with clause := clause, role := .subj, id := id }
  change resolveCandidates .bare .one { referents := subject :: rs } = [subject]
  have visible : (DRS.visibleReferents { referents := subject :: rs }) = subject :: rs := by
    apply List.filter_eq_self.mpr
    intro r _
    rfl
  have eligible : reaches .bare .one subject = true := by rfl
  unfold resolveCandidates
  dsimp only
  rw [visible]
  simp only [Bool.false_eq_true, ↓reduceIte,
    show ((none : Option Zone) == some .battlefield) = false from rfl,
    Bool.false_and, Bool.or_false, show subject.linked = false from rfl, Bool.not_false,
    Bool.true_and, List.filter_cons, eligible]
  simp only [List.foldl_cons, show subject.clause = clause from rfl, Nat.zero_max,
    olderClauseMaximum _ _ (filteredOlder _), BEq.rfl, ↓reduceIte,
    olderClauseFilter _ _ (filteredOlder _), List.foldl_nil]
  rfl


/-- The actual modification pipeline selects its own fresh target in every earlier outer
context, whether X was already bound or introduces a same-clause letter. -/
theorem ownStatActionIdentityStableForEveryOuterContext (rs : List Referent) (clause id : Nat)
    (older : ∀ r ∈ rs, r.clause < clause) :
    resolveCandidates .bare .one
      (powerAfter { referents := rs, currentClause := clause, nextId := id }) =
      [{ powerSubject with clause := clause, role := .subj, id := id }] := by
  change resolveCandidates .bare .one
    (((stepOps (58 + 6)).inst .postEvent .obj
      { referents := rs, currentClause := clause, nextId := id }
      (.establish (Templates.modification (target isCreature) .power (.up (.letter .x))) none)).1) = _
  rw [actualPowerPipeline]
  dsimp only
  unfold introducedLetters
  split <;> change resolveCandidates .bare .one
    { referents := { powerSubject with clause := clause, role := .subj, id := id } :: rs } = _
  all_goals exact powerSubjectWins rs clause id older

/-- The ordinary pronoun checker accepts the result of the actual modification pipeline,
not merely a manually constructed subject context. -/
theorem ownStatActionStableForEveryOuterContext (rs : List Referent) (clause id : Nat)
    (older : ∀ r ∈ rs, r.clause < clause) :
    NounPhrase.checkAt (some .object)
      (powerAfter { referents := rs, currentClause := clause, nextId := id }) it = [] := by
  let after := powerAfter { referents := rs, currentClause := clause, nextId := id }
  change (let winners := resolveCandidates .bare .one after
    (match winners with
    | [w] => (after.recordMention w.id .other false true, [w.id], ([] : List Refusal))
    | _ => (after, [], [Refusal.anaphor .bare .one winners.length])).2.2) ++ [] = []
  rw [ownStatActionIdentityStableForEveryOuterContext rs clause id older]
  rfl

end Semantics.Proofs.ReferenceScopes

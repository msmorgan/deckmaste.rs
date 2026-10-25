import Semantics.Check.AbilityRules

namespace Semantics.Proofs.ComparisonBox

private def outerArtifact : DRS := DRS.ofList [{
  det := .target, plur := .one,
  payload := .object [.artifact] (some .battlefield) none none (some 1) }]

private def ownPower : Amount := .statOf (.stat .power) (.pro .bare .one)

private def comparison : Predicate :=
  .compareOver (.hasType .creature) ownPower .atLeast 1

/-- The subordinate comparison candidate outranks an enclosing subject from the older clause. -/
theorem comparisonReadsItsCandidateRatherThanTheOuterArtifact :
    let member := bindFor .the .one .object (.hasType .creature)
    let localContext := (nextClause outerArtifact).push member .other false
    Predicate.checkAt .object outerArtifact comparison = [] ∧
      (resolveCandidates .bare .one localContext).map (·.id) = [1] ∧
      (resolveCandidates .bare .one localContext).map (·.ty) = [[.creature]] ∧
      (localContext.findById? 1).map (·.clause) = some (outerArtifact.currentClause + 1) := by decide

/-- The candidate closes before the bound and does not escape the comparison box. -/
theorem comparisonCandidateDoesNotEscapeIntoTheBoundOrEnclosingContext :
    let out := (stepPredicate 64 outerArtifact comparison).1
    (out.findById? 1) = none ∧
      (out.findById? 0).map (·.payload) = (outerArtifact.findById? 0).map (·.payload) ∧
      out.currentClause = outerArtifact.currentClause ∧
      countOnes .object out = 1 ∧
      Predicate.checkAt .object outerArtifact
        (.compareOver (.hasType .creature) ownPower .atLeast
          (.statOf (.stat .power) (.pro (.word (.type .creature)) .one))) =
        [.anaphor (.word (.type .creature)) .one 0] := by decide

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

/-- With no direct-argument mention buffered in the current clause, `D′` ranks every referent by
the clause and role it was introduced or last foregrounded in. -/
private theorem rankingWithoutPending (drs : DRS) (h : drs.pendingRoles = []) :
    drs.rankingClause = (·.clause) ∧ drs.rankingRole = (·.role.rank) := by
  constructor <;> funext ref <;> simp [DRS.rankingClause, DRS.rankingRole, DRS.isClauseMate, h]

private def candidate (clause id : Nat) : Referent := {
  det := .the, plur := .one, clause, id, role := .other,
  payload := .object [.creature] (some .battlefield) none none none }

private theorem candidateWins (rs : List Referent) (clause id : Nat)
    (older : ∀ r ∈ rs, r.clause < clause) :
    resolveCandidates .bare .one
      { referents := candidate clause id :: rs, characteristicRead := true } =
        [candidate clause id] := by
  have filteredOlder : ∀ (p : Referent → Bool), ∀ r ∈ rs.filter p, r.clause < clause := by
    intro p r h
    exact older r (List.mem_filter.mp h).1
  have visible : (DRS.visibleReferents {
      referents := candidate clause id :: rs, characteristicRead := true }) =
      candidate clause id :: rs := by
    apply List.filter_eq_self.mpr
    intro r _
    rfl
  have eligible : reachesCharacteristic .bare .one (candidate clause id) = true := by rfl
  unfold resolveCandidates
  dsimp only
  rw [visible]
  obtain ⟨byClause, byRole⟩ := rankingWithoutPending
    { referents := candidate clause id :: rs, characteristicRead := true } rfl
  simp only [byClause, byRole]
  simp only [Bool.false_eq_true, ↓reduceIte,
    Bool.false_and, Bool.or_false, show (candidate clause id).linked = false from rfl,
    Bool.not_false, Bool.true_and, List.filter_cons, eligible,
    show Plurality.one.isOne = true from rfl]
  simp only [List.foldl_cons, show (candidate clause id).clause = clause from rfl, Nat.zero_max,
    olderClauseMaximum _ _ (filteredOlder _), BEq.rfl, ↓reduceIte,
    olderClauseFilter _ _ (filteredOlder _), List.foldl_nil]
  rfl

private def comparisonContext (drs : DRS) : DRS :=
  (nextClause drs).push (bindFor .the .one .object (.hasType .creature)) .other false

private theorem comparisonErrors (n : Nat) (drs : DRS) :
    ((stepOps (n + 4)).pred drs
      (.compareOver (.hasType .creature) (.statOf (.stat .power) (.pro .bare .one))
        .atLeast 1)).2 =
    ((stepOps (n + 3)).amt (comparisonContext drs)
      (.statOf (.stat .power) (.pro .bare .one))).2 := by
  change [] ++ [] ++ ((stepOps (n + 3)).amt (comparisonContext drs)
    (.statOf (.stat .power) (.pro .bare .one))).2 ++ [] = _
  simp only [List.nil_append, List.append_nil]

private theorem proStep (n : Nat) (drs : DRS) :
    (stepOps (n + 1)).np drs .other false (.pro .bare .one) =
      let winners := resolveCandidates .bare .one drs
      match winners with
      | [w] => (drs.recordMention w.id .other false true, [w.id], [])
      | _ => (drs, [], [.anaphor .bare .one winners.length]) := by rfl

private theorem powerErrors (n : Nat) (drs : DRS) :
    ((stepOps (n + 2)).amt drs (.statOf (.stat .power) (.pro .bare .one))).2 =
      let selected := (stepOps (n + 1)).np { drs with characteristicRead := true }
        .other false (.pro .bare .one)
      let refs := selected.2.1.filterMap selected.1.findById?
      selected.2.2 ++ kindCheck .object ((refs.head?).map (·.kind)) ++
        refuse (!(refs.any (fun r => r.plur != .one))) .singular := by rfl

private theorem powerOnCandidate (n : Nat) (rs : List Referent) (clause id : Nat)
    (current next : Nat) (kind : Option Kind)
    (older : ∀ r ∈ rs, r.clause < clause) :
    ((stepOps (n + 2)).amt {
      referents := candidate clause id :: rs, currentClause := current,
      nextId := next, relativeKind := kind }
      (.statOf (.stat .power) (.pro .bare .one))).2 = [] := by
  have winners := candidateWins rs clause id older
  change resolveCandidates .bare .one {
    referents := candidate clause id :: rs, characteristicRead := true,
    currentClause := current, nextId := next, relativeKind := kind } = _ at winners
  rw [powerErrors, proStep, winners]
  simp [DRS.recordMention, DRS.findById?, candidate]
  exact ⟨rfl, rfl⟩

/-- The comparison checker succeeds in every outer context whose referents belong to the
current or an earlier clause. The subordinate candidate, rather than any outer object,
supplies its own power. -/
theorem comparisonScopeIgnoresEveryEarlierOuterContext (rs : List Referent) (c id : Nat)
    (older : ∀ r ∈ rs, r.clause ≤ c) :
    Predicate.checkAt .object { referents := rs, currentClause := c, nextId := id }
      (.compareOver (.hasType .creature) (.statOf (.stat .power) (.pro .bare .one))
        .atLeast 1) = [] := by
  change (stepPredicate 64 {
      referents := rs, currentClause := c, nextId := id, relativeKind := some .object }
    (.compareOver (.hasType .creature) (.statOf (.stat .power) (.pro .bare .one))
      .atLeast 1)).2 ++ [] = []
  rw [stepPredicate, comparisonErrors 60]
  let normalized := rs.map (fun r => { r with pronounResolved := false })
  have normalizedOlder : ∀ r ∈ normalized, r.clause < c + 1 := by
    intro r member
    obtain ⟨original, mem, rfl⟩ := List.mem_map.mp member
    exact Nat.lt_succ_of_le (older original mem)
  have contextEq : comparisonContext {
      referents := rs, currentClause := c, nextId := id, relativeKind := some .object } = {
      referents := candidate (c + 1) id :: normalized,
      currentClause := c + 1, nextId := id + 1, relativeKind := some .object } := by rfl
  rw [contextEq, powerOnCandidate 61 normalized (c + 1) id (c + 1) (id + 1)
    (some .object) normalizedOlder]
  rfl

end Semantics.Proofs.ComparisonBox

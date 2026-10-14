import Semantics.Check.Words

namespace Semantics.Proofs.DrtResolution

private def creature (id clause : Nat) (role : GramRole) : Referent :=
  { id, clause, role, det := .target, plur := .one,
    payload := .object [.creature] (some .battlefield) none none (some 1) none }

private def winners (reach : Reach) (drs : DRS) (direct : Bool := false) : List ReferentId :=
  (resolveCandidates reach .one drs direct).map (·.id)

theorem nearestClauseBeatsOlderSubject :
    winners .bare { referents := [creature 1 1 .subj, creature 2 2 .other] } = [2] := by decide

theorem subjectBeatsObject :
    winners .bare { referents := [creature 1 2 .subj, creature 2 2 .obj] } = [1] := by decide

theorem objectBeatsOblique :
    winners .bare { referents := [creature 1 2 .obj, creature 2 2 .other] } = [1] := by decide

theorem barePronounRetainsTrueTie :
    winners .bare { referents := [creature 1 2 .obj, creature 2 2 .obj] } = [1, 2] := by decide

theorem clauseMateIsExcludedInDirectSlot :
    winners .bare
      { referents := [creature 1 1 .obj, creature 2 2 .subj],
        currentClause := 2, pendingRoles := [(2, .subj)] } true = [1] := by decide

theorem possessorMayReadMatrixSubject :
    winners .bare
      { referents := [creature 1 1 .obj, creature 2 2 .subj],
        currentClause := 2, pendingRoles := [(2, .subj)] } false = [2] := by decide

theorem maskedReferentIsInaccessible :
    winners .bare
      { referents := [creature 1 1 .obj, creature 2 2 .subj],
        scopeStack := [{ preEntryIds := [2], maskedIds := [2], callerClause := 1 }] } = [1] := by
  decide

theorem singularPronounExcludesPluralReferent :
    winners .bare { referents := [creature 1 1 .obj, { creature 2 2 .subj with plur := .many }] } =
      [1] := by decide

theorem creatureDemonstrativeExcludesGraveyardCard :
    winners (.word (.type .creature))
      { referents := [creature 1 1 .obj,
          { creature 2 2 .subj with
            payload := .object [.creature] (some .graveyard) none none (some 1) none }] } =
      [1] := by decide

theorem demonstrativeRecencyIgnoresRoleRanking :
    winners (.word (.type .creature))
      { referents := [creature 1 2 .subj, creature 2 1 .other] } = [2] := by decide

theorem demonstrativeExcludesClauseMateBeforeRecency :
    winners (.word (.type .creature))
      { referents := [creature 1 1 .obj, creature 2 2 .subj],
          currentClause := 2, pendingRoles := [(2, .subj)] } true = [1] := by decide

theorem demonstrativeExcludesPronounReferentBeforeRecency :
    winners (.word (.type .creature))
      { referents := [creature 1 1 .obj, { creature 2 2 .subj with pronounResolved := true }],
        currentClause := 2 } = [1] := by decide

theorem demonstrativeRefusesWhenEveryCandidateExcluded :
    winners (.word (.type .creature))
      { referents := [{ creature 1 1 .obj with pronounResolved := true }, creature 2 2 .subj],
          currentClause := 2, pendingRoles := [(2, .subj)] } true = [] := by decide

theorem reflexiveSelectsClauseMateSubject :
    (resolveReflexive .one
      { referents := [creature 1 1 .obj, creature 2 2 .subj],
          currentClause := 2, pendingRoles := [(2, .subj)] }).map (·.id) = [2] := by decide

theorem reflexiveRefusesWithoutClauseMateSubject :
    (resolveReflexive .one
      { referents := [creature 1 1 .obj], currentClause := 2 }).map (·.id) = [] := by decide

end Semantics.Proofs.DrtResolution

import Semantics.Check.Words

namespace Semantics.Proofs.DrtResolution

private def isCreature (id clause : Nat) (role : GramRole) : Referent :=
  { id, clause, role, det := .target, plur := .one,
    payload := .object [.creature] (some .battlefield) none none (some 1) }

private def winners (reach : Reach) (drs : DRS) (direct : Bool := false) : List ReferentId :=
  (resolveCandidates reach .one drs direct).map (·.id)

theorem nearestClauseBeatsOlderSubject :
    winners .bare { referents := [isCreature 1 1 .subj, isCreature 2 2 .other] } = [2] := by decide

theorem subjectBeatsObject :
    winners .bare { referents := [isCreature 1 2 .subj, isCreature 2 2 .obj] } = [1] := by decide

theorem objectBeatsOblique :
    winners .bare { referents := [isCreature 1 2 .obj, isCreature 2 2 .other] } = [1] := by decide

theorem barePronounRetainsTrueTie :
    winners .bare { referents := [isCreature 1 2 .obj, isCreature 2 2 .obj] } = [1, 2] := by decide

theorem clauseMateIsExcludedInDirectSlot :
    winners .bare
      { referents := [isCreature 1 1 .obj, isCreature 2 2 .subj],
        currentClause := 2, pendingRoles := [(2, .subj)] } true = [1] := by decide

theorem possessorMayReadMatrixSubject :
    winners .bare
      { referents := [isCreature 1 1 .obj, isCreature 2 2 .subj],
        currentClause := 2, pendingRoles := [(2, .subj)] } false = [2] := by decide

theorem maskedReferentIsInaccessible :
    winners .bare
      { referents := [isCreature 1 1 .obj, isCreature 2 2 .subj],
        scopeStack := [{ preEntryIds := [2], maskedIds := [2], callerClause := 1 }] } = [1] := by
  decide

theorem singularPronounExcludesPluralReferent :
    winners .bare { referents := [isCreature 1 1 .obj, { isCreature 2 2 .subj with plur := .many }] } =
      [1] := by decide

theorem creatureDemonstrativeExcludesGraveyardCard :
    winners (.word (.type .creature))
      { referents := [isCreature 1 1 .obj,
          { isCreature 2 2 .subj with
            payload := .object [.creature] (some .graveyard) none none (some 1) }] } =
      [1] := by decide

theorem demonstrativeRecencyIgnoresRoleRanking :
    winners (.word (.type .creature))
      { referents := [isCreature 1 2 .subj, isCreature 2 1 .other] } = [2] := by decide

theorem demonstrativeExcludesClauseMateBeforeRecency :
    winners (.word (.type .creature))
      { referents := [isCreature 1 1 .obj, isCreature 2 2 .subj],
          currentClause := 2, pendingRoles := [(2, .subj)] } true = [1] := by decide

theorem demonstrativeExcludesPronounReferentBeforeRecency :
    winners (.word (.type .creature))
      { referents := [isCreature 1 1 .obj, { isCreature 2 2 .subj with pronounResolved := true }],
        currentClause := 2 } = [1] := by decide

theorem demonstrativeRefusesWhenEveryCandidateExcluded :
    winners (.word (.type .creature))
      { referents := [{ isCreature 1 1 .obj with pronounResolved := true }, isCreature 2 2 .subj],
          currentClause := 2, pendingRoles := [(2, .subj)] } true = [] := by decide

theorem reflexiveSelectsClauseMateSubject :
    (resolveReflexive .one
      { referents := [isCreature 1 1 .obj, isCreature 2 2 .subj],
          currentClause := 2, pendingRoles := [(2, .subj)] }).map (·.id) = [2] := by decide

theorem reflexiveRefusesWithoutClauseMateSubject :
    (resolveReflexive .one
      { referents := [isCreature 1 1 .obj], currentClause := 2 }).map (·.id) = [] := by decide

end Semantics.Proofs.DrtResolution

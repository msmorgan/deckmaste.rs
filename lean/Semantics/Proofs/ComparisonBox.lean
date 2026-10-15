import Semantics.Check.AbilityRules

namespace Semantics.Proofs.ComparisonBox

private def outerArtifact : DRS := DRS.ofList [{
  det := .target, plur := .one,
  payload := .object [.artifact] (some .battlefield) none none (some 1) none }]

private def ownPower : Amount := .statOf (.stat .power) (.pro .bare .one)

private def comparison : Predicate :=
  .compareOver (.hasType .creature) ownPower .atLeast (.lit 1)

/-- The subordinate comparison candidate outranks an enclosing subject from the older clause. -/
theorem comparisonReadsItsCandidateRatherThanTheOuterArtifact :
    let member := bindFor outerArtifact .the .one .object (.hasType .creature)
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

end Semantics.Proofs.ComparisonBox

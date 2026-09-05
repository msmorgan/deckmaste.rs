import Semantics.Macros
import Semantics.Check.Card

/-!
# Semantics.Proofs.Tables

Table-integrity pins: not a port of an Idris family, but `decide` theorems asserting what the
checker's label tables assume of themselves — that `actFacts`, `abilityDeedFacts`,
`counterFacts`, `keywordFacts`, `designationTable`, and `subtypeFacts` each name every row once.
A duplicate label would let one row shadow another silently, so each table's existing `distinct*`
validator is asserted here rather than left unchecked.
-/

open Semantics Semantics.Macros

namespace Semantics.Proofs.Tables

theorem actLabelsDistinct : distinctActLabels actFacts = true := by decide

theorem abilityDeedLabelsDistinct : distinctAbilityDeedLabels abilityDeedFacts = true := by decide

theorem counterLabelsDistinct : distinctCounterLabels counterFacts = true := by decide

theorem keywordWordsDistinct : distinctKeywordWords keywordFacts = true := by decide

theorem designationLabelsDistinct : distinctDesignationLabels designationTable = true := by decide

theorem subtypeFactsDistinct : distinctSubtypeFacts subtypeFacts = true := by decide

theorem spaceSculptorConfersEverySector :
    ["alpha sector", "beta sector", "gamma sector"].all
      (fun label => conferralOk label (.byKeyword "SpaceSculptor")) = true := by decide

theorem unlockConfersEitherDoor :
    ["left half unlocked", "right half unlocked"].all
      (fun label => conferralOk label (.byDeed (.core .unlock))) = true := by decide

theorem storiedDoesNotConferCitysBlessing :
    conferralOk "the city's blessing" (.byKeyword "Storied") = false := by decide

theorem unlockDoesNotConferSector :
    conferralOk "alpha sector" (.byDeed (.core .unlock)) = false := by decide

theorem coreFactsIncludeTheirDeclaredConferrals :
    (coreDeedFacts .unlock).confers = ["left half unlocked", "right half unlocked"] := by decide

end Semantics.Proofs.Tables

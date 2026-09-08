import Semantics.Macros
import Semantics.Check.Card

/-!
# Semantics.Proofs.Tables

Table-integrity pins: not a port of an Idris family, but `decide` theorems asserting what the
checker's label tables assume of themselves — that `actFacts`, `abilityDeedFacts`,
`counterFacts`, `keywordFacts`, `designationTable`, and `subtypeFacts` each name every row once.
A duplicate label would let one row shadow another silently, so each table's existing `distinct*`
validator is asserted here rather than left unchecked.

`actFacts` is hand-written in `Check.Words` while the keyword actions it keys are declared in the
registry, so two theorems here also pin the two lists against each other in both directions —
the check the generator used to make of its own overlay.
-/

open Semantics Semantics.Macros

namespace Semantics.Proofs.Tables

theorem actLabelsDistinct : distinctActLabels actFacts = true := by decide

/-- Every keyword action the registry declares [CR#701.1] has a row in the hand-written
`actFacts` table: a declaration added without one is caught here, not left to a lookup that
silently finds nothing. -/
theorem everyDeclaredActionHasFacts :
    keywordActionLabels.all (fun l => (actFacts.find? (·.1 == l)).isSome) = true := by decide

/-- And every `actFacts` row names a keyword action the registry declares, so a row left behind
by a retired declaration is caught too. -/
theorem everyActRowIsDeclared :
    actFacts.all (fun row => keywordActionLabels.contains row.1) = true := by decide

theorem abilityDeedLabelsDistinct : distinctAbilityDeedLabels abilityDeedFacts = true := by decide

theorem counterLabelsDistinct : distinctCounterLabels counterFacts = true := by decide

theorem keywordWordsDistinct : distinctKeywordWords keywordFacts = true := by decide

theorem designationLabelsDistinct : distinctDesignationLabels designationTable = true := by decide

theorem subtypeFactsDistinct : distinctSubtypeFacts subtypeFacts = true := by decide

/-- The enum-valued designation reaches the table under every one of its members, and each is
effectful, so an instruction may confer it. -/
theorem everySectorLabelIsAnEffectfulDesignation :
    ["alpha sector", "beta sector", "gamma sector"].all DesignationLabel.checked = true := by decide

/-- The two door designations are exactly the rows the table marks with a room half. -/
theorem bothDoorLabelsAreDeclaredHalves :
    (designationTable.filter (·.half.isSome)).map (·.label)
      = ["left half unlocked", "right half unlocked"] := by decide

/-- The one designation the table marks non-effectful: a rule confers it, so no instruction
may. -/
theorem aRuleOnlyDesignationIsNotEffectful : DesignationLabel.checked "commander" = false := by
  decide

/-- A label outside the declared enum is no designation at all, so nothing may confer it. -/
theorem anUndeclaredSectorIsNotADesignation :
    DesignationLabel.checked "delta sector" = false := by decide

end Semantics.Proofs.Tables

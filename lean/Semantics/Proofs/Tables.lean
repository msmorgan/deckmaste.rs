import Semantics.Macros
import Semantics.Check.Card

/-!
# Semantics.Proofs.Tables

Table-integrity pins: not a port of an Idris family, but `decide` theorems asserting what the
checker's label tables assume of themselves — that `actFacts`, `counterFacts`, `keywordFacts`,
`designationTable`, and `subtypeFacts` each name every row once. A duplicate label would let one
row shadow another silently, so each table's existing `distinct*` validator is asserted here
rather than left unchecked.
-/

open Semantics Semantics.Macros

namespace Semantics.Proofs.Tables

theorem actLabelsDistinct : distinctActLabels actFacts = true := by decide

theorem counterLabelsDistinct : distinctCounterLabels counterFacts = true := by decide

theorem keywordWordsDistinct : distinctKeywordWords keywordFacts = true := by decide

theorem designationLabelsDistinct : distinctDesignationLabels designationTable = true := by decide

theorem subtypeFactsDistinct : distinctSubtypeFacts subtypeFacts = true := by decide

end Semantics.Proofs.Tables

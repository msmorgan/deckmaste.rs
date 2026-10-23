import Semantics.Words
import Semantics.Events

/-! The row types of the registry tables in `Check.Facts`. Those tables are hand-maintained Lean:
a keyword declared under `Semantics/Definitions/` gets its row there by hand, and `Proofs/Tables`
pins each table's labels distinct. A row carries only the columns the checker reads. A fact type
the checker owns rather than the registry lives beside its hand-written table in
`Check.Words`. -/

namespace Semantics

/-- The role of one ordered keyword argument. -/
inductive KeywordParamShape where
  | cost | quality | subject | number | ability | deckCondition
  deriving DecidableEq, Repr

/-- Registry-supplied checking data for one argument position. Quality domains may be
restricted to objects; an absent restriction retains the predicate's inferred domain. -/
structure KeywordParamSpec where
  shape : KeywordParamShape
  qualityDomain : Option Kind := none
  deriving DecidableEq, Repr

abbrev KeywordSchema := List KeywordParamSpec

structure KeywordFacts where
  word : KeywordLabel
  /-- Ordered argument schemas admitted by the keyword declaration and its variants. -/
  argumentSchemas : List KeywordSchema := []
  paidCost : Bool := false
  wantsModes : Bool := false
  deriving Repr, BEq

end Semantics

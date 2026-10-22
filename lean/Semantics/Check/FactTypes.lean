import Semantics.Words
import Semantics.Events

/-! The row types of the registry tables in `Check.Facts`. Those tables are hand-maintained Lean:
a declaration added under `Semantics/Definitions/` gets its row there by hand, and
`Proofs/Tables` pins each table's labels distinct. A row carries only the columns the checker
reads; what else a declaration says (a designation's zone or Room half, a relation's sides)
is its `Definition` record's (`Semantics.Rules`), not a facts column. A fact type the checker
owns rather than the registry lives beside its hand-written table in `Check.Words`. -/

namespace Semantics

/-- What a subtype declares about the card frame it sits on: a Saga's chapter frame [CR#714.1],
an Adventure's inset [CR#715.1], a Room's doors [CR#709.5j]. -/
inductive FrameFeature where
  | chapters | adventureInset | doors
  deriving DecidableEq, Repr

structure SubtypeFacts where
  subtype : Subtype
  frame : FrameFeature
  deriving Repr, BEq

structure DesignationFacts where
  /-- The designation's name (`DesignationLabel.name`): its declaration's name, or a member's. -/
  label : String
  /-- Who holds it: a player, an object, its card, or the game. -/
  scope : DesignationScope
  deriving Repr, BEq

/-- One Relation's row [CR#702.55b,702.95b,702.99b]: its name, which is all the checker reads of
it (`RelationLabel.known`). -/
structure RelationFacts where
  /-- The relation's name (`RelationLabel.name`): its declaration's name. -/
  label : String
  deriving Repr, BEq

structure CounterFacts where
  label : String
  /-- What the counter is placed on: an object or a player [CR#122.1]. -/
  holder : Kind
  deriving Repr, BEq

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

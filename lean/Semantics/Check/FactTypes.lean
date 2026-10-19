import Semantics.Words
import Semantics.Events

/-! The row types of the tables `cargo xtask facts generate` writes into `Check.Facts`, and
only those: the Lean/Rust mirror contract of `docs/decisions/semantics-v2.md` §10 covers
exactly this module. A fact type the checker owns rather than the registry lives beside its
hand-written table in `Check.Words`, outside the mirror. -/

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
  scope : DesignationScope
  /-- Whether an action may confer it directly ("becomes the monarch"); a designation that
  only a rule confers (the commander) is not conferred by text. -/
  effectful : Bool
  zone : Option Zone
  type : Option CardType
  /-- Which half of a Room permanent this designation unlocks, for the two that do. -/
  half : Option RoomHalf := none
  deriving Repr, BEq

def playerHeld (label : String) : DesignationFacts :=
  ⟨label, .heldBy .player, true, none, none, none⟩

def permanentHeld (label : String) : DesignationFacts :=
  ⟨label, .heldBy .object, true, some .battlefield, none, none⟩

/-- One Relation's columns [CR#702.55b,702.95b,702.99b]: its name and the kind of each side. -/
structure RelationFacts where
  /-- The relation's name (`RelationLabel.name`): its declaration's name. -/
  label : String
  left : Kind
  right : Kind
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
  onPermanentCard : Bool := true
  onInstantOrSorceryCard : Bool := false
  paidCost : Bool := false
  /-- True where paying the keyword's cost chooses a player, whom the ability linked to that cost
  reads back, as gift's "you may choose an opponent" [CR#702.174a,607.2d]. -/
  costChoosesPlayer : Bool := false
  wantsModes : Bool := false
  deriving Repr, BEq

end Semantics

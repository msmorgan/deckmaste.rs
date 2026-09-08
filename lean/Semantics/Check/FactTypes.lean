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
  label : DesignationLabel
  scope : DesignationScope
  /-- Whether an instruction may confer it directly ("becomes the monarch"); a designation that
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

inductive StackRegime where
  | atCasting | atResolution
  deriving DecidableEq, Repr

/-- The general category an ability belongs to [CR#113.3]: a statement that is simply true, a
trigger condition with an effect, or a cost with an effect [CR#113.3b,113.3c,113.3d]. A keyword
ability's definition is written in one of these three [CR#702.1]. -/
inductive AbilityCategory where
  | static | triggered | activated
  deriving DecidableEq, Repr

structure KeywordFacts where
  word : KeywordLabel
  /-- Ordered argument schemas admitted by the keyword declaration and its variants. -/
  argumentSchemas : List KeywordSchema := []
  counterEligible : Bool := false
  regime : Option StackRegime := none
  /-- True where the ability is the spell's own, so no permanent holds it [CR#113.6]. -/
  functionsOnStack : Bool := false
  onPermanentCard : Bool := true
  onInstantOrSorceryCard : Bool := false
  paidCost : Bool := false
  /-- The categories the keyword's definition is written in [CR#702.1]. A card may leave the
  definition unwritten; a written one is an ability of one of these categories
  [CR#702.9b,702.21a,702.24a,702.29a,702.30a,702.40a,702.45a,702.86a,702.112a,702.135a]. An
  empty list is a keyword whose definition the workbench has not yet declared. -/
  definition : List AbilityCategory := []
  wantsModes : Bool := false
  deriving Repr, BEq

end Semantics

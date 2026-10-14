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

inductive StackRegime where
  | atCasting | atResolution
  deriving DecidableEq, Repr

/-- The general category an ability belongs to [CR#113.3]: instructions followed while an
instant or sorcery spell resolves, a statement that is simply true, a trigger condition with an
effect, or a cost with an effect [CR#113.3a,113.3b,113.3c,113.3d]. A keyword ability's definition
is written in these [CR#702.1]; a spell ability in one is the half a keyword represents on an
instant or sorcery spell, guarded by that class [CR#702.131a,702.174b]. -/
inductive AbilityCategory where
  | static | triggered | activated | spell
  deriving DecidableEq, Repr

/-- One ability of a keyword's definition as the registry declares it [CR#702.1]: its category
and the stack regime the rules give it, `none` where it functions off the stack. A keyword's
abilities need not share one regime: offspring pays its cost as the spell is cast and its
second ability triggers as the permanent enters [CR#702.175a], casualty's second ability
triggers as the spell is cast [CR#702.153a], and an instant or sorcery's spell ability is
followed as the spell resolves [CR#113.3a,702.174b]. -/
structure ComponentFacts where
  category : AbilityCategory
  regime : Option StackRegime := none
  deriving DecidableEq, Repr

structure KeywordFacts where
  word : KeywordLabel
  /-- Ordered argument schemas admitted by the keyword declaration and its variants. -/
  argumentSchemas : List KeywordSchema := []
  counterEligible : Bool := false
  /-- The keyword's own stack regime, read where it is granted to a spell
  [CR#113.6]. A keyword whose definition is declared has the summary of its components': the
  first stack regime among them (`ComponentFacts.summary`, pinned in `Proofs/Tables`). -/
  regime : Option StackRegime := none
  /-- True where the ability is the spell's own, so no permanent holds it [CR#113.6]. -/
  functionsOnStack : Bool := false
  onPermanentCard : Bool := true
  onInstantOrSorceryCard : Bool := false
  paidCost : Bool := false
  /-- True where paying the keyword's cost chooses a player, whom the ability linked to that cost
  reads back, as gift's "you may choose an opponent" [CR#702.174a,607.2d]. -/
  costChoosesPlayer : Bool := false
  /-- The abilities the keyword's definition is written in [CR#702.1], each with its category
  and its own stack regime. A card may leave the definition unwritten; a written one is an
  ability of one of these components
  [CR#702.9b,702.21a,702.24a,702.29a,702.30a,702.40a,702.45a,702.86a,702.112a,702.135a]. An
  empty list is a keyword whose definition the workbench has not yet declared. -/
  definition : List ComponentFacts := []
  wantsModes : Bool := false
  deriving Repr, BEq

end Semantics

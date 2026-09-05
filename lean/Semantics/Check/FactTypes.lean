import Semantics.Words
import Semantics.Events

/-! The data columns shared by generated registry facts and their consumers. -/

namespace Semantics

/-- The Entity domain a referent is drawn from: a player, an object, or either [CR#102.1,109.1]. -/
inductive EntityDomain where
  | player | object | either
  deriving DecidableEq, Repr

/-- An object class [CR#109.1]: the overlapping ways an object is classified. -/
inductive ObjectClass where
  | card | token | spell | permanent | emblem | ability
  deriving DecidableEq, Repr

/-- What a referent must be: an Entity domain, optionally narrowed by object classes and card
types. Only an object has a class. Empty `classes` admit no ability; empty `types` admit only a
noun that names no type (`deedAltOk`), so a row that takes any typed noun lists `allTypes`. -/
structure ReferentSort where
  domain : EntityDomain
  classes : List ObjectClass := []
  types : List CardType := []
  deriving Repr, BEq

/-- Which kinds a domain admits: `.either` admits both, the others admit only their own. -/
def EntityDomain.admits : EntityDomain → Kind → Bool
  | .player, .player => true
  | .object, .object => true
  | .either, .object => true
  | .either, .player => true
  | _, _ => false

/-- A domain's admitted kinds, listed; kept for callers built around a kind list rather than a
membership test. -/
def EntityDomain.kinds : EntityDomain → List Kind
  | .player => [.player]
  | .object => [.object]
  | .either => [.object, .player]

structure DeedRole where
  sort : Option ReferentSort  -- none = no noun fills this role
  bare : Bool
  zone : Option Zone
  deriving Repr, BEq

def noRole : DeedRole := ⟨none, false, none⟩

/-- The card types a role's sort names; an absent sort names none. -/
def DeedRole.types (dr : DeedRole) : List CardType := dr.sort.elim [] (·.types)

inductive PremiseSort where
  | object | mana | value
  deriving DecidableEq, Repr

/-- The keyword actions [CR#701.1] a core constructor must name, declared so a guard reads a
feature rather than a verb's spelling. A deed the core rules define carries no feature: it is a
`CoreDeed` constructor, which a guard matches on directly. -/
inductive DeedFeature where
  | librarySearch | sacrificing | tapping
  deriving DecidableEq, Repr

/-- What the checker knows about one deed. The record carries no label: the deed itself is the
key, and each of the three sources keys its rows its own way. -/
structure ActFacts where
  participle : Option String := none
  dest : Option Zone := none
  stepwise : Bool := false
  loci : List Zone := []
  intransitive : Bool := false
  agentRole : DeedRole := noRole
  patientRole : DeedRole := noRole
  feature : Option DeedFeature := none
  counterfactual : Option PremiseSort := none
  rides : Bool := false
  plays : Bool := false
  bounded : Bool := false
  /-- The deed opens an opponent's library ("fateseal" [CR#701.29a]); the same look over one's
  own library is a different deed. -/
  opponentsLibrary : Bool := false
  /-- The designations this keyword action's expansion confers [CR#701.37a]. -/
  confers : List DesignationLabel := []
  deriving Repr, BEq

def playerAgent : DeedRole := ⟨some ⟨.player, [], []⟩, true, none⟩
def fieldObject : DeedRole := ⟨some ⟨.object, [], []⟩, false, some .battlefield⟩
def permanentTypes : List CardType :=
  [.creature, .artifact, .land, .enchantment, .planeswalker, .battle]
def spellTypes : List CardType :=
  [.creature, .artifact, .enchantment, .instant, .sorcery, .planeswalker, .battle, .kindred]
def allTypes : List CardType :=
  [.creature, .artifact, .land, .enchantment, .instant, .sorcery, .planeswalker, .battle, .kindred]

/-- What a subtype declares about the card frame it sits on: a Saga's chapter frame [CR#714.1],
an Adventure's inset [CR#715.1], a Room's doors [CR#709.5j]. -/
inductive FrameFeature where
  | chapters | adventureInset | doors
  deriving DecidableEq, Repr

structure SubtypeFacts where
  subtype : Subtype
  frame : FrameFeature
  deriving Repr, BEq

inductive DesignationScope where
  | heldBy (holder : Kind)
  | heldByCard
  | heldByGame
  deriving DecidableEq, Repr

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

inductive CompoundHead where
  | quality | number
  deriving DecidableEq, Repr

def CompoundHead.optional : CompoundHead → Bool
  | .quality => true
  | .number => false

/-- `quality` covers "partner with [name]" [CR#702.124j], whose slot is a card name. -/
inductive KeywordParamShape where
  | noParam | cost | quality | subject | number | ability
  | compound (head : CompoundHead)
  | deckCondition
  deriving DecidableEq, Repr

def KeywordParamShape.fits : KeywordParamShape → KeywordParamShape → Bool
  | .compound h, .cost => h.optional
  | want, got => want == got

/-- A written parameter fits a keyword when any of the row's admitted shapes takes it, so a
keyword's CR-defined variants live on one row. -/
def paramShapesFit (wants : List KeywordParamShape) (got : KeywordParamShape) : Bool :=
  wants.any (·.fits got)

inductive StackRegime where
  | atCasting | atResolution
  deriving DecidableEq, Repr

structure KeywordFacts where
  word : KeywordLabel
  /-- Every parameter shape the CR admits for this keyword [CR#702]. -/
  paramShapes : List KeywordParamShape := []
  counterEligible : Bool := false
  regime : Option StackRegime := none
  /-- True where the ability is the spell's own, so no permanent holds it [CR#113.6]. -/
  functionsOnStack : Bool := false
  onPermanentCard : Bool := true
  onInstantOrSorceryCard : Bool := false
  paidCost : Bool := false
  /-- Defined by the CR as a triggered ability with a quoted expansion
  [CR#702.21a,702.24a,702.30a,702.40a,702.45a,702.86a,702.112a,702.135a]. -/
  bodied : Bool := false
  wantsModes : Bool := false
  /-- The designations this keyword ability's expansion confers [CR#702.112a]. -/
  confers : List DesignationLabel := []
  deriving Repr, BEq

end Semantics

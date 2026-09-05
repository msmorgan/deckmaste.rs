import Semantics.Abilities

/-!
# Semantics.Card

The printed card: its faces and frames. Port of `idris/src/Experimental/Card.idr`, syntax
only. A face is a `CardFace`; the frames that carry more than one set name each.
-/

namespace Semantics

/-- A printed face: its characteristics and the choices its text announces as it enters (the
joint-choice device the face checker reads). -/
structure CardFace where
  characteristics : Characteristics
  choices : List QualitySort := []
  deriving Repr, BEq

/-- One half of a split card whose halves share a type line: its own name, cost, and text. -/
structure SharedLineHalf where
  name : String
  cost : Option ManaCost := none
  text : List Ability := []
  deriving Repr, BEq

/-- [CR#711.2a] a closed band, [CR#711.2b] the open last band. -/
inductive LevelRange where
  | between (from_ to : Nat)
  | atLeast (from_ : Nat)
  deriving DecidableEq, Repr

/-- One striation of a leveler's text box: the level symbol's range, the abilities printed in
that striation, and its power/toughness box [CR#711.2a,711.2b]. -/
structure LevelBand where
  range : LevelRange
  text : List Ability := []
  power : Option Amount := none
  toughness : Option Amount := none
  deriving Repr, BEq

/-- [CR#718.1] the inset frame's second set: a mana cost and a power/toughness box. -/
structure PrototypeFrame where
  cost : Option ManaCost := none
  power : Option Amount := none
  toughness : Option Amount := none
  deriving Repr, BEq

inductive Card where
  | singleFaced (face : CardFace)
  | transforming (front back : CardFace)
  | modalDfc (front back : CardFace)
  | split (left right : CardFace)
  /-- The shared line and box, and the two halves' own name, cost, and text. -/
  | sharedLineSplit (shared : CardFace) (left right : SharedLineHalf)
  | adventurer (normal adventure : CardFace)
  | flip (normal alternative : CardFace)
  | leveler (inner : CardFace) (bands : List LevelBand)
  /-- [CR#718.1] the inset frame's second set: a mana cost and a power/toughness box. -/
  | prototype (inner : CardFace) (alternative : PrototypeFrame)
  deriving Repr, BEq

end Semantics

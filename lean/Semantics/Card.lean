import Semantics.Abilities

/-!
# Semantics.Card

The printed card: its faces and frames. Port of `idris/src/Experimental/Card.idr`, syntax
only. A face is a `Characteristics`; the frames that carry more than one set name each.
-/

namespace Semantics

/-- A card face is its characteristics [CR#109.3]. -/
abbrev CardFace := Characteristics

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

structure LevelBand where
  range : LevelRange
  band : Characteristics
  deriving Repr, BEq

inductive Card where
  | singleFaced (face : CardFace)
  | transforming (front back : CardFace)
  | modalDfc (front back : CardFace)
  | split (left right : CardFace)
  /-- The shared line and box, and the two halves' own name, cost, and text. -/
  | sharedLineSplit (shared : Characteristics) (left right : SharedLineHalf)
  | adventurer (normal adventure : Characteristics)
  | flip (normal alternative : Characteristics)
  | leveler (inner : Characteristics) (bands : List LevelBand)
  /-- [CR#718.1] the inset frame's second set: a mana cost and a power/toughness box. -/
  | prototype (inner alternative : Characteristics)
  deriving Repr, BEq

end Semantics

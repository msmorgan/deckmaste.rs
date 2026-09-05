import Experimental.Effect

/-!
# Experimental.Card

The printed card: characteristics, faces, and the card frames. Port of
`idris/src/Experimental/Card.idr`, syntax only.

Not ported (the frame laws, all witnesses): `CardLine`, `CardBox`, `CharacteristicsLaws`,
`SharedLineHalfLaws`, `LevelBandLaws`, `LevelBandsLaws`, `PrototypeAltLaws`, `FaceLaws`.
-/

namespace Mtg

inductive CardClass where
  | permanentCard | spellCard
  deriving DecidableEq, Repr

/-- A printed power, toughness, loyalty, or defense value. -/
inductive PrintedStat where
  | num (n : Int)
  | star
  | starPlus (n : Nat)
  | minusStar (n : Nat)
  deriving DecidableEq, Repr

inductive PrintedBox where
  | pt (power toughness : PrintedStat)
  | loyalty (start : PrintedStat)
  | defense (def_ : PrintedStat)
  deriving DecidableEq, Repr

inductive FaceSide where
  | front | back
  deriving DecidableEq, Repr

structure Characteristics where
  name : String
  cost : Option ManaCost := none
  choices : List QualitySort := []
  typeLine : TypeLine
  text : AbilitySeq := []
  box : Option PrintedBox := none
  deriving Repr, BEq

/-- The Idris wraps `Characteristics` in a one-field record; here a face is its
characteristics. -/
abbrev CardFace := Characteristics

/-- One half of a split card whose halves share a type line. -/
structure SharedLineHalf where
  name : String
  cost : Option ManaCost
  text : AbilitySeq
  deriving Repr, BEq

inductive LevelRange where
  | between (from_ to : Nat)
  | atLeast (from_ : Nat)
  deriving DecidableEq, Repr

structure LevelBand where
  range : LevelRange
  box : PrintedBox
  text : AbilitySeq
  deriving Repr, BEq

structure PrototypeAlt where
  cost : ManaCost
  box : PrintedBox
  deriving Repr, BEq

inductive Card where
  | singleFaced (face : CardFace)
  | transforming (front back : CardFace)
  | modalDfc (front back : CardFace)
  | split (left right : CardFace)
  | sharedLineSplit (typeLine : TypeLine) (box : Option PrintedBox)
      (left right : SharedLineHalf)
  | adventurer (normal adventure : Characteristics)
  | flip (normal alternative : Characteristics)
  | leveler (inner : CardFace) (bands : List LevelBand)
  | prototype (inner : CardFace) (alt : PrototypeAlt)
  deriving Repr, BEq

end Mtg

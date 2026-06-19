module Main

import Data.Vect

data Color
  = White
  | Blue
  | Black
  | Red
  | Green

data SimpleManaSymbol
  = Generic Nat
  | Specific (Maybe Color)

data ManaSymbol
  = Simple SimpleManaSymbol
  | Hybrid SimpleManaSymbol Color

ManaCost : Type
ManaCost = List ManaSymbol

data Type_
  = Artifact
  | Battle
  | Creature
  | Enchantment
  | Instant
  | Kindred
  | Land
  | Planeswalker
  | Sorcery

data Zone
  = Battlefield
  | Command
  | Exile
  | Graveyard
  | Hand
  | Library
  | Stack

data Frame : { hasThis : Bool } -> { targetCount : Nat } -> Type where
  MkFrame : { default False hasThis : Bool } -> { default 0 targetCount : Nat } -> Frame {hasThis, targetCount}

bindTargets : (n : Nat) -> Frame {hasThis} -> Frame {hasThis, targetCount = n}
bindTargets n (MkFrame {hasThis}) = MkFrame {hasThis, targetCount = n}

mutual
  data Filter : Frame -> Type where
    IsAll : (List (Filter f)) -> Filter f
    IsAny : (List (Filter f)) -> Filter f
    IsPlayer : Filter f
    IsInZone : Zone -> Filter f
    IsType : Type_ -> Filter f
    -- should be min target = 1
    IsTargeted : Filter (MkFrame {targetCount = 1})
    IsRef : Reference f -> Filter f

  data Reference : Frame -> Type where
    This : Reference (MkFrame {hasThis = True})
    You : Reference f
    GetTarget : (n : Nat) -> Reference (MkFrame {targetCount = (S n)})
    Only : Filter f -> Reference f

data TargetSpec : Frame -> Type where
  Target : Nat -> Filter f -> TargetSpec f

data Effect : Frame -> Type where
  Sequence : (List (Effect f)) -> Effect f
  Targeted : (Vect n (TargetSpec f)) -> Effect (bindTargets n f) -> Effect f
  Damage : (source : Reference f) -> (to : Reference f) -> Nat -> Effect f

data Ability = Spell (Effect f)

permanent : Filter f
permanent = IsInZone Battlefield

anyTarget : TargetSpec f
anyTarget = Target 1 $ IsAny
  [ IsPlayer
  , IsAll [permanent, IsType Battle]
  , IsAll [permanent, IsType Creature]
  , IsAll [permanent, IsType Planeswalker]
  ]

playerOrPlaneswalker : TargetSpec f
playerOrPlaneswalker = Target 1 $ IsAny
  [ IsPlayer
  , IsAll [permanent, IsType Planeswalker]
  ]


record Face where
  constructor MkFace
  name : String
  manaCost : ManaCost
  types : List Type_
  abilities : List Ability

interface DefaultValue a where
  defaultValue : a

fromDefault : (DefaultValue a) => (a -> a) -> a
fromDefault f = f defaultValue

DefaultValue Face where
  defaultValue = MkFace
    { name = ""
    , manaCost = []
    , types = []
    , abilities = []
    }

data Card = Normal Face

LightningBolt : Card
LightningBolt = Normal $ fromDefault
  { name := "Lightning Bolt"
  , manaCost := [Simple . Specific . Just $ Red]
  , types := [Instant]
  , abilities :=
      [ Spell (Targeted [anyTarget] 
          (Damage This (GetTarget 0) 3)
        )
      ]
  }

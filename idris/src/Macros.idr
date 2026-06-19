||| Reusable filter / target templates — the Idris analogue of the deckmaste
||| plugin macros (`permanent`, `anyTarget`, …): named, parameter-free grammar
||| fragments shared across cards. They are ordinary `Core` values, polymorphic
||| in the `Bindings` they sit in.
module Macros

import public Core

public export
permanent : Filter b
permanent = IsInZone Battlefield

public export
anyTarget : TargetSpec b
anyTarget = Target 1 $ IsAny
  [ IsPlayer
  , IsAll [permanent, IsType Battle]
  , IsAll [permanent, IsType Creature]
  , IsAll [permanent, IsType Planeswalker]
  ]

public export
playerOrPlaneswalker : TargetSpec b
playerOrPlaneswalker = Target 1 $ IsAny
  [ IsPlayer
  , IsAll [permanent, IsType Planeswalker]
  ]

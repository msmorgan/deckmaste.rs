||| Reusable named filters — the Idris analogue of the deckmaste plugin macros:
||| a `Predicate` given a domain name, so cards read `SelectAll creature`. The
||| combinators (`AllOf`/`OneOf`/`IsNot`) and identity test (`SameAs`) are `Core`
||| constructors used directly — no redundant `allF`/`notF`/`isRef` aliases.
module Macros

import public Core

public export
permanent : Filter b
permanent = InZone Battlefield

public export
creature : Filter b
creature = HasType Creature

public export
inHand : Filter b
inHand = InZone Hand

-- "at the beginning of the next end step" — the common delayed-trigger event.
public export
nextEndStep : EventQuery b
nextEndStep = KindIs (BeginStep (EndingPhase EndStep))

public export
anyTarget : TargetSpec b
anyTarget = Target 1 $ OneOf
  [ IsKind IsPlayerKind
  , AllOf [permanent, HasType Battle]
  , AllOf [permanent, creature]
  , AllOf [permanent, HasType Planeswalker]
  ]

public export
playerOrPlaneswalker : TargetSpec b
playerOrPlaneswalker = Target 1 $ OneOf
  [ IsKind IsPlayerKind
  , AllOf [permanent, HasType Planeswalker]
  ]

||| Reusable filter / target templates — the Idris analogue of the deckmaste
||| plugin macros. Named `Subject`-predicates (built on `Core`'s `where*`
||| helpers) and the filter combinators, so cards read `SelectAll creature`.
module Macros

import public Core

public export
permanent : Filter b
permanent = whereInZone Battlefield

public export
creature : Filter b
creature = whereHasType Creature

public export
inHand : Filter b
inHand = whereInZone Hand

-- "the candidate is exactly r" — the filter form of a single reference.
public export
isRef : Reference (bindSubject b) -> Filter b
isRef = whereSame

-- filter combinators: the `Condition` combinators lifted through the `Where` tag.
public export
allF : List (Filter b) -> Filter b
allF fs = Where (And (map unFilter fs))

public export
anyF : List (Filter b) -> Filter b
anyF fs = Where (Or (map unFilter fs))

public export
notF : Filter b -> Filter b
notF = Where . Not . unFilter

public export
anyTarget : TargetSpec b
anyTarget = Target 1 $ anyF
  [ whereIsKind IsPlayerKind
  , allF [permanent, whereHasType Battle]
  , allF [permanent, creature]
  , allF [permanent, whereHasType Planeswalker]
  ]

public export
playerOrPlaneswalker : TargetSpec b
playerOrPlaneswalker = Target 1 $ anyF
  [ whereIsKind IsPlayerKind
  , allF [permanent, whereHasType Planeswalker]
  ]

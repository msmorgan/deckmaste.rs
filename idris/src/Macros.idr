||| Reusable filter / target templates — the Idris analogue of the deckmaste
||| plugin macros. Named object predicates and the predicate combinators, so
||| cards read `SelectAll creature`. A *filter* is just a `Predicate`.
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

-- "the candidate is exactly r" — the filter form of a single reference.
public export
isRef : Reference b -> Filter b
isRef = SameAs

-- filter combinators: the `Predicate` combinators under friendlier names.
public export
allF : List (Filter b) -> Filter b
allF = AllOf

public export
anyF : List (Filter b) -> Filter b
anyF = OneOf

public export
notF : Filter b -> Filter b
notF = Except

public export
anyTarget : TargetSpec b
anyTarget = Target 1 $ anyF
  [ IsKind IsPlayerKind
  , allF [permanent, HasType Battle]
  , allF [permanent, creature]
  , allF [permanent, HasType Planeswalker]
  ]

public export
playerOrPlaneswalker : TargetSpec b
playerOrPlaneswalker = Target 1 $ anyF
  [ IsKind IsPlayerKind
  , allF [permanent, HasType Planeswalker]
  ]

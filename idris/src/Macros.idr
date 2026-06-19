||| Reusable filter / target templates — the Idris analogue of the deckmaste
||| plugin macros. A *filter* is a `Condition` with `Subject` in scope, wrapped as
||| `Core.Filter`; these name the common Subject-predicates and lift the condition
||| combinators through the wrapper, so cards read `SelectAll creature`.
module Macros

import public Core

public export
permanent : Filter b
permanent = Where (InZone Subject Battlefield)

public export
creature : Filter b
creature = Where (HasType Subject Creature)

public export
inHand : Filter b
inHand = Where (InZone Subject Hand)

-- "the candidate is exactly r" — the filter form of a single reference. `r` is
-- taken in the filter (subject) context; `This`/`GetTarget`/`It` are polymorphic.
public export
isRef : Reference (bindSubject b) -> Filter b
isRef r = Where (SameObject Subject r)

-- filter combinators: the `Condition` combinators lifted through the wrapper.
public export
allF : List (Filter b) -> Filter b
allF fs = Where (AllOf (map unFilter fs))

public export
anyF : List (Filter b) -> Filter b
anyF fs = Where (OneOf (map unFilter fs))

public export
notF : Filter b -> Filter b
notF = Where . Not . unFilter

public export
anyTarget : TargetSpec b
anyTarget = Target 1 $ anyF
  [ Where (OfKind Subject IsPlayerKind)
  , allF [permanent, Where (HasType Subject Battle)]
  , allF [permanent, creature]
  , allF [permanent, Where (HasType Subject Planeswalker)]
  ]

public export
playerOrPlaneswalker : TargetSpec b
playerOrPlaneswalker = Target 1 $ anyF
  [ Where (OfKind Subject IsPlayerKind)
  , allF [permanent, Where (HasType Subject Planeswalker)]
  ]

||| Reusable named filters — the Idris analogue of the deckmaste plugin macros:
||| a `Predicate` given a domain name, so cards read `SelectAll creature`. The
||| combinators (`AllOf`/`OneOf`/`IsNot`) and identity test (`SameAs`) are `Core`
||| constructors used directly — no redundant `allF`/`notF`/`isRef` aliases.
module Macros

import public Core

public export
permanent : Predicate b AnObject
permanent = InZone Battlefield

public export
creature : Predicate b AnObject
creature = HasType Creature

public export
inHand : Predicate b AnObject
inHand = InZone Hand

-- player-predicates ([CR#102.1]): `you` is the controller; `opponent` is any OTHER player
-- (team-free — "a player who isn't you"). Feed `ControlledBy`/`ActorIs`/`Target 1`/`SelectAll`.
public export
you : Predicate b APlayer
you = SameAs You

public export
opponent : Predicate b APlayer
opponent = IsNot (SameAs You)

-- "at the beginning of the next end step" — the common delayed-trigger event.
public export
nextEndStep : EventQuery b
nextEndStep = KindIs (BeginStep (EndingPhase EndStep))

-- "any target" ([CR#115.4]): a creature/planeswalker/battle permanent, OR any player. A
-- FLAT `OneOf` — the player arm (`Anyone`) sits beside the object arms, and the result
-- kind is their join (`Anything`), computed by `\/`. No `Widen`.
public export
anyTarget : TargetSpec b Anything
anyTarget = Target 1 $ OneOf
  [ AllOf [permanent, HasType Battle]
  , AllOf [permanent, creature]
  , AllOf [permanent, HasType Planeswalker]
  , Anyone ]

public export
playerOrPlaneswalker : TargetSpec b Anything
playerOrPlaneswalker = Target 1 $ OneOf [ AllOf [permanent, HasType Planeswalker], Anyone ]

-- "each player": a player-`Selection` for `ForEach` to distribute over (the old plural
-- `EachPlayer` reference is gone — plurality lives in `Selection`, kinded `APlayer`).
public export
eachPlayer : Selection b APlayer
eachPlayer = SelectAll Anyone

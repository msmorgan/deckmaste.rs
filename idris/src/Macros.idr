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

-- "any spell or ability" — the universal targeting SOURCE.
public export
spellOrAbility : Predicate b AnObject
spellOrAbility = OneOf [IsKind IsSpell, IsKind IsAbility]

-- KEYWORD macros: each builds the FULL keyword `Ability` — a `Composite` of the `KeywordSpec`
-- tag + the `Cant` clause it desugars to (over `This`). The non-deontic keywords (FirstStrike/
-- Deathtouch/Trample = damage; Vigilance = event-edit; Reach/Flash = flag/window) carry no
-- clause. (Flying reads `HasKeyword Flying`/`Reach` on the BLOCKER — the tag its clause consults.)
public export
flying : Ability b
flying = Keyword (Composite Flying [Static (Cant (Blocks (IsNot (OneOf [HasKeyword Flying, HasKeyword Reach])) (SameAs This)))])

public export
defender : Ability b
defender = Keyword (Composite Defender [Static (Cant (Attacks (SameAs This)))])

public export
shroud : Ability b
shroud = Keyword (Composite Shroud [Static (Cant (BeTargeted (SameAs This)))])   -- by = any spell/ability (default)

public export
hexproof : Ability b
hexproof = Keyword (Composite (Hexproof Nothing) [Static (Cant (BeTargeted (SameAs This) {by = ControlledBy opponent}))])

-- "hexproof from [f]": can't be targeted by an opponent's source matching `f`. `f` may be an
-- ANAPHOR ("from the CHOSEN color") — the reason `Ability` is `Bindings`-indexed.
public export
hexproofFrom : Predicate b AnObject -> Ability b
hexproofFrom f = Keyword (Composite (Hexproof (Just f)) [Static (Cant (BeTargeted (SameAs This) {by = AllOf [ControlledBy opponent, f]}))])

-- desugar a `KeywordSpec` into its full `Ability` — dispatches to the macros above. EXHAUSTIVE
-- (no catch-all): adding a `KeywordSpec` constructor forces a clause here, so a new keyword can't
-- silently fall through to an empty `Composite`. The intrinsic keywords are spelled out as
-- clause-free `Composite`s (engine-handled). (A plain function, NOT a `Cast` instance: a
-- polymorphic spec's `b` is a metavar at the call site, which interface search can't fire on.)
public export
keyword : KeywordSpec b -> Ability b
keyword Flying              = flying
keyword FirstStrike         = Keyword (Composite FirstStrike [])
keyword DoubleStrike        = Keyword (Composite DoubleStrike [])
keyword Deathtouch          = Keyword (Composite Deathtouch [])
keyword Reach               = Keyword (Composite Reach [])
keyword Trample             = Keyword (Composite Trample [])
keyword Vigilance           = Keyword (Composite Vigilance [])
keyword Flash               = Keyword (Composite Flash [])
keyword Defender            = defender
keyword Shroud              = shroud
keyword (Hexproof Nothing)  = hexproof
keyword (Hexproof (Just f)) = hexproofFrom f

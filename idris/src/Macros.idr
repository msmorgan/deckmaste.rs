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
-- (team-free — "a player who isn't you"). Feed `ControlledBy`/`ActorIs`/`Target (^1)`/`SelectAll`.
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
anyTarget = Target (^1) $ OneOf
  [ AllOf [permanent, HasType Battle]
  , AllOf [permanent, creature]
  , AllOf [permanent, HasType Planeswalker]
  , Anyone ]

public export
playerOrPlaneswalker : TargetSpec b Anything
playerOrPlaneswalker = Target (^1) $ OneOf [ AllOf [permanent, HasType Planeswalker], Anyone ]

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

-- Flash ([CR#702.8a]): a deontic `Can` to cast THIS at instant speed — a widened cast window, not
-- an as-though. ("Granted as-though-flash" for OTHER spells is `AsThough`, the deferred-tail case.)
public export
flash : Ability b
flash = Keyword (Composite Flash [Static (Can (Casts you (SameAs This)) {window = Just InstantWindow})])

-- Menace ([CR#702.111b]): a SET-LEVEL `Cant` — "can't be blocked except by two or more", i.e.
-- can't be blocked by a lone (size-1) blocker set. The whole-set predicate [CR#509.1c] needs the
-- `BlockedBy` deed, not the per-blocker `Blocks` (which flying/Cant uses).
public export
menace : Ability b
menace = Keyword (Composite Menace [Static (Cant (BlockedBy (SameAs This) (^1)))])

-- desugar a `KeywordSpec` into its full `Ability` — dispatches to the macros above. EXHAUSTIVE
-- (no catch-all): adding a `KeywordSpec` constructor forces a clause here. `Bare` = an engine-
-- PRIMITIVE keyword the grammar can't desugar (FirstStrike/DoubleStrike/Deathtouch/Trample =
-- damage pipeline; Vigilance = attack event-edit). The rest are `Composite`: the deontic ones
-- carry a `Cant`; `Reach` carries `[]` (just a flag `flying`'s clause reads); `Flash` carries a
-- `Can (Casts …) {window = InstantWindow}` — you may cast it at instant speed ([CR#702.8a]).
-- (A plain function, NOT an interface instance: a polymorphic spec's `b` is a metavar interface search can't fire on.)
public export
keyword : KeywordSpec b -> Ability b
keyword Flying              = flying
keyword FirstStrike         = Keyword (Bare FirstStrike)
keyword DoubleStrike        = Keyword (Bare DoubleStrike)
keyword Deathtouch          = Keyword (Bare Deathtouch)
keyword Trample             = Keyword (Bare Trample)
keyword Vigilance           = Keyword (Bare Vigilance)
keyword Reach               = Keyword (Composite Reach [])
keyword Flash               = flash
keyword Defender            = defender
keyword Shroud              = shroud
keyword Menace              = menace
keyword (Hexproof Nothing)  = hexproof
keyword (Hexproof (Just f)) = hexproofFrom f

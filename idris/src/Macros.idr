||| Reusable named filters — the Idris analogue of the deckmaste plugin macros:
||| a `Predicate` given a domain name, so cards read `SelectAll creature`. The
||| combinators (`And`/`Or`/`Not`) and identity test (`SameAs`) are `Core`
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
-- (team-free — "a player who isn't you"). Feed `ControlledBy`/`Actor`/`Target (^1)`/`SelectAll`.
public export
you : Predicate b APlayer
you = SameAs You

public export
opponent : Predicate b APlayer
opponent = Not (SameAs You)

-- "at the beginning of the next end step" — the common delayed-trigger event.
public export
nextEndStep : EventQuery b
nextEndStep = MkQuery [BeginStep (EndingPhase EndStep)] []

-- "when THIS enters the battlefield" — the ETB trigger event, inlined across most permanents.
public export
thisEnters : EventQuery b
thisEnters = MkQuery [ZoneChanged Nothing (Just Battlefield)] [Agent (SameAs This)]

-- "any target" ([CR#115.4]): a creature/planeswalker/battle permanent, OR any player. A
-- FLAT `Or` — the player arm (`Anyone`) sits beside the object arms, and the result
-- kind is their join (`Anything`), computed by `\/`. No `Widen`.
public export
anyTarget : TargetSpec b Anything
anyTarget = Target (^1) $ Or
  [ And [permanent, HasType Battle]
  , And [permanent, creature]
  , And [permanent, HasType Planeswalker]
  , Anyone ]

public export
playerOrPlaneswalker : TargetSpec b Anything
playerOrPlaneswalker = Target (^1) $ Or [ And [permanent, HasType Planeswalker], Anyone ]

-- "each player": a player-`Selection` for `ForEach` to distribute over (the old plural
-- `EachPlayer` reference is gone — plurality lives in `Selection`, kinded `APlayer`).
public export
eachPlayer : Selection b APlayer
eachPlayer = SelectAll Anyone

-- "any spell or ability" — the universal targeting SOURCE.
public export
spellOrAbility : Predicate b AnObject
spellOrAbility = Or [IsKind IsSpell, IsKind IsAbility]

-- KEYWORD macros: each builds the FULL keyword `Ability` — a `Composite` of the `KeywordSpec`
-- tag + the `Cant` clause it desugars to (over `This`). The non-deontic keywords (FirstStrike/
-- Deathtouch/Trample = damage; Vigilance = event-edit; Reach/Flash = flag/window) carry no
-- clause. (Flying reads `HasKeyword Flying`/`Reach` on the BLOCKER — the tag its clause consults.)
public export
flying : Ability b
flying = Keyword (Composite Flying [Static (Cant (Enact Block (Not (Or [HasKeyword Flying, HasKeyword Reach])) (SameAs This)))])

public export
defender : Ability b
defender = Keyword (Composite Defender [Static (Cant (Enact Attack (SameAs This) Anyone))])

public export
shroud : Ability b
shroud = Keyword (Composite Shroud [Static (Cant (Enact Target spellOrAbility (SameAs This)))])   -- by any spell/ability

public export
hexproof : Ability b
hexproof = Keyword (Composite (Hexproof Nothing) [Static (Cant (Enact Target (ControlledBy opponent) (SameAs This)))])

-- "hexproof from [f]": can't be targeted by an opponent's source matching `f`. `f` may be an
-- ANAPHOR ("from the CHOSEN color") — the reason `Ability` is `Bindings`-indexed.
public export
hexproofFrom : Predicate b AnObject -> Ability b
hexproofFrom f = Keyword (Composite (Hexproof (Just f)) [Static (Cant (Enact Target (And [ControlledBy opponent, f]) (SameAs This)))])

-- Flash ([CR#702.8a]): a deontic `Can` to cast THIS at instant speed — a widened cast window, not
-- an as-though. ("Granted as-though-flash" for OTHER spells is `AsThough`, the deferred-tail case.)
public export
flash : Ability b
flash = Keyword (Composite Flash [Static (Can (Enact Cast you (SameAs This)) {window = Just InstantWindow})])

-- Menace ([CR#702.111b]): a SET-LEVEL `Cant` — "can't be blocked except by two or more", i.e.
-- can't be blocked by a lone (size-1) blocker set. The whole-set predicate [CR#509.1c] needs the
-- `BlockedBy` deed, not the per-blocker `Blocks` (which flying/Cant uses).
public export
menace : Ability b
menace = Keyword (Composite Menace [Static (Cant (BlockedBy (SameAs This) (^1)))])

-- Haste ([CR#702.10]): a CONTINUOUS grant letting THIS attack and tap-activate "as though it had
-- been controlled continuously" — i.e. as though it weren't summoning-sick ([CR#302.6]). Built with
-- the AsThough machinery: pretend `Not (HasState SummoningSick)`, then `Can` the deed. (Grantable
-- via `GrantAbility (keyword Haste)` — e.g. Through the Breach. The doc spells haste as a flag the
-- summoning-sickness `Cant` reads; the as-though framing is the dual, and the one the toy carries.)
public export
haste : Ability b
haste = Keyword (Composite Haste
  [ Static (AsThough (Matches This (Not (HasState SummoningSick))) (Can (Enact Attack (SameAs This) Anyone)))
  , Static (AsThough (Matches This (Not (HasState SummoningSick))) (Can (Enact Activate you (SameAs This)))) ])

-- Indestructible ([CR#702.12]): "can't be destroyed by damage or 'destroy'." A continuous PROHIBITION,
-- not a replace-with-nothing — its `Composite` clause is `CantHappen` (the destroy of THIS can't happen),
-- semantically distinct from a `Replaces`-empty skip. Grantable like any keyword.
public export
indestructible : Ability b
indestructible = Keyword (Composite Indestructible
  [Static (CantHappen (MkQuery [Destroyed] [Patient (SameAs This)]))])

-- Devoid ([CR#702.114]): "this object is colorless" — a CDA, expressible now that the unified `Set`
-- can CLEAR a characteristic. `Set Colors []` on This empties its color set (the Tarmogoyf-`*/*` pattern).
public export
devoid : Ability b
devoid = Keyword (Composite Devoid [Static (Modify This [Set Colors []])])

-- "Regenerate this" ([CR#701.19]): a ONE-SHOT, this-turn shield — the next time This would be destroyed,
-- instead remove all damage, tap it, and remove it from combat. The `UpTo (^1)` limit consumes the
-- replacement after one destroy (vs `Replaces`'s default `Unlimited`).
public export
regenerate : OneShotEffect b
regenerate = Continuously
  (Replaces (MkQuery [Destroyed] [Patient (SameAs This)])
            (Sequence [Act (RemoveAllDamage This), Act (Tap This), Act (RemoveFromCombat This)])
            {limit = UpTo (^1)})
  UntilEndOfTurn

-- "Protection from [q]" ([CR#702.16]): the DEBT bundle, keyed to the quality `q` — can't be Damaged by
-- `q` sources, Enchanted/equipped by `q`, Blocked by `q`, or Targeted by `q`. ONE construct over the
-- existing `Cant`/`ReplaceAmount` parts (the `Agent` facet — the damage source — for the D leg).
public export
protection : Predicate b AnObject -> Ability b
protection q = Keyword (Composite (Protection q)
  [ Static (ReplaceAmount DealDamage (^0) {facets = [Patient (SameAs This), Agent q]})   -- D
  , Static (Cant (Enact Attach q (SameAs This)))        -- E
  , Static (Cant (Enact Block q (SameAs This)))         -- B
  , Static (Cant (Enact Target q (SameAs This))) ])     -- T

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
keyword Haste               = haste
keyword Indestructible      = indestructible
keyword Defender            = defender
keyword Shroud              = shroud
keyword Menace              = menace
keyword (Hexproof Nothing)  = hexproof
keyword (Hexproof (Just f)) = hexproofFrom f
keyword Morph               = Keyword (Bare Morph)   -- DEGENERATE (bare morph carries no cost) — use the `morph <cost>` macro for the real ability
keyword Devoid              = devoid
keyword (Protection q)      = protection q

-- KEYWORD ACTIONS (composite verbs over the primitives — the Idris analogue of the engine's
-- keyword-action macros; named here rather than inlined per card).

-- "Monstrosity N" ([CR#701.32]): if THIS isn't monstrous, put N +1/+1 counters on it and it becomes
-- monstrous. An activated ability whose cost varies per card.
public export
monstrosity : Cost b -> Count b -> Ability b
monstrosity cost n = Activated cost
  (If (Matches This (Not (HasDesignation Monstrous)))
      (Sequence [ Act (PutCounters P1P1 n This), Act (GrantDesignation Monstrous This) ]))

-- "Level up [cost]" ([CR#702.87]): put a level counter on THIS; sorcery-speed only.
public export
levelUp : Cost b -> Ability b
levelUp cost = Activated cost (Act (PutCounters Level (^1) This)) {window = SorceryWindow}

-- "Crew N" ([CR#702.122]): tap any creatures with total power ≥ N → this Vehicle becomes an artifact
-- creature until end of turn. The aggregate-tap cost is `TapTotal`.
public export
crew : Count b -> Ability b
crew n = Activated (TapTotal Power GreaterEq n creature)
  (Continuously (Modify This [AddType Creature]) UntilEndOfTurn)

-- "Morph [cost]" ([CR#702.37]): you may cast this face down as a 2/2 for {3} (`CastFaceDown`), and turn
-- it face up any time for [cost] (`TurnFaceUp`). The 2/2-colorless-vanilla face-down body is the global
-- [CR#708.2] rule (engine-applied on the `FaceDown` state), not restated here.
public export
morph : Cost b -> Ability b
morph c = Keyword (Composite Morph [ Static (CastFaceDown (Mana [^3])), TurnFaceUp c ])

-- the number of card TYPES among cards in all graveyards (Tarmogoyf's `*`) — a sum of per-type
-- INDICATORS: `Min (CountOf (graveyard ∧ that type)) 1` is 1 when present, else 0, over every card type.
public export
typesInGraveyards : Count b
typesInGraveyards = foldr Plus (Literal 0) (map indicator allCardTypes)
  where
    allCardTypes : List Type_
    allCardTypes = [Artifact, Battle, Creature, Enchantment, Instant, Kindred, Land, Planeswalker, Sorcery]
    indicator : Type_ -> Count b
    indicator t = Min (CountOf (And [InZone Graveyard, HasType t])) (Literal 1)

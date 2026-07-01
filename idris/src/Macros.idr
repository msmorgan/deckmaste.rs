||| Reusable named filters — the Idris analogue of the deckmaste plugin macros:
||| a `Predicate` given a domain name, so cards read `SelectAll creature`. The
||| combinators (`And`/`Or`/`Not`) and identity test (`SameAs`) are `Core`
||| constructors used directly — no redundant `allF`/`notF`/`isRef` aliases.
module Macros

import public Core

-- terse read sugar over the unified `HasChar`/`SharesChar`/`PlayerStatOf` primitives,
-- so card source stays readable (the primitive is what the RON serializes).
public export
hasType : Type_ -> Predicate b AnObject
hasType t = HasChar Types t

public export
hasColor : Color -> Predicate b AnObject
hasColor c = HasChar Colors c

public export
hasSubtype : Subtype -> Predicate b AnObject
hasSubtype s = HasChar Subtypes s

public export
hasSupertype : Supertype -> Predicate b AnObject
hasSupertype s = HasChar Supertypes s

public export
sharesSubtype : Reference b AnObject -> Predicate b AnObject
sharesSubtype r = SharesChar Subtypes r

public export
lifeTotal : Reference b APlayer -> Count b
lifeTotal r = PlayerStatOf r Life

public export
handSize : Reference b APlayer -> Count b
handSize r = PlayerStatOf r HandSize

public export
permanent : Predicate b AnObject
permanent = InZone Battlefield

public export
creature : Predicate b AnObject
creature = hasType Creature

public export
inHand : Predicate b AnObject
inHand = InZone Hand

-- player-predicates: `you` is the controller; `opponent`/`teammate` are TEAM-relative ([CR#102.3]) — an
-- opponent is a player NOT on your team (NOT merely "not you": a teammate in Two-Headed Giant is neither),
-- a teammate another player ON your team. Lowercase sugar for the `OpponentOf`/`TeammateOf` engine primitives.
-- Feed `ControlledBy`/`Actor`/`Target (^1)`/`SelectAll`.
public export
you : Predicate b APlayer
you = SameAs You

public export
opponent : Predicate b APlayer
opponent = OpponentOf

public export
teammate : Predicate b APlayer
teammate = TeammateOf

-- "at the beginning of the next end step" — the common delayed-trigger event.
public export
nextEndStep : EventQuery b
nextEndStep = MkEventQuery [BeginStep (EndingPhase EndStep)] []

-- "when THIS enters the battlefield" — the ETB trigger event, inlined across most permanents.
public export
thisEnters : EventQuery b
thisEnters = MkEventQuery [ZoneChanged Nothing (Just Battlefield)] [Agent (SameAs This)]

-- "any target" ([CR#115.4]): a creature/planeswalker/battle permanent, OR any player. A
-- FLAT `Or` — the player arm (`Anyone`) sits beside the object arms, and the result
-- kind is their join (`Anything`), computed by `\/`. No `Widen`.
public export
anyTarget : TargetSpec b Anything
anyTarget = Target (^1) $ Or
  [ And [permanent, hasType Battle]
  , And [permanent, creature]
  , And [permanent, hasType Planeswalker]
  , Anyone ]

public export
playerOrPlaneswalker : TargetSpec b Anything
playerOrPlaneswalker = Target (^1) $ Or [ And [permanent, hasType Planeswalker], Anyone ]

-- "each player": a player-`Selection` for `Each` to distribute over (the old plural
-- `EachPlayer` reference is gone — plurality lives in `Selection`, kinded `APlayer`).
public export
eachPlayer : Selection b APlayer
eachPlayer = SelectAll Anyone

-- "any spell or ability" — the universal targeting SOURCE.
public export
spellOrAbility : Predicate b AnObject
spellOrAbility = Or [IsKind Spell, IsKind Ability]

-- the two COMPULSION aliases over the single polarized `Constrain` ([CR#508.1c] restriction /
-- [CR#508.1d] requirement): `cant d` forbids the deed, `must d` requires it (the combat solver
-- balances both). The deed-legality surface stays ergonomic; the primitive stays one constructor.
public export
cant : Deed b -> StaticEffect b
cant = Constrain Forbid

public export
must : Deed b -> StaticEffect b
must = Constrain Require

-- KEYWORD macros: each builds the FULL keyword `Ability` — a `Composite` of the `KeywordSpec`
-- tag + the deontic clause it desugars to (over `This`). The non-deontic keywords (FirstStrike/
-- Deathtouch/Trample = damage; Vigilance = event-edit; Reach/Flash = flag/window) carry no
-- clause. (Flying reads `HasKeyword Flying`/`Reach` on the BLOCKER — the tag its clause consults.)
public export
flying : Ability b
flying = Keyword (Composite Flying [Static (cant (Enact Block (Not (Or [HasKeyword Flying, HasKeyword Reach])) (SameAs This)))])

public export
defender : Ability b
defender = Keyword (Composite Defender [Static (cant (Enact Attack (SameAs This) Anyone))])

public export
shroud : Ability b
shroud = Keyword (Composite Shroud [Static (cant (Enact Target spellOrAbility (SameAs This)))])   -- by any spell/ability

public export
hexproof : Ability b
hexproof = Keyword (Composite (Hexproof Nothing) [Static (cant (Enact Target (ControlledBy opponent) (SameAs This)))])

-- "hexproof from [f]": can't be targeted by an opponent's source matching `f`. `f` may be an
-- ANAPHOR ("from the CHOSEN color") — the reason `Ability` is `Endophora`-indexed.
public export
hexproofFrom : Predicate b AnObject -> Ability b
hexproofFrom f = Keyword (Composite (Hexproof (Just f)) [Static (cant (Enact Target (And [ControlledBy opponent, f]) (SameAs This)))])

-- Flash ([CR#702.8a]): a deontic `Can` to cast THIS at instant speed — a widened cast window, not
-- an as-though. ("Granted as-though-flash" for OTHER spells is `AsThough`, the deferred-tail case.)
public export
flash : Ability b
flash = Keyword (Composite Flash [Static (Can (Enact Cast you (SameAs This)) {window = Just AsInstant})])

-- Menace ([CR#702.111b]): a SET-LEVEL `cant` — "can't be blocked except by two or more", i.e.
-- can't be blocked by a lone (size-1) blocker set. The whole-set predicate [CR#509.1c] needs the
-- `BlockedBy` deed, not the per-blocker `Enact Block` (which flying/cant uses).
public export
menace : Ability b
menace = Keyword (Composite Menace [Static (cant (BlockedBy (SameAs This) (^1)))])

-- Haste ([CR#702.10]): a CONTINUOUS grant letting THIS attack and tap-activate "as though it had
-- been controlled continuously" — i.e. as though it weren't summoning-sick ([CR#302.6]). Built with
-- the AsThough machinery: pretend `Not (HasState SummoningSick)`, then `Can` the deed. (Grantable
-- via `GrantAbility (keyword Haste)` — e.g. Through the Breach. The doc spells haste as a flag the
-- summoning-sickness `cant` reads; the as-though framing is the dual, and the one the toy carries.)
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
  [Static (CantHappen (MkEventQuery [Destroy] [Patient (SameAs This)]))])

-- Devoid ([CR#702.114]): "this object is colorless" — a CDA, expressible now that an `Alter` op can CLEAR
-- a characteristic. `Alter Colors (Set [])` on This empties its color set (the Tarmogoyf-`*/*` pattern).
public export
devoid : Ability b
devoid = Keyword (Composite Devoid [Static (Modify This (Alter Colors (Set [])))])

-- "Regenerate this" ([CR#701.19]): a ONE-SHOT, this-turn shield — the next time This would be destroyed,
-- instead remove all damage, tap it, and remove it from combat. The `UpTo (^1)` limit consumes the
-- replacement after one destroy (vs `Replaces`'s default `Unlimited`).
public export
regenerate : OneShotEffect b
regenerate = Continuously UntilEndOfTurn
  (Replaces (MkEventQuery [Destroy] [Patient (SameAs This)])
            (Sequence [Act (RemoveAllDamage This), Act (Tap This), Act (RemoveFromCombat This)])
            {limit = UpTo (^1)})

-- transport a P-op to toughness: power and toughness share a value class (`CharValue _ Power = Count =
-- CharValue _ Toughness`) and both are `Numeric`, so the op carries over by definitional equality.
public export
ptTwin : ModificationOp b Power -> ModificationOp b Toughness
ptTwin (Set v)  = Set v
ptTwin (Up x)   = Up x
ptTwin (Down x) = Down x

-- "+x/+x" (any SYMMETRIC P/T mod): `ModifyPT` is no longer a primitive — the same `ModificationOp` on both
-- axes is just two `Alter`s. `modifyPT` doubles a P-op, returning a `List` ready to splice into `ApplyAll`:
-- "gets +3/+3" = `Modify This (ApplyAll (modifyPT (Up (^3))))`. Asymmetric "+2/-1" writes the two `Alter`s.
public export
modifyPT : ModificationOp b Power -> List (Modification b)
modifyPT op = [Alter Power op, Alter Toughness (ptTwin op)]

-- KEYWORD ACTIONS as macros over primitives ([CR#701]) — the action-side twin of the keyword
-- ABILITIES above. Each is a plain `OneShotEffect` (used directly, no `Act` wrapper), composited from
-- `Each`/`With`/`Modal`/`Move`/`ToLibrary`/`DealDamage`, so there are no bespoke `Scry`/`Surveil`/
-- `Fight` verbs in `Action`.

-- mill n ([CR#701.17a]): put the top n of your library into your graveyard. The graveyard is unordered,
-- so a simultaneous `Each` over the top-n needs no `Arrangement`.
public export
mill : Count b -> OneShotEffect b
mill n = Act (Composite Mill (Each (Existing (TopOfLibrary n)) (Act (Move It (ToZone Graveyard)))))

-- scry n ([CR#701.22a]): look at the top n, then put each on top or on the bottom; the within-group
-- order is the [CR#401.4] "any order" freebie (simultaneous `Each`). The per-card top/bottom pick is a
-- 1-of-2 `Modal`.
public export
scry : Count b -> OneShotEffect b
scry n = Act (Composite Scry
  (Each (Existing (TopOfLibrary n))
    (Modal (MkChooseSpec (Range (Just (^1)) (Just (^1))))
      [ MkMode (Act (Move It (ToLibrary (FromTop (^0)))))
      , MkMode (Act (Move It (ToLibrary (FromBottom (^0))))) ])))

-- surveil n ([CR#701.25a]): scry's shape, but the spill zone is the graveyard, not the library bottom.
public export
surveil : Count b -> OneShotEffect b
surveil n = Act (Composite Surveil
  (Each (Existing (TopOfLibrary n))
    (Modal (MkChooseSpec (Range (Just (^1)) (Just (^1))))
      [ MkMode (Act (Move It (ToLibrary (FromTop (^0)))))
      , MkMode (Act (Move It (ToZone Graveyard))) ])))

-- fight ([CR#701.14a]): two creatures each deal damage equal to their power to the other (simultaneous).
public export
fight : Reference b AnObject -> Reference b AnObject -> OneShotEffect b
fight x y = Act (Composite Fight
  (Sequence [ Act (DealDamage {source = x} y (StatOf x Power))
            , Act (DealDamage {source = y} x (StatOf y Power)) ]))

-- "Protection from [q]" ([CR#702.16]): the DEBT bundle, keyed to the quality `q` — can't be Damaged by
-- `q` sources, Enchanted/equipped by `q`, Blocked by `q`, or Targeted by `q`. ONE construct over the
-- existing `cant`/`ReplaceAmount` parts (the `Agent` facet — the damage source — for the D leg).
public export
protection : Predicate b AnObject -> Ability b
protection q = Keyword (Composite (Protection q)
  [ Static (ReplaceAmount (MkEventQuery [DealDamage Nothing] [Patient (SameAs This), Agent q]) (^0))   -- D
  , Static (cant (Enact Attach q (SameAs This)))        -- E
  , Static (cant (Enact Block q (SameAs This)))         -- B
  , Static (cant (Enact Target q (SameAs This))) ])     -- T

-- "Enchant [hosts]" ([CR#303.4],[CR#702.5]): NOT an engine keyword — a MACRO bundling the aura's per-card
-- behaviour, parameterised by the legal-host filter, spliced into `abilities` with `++`. (1) the PERMISSION
-- to attach (attaching is default-forbidden, so this ENABLES it — the dual of a planeswalker's `Can (Enact
-- Attack … This)`); (2) the non-cast ENTRY rule ([CR#303.4f]) — as This enters, if it isn't already
-- attached, choose a valid host and enter attached (`Also thisEnters`, the documented enters-attached
-- idiom; host chosen via `Choose`, read back as `That`). The `If (Not (LegallyAttached This))` guard is
-- what scopes this to NON-cast entry: a cast aura entered attached to its target (ability 3), so the
-- guard skips it. That guard is the SAME condition the falls-off SBA reads, so the two compose — choose a
-- host on entry, and if none is legal the SBA sweeps it. (3) the aura's SPELL — cast it targeting a valid
-- host, attach to that host on resolution. The falls-off SBA ("no valid attachment → graveyard",
-- [CR#704.5n]) is conferred by the Aura SUBTYPE (`subtypeConfers`), not here.
public export
enchant : {b : Endophora} -> ({0 c : Endophora} -> Predicate c AnObject) -> List (Ability b)
enchant hosts =
  [ Static (Can (Enact Attach (SameAs This) hosts))                                  -- (1) permission: the aura ENABLES attaching
  , Static (Also thisEnters (If (Not (LegallyAttached This))                         -- (2) [CR#303.4f] non-cast entry only (cast path is already attached):
              (With (ChooseOne hosts) (Act (Attach This That)))))                    --     choose a valid host, enter attached
  , Spell (Targeted [Target (^1) hosts] (Act (Attach This (GetTarget 0)))) ]         -- (3) cast → target a host → attach on resolution

-- desugar a `KeywordSpec` into its full `Ability` — dispatches to the macros above. EXHAUSTIVE
-- (no catch-all): adding a `KeywordSpec` constructor forces a clause here. `Bare` = an engine-
-- PRIMITIVE keyword the grammar can't desugar (FirstStrike/DoubleStrike/Deathtouch/Trample =
-- damage pipeline; Vigilance = attack event-edit). The rest are `Composite`: the deontic ones
-- carry a `cant`; `Reach` carries `[]` (just a flag `flying`'s clause reads); `Flash` carries a
-- `Can (Casts …) {window = AsInstant}` — you may cast it at instant speed ([CR#702.8a]).
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
keyword Flashback           = Keyword (Bare Flashback)  -- DEGENERATE (bare flashback carries no cost) — use the `flashback <cost>` macro for the real ability
keyword Dash                = Keyword (Bare Dash)        -- DEGENERATE (alt cost + rider ride a `dash <cost>` macro)
keyword Evoke               = Keyword (Bare Evoke)       -- DEGENERATE (use an `evoke <cost>` macro)
keyword Blitz               = Keyword (Bare Blitz)       -- DEGENERATE (use a `blitz <cost>` macro)
keyword Prowl               = Keyword (Bare Prowl)       -- DEGENERATE (alt cost + `when` guard ride a `prowl <cost>` macro)
keyword Spectacle           = Keyword (Bare Spectacle)   -- DEGENERATE (use a `spectacle <cost>` macro)
keyword Devoid              = devoid
keyword (Protection q)      = protection q
keyword (Banding mq)        = Keyword (Bare (Banding mq))  -- INTRINSIC; the engine bakes banding's combat rules off the tag
keyword Mutate              = Keyword (Bare Mutate)        -- DEGENERATE (alt cost + merge ride a `mutate <cost>` macro)

-- KEYWORD ACTIONS (composite verbs over the primitives — the Idris analogue of the engine's
-- keyword-action macros; named here rather than inlined per card).

-- "Monstrosity N" ([CR#701.37]): if THIS isn't monstrous, put N +1/+1 counters on it and it becomes
-- monstrous. An activated ability whose cost varies per card.
public export
monstrosity : Cost b -> Count b -> Ability b
monstrosity cost n = Activated cost
  (If (Matches This (Not (HasDesignation Monstrous)))
      (Sequence [ Act (PutCounters P1P1 n This), Act (GrantDesignation Monstrous This) ]))

-- "Level up [cost]" ([CR#702.87]): put a level counter on THIS; sorcery-speed only.
public export
levelUp : Cost b -> Ability b
levelUp cost = Activated cost (Act (PutCounters Level (^1) This)) {window = AsSorcery}

-- "Crew N" ([CR#702.122]): tap any creatures with total power ≥ N → this Vehicle becomes an artifact
-- creature until end of turn. The aggregate-tap cost is `TapTotal`.
public export
crew : Count b -> Ability b
crew n = Activated (TapTotal Power AtLeast n creature)
  (Continuously UntilEndOfTurn (Modify This (Alter Types (Add Creature))))

-- "Morph [cost]" ([CR#702.37]): you may cast this face down as a 2/2 for {3} (`CastFaceDown`), and turn
-- it face up any time for [cost] (`TurnFaceUp`). The 2/2-colorless-vanilla face-down body is the global
-- [CR#708.2] rule (engine-applied on the `FaceDown` state), not restated here.
public export
morph : Cost b -> Ability b
morph c = Keyword (Composite Morph [ Static (CastFaceDown (Mana [^3])), TurnFaceUp c ])

-- "Flashback [cost]" ([CR#702.34]): you may cast this from your graveyard for [cost] rather than its mana
-- cost (`MayCastFor … {from = [Graveyard]}`); the engine exiles it afterward off the `Flashback` tag
-- ([CR#702.34a]), so that rider isn't restated here (cf. `morph`/[CR#708.2]).
public export
flashback : List (Cost b) -> Ability b
flashback costs = Keyword (Composite Flashback [ Static (MayCastFor costs {from = [Graveyard]}) ])

-- the number of distinct card TYPES among cards in all graveyards (Tarmogoyf's `*`) — directly the
-- DISTINCT-union count the value-language now provides, replacing the old hand-rolled sum of per-type
-- `Min (CountOf (graveyard ∧ that type)) 1` indicators over an enumerated card-type list.
public export
typesInGraveyards : Count b
typesInGraveyards = CountDistinct Types (Objects (InZone Graveyard))

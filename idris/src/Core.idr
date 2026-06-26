||| Core grammar of the toy MTG card model: characteristics, the `Bindings`
||| typestate (what references are in scope), and the filter / reference /
||| selection / action / effect / ability trees. Kept deliberately brief.
module Core

import public Data.Vect
import public Data.Nat
import public Data.List
import public Data.List.Elem
import public Data.List.Quantifiers

public export
data Color
  = White
  | Blue
  | Black
  | Red
  | Green

public export
Colorless : Maybe Color
Colorless = Nothing

public export
data SimpleManaSymbol
  = Generic Nat
  | Specific (Maybe Color)

public export
-- the PRINTED cost language ([CR#107.4]) — what appears on a card as a mana cost. NOT what a mana
-- ability produces (that's `ProducedMana` below — a different domain; the user's distinction).
data ManaSymbol
  = Simple SimpleManaSymbol
  | Hybrid SimpleManaSymbol Color
  | Variable
  | Phyrexian Color (Maybe Color)  -- "{W/P}" = `Phyrexian White Nothing` (pay the color OR 2 life); a HYBRID Phyrexian "{G/U/P}" = `Phyrexian Green (Just Blue)` is both component colors ([CR#107.4f])
  | SnowMana                  -- "{S}" — one mana from a snow source ([CR#107.4h]); `SnowMana`, not `Snow` (the supertype)

-- `Promote a b` (method `promote`) is the toy's value-injection interface — formerly Prelude's
-- `Cast`/`cast`, renamed so the precious MTG words `cast`/`Cast` stay free for actual casting.
public export
interface Promote a b where
  promote : a -> b

-- `^x` — a terse PREFIX alias for `promote x` (e.g. `^Red`, `^2`). (`~` would mirror the
-- self-reference sigil but is reserved for Delay/Force; `^` is free — no infix `^` in base.)
export prefix 10 ^
public export
(^) : Promote a b => a -> b
(^) = promote

public export
implementation Promote Nat ManaSymbol where
  promote = Simple . Generic

public export
implementation Promote Integer ManaSymbol where
  promote = promote . integerToNat

public export
implementation Promote Color ManaSymbol where
  promote = Simple . Specific . Just

-- `^Colorless` = {C} (and `^(Just c)` = {c}); `Specific Nothing` is the colorless pip.
public export
implementation Promote (Maybe Color) ManaSymbol where
  promote = Simple . Specific

-- PRODUCED mana ([CR#106.1]) — actual mana a mana ability adds. A DIFFERENT domain from the printed
-- cost `ManaSymbol`: you produce colored/colorless units or "any color", never `{X}`/`{W/P}`/`{S}`.
public export
data ProducedMana = OfColor (Maybe Color)   -- `OfColor (Just c)` = one {c}; `OfColor Nothing` = one {C}
                  | AnyColor                 -- one mana of any color (the producer picks)
                  | OneOf (List (Maybe Color)) -- one mana, the producer choosing from a FIXED set ("add {W} or {U}" = `OneOf [Just White, Just Blue]`); distinct from `AnyColor` (all five) and from a heterogeneous list (add ALL) ([CR#106.1])

public export
implementation Promote Color ProducedMana where
  promote = OfColor . Just

public export
implementation Promote (Maybe Color) ProducedMana where
  promote = OfColor

public export
ManaCost : Type
ManaCost = List ManaSymbol

public export
data Type_
  = Artifact
  | Battle
  | Creature
  | Enchantment
  | Instant
  | Kindred
  | Land
  | Planeswalker
  | Sorcery

public export
data Zone
  = Battlefield
  | Command
  | Exile
  | Graveyard
  | Hand
  | Library
  | Stack

-- Subtypes are partitioned by card type [CR#205.3g..205.3q]; each belongs to
-- exactly one card type. `subtypeCategory` is that (total) correlation.
public export
data CreatureSubtype
  = Bear | Rat | Spider | Human | Knight | Goblin | Elf | Zombie | Elemental | Wall | Spirit
  | Rogue | Warrior | Merfolk | Wizard | Juggernaut | Angel | Faerie | Insect | Cat | Vampire | Noble  -- creature types
public export
data EnchantmentSubtype
  = Aura | Saga
public export
data ArtifactSubtype
  = Equipment | Vehicle
public export
data LandSubtype
  = Plains | Island | Swamp | Mountain | Forest   -- the basic land types
public export
data BattleSubtype
  = Siege

public export
data Subtype
  = CreatureSub CreatureSubtype
  | EnchantmentSub EnchantmentSubtype
  | ArtifactSub ArtifactSubtype
  | LandSub LandSubtype
  | BattleSub BattleSubtype

public export
implementation Promote CreatureSubtype Subtype where
  promote = CreatureSub
public export
implementation Promote EnchantmentSubtype Subtype where
  promote = EnchantmentSub
public export
implementation Promote LandSubtype Subtype where
  promote = LandSub
public export
implementation Promote ArtifactSubtype Subtype where
  promote = ArtifactSub
public export
implementation Promote BattleSubtype Subtype where
  promote = BattleSub

public export
subtypeCategory : Subtype -> Type_
subtypeCategory (CreatureSub _) = Creature
subtypeCategory (EnchantmentSub _) = Enchantment
subtypeCategory (ArtifactSub _) = Artifact
subtypeCategory (LandSub _) = Land
subtypeCategory (BattleSub _) = Battle

-- Leaf types used inside the filter/condition language ---------------------

public export
data Stat = Power | Toughness | ManaValue | Defense

-- how to FOLD a `Stat` over a predicate-matched SET of objects (`Aggregate`, a `Count`): the SUM, the
-- GREATEST, or the LEAST. Over the empty set all three are 0 ("the greatest power among" no creatures = 0),
-- an engine detail. (Cardinality is `CountOf`, not an `AggOp`; folding event amounts over time is `EventSum`.)
public export
data AggOp = Total | Greatest | Least

public export
data Cmp = Equal | GreaterEq | LessEq | Greater | Less

-- What kind of object a filter matches ([CR#109.3]). Rust: ObjectKind.
public export
data ObjectKind = IsCard | IsEmblem | IsSpell | IsToken | IsAbility

-- Supertypes ([CR#205.4a]); independent of card type and subtype.
public export
data Supertype = Basic | Legendary | Ongoing | Snow | World

-- The word classes a TEXT-CHANGE effect may swap ([CR#612.1]): a color word (white/blue/…) or a basic
-- land type (Plains/Island/…). Mind Bend allows either; the specific words are a player's choice.
public export
data TextWordClass = ColorWords | BasicLandTypes

-- A kind of counter ([CR#122]). The TYPE is `CounterKind` — bare `Counter` is taken by the spell-
-- countering `Action`. A CLOSED set (curated — NOT an open name+registry like the Rust engine, which
-- needs that for plugins); the carrier (object vs player) is the total function `counterScope`
-- below, which indexes the counter ops dependently. Counter kinds are engine-OPEN but grammar-CURATED,
-- the SAME family as `Designation` and `KeywordSpec` (each a curated enum + a dependent scope fn) — so
-- coverage grows by adding curated entries here as the canon needs them, never by an open string hatch.
public export
data CounterKind = Loyalty | Fate | Charge | P1P1 | M1M1 | Level | Lore | Stun | Shield
                 | Poison | Energy | Experience

-- A timing WINDOW — the speed at which an action is allowed: `AsInstant` (any time you have
-- priority) or `AsSorcery` (your main phase, empty stack — [CR#601.3,602.5d]). The ONE timing
-- notion, shared by a deontic `Can (Casts …)` (Flash widens to `AsInstant`, [CR#702.8a]) and
-- by `Activated` (instant by default; "activate only as a sorcery" narrows to `AsSorcery`).
public export
data Timing = AsInstant | AsSorcery

-- Activation USE-LIMITS on an activated ability ([CR#602.5b]) — frequency caps, NOT timing (that's
-- `Timing` above; the two used to overlap on a `SorcerySpeed` constructor). A loyalty ability
-- is `{window = AsSorcery, limits = [OncePerTurn]}`.
public export
data UsageLimit = OncePerTurn | OncePerGame

-- Runtime object STATE (not a printed characteristic) — what a `HasState` predicate tests
-- ([CR#701.20] tap, [CR#302.6] summoning sickness, [CR#702.26] phasing, [CR#708] face-down). The RELATIONAL
-- states moved to the spine: combat (attacking/blocking/blocked) is `Holds Attack/Block Agent/Patient`, and
-- "attached" is `Holds Attach Agent` — none are `HasState`. Negatives via `Not` ("untapped" = `Not (HasState
-- Tapped)`). `SummoningSick` is what `haste` lifts — "as though not summoning-sick" (`AsThough`, see Macros).
public export
data ObjectState = Tapped | SummoningSick
                 | PhasedOut       -- phased out ([CR#702.26]); "becomes phased" = `Becomes PhasedOut`
                 | FaceDown        -- face down ([CR#708]); the engine applies the global 2/2-colorless-vanilla override here

-- which `ObjectState`s an object TRANSITIONS into as a game event (gates `Becomes`). `SummoningSick`
-- isn't one — it's a derived continuous condition `haste` lifts, never a "becomes" event. (`IsCharDomain` idiom.)
public export
IsBecomesState : ObjectState -> Type
IsBecomesState SummoningSick = Void
IsBecomesState _             = ()

-- Whether a `Reference` denotes an object or a player ([CR#109.1]). One reference
-- language, indexed by this — strict on the kind where it matters, lax where it doesn't.
-- `Anything` is the union kind for "any target" ([CR#115.4]) — an object OR a player;
-- only lax ops (damage) accept it, so it can't be read as a definite object/player.
public export
data RefKind = Empty | AnObject | APlayer | Anything

-- The JOIN on `RefKind` (least upper bound): `Empty` is the identity (bottom),
-- like-with-like is itself, two distinct kinds widen to `Anything` (the top) —
-- so `(RefKind, \/, Empty)` is a bounded join-semilattice. `Or` folds it
-- over its arms' kinds (base `Empty`) to COMPUTE a union's kind — what retires
-- `Widen`; an empty union folds to `Empty` (a vacuous predicate, matches
-- nothing).
public export
(\/) : RefKind -> RefKind -> RefKind
(\/) Empty x = x
(\/) x Empty = x
(\/) AnObject AnObject = AnObject
(\/) APlayer APlayer = APlayer
(\/) _ _ = Anything

-- the CARRIER of a counter ([CR#122.1]): most are object-borne; poison/energy/experience are borne by
-- PLAYERS. This indexes the counter ops dependently — `PutCounters Poison n You` typechecks and
-- `PutCounters Poison n <object>` does not, with no runtime check. (Players-are-objects: the `Reference`
-- language already names players, so a player-carried counter needs no new machinery, just this kind.)
public export
counterScope : CounterKind -> RefKind
counterScope Poison     = APlayer
counterScope Energy     = APlayer
counterScope Experience = APlayer
counterScope _          = AnObject

-- RELATION SPINE. A `Relation` is an agent→patient relation the game tracks; from ONE relation we derive
-- three ASPECTS — durative (`Holds`, a state predicate), inchoative (`Begins`, an event), deontic (`Enact`,
-- a deed). `agentScope` fixes the AGENT's kind per relation (the Agent/Actor resolution: ONE agent slot, an
-- OBJECT for combat/attach/target/counter, a PLAYER for cast/activate/play) — `counterScope`'s sibling.
-- The PATIENT stays kind-poly (an attack's defender is a player/planeswalker/battle). The constructors are
-- NAMESPACED — `Target`/`Counter`/`Attach` clash with the TargetSpec/Action of the same
-- name — and disambiguate by type, like `Facet.Patient`/`Role.Patient` share `Patient`.
namespace Relation
  public export
  data Relation = Attack | Block            -- combat
                | Cast | Activate | Play     -- the stack: a PLAYER casts a spell / activates an ability / plays a card
                | Attach                      -- an aura/equipment (object) attaches to a host
                | Target | Counter           -- a source (spell/ability, object) targets / counters an object

public export
agentScope : Relation -> RefKind
agentScope Attack   = AnObject
agentScope Block    = AnObject
agentScope Cast     = APlayer
agentScope Activate = APlayer
agentScope Play     = APlayer
agentScope Attach   = AnObject
agentScope Target   = AnObject   -- the source (a spell/ability) does the targeting
agentScope Counter  = AnObject   -- the source (a spell/ability) does the countering

-- the two participant SLOTS, as role selectors for the durative aspect (`Holds Attack Agent` = an attacker,
-- `Holds Block Patient` = a blocked creature). `Agent`/`Patient` are the SAME role pair the event `Facet`s
-- use — ONE vocabulary across the spine's aspects. (`Actor`, the responsible PLAYER, is a separate axis,
-- not a role.) Unifies the old `Attacking`/`Blocking`/`Blocked` states.
public export
data Role = Agent | Patient

-- DESIGNATIONS (the 700-ish global flags: monarch, the initiative, city's blessing, monstrous,
-- goaded, renowned, suspected, saddled, solved…). The Rust engine carries these as an OPEN name +
-- a runtime `Decl` whose `scope` field says object/player/game — needed for plugins. The curated toy
-- uses a CLOSED enum + a total `designationScope`, so ONE `HasDesignation`/`GrantDesignation` pair
-- covers every flag with the carrier (player vs object) enforced dependently — no runtime scope check.
public export
data Designation = Monarch | TheInitiative | CitysBlessing      -- player-borne
                 | Monstrous | Goaded | Renowned | Suspected | Saddled | Solved   -- object-borne

public export
designationScope : Designation -> RefKind
designationScope Monarch       = APlayer
designationScope TheInitiative = APlayer
designationScope CitysBlessing = APlayer
designationScope _             = AnObject

public export
data BeginningStep
  = UntapStep
  | UpkeepStep
  | DrawStep

public export
data CombatStep
  = BeginningOfCombatStep
  | DeclareAttackersStep
  | DeclareBlockersStep
  | FirstCombatDamageStep
  | CombatDamageStep
  | EndOfCombatStep

public export
data EndingStep
  = EndStep
  | CleanupStep

-- a turn has exactly TWO main phases ([CR#505.1]) — a closed enum, not an open `Nat`.
public export
data MainPhaseKind = PreCombat | PostCombat

public export
data PhaseStep
  = BeginningPhase BeginningStep
  | MainPhase MainPhaseKind
  | CombatPhase CombatStep
  | EndingPhase EndingStep

public export
implementation Promote BeginningStep PhaseStep where
  promote = BeginningPhase
public export
implementation Promote CombatStep PhaseStep where
  promote = CombatPhase
public export
implementation Promote EndingStep PhaseStep where
  promote = EndingPhase

-- A history-lookback / timing scope for an `EventQuery`. Rust: Window.
public export
data Window = ThisGame | ThisTurn | LastTurn | ThisCombat | ThisStep

-- What KIND of event an `EventQuery` matches. `ZoneChanged`/`BeginStep` carry data; "dies" =
-- ZoneChanged (Just Battlefield) (Just Graveyard). The verb-named events live in `namespace
-- EventKind` so they REUSE the `Action` verb names — a `kinds` slot `[Draw]` pins `EventKind`, `Act (Draw …)`
-- pins `Action` (type-directed disambiguation; no more past-tense `Drew`/`DealtDamage`).
namespace EventKind
  public export
  data EventKind : Type where
    Sacrifice : EventKind
    Draw : EventKind
    Discard : EventKind
    -- damage ([CR#120]). The `Maybe Bool` is the COMBAT flag — intrinsic event data, so it rides the
    -- KIND (like `ZoneChanged`'s zones), wildcarded the same way: `Nothing` = any damage (Furnace,
    -- protection, prevention), `Just True` = combat damage ([CR#510]), `Just False` = noncombat.
    DealDamage : Maybe Bool -> EventKind
    CreateToken : EventKind
    PutCounters : EventKind
    RemoveCounters : EventKind
    Destroy : EventKind
    ZoneChanged : Maybe Zone -> Maybe Zone -> EventKind
    BeginStep : PhaseStep -> EventKind
    -- "whenever ~ BECOMES [state]" — TRANSITION states only (gated; not `SummoningSick`).
    Becomes : (s : ObjectState) -> {auto prf : IsBecomesState s} -> EventKind
    -- the ONSET of a relation ([CR#508],[CR#509]) — the inchoative aspect of the relation spine. `Begins
    -- Attack` fires once per attack; FACETS pick the side (`[Agent This]` = it attacks, `[Patient you]` =
    -- you're attacked). Unifies the paired `Becomes Attacking`/`Becomes Attacked` (one happening, two
    -- views). The STACK relations fold in too: a cast is `Begins Cast` (no bespoke `Cast` kind — it was
    -- redundant), the caster supplied as its actor (the `agentScope`-driven caps below).
    Begins : Relation -> EventKind

-- the per-event CAPABILITIES an event provides its body's anaphora: a distinguished OBJECT ("that card"),
-- an ACTOR ("that player"), a numeric AMOUNT. Read by `EventObject`/`EventActor`/`ThatMuch` so each is
-- valid ONLY where the event actually supplies it — the invalid-reference gate.
public export
record EventCaps where
  constructor MkCaps
  hasObject : Bool
  hasActor  : Bool
  hasAmount : Bool

public export
noCaps : EventCaps
noCaps = MkCaps False False False

-- what each event-kind supplies. Damage/token/counter carry an amount; a step-begin carries nothing; a
-- zone-change/destroy/becomes has an object but no actor; a cast/draw/discard/sacrifice has an actor.
public export
eventKindCaps : EventKind -> EventCaps
eventKindCaps Sacrifice         = MkCaps True  True  False
eventKindCaps Draw              = MkCaps False True  False
eventKindCaps Discard           = MkCaps True  True  False
eventKindCaps (DealDamage _)    = MkCaps True  True  True
eventKindCaps CreateToken       = MkCaps True  True  True
eventKindCaps PutCounters       = MkCaps True  True  True
eventKindCaps RemoveCounters    = MkCaps True  True  True
eventKindCaps Destroy         = MkCaps True  False False
eventKindCaps (ZoneChanged _ _) = MkCaps True  False False
eventKindCaps (BeginStep _)     = MkCaps False False False
eventKindCaps (Becomes _)       = MkCaps True  False False
-- a relation-ONSET supplies the agent's player as "that player" ONLY when the agent IS a player
-- (cast/activate/play); an object-agent onset (combat/attach/target/counter) reaches the controller via
-- `ControlledBy`. There is always a distinguished object, never an amount.
eventKindCaps (Begins r)        =
  case agentScope r of
    APlayer => MkCaps True True  False
    _       => MkCaps True False False

-- which event-kinds carry an AMOUNT (gates `ReplaceAmount`/`EventSum`) — derived from the caps.
public export
eventKindHasAmount : EventKind -> Bool
eventKindHasAmount k = hasAmount (eventKindCaps k)

-- A VALUE-choice DOMAIN: what an as-enters "choose …" picks from when the pick is a VALUE, not a game
-- entity ([CR#614.12]). The chosen value is bound in `Bindings.chosenKind` and read back by `OfChosen`
-- (characteristic domains) / `ChosenIs` (mode) / `ChosenNumber`. Characteristic domains (color / creature
-- type / name) name something an object can HAVE; a mode/number domain won't. Choosing a game ENTITY (a
-- creature to copy, a player) is NOT a value — it's a filtered `AsEntersChoosing` binding `chosenRefKind`.
public export
data ChooseDomain = AColor | ACreatureType | AMode Nat | AName | ANumber   -- `AMode n` = an n-way mode pick; AName = a card name; ANumber = a number

-- a mode domain must offer ≥1 mode ([CR#700.2]) — gates `AsEnters` (not `AMode` itself, which stays a
-- plain constructor so `ChosenIs`'s `AMode n` equality keeps working). Lenient for non-mode domains.
public export
ModeDomainOk : ChooseDomain -> Type
ModeDomainOk (AMode n) = LT 0 n
ModeDomainOk _         = ()

-- which domains name a CHARACTERISTIC `OfChosen` can test on an object — the gate on that anaphor (a
-- mode choice is not a characteristic — it gates abilities via `ChosenIs`; no choice can't be read).
-- Total over the domain.
public export
IsCharDomain : Maybe ChooseDomain -> Type
IsCharDomain (Just AColor)        = ()
IsCharDomain (Just ACreatureType) = ()
IsCharDomain (Just AName)         = ()   -- "has the chosen NAME" is an `OfChosen` test (Meddling Mage)
IsCharDomain (Just (AMode _))     = Void
IsCharDomain (Just ANumber)       = Void  -- a number isn't a characteristic — read it with `ChosenNumber`
IsCharDomain Nothing              = Void

-- which `RefKind`s an as-enters ENTITY choice (`AsEntersChoosing`) may pick: a game object (Clone's
-- creature) or a player ("choose a player"). `Empty`/`Anything` are not pickable entities. The gate that
-- makes `chosenRefKind` only ever hold an object or a player — the `ChosenObject`/`ChosenPlayer` twins.
public export
ChoiceRefKind : RefKind -> Type
ChoiceRefKind AnObject = ()
ChoiceRefKind APlayer  = ()
ChoiceRefKind _        = Void

-- `Bindings`: the typestate of what references are in scope. Its fields are
-- PROJECTIONS we write constraints against; it grows as the model binds roles.
public export
record Bindings where
  constructor MkBindings
  targetKinds : List RefKind     -- one `RefKind` per target slot (the slot's kind, from its filter)
  thatKind    : Maybe RefKind    -- a `With`-bound group's element kind (`That`), if bound
  itKind      : Maybe RefKind    -- a `Each`-bound element's kind (`It`), if bound
  evCaps      : EventCaps    -- the surrounding event's caps (`noCaps` outside an event body) — gates `EventObject`/`EventActor`/`ThatMuch`
  chosenKind  : Maybe ChooseDomain  -- an as-enters "choose …" VALUE (color/type/name/number/mode) in scope (`OfChosen`/`ChosenIs`/`ChosenNumber`), if bound
  chosenRefKind : Maybe RefKind  -- an as-enters "choose …" GAME ENTITY (an object/player) in scope (`ChosenObject`/`ChosenPlayer`), if bound — the identity twin of `chosenKind`
  hasAllotment : Bool    -- inside a `Distribute` body: a per-element share is in scope (gates `Allotment`)

-- The bindings a resolving spell starts in: nothing bound yet.
public export
Base : Bindings
Base = MkBindings [] Nothing Nothing noCaps Nothing Nothing False

-- Each sets one field, reconstructing `MkBindings` explicitly so a projection of a
-- bind result reduces definitionally even for abstract `b` (record-update sugar
-- has no get-after-set law for an abstract record).
public export
bindTargets : List RefKind -> Bindings -> Bindings
bindTargets ks b = MkBindings ks (thatKind b) (itKind b) (evCaps b) (chosenKind b) (chosenRefKind b) (hasAllotment b)

public export
unbindTargets : Bindings -> Bindings
unbindTargets b = MkBindings [] (thatKind b) (itKind b) (evCaps b) (chosenKind b) (chosenRefKind b) (hasAllotment b)

public export
bindThat : RefKind -> Bindings -> Bindings
bindThat k b = MkBindings (targetKinds b) (Just k) (itKind b) (evCaps b) (chosenKind b) (chosenRefKind b) (hasAllotment b)

public export
bindIt : RefKind -> Bindings -> Bindings
bindIt k b = MkBindings (targetKinds b) (thatKind b) (Just k) (evCaps b) (chosenKind b) (chosenRefKind b) (hasAllotment b)

-- entering a trigger/replacement/delayed body, carrying the event's CAPS (what anaphora it supplies).
public export
bindEvent : EventCaps -> Bindings -> Bindings
bindEvent caps b = MkBindings (targetKinds b) (thatKind b) (itKind b) caps (chosenKind b) (chosenRefKind b) (hasAllotment b)

-- the as-enters VALUE choice ([CR#614.12]): binds `chosenKind` (a color/type/name/number/mode) for the
-- whole card's abilities.
public export
bindChosen : ChooseDomain -> Bindings -> Bindings
bindChosen d b = MkBindings (targetKinds b) (thatKind b) (itKind b) (evCaps b) (Just d) (chosenRefKind b) (hasAllotment b)

-- the as-enters GAME-ENTITY choice ([CR#614.12]): binds `chosenRefKind` (a chosen object/player) for the
-- whole card's abilities — the identity-reference twin of `bindChosen`, opened by `AsEntersChoosing`.
public export
bindChosenRef : RefKind -> Bindings -> Bindings
bindChosenRef k b = MkBindings (targetKinds b) (thatKind b) (itKind b) (evCaps b) (chosenKind b) (Just k) (hasAllotment b)

-- a `Distribute` body ([CR#601.2d] division): binds the loop element `It` of kind k AND marks a per-element
-- share in scope (read back by `Allotment`). The allotment-bearing twin of `bindIt`.
public export
bindAllot : RefKind -> Bindings -> Bindings
bindAllot k b = MkBindings (targetKinds b) (thatKind b) (Just k) (evCaps b) (chosenKind b) (chosenRefKind b) True

-- KeywordSpec / Reference / Count / Predicate / Condition / EventQuery are one mutually
-- recursive language. A PREDICATE is an object test — its candidate is IMPLICIT. A `Condition`
-- is a closed/game-state test reaching objects via `Matches`/`exists`/`unique`. `Predicate`,
-- `Condition`, and `EventQuery` SHARE the combinator names `And`/`Or`/`Not` — each in its own
-- `namespace`, resolved by the expected type at the use site (no `AllOf`/`Query` aliasing).
mutual
  -- A KEYWORD's tag + params ([CR#702]) — the "name" side of a keyword. In this block so
  -- `HasKeyword` can read it and `Hexproof`'s "from" filter can be a `Predicate` (which may name
  -- an anaphor — "from the CHOSEN color"). `keyword` (Macros) desugars a spec into its full `Ability`
  -- (a `Composite`): the deontic ones (Flying/Defender/Shroud/Hexproof/Menace) get a `cant` (Menace's
  -- is the SET-level `BlockedBy`); the rest (FirstStrike/Deathtouch/Trample = damage; Vigilance =
  -- event-edit; Reach/Flash = flag/window) carry no clause.
  public export
  data KeywordSpec : Bindings -> Type where
    Flying : KeywordSpec b
    FirstStrike : KeywordSpec b
    DoubleStrike : KeywordSpec b
    Deathtouch : KeywordSpec b
    Reach : KeywordSpec b
    Trample : KeywordSpec b
    Vigilance : KeywordSpec b
    Flash : KeywordSpec b
    Haste : KeywordSpec b
    Indestructible : KeywordSpec b
    Defender : KeywordSpec b
    Shroud : KeywordSpec b
    Menace : KeywordSpec b
    Hexproof : Maybe (Predicate b AnObject) -> KeywordSpec b   -- "from [filter]" — a SOURCE predicate (objects); "from a player" = ControlledBy that player
    Morph : KeywordSpec b   -- the tag for the `morph` macro ([CR#702.37]); the face-up cost rides its desugared `TurnFaceUp` (bare here — `KeywordSpec` precedes `Cost`)
    Devoid : KeywordSpec b  -- "this object is colorless" ([CR#702.114]) — a CDA; desugars to `Set Colors []` on This
    Protection : Predicate b AnObject -> KeywordSpec b   -- "protection from [quality]" ([CR#702.16]) — the tag carries q; desugars to the DEBT bundle (`protection` macro)
  -- A REFERENCE to a single game entity, indexed by `RefKind` (object vs player). One
  -- reference language now: object-refs and player-refs together, strict on the kind
  -- where it matters (`StatOf` needs `AnObject`, `LifeTotal` needs `APlayer`) and lax
  -- where it doesn't (`SameAs`, damage). A target's kind FLEXES — `AnObject` by default,
  -- `APlayer` where a player op forces it.
  public export
  data Reference : Bindings -> RefKind -> Type where
    -- the source object; always available [CR#113.7].
    This : Reference b AnObject
    -- demote a `Selection` to its SOLE element. Partial — the author asserts singularity, exactly
    -- like `Only` (undefined on a 0- or 2+-element set). `GetTarget`/`Only` are sugar over it.
    Single : Selection b k -> Reference b k
    -- the host this is attached to ("enchanted creature"); and its inverse.
    AttachHostOf : Reference b AnObject -> Reference b AnObject
    AttachedTo : Reference b AnObject -> Reference b AnObject
    -- the current element ("it"): the `Each`-bound loop element OR the `Modify`-bound per-subject
    -- object (an anthem's candidate); its kind is the binder's (`itKind`). Serves as the "Subject" an
    -- anthem's mods read, without a dedicated reference — predicates are already candidate-implicit.
    It : {auto prf : itKind b = Just k} -> Reference b k
    -- the triggering event's object ("that card") — valid only if the event SUPPLIES one ([CR#608.2k]).
    EventObject : {auto prf : hasObject (evCaps b) = True} -> Reference b AnObject
    -- PLAYERS (the old `PlayerRef`, folded in here):
    You : Reference b APlayer                            -- controller of this ability [CR#109.5]
    ControllerOf : Reference b AnObject -> Reference b APlayer   -- the controller of an object
    OwnerOf : Reference b AnObject -> Reference b APlayer        -- the owner of an object [CR#108.3]
    EventActor : {auto prf : hasActor (evCaps b) = True} -> Reference b APlayer  -- the event's player ("that player") — only if supplied
    ChosenPlayer : {auto prf : chosenRefKind b = Just APlayer} -> Reference b APlayer  -- the as-enters chosen PLAYER (the identity-reference twin of OfChosen/ChosenNumber); opened by `AsEntersChoosing APlayer …`
    ChosenObject : {auto prf : chosenRefKind b = Just AnObject} -> Reference b AnObject  -- the as-enters chosen OBJECT (the object-twin of `ChosenPlayer`; Clone copies it via `BecomeCopyOf ChosenObject`); opened by `AsEntersChoosing AnObject …`

  -- A numeric value ([CR#107.3]). `Literal` is a bare number; the rest read the game
  -- state — object counts, stats, counters, life/hand totals, event tallies, arithmetic.
  public export
  data Count : Bindings -> Type where
    Literal : Nat -> Count b                  -- a bare number
    X : Count b                               -- the chosen {X} value
    CountOf : Predicate b k -> Count b        -- how many entities match a predicate
    StatOf : Reference b AnObject -> Stat -> Count b     -- an object's power/toughness/etc.
    -- a `Stat` FOLDED over the SET of objects matching a predicate, per `AggOp` ("greatest power among
    -- creatures you control" = `Aggregate Greatest Power (And [creature, ControlledBy you])`). The set-twin
    -- of `StatOf` (one object) and the spatial-twin of `EventSum` (event amounts over a window).
    Aggregate : AggOp -> Stat -> Predicate b AnObject -> Count b
    Devotion : (colors : List Color) -> {auto prf : NonEmpty colors} -> Count b   -- devotion: pips of these (≥1) colors among your permanents
    EventCount : EventQuery b -> Count b      -- how many matching events occurred (window is in the query)
    -- the SUM of the matching events' amounts (the amount-twin of `EventCount`). Takes the amount-bearing
    -- KIND explicitly (gated by `eventKindHasAmount`, so `EventSum (Begins Cast)` is rejected) + optional facets.
    EventSum : (k : EventKind) -> {auto amt : eventKindHasAmount k = True} -> {default [] facets : List (Facet b)} -> Count b
    Damage : Reference b AnObject -> Count b  -- marked damage on r ([CR#120.3]); the lethal-damage SBA reads `Compare (Damage This) GreaterEq (StatOf This Toughness)`
    CountersOn : (c : CounterKind) -> Reference b (counterScope c) -> Count b   -- number of [kind] counters on r (object or player, per `counterScope`)
    LifeTotal : Reference b APlayer -> Count b           -- a player's life total
    HandSize : Reference b APlayer -> Count b            -- cards in a player's hand
    Plus  : Count b -> Count b -> Count b                -- arithmetic on values
    Minus : Count b -> Count b -> Count b
    Times : Count b -> Count b -> Count b
    HalfUp : Count b -> Count b                          -- "half, rounded up"
    HalfDown : Count b -> Count b
    Min : Count b -> Count b -> Count b                  -- the lesser ([CR#704.5q] +1/+1 vs −1/−1 annihilation; "the lesser of X and Y")
    Max : Count b -> Count b -> Count b                  -- the greater
    ThatMuch : {auto prf : hasAmount (evCaps b) = True} -> Count b   -- the event's amount — valid only where the event SUPPLIES one
    Allotment : {auto prf : hasAllotment b = True} -> Count b   -- inside a `Distribute`: the share allotted to the current element ([CR#601.2d])
    ChosenNumber : {auto prf : chosenKind b = Just ANumber} -> Count b   -- the as-enters chosen NUMBER (the value-anaphor twin of OfChosen/ChosenIs)

  -- A PREDICATE: a test on a single IMPLICIT candidate object — i.e. a *filter*.
  -- The atoms read the candidate's characteristics; `SameAs r` tests identity.
  namespace Predicate
    public export
    data Predicate : Bindings -> RefKind -> Type where
      HasType : Type_ -> Predicate b AnObject
      HasSupertype : Supertype -> Predicate b AnObject
      HasSubtype : Subtype -> Predicate b AnObject
      HasColor : Color -> Predicate b AnObject
      IsKind : ObjectKind -> Predicate b AnObject
      InZone : Zone -> Predicate b AnObject
      HasKeyword : KeywordSpec b -> Predicate b AnObject
      SameAs : Reference b k -> Predicate b k    -- the candidate IS r (same kind; "another" = Not (SameAs This))
      SameName : Reference b AnObject -> Predicate b AnObject   -- shares a name with r ("named [its own name]" = SameName This)
      SharesSubtype : Reference b AnObject -> Predicate b AnObject   -- shares ≥1 subtype with r (Coat of Arms: "shares a creature type with It")
      WasCastFrom : Zone -> Predicate b AnObject -- the object was cast from this zone (cast provenance)
      ExiledBy : Reference b AnObject -> Predicate b AnObject   -- set aside by r's effect ("cards exiled by this" = ExiledBy
                                                 -- This); the engine holds the association ([CR#607] linked abilities)
      DamagedBy : Reference b AnObject -> Predicate b AnObject  -- was dealt damage by r THIS TURN ("a creature dealt damage
                                                 -- by ~ this turn" = And [creature, DamagedBy This]); engine-held, like ExiledBy. Turn-scoped reset is the engine's.
      HasName : String -> Predicate b AnObject   -- named a specific card (tutors / token names)
      HasCounter : (c : CounterKind) -> Predicate b (counterScope c)   -- has ≥1 of this counter; the candidate's kind follows the carrier ("ten poison" tests a player)
      HasState : ObjectState -> Predicate b AnObject      -- runtime state: "target ATTACKING / TAPPED creature"
      -- the DURATIVE aspect of the relation spine: "the candidate currently fills [role] of [r]" — object-only
      -- (only objects bear durative state; a player defender has none). `Holds Attack Agent` = an attacker,
      -- `Holds Block Patient` = a blocked creature. Unifies the legacy `Attacking`/`Blocking`/`Blocked` states.
      Holds : Relation -> Role -> Predicate b AnObject
      -- carries a DESIGNATION; the candidate's kind follows `designationScope` ("you're the monarch" =
      -- `HasDesignation Monarch` is a player test, "while ~ is monstrous" an object test).
      HasDesignation : (d : Designation) -> Predicate b (designationScope d)
      -- a numeric STAT comparison on the candidate — "target creature with power ≤ 2" =
      -- `And [creature, StatCmp Power LessEq (^2)]`. (Closes the "no stat filter" hole — stat
      -- comparison was a `Condition` only; this lifts it into the `Predicate`/filter language.)
      StatCmp : Stat -> Cmp -> Count b -> Predicate b AnObject
      ControlledBy : Predicate b APlayer -> Predicate b AnObject   -- controller MATCHES a player-pred: "you control" = ControlledBy you, "an opponent controls" = ControlledBy opponent
      OwnedBy : Predicate b APlayer -> Predicate b AnObject
      Controls : Predicate b AnObject -> Predicate b APlayer   -- the INVERSE: a PLAYER who controls a [pred] ("each player who controls a creature")
      Multicolored : Predicate b AnObject   -- ≥2 colors ([CR#105.2b])
      IsColorless : Predicate b AnObject    -- 0 colors (named to avoid the `Colorless : Maybe Color` value)
      -- STACK-object filters: a spell/ability BY its targets ([CR#115]). "Spell that targets you" =
      -- `And [IsKind IsSpell, Targets (SameAs You)]`; "single-target spell" = `TargetCount Equal (^1)`.
      Targets : Predicate b k -> Predicate b AnObject
      TargetCount : Cmp -> Count b -> Predicate b AnObject
      WasKicked : Predicate b AnObject           -- FLAG: kicker as a boolean flag on the object (no cost-mode model)
      -- ANAPHOR: "the candidate has the chosen characteristic" — the chosen color (Iona: "spells of the
      -- chosen color") or creature type (Cavern: "a creature spell of the chosen type"). Gated on an
      -- as-enters CHARACTERISTIC choice being in scope (`IsCharDomain (chosenKind b)`); the engine
      -- resolves which characteristic to test from the domain. No per-color/-type literal anaphor needed.
      OfChosen : {auto prf : IsCharDomain (chosenKind b)} -> Predicate b AnObject
      -- `Anyone` is the player top-predicate ("any player" — a person, hence `APlayer`).
      Anyone : Predicate b APlayer
      -- combinators (`Predicate.And/Or/Not`, sharing names with `Condition`/`EventQuery`). `And`
      -- is same-kind — a candidate is ONE kind, so all conjuncts share it. `Or` (the union) is
      -- HETEROGENEOUS: its arms may differ in kind and the result kind is their JOIN
      -- (`foldr (\/) Empty` over the arms' kinds), so an `Or` mixing object and player predicates is
      -- `Anything` — no `Widen`. "Any target" = `Or [creature…, Anyone]`; an empty `Or` is `Empty`.
      And : List (Predicate b k) -> Predicate b k
      Or : {ks : List RefKind} -> All (Predicate b) ks -> Predicate b (foldr (\/) Empty ks)
      Not : Predicate b k -> Predicate b k     -- negation

  -- A CLOSED / game-state test ([CR#603.4]); reaches objects only via `Matches`
  -- (apply a `Predicate` to a named `Reference`) or `exists`/`unique` (below).
  namespace Condition
    public export
    data Condition : Bindings -> Type where
      Matches : Reference b k -> Predicate b k -> Condition b   -- does r satisfy the (same-kind) predicate
      Compare : Count b -> Cmp -> Count b -> Condition b
      TurnOf : Predicate b APlayer -> Condition b   -- it's a (matching) player's turn (`yourTurn = TurnOf (SameAs You)`)
      During : PhaseStep -> Condition b
      -- "[r] is LEGALLY attached" ([CR#701.3b,303.4d]): has a host that passes the attach-legality
      -- predicate. The Aura graveyard SBA reads its negation (`Not (LegallyAttached This)`).
      LegallyAttached : Reference b AnObject -> Condition b
      -- ANAPHOR (modal): "the chosen MODE is index i" — reads an as-enters `AMode` choice ([CR#614.12]).
      -- `i` is bounded by the choice's mode count `n` (recovered from `chosenKind b = Just (AMode n)`),
      -- so `ChosenIs 2` on a 2-mode card is rejected. Each siege ability gates on it: `If (ChosenIs k) …`.
      ChosenIs : (i : Nat) -> {auto prf : chosenKind b = Just (AMode n)} -> {auto inb : LT i n} -> Condition b
      And : List (Condition b) -> Condition b
      Or : List (Condition b) -> Condition b
      Not : Condition b -> Condition b

  -- The kind-free EVENT-FACET language: conditions refining WHICH event (never its kind, which lives in
  -- the `EventQuery` record's `kinds` slot). Facets conjoin via `And`; `Or` disjoins, `Not` negates (same
  -- combinator names as Predicate/Condition, in this namespace). The THEMATIC-ROLE facets embed the object/
  -- player language; `Within`/`DuringStep`/`DuringTurn` are timing facets ("not during your turn" = `Not (DuringTurn You)`).
  namespace Facet
    public export
    data Facet : Bindings -> Type where
      -- ACTOR: the responsible PLAYER matches a player-pred (you / opponent) — the player AXIS, orthogonal to
      -- the agent→patient relation, NOT a third role. Double duty: the direct doer of a player-event
      -- (`[Begins Cast] [Actor you]`) and the CONTROLLER behind an object-`Agent` (`[DealDamage] [Agent ~, Actor you]`).
      Actor   : Predicate b APlayer -> Facet b
      -- AGENT: the event's DOER/INITIATOR object matches — the moving object of a zone-change, or the
      -- SOURCE of damage (the object dealing it; protection's D leg). The two feed the SAME role.
      Agent   : Predicate b AnObject -> Facet b
      -- PATIENT: the ACTED-UPON thing matches — a damage recipient, a destroyed/countered object, the spell
      -- being cast, the object gaining counters, OR the DEFENDER of an attack ([CR#508.1]). KIND-POLY (the
      -- defender, like a damage recipient, may be a PLAYER): "whenever YOU are attacked" = `[Begins Attack]
      -- [Patient you]`; "deals damage to you" = `Patient you`. Distinct from the `Agent` (the doer).
      Patient : Predicate b k -> Facet b
      Within        : Window -> Facet b
      DuringStep    : PhaseStep -> Facet b
      DuringTurn    : Predicate b APlayer -> Facet b   -- the turn's player matches a player-pred
      -- "this is the FIRST event (matching the surrounding facets) in the window" — an ORDINAL facet,
      -- engine-resolved like `EventCount` ([CR#603.2e] "the first time each…"). Notion Thief: "except the
      -- first draw each draw step" = `Not (And [DuringStep drawStep, IsFirst ThisStep])`.
      IsFirst       : Window -> Facet b
      And  : List (Facet b) -> Facet b   -- AND
      Or   : List (Facet b) -> Facet b   -- OR
      Not : Facet b -> Facet b          -- NOT

  -- an EVENT QUERY = an optional kind-DISJUNCTION + kind-free facets ([CR#603.2]). `kinds` empty = any
  -- kind; `[k]` = exactly k; `[k1,k2]` = any of these (an OR). Facets (implicitly AND-ed) refine WHICH
  -- event. The kind lives in ONE slot, never a facet — so caps stay sound (the INTERSECTION over `kinds`).
  public export
  record EventQuery b where
    constructor MkQuery
    kinds  : List EventKind
    facets : List (Facet b)

  -- whether a literal `Range`'s bounds are ORDERED (lo ≤ hi). Only literal-vs-literal is checked — a
  -- dynamic bound (any `Count` expression) is lenient, exactly like `NonZeroQ`.
  public export
  OrderedRange : Maybe (Count b) -> Maybe (Count b) -> Type
  OrderedRange (Just (Literal lo)) (Just (Literal hi)) = LTE lo hi
  OrderedRange _ _ = ()

  -- A cardinality spec for a choice ([CR#107.3]). In the mutual block so `Selection` can use it.
  public export
  data Quantity : Bindings -> Type where
    Range : Maybe (Count b) -> Maybe (Count b) -> Quantity b

  -- A resolution-time GROUP / choice. In the mutual block because `Single` (a `Reference`)
  -- demotes it. `GetTargets n` = the n-th target slot's targets (`GetTarget` demotes to one).
  public export
  data Selection : Bindings -> RefKind -> Type where
    SelectAll : Predicate b k -> Selection b k                  -- every match (a group)
    Union : List (Selection b k) -> Selection b k              -- groups combined ("each X and each Y"); a fixed set = `Union` of `SameAs` singletons
    That : {auto prf : thatKind b = Just k} -> Selection b k    -- the `With`-bound group
    GetTargets : (n : Nat) -> {auto prf : InBounds n (targetKinds b)} -> Selection b (index n (targetKinds b))
    Random : Quantity b -> Predicate b k -> Selection b k
    TopOfLibrary : (count : Count b) -> {default You whose : Reference b APlayer} -> Selection b AnObject
    BottomOfLibrary : (count : Count b) -> {default You whose : Reference b APlayer} -> Selection b AnObject


public export
andCaps : EventCaps -> EventCaps -> EventCaps
andCaps (MkCaps o1 a1 m1) (MkCaps o2 a2 m2) = MkCaps (o1 && o2) (a1 && a2) (m1 && m2)

-- the caps a whole event-QUERY guarantees its body: the INTERSECTION over its kind-disjunction — the
-- body gets only anaphora that EVERY listed kind supplies. Empty `kinds` (any kind) ⇒ `noCaps`. So a
-- multi-kind trigger ("attacks or blocks") is sound, and there is no way to union incompatible kinds.
public export
eventQueryCaps : EventQuery b -> EventCaps
eventQueryCaps q = case q.kinds of
  []        => noCaps
  (k :: ks) => foldl andCaps (eventKindCaps k) (map eventKindCaps ks)

-- "it's your turn" — the common specialization of `TurnOf`.
public export
yourTurn : Condition b
yourTurn = TurnOf (SameAs You)

-- `exists`/`unique`: a predicate matches ≥1 / exactly-1 object. DERIVED from
-- `CountOf` + `Compare`, not primitive constructors. (`CountOf` takes a `Predicate`,
-- so `exists (During …)` is now a TYPE error, not a degenerate term.)
public export
exists : Predicate b k -> Condition b
exists p = Compare (CountOf p) Greater (Literal 0)

public export
unique : Predicate b k -> Condition b
unique p = Compare (CountOf p) Equal (Literal 1)

public export
implementation Promote Nat (Count b) where
  promote = Literal
public export
implementation Promote Integer (Count b) where
  promote = Literal . integerToNat

-- Integer literals + `+`/`*` sugar for the value language (so `power := Just 2` and
-- `SetPT 1 1` typecheck; `Plus`/`Times` back the operators).
public export
implementation Num (Count b) where
  (+) = Plus
  (*) = Times
  fromInteger = Literal . integerToNat

-- A SIGNED change to a value (layer-7c "+N/+N" P/T modifications). `promote` builds it from
-- an Integer (`promote 2` = Up 2, `promote (-1)` = Down 1); use `Up`/`Down` for dynamic deltas.
public export
data Delta : Bindings -> Type where
  Up   : Count b -> Delta b   -- "+N"
  Down : Count b -> Delta b   -- "−N"

public export
implementation Promote Integer (Delta b) where
  promote n = if n >= 0 then Up (promote n) else Down (promote (negate n))

-- A game-result effect ([CR#104]). Its own category above `Action` — a game-ender
-- isn't just another verb; `OneShotEffect`'s `Conclude` wraps it.
public export
data Outcome : Bindings -> Type where
  WinGame  : Reference b APlayer -> Outcome b
  LoseGame : Reference b APlayer -> Outcome b

-- A STATIC suppressor of a game outcome ([CR#104.2b,104.3e]) — distinct from the imperative `Outcome`
-- above (Rust's lesson: win/lose-the-game is not a deontic over actions, nor a replaceable event, so
-- it needs its own static channel). `OutcomeGate CantLose you` = Platinum Angel's first clause.
public export
data OutcomeGateKind = CantLose | CantWin

-- A position in an ORDERED zone ([CR#401]) — an END plus an offset. `FromTop (^0)` = on top. Named
-- `Anchor` (general over ordered zones — currently the library) rather than `LibraryPosition`.
public export
data Anchor : Bindings -> Type where
  FromTop    : Count b -> Anchor b
  FromBottom : Count b -> Anchor b

-- WHERE a card goes: a plain (unordered) zone, or an ordered zone at an `Anchor`. ONE notion of a
-- destination — subsumes the old bare-`Zone` `Move` argument AND the single-object `PutIntoLibrary`.
-- Sound by construction: only `ToLibrary` carries a position, so "graveyard at FromBottom 0" is
-- unrepresentable.
public export
data Destination : Bindings -> Type where
  ToZone    : Zone -> Destination b
  ToLibrary : Anchor b -> Destination b

-- How a SIMULTANEOUS group of cards is ordered as it lands at a position in an ORDERED zone — the
-- order is a property of the PLACEMENT, not the loop. `ChosenOrder` = the owner arranges them, the
-- [CR#401.4] "any order" default; `RandomOrder` = shuffled into place ([MTR 3.10], a randomized pile
-- is the same kind of object as a shuffled library); `SameOrder` = preserve the source order (only
-- meaningful from an already-ordered source). A single object has no internal order, so only
-- `MoveArranged` (a group) carries it. The `…Order` suffix keeps `RandomOrder` distinct from the
-- `Selection.Random` constructor ("N random objects").
public export
data Arrangement = ChosenOrder | RandomOrder | SameOrder

-- A continuous effect's lifetime ([CR#611.2]). Rust: Duration. (Above `Action` so a
-- duration-bounded verb like `ExileUntil` can name it.)
public export
data Duration : Bindings -> Type where
  UntilEndOfTurn : Duration b
  UntilEvent : EventQuery b -> Duration b
  ForAsLongAs : Condition b -> Duration b
  Permanent : Duration b                       -- rest of game (Rust: EndOfGame)

-- `Range lo hi`: `Nothing` bound = unbounded that side. A bare numeral is the
-- EXACTLY case (`Range (Just n) (Just n)`); the helpers below name the rest.
public export
implementation Promote Integer (Quantity b) where
  promote n = let k = Literal (integerToNat n) in Range (Just k) (Just k)

public export
atLeast : Count b -> Quantity b
atLeast n = Range (Just n) Nothing

public export
atMost : Count b -> Quantity b
atMost n = Range Nothing (Just n)

public export
between : (lo : Count b) -> (hi : Count b) -> {auto 0 prf : OrderedRange (Just lo) (Just hi)} -> Quantity b
between lo hi = Range (Just lo) (Just hi)

public export
anyNumber : Quantity b
anyNumber = Range Nothing Nothing

-- A target slot's `Quantity` must permit ≥1 target ([CR#115.1] — a slot can't target nothing).
-- Guards the UPPER bound: a statically-zero max ("up to 0") is rejected; "up to N>0" (lower 0) is fine.
public export
NonZeroQ : Quantity b -> Type
NonZeroQ (Range _ (Just (Literal Z))) = Void
NonZeroQ _ = ()

public export
data TargetSpec : Bindings -> RefKind -> Type where
  -- a target slot: a NON-ZERO `Quantity` of targets matching the predicate (`Target (^1)` = one;
  -- `Target (between (^1) (^2))` = "one or two"). The slot's targets are `GetTargets n` (a group);
  -- `GetTarget n` demotes a single-target slot to a `Reference`.
  Target : (q : Quantity b) -> {auto 0 prf : NonZeroQ q} -> Predicate b k -> TargetSpec b k

-- the n-th target as a single `Reference` (the common case) — sugar that demotes the slot's
-- targets via `Single`. A plural slot (`Target (between …)`) uses `GetTargets` directly.
public export
GetTarget : (n : Nat) -> {auto prf : InBounds n (targetKinds b)} -> Reference b (index n (targetKinds b))
GetTarget n = Single (GetTargets n)

-- "the unique object matching a predicate" — sugar: the sole element of `SelectAll p`.
public export
Only : Predicate b AnObject -> Reference b AnObject
Only p = Single (SelectAll p)

-- a use-LIMIT on a `Replaces` — how many times it fires before it's CONSUMED (a shield). `Unlimited` =
-- today's continuous replacement; `UpTo n` = "the next n" — n OCCURRENCES for an amountless event
-- (regeneration: the next destroy), n AMOUNT-POINTS for an amount event (prevention: the next n damage).
public export
data ReplaceLimit : Bindings -> Type where
  Unlimited : ReplaceLimit b
  UpTo : Count b -> ReplaceLimit b

-- WHICH counters a `MoveCounters` relocates ([CR#122.5]). `Some c n` = n counters of one kind (Power
-- Conduit, Leech Bonder; "all of that kind" = `Some c (CountersOn c from)`, the `RemoveCounters` idiom).
-- `AllKinds` = every counter regardless of kind (Ozolith, Fate Transfer) — the one move case the single-
-- kind form can't reach, since it quantifies over kinds rather than naming one. So move stays ONE verb;
-- the kind/quantity (or "everything") is data, not a second constructor.
public export
data CounterSpec : Bindings -> Type where
  Some : (c : CounterKind) -> Count b -> CounterSpec b
  AllKinds : CounterSpec b

-- How many modes to choose, for a modal effect ([CR#700.2]). Rust: ChooseSpec. The count is a `Quantity`
-- (the same range language as `Target`), so "choose one" = `^1`, "choose one or both" = `between (^1) (^2)`,
-- "choose one or more" = `atLeast (^1)`, "choose up to two" = `atMost (^2)` (subsumes the old `upTo` flag).
public export
data ChooseSpec : Bindings -> Type where
  MkChooseSpec : (count : Quantity b) -> {default False repeats : Bool} -> ChooseSpec b

-- a modal choose-count must not exceed the number of modes ([CR#700.2d]) — checked only when the UPPER
-- bound is a LITERAL and modes can't repeat (a repeating choice, or an unbounded "one or more", is lenient,
-- exactly like `NonZeroQ` guards only a literal bound). An unbounded upper is implicitly the mode count.
public export
ModalCountOk : ChooseSpec b -> (modeCount : Nat) -> Type
ModalCountOk (MkChooseSpec (Range _ (Just (Literal hi))) {repeats = False}) modeCount = LTE hi modeCount
ModalCountOk _ _ = ()

-- A DEONTIC clause's carrier: a game ACTION a player may attempt ([CR#101.2,601.3] the deontic
-- layer) — distinct from the resolving `Action` verbs. Each names its participants; the CR's
-- "where ⟨pred⟩" qualifier rides the variable participant (`who`/`blocker`/`source`). The
-- polarities `Constrain` (Require/Forbid)/`Priced` (in `StaticEffect`) wrap a `Deed`. BOUNDARY [CR#614.17]:
-- this is choice-LEGALITY ("can't attack"); event-edits ("doesn't tap", "can't be regenerated",
-- "can't lose") are `Replaces`/SBA, NOT a `Constrain`.
-- the two COMPULSION polarities of a declaration constraint — the pair the combat solver balances
-- ([CR#508.1c] restriction / [CR#508.1d] requirement): `Forbid` prevents the deed, `Require` forces
-- it if able. `Constrain` (in `StaticEffect`) carries one; `cant`/`must` (Macros) are the aliases.
public export
data Compulsion = Require | Forbid

-- the two PRICED-deed timings, folded into one `Priced` constructor: `AtDeclaration` = the cost is paid
-- when the deed is declared (the old `Gate`, never compulsory, [CR#508.1d]); `Downstream` = it is punished
-- after the fact (the old `Toll`, ward [CR#702.21a]).
public export
data TollTiming = AtDeclaration | Downstream

public export
data Deed : Bindings -> Type where
  -- the DEONTIC aspect of the relation spine: "[agent] enacts [r] upon [patient]" (under Can/Constrain/
  -- Priced). The AGENT's kind is fixed by `agentScope r` (ONE agent slot — no `Agent`/`Actor` split): a PLAYER for
  -- Cast/Activate/Play, the SOURCE OBJECT for Attack/Block/Attach/Target/Counter. The PATIENT stays kind-
  -- poly (an attack's defender is a player OR a permanent, [CR#508.1]). The two PASSIVE deeds fold in once
  -- the source is the explicit agent. Examples:
  --   Defender             = `cant (Enact Attack (SameAs This) Anyone)`
  --   "q can't block this" = `cant (Enact Block q (SameAs This))`
  --   "Enchant creature"   = `Can  (Enact Attach (SameAs This) creature)`  ([CR#701.3a]) — attach is default-FORBIDDEN, Enchant ENABLES it
  --   Shroud               = `cant (Enact Target spellOrAbility (SameAs This))`  (the source spell/ability is the agent)
  --   "can't be countered" = `cant (Enact Counter spellOrAbility (SameAs This))`
  --   flash                = `Can  (Enact Cast you (SameAs This)) {window = AsInstant}`  ([CR#702.8a])
  -- (Subsumed the old Attacks/Blocks/Attaches/BeTargeted/Casts/Activates/Plays/Countered verbs.)
  Enact      : (r : Relation) -> (agent : Predicate b (agentScope r)) -> (patient : Predicate b k) -> Deed b
  -- SET-LEVEL block ([CR#509.1c],[CR#702.111b]): "[attacker] is blocked by a DECLARED set of `size`
  -- creatures" (a block, so size ≥ 1 — ENFORCED by `NonZeroQ`). `cant (BlockedBy This …)` constrains the
  -- WHOLE blocker set, not one blocker at a time — Menace = `cant (BlockedBy (SameAs This) (^1))`
  -- (forbid the lone blocker; 0 = unblocked and 2+ stay legal). The one combat constraint the identity
  -- spine doesn't subsume: it's about HOW MANY blockers, not WHICH. [CR#509.1c] judges the whole set.
  BlockedBy  : (attacker : Predicate b AnObject) -> (size : Quantity b) -> {auto prf : NonZeroQ size} -> Deed b

-- A CHARACTERISTIC a `Set` modification can OVERWRITE ([CR#613] at the layer the engine knows per
-- characteristic — colors L5, types L4, P/T L7b, …). ONE mechanism for all of them: each maps to its
-- value TYPE via the total `CharValue`, so `Set` takes the right value by construction.
public export
data Characteristic = Colors | CardTypes | Subtypes | Supertypes | BasePower | BaseToughness | Name

public export
CharValue : Bindings -> Characteristic -> Type
CharValue _ Colors        = List Color
CharValue _ CardTypes     = List Type_
CharValue _ Subtypes      = List Subtype
CharValue _ Supertypes    = List Supertype
CharValue b BasePower     = Count b        -- may be dynamic (a CDA "*/*")
CharValue b BaseToughness = Count b
CharValue _ Name          = Maybe String   -- `Nothing` = "has no name"

-- One big mutual block: `Ability → OneShotEffect → Action → CreateToken → Characteristics` is a
-- cycle, so `Characteristics`/`Action`/`Bindable` join the effect/ability block below. `Cost` joins
-- too — its `Do` wraps an `Action` ([CR#118.3]) — dragging the Cost-referencing `CostChange`/
-- `AlternativeCost` in with it. (The leaf `ChooseSpec`/`Deed` stay OUT — they only reach into block 1.)
mutual
  -- A cost paid to activate an ability ([CR#118,602]). `Costs` conjoins components. Most costs ARE actions
  -- the payer performs ([CR#118.3]), so they ride `Do` rather than each getting a duplicate cost verb.
  public export
  data Cost : Bindings -> Type where
    Mana      : ManaCost -> Cost b                 -- "{4}"
    -- pay a cost by PERFORMING an action ([CR#118.3]): "{T}" = `Do (Tap This)`, "Pay N life" =
    -- `Do (LoseLife (^N))`, "Sacrifice this" = `Do (Sacrifice You (SameAs This))`, "Pay {E}×N" =
    -- `Do (RemoveCounters Energy (^N) You)` (energy is a player counter — no dedicated `PayEnergy` verb),
    -- loyalty "+N"/"−N" = `Do (PutCounters/RemoveCounters Loyalty (^N) This)`. UNRESTRICTED — ANY action
    -- (even scry/shuffle as a cost is legal); a senseless cost just no-ops, and nonsense is the grammar
    -- layer's to catch, not a gate.
    Do        : Action b -> Cost b
    Scaled    : Count b -> Cost b -> Cost b         -- the cost paid once per unit of the count ("{2} for each X" = Scaled (CountOf X) (Mana [promote 2]))
    Costs     : List (Cost b) -> Cost b            -- all components together
    -- AGGREGATE cost: tap a chosen subset of [of_] whose summed [stat] satisfies [cmp] [n]. ONE shape
    -- for Crew ("tap creatures, total power ≥ N" = `TapTotal Power GreaterEq (^n) creature`) — and the
    -- Convoke/devotion-scaling family the engine's authors flagged it should subsume.
    TapTotal  : Stat -> Cmp -> Count b -> (of_ : Predicate b AnObject) -> Cost b

  -- A continuous CHANGE to a spell/ability cost ([CR#118.7]), carried by `StaticEffect::CostModifier`.
  -- Borrowed from the Rust engine's key split: this MODIFIES an existing base — it is NOT an alternative
  -- cost (a base SWAP), which would be a separate type. Count-scaling is ONE recursive node, so affinity
  -- (`ScaledBy (Reduce [Mana [^1]]) (CountOf …)`) and taxers (scale an `Increase`) need no own constructor.
  public export
  data CostChange : Bindings -> Type where
    Reduce     : List (Cost b) -> CostChange b            -- "costs {…} less"
    Increase   : List (Cost b) -> CostChange b            -- "costs {…} more"
    Additional : List (Cost b) -> Bool -> CostChange b    -- "as an additional cost, …"; the Bool = OPTIONAL (the kicker shape)
    ScaledBy   : CostChange b -> Count b -> CostChange b  -- the change applied once per unit of the count (affinity)

  -- An ALTERNATIVE base cost ([CR#118.9]) — a base SWAP, the type the engine keeps DISTINCT from
  -- `CostChange` (a base modify). "Without paying its mana cost" = `AltCost []`; Force of Will = `AltCost […]`.
  public export
  data AlternativeCost : Bindings -> Type where
    AltCost  : List (Cost b) -> AlternativeCost b

  -- The printable CHARACTERISTICS of an object ([CR#109.3]) — shared by a card `Face`
  -- (`Characteristics Base`) and a created token (`Characteristics b`, so a token's P/T can be a
  -- `Count b`: "an X/X where X = [a value known at creation]"). `colors` is the explicit color (a
  -- color indicator [CR#204.2] / a token's printed color); a card's color-FROM-MANA is derived.
  public export
  record Characteristics (b : Bindings) where
    constructor MkCharacteristics
    name : Maybe String          -- optional — most tokens are nameless ([CR#111.4])
    manaCost : ManaCost
    colors : List Color
    types : List Type_
    supertypes : List Supertype
    subtypes : List Subtype
    abilities : List (Ability b)
    power : Maybe (Count b)
    toughness : Maybe (Count b)
    loyalty : Maybe (Count b)
    defense : Maybe (Count b)

  -- A DELIBERATELY LENIENT well-formedness floor ([CR#109.3]): an object has ≥1 card type. That's
  -- the only safe universal — printed stats can't be pinned to types (a Vehicle is an Artifact with
  -- P/T; Tarmogoyf is a Creature whose P/T come from a CDA, not printed fields), and `name` is
  -- optional. Demanded at `Normal` and `CreateToken`.
  public export
  CharacteristicsOk : Characteristics b -> Type
  CharacteristicsOk c = NonEmpty (types c)

  -- A per-mana STRING attached to produced mana ([CR#106.6] enumerates exactly these three; they ride
  -- EACH mana the production makes, [CR#106.6a]). NOT snow (that's the source's `Snow` supertype) nor
  -- "doesn't empty" (a separate static/replacement over all your mana — Omnath/Upwelling), which would
  -- double-represent. The paid-for object is bound `It` in the two effect-bearing riders.
  public export
  data ManaRider : Bindings -> Type where
    SpendOnly      : Predicate b AnObject -> ManaRider b               -- (1) "spend only to cast/activate a [pred]" (Cavern's creature spell of the chosen type)
    GrantOnSpend   : StaticEffect (bindIt AnObject b) -> ManaRider b   -- (2) the object it's spent on (`It`) gains [static] (Cavern's "that spell can't be countered")
    TriggerOnSpend : OneShotEffect (bindIt AnObject b) -> ManaRider b  -- (3) a delayed trigger when the mana is spent ([CR#603.7a]); `It` = the object paid for

  -- The verbs ([CR#701]). `Effect::Act` wraps these. Object verbs carry an object
  -- `source` (default `This`); player verbs an `actor : Reference b APlayer` (default `You`).
  public export
  data Action : Bindings -> Type where
    -- deal damage to ONE recipient ([CR#120.1] — damage is to a single object/player per event);
    -- `source` object is the agent. "Deals N to EACH …" is a `Each` over the recipients.
    DealDamage : {default This source : Reference b AnObject} -> Reference b k -> Count b -> Action b
    -- (divided damage — "N damage divided as you choose among [a group]" — is the general `Distribute`
    --  effect: `Distribute (^n) group (Act (DealDamage It Allotment))`, not a bespoke action.)
    -- a plain zone change [CR#400.7]; owner-relative, control implicit.
    Move : Reference b AnObject -> Destination b -> Action b
    -- exile a selection UNTIL a duration ends, then return it — the duration-bounded
    -- "exile until ~" form (return via a delayed trigger, [CR#603.7a]), NOT a leave-triggered return (see Oblivion Ring).
    ExileUntil : Reference b AnObject -> Duration b -> Action b
    -- destroy [CR#701.8] / counter a stack object [CR#701.6a]. (Return-to-hand is just
    -- `Move … Hand` — `Move` is owner-relative — so there's no dedicated bounce verb.)
    Destroy : Reference b AnObject -> Action b
    Counter : Reference b AnObject -> Action b
    -- tap / untap [CR#701.26]; attach / unattach [CR#701.3].
    Tap : Reference b AnObject -> Action b
    Untap : Reference b AnObject -> Action b
    RemoveAllDamage : Reference b AnObject -> Action b    -- remove all damage marked on r (regeneration's heal, [CR#701.19])
    RemoveFromCombat : Reference b AnObject -> Action b   -- remove r from combat ([CR#506.4])
    Transform : Reference b AnObject -> Action b   -- turn a transforming DFC to its other face ([CR#701.27])
    PhaseOut : Reference b AnObject -> Action b     -- phase a permanent out ([CR#702.26]); phasing back in is the engine's turn-based action
    -- "[r] becomes/gets the designation" — the target's kind follows `designationScope` (you become the
    -- monarch; this creature becomes monstrous). Single-holder eviction (monarch) is the engine's.
    GrantDesignation : (d : Designation) -> Reference b (designationScope d) -> Action b
    Attach : (what : Reference b AnObject) -> (to : Reference b AnObject) -> Action b
    Unattach : Reference b AnObject -> Action b
    -- a player verb: the `actor` draws n cards. Rust: PlayerAction::Draw(Count).
    Draw : {default You actor : Reference b APlayer} -> Count b -> Action b
    -- the `actor` gains n life. Rust: PlayerAction::GainLife(Count).
    GainLife : {default You actor : Reference b APlayer} -> Count b -> Action b
    -- put a GROUP at an ordered position with an `Arrangement` ([CR#401.4]): "put the top three on the
    -- bottom in any order" = `MoveArranged (TopOfLibrary (^3)) ChosenOrder (ToLibrary (FromBottom (^0)))`;
    -- "...in a random order" = `… RandomOrder …`. Distinct from single `Move` — order
    -- only EMERGES for a simultaneous group landing in an ordered zone. (Per `DealDamage`-single +
    -- group-via-`Each` house style, single moves stay `Move`; this is the group verb.)
    MoveArranged : Selection b AnObject -> Arrangement -> Destination b -> Action b
    -- put / remove counters ([CR#122]). `RemoveCounters` is symmetric with `PutCounters` (a `Count`);
    -- "remove all of a kind" is `RemoveCounters c (CountersOn c r) r`. Loyalty/counter COSTS reuse these via
    -- `Do` (e.g. "−2" = `Do (RemoveCounters Loyalty (^2) This)`), so there is no duplicate counter-cost verb.
    PutCounters : (c : CounterKind) -> Count b -> Reference b (counterScope c) -> Action b
    RemoveCounters : (c : CounterKind) -> Count b -> Reference b (counterScope c) -> Action b
    -- MOVE counters object→object ([CR#122.5] = remove-from + put-on, one operation). The `CounterSpec`
    -- says which: `Some c n` (Power Conduit, Leech Bonder) or `AllKinds` (Ozolith). Both ends are objects
    -- (counters don't move between players), so no `counterScope` indexing — a senseless kind just no-ops.
    MoveCounters : CounterSpec b -> (from : Reference b AnObject) -> (to : Reference b AnObject) -> Action b
    -- player verbs: discard / lose life; and a chooser-verb where a player sacrifices.
    Discard : {default You actor : Reference b APlayer} -> Count b -> Action b
    LoseLife : {default You actor : Reference b APlayer} -> Count b -> Action b
    Sacrifice : Reference b APlayer -> Predicate b AnObject -> Action b   -- "[player] sacrifices a [pred]" (they choose which)
    -- further keyword-action verbs ([CR#701]). The interactive bits (reorder, search choice, copy
    -- characteristics) are the engine's; the grammar names the verb. Scry/Surveil/Fight are NOT verbs
    -- here — they COMPOSITE over primitives (`Each`/`With`/`Modal`/`Move`/`DealDamage`) as
    -- macros in `Macros.idr`, exactly like the keyword ABILITIES.
    Reveal : Reference b AnObject -> Action b
    Shuffle : {default You actor : Reference b APlayer} -> Action b
    -- "[player] takes an extra turn after this one" ([CR#500.7]) — Time Walk.
    ExtraTurn : {default You actor : Reference b APlayer} -> Action b
    -- "you control [whom] during their next turn" ([CR#723]) — Mindslaver: you make all of their
    -- decisions. The next-turn duration is the standard one the engine applies.
    ControlPlayer : (whom : Reference b APlayer) -> Action b
    CreateToken : Count b -> (c : Characteristics b) -> {auto wf : CharacteristicsOk c} -> Action b   -- the token's full characteristics (P/T may be a `Count b`)
    CopySpell : Reference b AnObject -> Action b                   -- "copy target SPELL" (a copy on the stack); permanent-copy is `BecomeCopyOf`/`CreateTokenCopy`
    CreateTokenCopy : Reference b AnObject -> Action b             -- "create a token that's a COPY of [r]" ([CR#707.2]); alterations layer on separately
    -- "add mana" (a mana-ability effect; pool/paying is engine) ([CR#106.1,106.4]). ONE verb (merges the
    -- old `AddMana` + `AddManaFor`): `amount` copies of one `ProducedMana`, so fixed "{C}" (`amount = ^1`),
    -- {X}/devotion/count-scaled production (Gaea's Cradle, Karametra's Acolyte), and a producer-chosen
    -- color (`OneOf`/`AnyColor`) all fall out of the value language. `riders` are the per-mana strings of
    -- [CR#106.6] applied to each of the `amount` mana ([CR#106.6a]) — Cavern: a chosen color, only to cast
    -- the chosen creature type, uncounterable. (Fixed HETEROGENEOUS production — "add {R}{G}" — is a
    -- `Sequence` of `AddMana`s, so the old per-action list is gone.)
    AddMana : {default You actor : Reference b APlayer} -> (amount : Count b) -> ProducedMana
              -> {default [] riders : List (ManaRider b)} -> Action b

  -- What a binder (`With`) binds as `That`: a QUERY of existing objects, a PRODUCER
  -- (an `Action` run for effect, binding its product), or a CHOICE (a player picks).
  -- The grammar only names the role; the ENGINE resolves `That` to the live (reminted
  -- or gone) object, so `MovedRef`/lki/became is a runtime concern, NOT modeled here.
  public export
  data Bindable : Bindings -> RefKind -> Type where
    Existing : Selection b k -> Bindable b k  -- bind existing entities (a plain selection)
    Produce : Action b -> Bindable b AnObject -- run the action, bind its product (the moved object) as `That`
    -- `by` chooses a `Quantity` of entities matching the filter; the chosen are bound as
    -- `That`. Choosing is interactive, so it lives here, not in `Selection`. Rust: Selection::Choose.
    Choose : {default You by : Reference b APlayer} -> Quantity b -> Predicate b k -> Bindable b k
    -- `by` searches `whose`'s `from`-zones (one or more — "library and/or graveyard") for
    -- matching cards, bound as `That` — like `Choose`, but from (hidden) zones the engine
    -- reveals/shuffles. Search ANOTHER player's via `whose`; the found card's destination
    -- is a following owner-routed `Move That …`. Rust: Selection::Search.
    Search : {default You by : Reference b APlayer} -> {default You whose : Reference b APlayer} -> {default [Library] from : List Zone} -> Quantity b -> Predicate b k -> Bindable b k

  -- Effects, continuous effects, and abilities are mutually recursive: a one-shot
  -- can CREATE a continuous effect (`Continuously`), a static ability can grant an
  -- ability, and an ability wraps an effect.
  public export
  data OneShotEffect : Bindings -> Type where
    Sequence : (List (OneShotEffect b)) -> OneShotEffect b
    -- each target slot carries its OWN kind (its filter's), gathered as `ks : List RefKind`
    -- (a heterogeneous `All`), so the body's `GetTarget i` is strictly kinded PER SLOT —
    -- mixed-kind multi-target ("target player gains control of target creature", Donate) works.
    Targeted : {ks : List RefKind} -> All (TargetSpec b) ks -> OneShotEffect (bindTargets ks b) -> OneShotEffect b
    -- binds `that` as `That` (of the bound kind) for `body`; `that` may PRODUCE a moved object. Rust: Effect::With.
    With : Bindable b k -> OneShotEffect (bindThat k b) -> OneShotEffect b
    -- a single intrinsic instruction (the verb compartment). Rust: Effect::Act.
    Act : Action b -> OneShotEffect b
    -- end the game (or a player's part in it) — the `Outcome` compartment. Rust: Effect::Conclude.
    Conclude : Outcome b -> OneShotEffect b
    -- "you may [effect]", with optional "if you do / if you don't". Rust: Effect::May.
    May : (effect : OneShotEffect b) -> {default Nothing ifDid : Maybe (OneShotEffect b)} -> {default Nothing ifNot : Maybe (OneShotEffect b)} -> OneShotEffect b
    -- "if [cond], [thenDo]; otherwise [else]". Rust: Effect::If.
    If : Condition b -> (thenDo : OneShotEffect b) -> {default Nothing otherwise : Maybe (OneShotEffect b)} -> OneShotEffect b
    -- COST-payment DECISIONS — a player chooses whether to pay (the common decider slice; the
    -- full `Cost` algebra rides both). Rust: Effect::MayPay / Effect::MustPay.
    --  • `MayPay`  — "[actor] MAY pay [cost]; if they do → `and_then`; if not → optional `or_else`."
    --  • `MustPay` — "[actor] must pay [cost], OR ELSE `or_else`" — the resolution-stage punisher
    --    (Mana Leak: "counter target spell unless its controller pays {2}"; supersedes `Unless`).
    MayPay  : {default You actor : Reference b APlayer} -> Cost b -> (and_then : OneShotEffect b) -> {default Nothing or_else : Maybe (OneShotEffect b)} -> OneShotEffect b
    MustPay : {default You actor : Reference b APlayer} -> Cost b -> (or_else : OneShotEffect b) -> OneShotEffect b
    -- create a continuous effect for a duration ([CR#611.2]). Rust: Effect::Continuously.
    Continuously : StaticEffect b -> Duration b -> OneShotEffect b
    -- choose modes, then apply them ([CR#700.2]). Rust: Effect::Modal.
    Modal : (spec : ChooseSpec b) -> (modes : List (Mode b)) -> {auto 0 ne : NonEmpty modes} -> {auto 0 cnt : ModalCountOk spec (length modes)} -> OneShotEffect b
    -- "for each [domain], [body]" — binds each element as `It`. The distributive
    -- primitive (subsumes the old `Selection::Each`). Rust: Effect::ForEach.
    Each : Selection b k -> OneShotEffect (bindIt k b) -> OneShotEffect b
    -- "[amount] divided as you choose among [a group]" ([CR#601.2d]): bind each element as `It` with its
    -- `Allotment` (the split is engine-resolved, ≥1 each summing to amount), then apply `body`. GENERAL over
    -- the per-element effect — subsumes divided damage (`Act (DealDamage It Allotment)`) and divided
    -- counters (`Act (PutCounters c Allotment It)`); replaced the bespoke `DealDamageDivided`. (`amount`,
    -- not `total` — the latter is a reserved totality keyword.)
    Distribute : (amount : Count b) -> Selection b k -> OneShotEffect (bindAllot k b) -> OneShotEffect b
    -- "when you do [the preceding], [effect]" — a reflexive trigger. It NESTS, so
    -- `That`/targets stay in scope; no event-scanning sibling. Rust: Effect::Reflexive.
    Reflexive : OneShotEffect b -> OneShotEffect b
    -- schedule `body` for `event`; `unbindTargets` keeps `That`, drops targets. Rust: Effect::Delayed.
    Delayed : (q : EventQuery b) -> OneShotEffect (bindEvent (eventQueryCaps q) (unbindTargets b)) -> OneShotEffect b

  -- one option of a modal effect: an effect plus an optional extra cost. Rust: Mode.
  public export
  data Mode : Bindings -> Type where
    MkMode : (effect : OneShotEffect b) -> {default Nothing cost : Maybe (Cost b)} -> Mode b

  -- A continuous modification a static ability applies to its subject.
  public export
  data Modification : Bindings -> Type where
    ModifyPT : Delta b -> Delta b -> Modification b     -- "gets +x/+y" (SIGNED, layer 7c — Up/Down)
    -- SET any characteristic to a value ([CR#613]): "becomes blue" = `Set Colors [Blue]`; "loses all
    -- creature types" = `Set Subtypes []`; "base p/t are x/y" = `Set BasePower x` + `Set BaseToughness y`
    -- (a CDA `*/*` sets a dynamic `Count`). One mechanism, value-typed by `CharValue`; subsumes old `SetPT`.
    Set : (c : Characteristic) -> CharValue b c -> Modification b
    AddType : Type_ -> Modification b                   -- "is also a [type]"
    AddSubtype : Subtype -> Modification b              -- "becomes an Island" (adds the subtype)
    -- TEXT-CHANGE ([CR#612], a layer-3 mod): "replace all instances of one word with another of its
    -- class" — the eligible classes are listed; the two specific words are the player's resolution-time
    -- choice (engine-resolved, like `Choose`). Mind Bend = `ChangeText [ColorWords, BasicLandTypes]`.
    ChangeText : List TextWordClass -> Modification b
    LoseAbilities : Modification b                      -- "loses all abilities" (Humility-style)
    GainControl : Reference b APlayer -> Modification b         -- "[player] gains control"
    GrantAbility : Ability b -> Modification b
    -- "becomes a COPY of [r]" ([CR#707.2], layer 1 — copiable values). Alterations ("a copy, except it's
    -- a 4/4") are SEPARATE higher-layer mods (Continuously/Modify on the result), not bundled here.
    BecomeCopyOf : Reference b AnObject -> Modification b

  -- A continuous effect a static (or `Continuously`) ability generates ([CR#611]):
  -- modify a subject, modify a whole filter (anthem), or REPLACE an event — a
  -- replacement effect is a continuous effect too ([CR#614]). Rust: the StaticEffect family.
  public export
  data StaticEffect : Bindings -> Type where
    -- continuous modification over a SELECTION (the one set-language): singleton = `SelectAll (SameAs r)`,
    -- anthem/filter = `SelectAll pred`, fixed set = `Union` of singletons. Each element is bound as `It`
    -- for the mods (the per-subject binder — no `Subject` reference, since a `Predicate`'s candidate is
    -- already implicit), so a PER-SUBJECT mod can read it: Coat of Arms = "+X/+X where X = other creatures
    -- sharing a type with It" = `ModifyPT (Up (CountOf (And [creature, SharesSubtype It, Not (SameAs It)]))) …`.
    -- Unifies the former `Modify`(singleton)/`ModifyAll`(filter) split and reaches the fixed-set case neither could.
    Modify : Selection b AnObject -> List (Modification (bindIt AnObject b)) -> StaticEffect b
    -- continuous COST modification ([CR#118.7]): spells/abilities matching `of_` get the `change`.
    -- "Instant/sorcery spells you cast cost {1} less" = `CostModifier (And […, ControlledBy you]) (Reduce
    -- [Mana [^1]])`; affinity is a SELF modifier `CostModifier (SameAs This) (ScaledBy (Reduce …) (CountOf …))`.
    CostModifier : Predicate b AnObject -> CostChange b -> StaticEffect b
    -- "if [event] would happen, do [effect] INSTEAD" — a replacement ([CR#614]). Empty body = a SKIP
    -- (a replacement that removes the event — e.g. "skip your draw step"). This is NOT a prohibition:
    -- the event still "would happen" and is intercepted; for "can't happen", use `CantHappen` below.
    Replaces : (q : EventQuery b) -> OneShotEffect (bindEvent (eventQueryCaps q) b) -> {default Unlimited limit : ReplaceLimit b} -> StaticEffect b
    -- "[event] CAN'T happen" — a continuous PROHIBITION, semantically distinct from replacing-with-
    -- nothing: it's not a one-shot ([CR#614.5]) application, isn't ordered against other replacements
    -- ([CR#616]), and the event never "would happen". Indestructible = `CantHappen (MkQuery [Destroy]
    -- [this])`; Solemnity = `CantHappen (MkQuery [PutCounters] [])`. (Event-level; the deontic `cant` is
    -- its player-ACTION sibling — "can't attack".)
    CantHappen : EventQuery b -> StaticEffect b
    -- PAYLOAD replacement ([CR#616]): the event still happens, but its numeric amount becomes
    -- `newAmount` (a `Count` over the event body, so it can read `ThatMuch`). Furnace of Rath =
    -- `ReplaceAmount DealDamage (Times ThatMuch (^2))`. The KIND is explicit + amount-gated, so
    -- `ReplaceAmount (Begins Cast) …` (a cast has no amount) is a TYPE ERROR; `facets` adds non-kind conditions.
    ReplaceAmount : (k : EventKind) -> {auto amt : eventKindHasAmount k = True} -> {default [] facets : List (Facet b)} -> (newAmount : Count (bindEvent (eventKindCaps k) b)) -> StaticEffect b
    -- a static OUTCOME suppressor: the matching players can't lose / can't win ([CR#104.2b,104.3e]). Platinum
    -- Angel = `OutcomeGate CantLose you` + `OutcomeGate CantWin opponent`. (Distinct from `CantHappen` —
    -- game-loss isn't a replaceable event — and from a deontic `cant` — it's not a player action.)
    OutcomeGate : OutcomeGateKind -> Predicate b APlayer -> StaticEffect b
    -- ADDITIVE replacement ([CR#614.13] "as well as"): when [event] happens it STILL happens, but
    -- [effect] also runs. An Aura enters attached via `Also thisEnters (Act (Attach This host))`.
    Also : (q : EventQuery b) -> OneShotEffect (bindEvent (eventQueryCaps q) b) -> StaticEffect b
    -- a STATE-BASED ACTION as data ([CR#704]): whenever [when] holds (with `This` = the carrier), do
    -- [then] in the SBA sweep. ONE primitive for the Aura graveyard rule (`Sba (Not (LegallyAttached
    -- This)) (Act (Move This (ToZone Graveyard)))`, [CR#704.5m]) AND a Saga's final-chapter sacrifice — the
    -- sweep never branches on subtype. (The engine confers the Aura one via the Aura subtype's conferral
    -- (`subtypeConfers` — a `Static (Sba …)`), so it's a shared rule here, shown once, not per-card.)
    Sba : Condition b -> OneShotEffect b -> StaticEffect b
    -- "[who]'s unspent mana doesn't empty" ([CR#106.4] exception) — Kruphix/Omnath. A pool-policy
    -- static over ALL your mana — which is WHY "doesn't empty" is NOT a per-mana `ManaRider` (that would
    -- double-represent this blanket form); the `ManaRider` set is exactly the [CR#106.6] trio. Engine resolves.
    ManaPersists : Predicate b APlayer -> StaticEffect b
    -- "you may cast THIS for [alt] from [from]" ([CR#118.9]) — the alternative-cost permission (base swap,
    -- distinct from `CostModifier`'s base modify). `from` defaults to Hand; a non-default zone is the
    -- cast-from-zone family ([CR#702.34] flashback = `{from = Graveyard}`; escape/jump-start add a rider).
    -- Force of Will = `MayCastFor (AltCost [Do (LoseLife (^1)), …])`.
    MayCastFor : AlternativeCost b -> {default Hand from : Zone} -> StaticEffect b
    -- "you may cast THIS face down for [cost]" ([CR#702.37]) — an alternative cast that ALSO turns the
    -- object face down; the engine then applies the global [CR#708.2] 2/2-colorless-vanilla override.
    CastFaceDown : Cost b -> StaticEffect b
    -- the inner continuous effect applies only WHILE the condition holds ([CR#604.3]) —
    -- a conditional static ("gets +1/+1 as long as …").
    While : Condition b -> StaticEffect b -> StaticEffect b
    -- DEONTIC clauses over a `Deed` (choice-legality, [CR#101.2]): the permission FLOOR (`Can`, the
    -- deontic "may" — named `Can` to avoid the one-shot `May`), a `Constrain` (the two COMPULSION
    -- polarities — `Forbid` = a restriction "can't", `Require` = a requirement "must"; `cant`/`must`
    -- are the Macros aliases), or a cost-gate. The engine arbitrates can't-beats-can/must
    -- ([CR#101.2,508.1d]); the grammar only records the clauses. A `Priced` deed's cost comes FIRST —
    -- `AtDeclaration` (paid up front, never compulsory, [CR#508.1d]) or `Downstream` (ward, [CR#702.21a]).
    -- These gate CHOICES — the §6 sibling of `Replaces` (event-edits), never conflated with it.
    --  • `Can` — the permission floor made explicit ([CR#101.2,601.3]). A `Can (Casts …)` carries a
    --    `window`; Flash widens it to `AsInstant` ([CR#702.8a] — a wider window, NOT an as-though).
    --  • `AsThough` — a scoped COUNTERFACTUAL premise ([CR#609.4]) wrapping a clause: "[clause]
    --    treated as though [condition] held." "attack as though it didn't have defender" =
    --    `AsThough (Matches This (Not (HasKeyword Defender))) (Can (Enact Attack (SameAs This) Anyone))`.
    -- (Window-NARROWING `Only` is the `window : Timing` on `Activated` — `AsSorcery`; the
    -- as-though of a deed-INTERNAL participant — "as though the BLOCKER's attacker lacked flying" — is still deferred.)
    Can  : Deed b -> {default Nothing window : Maybe Timing} -> StaticEffect b
    AsThough : Condition b -> StaticEffect b -> StaticEffect b
    Constrain : Compulsion -> Deed b -> StaticEffect b   -- Forbid = a restriction (can't), Require = a requirement (must); the combat solver balances both ([CR#508.1c,508.1d])
    -- a PRICED deed (cost comes FIRST): `AtDeclaration` = paid up front (the old `Gate`, never compulsory);
    -- `Downstream` = punished after the fact (the old `Toll`, ward [CR#702.21a]).
    Priced : TollTiming -> Cost b -> Deed b -> StaticEffect b

  -- A keyword as it sits on a permanent ([CR#702]): either `Bare` — an engine-PRIMITIVE keyword
  -- the grammar can't desugar (FirstStrike/DoubleStrike/Deathtouch/Trample = damage pipeline;
  -- Vigilance = attack event-edit) — or a `Composite` of its tag + the `Ability`s it desugars to:
  -- Flying/Defender/Shroud/Hexproof/Menace → a `cant` (Menace's is SET-level, `BlockedBy`); Reach → `[]` (a flag flying's clause reads, no
  -- ability of its own); Flash → a `Can (Casts …) {window = AsInstant}` (cast at instant speed).
  -- `Keyword` wraps it; `keyword` (Macros) builds it.
  public export
  data KeywordAbility : Bindings -> Type where
    Bare      : KeywordSpec b -> KeywordAbility b
    Composite : KeywordSpec b -> List (Ability b) -> KeywordAbility b

  -- An ability, INDEXED by its context `b`. A card's top-level abilities are `Ability Base`
  -- (source bound, no targets); a keyword desugaring can be `Ability b` so its clause may
  -- reference an anaphor — "protection from the CHOSEN color/player" (Mother of Runes).
  public export
  data Ability : Bindings -> Type where
    Spell : OneShotEffect b -> Ability b
    Keyword : KeywordAbility b -> Ability b
    -- "{cost}: {effect}" — an activated ability ([CR#602]). `window` is its activation timing
    -- (instant by default; `AsSorcery` = "activate only as a sorcery"); `limits` are the
    -- use-frequency caps. A loyalty ability is `{window = AsSorcery, limits = [OncePerTurn]}`.
    Activated : Cost b -> OneShotEffect b -> {default AsInstant window : Timing} -> {default [] limits : List UsageLimit} -> Ability b
    -- a triggered ability: when `event` fires, resolve `effect`. `limits` are use-frequency caps —
    -- "triggers only once each turn" = `{limits = [OncePerTurn]}` — the same `UsageLimit` list
    -- `Activated` carries. Rust: Ability::Triggered.
    Triggered : (q : EventQuery b) -> OneShotEffect (bindEvent (eventQueryCaps q) b) -> {default [] limits : List UsageLimit} -> Ability b
    -- a TURN-BASED action ([CR#703]) intrinsic to the bearer: at `phase`, perform `effect` automatically —
    -- no stack, unlike `Triggered`. The Saga lore-increment ([CR#714.3c], conferred by the Saga subtype):
    -- `TurnBased (MainPhase PreCombat) (Act (PutCounters Lore (^1) This))`. (Was the old `PropTurnBased`.)
    TurnBased : PhaseStep -> OneShotEffect b -> Ability b
    -- (Retired `Enchant`: the engine has no dedicated aura ability — "enchant X" is a `Can (Enact Attach …)`
    --  PERMISSION (attaching is default-forbidden, so the aura ENABLES it), enters-attached an `Also`,
    --  falls-off an `Sba`. No subtype special-casing.)
    -- a static continuous ability — modifications, anthems, AND replacements live in `StaticEffect`.
    Static : StaticEffect b -> Ability b
    -- "[cost]: turn This face up" ([CR#708.9]) — a SPECIAL action (not stack-using), not an `Activated`
    -- ability. Pays [cost], removes `FaceDown`. The face-up cost of `morph`/`disguise`.
    TurnFaceUp : Cost b -> Ability b
    -- "As ~ enters, choose a [d]" ([CR#614.12]) for a VALUE choice: a single ability that makes the
    -- as-enters choice and SCOPES it to the abilities that read it — those nest at `bindChosen d b` (so
    -- `OfChosen`/`ChosenIs` resolve), while the card's other abilities (and its whole printed face) stay
    -- at `b`, untouched. `d` is value-only now; a chosen ENTITY goes through `AsEntersChoosing`.
    AsEnters : (d : ChooseDomain) -> {auto 0 ok : ModeDomainOk d} -> List (Ability (bindChosen d b)) -> Ability b
    -- "As ~ enters, choose a [filtered ENTITY]" ([CR#614.12]) — the game-entity twin of `AsEnters`. The
    -- `Predicate b k` is the choosable set (Clone's "a creature", lost when this was the unconstrained
    -- `AnObjectChoice`); `k` is gated to object/player by `ChoiceRefKind`. The chosen entity binds
    -- `chosenRefKind k`, read back by `ChosenObject`/`ChosenPlayer` in the nested abilities.
    AsEntersChoosing : (k : RefKind) -> {auto 0 ok : ChoiceRefKind k} -> Predicate b k -> List (Ability (bindChosenRef k b)) -> Ability b

-- A card's printed face is just `Characteristics` at the empty bindings.
public export
Face : Type
Face = Characteristics Base

-- A GLOBAL state-based action as data ([CR#704.5]) — the [CR#704.5] tier that belongs to NO object and
-- ranges over the battlefield, unlike the carrier-bound conferred SBA (`StaticEffect::Sba`, an Aura's
-- falls-off rule). `scope` is the binding domain for `This` (checked first — what makes a rule that lives
-- on no card range over objects, exactly the Rust `SbaRule { scope, when, then }`); `when` fires it (with
-- `This` = the scoped object); `thenDo` is the sweep action. This is a factoring of existing pieces
-- (`Predicate`/`Condition`/`OneShotEffect`), not a new primitive. INTRINSIC-keyword SBAs do NOT live here:
-- deathtouch's [CR#704.5h] rides the `Deathtouch` keyword (intrinsic — its prospective lethality rewrite
-- [CR#702.2c] can't be composed), so it is engine-baked, not a data rule.
public export
record SbaRule where
  constructor MkSbaRule
  scope  : Predicate Base AnObject  -- the domain `This` ranges over (e.g. a creature on the battlefield)
  when   : Condition Base           -- fires when this holds, with `This` = the scoped object
  thenDo : OneShotEffect Base       -- the sweep action (`then` is a reserved word, hence `thenDo`)

public export
interface DefaultValue a where
  defaultValue : a

public export
fromDefault : (DefaultValue a) => (a -> a) -> a
fromDefault b = b defaultValue

-- `^: { field := value … }` = `fromDefault` — build a record from its defaults + named overrides.
-- A distinct prefix (overloading `^` is ambiguous on a bare `^1`; `&`/`#` are reserved/builtin) —
-- the caret keeps the "lift into the expected type" flavor of `^`.
export prefix 10 ^:
public export
(^:) : (DefaultValue a) => (a -> a) -> a
(^:) = fromDefault

public export
implementation DefaultValue (Characteristics b) where
  defaultValue = MkCharacteristics
    { name = Nothing
    , manaCost = []
    , colors = []
    , types = []
    , supertypes = []
    , subtypes = []
    , abilities = []
    , power = Nothing
    , toughness = Nothing
    , loyalty = Nothing
    , defense = Nothing
    }

-- [CR#205.3d]: every subtype's governing card type must be among the card's
-- types. The proof is demanded at `Normal`, so `types`/`subtypes` stay plain
-- fields the `^ { … := … }` builder can still set.
public export
SubtypesOk : Characteristics Base -> Type
SubtypesOk c = All (\s => Elem (subtypeCategory s) (types c)) (subtypes c)

-- How a multi-faced card's two faces are arranged ([CR#712] transforming/modal DFC, [CR#709] split,
-- [CR#715] adventurer, [CR#710] flip). The LAYOUT carries the access rules; both faces are full faces.
public export
data FaceLayout = Transforming | ModalDFC | Split | Adventure | Flip

public export
data Card : Type where
  Normal : (c : Characteristics Base) -> {auto ok : SubtypesOk c} -> {auto wf : CharacteristicsOk c} -> Card
  -- a TWO-faced card: `front` (the primary/default face) and `back`, arranged per `layout`. Each face
  -- is a full `Face` with its own well-formedness ([CR#712.8] each face has its own characteristics);
  -- transform / cast-the-other-face is the engine's job — the grammar just holds both faces.
  TwoFaced : (layout : FaceLayout) -> (front : Face) -> (back : Face) ->
             {auto okF : SubtypesOk front} -> {auto wfF : CharacteristicsOk front} ->
             {auto okB : SubtypesOk back} -> {auto wfB : CharacteristicsOk back} -> Card

-- What a counter / subtype / type CONFERS on its bearer is just a list of `Ability`s — the engine's
-- mechanism for intrinsic behavior with NO subtype special-casing (`This` = the bearer). There is no
-- `Property` wrapper: a conferral IS an ability (`Static (Modify …)` for a continuous self-mod, `Static
-- (Sba …)` for an SBA, `TurnBased …` for a turn-based action, a plain `Static`/keyword otherwise).
-- Closed; attached via the total functions below (the dependent-index style of `counterScope`/
-- `designationScope`), not an open registry.

-- +1/+1 and −1/−1 carry their OWN P/T pump (a `Static (Modify …)`), so it's not a hard-coded engine rule
-- (`CountersOn c This` reads the count). The rest confer nothing intrinsic.
public export
counterConfers : CounterKind -> List (Ability b)
counterConfers P1P1 = [Static (Modify (SelectAll (SameAs This)) [ModifyPT (Up (CountersOn P1P1 This)) (Up (CountersOn P1P1 This))])]
counterConfers M1M1 = [Static (Modify (SelectAll (SameAs This)) [ModifyPT (Down (CountersOn M1M1 This)) (Down (CountersOn M1M1 This))])]
counterConfers _    = []

-- what a SUBTYPE confers on its bearer. The Aura falls-off SBA ([CR#704.5m], a `Static (Sba …)`) and the
-- Saga lore-increment ([CR#714.3c], a `TurnBased` action) live here — shared rules, not per-card statics,
-- and never a subtype `if`-branch.
public export
subtypeConfers : Subtype -> List (Ability b)
subtypeConfers (EnchantmentSub Aura) = [Static (Sba (Not (LegallyAttached This)) (Act (Move This (ToZone Graveyard))))]
subtypeConfers (EnchantmentSub Saga) = [TurnBased (MainPhase PreCombat) (Act (PutCounters Lore (^1) This))]
subtypeConfers _                     = []

-- what a card TYPE confers on its bearer (parallel to `subtypeConfers`). A Planeswalker or Battle CREATES
-- a deontic permitting creatures to attack IT ([CR#508.1] — attackability is a granted permission, not a
-- hardcoded target list), using `Enact Attack` with the permanent itself as the object (patient) defender.
public export
typeConfers : Type_ -> List (Ability b)
typeConfers Planeswalker = [Static (Can (Enact Attack (HasType Creature) (SameAs This)))]
typeConfers Battle       = [Static (Can (Enact Attack (HasType Creature) (SameAs This)))]
typeConfers _            = []

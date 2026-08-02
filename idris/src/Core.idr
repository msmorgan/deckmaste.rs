||| Core grammar of the toy MTG card model: characteristics, the antecedent
||| STACK binding context (`Ctx` — what references are in scope, v2), and the
||| filter / reference / selection / action / effect / ability trees. Kept
||| deliberately brief.
module Core

import public Data.Vect
import public Data.Nat
import public Data.List
import public Data.List.Elem
import public Data.List.Quantifiers
import public Data.Maybe

%default total

namespace Color
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

namespace SimpleManaSymbol
  public export
  data SimpleManaSymbol
    = Generic Nat
    | Specific (Maybe Color)

namespace ManaSymbol
  public export
  -- the PRINTED cost language ([CR#107.4]) — what appears on a card as a mana cost. NOT what a mana
  -- ability produces (that's `ProducedMana` below — a different domain; the user's distinction).
  -- Slightly more permissive than [CR#107.4]: the hybrid left half is any SimpleManaSymbol, so
  -- unprinted forms like `{5/W}` are representable on purpose (variant/design headroom).
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
Promote Nat ManaSymbol where
  promote = Simple . Generic

public export
Promote Integer ManaSymbol where
  promote = promote . integerToNat

public export
Promote Color ManaSymbol where
  promote = Simple . Specific . Just

-- `^Colorless` = {C} (and `^(Just c)` = {c}); `Specific Nothing` is the colorless pip.
public export
Promote (Maybe Color) ManaSymbol where
  promote = Simple . Specific

-- PRODUCED mana ([CR#106.1]) — actual mana a mana ability adds. A DIFFERENT domain from the printed
-- cost `ManaSymbol`: you produce colored/colorless units or "any color", never `{X}`/`{W/P}`/`{S}`.
-- (`data ProducedMana` itself lives in the mutual block, beside `ManaRider`, so its `AmongColorsOf`/
-- `ProducedByEvent` variants can name `Reference`/`eventCaps`.)

public export
ManaCost : Type
ManaCost = List ManaSymbol

namespace Type_
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

namespace Zone
  public export
  data Zone
    = Battlefield
    | Command
    | Exile
    | Graveyard
    | Hand
    | Library
    | Sideboard   -- a private, searchable, UNORDERED out-of-game store ([MTR 2.4] sideboard) — Wishes (Burning/Living Wish)
                  -- fetch from here. NOT the CR's nebulous "outside the game": a concrete zone (deck 75 = library 60 + sideboard 15).
    | Stack

-- Subtypes are partitioned by card type [CR#205.3g..205.3q]. A subtype's
-- CATEGORY is the group of card types it may appear on: usually one, but a
-- creature type sits on both Creature and Kindred cards, and a spell type on
-- both Instant and Sorcery — so a single `Type_` won't serve. `categoryTypes`
-- lists that group; `SubtypesOk` demands a card carry at least one of them.
namespace Category
  public export
  data Category = Creature | Enchantment | Artifact | Land | Battle | Planeswalker | Spell

public export
categoryTypes : Category -> List Type_
categoryTypes Creature     = [Creature, Kindred]
categoryTypes Enchantment  = [Enchantment]
categoryTypes Artifact     = [Artifact]
categoryTypes Land         = [Land]
categoryTypes Battle       = [Battle]
categoryTypes Planeswalker = [Planeswalker]
categoryTypes Spell        = [Instant, Sorcery]

-- NOTE: `Subtype` itself is an OPEN name-carrying type whose third field is
-- `List (Ability Base)` (the conferred abilities), so its definition (and
-- `subtypeCategory`) live DOWN in the big mutual block, next to `Ability`.

-- Leaf types used inside the filter/condition language ---------------------

-- how to ROUND a fractional value — the spell/ability states the direction ([CR#107.1a]). Shared by the
-- `AverageOf` fold AND the `Half` value constructor (one rounding vocabulary, not two).
namespace RoundMode
  public export
  data RoundMode = RoundUp | RoundDown

-- how to FOLD a collection of values into ONE: the sum, the least, the greatest, or the (rounded) mean.
-- `SumOf` of the EMPTY collection is 0 ([CR#107.2] — a value that can't be determined is 0); `MinOf`/`MaxOf`/
-- `AverageOf` of the empty collection are genuinely undefined (no extremum; a 0/0 mean), so the ENGINE must
-- guard the empty case — the grammar can't, there is no cardinality evidence here. Shared by object-projection
-- (`Aggregate` over a `Projection`) and event-amounts (`EventAgg`). Replaces the old `Stat`-only `AggOp`
-- (Total/Greatest/Least -> SumOf/MinOf/MaxOf), adding the mean. (Cardinality is `CountOf`, not an op.)
namespace AggregateOp
  public export
  data AggregateOp = SumOf | MinOf | MaxOf | AverageOf RoundMode

-- the EXTREMAL ops — the subset of `AggregateOp` along which you can pick an extremal ELEMENT (`Pick`). A
-- PROOF (the file's superset-enum + legal-subset idiom, cf. `Projectable`/`IsCharDomain`), NOT a parallel
-- `Highest`/`Lowest` enum re-encoding min/max: so `Pick MinOf`/`Pick MaxOf` are the only well-typed picks
-- and `Pick SumOf`/`Pick (AverageOf …)` are unrepresentable.
namespace IsExtremal
  public export
  data IsExtremal : AggregateOp -> Type where
    MinIsExtremal : IsExtremal MinOf
    MaxIsExtremal : IsExtremal MaxOf

-- a small predicate algebra over MANA SYMBOLS, for filtering a printed/spent cost ([CR#107.4]). `CountsAs c`
-- = the symbol counts as colour c (hybrid {W/U} counts as both; Phyrexian {W/P} as white; a generic symbol
-- as none) — the one home for "counts as colour" identity. Its own namespace: `And`/`Or`/`Not` are shared
-- combinator names (Predicate/Condition/EventQuery each carry their own), disambiguated by expected type.
namespace SymbolPred
  public export
  data SymbolPred : Type where
    CountsAs : Color -> SymbolPred
    IsGeneric : SymbolPred
    -- `And`/`Or` are NON-EMPTY: an empty conjunction/disjunction of symbol filters is meaningless (and a
    -- vacuous `Or []` is exactly "devotion to no colours", which the old `Devotion`'s `NonEmpty` guard rejected
    -- — preserved here rather than dropped).
    And : (ps : List SymbolPred) -> {auto 0 ne : NonEmpty ps} -> SymbolPred
    Or : (ps : List SymbolPred) -> {auto 0 ne : NonEmpty ps} -> SymbolPred
    Not : SymbolPred -> SymbolPred

namespace Cmp
  public export
  data Cmp = Eq | AtLeast | AtMost | Greater | Less

-- What kind of object a filter matches ([CR#109.3]). Rust: ObjectKind.
namespace ObjectKind
  public export
  data ObjectKind = Card | Emblem | Spell | Token | Ability

-- Supertypes ([CR#205.4a]); independent of card type and subtype.
namespace Supertype
  public export
  data Supertype = Basic | Legendary | Ongoing | Snow | World

-- A CHARACTERISTIC axis of an object ([CR#109.3]/[CR#613]) — name, colour, types, power/toughness, defense,
-- mana cost, … ONE vocabulary shared by the reads (`StatOf`/`StatCmp`/`CountDistinct`, in the mutual block
-- below) and the writes (`ModificationOp`/`Alter`, further down). Defined HERE so both sides see it. Membership
-- IS settability for the SETTABLE axes: each can be `Set`. The DERIVED mana value is NOT a characteristic
-- ([CR#202.3]) — it has no axis here and is read via `ManaValueOf`. `BasicLandTypes` ([CR#205.3i]) is the
-- one READ-ONLY axis — the count of DISTINCT basic land types (Domain, [CR#207.2c]), NOT the whole `Subtypes`
-- axis (a Gate's `Gate` subtype never counts). It is unsettable by construction (`CharValue … BasicLandTypes =
-- Void`, so `Set`/`Up`/`Add` can't name it), readable only via `CountDistinct`. The object's actual VALUE
-- (`Maybe Int`, …) is the engine's; the grammar only *specifies* it (via `Count`) and *classifies* the axis
-- (the families below), never materializes it (no `valueOf` — it couldn't cross the RON boundary). Its own
-- `namespace` keeps the leaf names tidy (`Name`/`Defense`/… are generic).
namespace Characteristic
  public export
  data Characteristic = Colors | Types | Subtypes | Supertypes | Power | Toughness | Defense | ManaCost | Name
                      | BasicLandTypes   -- READ-ONLY: distinct-basic-land-type count (Domain); unsettable (CharValue = Void)

-- a PLAYER's numeric attributes — the player-side twin of the object `Characteristic`
-- numeric axes. Read via `PlayerStatOf` (a `Count`), mirroring `StatOf` for objects.
-- `HandSizeLimit` (normally 7) and `LandPlaysPerTurn` (normally 1) are caps modified by
-- statics (Reliquary Tower / Exploration) via `ModifyPlayer`; "no maximum" is `NoMax` (not a value).
namespace PlayerAttr
  public export
  data PlayerAttr = Life | HandSize | HandSizeLimit | LandPlaysPerTurn

-- the numeric axes — gate `Up`/`Down` (write delta, layer 7c) AND the numeric reads (`StatOf`/`StatCmp`/
-- `TapTotal`). `Defense` is lax-included (setting/pumping it is a describable no-op, like loyalty). The file's
-- `IsCharDomain` idiom: a Type-returning subset fn (`()`/`Void`), not a Bool flag.
public export
Numeric : Characteristic -> Type
Numeric Power     = ()
Numeric Toughness = ()
Numeric Defense   = ()
Numeric _         = Void

public export
Collection : Characteristic -> Type     -- accepts element `Add`/`Remove` (the "set-kinded" list axes)
Collection Colors     = ()
Collection Types  = ()
Collection Subtypes   = ()
Collection Supertypes = ()
Collection _          = Void

-- NOTE: `ElemOf` moved DOWN into the big mutual block (next to `CharValue`),
-- because its `Subtypes` clause returns `Subtype`, which now lives there.

-- The word classes a TEXT-CHANGE effect may swap ([CR#612.1]): a color word (white/blue/…) or a basic
-- land type (Plains/Island/…). Mind Bend allows either; the specific words are a player's choice.
namespace TextWordClass
  public export
  data TextWordClass = ColorWords | BasicLandTypes

-- `CounterKind` is now an OPEN name-carrying value (`MkCounterKind Scope String (List (Ability Base))`),
-- defined inside the big mutual block alongside `Subtype` (its conferral field references `Ability`). Its
-- scope projection `counterKindScope` (replacing the old closed `counterScope`) indexes the counter ops.

-- A timing WINDOW — the speed at which an action is allowed: `AsInstant` (any time you have
-- priority) or `AsSorcery` (your main phase, empty stack — [CR#601.3,602.5d]). The ONE timing
-- notion, shared by a deontic `Can (Casts …)` (Flash widens to `AsInstant`, [CR#702.8a]) and
-- by `Activated` (instant by default; "activate only as a sorcery" narrows to `AsSorcery`).
namespace Timing
  public export
  data Timing = AsInstant | AsSorcery

-- Activation USE-LIMITS on an activated ability ([CR#602.5b]) — frequency caps, NOT timing (that's
-- `Timing` above; the two used to overlap on a `SorcerySpeed` constructor). A loyalty ability
-- is `{window = AsSorcery, limits = [LoyaltyOncePerTurn]}` — `LoyaltyOncePerTurn` [CR#606.3,306.5d]
-- is SHARED across every loyalty ability a permanent carries (activating any one blocks the rest for
-- the turn), unlike `OncePerTurn`, which is per-ability.
namespace UsageLimit
  public export
  data UsageLimit = OncePerTurn | OncePerGame | LoyaltyOncePerTurn

-- Runtime object STATE (not a printed characteristic) — what a `HasState` predicate tests
-- ([CR#701.20] tap, [CR#302.6] summoning sickness, [CR#702.26] phasing, [CR#708] face-down). The RELATIONAL
-- states moved to the spine: combat (attacking/blocking/blocked) is `Holds Attack/Block Agent/Patient`, and
-- "attached" is `Holds Attach Agent` — none are `HasState`. Negatives via `Not` ("untapped" = `Not (HasState
-- Tapped)`). `SummoningSick` is what `haste` lifts — "as though not summoning-sick" (`AsThough`, see Macros).
namespace ObjectState
  public export
  data ObjectState = Tapped | SummoningSick
                   | Untapped        -- untapped ([CR#502.3]); "becomes untapped" = `Becomes Untapped` (the untap event, symmetric with `Becomes Tapped`); `HasState Untapped` = "an untapped permanent". "Doesn't untap during your untap step" is a window-scoped SKIP: `Replaces [Becomes Untapped] [Agent (SameAs This), <during your untap step>] (Sequentially [])`
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
namespace RefKind
  public export
  data RefKind = Empty | AnObject | APlayer | Anything

-- One vs many: the type-level CARDINALITY of a binder. A one-binder's `That` is a single `Reference`; a
-- many-binder's `That` is a `Selection` you `Each` over. `^1`/`^3` share the one `Quantity` type, so
-- cardinality can't ride the quantity value — it lives in the `Bindable` CONSTRUCTOR (`ChooseOne` vs `Choose`).
public export
data Cardinality = One | Many

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

-- The closed SCOPE of an open counter/designation ([CR#122.1] carrier): a counter kind / designation is
-- borne by an OBJECT or a PLAYER. This is the one closed field of the otherwise open `MkCounterKind` /
-- `MkDesignation` values, mapped into `RefKind` by `scopeRef` to index the counter/designation ops
-- dependently (players-are-objects: the `Reference` language already names players, so a player-carried
-- counter needs no new machinery, just this scope).
namespace Scope
  public export
  data Scope = Object | Player

public export
scopeRef : Scope -> RefKind
scopeRef Object = AnObject
scopeRef Player = APlayer

-- RELATION SPINE. A `Relation` is an agent→patient relation the game tracks; from ONE relation we derive
-- three ASPECTS — durative (`Holds`, a state predicate), inchoative (`Begins`, an event), deontic (`Enact`,
-- a deed). `agentScope` fixes the AGENT's kind per relation (the Agent/Actor resolution: ONE agent slot, an
-- OBJECT for combat/attach/target/counter, a PLAYER for cast/activate/play) — `counterKindScope`'s sibling.
-- The PATIENT stays kind-poly (an attack's defender is a player/planeswalker/battle). The constructors are
-- NAMESPACED — `Target`/`Counter`/`Attach` clash with the TargetSpec/Action of the same
-- name — and disambiguate by type, like `Facet.Patient`/`Role.Patient` share `Patient`.
namespace Relation
  public export
  data Relation = Attack | Block            -- combat
                | Cast | Activate | Play     -- the stack: a PLAYER casts a spell / activates an ability / plays a card
                | Attach                      -- an aura/equipment (object) attaches to a host
                | Target | Counter           -- a source (spell/ability, object) targets / counters an object
                | Regenerate                  -- a player regenerates an object ([CR#701.19]); lets `cant (Enact Regenerate Anyone (SameAs This))` express "can't be regenerated"

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
agentScope Regenerate = APlayer  -- the controller regenerates the creature

-- `agentScope`'s twin for the PATIENT slot: what kind of participant each relation acts upon.
-- Attack reaches players, planeswalkers, and battles ([CR#508.1b]) and Target anything targetable
-- ([CR#115.4]) — both object-or-player, `Anything`; Attach reaches "an object or player"
-- ([CR#701.3a]); everything else acts on an object — a blocked ATTACKER ([CR#509.1a]), a cast/played
-- card ([CR#601.2a,701.18a]), an object's activated ability ([CR#113.3b]), a countered spell/ability
-- ([CR#701.6a]), a regenerated permanent ([CR#701.19a]).
public export
patientScope : Relation -> RefKind
patientScope Attack   = Anything
patientScope Block    = AnObject
patientScope Cast     = AnObject
patientScope Activate = AnObject
patientScope Play     = AnObject
patientScope Attach   = Anything
patientScope Target   = Anything
patientScope Counter  = AnObject
patientScope Regenerate = AnObject

-- the two participant SLOTS, as role selectors for the durative aspect (`Holds Attack Agent` = an attacker,
-- `Holds Block Patient` = a blocked creature). `Agent`/`Patient` are the SAME role pair the event `Facet`s
-- use — ONE vocabulary across the spine's aspects. (`Actor`, the responsible PLAYER, is a separate axis,
-- not a role.) Unifies the old `Attacking`/`Blocking`/`Blocked` states.
namespace Role
  public export
  data Role = Agent | Patient

-- DESIGNATIONS (the 700-ish global flags: monarch, the initiative, city's blessing, monstrous,
-- goaded, renowned, suspected, saddled, solved…) are now an OPEN name-carrying value
-- (`MkDesignation Scope String (List (Ability Base))`), defined inside the mutual block alongside
-- `Subtype`/`CounterKind`. Its scope projection `designationKindScope` (replacing the old closed
-- `designationScope`) indexes the `HasDesignation`/`GrantDesignation` pair — carrier enforced dependently.

namespace BeginningStep
  public export
  data BeginningStep
    = UntapStep
    | UpkeepStep
    | DrawStep

namespace CombatStep
  public export
  data CombatStep
    = BeginningOfCombatStep
    | DeclareAttackersStep
    | DeclareBlockersStep
    | FirstCombatDamageStep
    | CombatDamageStep
    | EndOfCombatStep

namespace EndingStep
  public export
  data EndingStep
    = EndStep
    | CleanupStep

-- a turn has exactly TWO main phases ([CR#505.1]) — a closed enum, not an open `Nat`.
namespace MainPhaseKind
  public export
  data MainPhaseKind = PreCombat | PostCombat

namespace PhaseStep
  public export
  data PhaseStep
    = BeginningPhase BeginningStep
    | MainPhase MainPhaseKind
    | CombatPhase CombatStep
    | EndingPhase EndingStep

public export
Promote BeginningStep PhaseStep where
  promote = BeginningPhase
public export
Promote CombatStep PhaseStep where
  promote = CombatPhase
public export
Promote EndingStep PhaseStep where
  promote = EndingPhase

-- A history-lookback / timing scope for an `EventQuery`. Rust: Lookback.
namespace Window
  public export
  data Window = ThisGame | ThisTurn | LastTurn | ThisCombat | ThisStep

-- the planar die's face ([CR#901.3a] — one Planeswalker symbol, one chaos symbol, four blanks) —
-- the four blank faces collapse to one `Blank`, a no-op roll ([CR#901.9a]); `Chaos` triggers the
-- chaos ability ([CR#901.9b,311.7]), `Planeswalker` the planeswalking ability ([CR#901.8,901.9c]).
namespace PlanarFace
  public export
  data PlanarFace = Blank | Chaos | Planeswalker

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
    -- `toKind` is the RECIPIENT's kind — also intrinsic event data riding the kind, keeping the patient
    -- binder sound under caps-from-kinds ([CR#120.3] "a player or permanent"): `Just APlayer` = "damage
    -- to a player" → `EventPatient : Reference b APlayer`; `Just AnObject` = "to a creature/permanent";
    -- `Nothing` (default) = any recipient, patient unreachable. The `Patient` FACET still refines WHICH.
    DealDamage : Maybe Bool -> {default Nothing toKind : Maybe RefKind} -> EventKind
    CreateToken : EventKind
    PutCounters : EventKind
    RemoveCounters : EventKind
    Destroy : EventKind
    -- the remaining keyword-action verbs ([CR#701]) as event kinds, REUSING the
    -- `KeywordActionSpec`/`Action` verb names (like `Draw`/`Discard` above) so an
    -- `EventFilter::Act {verb, who, on}` lowers here: the verb pins the kind, the
    -- performer rides the `Actor` facet and the affected object the `Agent` facet.
    -- This is what makes "whenever you scry/mill/fateseal", "whenever a creature
    -- fights", and cant/replace over those verbs match on the finalized name-fact
    -- ([CR#616.1,701.9c]). No new axis — verb kinds sit beside `Destroy`.
    Mill : EventKind
    Scry : EventKind
    Surveil : EventKind
    Fateseal : EventKind
    Fight : EventKind
    ZoneChanged : Maybe Zone -> Maybe Zone -> EventKind
    BeginStep : PhaseStep -> EventKind
    -- "whenever ~ BECOMES [state]" — TRANSITION states only (gated; not `SummoningSick`).
    Becomes : (s : ObjectState) -> {auto 0 prf : IsBecomesState s} -> EventKind
    -- the ONSET of a relation ([CR#508],[CR#509]) — the inchoative aspect of the relation spine. `Begins
    -- Attack` fires once per attack; FACETS pick the side (`[Agent This]` = it attacks, `[Patient you]` =
    -- you're attacked). Unifies the paired `Becomes Attacking`/`Becomes Attacked` (one happening, two
    -- views). The STACK relations fold in too: a cast is `Begins Cast` (no bespoke `Cast` kind — it was
    -- redundant), the caster supplied as its actor (the `agentScope`-driven caps below).
    Begins : Relation -> EventKind
    -- life-change ([CR#119.3]) — the EVENT twin of the `GainLife`/`LoseLife` actions (verb names reused,
    -- like `Draw`/`Discard`): "whenever you gain/lose life". Supplies the affected player (actor) + the amount,
    -- so `EventAgg`/`ReplaceAmount` work over them. No object, no patient.
    GainLife : EventKind
    LoseLife : EventKind
    -- control change ([CR#613.1b]) — NOT a zone change, so `ZoneChanged` misses it. Named for the GAINER so
    -- the `Actor` is unambiguous (the NEW controller): "whenever you gain control of a creature" =
    -- `[GainControl] [Actor you, Agent creature]`. Verb-name reuse with the `GainControl` MODIFICATION
    -- (type-directed, like `GainLife`). Supplies the permanent (object) + the gainer (actor); no amount.
    GainControl : EventKind
    -- "whenever a land is tapped for mana" ([CR#106.12]) — the ONE event kind whose `producesMana` cap is
    -- True (it IS a mana ability resolving), gating `ProducedByEvent` ([CR#106.12a]). Object = the tapped
    -- permanent, actor = its controller (the tapper); Vorinclex/Dictate of Karametra read the object's
    -- production via `ProducedByEvent`, Nyxbloom/Mana Reflection the amount via `ReplaceAmount`/`EventAmount`.
    TapForMana : EventKind
    -- a numeric die roll ([CR#706.1,706.2]) — the amount is the RESULT rolled (Wyll's die-roll payoffs
    -- read it via `EventAmount`). No object/patient; the actor is the roller.
    RollDice : EventKind
    -- a coin flip ([CR#705.1,705.2]) — `Just True` = won / `Just False` = lost / `Nothing` = any flip,
    -- intrinsic event data riding the kind (like `DealDamage`'s combat flag). No amount of its own; the
    -- ENGINE fact is win/loss, not a number ([CR#705.2] "the player who called it wins the flip").
    FlipCoin : Maybe Bool -> EventKind
    -- the fixed six-face planar die ([CR#901.3a]), face wildcarded like `ZoneChanged`'s zones
    -- (`Nothing` = any face). NO amount — the face is not numeric ([CR#901.9d]), so a query unioning
    -- this with `RollDice` loses `kindsHaveAmount` ([CR#706.7]: a "roll dice, planar die included"
    -- effect can't read a result).
    RollPlanarDie : Maybe PlanarFace -> EventKind

-- the per-event CAPABILITIES an event provides its body's anaphora: a distinguished OBJECT ("that card"),
-- an ACTOR ("that player"), a numeric AMOUNT. Read by `EventObject`/`EventActor`/`EventAmount` so each is
-- valid ONLY where the event actually supplies it — the invalid-reference gate.
public export
record EventCaps where
  constructor MkEventCaps
  hasObject : Bool
  hasActor  : Bool
  hasAmount : Bool
  patientKind : Maybe RefKind   -- the ACTED-UPON thing's kind when the event FIXES one (gates `EventPatient`); distinct from the Agent exposed via `hasObject`. `Nothing` = no sound patient binder
  hasDefender : Bool            -- combat scope: a defending player is in scope (gates `DefendingPlayer`) ([CR#506.2])
  producesMana : Bool           -- the event IS a mana ability resolving and producing mana (gates `ProducedByEvent`) ([CR#106.12a])

public export
NoCaps : EventCaps
NoCaps = MkEventCaps False False False Nothing False False

-- what each event-kind supplies. Damage/token/counter carry an amount; a step-begin carries nothing; a
-- zone-change/destroy/becomes has an object but no actor; a cast/draw/discard/sacrifice has an actor.
public export
eventKindCaps : EventKind -> EventCaps
eventKindCaps Sacrifice         = MkEventCaps True  True  False Nothing       False False
-- a draw's per-fact amount is ONE card ([CR#121.2] -- drawn one at a time):
-- the fact channel sums draws ("for each card drawn"), while the multi-card
-- amount BOUND stays bridge-capped (`Drawn:amount`).
eventKindCaps Draw              = MkEventCaps False True  True  Nothing       False False
eventKindCaps GainLife          = MkEventCaps False True  True  Nothing       False False
eventKindCaps LoseLife          = MkEventCaps False True  True  Nothing       False False
eventKindCaps GainControl       = MkEventCaps True  True  False Nothing       False False
-- a discard's amount is its card count ([CR#701.9a]; the engine's apply
-- funnel fixes the batch size) -- Collective Defiance's "then draws that
-- many cards" reads it.
eventKindCaps Discard           = MkEventCaps True  True  True  Nothing       False False
-- damage: the source object is the Agent (`hasObject`), its controller the actor, the amount the amount,
-- and the RECIPIENT's kind rides the kind as `toKind` → the patient (`EventPatient`).
eventKindCaps (DealDamage _ {toKind}) = MkEventCaps True True True toKind False False
eventKindCaps CreateToken       = MkEventCaps True  True  True  Nothing       False False
eventKindCaps PutCounters       = MkEventCaps True  True  True  Nothing       False False
eventKindCaps RemoveCounters    = MkEventCaps True  True  True  Nothing       False False
eventKindCaps Destroy           = MkEventCaps True  False False Nothing       False False
-- the slice-family / fight keyword-action kinds (their `KeywordActionSpec` args ARE these caps):
-- mill ([CR#701.17a]) is the graveyard twin of Draw — a slice verb whose PERFORMER rides the actor
-- ("whenever an opponent mills"), per-fact amount ONE card (multi-card forms ride the Batch tier), no
-- distinguished object.
eventKindCaps Mill              = MkEventCaps False True  True  Nothing       False False
-- scry/surveil ([CR#701.22a,701.25a]) act on YOUR library by definition, so they carry ONLY the count —
-- no performer (actor) and no bound object (the reordered / graveyard'd cards aren't distinguished).
eventKindCaps Scry              = MkEventCaps False False True  Nothing       False False
eventKindCaps Surveil           = MkEventCaps False False True  Nothing       False False
-- fateseal ([CR#701.29a]) names the fatesealed player (an opponent — the actor "that player") plus the
-- count; no bound object (that library's cards aren't distinguished).
eventKindCaps Fateseal          = MkEventCaps False True  True  Nothing       False False
-- fight ([CR#701.14a]): two fighter creatures — one rides the object (agent), the other the patient
-- (`Just AnObject`). No player actor, no amount, and the damage dealt isn't combat damage ([CR#701.14d]).
eventKindCaps Fight             = MkEventCaps True  False False (Just AnObject) False False
eventKindCaps (ZoneChanged _ _) = MkEventCaps True  False False Nothing       False False
eventKindCaps (BeginStep _)     = MkEventCaps False False False Nothing       False False
eventKindCaps (Becomes _)       = MkEventCaps True  False False Nothing       False False
-- a relation-ONSET supplies the agent's player as "that player" ONLY when the agent IS a player
-- (cast/activate/play); an object-agent onset (combat/attach/target/counter) reaches the controller via
-- `ControlledBy`. There is always a distinguished object, never an amount. Combat onsets (`Attack`/`Block`)
-- additionally put the DEFENDING PLAYER in scope (`DefendingPlayer`) — always a player, even vs a
-- planeswalker/battle ([CR#506.2,508.5]).
eventKindCaps (Begins Attack)   = MkEventCaps True  False False Nothing       True  False
eventKindCaps (Begins Block)    = MkEventCaps True  False False Nothing       True  False
eventKindCaps (Begins r)        =
  case agentScope r of
    APlayer => MkEventCaps True True  False Nothing       False False
    _       => MkEventCaps True False False Nothing       False False
-- the tapped land is the object, its controller the actor, the mana produced the amount, and
-- `producesMana` (the trailing `True` — the ONLY row that's True) gates `ProducedByEvent` ([CR#106.12,106.12a]).
eventKindCaps TapForMana        = MkEventCaps True  True  True  Nothing       False True
-- randomness events ([CR#705.1,706.1,706.2,901.9d]): no object, actor = the roller/flipper, no
-- patient/defender/mana. `RollDice` carries an amount (the result); the coin flip and planar die don't
-- (a flip's fact is win/loss, the die's is a face) — this asymmetry is what makes the
-- `[RollDice, RollPlanarDie Nothing]` union amountless ([CR#706.7]).
eventKindCaps RollDice          = MkEventCaps False True  True  Nothing       False False
eventKindCaps (FlipCoin _)      = MkEventCaps False True  False Nothing       False False
eventKindCaps (RollPlanarDie _) = MkEventCaps False True  False Nothing       False False

-- which event-kinds carry an AMOUNT — derived from the caps; the per-kind base for `kindsHaveAmount`.
public export
eventKindHasAmount : EventKind -> Bool
eventKindHasAmount k = hasAmount (eventKindCaps k)

-- whether an event-QUERY's kind-disjunction guarantees an amount to sum/replace (gates `ReplaceAmount`/
-- `EventSum`): EVERY listed kind must carry one, and empty `kinds` (any kind) guarantees nothing. This is
-- `hasAmount (eventQueryCaps q)` by construction (the query's caps INTERSECT over its disjunction), phrased
-- over the `kinds` list so it is in scope for the constructor signatures (above the `EventQuery` record).
public export
kindsHaveAmount : List EventKind -> Bool
kindsHaveAmount []        = False
kindsHaveAmount (k :: ks) = eventKindHasAmount k && all eventKindHasAmount ks

-- per-kind base for `isRandomnessQuery`: is this a roll-more-eligible randomness kind
-- ([CR#614.3] Krark's Thumb-family replacements)? `RollDice`/`FlipCoin` yes; the planar die
-- is excluded ([CR#901.9d] — no roll-more interaction for it).
public export
kindIsRandomness : EventKind -> Bool
kindIsRandomness RollDice     = True
kindIsRandomness (FlipCoin _) = True
kindIsRandomness _            = False

-- whether an event-QUERY's kind-disjunction is entirely roll-more-eligible randomness — EVERY listed
-- kind must be `RollDice`/`FlipCoin`, and empty `kinds` (any kind) doesn't qualify. Mirrors
-- `kindsHaveAmount`'s list-then-query split (list helper here, `EventQuery`-typed wrapper beside
-- `eventQueryHasAmount` once `EventQuery` is in scope).
public export
kindsAreRandomness : List EventKind -> Bool
kindsAreRandomness []        = False
kindsAreRandomness (k :: ks) = kindIsRandomness k && all kindIsRandomness ks

-- A VALUE-choice DOMAIN: what an as-enters "choose …" picks from when the pick is a VALUE, not a game
-- entity ([CR#614.12]). The chosen value is bound in `Ctx.chosenKind` and read back by `OfChosen`
-- (characteristic domains) / `ChosenIs` (mode) / `ChosenNumber`. Characteristic domains (color / creature
-- type / name) name something an object can HAVE; a mode/number domain won't. Choosing a game ENTITY (a
-- creature to copy, a player) is NOT a value — it's a filtered `AsEntersChoosing` binding `chosenRefKind`.
namespace ChooseDomain
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
ChoiceRefKindOk : RefKind -> Type
ChoiceRefKindOk AnObject = ()
ChoiceRefKindOk APlayer  = ()
ChoiceRefKindOk _        = Void

-- ===========================================================================
-- THE ANTECEDENT STACK (v2). The binding context is an ordered stack of
-- ANTECEDENTS — one entry per thing the text has introduced (an announced
-- target slot [CR#115.3], a producing clause's moved/created object
-- [CR#400.7], an event body's roles [CR#608.2k], a binder's choice
-- [CR#608.2d], a loop element) — and references are SORTED ANAPHORS resolved
-- against it: R1 nearest-compatible, R2 uniqueness gate, R3 strictly leftward
-- (the telescope `Sequentially` below threads introductions in sentence order).
-- Replaces the v1 role-named `Ctx` record. The Rust side no longer
-- re-implements this resolution: each Rust card is checked by re-emitting it
-- as a raw Core.idr term and typechecking it here (`cargo xtask idris-check`),
-- so this model is the single source of truth for anaphor soundness. (At
-- ENGINE eval time the Rust runtime resolves anaphora dynamically via
-- `Frame`/`Anaphora`, degrading an unresolvable read to the null object — the
-- never-crash runtime, separate from this compile-time soundness gate.)
-- ===========================================================================

-- The SORT of an anaphor/antecedent — the English NOUN it answers to
-- ("that card" / "that creature" / "those tokens"). Mirrors the Rust `Sort`.
-- `OfType t` is "that creature"/"that
-- land"/… ([CR#205.2a]); `Card` is a non-battlefield object ([CR#108.2]);
-- `Amount` is a value antecedent ("that many/much", [CR#608.2i]).
namespace Sort
  public export
  data Sort : Type where
    Player : Sort                -- a player ([CR#102.1])
    Card : Sort                  -- a non-battlefield object ([CR#108.2]); tokens aren't cards ([CR#108.2b])
    Token : Sort                 -- a created token ([CR#111.1])
    Spell : Sort                 -- a spell on the stack ([CR#112.1])
    StackObject : Sort           -- a spell or ability ([CR#405.1]) — the widened stack noun
    Permanent : Sort             -- a battlefield object of unknown type ([CR#110.1])
    OfType : Type_ -> Sort       -- a typed battlefield object — "that creature" ([CR#110.1,205.2a])
    Amount : Sort                -- a value antecedent — "that many"/"that much" ([CR#608.2i])
    Pile : Sort                  -- a labeled pile group ([CR#700.3a])

-- structural equality on card types / sorts — plain Bool functions (not an
-- interface) so type-level `resolve` reduces without instance search.
public export
sameType : Type_ -> Type_ -> Bool
sameType Artifact Artifact = True
sameType Battle Battle = True
sameType Creature Creature = True
sameType Enchantment Enchantment = True
sameType Instant Instant = True
sameType Kindred Kindred = True
sameType Land Land = True
sameType Planeswalker Planeswalker = True
sameType Sorcery Sorcery = True
sameType _ _ = False

public export
sameSort : Sort -> Sort -> Bool
sameSort Player Player = True
sameSort Card Card = True
sameSort Token Token = True
sameSort Spell Spell = True
sameSort StackObject StackObject = True
sameSort Permanent Permanent = True
sameSort (OfType a) (OfType b) = sameType a b
sameSort Amount Amount = True
sameSort Pile Pile = True
sameSort _ _ = False

-- R1 sort COMPATIBILITY: which antecedent nouns an anaphor of each wanted
-- noun reaches (`Spec.tCompatRows` pins this function's valuation over one
-- representative Sort per key). "That card" never reaches a
-- token ([CR#108.2b]); "that permanent" reaches any battlefield noun
-- ([CR#110.1]); "spell or ability" reaches a spell ([CR#405.1]); a typed
-- noun reaches exactly its own type ([CR#205.2a]). (The exact-vs-widened
-- distinction feeds the R2 gate's ONE pre-approved loosening,
-- `exact_sort_precedence` — calibrated and FROZEN STRICT=off by the corpus
-- dry-run, so the flag is not modeled here.)
public export
compat : (want : Sort) -> (have : Sort) -> Bool
compat Player Player = True
compat Card Card = True
compat Token Token = True
compat Spell Spell = True
compat StackObject Spell = True
compat StackObject StackObject = True
compat Permanent Permanent = True
compat Permanent (OfType _) = True
compat Permanent Token = True
compat (OfType a) (OfType b) = sameType a b
compat Amount Amount = True
compat Pile Pile = True
compat _ _ = False

-- the WILDCARD anaphor ("it"/"they") reaches every object/player/pile noun,
-- but never a value antecedent (value anaphora is `ThatMany`).
public export
wildReaches : Sort -> Bool
wildReaches Amount = False
wildReaches _ = True

public export
reaches : Maybe Sort -> Sort -> Bool
reaches Nothing h = wildReaches h
reaches (Just w) h = compat w h

-- WHERE an antecedent came from — the survival rules key on it: `Delayed`
-- bodies drop `TargetSlot`s and keep `Product`s ([CR#603.7c]); event bodies
-- replace `EventRole`s ([CR#608.2k]); loop bodies clear `Allot` ([CR#601.2d]).
-- `Frame` is a `With`/pile CHOICE binder — the legacy deterministic scope:
-- anaphors inside it bind the frame directly, never R2-gated ([CR#608.2d]).
namespace Site
  public export
  data Site = TargetSlot | Product | EventRole | Loop | Allot | Chosen | Frame

public export
isTargetSlot : Site -> Bool
isTargetSlot TargetSlot = True
isTargetSlot _ = False

public export
isEventRole : Site -> Bool
isEventRole EventRole = True
isEventRole _ = False

public export
isAllotSite : Site -> Bool
isAllotSite Allot = True
isAllotSite _ = False

public export
isBinderSite : Site -> Bool    -- `It`'s deterministic scopes: loop elements + choice frames
isBinderSite Loop = True
isBinderSite Frame = True
isBinderSite _ = False

public export
isFrameSite : Site -> Bool     -- the sorted/plural anaphors' deterministic scope
isFrameSite Frame = True
isFrameSite _ = False

public export
sameCard : Cardinality -> Cardinality -> Bool
sameCard One One = True
sameCard Many Many = True
sameCard _ _ = False

-- ONE antecedent on the stack. `kind` rides along (an "any target" slot is
-- sort `Permanent` but kind `Anything`); `expectedZone` is the [CR#603.7c]
-- stamp a producing clause leaves (where the object went); `label` is
-- vestigial (see `labelFresh`) — no live constructor sets it since
-- positional `Target n` replaced the labeled-read mechanism ([CR#608.2d]).
public export
record Ant where
  constructor MkAnt
  sort : Sort
  kind : RefKind
  card : Cardinality
  site : Site
  expectedZone : Maybe Zone
  label : Maybe String

-- the RESULT of an anaphor resolution — the constructor names the failure, so
-- a failed auto-search error names it too: `Unbound` = "no compatible
-- antecedent in scope", `Ambiguous` = a second compatible antecedent in scope
-- (the R2 uniqueness gate).
namespace Bind
  public export
  data Bind = Bound RefKind | Unbound | Ambiguous

public export
isBound : Bind -> Bool
isBound (Bound _) = True
isBound _ = False

-- an antecedent is a CANDIDATE for an anaphor read when it is not the
-- `Allot` share (that is `Allotment`'s alone), its cardinality matches
-- (`It`/`That` = One, `They`/`Them` = Many), and its sort is reachable (R1).
public export
candidate : Maybe Sort -> Cardinality -> Ant -> Bool
candidate w cd a = not (isAllotSite a.site) && sameCard cd a.card && reaches w a.sort

public export
candidates : Maybe Sort -> Cardinality -> List Ant -> List Ant
candidates w cd [] = []
candidates w cd (a :: as) = case candidate w cd a of
  True => a :: candidates w cd as
  False => candidates w cd as

-- classify a candidate list: none = `Unbound`, a unique one binds, a
-- second candidate ANYWHERE is `Ambiguous` (the R2 uniqueness gate, STRICT
-- per the corpus dry-run freeze — an insertion flips "resolves" to a load
-- error, never to a silent re-point).
public export
classifyCandidates : List Ant -> Bind
classifyCandidates [] = Unbound
classifyCandidates [a] = Bound a.kind
classifyCandidates (_ :: _ :: _) = Ambiguous

-- R1 + R2 over the stack (nearest FIRST).
public export
resolveStack : Maybe Sort -> Cardinality -> List Ant -> Bind
resolveStack w cd s = classifyCandidates (candidates w cd s)

-- THE stack-weakening lemma ([type-theory#4] — one lemma over the one
-- resolve function, instead of per-rule ad-hoc weakening): pushing an
-- antecedent that is NOT a candidate for a read leaves that read's
-- resolution untouched. Sorted anaphors are deliberately NOT weakenable by
-- a COMPATIBLE push — R2 flips them to `Ambiguous` (the ambiguity gate is
-- the design); a positional `Target n` read is the escape hatch.
export
weakenResolve : {w : Maybe Sort} -> {cd : Cardinality} -> {s : List Ant}
             -> (a : Ant) -> (off : candidate w cd a = False)
             -> resolveStack w cd (a :: s) = resolveStack w cd s
weakenResolve a off = rewrite off in Refl

-- the innermost BINDER (a loop element or a `With`/pile choice frame) —
-- `It`'s deterministic antecedent: inside a binder, `It` is ALWAYS the
-- bound thing, never gated ([CR#608.2]; the pre-stack semantics, kept).
public export
innermostBinder : List Ant -> Maybe Ant
innermostBinder [] = Nothing
innermostBinder (a :: as) = if isBinderSite a.site then Just a else innermostBinder as

-- the innermost choice FRAME — the sorted/plural anaphors' deterministic
-- scope ([CR#608.2d]; never R2-gated).
public export
innermostFrame : List Ant -> Maybe Ant
innermostFrame [] = Nothing
innermostFrame (a :: as) = if isFrameSite a.site then Just a else innermostFrame as

-- `It` — the wildcard singular anaphor: the innermost binder when one
-- encloses, else R1+R2 over the stack.
public export
resolveIt : List Ant -> Bind
resolveIt s = case innermostBinder s of
  Just a => Bound a.kind
  Nothing => resolveStack Nothing One s

-- `That (w)` — the sorted singular anaphor: the innermost choice frame when
-- one encloses (it must be a single object of a reachable sort), else R1+R2.
public export
resolveThat : Sort -> List Ant -> Bind
resolveThat w s = case innermostFrame s of
  Just a => if sameCard One a.card && compat w a.sort then Bound a.kind else Unbound
  Nothing => resolveStack (Just w) One s

-- `They` / `Them (w)` — the plural anaphors (`Nothing` = the wildcard).
public export
resolveThey : Maybe Sort -> List Ant -> Bind
resolveThey w s = case innermostFrame s of
  Just a => if sameCard Many a.card && reaches w a.sort then Bound a.kind else Unbound
  Nothing => resolveStack w Many s

-- `Target n` / `Targets n` — the POSITIONAL read of the announce list
-- ([CR#115.3,601.2c]). Not an anaphor resolution: there is no R1 nearest-match
-- and no R2 uniqueness gate to run, because an index names exactly one slot.
-- It can still fail two ways, and both must be type errors rather than silent
-- degrades: the index can be out of range (notably in a `Delayed` body, whose
-- announce list is empty [CR#603.7c] — the staleness the old model caught only
-- because a target used to sit on the anaphor stack), and the slot's
-- cardinality can disagree with the read (a Many slot read singularly would
-- silently take one of several; a One slot read as a group would splay it).
public export
resolveTarget : Cardinality -> Nat -> List Ant -> Bind
resolveTarget cd _ [] = Unbound
resolveTarget cd Z (a :: _) = if sameCard cd a.card then Bound a.kind else Unbound
resolveTarget cd (S n) (_ :: as) = resolveTarget cd n as

-- `ThatMany`/`ThatMuch` — the value anaphor over `Amount` antecedents
-- ([CR#608.2i]).
public export
resolveAmount : List Ant -> Bind
resolveAmount = resolveStack (Just Amount) One

public export
sameLabel : Maybe String -> Maybe String -> Bool
sameLabel (Just a) (Just b) = a == b
sameLabel _ _ = False

public export
hasAllot : List Ant -> Bool
hasAllot [] = False
hasAllot (a :: as) = isAllotSite a.site || hasAllot as

-- the chosen-pile FRAME a `DivideAndChoose` body binds ([CR#700.3,608.2d]).
public export
PileFrame : Ant
PileFrame = MkAnt Pile AnObject Many Frame Nothing Nothing

-- the value antecedent an amount-bearing clause pushes ("that much",
-- [CR#608.2i]).
public export
amountAnte : Ant
amountAnte = MkAnt Amount AnObject One Product Nothing Nothing

-- the paid-for SPELL a mana rider binds as its loop element ([CR#106.6a]).
public export
PaidSpellAnte : Ant
PaidSpellAnte = MkAnt Spell AnObject One Loop Nothing Nothing

-- the noun an object answers to once it sits in a zone (`Spec.tZoneSorts`
-- pins this function's full valuation) ([CR#110.1,112.1,108.2]). This
-- is why "exile target creature … return that card" resolves: the exile
-- clause's product answers to `Card`.
public export
zoneSort : Zone -> Sort
zoneSort Battlefield = Permanent
zoneSort Stack = Spell
zoneSort Command = Card
zoneSort Exile = Card
zoneSort Graveyard = Card
zoneSort Hand = Card
zoneSort Library = Card
zoneSort Sideboard = Card

-- the fallback noun of a bare reference of known kind (a `TheRef` binder).
public export
kindSort : RefKind -> Sort
kindSort APlayer = Player
kindSort _ = Permanent

-- conjunctive PINS read off a filter (the Rust walker's `filter_pins`):
-- an object-kind pin, a zone pin, a card-type pin. `sortFromPins` turns
-- them into the filter's noun — battlefield objects refine to `Token`/
-- `OfType t` when pinned ([CR#110.1]).
public export
record Pins where
  constructor MkPins
  pKind : Maybe ObjectKind
  pZone : Maybe Zone
  pType : Maybe Type_

public export
noPins : Pins
noPins = MkPins Nothing Nothing Nothing

public export
orElse : Maybe a -> Maybe a -> Maybe a
orElse (Just x) _ = Just x
orElse Nothing y = y

public export
mergePins : Pins -> Pins -> Pins
mergePins (MkPins k1 z1 t1) (MkPins k2 z2 t2) =
  MkPins (orElse k1 k2) (orElse z1 z2) (orElse t1 t2)

public export
sortFromPins : Pins -> Sort
sortFromPins (MkPins (Just Spell) _ _) = Spell
sortFromPins (MkPins (Just Ability) _ _) = StackObject
sortFromPins (MkPins k z t) =
  case zoneSort (fromMaybe Battlefield z) of
    Permanent => case k of
      Just Token => Token
      _ => case t of
        Just ty => OfType ty
        Nothing => Permanent
    s => s

-- the object-role NOUN an event kind supplies its body's "that …" anaphor
-- ([CR#400.7e] — a moved object's noun follows the zone it moved to; a
-- sacrificed/destroyed/discarded object went to a graveyard, an exiled one
-- to exile — the entailment rows). The `object_sort` column this once drove
-- was deleted from EmitTables; kept as model documentation of each row's noun.
public export
eventKindObjectSort : EventKind -> Sort
eventKindObjectSort Sacrifice = Card                       -- [CR#701.21a] battlefield -> graveyard
eventKindObjectSort Draw = Permanent                       -- (no object cap; unused)
eventKindObjectSort Discard = Card                         -- [CR#701.9a] hand -> graveyard
eventKindObjectSort (DealDamage _) = Permanent             -- the damage SOURCE
eventKindObjectSort CreateToken = Token                    -- [CR#701.7]
eventKindObjectSort PutCounters = Permanent                -- the "on" carrier
eventKindObjectSort RemoveCounters = Permanent
eventKindObjectSort Destroy = Card                         -- [CR#701.8a] battlefield -> graveyard
eventKindObjectSort Mill = Card                            -- [CR#701.17a] library -> graveyard (no object cap; unused)
eventKindObjectSort Scry = Card                            -- [CR#701.22a] library cards, reordered (no object cap; unused)
eventKindObjectSort Surveil = Card                         -- [CR#701.25a] library -> graveyard/top (no object cap; unused)
eventKindObjectSort Fateseal = Card                        -- [CR#701.29a] an opponent's library cards (no object cap; unused)
eventKindObjectSort Fight = Permanent                      -- [CR#701.14a] the fighter creature (a battlefield permanent)
eventKindObjectSort (ZoneChanged _ (Just z)) = zoneSort z  -- the NEW zone's noun
eventKindObjectSort (ZoneChanged _ Nothing) = Card         -- left; destination unfixed
eventKindObjectSort (BeginStep _) = Permanent              -- (no object cap; unused)
eventKindObjectSort (Becomes _) = Permanent                -- the "of" permanent
eventKindObjectSort (Begins Cast) = Spell                  -- [CR#601.2i]
eventKindObjectSort (Begins Activate) = StackObject        -- [CR#602.2a]
eventKindObjectSort (Begins _) = Permanent                 -- attacker/blocker/attachment/…
eventKindObjectSort GainLife = Permanent                   -- (no object cap; unused)
eventKindObjectSort LoseLife = Permanent
eventKindObjectSort GainControl = Permanent                -- the "of" permanent
eventKindObjectSort TapForMana = Permanent                 -- the tapped land
eventKindObjectSort RollDice = Permanent                   -- (no object cap; unused)
eventKindObjectSort (FlipCoin _) = Permanent               -- (no object cap; unused)
eventKindObjectSort (RollPlanarDie _) = Permanent          -- (no object cap; unused)

-- one antecedent per CAPS guarantee ([CR#608.2k]): the roles an
-- event/payment body pushes for its anaphora — object, patient (sort fixed
-- by its kind), actor, amount, defending player. Push order mirrors the
-- Rust walker (later pushes land nearer).
public export
roleAntes : EventCaps -> (objectSort : Sort) -> List Ant
roleAntes (MkEventCaps o a m p d _) objSort =
  (if o then [MkAnt objSort AnObject One EventRole Nothing Nothing] else [])
  ++ (case p of
        Just k => [MkAnt (kindSort k) k One EventRole Nothing Nothing]
        Nothing => [])
  ++ (if a then [MkAnt Player APlayer One EventRole Nothing Nothing] else [])
  ++ (if m then [MkAnt Amount Anything One EventRole Nothing Nothing] else [])
  ++ (if d then [MkAnt Player APlayer One EventRole Nothing Nothing] else [])

-- a COST payment's roles ([CR#601.2f]): payment objects leave for another
-- zone, so the object noun is `Card`.
public export
costRoles : EventCaps -> List Ant
costRoles caps = roleAntes caps Card

-- The binding CONTEXT: the ordered antecedent stack (nearest FIRST) plus
-- the value channels the stack doesn't carry — the surrounding event's caps
-- (gating the EXPLICIT role reads `EventObject`/`EventActor`/…) and the
-- card-scoped as-enters choices ([CR#614.12]).
public export
record Ctx where
  constructor MkCtx
  stack : List Ant
  -- the ANNOUNCE LIST in scope ([CR#115.3,601.2c]) — its OWN channel, not the
  -- antecedent stack. A target is not something a clause produced and referred
  -- back to; it is announced at index n and read at index n (`Target n` /
  -- `Targets n`), so it never competes with an anaphor and two same-sort slots
  -- need no labels. `Targeted` binds it, `Delayed` clears it ([CR#603.7c]).
  targets : List Ant
  eventCaps : EventCaps          -- `NoCaps` outside an event body
  chosenKind : Maybe ChooseDomain    -- an as-enters VALUE choice in scope (`OfChosen`/`ChosenIs`/`ChosenNumber`)
  chosenRefKind : Maybe RefKind      -- an as-enters ENTITY choice in scope (`ChosenObject`/`ChosenPlayer`)

-- The context a resolving spell starts in: nothing bound yet.
public export
Base : Ctx
Base = MkCtx [] [] NoCaps Nothing Nothing

-- Each transition reconstructs `MkCtx` explicitly so a projection of a bind
-- result reduces definitionally even for abstract `b` (record-update sugar
-- has no get-after-set law for an abstract record).

-- push antecedents, listed in PUSH ORDER (the last pushed lands nearest —
-- the stack is nearest-first, so the list is reversed on).
public export
pushAntes : List Ant -> Ctx -> Ctx
pushAntes as b = MkCtx (reverse as ++ stack b) (targets b) (eventCaps b) (chosenKind b) (chosenRefKind b)

public export
notTargetSlotA : Ant -> Bool
notTargetSlotA a = not (isTargetSlot a.site)

public export
notEventRoleA : Ant -> Bool
notEventRoleA a = not (isEventRole a.site)

public export
notAllotA : Ant -> Bool
notAllotA a = not (isAllotSite a.site)

-- announce the target slots ([CR#115.3,601.2c]): they REPLACE any outer
-- announce list (a nested `Targeted` is its own, and its slot 0 is its own).
-- The antecedent stack is untouched — a slot pushes NO antecedent, so `It` /
-- `That` / `They` can never reach a target and no announce can make an
-- anaphor ambiguous. The list is read positionally, by `Target n` / `Targets n`.
public export
bindTargets : List Ant -> Ctx -> Ctx
bindTargets slots b = MkCtx (stack b) slots (eventCaps b) (chosenKind b) (chosenRefKind b)

-- a `Delayed` body drops the announced targets ([CR#603.7c]) — a later
-- resolution has no announce list of its own, so every positional read goes
-- out of range and no `Target n` typechecks there. `Product` antecedents (with
-- their expected zones) live on the stack and survive.
public export
unbindTargets : Ctx -> Ctx
unbindTargets b = MkCtx (stack b) [] (eventCaps b) (chosenKind b) (chosenRefKind b)

-- a `With` binder's antecedent ([CR#608.2d]): a choice binds a `Frame`, a
-- search/produce binder a whiffable `Product` — the ante's own site stands.
public export
bindThat : Ant -> Ctx -> Ctx
bindThat a = pushAntes [a]

-- a loop element ([CR#608.2]): re-sited at `Loop`, one element at a time;
-- clears any `Allot` share (it was indexed to a DIFFERENT element, so it
-- must not leak into a nested loop).
public export
bindIt : Ant -> Ctx -> Ctx
bindIt a b = MkCtx (MkAnt a.sort a.kind One Loop a.expectedZone a.label :: filter notAllotA (stack b)) (targets b) (eventCaps b) (chosenKind b) (chosenRefKind b)

-- a `Distribute` body ([CR#601.2d]): the loop element plus its `Allot`
-- share (read back only by `Allotment`).
public export
bindAllot : Ant -> Ctx -> Ctx
bindAllot a b = MkCtx (MkAnt Amount Anything One Allot Nothing Nothing :: MkAnt a.sort a.kind One Loop a.expectedZone a.label :: stack b) (targets b) (eventCaps b) (chosenKind b) (chosenRefKind b)

-- entering a trigger/replacement/delayed/payment body: carry the event's
-- CAPS and push one role antecedent per guarantee; an inner event's roles
-- SHADOW an outer's ([CR#608.2k] — one antecedent set per body).
public export
bindEvent : EventCaps -> List Ant -> Ctx -> Ctx
bindEvent caps roles b = MkCtx (reverse roles ++ filter notEventRoleA (stack b)) (targets b) caps (chosenKind b) (chosenRefKind b)

-- the as-enters VALUE choice ([CR#614.12]): binds `chosenKind` (a color/
-- type/name/number/mode) for the abilities that read it.
public export
bindChosen : ChooseDomain -> Ctx -> Ctx
bindChosen d b = MkCtx (stack b) (targets b) (eventCaps b) (Just d) (chosenRefKind b)

-- the as-enters GAME-ENTITY choice ([CR#614.12]): binds `chosenRefKind` (a
-- chosen object/player) — the identity twin of `bindChosen`.
public export
bindChosenRef : RefKind -> Ctx -> Ctx
bindChosenRef k b = MkCtx (stack b) (targets b) (eventCaps b) (chosenKind b) (Just k)


-- KeywordSpec / Reference / Count / Predicate / Condition / EventQuery are one mutually
-- recursive language. A PREDICATE is an object test — its candidate is IMPLICIT. A `Condition`
-- is a closed/game-state test reaching objects via `Matches`/`exists`/`unique`.
mutual
  -- `ElemOf`/`CharValue` — the element type and the settable value-type of each `Characteristic` axis.
  -- They must be DEFINED EARLY in this block (before `pins`/`Predicate` reduce `ElemOf`), yet they can
  -- only live in this block at all because their `Subtypes` clause mentions `Subtype`, whose conferral
  -- field references `Ability` (declared far below in this same mutual block).
  public export
  ElemOf : Characteristic -> Type         -- the element `Add`/`Remove` takes (only consulted under `Collection`)
  ElemOf Colors     = Color
  ElemOf Types  = Type_
  ElemOf Subtypes   = Subtype
  ElemOf Supertypes = Supertype
  ElemOf _          = Unit

  public export
  CharValue : Ctx -> Characteristic -> Type
  CharValue _ Colors     = List Color
  CharValue _ Types  = List Type_
  CharValue _ Subtypes   = List Subtype
  CharValue _ Supertypes = List Supertype
  CharValue b Power      = Count b        -- specify a (possibly dynamic, CDA "*/*") value in the amount language
  CharValue b Toughness  = Count b
  CharValue b Defense    = Count b
  CharValue _ ManaCost   = ManaCost       -- the symbol list ("no mana cost" = `Set ManaCost []`, eternalize)
  CharValue _ Name       = Maybe String   -- `Nothing` = "has no name"
  CharValue _ BasicLandTypes = Void       -- READ-ONLY axis (Domain): unsettable, so `Set`'s argument is uninhabited

  -- An OPEN counter kind ([CR#122]): its `Scope` (object/player carrier), its open registry name, and the
  -- abilities it CONFERS on its bearer (+1/+1 & −1/−1's P/T pump). Open + name-keyed like `Subtype`, and in
  -- this mutual block for the same reason (its conferral field references `Ability`, declared below). Defined
  -- EARLY in the block so `counterKindScope` reduces at the counter-op type-index sites below.
  public export
  data CounterKind : Type where
    MkCounterKind : Scope -> String -> List (Ability Base) -> CounterKind

  -- An OPEN designation (monarch, monstrous, …): its `Scope`, its open name, and its conferrals (always []
  -- for now — query-only; the real payload lives in Rust). The designation analogue of `CounterKind`.
  public export
  data Designation : Type where
    MkDesignation : Scope -> String -> List (Ability Base) -> Designation

  -- the carrier of a counter kind / designation as a `RefKind` — the total projection that indexes the
  -- counter/designation ops dependently (replaces the old closed `counterScope`/`designationScope`).
  public export
  counterKindScope : CounterKind -> RefKind
  counterKindScope (MkCounterKind s _ _) = scopeRef s

  public export
  designationKindScope : Designation -> RefKind
  designationKindScope (MkDesignation s _ _) = scopeRef s

  -- A KEYWORD's tag + params ([CR#702]) — the "name" side of a keyword. In this block so
  -- `HasKeyword` can read it and `Hexproof`'s "from" filter can be a `Predicate` (which may name
  -- an anaphor — "from the CHOSEN color"). `keyword` (Macros) desugars a spec into its full `Ability`
  -- (a `Composite`): the deontic ones (Flying/Defender/Shroud/Hexproof/Menace) get a `cant` (Menace's
  -- is the SET-level `BlockedBy`); the rest (FirstStrike/Deathtouch/Trample = damage; Vigilance =
  -- event-edit; Reach/Flash = flag/window) carry no clause.
  namespace KeywordSpec
    public export
    data KeywordSpec : Ctx -> Type where
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
      Flashback : KeywordSpec b -- the tag for the `flashback` macro ([CR#702.34]); the cost rides its desugared `MayCastFor` (bare — `KeywordSpec` precedes `Cost`). Engine applies the [CR#702.34a] exile-instead-of-graveyard off this tag.
      -- ALTERNATIVE-COST tags: each is `MayCastFor [altCost] {tag = Just X}` + a rider keyed on `WasCastWith X`
      -- ("if its X cost was paid"). Dash ([CR#702.109]): haste + return to hand at end of turn. Evoke
      -- ([CR#702.74]): sacrifice on ETB. Blitz ([CR#702.152]): haste + sac EOT + draw-on-death. Prowl
      -- ([CR#702.76]) and Spectacle ([CR#702.137]) ride a `{when}` availability guard instead of an ETB rider.
      Dash : KeywordSpec b
      Evoke : KeywordSpec b
      Blitz : KeywordSpec b
      Prowl : KeywordSpec b
      Spectacle : KeywordSpec b
      Devoid : KeywordSpec b  -- "this object is colorless" ([CR#702.114]) — a CDA; desugars to `Set Colors []` on This
      Protection : Predicate b AnObject -> KeywordSpec b   -- "protection from [quality]" ([CR#702.16]) — the tag carries q; desugars to the DEBT bundle (`protection` macro)
      -- INTRINSIC combat keyword ([CR#702.22]): the band-attacking + damage-assignment-delegation rules are
      -- engine-baked (not composable data, like Deathtouch). `Nothing` = plain banding; `Just q` = "bands with
      -- other [q]" ([CR#702.22b], the quality-restricted band). Bare — `keyword (Banding mq) = Bare (Banding mq)`.
      Banding : Maybe (Predicate b AnObject) -> KeywordSpec b
      -- the tag for the `mutate <cost>` macro ([CR#702.140]): an ALTERNATIVE cost (rides `MayCastFor {tag = Just
      -- Mutate}`, like Dash) whose intrinsic MERGE ([CR#730]) the engine bakes off this tag. Bare here (the cost
      -- rides the macro); degenerate in `keyword`, like Flashback/Morph.
      Mutate : KeywordSpec b
  -- A REFERENCE to a game object/player ([CR#108,109.1]) — a text-internal anaphor.
  namespace Reference
    public export
    data Reference : Ctx -> RefKind -> Type where
      -- the source object; always available [CR#113.7].
      This : Reference b AnObject
      -- demote a `Selection` to its SOLE element. Partial — the author asserts singularity, exactly
      -- like `Only` (undefined on a 0- or 2+-element set). `GetTarget`/`Only` are sugar over it.
      Single : Selection b k -> Reference b k
      -- the host this is attached to ("enchanted creature").
      AttachHostOf : Reference b AnObject -> Reference b AnObject
      -- the WILDCARD singular anaphor ("it"): inside a binder it is ALWAYS the
      -- innermost bound thing — a loop element or a `With`/pile choice frame —
      -- deterministic, never gated ([CR#608.2]); outside every binder it
      -- resolves over the antecedent stack (R1 nearest + the R2 uniqueness
      -- gate). Also the per-subject candidate an anthem's mods read.
      It : {auto 0 prf : resolveIt (stack b) = Bound k} -> Reference b k
      -- the nth ANNOUNCED target ([CR#115.3,601.2c]) — a positional read of
      -- the `Targeted` slot list, and with `Targets` the ONLY way to name a
      -- target. Not an anaphor: it reads the announce-list channel
      -- (`targets b`), never the stack, so it is immune to R1/R2 and two
      -- same-sort slots (the fight family) are simply 0 and 1.
      -- The obligation is the slot's own: index in range, cardinality One, and
      -- `k` comes from the SLOT (an "any target" slot is `Anything`, a player
      -- slot `APlayer`) rather than being free — so a player slot can't be read
      -- where an object is wanted, and a `Delayed` body (empty announce list,
      -- [CR#603.7c]) admits no `Target n` at all.
      -- Rust: Reference::Target(n).
      Target : (n : Nat) -> {auto 0 prf : resolveTarget One n (targets b) = Bound k} -> Reference b k
      -- the SORTED singular anaphor — "that card" / "that creature" / "that
      -- player" ([CR#608.2d]): the antecedent answering to the noun `w` (R1
      -- per `compat`, R2 strict — a second compatible antecedent is a type
      -- error, not a guess). Inside a `With`/pile choice frame it binds the
      -- frame deterministically. Antecedents are pushed by producing clauses
      -- ([CR#400.7]), event bodies ([CR#608.2k]), and binders ([CR#608.2d]) —
      -- NOT by announced target slots, which are read positionally as
      -- `Target n`. This is the read for a move's PRODUCT ("exile target
      -- creature … return that card": the card is a new object, so `Target 0`
      -- cannot name it and `That Card` does).
      That : (w : Sort) -> {auto 0 prf : resolveThat w (stack b) = Bound k} -> Reference b k
      -- the triggering event's object ("that card") — valid only if the event SUPPLIES one ([CR#608.2k]).
      EventObject : {auto 0 prf : hasObject (eventCaps b) = True} -> Reference b AnObject
      -- the triggering event's PATIENT — the acted-upon thing (damage recipient, …), KIND-POLY but fixed by
      -- the event's `patientKind` cap so the binder stays sound ([CR#120.3]); distinct from the Agent
      -- (`EventObject`). For the combat DEFENDER specifically, prefer `DefendingPlayer` (always a player).
      EventPatient : {auto 0 prf : patientKind (eventCaps b) = Just k} -> Reference b k
      -- PLAYERS (the old `PlayerRef`, folded in here):
      You : Reference b APlayer                            -- controller of this ability [CR#109.5]
      ControllerOf : Reference b AnObject -> Reference b APlayer   -- the controller of an object
      Coalesce : List (Reference b k) -> Reference b k
      OwnerOf : Reference b AnObject -> Reference b APlayer        -- the owner of an object [CR#108.3]
      EventActor : {auto 0 prf : hasActor (eventCaps b) = True} -> Reference b APlayer  -- the event's player ("that player") — only if supplied
      DefendingPlayer : {auto 0 prf : hasDefender (eventCaps b) = True} -> Reference b APlayer  -- the defending player of an attack/combat — ALWAYS a player, even vs a planeswalker/battle ([CR#506.2,508.5]); landwalk/Annihilator/Afflict
      ChosenPlayer : {auto 0 prf : chosenRefKind b = Just APlayer} -> Reference b APlayer  -- the as-enters chosen PLAYER (the identity-reference twin of OfChosen/ChosenNumber); opened by `AsEntersChoosing APlayer …`
      ChosenObject : {auto 0 prf : chosenRefKind b = Just AnObject} -> Reference b AnObject  -- the as-enters chosen OBJECT (the object-twin of `ChosenPlayer`; Clone copies it via `BecomeCopyOf ChosenObject`); opened by `AsEntersChoosing AnObject …`

  -- A query selecting events ([CR#700]): kinds and filter filters (e.g. "Whenever a creature
  -- dies" = actor creature). Event predicates (`EventObject`/`EventActor`/`EventPatient`/
  -- `DefendingPlayer`) can ONLY be read if the event query's kinds have those caps.
  public export
  record EventQuery (b : Ctx) where
    constructor MkEventQuery
    kinds  : List EventKind
    facets : List (Facet b)

  -- whether an event-QUERY guarantees an amount to sum/replace: lifts `kindsHaveAmount` onto the bundled
  -- query (every listed kind must carry one; empty `kinds` = any kind ⇒ False). Phrased as a DECLARED
  -- function over the record (defined just above), so `EventSum`/`ReplaceAmount` — whose constructor types
  -- sit just below — can name it (a record `.kinds` projection would not yet be in scope there). Refines the
  -- plain `EventQuery` that `EventCount`/`Triggered`/`Replaces` accept, rejecting amountless kinds
  -- (`MkEventQuery [Begins Cast] []`) at the type level.
  public export
  eventQueryHasAmount : EventQuery b -> Bool
  eventQueryHasAmount (MkEventQuery ks _) = kindsHaveAmount ks

  -- whether an event-query is entirely roll-more-eligible randomness ([CR#614.3]): lifts
  -- `kindsAreRandomness` onto the bundled query, same split as `eventQueryHasAmount`/`kindsHaveAmount`.
  -- Gates `ReplaceRoll` — restricts it to `RollDice`/`FlipCoin`; the planar die has no roll-more
  -- interaction ([CR#901.9d]).
  public export
  isRandomnessQuery : EventQuery b -> Bool
  isRandomnessQuery (MkEventQuery ks _) = kindsAreRandomness ks

  -- Something countable ([CR#107]): a list of objects (existing matching ones, or top N of library),
  -- or a count of matching ones (for affinity/devotion), or a list of events.
  namespace Countable
    public export
    data Countable : Ctx -> Type where
      Objects     : Predicate b AnObject -> Countable b
      Players     : Predicate b APlayer -> Countable b
      Events      : EventQuery b -> Countable b
      ManaSymbols : Reference b AnObject -> SymbolPred -> Countable b
      ManaSpent   : {default This forObj : Reference b AnObject} -> Countable b   -- mana SPENT to cast/activate `forObj` (default `This`)
      -- ONE object treated as a singleton set — the per-object twin of `Objects`, so `CountDistinct`
      -- can read a characteristic off a SINGLE object rather than a filtered many (Embiggen's
      -- "number of card types [this creature] has" = `CountDistinct Types (Singleton This)`;
      -- likewise a color-count). [CR#105.2] (an object's color/colors).
      Singleton : (ref : Reference b AnObject) -> Countable b
      -- mana SPENT to cast/activate `forObj`, FILTERED by a `SymbolPred` — Adamant's "if at least
      -- three white [mana symbols were spent]" ([CR#107.4]: what a mana symbol counts as, e.g. a
      -- hybrid/Phyrexian symbol counting as its component color(s)). Used via `CountOf` (cardinality
      -- of matching spent symbols), unlike plain `ManaSpent` which is read via `CountDistinct Colors`.
      ManaSpentMatching : (forObj : Reference b AnObject) -> (filter : SymbolPred) -> Countable b

  -- which `Countable`s can be PROJECTED per-element (bind `It` to each element and read a value): only
  -- objects — an atomic mana symbol, player count, or event has no element to bind. A PROOF on the `Countable`,
  -- not a second type: `Project` demands it, so `Project (Events …) …` is ill-typed (no `Projectable (Events
  -- …)`). Same auto-implicit idiom as `eventQueryHasAmount`/`NonEmpty`.
  namespace Projectable
    public export
    data Projectable : Countable b -> Type where
      ObjectsAreProjectable : Projectable (Objects p)
      PlayersAreProjectable : Projectable (Players p)   -- cross-player fold (`It` : APlayer) — Balance/Arbiter of Knollridge [CR#119.1]

  -- which CHARACTERISTIC can be read off each element of a `Countable` — the gate on `CountDistinct`, mirroring
  -- `Projectable`/`eventQueryHasAmount`. `Colors` is readable off objects AND mana (Sunburst counts colours of
  -- mana spent); the other axes (name / subtypes / card-types / a numeric stat) are object-only; players and
  -- events carry none. So `CountDistinct Power (Events …)` and `CountDistinct Name ManaSpent` are ill-typed. A
  -- Type-returning subset fn (the file's `IsCharDomain` idiom — `()`/`Void`), named so the `Count` gate can reach it.
  public export
  readableOn : Characteristic -> Countable b -> Type
  readableOn Colors (Objects _)       = ()
  readableOn Colors (ManaSymbols _ _) = ()
  readableOn Colors ManaSpent         = ()
  readableOn _      (Objects _)       = ()
  readableOn _      (Singleton _)     = ()
  readableOn _      _                 = Void

  -- A PROJECTION of a game property/state to a value — indexed by its element `RefKind` (`AnObject` for the
  -- usual per-permanent fold, `APlayer` for a cross-player fold, §8) so `Pick` can pin to objects while
  -- `Aggregate` stays poly over both.
  namespace Projection
    public export
    data Projection : Ctx -> RefKind -> Type where
      Project : (src : Countable b) -> {auto 0 prj : Projectable src} -> Count (bindIt (projElemAnte src) b) -> Projection b ((projElemAnte src).kind)

  -- A COUNT / mathematical expression producing a number.
  namespace Count
    public export
    data Count : Ctx -> Type where
      Literal : Nat -> Count b                  -- a bare number
      X : Count b                               -- the chosen {X} value ([CR#107.3])
      CountOf : Countable b -> Count b          -- the CARDINALITY of a countable (|set|); `CountMatching`/`CountEvents` are sugar
      -- size of the DISTINCT union of a characteristic across a set (Domain = `CountDistinct Subtypes (Objects
      -- yourLands)` — granularity from the source filter; Collector's Cage = `CountDistinct Power (Objects …)`).
      -- Gated by `readableOn`, so the axis must be readable off the source's elements (`CountDistinct Power
      -- (Events …)` and `CountDistinct Name ManaSpent` are rejected).
      CountDistinct : (c : Characteristic) -> (src : Countable b) -> {auto 0 ok : readableOn c src} -> Count b
      -- read an object's CURRENT numeric characteristic ([CR#613]) — power/toughness/defense (`Numeric`-gated).
      StatOf : Reference b AnObject -> (c : Characteristic) -> {auto 0 _ : Numeric c} -> Count b
      ManaValueOf : Reference b AnObject -> Count b   -- an object's DERIVED mana value ([CR#202.3]) — a number, not a characteristic axis
      -- FOLD a `Projection` to one value, per `AggregateOp` ("greatest power among creatures you control" =
      -- `Aggregate MaxOf (eachOf yourCreatures (StatOf It Power))`; devotion sums a per-permanent pip-count).
      -- The value-twin of `Pick` (which takes the extremal element from the same `Projection`). Poly over the
      -- element `RefKind` (free implicit `k`) — a cross-player fold works too ([CR#119.1] Arbiter of
      -- Knollridge/Balance, §8), unlike `Pick` which stays pinned to objects.
      Aggregate : AggregateOp -> Projection b k -> Count b
      -- fold the matching events' AMOUNTS, per `AggregateOp` (`EventAgg SumOf q` is the old `EventSum`; events
      -- have no `It` to bind, so they fold their single amount rather than a projection). Gated by
      -- `eventQueryHasAmount`, so every queried kind must carry one (`EventAgg SumOf (MkEventQuery [Begins Cast] [])`
      -- is rejected — a cast has no amount). Cardinality of events is `CountEvents q` = `CountOf (Events q)`.
      EventAgg : AggregateOp -> (q : EventQuery b) -> {auto 0 amt : eventQueryHasAmount q = True} -> Count b
      Damage : Reference b AnObject -> Count b  -- marked damage on r ([CR#120.3]); the lethal-damage SBA reads `Compare (Damage This) GreaterEq (StatOf This Toughness)`
      CountersOn : (c : CounterKind) -> Reference b (counterKindScope c) -> Count b   -- number of [kind] counters on r (object or player, per `counterKindScope`)
      TimesPaid : (tag : String) -> Count b     -- how many times the tagged optional cost (`CostOption`) was paid (multikicker, [CR#702.33c..702.33d]). Rust: Count::TimesPaid(CostTag)
      PlayerStatOf : Reference b APlayer -> PlayerAttr -> Count b   -- a player's numeric attribute (`Life`/`HandSize`) — the player-side twin of `StatOf`. Sugar: `lifeTotal`/`handSize`.
      Plus  : Count b -> Count b -> Count b                -- arithmetic on values
      Minus : Count b -> Count b -> Count b
      Times : Count b -> Count b -> Count b
      Half : RoundMode -> Count b -> Count b               -- half, rounded per `RoundMode` ([CR#107.1a]); one rounding vocabulary, shared with `AverageOf`
      Min : Count b -> Count b -> Count b                  -- the lesser ([CR#704.5q] +1/+1 vs −1/−1 annihilation; "the lesser of X and Y")
      Max : Count b -> Count b -> Count b                  -- the greater
      Divide : RoundMode -> Count b -> Count b -> Count b  -- divide the first by the second, rounded per `RoundMode` ([CR#107.1a]); `Half` stays the dedicated /2 constructor
      Mod : Count b -> Count b -> Count b                  -- remainder of the first divided by the second (parity: `Compare (Mod x (^2)) Eq (^0)`)
      Pow : Count b -> Count b -> Count b                  -- base raised to exponent (exponential growth effects; Mathemagics)
      TargetsOf : Reference b AnObject -> Count b          -- the number of times r was chosen as a target when put on the stack ([CR#115.9a]); Strive: `Minus (TargetsOf This) (^1)`
      EventAmount : {auto 0 prf : hasAmount (eventCaps b) = True} -> Count b   -- the event's amount — valid only where the event SUPPLIES one
      -- the VALUE anaphor — "that many"/"that much" ([CR#608.2i]): the unique
      -- `Amount` antecedent a producing clause pushed (a draw's card count,
      -- a damage clause's amount); R2 gates a second one like any anaphor.
      ThatMany : {auto 0 prf : isBound (resolveAmount (stack b)) = True} -> Count b
      Allotment : {auto 0 prf : hasAllot (stack b) = True} -> Count b   -- inside a `Distribute`: the share allotted to the current element ([CR#601.2d])
      ChosenNumber : {auto 0 prf : chosenKind b = Just ANumber} -> Count b   -- the as-enters chosen NUMBER (the value-anaphor twin of OfChosen/ChosenIs)

  -- A PREDICATE / filter atom testing a candidate. Gated by candidate kind (`RefKind`).
  namespace Predicate
    public export
    data Predicate : Ctx -> RefKind -> Type where
      -- the candidate's collection axis `c` CONTAINS this element — the read mirror of
      -- `Alter c (Add …)`. Gated by `Collection` (the 4 set-kinded axes); `ElemOf c` is the
      -- element type. Terse sugar: `hasType`/`hasColor`/`hasSubtype`/`hasSupertype`.
      HasChar : (c : Characteristic) -> {auto 0 _ : Collection c} -> ElemOf c -> Predicate b AnObject
      IsKind : ObjectKind -> Predicate b AnObject
      InZone : Zone -> Predicate b AnObject
      HasKeyword : KeywordSpec b -> Predicate b AnObject
      SameAs : Reference b k -> Predicate b k    -- the candidate IS r (same kind; "another" = Not (SameAs This))
      SameName : Reference b AnObject -> Predicate b AnObject   -- shares a name with r ("named [its own name]" = SameName This)
      SharesChar : (c : Characteristic) -> {auto 0 _ : Collection c} -> Reference b AnObject -> Predicate b AnObject   -- shares ≥1 element of collection axis c with r (Coat of Arms: `SharesChar Subtypes It`); `SharesColor`/`SharesType` are just `SharesChar Colors`/`SharesChar Types`. Sugar: `sharesSubtype`.
      WasCastFrom : Zone -> Predicate b AnObject -- the object was cast from this zone (cast provenance)
      WasCastWith : KeywordSpec b -> Predicate b AnObject  -- cast using the keyword's ALTERNATIVE cost ("if its dash cost was paid",
                                                 -- [CR#702.109a] etc.) — the alt-cost twin of `WasCastFrom`; engine records the `MayCastFor` tag used
      WasPutFrom : Zone -> Predicate b AnObject  -- the object was PUT into its current zone directly from this zone
                                                 -- (move provenance) — the move-provenance twin of `WasCastFrom`; engine-held,
                                                 -- turn-scoped (like `DamagedBy`). "milled" = And [InZone Graveyard,
                                                 -- WasPutFrom Library] ([CR#701.17a]); "discarded" = WasPutFrom Hand
                                                 -- ([CR#701.9a]); `WasMilled`/`WasDiscarded` decompose over this, not minted.
      ExiledBy : Reference b AnObject -> Predicate b AnObject   -- set aside by r's effect ("cards exiled by this" = ExiledBy
                                                 -- This); the engine holds the association ([CR#607] linked abilities)
      DamagedBy : Reference b AnObject -> Predicate b AnObject  -- was dealt damage by r THIS TURN ("a creature dealt damage
                                                 -- by ~ this turn" = And [creature, DamagedBy This]); engine-held, like ExiledBy. Turn-scoped reset is the engine's.
      Adjacent : Adjacency -> Reference b AnObject -> Predicate b AnObject   -- the candidate is directly Above/Below r in
                                                 -- an ORDERED zone, nothing between (Death Spark's "directly above a
                                                 -- Spirit card in your graveyard", [CR#404.2]); object-relative, unlike
                                                 -- the end-relative `Anchor` (FromTop/FromBottom), which can't express
                                                 -- this. Engine-held, like `DamagedBy`.
      HasName : String -> Predicate b AnObject   -- named a specific card (tutors / token names)
      HasCounter : (c : CounterKind) -> Predicate b (counterKindScope c)   -- has ≥1 of this counter; the candidate's kind follows the carrier ("ten poison" tests a player)
      HasState : ObjectState -> Predicate b AnObject      -- runtime state: "target ATTACKING / TAPPED creature"
      -- the DURATIVE aspect of the relation spine: "the candidate currently fills [role] of [r]" — object-only
      -- (only objects bear durative state; a player defender has none). `Holds Attack Agent` = an attacker,
      -- `Holds Block Patient` = a blocked creature. Unifies the legacy `Attacking`/`Blocking`/`Blocked` states.
      Holds : Relation -> Role -> Predicate b AnObject
      -- carries a DESIGNATION; the candidate's kind follows `designationKindScope` ("you're the monarch" =
      -- `HasDesignation monarch` is a player test, "while ~ is monstrous" an object test).
      HasDesignation : (d : Designation) -> Predicate b (designationKindScope d)
      -- a numeric-characteristic comparison on the candidate — "target creature with power ≤ 2" =
      -- `And [creature, StatCmp Power AtMost (^2)]`. (Closes the "no stat filter" hole — stat
      -- comparison was a `Condition` only; this lifts it into the `Predicate`/filter language.)
      StatCmp : (c : Characteristic) -> {auto 0 _ : Numeric c} -> Cmp -> Count b -> Predicate b AnObject
      -- the PLAYER-side stat filter — "an opponent has 10 or less life" = `And [opponent, PlayerStatCmp Life AtMost (^10)]`.
      -- The player twin of `StatCmp`; reads the SUBJECT player's `PlayerAttr`.
      PlayerStatCmp : PlayerAttr -> Cmp -> Count b -> Predicate b APlayer
      ControlledBy : Predicate b APlayer -> Predicate b AnObject   -- controller MATCHES a player-pred: "you control" = ControlledBy you, "an opponent controls" = ControlledBy opponent
      OwnedBy : Predicate b APlayer -> Predicate b AnObject
      Controls : Predicate b AnObject -> Predicate b APlayer   -- the INVERSE: a PLAYER who controls a [pred] ("each player who controls a creature")
      Multicolored : Predicate b AnObject   -- ≥2 colors ([CR#105.2b])
      IsColorless : Predicate b AnObject    -- 0 colors (named to avoid the `Colorless : Maybe Color` value)
      -- STACK-object filters: a spell/ability BY its targets ([CR#115]). "Spell that targets you" =
      -- `And [IsKind Spell, Targets (SameAs You)]`; "single-target spell" = `TargetCount Eq (^1)`.
      Targets : Predicate b k -> Predicate b AnObject
      TargetCount : Cmp -> Count b -> Predicate b AnObject
      -- the candidate's TAGGED optional cost (`CostOption`) was paid ("a kicked spell",
      -- [CR#702.33d..702.33e]) — the cost-mode read that retires the old `WasKicked` boolean flag.
      -- Rust: Predicate::WasPaidWith(CostTag).
      WasPaidWith : (tag : String) -> Predicate b AnObject
      -- ANAPHOR: "the candidate has the chosen characteristic" — the chosen color (Iona: "spells of the
      -- chosen color") or creature type (Cavern: "a creature spell of the chosen type"). Gated on an
      -- as-enters CHARACTERISTIC choice being in scope (`IsCharDomain (chosenKind b)`); the engine
      -- resolves which characteristic to test from the domain. No per-color/-type literal anaphor needed.
      OfChosen : {auto 0 prf : IsCharDomain (chosenKind b)} -> Predicate b AnObject
      -- `Anyone` is the player top-predicate ("any player" — a person, hence `APlayer`).
      Anyone : Predicate b APlayer
      -- team-relative player predicates ([CR#102.3]): `OpponentOf` = a player NOT on your team; `TeammateOf` = another
      -- player ON your team. PRIMITIVE — the engine resolves team membership; they are NOT `Not (SameAs You)`, which
      -- wrongly counts a teammate as an opponent in Two-Headed Giant ([CR#810]) and other team games. ("you" stays
      -- the derived `SameAs You`; "your team" [CR#102.4] = `Or [SameAs You, TeammateOf]`.)
      OpponentOf : Predicate b APlayer
      TeammateOf : Predicate b APlayer
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
    data Condition : Ctx -> Type where
      Matches : Reference b k -> Predicate b k -> Condition b   -- does r satisfy the (same-kind) predicate
      Compare : Count b -> Cmp -> Count b -> Condition b
      TurnOf : Predicate b APlayer -> Condition b   -- it's a (matching) player's turn (`yourTurn = TurnOf (SameAs You)`)
      During : PhaseStep -> Condition b
      -- "[r] is LEGALLY attached" ([CR#701.3b,303.4d]): has a host that passes the attach-legality
      -- predicate. The Aura graveyard SBA reads its negation (`Not (LegallyAttached This)`).
      LegallyAttached : Reference b AnObject -> Condition b
      -- this object's tagged optional cost (`CostOption`) was paid ("if it was kicked",
      -- [CR#702.33d]; buyback's "if the buyback cost was paid", [CR#702.27a]) — the [CR#607]
      -- linked read. Rust: Condition::PaidCost(CostTag).
      PaidCost : (tag : String) -> Condition b
      -- ANAPHOR (modal): "the chosen MODE is index i" — reads an as-enters `AMode` choice ([CR#614.12]).
      -- `i` is bounded by the choice's mode count `n` (recovered from `chosenKind b = Just (AMode n)`),
      -- so `ChosenIs 2` on a 2-mode card is rejected. Each siege ability gates on it: `If (ChosenIs k) …`.
      ChosenIs : (i : Nat) -> {auto 0 prf : chosenKind b = Just (AMode n)} -> {auto 0 inb : LT i n} -> Condition b
      And : List (Condition b) -> Condition b
      Or : List (Condition b) -> Condition b
      Not : Condition b -> Condition b

  -- The kind-free EVENT-FACET language: conditions refining WHICH event (never its kind, which lives in
  -- the `EventQuery` record's `kinds` slot). Facets conjoin via `And`; `Or` disjoins, `Not` negates (same
  -- combinator names as Predicate/Condition, in this namespace). The THEMATIC-ROLE facets embed the object/
  -- player language; `Within` and `Whenever` (over `During`/`TurnOf`) are the timing facets ("not during your turn" = `Not (Whenever (TurnOf You))`).
  namespace Facet
    public export
    data Facet : Ctx -> Type where
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
      -- BRIDGE: any game-state `Condition` (timing or otherwise) as an event facet — "the event matches when
      -- [cond] holds". Subsumes the former `DuringStep`/`DuringTurn` (now `Whenever (During …)` / `Whenever
      -- (TurnOf …)`), so timing atoms live once in `Condition` ([CR#603.2]). `Within` stays (no Condition twin).
      Whenever : Condition b -> Facet b
      -- "this is the Nth event (matching the surrounding facets) in the window" — an ORDINAL facet,
      -- engine-resolved like `EventCount`. `IsFirst` is the `n=1`
      -- sugar (below). Erayo "4th spell cast each turn" = `IsNth 4 ThisTurn`; "your second draw each turn"
      -- = `IsNth 2 ThisTurn`. `n=0` never matches (a harmless no-op). Notion Thief: "except the first draw
      -- each draw step" = `Not (And [Whenever (During drawStep), IsFirst ThisStep])`.
      IsNth : Nat -> Window -> Facet b
      And  : List (Facet b) -> Facet b   -- AND
      Or   : List (Facet b) -> Facet b   -- OR
      Not : Facet b -> Facet b          -- NOT

    -- "the FIRST matching event in the window" — sugar for the `n=1` ordinal.
    public export
    IsFirst : Window -> Facet b
    IsFirst = IsNth 1

  -- whether a literal `Range`'s bounds are ORDERED (lo ≤ hi). Only literal-vs-literal is checked — a
  -- dynamic bound (any `Count` expression) is lenient, exactly like `NonZeroQ`.
  public export
  OrderedRange : Maybe (Count b) -> Maybe (Count b) -> Type
  OrderedRange (Just (Literal lo)) (Just (Literal hi)) = LTE lo hi
  OrderedRange _ _ = ()

  -- A cardinality spec for a choice ([CR#107.3]). In the mutual block so `Selection` can use it.
  namespace Quantity
    public export
    data Quantity : Ctx -> Type where
      Range : Maybe (Count b) -> Maybe (Count b) -> Quantity b

  -- A resolution-time GROUP / choice. In the mutual block because `Single` (a `Reference`)
  -- demotes it. Plural anaphora (`They`/`Them`) lives here — the
  -- group-side twins of `It`/`That`.
  namespace Selection
    public export
    data Selection : Ctx -> RefKind -> Type where
      SelectAll : Predicate b k -> Selection b k                  -- every match (a group)
      Union : List (Selection b k) -> Selection b k              -- groups combined ("each X and each Y"); a fixed set = `Union` of `SameAs` singletons
      -- the nth announced slot read as its whole GROUP ([CR#115.3,601.2c]) —
      -- the plural twin of `Reference.Target`, reading the same announce-list
      -- channel by index. Arc Lightning's 1–3 targets are `Targets 0`. The
      -- obligation is the slot's: in range, cardinality Many (a One slot has no
      -- group read — that is `Target n`), kind from the slot.
      -- Rust: Selection::Targets(n).
      Targets : (n : Nat) -> {auto 0 prf : resolveTarget Many n (targets b) = Bound k} -> Selection b k
      -- the PLURAL anaphors ([CR#608.2d]): `They` (any noun) / `Them w` ("those
      -- tokens") read the nearest Many antecedent — a group-producing clause
      -- ("create two tokens … THEY gain haste", [CR#111.2]) or a many-binder —
      -- under the R1/R2 rules. NEVER a target: an announced slot is not on the
      -- stack, and is read positionally as `Targets n` above.
      They : {auto 0 prf : resolveThey Nothing (stack b) = Bound k} -> Selection b k
      Them : (w : Sort) -> {auto 0 prf : resolveThey (Just w) (stack b) = Bound k} -> Selection b k
      Random : Quantity b -> Predicate b k -> Selection b k
      -- the whole library as one collection ([CR#701.24a]) — shuffle's own object
      -- (`Action.Shuffle`); `whose` is ALWAYS spelled (Law 2 — no bare default), so
      -- "shuffle your library" is `LibraryOf You`, never a bare `LibraryOf`.
      LibraryOf : Reference b APlayer -> Selection b AnObject
      TopOfLibrary : (count : Count b) -> {default You whose : Reference b APlayer} -> Selection b AnObject
      BottomOfLibrary : (count : Count b) -> {default You whose : Reference b APlayer} -> Selection b AnObject
      -- the top `count` card(s) of `whose` GRAVEYARD ([CR#404.1,404.2] — cards are put on top of an
      -- ordered graveyard pile). Mirror of `TopOfLibrary`, but `whose` is EXPLICIT (not `{default You}`) —
      -- graveyard-topped effects (Volrath's Shapeshifter) name the player they inspect, unlike library
      -- manipulation's usual "your" default. No `BottomOfGraveyard` (no consumer).
      TopOfGraveyard : (count : Count b) -> (whose : Reference b APlayer) -> Selection b AnObject
      -- the extremal ELEMENT(s) of a `Projection` ("the creature with the greatest power" = `Pick MaxOf (eachOf
      -- yourCreatures (StatOf It Power))`). The element-twin of `Aggregate` (which folds the same `Projection` to
      -- a value); the op is gated to the extremal ones by `IsExtremal` (no `Pick SumOf`); ties yield the whole
      -- group, narrowed by the usual `Single`/choice path. Pinned to `AnObject` — a player-`Pick` ("target
      -- player with the most X") is sound but unrepresentable for now (no consumer; §8 leaves it as a
      -- one-line future generalization to `Projection b k`).
      Pick : (op : AggregateOp) -> {auto 0 ext : IsExtremal op} -> Projection b AnObject -> Selection b AnObject

  -- conjunctive PINS off a `Predicate` (the object-kind / zone / card-type
  -- atoms a conjunction fixes) — `sortFromPins` turns them into the noun the
  -- filter's objects answer to. Only pinning atoms contribute; a disjunction
  -- pins nothing (the conservative direction).
  public export
  pins : Predicate b k -> Pins
  pins (IsKind o) = MkPins (Just o) Nothing Nothing
  pins (InZone z) = MkPins Nothing (Just z) Nothing
  pins (HasChar Types t) = MkPins Nothing Nothing (Just t)
  pins (And ps) = pinsAll ps
  pins _ = noPins

  public export
  pinsAll : List (Predicate b k) -> Pins
  pinsAll [] = noPins
  pinsAll (p :: ps) = mergePins (pins p) (pinsAll ps)

  -- the noun a filter's objects answer to ([CR#110.1,108.2]): a player
  -- filter is `Player` (the kind index says so); an object filter derives
  -- from its pins — this is what an announced slot / binder stamps on the
  -- antecedent it pushes.
  public export
  filterSort : {k : RefKind} -> Predicate b k -> Sort
  filterSort {k = APlayer} _ = Player
  filterSort p = sortFromPins (pins p)

  -- the loop-element antecedent of a per-element construct over a filter.
  public export
  loopOf : {k : RefKind} -> Predicate b k -> Ant
  loopOf p = MkAnt (filterSort p) k One Loop Nothing Nothing

  -- `RevealUntil`'s BODY antecedents ([CR#702.85,701.57] dig-until): the
  -- found card (`Card` sort — a non-battlefield object, [CR#108.2] — not
  -- `filterSort`-derived; a revealed library card, never a battlefield
  -- noun) pushed `bindIt`-style so it re-sites to `Loop`; the passed-over
  -- prefix pushed as a `Frame` (mirrors `PileFrame`, [CR#608.2d]). ORDER is
  -- load-bearing: found is pushed LAST (nearest), so `resolveIt`'s
  -- `innermostBinder` scan reaches its `Loop` before it would otherwise
  -- reach the prefix's `Frame` (also a binder site); `resolveThey`'s
  -- `innermostFrame` scan ignores `Loop` sites regardless, so it reaches
  -- the prefix either way. `match` only fixes the found card's `RefKind`
  -- (`AnObject`, from `Predicate b AnObject`'s own index) at the type level.
  public export
  bindFound : Predicate b AnObject -> Ctx -> Ctx
  bindFound _ = bindIt (MkAnt Card AnObject One Product Nothing Nothing)
              . bindThat (MkAnt Card AnObject Many Frame Nothing Nothing)

  -- the element antecedent a `Projection` binds (`It` = each counted
  -- object/player); total over `Countable`, though only `Objects`/`Players` are projectable.
  public export
  projElemAnte : Countable b -> Ant
  projElemAnte (Objects p) = loopOf p
  projElemAnte (Players p) = loopOf p
  projElemAnte _ = MkAnt Permanent AnObject One Loop Nothing Nothing


  -- the patient kind survives a cap-combine only when BOTH sources agree on it (a disjunction's body can
  -- name the patient soundly iff every queried kind fixes the SAME kind); any mismatch or gap collapses to
  -- `Nothing`. No `Eq RefKind` needed — matched structurally.
  public export
  sameKind : Maybe RefKind -> Maybe RefKind -> Maybe RefKind
  sameKind (Just AnObject) (Just AnObject) = Just AnObject
  sameKind (Just APlayer)  (Just APlayer)  = Just APlayer
  sameKind (Just Anything) (Just Anything) = Just Anything
  sameKind (Just Empty)    (Just Empty)    = Just Empty
  sameKind _ _ = Nothing

  public export
  andCaps : EventCaps -> EventCaps -> EventCaps
  andCaps (MkEventCaps o1 a1 m1 p1 d1 pm1) (MkEventCaps o2 a2 m2 p2 d2 pm2) = MkEventCaps (o1 && o2) (a1 && a2) (m1 && m2) (sameKind p1 p2) (d1 && d2) (pm1 && pm2)

  -- the UNION twin of `andCaps`: a body gets an anaphor if ANY combined source supplies it. Used to fold the
  -- caps of a composite COST (`Costs […]`) — if one component sacrifices an object, the payment event binds it.
  -- The patient kind still needs agreement (`sameKind`), since a kind can't be unioned soundly.
  public export
  orCaps : EventCaps -> EventCaps -> EventCaps
  orCaps (MkEventCaps o1 a1 m1 p1 d1 pm1) (MkEventCaps o2 a2 m2 p2 d2 pm2) = MkEventCaps (o1 || o2) (a1 || a2) (m1 || m2) (sameKind p1 p2) (d1 || d2) (pm1 || pm2)

  -- the caps a whole event-QUERY guarantees its body: the INTERSECTION over its kind-disjunction — the
  -- body gets only anaphora that EVERY listed kind supplies. Empty `kinds` (any kind) ⇒ `NoCaps`. So a
  -- multi-kind trigger ("attacks or blocks") is sound, and there is no way to union incompatible kinds.
  public export
  eventQueryCaps : EventQuery b -> EventCaps
  eventQueryCaps q = case q.kinds of
    []        => NoCaps
    (k :: ks) => foldl andCaps (eventKindCaps k) (map eventKindCaps ks)

  -- the object-role NOUN a whole query supplies ([CR#400.7e]): its kinds must
  -- AGREE on it (the sort mirror of the caps intersection); disagreement falls
  -- back to the conservative `Permanent`.
  public export
  queryObjectSort : EventQuery b -> Sort
  queryObjectSort q = case map eventKindObjectSort q.kinds of
    [] => Permanent
    (s :: ss) => if all (sameSort s) ss then s else Permanent

  -- the role antecedents an event body pushes ([CR#608.2k]): one per
  -- caps guarantee, the object's noun derived from the query's kinds.
  public export
  queryRoles : EventQuery b -> List Ant
  queryRoles q = roleAntes (eventQueryCaps q) (queryObjectSort q)

  -- "it's your turn" — the common specialization of `TurnOf`.
  public export
  yourTurn : Condition b
  yourTurn = TurnOf (SameAs You)

  -- Sugar over the `Countable` core — readable common cases, no redundant constructors. `CountMatching`/
  -- `CountEvents` are the old `CountOf (Predicate)` / `EventCount`; `eachOf` builds a `Projection` without
  -- spelling `Objects` (so devotion reads `Aggregate SumOf (eachOf yourPermanents …)`). These are the canonical
  -- object/event spellings; raw `CountOf (Objects …)` / `Project (Objects …)` are reserved for the negative
  -- tests, and raw `CountOf (ManaSymbols …)` / `CountOf (Players …)` are the only spelling for those sources.
  public export
  CountMatching : Predicate b AnObject -> Count b
  CountMatching p = CountOf (Objects p)

  public export
  CountEvents : EventQuery b -> Count b
  CountEvents q = CountOf (Events q)

  public export
  eachOf : (p : Predicate b AnObject) -> Count (bindIt (loopOf p) b) -> Projection b AnObject
  eachOf p acc = Project (Objects p) acc

  -- the PLAYER-side twin of `eachOf`: builds a cross-player `Projection` without spelling `Players`
  -- ("highest life total among all players" = `Aggregate MaxOf (eachPlayer Anyone (PlayerStatOf It Life))`,
  -- Arbiter of Knollridge; "fewest lands any player controls" = Balance, §8 [CR#119.1]).
  public export
  eachPlayer : (p : Predicate b APlayer) -> Count (bindIt (loopOf p) b) -> Projection b APlayer
  eachPlayer p acc = Project (Players p) acc

  -- `exists`/`unique`: a predicate matches ≥1 / exactly-1 object. DERIVED from `CountOf` + `Compare`, not
  -- primitive constructors. `CountOf` takes a `Countable`, so `exists (During …)` is a TYPE error (a
  -- `Condition` is not a `Countable`), not a degenerate term.
  public export
  exists : Predicate b AnObject -> Condition b
  exists p = Compare (CountMatching p) Greater (Literal 0)

  public export
  unique : Predicate b AnObject -> Condition b
  unique p = Compare (CountMatching p) Eq (Literal 1)

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

  -- A game-result effect ([CR#104]). Its own category above `Action` — a game-ender
  -- isn't just another verb; `OneShotEffect`'s `Conclude` wraps it.
  namespace Outcome
    public export
    data Outcome : Ctx -> Type where
      WinGame  : Reference b APlayer -> Outcome b
      LoseGame : Reference b APlayer -> Outcome b

  -- A STATIC suppressor of a game outcome ([CR#104.2b,104.3e]) — distinct from the imperative `Outcome`
  -- above (Rust's lesson: win/lose-the-game is not a deontic over actions, nor a replaceable event, so
  -- it needs its own static channel). `OutcomeGate CantLose you` = Platinum Angel's first clause.
  namespace OutcomeGateKind
    public export
    data OutcomeGateKind = CantLose | CantWin

  -- Relative position between two objects in an ORDERED zone ([CR#404.2] — a graveyard is kept in a
  -- single face-up pile with a fixed order): OBJECT-relative, unlike the end-relative `Anchor` below.
  -- Gates `Predicate.Adjacent` (Death Spark's "directly above").
  namespace Adjacency
    public export
    data Adjacency = Above | Below

  -- A position in an ORDERED zone ([CR#401]) — an END plus an offset. `FromTop (^0)` = on top. Named
  -- `Anchor` (general over ordered zones — currently the library) rather than `LibraryPosition`.
  namespace Anchor
    public export
    data Anchor : Ctx -> Type where
      FromTop    : Count b -> Anchor b
      FromBottom : Count b -> Anchor b

  -- WHERE a card goes: a plain (unordered) zone, or an ordered zone at an `Anchor`. ONE notion of a
  -- destination — subsumes the old bare-`Zone` `Move` argument AND the single-object `PutIntoLibrary`.
  -- Sound by construction: only `ToLibrary` carries a position, so "graveyard at FromBottom 0" is
  -- unrepresentable.
  namespace Destination
    public export
    data Destination : Ctx -> Type where
      ToZone    : Zone -> Destination b
      ToLibrary : Anchor b -> Destination b

  -- the zone / noun a destination fixes for the moved object — the
  -- [CR#603.7c] expected-zone stamp and the [CR#400.7e] product noun.
  public export
  destZone : Destination b -> Zone
  destZone (ToZone z) = z
  destZone (ToLibrary _) = Library

  public export
  destSort : Destination b -> Sort
  destSort d = zoneSort (destZone d)

  -- an ORDERED zone is only a destination AT A POSITION ([CR#401.4] — bare
  -- Library is unanchored), and the stack is never a `Move` destination
  -- ([CR#405.1] — casting puts a spell there, not a move). Demanded by
  -- `Move`/`MoveGroup`.
  public export
  DestinationOk : Destination b -> Type
  DestinationOk (ToZone Library) = Void
  DestinationOk (ToZone Stack) = Void
  DestinationOk _ = ()

  -- How a SIMULTANEOUS group of cards is ordered as it lands at a position in an ORDERED zone — the
  -- order is a property of the PLACEMENT, not the loop. `ChosenOrder` = the owner arranges them, the
  -- [CR#401.4] "any order" default; `RandomOrder` = shuffled into place ([MTR 3.10], a randomized pile
  -- is the same kind of object as a shuffled library); `SameOrder` = preserve the source order (only
  -- meaningful from an already-ordered source). A single object has no internal order, so only
  -- `MoveGroup` (a group) carries it. The `…Order` suffix keeps `RandomOrder` distinct from the
  -- `Selection.Random` constructor ("N random objects").
  namespace Arrangement
    public export
    data Arrangement = ChosenOrder | RandomOrder | SameOrder

  -- A continuous effect's lifetime ([CR#611.2]). Rust: Duration. (Above the effect types
  -- so `Continuously` can name it.)
  namespace Duration
    public export
    data Duration : Ctx -> Type where
      UntilEndOfTurn : Duration b
      UntilEndOfCombat : Duration b                -- ends at the combat phase's end ([CR#511.2]; Rust: FixedUntil EndOfCombat)
      UntilYourNextTurn : Duration b               -- ends as the controller's next turn begins ([CR#611.2a]; Rust: FixedUntil YourNextTurn)
      UntilEvent : EventQuery b -> Duration b
      ForAsLongAs : Condition b -> Duration b   -- a resolution effect's duration: affected set FIXED at start, ends when the cond lapses ([CR#611.2b,611.2c]). DISTINCT from the re-evaluated conditional static `StaticEffect.While` ([CR#604.3]) — NOT a redundancy.
      Forever : Duration b                         -- rest of game (Rust: EndOfGame)

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

  namespace TargetSpec
    public export
    data TargetSpec : Ctx -> RefKind -> Type where
      -- a target slot: a NON-ZERO `Quantity` of targets matching the predicate (`Target (^1)` = one;
      -- `Target (between (^1) (^2))` = "one or two"). The announced slot pushes an ANTECEDENT the body
      -- may read back as an anaphor (`It` / `That w` / `They`), or read POSITIONALLY by its index in
      -- the `Targeted` list (`Reference.Target n`, this constructor's namesake in a different
      -- namespace) — the context-free escape hatch for same-sort/ambiguous slots.
      Target : (q : Quantity b) -> {auto 0 prf : NonZeroQ q} -> Predicate b k -> TargetSpec b k
      -- a co-target set-DISTINCTNESS constraint ([CR#115.7e], "any OTHER target"): this spec's picks
      -- must not overlap the sibling slots at these indices; `Targeted` bounds the indices.
      Distinct : (siblings : List Nat) -> TargetSpec b k -> TargetSpec b k

  -- a slot announces ONE target iff its quantity is literally one ([CR#115.3]);
  -- anything wider announces a Many group, read back as `They` ([CR#601.2d]).
  public export
  quantityCard : Quantity b -> Cardinality
  quantityCard (Range (Just (Literal 1)) (Just (Literal 1))) = One
  quantityCard _ = Many

  -- an announced slot's ENTRY in the announce list ([CR#115.3,601.2c]): noun
  -- from its filter, kind from its slot, cardinality from its quantity. It goes
  -- to `Ctx.targets` (read by index), never to the antecedent stack.
  public export
  slotAnte : {k : RefKind} -> TargetSpec b k -> Ant
  slotAnte (Target q p) = MkAnt (filterSort p) k (quantityCard q) TargetSlot Nothing Nothing
  slotAnte (Distinct _ t) = slotAnte t

  public export
  slotAntes : {ks : List RefKind} -> All (TargetSpec b) ks -> List Ant
  slotAntes [] = []
  slotAntes {ks = k :: ks'} (t :: ts) = slotAnte t :: slotAntes ts

  -- VESTIGIAL: no live constructor stamps an antecedent's `label` anymore
  -- (`As`/`The`/`TheGroup` retired in favour of positional `Target n` reads),
  -- so every `.label` is `Nothing` and this is trivially true. Kept only
  -- because `Targeted`'s `lbl` obligation below still names it — a home for
  -- labeled disambiguation should it ever return.
  public export
  labelFresh : Maybe String -> List Ant -> Bool
  labelFresh _ [] = True
  labelFresh l (a :: as) = not (sameLabel l a.label) && labelFresh l as

  public export
  labelsOk : List Ant -> Bool
  labelsOk [] = True
  labelsOk (a :: as) = labelFresh a.label as && labelsOk as

  -- a `Distinct` constraint may only name announce siblings that exist
  -- ([CR#115.7e,601.2c]).
  public export
  specDistinctOk : TargetSpec b k -> Nat -> Bool
  specDistinctOk (Target _ _) n = True
  specDistinctOk (Distinct sibs t) n = all (\i => i < n) sibs && specDistinctOk t n

  public export
  distinctOk : All (TargetSpec b) ks -> Nat -> Bool
  distinctOk [] n = True
  distinctOk (t :: ts) n = specDistinctOk t n && distinctOk ts n

  -- "the unique object matching a predicate" — sugar: the sole element of `SelectAll p`.
  public export
  Only : Predicate b AnObject -> Reference b AnObject
  Only p = Single (SelectAll p)

  -- a use-LIMIT on a `Replaces` — how many times it fires before it's CONSUMED (a shield). `Unlimited` =
  -- today's continuous replacement; `UpTo n` = "the next n" — n OCCURRENCES for an amountless event
  -- (regeneration: the next destroy), n AMOUNT-POINTS for an amount event (prevention: the next n damage).
  namespace ReplaceLimit
    public export
    data ReplaceLimit : Ctx -> Type where
      Unlimited : ReplaceLimit b
      UpTo : Count b -> ReplaceLimit b

  -- WHICH counters a `MoveCounters` relocates ([CR#122.5]). `Some c n` = n counters of one kind (Power
  -- Conduit, Leech Bonder; "all of that kind" = `Some c (CountersOn c from)`, the `RemoveCounters` idiom).
  -- `AllKinds` = every counter regardless of kind (Ozolith, Fate Transfer) — the one move case the single-
  -- kind form can't reach, since it quantifies over kinds rather than naming one. So move stays ONE verb;
  -- the kind/quantity (or "everything") is data, not a second constructor.
  namespace CounterSpec
    public export
    data CounterSpec : Ctx -> Type where
      Some : (c : CounterKind) -> Count b -> CounterSpec b
      AllKinds : CounterSpec b

  -- How many modes to choose, for a modal effect ([CR#700.2]). Rust: ChooseSpec. The count is a `Quantity`
  -- (the same range language as `Target`), so "choose one" = `^1`, "choose one or both" = `between (^1) (^2)`,
  -- "choose one or more" = `atLeast (^1)`, "choose up to two" = `atMost (^2)` (subsumes the old `upTo` flag).
  namespace ChooseSpec
    public export
    data ChooseSpec : Ctx -> Type where
      MkChooseSpec : (count : Quantity b) -> {default False repeats : Bool} -> ChooseSpec b

  -- a modal choose-count must not exceed the number of modes ([CR#700.2d]) — checked only when the UPPER
  -- bound is a LITERAL and modes can't repeat (a repeating choice, or an unbounded "one or more", is lenient,
  -- exactly like `NonZeroQ` guards only a literal bound). An unbounded upper is implicitly the mode count.
  -- A BOOL law (gated as `modalCountOk … = True`), not a Type family: a stuck Type-level gate over the
  -- mode list is opaque to the strict-positivity checker, a stuck Bool equation is not.
  public export
  modalCountOk : ChooseSpec b -> (modeCount : Nat) -> Bool
  modalCountOk (MkChooseSpec (Range _ (Just (Literal hi))) {repeats = False}) modeCount = hi <= modeCount
  modalCountOk _ _ = True

  -- A DEONTIC clause's carrier: a game ACTION a player may attempt ([CR#101.2,601.3] the deontic
  -- layer) — distinct from the resolving `Action` verbs. Each names its participants; the CR's
  -- "where ⟨pred⟩" qualifier rides the variable participant (`who`/`blocker`/`source`). The
  -- polarities `Constrain` (Require/Forbid)/`Priced` (in `StaticEffect`) wrap a `Deed`. BOUNDARY [CR#614.17]:
  -- this is choice-LEGALITY ("can't attack"); event-edits ("doesn't tap", "can't be regenerated",
  -- "can't lose") are `Replaces`/SBA, NOT a `Constrain`.
  -- the two COMPULSION polarities of a declaration constraint — the pair the combat solver balances
  -- ([CR#508.1c] restriction / [CR#508.1d] requirement): `Forbid` prevents the deed, `Require` forces
  -- it if able. `Constrain` (in `StaticEffect`) carries one; `cant`/`must` (Macros) are the aliases.
  namespace Compulsion
    public export
    data Compulsion = Require | Forbid

  -- the two PRICED-deed timings, folded into one `Priced` constructor: `AtDeclaration` = the cost is paid
  -- when the deed is declared (the old `Gate`, never compulsory, [CR#508.1d]); `Downstream` = it is punished
  -- after the fact (the old `Toll`, ward [CR#702.21a]).
  namespace PricedTiming
    public export
    data PricedTiming = AtDeclaration | Downstream

  namespace Deed
    public export
    data Deed : Ctx -> Type where
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
      BlockedBy  : (attacker : Predicate b AnObject) -> (size : Quantity b) -> {auto 0 prf : NonZeroQ size} -> Deed b

  -- A modification OPERATION on one characteristic axis `c` ([CR#613]) — the layer/base-vs-current is the
  -- OPERATION, not the axis name (so `Power`/`Toughness` drop the "Base"). `Set` overwrites any axis; `Up`/
  -- `Down` are the signed numeric deltas (layer 7c); `Add`/`Remove` add/remove one element of a collection
  -- axis. The gates make the mismatches unrepresentable: `Up` on `Colors` and `Add` on `Power` are ill-typed.
  namespace ModificationOp
    public export
    data ModificationOp : Ctx -> Characteristic -> Type where
      Set    : CharValue b c -> ModificationOp b c                          -- overwrite (any axis)
      Up     : {auto 0 _ : Numeric c}    -> Count b  -> ModificationOp b c   -- "+N" (numeric, layer 7c)
      Down   : {auto 0 _ : Numeric c}    -> Count b  -> ModificationOp b c   -- "−N"
      Add    : {auto 0 _ : Collection c} -> ElemOf c -> ModificationOp b c   -- add one element (collection)
      Remove : {auto 0 _ : Collection c} -> ElemOf c -> ModificationOp b c

  -- a best-effort NOUN for an `Existing` binder's group ([CR#608.2d]): the
  -- filter-backed shapes derive from their filter; library windows are cards;
  -- a union agrees or falls back; the plural anaphors re-read an existing
  -- antecedent, so their group keeps the conservative `Permanent`.
  public export
  selectionSort : {k : RefKind} -> Selection b k -> Sort
  selectionSort (SelectAll p) = filterSort p
  selectionSort (Random _ p) = filterSort p
  selectionSort (LibraryOf _) = Card
  selectionSort (TopOfLibrary c) = Card
  selectionSort (BottomOfLibrary c) = Card
  selectionSort (TopOfGraveyard _ _) = Card
  selectionSort (Pick op prj) = projSort prj
  selectionSort (Union gs) = unionSort gs
  -- `Targets n`'s group noun would be its slot's, but the `Ctx` index is
  -- erased here, so it takes the same coarse `Permanent` the plural anaphors
  -- take — the sort a group binder hands its loop elements. Unchanged from
  -- when Arc Lightning's slot was read as `They`.
  selectionSort (Targets _) = Permanent
  selectionSort They = Permanent
  selectionSort (Them _) = Permanent

  public export
  projSort : Projection b k -> Sort
  projSort (Project (Objects p) _) = filterSort p
  projSort (Project (Players p) _) = filterSort p
  projSort (Project _ _) = Permanent

  public export
  unionAgrees : {k : RefKind} -> Sort -> List (Selection b k) -> Bool
  unionAgrees x [] = True
  unionAgrees x (g :: gs) = sameSort x (selectionSort g) && unionAgrees x gs

  public export
  unionSort : {k : RefKind} -> List (Selection b k) -> Sort
  unionSort [] = Permanent
  unionSort (g :: gs) = if unionAgrees (selectionSort g) gs then selectionSort g else Permanent

  -- One big mutual block: `Ability → OneShotEffect → Action → CreateToken → Characteristics` is a
  -- cycle, so `Characteristics`/`Action`/`Bindable` join the effect/ability block below. `Cost` joins
  -- too — its `Do` wraps an `Action` ([CR#118.3]) — dragging the Cost-referencing `CostChange`
  -- in with it. (The leaf `ChooseSpec`/`Deed` stay OUT — they only reach into block 1.)
  -- A cost paid to activate an ability ([CR#118,602]). `Costs` conjoins components. Most costs ARE actions
  -- the payer performs ([CR#118.3]), so they ride `Do` rather than each getting a duplicate cost verb.
  namespace Cost
    public export
    data Cost : Ctx -> Type where
      Mana      : ManaCost -> Cost b                 -- "{4}"
      -- pay mana equal to an object's MANA COST ([CR#202.1]) — the full COLORED cost, the cost-language twin of
      -- the numeric `ManaValueOf`. "The flashback cost is equal to its mana cost" (Snapcaster's granted flashback).
      ManaCostOf : Reference b AnObject -> Cost b
      -- pay a cost by PERFORMING an action ([CR#118.3]): "{T}" = `Do (Tap This)`, "Pay N life" =
      -- `Do (ChangeLife You (Down (^N)))`, "Sacrifice this" = `Do (Sacrifice You (SameAs This))`, "Pay {E}×N" =
      -- `Do (RemoveCounters energy (^N) You)` (energy is a player counter — no dedicated `PayEnergy` verb),
      -- loyalty "+N"/"−N" = `Do (PutCounters/RemoveCounters loyaltyCounter (^N) This)`. UNRESTRICTED — ANY action
      -- (even scry/shuffle as a cost is legal); a senseless cost just no-ops, and nonsense is the grammar
      -- layer's to catch, not a gate.
      Do        : Action b -> Cost b
      Scaled    : Count b -> Cost b -> Cost b         -- the cost paid once per unit of the count ("{2} for each X" = Scaled (CountOf X) (Mana [promote 2]))
      Costs     : List (Cost b) -> Cost b            -- all components together
      -- AGGREGATE cost: tap a chosen subset of [of_] whose summed numeric characteristic [c] satisfies [cmp] [n].
      -- ONE shape for Crew ("tap creatures, total power ≥ N" = `TapTotal Power GreaterEq (^n) creature`) — and the
      -- Convoke/devotion-scaling family the engine's authors flagged it should subsume.
      TapTotal  : (c : Characteristic) -> {auto 0 _ : Numeric c} -> Cmp -> Count b -> (of_ : Predicate b AnObject) -> Cost b

  -- A continuous CHANGE to a spell/ability cost ([CR#118.7]), carried by `StaticEffect::CostModifier`.
  -- Borrowed from the Rust engine's key split: this MODIFIES an existing base — it is NOT an alternative
  -- cost (a base SWAP), which would be a separate type. Count-scaling is ONE recursive node, so affinity
  -- (`ScaledBy (Reduce [^1]) (CountOf …)`) and taxers (scale an `Increase`) need no own constructor.
  namespace CostChange
    public export
    data CostChange : Ctx -> Type where
      Reduce     : ManaCost -> CostChange b                 -- "costs {…} less" — a mana amount only (the mana-numeric layer [CR#118.7a])
      Increase   : ManaCost -> CostChange b                 -- "costs {…} more" — likewise mana-only; non-mana extras go through `Additional`
      Additional : List (Cost b) -> CostChange b            -- mandatory "as an additional cost, …"; may be NON-mana (sacrifice/discard/life) [CR#118.8]
      ScaledBy   : CostChange b -> Count b -> CostChange b  -- the change applied once per unit of the count (affinity)

  -- which pips of a spell's total cost an alternative payment ([CR#601.2]) may cover.
  namespace PipClass
    public export
    data PipClass = Generic | Colored   -- generic {1}, or a colored pip the helper must color-match (convoke's colored clause)

  -- the per-{1} alternative-payment action for `PayPips` ([CR#601.2]) — the object is chosen at payment time.
  namespace PayAct
    public export
    data PayAct : Ctx -> Type where
      TapToPay   : Predicate b AnObject -> PayAct b   -- tap an untapped matching permanent you control (convoke creatures / improvise artifacts / waterbend artifacts-or-creatures)
      ExileToPay : Predicate b AnObject -> PayAct b   -- exile a matching card from your graveyard (delve)

  -- An OPEN subtype ([CR#205.3g..205.3q]): its owning card-type category, its open name, and the
  -- abilities it CONFERS on its bearer. The conferral field (`List (Ability Base)`) makes this type
  -- reference `Ability` (declared below in this same mutual block), which is why `Subtype` had to move
  -- DOWN here. Its terse category helpers + `subtypeCategory` live just after this block.
  public export
  data Subtype : Type where
    MkSubtype : Category -> String -> List (Ability Base) -> Subtype

  -- The printable CHARACTERISTICS of an object ([CR#109.3]) — shared by a card `Face`
  -- (`Characteristics Base`) and a created token (`Characteristics b`, so a token's P/T can be a
  -- `Count b`: "an X/X where X = [a value known at creation]"). `colors` is the explicit color (a
  -- color indicator [CR#204.2] / a token's printed color); a card's color-FROM-MANA is derived.
  public export
  record Characteristics (b : Ctx) where
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
  -- optional. Demanded at `Normal` and `Create`.
  public export
  CharacteristicsOk : Characteristics b -> Type
  CharacteristicsOk c = NonEmpty (types c)

  -- PRODUCED mana ([CR#106.1]) — actual mana a mana ability adds. A DIFFERENT domain from the printed
  -- cost `ManaSymbol`: you produce colored/colorless units or "any color", never `{X}`/`{W/P}`/`{S}`.
  -- `Ctx`-indexed (not just an inert value type) because `AmongColorsOf`/`ProducedByEvent` name the
  -- ambient `Reference`/`eventCaps` — hence its home in this mutual block, beside `ManaRider`.
  namespace ProducedMana
    public export
    data ProducedMana : Ctx -> Type where
      OfColor : Maybe Color -> ProducedMana b     -- `OfColor (Just c)` = one {c}; `OfColor Nothing` = one {C}
      AnyColor : ProducedMana b                   -- one mana of any color (the producer picks)
      OneOf : List (Maybe Color) -> ProducedMana b -- one mana, the producer choosing from a FIXED set ("add {W} or {U}" = `OneOf [Just White, Just Blue]`); distinct from `AnyColor` (all five) and from a heterogeneous list (add ALL) ([CR#106.1])
      OneOfRuns : List (List (Maybe Color)) -> ProducedMana b -- one of several multi-symbol RUNS, producer choosing on resolution ("add {W}{W}, {W}{U}, or {U}{U}" = the filterland cycle = `OneOfRuns [[Just White, Just White], [Just White, Just Blue], [Just Blue, Just Blue]]`); the chosen run's whole sequence is added ([CR#106.1b])
      -- one mana of any of `r`'s colors ([CR#105.2]) — the producer picks AMONG a referenced object's
      -- colors, not a fixed set (Chrome Mox: `AmongColorsOf (Only (ExiledBy This))`, its imprinted card).
      -- Distinct from `OneOf` (a literal color list authored on the card).
      AmongColorsOf : Reference b AnObject -> ProducedMana b
      -- the TYPE the ambient mana-producing event actually produced ([CR#106.1b,106.12a]) — Vorinclex/
      -- Dictate of Karametra's "of any type that land produced". Gated by `producesMana (eventCaps b)`
      -- so it only typechecks inside a `TapForMana`-triggered body — the `EventObject`/`EventAmount` pattern.
      ProducedByEvent : {auto 0 prf : producesMana (eventCaps b) = True} -> ProducedMana b

  public export
  implementation Promote Color (ProducedMana b) where
    promote = OfColor . Just

  public export
  implementation Promote (Maybe Color) (ProducedMana b) where
    promote = OfColor

  -- A per-mana STRING attached to produced mana ([CR#106.6] enumerates exactly these three; they ride
  -- EACH mana the production makes, [CR#106.6a]). NOT snow (that's the source's `Snow` supertype) nor
  -- "doesn't empty" (a separate static/replacement over all your mana — Omnath/Upwelling), which would
  -- double-represent. The paid-for object is bound `It` in the two effect-bearing riders.
  namespace ManaRider
    public export
    data ManaRider : Ctx -> Type where
      SpendOnly      : Predicate b AnObject -> ManaRider b               -- (1) "spend only to cast/activate a [pred]" (Cavern's creature spell of the chosen type)
      GrantOnSpend   : StaticEffect (bindIt PaidSpellAnte b) -> ManaRider b   -- (2) the object it's spent on (`It`) gains [static] (Cavern's "that spell can't be countered")
      TriggerOnSpend : OneShotEffect (bindIt PaidSpellAnte b) -> ManaRider b  -- (3) a delayed trigger when the mana is spent ([CR#603.7a]); `It` = the object paid for

  -- the COMPOSITE keyword actions ([CR#701]) — the ones the grammar desugars rather than naming as a
  -- primitive verb (Reveal/Shuffle/… ARE primitive verbs below; Destroy is NOT — it composites over
  -- `Move` via the `destroy` macro). Their mechanics are just the primitives, but the engine must still
  -- RECOGNIZE the verb so triggers/replacements can name it ("whenever you scry", Aang's "whenever you
  -- waterbend …"); `Action.Composite` carries the tag. Minimal set today — the six constructors below;
  -- grows with card pressure (Proliferate, the bends, …).
  namespace KeywordActionSpec
    public export
    -- PARAMETERIZED (indexed) so each atom carries its natural arguments — the
    -- recognizable payload the `Act` event/filter matches on (the desugared
    -- `body` also mentions them, but the body is gone once the event fires). The
    -- arg-carrying arms mirror the already-parameterized `KeywordSpec`
    -- (`Ward Cost`, `Protection Quality`). ENTITY-KEYED and heterogeneous —
    -- each atom names only the coordinates it fixes ([CR#701]):
    --  * SLICE-family verbs read the top of a library
    --    ([CR#701.17a,701.22a,701.20a]). Scry/Surveil act on YOUR library by
    --    definition, so they carry only the `count` — no performer. Mill carries
    --    the PERFORMING player
    --    `who` (so `Act` records who milled — "whenever an opponent mills"),
    --    but NOT a count: its multi-card form rides the `Batch` count tier and
    --    each contained atom is a single top-slot slice.
    --    Fateseal ([CR#701.20a]) names the fatesealed player's library plus the
    --    `count` (an opponent's top, not yours).
    --    (Drawing is NOT here: [CR#701] does not list it — it is [CR#121], a
    --    game action — and it is irreducible ([CR#121.5]: a library → hand move
    --    made without the word "draw" is not a draw), so it has no body to read
    --    coordinates off. Draw is the `Action.DrawCard` verb below, carrying its own
    --    actor and count; Rust's twin is `DrawCard(who)` under a `Batch`.)
    --  * `Discard` ([CR#701.9a]) is a CHOICE verb: the affected cards live in the
    --    body's `With (Choose ..) ..`/`With (Existing (Random ..)) ..` decision
    --    (the [CR#701.9b] batch choice, or the bound single `Move` for "discard
    --    this card", [CR#702.29a]); the atom carries the discarding player `who`
    --    plus the count of the choice. The engine realizes per-card
    --    `Act (Discard …)` events so madness ([CR#702.35a]) and discard triggers
    --    bite card-by-card. (The per-card contained `Composite (Discard …)`
    --    carries the same `who`/count over one card.)
    --  * `Destroy` carries the patient object; its performer is the cause's
    --    agent/source (not a player), so no `who`.
    --  * `Fight` carries its two fighter objects.
    -- (Rust data-fies these as `Action::Composite(VerbName, body)` — the verb name
    -- plus the coordinates reconstructed from the expanded body; this typed mirror
    -- stays the soundness gate. `Mill` is the landed slice-family shape:
    -- an earlier draft keyed `Mill` on an object, superseded by the 2026-07-16
    -- `Batch`-containment ruling that made mill a slice verb.)
    data KeywordActionSpec : Ctx -> Type where
      Scry     : Count b -> KeywordActionSpec b
      Surveil  : Count b -> KeywordActionSpec b
      Fateseal : Reference b APlayer -> Count b -> KeywordActionSpec b
      Mill     : Reference b APlayer -> KeywordActionSpec b
      Discard  : Reference b APlayer -> Count b -> KeywordActionSpec b
      Destroy  : Reference b AnObject -> KeywordActionSpec b
      Fight    : Reference b AnObject -> Reference b AnObject -> KeywordActionSpec b

  -- The operand of `Action.ChangeLife` ([CR#119.3,119.5]) — mirrors the merged Rust `LifeOp`
  -- (`Set`/`Up`/`Down`): `Set` carries a plain `Count` (resolves as the gain/loss of the
  -- necessary difference, [CR#119.5]), `Up`/`Down` a plain gain/loss ([CR#119.3]). Paying life
  -- is losing life ([CR#119.4]) — cost position spells `Down`.
  namespace LifeOp
    public export
    data LifeOp : Ctx -> Type where
      Set : Count b -> LifeOp b
      Up : Count b -> LifeOp b
      Down : Count b -> LifeOp b

  -- The [CR#115.7a..115.7d] four-way discriminant `Action.Retarget` collapses — Bolt Bend and
  -- Redirect are different modes of the same verb, not different verbs. Mirrors the Rust
  -- `RetargetMode`.
  namespace RetargetMode
    public export
    data RetargetMode : Type where
      ChangeAll : RetargetMode
      ChangeOne : RetargetMode
      ChangeAny : RetargetMode
      ChooseNew : RetargetMode

  -- The verbs ([CR#701]). `OneShotEffect::Act` wraps these — merged with the former
  -- `PlayerAction` (the action-role-reshape design, 2026-08-01): a verb whose CR rule names a
  -- performer/patient carries that reference as an explicit, non-defaulted positional argument
  -- (Law 2 — no read-time default on any role slot); a verb whose CR rule is agent-silent
  -- carries none.
  namespace Action
    public export
    data Action : Ctx -> Type where
      -- deal damage to ONE recipient ([CR#120.1] — damage is to a single object/player per event);
      -- `source` object is the agent, ALWAYS spelled (`This` for the ability's source object / the
      -- resolving spell). "Deals N to EACH …" is a `Each` over the recipients.
      -- fields in printed-sentence order: `source` deals `amount` to `recipient`
      -- (`source, amount, target`, matching the RON `DealDamage` verb).
      DealDamage : Reference b AnObject -> Count b -> Reference b k -> Action b
      -- (divided damage — "N damage divided as you choose among [a group]" — is the general `Distribute`
      --  effect: `Distribute (^n) group (Act (DealDamage Allotment It))`, not a bespoke action.)
      -- a plain zone change [CR#400.7]; owner-relative, control implicit. `enteringAttacking` ([CR#508.4],
      -- default `Nothing`): on a battlefield destination, put the object on ATTACKING the named player
      -- (Ninjutsu's "tapped and attacking", Encore) — senseless (no-op) for any non-battlefield destination.
      -- `from` (default `Nothing`): a FIZZLE-GUARD — commit the move only if the object is CURRENTLY in
      -- that zone, the zone precondition destroy/discard/mill state declaratively; a mismatch is a silent
      -- no-op, never an error [CR#701.8a,701.9a,701.17a].
      Move : Reference b AnObject -> (d : Destination b) -> {auto 0 dOk : DestinationOk d} -> {default Nothing enteringAttacking : Maybe (Reference b APlayer)} -> {default Nothing from : Maybe Zone} -> Action b
      -- (There is no "exile until ~" verb. A duration-bounded zone change is `Relocate` (a
      --  StaticEffect) under a `Continuously`/`While` duration — see `Relocate` and
      --  card_BanishingLight. Permanent exile is just `Move … (ToZone Exile)`.)
      -- counter a stack object [CR#701.6a]. (Destroy [CR#701.8] is NOT a primitive verb:
      -- it composites as `Composite (Destroy r) (Move r Graveyard)` via the `destroy` macro,
      -- like scry/mill. Return-to-hand is just `Move … Hand` — `Move` is owner-relative —
      -- so there's no dedicated bounce verb.)
      Counter : Reference b AnObject -> Action b
      -- tap / untap [CR#701.26]; attach / unattach [CR#701.3].
      Tap : Reference b AnObject -> Action b
      Untap : Reference b AnObject -> Action b
      RemoveDamage : Reference b AnObject -> Action b    -- remove all damage marked on r (regeneration's heal, [CR#701.19])
      Transform : Reference b AnObject -> Action b   -- turn a transforming DFC to its other face ([CR#701.27])
      Attach : (what : Reference b AnObject) -> (to : Reference b AnObject) -> Action b
      Unattach : Reference b AnObject -> Action b
      -- `agent` draws a card ([CR#121.1]) — exactly ONE card ([CR#121.2]). Rust's twin is
      -- `DrawCard(agent)` under a `Batch` count ([CR#121.2] — the
      -- individual card draw is the replaceable unit there); the emitter folds that
      -- `Batch` into this `Count`, since `actionIntro` derives the card/amount
      -- anaphora from it ("draw three cards, then gain THAT MUCH life").
      -- Deliberately NOT a `KeywordActionSpec`: [CR#701] does not list drawing, and
      -- [CR#121.5] makes it irreducible, so there is no `Composite` body for it.
      DrawCard : Reference b APlayer -> Count b -> Action b
      -- change a player's life total ([CR#119.3,119.9]) — the merged `GainLife`/`LoseLife`/
      -- `SetLifeTo` family (Rust: `Action::ChangeLife`, the funnel criterion: every route into
      -- the life total is a gain or a loss, [CR#119.2,119.4,119.5,119.7,119.8]). `patient` is the
      -- player whose total changes; `LifeOp` carries the direction.
      ChangeLife : Reference b APlayer -> LifeOp b -> Action b
      -- put a GROUP at an ordered position with an `Arrangement` ([CR#401.4]): "put the top three on the
      -- bottom in any order" = `MoveGroup (TopOfLibrary (^3)) ChosenOrder (ToLibrary (FromBottom (^0)))`;
      -- "...in a random order" = `… RandomOrder …`. Distinct from single `Move` — order
      -- only EMERGES for a simultaneous group landing in an ordered zone. (Per `DealDamage`-single +
      -- group-via-`Each` house style, single moves stay `Move`; this is the group verb.)
      MoveGroup : Selection b AnObject -> Arrangement -> (d : Destination b) -> {auto 0 dOk : DestinationOk d} -> Action b
      -- put / remove counters ([CR#122]). `RemoveCounters` is symmetric with `PutCounters` (a `Count`);
      -- "remove all of a kind" is `RemoveCounters c (CountersOn c r) r`. Loyalty/counter COSTS reuse these via
      -- `Do` (e.g. "−2" = `Do (RemoveCounters loyaltyCounter (^2) This)`), so there is no duplicate counter-cost verb.
      PutCounters : (c : CounterKind) -> Count b -> Reference b (counterKindScope c) -> Action b
      RemoveCounters : (c : CounterKind) -> Count b -> Reference b (counterKindScope c) -> Action b
      -- MOVE counters object→object ([CR#122.5] = remove-from + put-on, one operation). The `CounterSpec`
      -- says which: `Some c n` (Power Conduit, Leech Bonder) or `AllKinds` (Ozolith). Both ends are objects
      -- (counters don't move between players), so no `counterKindScope` indexing — a senseless kind just no-ops.
      MoveCounters : CounterSpec b -> (from : Reference b AnObject) -> (to : Reference b AnObject) -> Action b
      -- `agent` sacrifices a permanent matching `pred` ([CR#701.21a] — a player can sacrifice only
      -- what they control, so `agent` is that permanent's controller). Rust's `Sacrifice(Reference,
      -- Reference)` resolves "what" to a single reference before this verb runs; the emitter
      -- re-derives an equivalent Idris `Predicate` off that reference (`SameAs`).
      Sacrifice : Reference b APlayer -> Predicate b AnObject -> Action b
      -- further keyword-action verbs ([CR#701]). The interactive bits (reorder, search choice, copy
      -- characteristics) are the engine's; the grammar names the verb. Scry/Surveil/Mill/Fight are NOT
      -- primitive verbs — they COMPOSITE over these primitives (`Each`/`With`/`Modal`/`Move`/`DealDamage`)
      -- as macros in `Macros.idr`, then wrap that desugaring in `Composite` (below) so the engine still
      -- RECOGNIZES the action — exactly the keyword ABILITIES' `Bare`/`Composite` split, on the verb side.
      -- (A bare, non-composite `Discard` verb no longer exists — [CR#701.9] discard is always the
      -- `Composite (Discard by n) body` shape now, matching Rust's `Action::discard`/`discard_what`.)
      Reveal : Reference b AnObject -> Action b
      -- shuffle a collection ([CR#701.24a]: "a library or a face-down pile of cards") — the
      -- collection is the PATIENT, not an agent; the owner is derivable from the collection term
      -- (Law 3). Rust: `Shuffle(Selection)` — `LibraryOf(who)` names a whole library.
      Shuffle : Selection b AnObject -> Action b
      -- "[player] takes an extra turn after this one" ([CR#500.7]) — Time Walk. (Rust: `ExtraPhase`;
      -- no Idris/Rust bridge exists for this verb — see `idris_emit.rs`'s `Action::ExtraPhase` gap.)
      ExtraTurn : (actor : Reference b APlayer) -> Action b
      -- "you control [whom] during their next turn" ([CR#723]) — Mindslaver: you make all of their
      -- decisions. The next-turn duration is the standard one the engine applies. (Rust:
      -- `GainControl`; no Idris/Rust bridge exists for this verb either — see the `Action::
      -- GainControl` gap.)
      ControlPlayer : (whom : Reference b APlayer) -> Action b
      -- `agent` creates the token(s) ([CR#111.1,701.7,701.7a] — "target player creates" is real
      -- information). The token's full characteristics (P/T may be a `Count b`); `enteringAttacking`
      -- ([CR#508.4], default `Nothing`) = create it ATTACKING the named player (Myriad/Encore's
      -- "attacking that [opponent]"). Rust: `Action::Create { agent, count, token, riders }`.
      Create : Reference b APlayer -> Count b -> (c : Characteristics b) -> {auto 0 wf : CharacteristicsOk c} -> {default Nothing enteringAttacking : Maybe (Reference b APlayer)} -> Action b
      -- "copy [r], except <mods>" — a spell/ability copy on the stack ([CR#707.10]) or a token copy of a permanent
      -- ([CR#707.2]). The `List (Modification b)` carries the copiable-value alterations ([CR#707.9] — "a copy, except it's
      -- a 4/4"), each a SIBLING higher-layer mod (never bundled INTO the copy, same doctrine as `BecomeCopyOf` below); it is
      -- `[]` for a bare copy (`Copy r []`). A permanent BECOMING a copy is `BecomeCopyOf`. Copies carry the original's
      -- modes/targets/X; "you may choose new targets" is a separate `Retarget`. Also the Idris
      -- target of Rust's `CopySpell { controller, spec, retarget }` (a stack-object copy,
      -- [CR#707.10]) — the token-copy and stack-copy sites share this one constructor.
      Copy : Reference b AnObject -> List (Modification b) -> Action b
      -- `by` picks new targets for the stack object `of`, bound by its original targetspec
      -- ([CR#115.7a..115.7d,707.10c] — Bolt Bend, Redirect, copy-with-new-targets). `mode` carries
      -- the four-way discriminant (`RetargetMode`) — Bolt Bend (`ChangeOne`) and Redirect
      -- (`ChangeAny`) are different modes, not the same verb. Rust: `Action::Retarget { mode, of, by }`.
      Retarget : RetargetMode -> (of_ : Reference b AnObject) -> (by : Reference b APlayer) -> Action b
      -- "add mana" (a mana-ability effect; pool/paying is engine) ([CR#106.1,106.4]). ONE verb (merges the
      -- old `AddMana` + `AddManaFor`): `amount` copies of one `ProducedMana`, so fixed "{C}" (`amount = ^1`),
      -- {X}/devotion/count-scaled production (Gaea's Cradle, Karametra's Acolyte), and a producer-chosen
      -- color (`OneOf`/`AnyColor`) all fall out of the value language. `riders` are the per-mana strings of
      -- [CR#106.6] applied to each of the `amount` mana ([CR#106.6a]) — Cavern: a chosen color, only to cast
      -- the chosen creature type, uncounterable. (Fixed HETEROGENEOUS production — "add {R}{G}" — is a
      -- `Sequentially` of `AddMana`s, so the old per-action list is gone.)
      AddMana : (recipient : Reference b APlayer) -> (amount : Count b) -> ProducedMana b
                -> {default [] riders : List (ManaRider b)} -> Action b
      -- a COMPOSITE keyword action ([CR#701]): `tag` NAMES the verb, `body` is its primitive desugaring
      -- (`Each`/`With`/`Modal`/`Sequentially` over the verbs above). The action-side twin of
      -- `KeywordAbility.Composite` — the mechanics are just the primitives, but the tag lets the engine
      -- RECOGNIZE the action ("scry 2" = `Composite (Scry You (^2)) (…)`) so "whenever you scry"/Aang's
      -- "whenever you waterbend" match. Built by the `scry`/`surveil`/`mill`/`destroy` macros here and
      -- the RON grammar macros (e.g. `Fight`, `plugins/builtin/macros/effect/Fight.ron`).
      -- Rust: Action::Composite(KeywordAction, Box<OneShotEffect>), reached via the flattened `Act`.
      Composite : KeywordActionSpec b -> OneShotEffect b -> Action b
      -- `agent` rolls `sides`-sided dice ([CR#706.1]) `count` times; the RESULT rides the pushed
      -- `amountAnte` antecedent (a later `Compare ThatMany …` reads it, [CR#706.2]).
      RollDice : Reference b APlayer -> (count : Count b) -> (sides : Nat) -> Action b
      -- `agent` flips `count` coins ([CR#705.1]); the win/loss RESULT rides the pushed `amountAnte`
      -- antecedent, like `RollDice`. `called` splits [CR#705.2]'s two kinds: True = the flipper
      -- calls heads/tails and wins or loses the flip; False = the effect reads only
      -- heads/tails and NO player wins or loses.
      FlipCoins : Reference b APlayer -> (count : Count b) -> (called : Bool) -> Action b
      -- `agent` rolls the (Planechase) planar die as a special action ([CR#901.9]); NO numeric result
      -- ([CR#901.9d]) so unlike `RollDice`/`FlipCoins` this introduces nothing for `ThatMany`.
      RollPlanarDie : Reference b APlayer -> Action b
      -- pay a cost as an action ([CR#118.12]) — a slotless `Cost` -> `Action` adapter; the payer is
      -- rule-forced by every legal host, never a slot (`May.who` for the collapsed `MayPay`/`MustPay`
      -- shape [CR#118.12a], `AdditionalCost`'s controller [CR#601.2b]). Bare effect-position `Pay` has
      -- no legal host and is unspellable. Rust: Action::Pay.
      Pay : Cost b -> Action b

  -- the event-anaphor caps the PAYMENT of a cost-action supplies its `AdditionalCost` body — the cost-side
  -- twin of `eventKindCaps`, for the object-moving cost verbs. Sacrifice/Discard bind the moved object
  -- (`EventObject`) + payer (`EventActor`); a zone change (exile-to-pay) binds the object; the tap/untap/
  -- life/counter payments carry their event kind's caps. TOTAL-BY-ENUMERATION (`%default total` demands
  -- it): every verb has an explicit row — a capless verb says `NoCaps` in its own arm, so a
  -- NEW verb forces a decision here rather than silently binding nothing.
  public export
  actionEventCaps : Action b -> EventCaps
  actionEventCaps (Action.Sacrifice _ _)      = eventKindCaps Sacrifice
  actionEventCaps (Action.Move _ _)           = eventKindCaps (ZoneChanged Nothing Nothing)
  actionEventCaps (Action.Tap _)              = eventKindCaps (Becomes Tapped)
  actionEventCaps (Action.Untap _)            = eventKindCaps (Becomes Untapped)
  actionEventCaps (Action.ChangeLife _ (LifeOp.Down _)) = eventKindCaps LoseLife
  actionEventCaps (Action.ChangeLife _ _)     = NoCaps
  actionEventCaps (Action.RemoveCounters _ _ _) = eventKindCaps RemoveCounters
  actionEventCaps (Action.Reveal _)           = NoCaps   -- reveals bind nothing ([CR#701.20a])
  actionEventCaps (Action.DealDamage _ _ _)   = NoCaps
  actionEventCaps (Action.Counter _)          = NoCaps
  actionEventCaps (Action.RemoveDamage _)     = NoCaps
  actionEventCaps (Action.Transform _)        = NoCaps
  actionEventCaps (Action.Attach _ _)         = NoCaps
  actionEventCaps (Action.Unattach _)         = NoCaps
  actionEventCaps (Action.DrawCard _ _)       = NoCaps
  actionEventCaps (Action.MoveGroup _ _ _)    = NoCaps
  actionEventCaps (Action.PutCounters _ _ _)  = NoCaps
  actionEventCaps (Action.MoveCounters _ _ _) = NoCaps
  actionEventCaps (Action.Shuffle _)          = NoCaps
  actionEventCaps (Action.ExtraTurn _)        = NoCaps
  actionEventCaps (Action.ControlPlayer _)    = NoCaps
  actionEventCaps (Action.Create _ _ _)       = NoCaps
  actionEventCaps (Action.Copy _ _)           = NoCaps
  actionEventCaps (Action.Retarget _ _ _)     = NoCaps
  actionEventCaps (Action.AddMana _ _ _)      = NoCaps
  actionEventCaps (Action.Composite _ _)      = NoCaps
  actionEventCaps (Action.RollDice _ _ _)     = eventKindCaps RollDice
  actionEventCaps (Action.FlipCoins _ _ _)    = eventKindCaps (FlipCoin Nothing)
  actionEventCaps (Action.RollPlanarDie _)    = eventKindCaps (RollPlanarDie Nothing)
  -- a payment's own event caps are never read through THIS channel (`Pay` is
  -- never a cost-side `Do` component — it has no legal host there); `NoCaps`
  -- keeps the enumeration total without a forward reference to `costCaps`.
  actionEventCaps (Action.Pay _)              = NoCaps

  -- the caps a COST's payment supplies its `AdditionalCost` body: an action pays via its event; a composite
  -- `Costs […]` binds ONLY when EXACTLY ONE component actually supplies an event (see `costsCaps`); `Scaled`
  -- rides its inner cost; pure mana binds nothing. Mirrors `eventQueryCaps` — the value-driven cap derivation
  -- an `AdditionalCost` body's type uses.
  public export
  costCaps : Cost b -> EventCaps
  costCaps (Do a)          = actionEventCaps a
  costCaps (Scaled _ c)    = costCaps c
  costCaps (Costs cs)      = costsCaps cs
  costCaps (Mana _)        = NoCaps
  costCaps (ManaCostOf _)  = NoCaps
  costCaps (TapTotal _ _ _ _) = NoCaps

  -- a component's caps are non-`NoCaps` iff it actually supplies SOME role — the discriminator `costsCaps`
  -- folds over to count real events, distinct from a capless component (`Mana`/`TapTotal`/a non-event `Do`)
  -- contributing nothing to count.
  public export
  capsPresent : EventCaps -> Bool
  capsPresent (MkEventCaps o a m p d pm) = o || a || m || isJust p || d || pm

  -- `costsCaps`'s structural fold, spelled with an explicit accumulator (Idris's totality checker can't see
  -- through `map`/`filter` to the mutually-recursive `costCaps` call, so this stays a direct `c :: cs` walk
  -- like the original `orCaps` fold it replaces). `acc` is the ONE real event's caps seen so far; a SECOND
  -- real event short-circuits straight to `NoCaps` (equivalent to scanning the rest and getting `NoCaps`
  -- anyway, since the count can only grow).
  public export
  costsCapsGo : Maybe EventCaps -> List (Cost b) -> EventCaps
  costsCapsGo acc [] = fromMaybe NoCaps acc
  costsCapsGo acc (c :: cs) with (capsPresent (costCaps c))
    costsCapsGo acc      (c :: cs) | False = costsCapsGo acc cs
    costsCapsGo Nothing  (c :: cs) | True  = costsCapsGo (Just (costCaps c)) cs
    costsCapsGo (Just _) (c :: cs) | True  = NoCaps

  -- the composite fold ([CR#608.2k,118.10]): a body reading a cost→effect role (`EventObject`/`EventActor`/
  -- `EventPatient`) refers to ONE event — so a multi-component cost binds its roles ONLY when EXACTLY ONE
  -- component actually supplies an event ([CR#118.10] "a payment... applies to only one spell, ability, or
  -- effect", extended here to the payment's OWN internal event-count). Zero real events (pure mana/tap-total)
  -- ⇒ `NoCaps`, unchanged. Two-or-more real events (e.g. `Costs [Do Sacrifice, Do Discard]`) ⇒ `NoCaps` too —
  -- AMBIGUOUS, so a body-side role read now FAILS TO TYPECHECK (`hasObject (eventCaps b) = True` has no
  -- proof) instead of silently reading whichever component the Rust walker (`cost_paid_object`) happens to
  -- see first. Upgrades the previous unconditional `orCaps` union, which bound roles for ANY qualifying
  -- multi-component cost regardless of count.
  public export
  costsCaps : List (Cost b) -> EventCaps
  costsCaps cs = costsCapsGo Nothing cs

  -- What a binder (`With`) binds as `That`: a QUERY of existing objects, a PRODUCER
  -- (an `Action` run for effect, binding its product), or a CHOICE (a player picks).
  -- The grammar only names the role; the ENGINE resolves `That` to the live (reminted
  -- or gone) object, so `MovedRef`/lki/became is a runtime concern, NOT modeled here.
  namespace Bindable
    public export
    -- CARDINALITY-typed (`One`/`Many`): a `One`-binder's `That` reads back as a single `Reference`; a
    -- `Many`-binder's `That` is a `Selection` you `Each` over. The `^1`/`^3` quantities share one type, so the
    -- ONE-ness can't ride the quantity — it's a dedicated constructor (`ChooseOne` vs `Choose`).
    data Bindable : Ctx -> Cardinality -> RefKind -> Type where
      -- ONE-binders → `That : Reference b k` (a single object):
      Produce : Action b -> Bindable b One AnObject  -- run the action, bind its product (the moved object) as `That`
      ChooseOne : {default You by : Reference b APlayer} -> Predicate b k -> Bindable b One k  -- `by` chooses exactly ONE match (interactive, so here not in `Selection`)
      SearchOne : {default You by : Reference b APlayer} -> {default You whose : Reference b APlayer} -> {default [Library] from : List Zone} -> Predicate b k -> Bindable b One k  -- search `whose`'s `from`-zones for exactly ONE
      TheRef : Reference b k -> Bindable b One k  -- bind an EXISTING single reference (a captured target / `This`)
      -- MANY-binders → `That : Selection b k` (a group, `Each`-iterated):
      Existing : Selection b k -> Bindable b Many k  -- bind existing entities (a plain selection / group)
      -- `by` chooses a `Quantity` of entities matching the filter; the chosen are bound as
      -- `That`. Choosing is interactive, so it lives here, not in `Selection`. Rust: Binder::Choose.
      Choose : {default You by : Reference b APlayer} -> Quantity b -> Predicate b k -> Bindable b Many k
      -- `by` searches `whose`'s `from`-zones (one or more — "library and/or graveyard") for
      -- matching cards, bound as `That` — like `Choose`, but from (hidden) zones the engine
      -- reveals/shuffles. Search ANOTHER player's via `whose`; the found card's destination
      -- is a following owner-routed `Move That …`. Rust: Binder::Search.
      Search : {default You by : Reference b APlayer} -> {default You whose : Reference b APlayer} -> {default [Library] from : List Zone} -> Quantity b -> Predicate b k -> Bindable b Many k

  -- the antecedent a `With`/`Each`/`Distribute` binder introduces for its
  -- body ([CR#608.2d]): a CHOICE binder is a deterministic `Frame` (the
  -- legacy `That`-scope semantics — inside it the anaphors bind the frame,
  -- never R2-gated); a search/produce binder pushes a whiffable `Product`
  -- ([CR#701.23,400.7j]) that resolves like any other stack antecedent.
  public export
  binderAnte : {k : RefKind} -> {cd : Cardinality} -> Bindable b cd k -> Ant
  binderAnte (Produce act) = MkAnt (produceSort act) k cd Product Nothing Nothing
  binderAnte (ChooseOne p) = MkAnt (filterSort p) k cd Frame Nothing Nothing
  binderAnte (SearchOne p) = MkAnt Card k cd Product Nothing Nothing
  binderAnte (TheRef r) = MkAnt (kindSort k) k cd Frame Nothing Nothing
  binderAnte (Existing g) = MkAnt (selectionSort g) k cd Frame Nothing Nothing
  binderAnte (Choose q p) = MkAnt (filterSort p) k cd Frame Nothing Nothing
  binderAnte (Search q p) = MkAnt Card k cd Product Nothing Nothing

  -- a produced (moved) object's noun follows its destination ([CR#400.7e]);
  -- a non-move producer is conservatively a `Permanent`.
  public export
  produceSort : Action b -> Sort
  produceSort (Move r d) = destSort d
  produceSort _ = Permanent

  -- Effects, continuous effects, and abilities are mutually recursive: a one-shot
  -- can CREATE a continuous effect (`Continuously`), a static ability can grant an
  -- ability, and an ability wraps an effect.
  namespace OneShotEffect
    public export
    data OneShotEffect : Ctx -> Type where
      -- the TELESCOPE ([CR#608.2d]): clause i+1 elaborates in the context
      -- extended by clause i's introductions (`SeqList`/`intro` below) —
      -- sentence order IS binder order, so "Exile target creature, then
      -- return THAT CARD" needs no binder inversion. Rust: OneShotEffect::Sequentially.
      Sequentially : SeqList b -> OneShotEffect b
      -- SIMULTANEOUS ([CR#701.14a]): every member reads ONE pre-application
      -- snapshot and the facts land as a single batch — NOT a telescope, so
      -- members don't thread introductions to each other (plain `List`, not
      -- `SeqList`). Fight's two `DealDamage` halves each read the pre-damage
      -- powers; a self-fight's same-source/same-target packets coalesce to one
      -- 2x instance ([CR#701.14c], engine-side). Rust: OneShotEffect::Simultaneously.
      Simultaneously : List (OneShotEffect b) -> OneShotEffect b
      -- each announced slot carries its OWN kind (its filter's), gathered as `ks : List RefKind`
      -- (a heterogeneous `All`); the slots push antecedents the body reads back as anaphors
      -- ([CR#115.3,601.2c]) — mixed-kind multi-target (Donate) disambiguates by SORT, same-sort
      -- slots POSITIONALLY (`Reference.Target n`, [CR#115.3]). The gate: `Distinct` sibling
      -- indices in range ([CR#115.7e]); `lbl` is a vestigial no-op (see `labelFresh`).
      Targeted : {ks : List RefKind} -> (ts : All (TargetSpec b) ks) -> {auto 0 lbl : labelsOk (slotAntes ts) = True} -> {auto 0 rng : distinctOk ts (length ks) = True} -> OneShotEffect (bindTargets (slotAntes ts) b) -> OneShotEffect b
      -- binds `that`'s antecedent for `body` ([CR#608.2d]): a choice binder is a deterministic
      -- frame; a produce/search binder a whiffable product. Read back by sort (`It`/`That w`/
      -- `They`). Rust: OneShotEffect::With.
      With : (that : Bindable b cd k) -> OneShotEffect (bindThat (binderAnte that) b) -> OneShotEffect b
      -- mid-resolution VALUE choice: "choose a [color/type/name/number/mode], then [body]".
      -- The effect-level twin of `AsEnters` (which is enters-only) — `body` runs at `bindChosen d b`, reading the
      -- pick via `OfChosen`/`ChosenIs`/`ChosenNumber`. Three Tree City's "{2},{T}: Choose a color. Add mana of
      -- that color…" = `WithChosenValue AColor (Act (AddMana … <of the chosen color>))`. Rust: Noting/ChooseAndNote (no WithChosenValue variant).
      WithChosenValue : (d : ChooseDomain) -> {auto 0 ok : ModeDomainOk d} -> OneShotEffect (bindChosen d b) -> OneShotEffect b
      -- a single intrinsic instruction (the verb compartment). Rust: OneShotEffect::Act.
      Act : Action b -> OneShotEffect b
      -- end the game (or a player's part in it) — the `Outcome` compartment. Rust has no
      -- `OneShotEffect::Conclude`: win/lose live in `PlayerAction::{WinGame,LoseGame}` (an
      -- `Action`, reached via `Act`), which the idris_emit bridge maps back to `Conclude (WinGame/LoseGame …)`.
      Conclude : Outcome b -> OneShotEffect b
      -- "you may [effect]", with optional "if you do / if you don't". The "if you do" branch —
      -- and the May's right siblings (`introduces`) — see the inner effect's introductions
      -- (the Through the Breach shape: "You may put a creature card … THAT creature gains
      -- haste"); a DECLINED May's products are runtime-skipped, not scope-blocked ([CR#701.23b]).
      -- `who` (default You, [CR#608.2d]) is the decider; every plain-`May` call site keeps `who`
      -- implicit (Idris's pre-existing simplification — Rust's `who` is required, Law 2), but the
      -- collapsed cost-payment shape below needs it spelled (Mana Leak's non-You payer). Rust:
      -- OneShotEffect::May.
      May : (effect : OneShotEffect b) -> {default You who : Reference b APlayer} -> {default Nothing ifDid : Maybe (OneShotEffect (intro effect b))} -> {default Nothing ifNot : Maybe (OneShotEffect b)} -> OneShotEffect b
      -- "if [cond], [thenDo]; otherwise [else]". Rust: OneShotEffect::If.
      If : Condition b -> (thenDo : OneShotEffect b) -> {default Nothing otherwise : Maybe (OneShotEffect b)} -> OneShotEffect b
      -- COST-payment DECISIONS collapse into `May (Act (Pay cost))` ([CR#118.12a] — the CR itself
      -- defines the punisher as `May`-without-`ifDid`): "[who] MAY pay [cost]; if they do → `ifDid`; if
      -- not → optional `ifNot`" is `mayPayCostBy who (Act (Pay cost)) ifDid ifNot`; the Mana Leak
      -- punisher ("counter target spell unless its controller pays {2}") is the same shape with
      -- `ifDid` absent. `intro`'s own `Pay` arm (below) types the "if you do" branch over the PAYMENT,
      -- like an `AdditionalCost` body ([CR#608.2d,601.2f]): the cost's caps + role antecedents; the
      -- "if you don't" branch runs unpaid, in the plain context.
      -- "As an additional cost, [pay]." Mirrors the printed additional-cost CLAUSE: the payment is an EVENT, so
      -- `body` reads the sacrificed/exiled object through the SAME `EventObject`/`EventActor`/`EventAmount`
      -- anaphors a trigger uses ("the sacrificed creature's power" = `StatOf EventObject Power`; Fling/Momentous
      -- Fall). At the ROOT of a `Spell`/`Activated` effect the engine HOISTS it to cast/activation time (the
      -- printed additional cost, [CR#601.2f,118.8]); nested, it's an extra resolution-time cost. The body's caps
      -- are value-derived (`costCaps pay`), exactly like `Delayed`/`Triggered`. Rust: OneShotEffect::AdditionalCost.
      AdditionalCost : (pay : Cost b) -> OneShotEffect (bindEvent (costCaps pay) (costRoles (costCaps pay)) b) -> OneShotEffect b
      -- create a continuous effect for a duration ([CR#611.2]): `Continuously UntilEndOfTurn (Modify This …)`.
      -- Duration FIRST so it reads "continuously, for [duration], [effect]". Rust: OneShotEffect::Continuously.
      Continuously : Duration b -> StaticEffect b -> OneShotEffect b
      -- choose modes, then apply them ([CR#700.2]). The mode list is a `Vect (S n)` — ≥1 mode
      -- BY CONSTRUCTION ([CR#700.2]; replaces the `NonEmpty` gate), and the choose-count gate
      -- ([CR#700.2d]) ranges over the NAT `S n`, not the mode list (a gate over a mutual-group
      -- value is opaque to the strict-positivity checker). Rust: OneShotEffect::Modal.
      Modal : (spec : ChooseSpec b) -> {n : Nat} -> (modes : Vect (S n) (Mode b)) -> {auto 0 cnt : modalCountOk spec (S n) = True} -> OneShotEffect b
      -- a VOTE ([CR#701.38]): starting with `starting` and proceeding in turn order ([CR#701.38a]), each player
      -- votes for one option on the `ballot`; the tally drives the effect. The two CR option kinds ([CR#701.38b])
      -- are the two `Ballot` arms (objects vs labeled outcomes) — Council's Judgment / Tyrant's Choice. The
      -- per-player vote and tally are engine-resolved; the grammar names the starting player + the ballot. Rust: none yet — unimplemented.
      Vote : {default You starting : Reference b APlayer} -> Ballot b -> OneShotEffect b
      -- DIVIDE AND CHOOSE (Fact or Fiction): the `divider` splits `group` into two piles; the `chooser` picks
      -- one as the "chosen" pile, the rest is "other". Each pile is a BOUND GROUP — a choice FRAME
      -- ([CR#700.3,608.2d]), so `chosen`/`other` read their pile as the plural anaphor (`They`, iterated with
      -- `Each (Existing They) …`); no bespoke pile slot. The split + pick are engine-resolved; the grammar
      -- names who does each and the per-pile fate. Rust: SeparatePiles + ChoosePile.
      DivideAndChoose : (group : Selection b AnObject) -> (divider : Reference b APlayer) -> {default You chooser : Reference b APlayer} -> (chosen : OneShotEffect (bindThat PileFrame b)) -> (other : OneShotEffect (bindThat PileFrame b)) -> OneShotEffect b
      -- "for each [domain], [body]" — binds each element as `It`. The distributive
      -- primitive (subsumes the old `Selection::Each`). Rust: OneShotEffect::Each.
      Each : (dom : Bindable b Many k) -> OneShotEffect (bindIt (binderAnte dom) b) -> OneShotEffect b
      -- "[amount] divided as you choose among [a group]" ([CR#601.2d]): bind each element as `It` with its
      -- `Allotment` (the split is engine-resolved, ≥1 each summing to amount), then apply `body`. GENERAL over
      -- the per-element effect — subsumes divided damage (`Act (DealDamage Allotment It)`) and divided
      -- counters (`Act (PutCounters c Allotment It)`); replaced the bespoke `DealDamageDivided`. (`amount`,
      -- not `total` — the latter is a reserved totality keyword.)
      Distribute : (amount : Count b) -> (among : Bindable b Many k) -> OneShotEffect (bindAllot (binderAnte among) b) -> OneShotEffect b
      -- "[body], [count] times": count-driven repetition. Slots with the quantifier family
      -- (`Each`/`Distribute`, which also leads with a `Count`) rather than the manner-adverb
      -- family (`Simultaneously`/`Continuously`), since it's a `Count` over ONE body, not a
      -- manner over a list. `body` elaborates in the SAME ctx `b` each iteration — no
      -- iteration-index binder; Storm/Replicate per-iteration semantics are engine-side
      -- (Storm rides `engine-copy-spells`). `Repeat (^2) (Act (Composite Proliferate …))` =
      -- proliferate twice.
      Repeat : (count : Count b) -> OneShotEffect b -> OneShotEffect b
      -- `Repeat`'s BATCHING twin ([CR#616.1g]): SHELL constructor — this stage resolves
      -- `body` `count` times identically to `Repeat` (purely sequential); a later pass
      -- rebuilds it into the true aggregate-count tier where "twice that many" replacements
      -- bite ([CR#121.2a,616.1g]) and aggregate triggers read. Rust: OneShotEffect::Batch.
      Batch : (count : Count b) -> OneShotEffect b -> OneShotEffect b
      -- "when you do [the preceding], [effect]" — a reflexive trigger. It NESTS, so
      -- `That`/targets stay in scope; no event-scanning sibling. Rust: OneShotEffect::Reflexive.
      Reflexive : OneShotEffect b -> OneShotEffect b
      -- schedule `body` for `event` ([CR#603.7c]): the announced TARGETS are dropped (stale after
      -- resolution), `Product` antecedents — with their expected-zone stamps — survive, and the
      -- delayed event's own caps/roles bind. Rust: OneShotEffect::Delayed.
      Delayed : (q : EventQuery b) -> OneShotEffect (bindEvent (eventQueryCaps q) (queryRoles q) (unbindTargets b)) -> OneShotEffect b
      -- the variable-length DIG-UNTIL ([CR#702.85] cascade, [CR#701.57] discover — the §13
      -- "mint when a consumer makes desugaring painful" case; the FIXED peek-N-and-sort shape
      -- (scry/surveil/fateseal/clash/explore/ripple/hideaway) stays watched-not-minted, expressed
      -- via `Move`+`TopOfLibrary`+`Modal`). `whose` reveals cards off the top of their library
      -- until one matches `match`; `body` elaborates at `bindFound match b` — the found card reads
      -- as `It`, the passed-over prefix as `They` ([CR#608.2]). No name-tag: nothing triggers on
      -- "cascade"/"discover" themselves, the keyword ABILITIES that wrap this name the trigger.
      -- Reveal-nothing (no match in the library) degrades at runtime, never a compile-time gate.
      RevealUntil : (whose : Reference b APlayer) -> (match : Predicate b AnObject) -> OneShotEffect (bindFound match b) -> OneShotEffect b

  -- the TELESCOPE ([CR#608.2d] — sentence order IS binder order; R3, no
  -- forward references, is structural: a clause reads only what stands to
  -- its left): each `Sequentially` cell elaborates in the context extended by
  -- the previous cell's introductions, per the total `intro` below.
  namespace SeqList
    public export
    data SeqList : Ctx -> Type where
      Nil : SeqList b
      (::) : (e : OneShotEffect b) -> SeqList (intro e b) -> SeqList b

  -- the `Chosen` antecedents a REFERENCE pushes for its right siblings
  -- ([CR#608.2d]). VESTIGIAL since the indefinite determiner `A` (the only
  -- source of a `Chosen` antecedent — a "choose one, then read it back"
  -- effect now goes through `With (ChooseOne …) … It`, whose antecedent
  -- comes from `binderAnte`, not here) was retired: every case now falls to
  -- the recursion/catch-all, so this is always `[]`. Kept as the hook for a
  -- reference that should introduce something for its siblings again.
  public export
  refIntro : {k : RefKind} -> Reference b k -> List Ant
  refIntro (ControllerOf r) = refIntro r
  -- Explicit structural recursion over the coalesced references (each `r` a
  -- subterm of `Coalesce rs`), NOT `concatMap refIntro rs`: the higher-order
  -- form hides the decreasing argument from the totality checker, which
  -- `%default total` (line 14) then rejects — and its non-totality cascades
  -- through `actionIntro`/`introduces`/`seqIntro`/`intro`, failing the whole
  -- `Core.idr` build and every re-emit typecheck.
  refIntro (Coalesce rs) = coalesceIntro rs
    where
      coalesceIntro : List (Reference b k) -> List Ant
      coalesceIntro [] = []
      coalesceIntro (r :: rrs) = refIntro r ++ coalesceIntro rrs
  refIntro (OwnerOf r) = refIntro r
  refIntro (AttachHostOf r) = refIntro r
  refIntro _ = []

  public export
  countCard : Count b -> Cardinality
  countCard (Literal 1) = One
  countCard _ = Many

  -- what one ACTION introduces for its right siblings — the intro rows:
  -- moved objects ([CR#400.7j]; noun + expected zone from the destination),
  -- created tokens ([CR#111.2]; arity from the count), the card-flow verbs
  -- (plus the "that many" amount companion), the bare amount verbs ("that
  -- much", [CR#608.2i]). TOTAL-BY-ENUMERATION: a verb that introduces
  -- nothing says so in its own arm.
  public export
  actionIntro : Action b -> List Ant
  actionIntro (Action.Move r d) = refIntro r ++ [MkAnt (destSort d) AnObject One Product (Just (destZone d)) Nothing]
  actionIntro (Action.MoveGroup g _ d) = [MkAnt (destSort d) AnObject Many Product (Just (destZone d)) Nothing]
  actionIntro (Action.Create _ n c) = [MkAnt Token AnObject (countCard n) Product (Just Battlefield) Nothing]
  actionIntro (Action.DrawCard _ n) = [MkAnt Card AnObject (countCard n) Product (Just Hand) Nothing, amountAnte]
  -- (Discard's card/amount anaphora is now derived from the Composite body's
  -- own coordinates, not a bare Action.Discard row — see the Composite arm.)
  -- (the DealDamage recipient's kind is an erased index — no `A` push there;
  -- damage recipients are targets or loop elements, never indefinites)
  actionIntro (Action.DealDamage _ n _) = [amountAnte]
  actionIntro (Action.ChangeLife _ (LifeOp.Set _)) = []
  actionIntro (Action.ChangeLife _ _) = [amountAnte]
  actionIntro (Action.Counter r) = refIntro r
  actionIntro (Action.Tap r) = refIntro r
  actionIntro (Action.Untap r) = refIntro r
  actionIntro (Action.RemoveDamage r) = refIntro r
  actionIntro (Action.Transform r) = refIntro r
  actionIntro (Action.Attach w t) = refIntro w ++ refIntro t
  actionIntro (Action.Unattach r) = refIntro r
  actionIntro (Action.Reveal r) = refIntro r
  actionIntro (Action.Copy r _) = refIntro r
  actionIntro (Action.Retarget _ o _) = refIntro o
  actionIntro (Action.PutCounters _ _ r) = refIntro r
  actionIntro (Action.RemoveCounters _ _ r) = refIntro r
  actionIntro (Action.MoveCounters _ f t) = refIntro f ++ refIntro t
  actionIntro (Action.Sacrifice _ _) = []
  actionIntro (Action.Shuffle _) = []
  actionIntro (Action.ExtraTurn _) = []
  actionIntro (Action.ControlPlayer r) = refIntro r
  actionIntro (Action.AddMana _ _ _) = []
  actionIntro (Action.Composite _ _) = []   -- composite tags introduce nothing (no intro row)
  actionIntro (Action.RollDice _ _ _) = [amountAnte]
  actionIntro (Action.FlipCoins _ _ _) = [amountAnte]
  actionIntro (Action.RollPlanarDie _) = []
  -- the payment's own event-role antecedents ([CR#118.12a]) — dead for the
  -- `May (Act (Pay cost))` path (`intro`'s own `Pay` arm below wins there via
  -- `bindEvent`, which ALSO wipes stale event-role antecedents); kept for
  -- totality and any other right-sibling context reaching a bare `Pay`.
  actionIntro (Action.Pay cost) = costRoles (costCaps cost)

  -- the antecedents an effect INTRODUCES for its right siblings
  -- ([CR#608.2d]): a clause's products, a `May`'s inner introductions, a
  -- `With`/`Targeted` body's escape. Loop bodies (`Each`/`Distribute`)
  -- introduce nothing — their elements pop with the body.
  public export
  introduces : OneShotEffect b -> List Ant
  introduces (Act a) = actionIntro a
  introduces (Sequentially es) = seqIntro es
  introduces (Targeted ts e) = introduces e
  introduces (With that e) = introduces e
  introduces (May e) = introduces e
  introduces _ = []

  public export
  seqIntro : SeqList b -> List Ant
  seqIntro [] = []
  seqIntro (e :: es) = introduces e ++ seqIntro es

  -- the telescope STEP: the context a clause's right sibling elaborates in.
  -- [CR#118.12a]: `May (Act (Pay cost))`'s "if you do" branch reads the
  -- PAYMENT like an `AdditionalCost` body — `bindEvent` (not the generic
  -- `pushAntes`) so it ALSO wipes stale event-role antecedents ("filter
  -- notEventRoleA" hygiene) before the payment's own roles land, exactly
  -- like `AdditionalCost`'s body typing (`bindEvent (costCaps pay) (costRoles
  -- (costCaps pay))`). The ONE special arm; every other effect keeps the
  -- generic antecedent push.
  public export
  intro : OneShotEffect b -> Ctx -> Ctx
  intro (Act (Pay cost)) = bindEvent (costCaps cost) (costRoles (costCaps cost))
  intro e = pushAntes (introduces e)

  -- one option of a modal effect: an effect plus an optional extra cost. Rust: Mode.
  namespace Mode
    public export
    data Mode : Ctx -> Type where
      MkMode : (effect : OneShotEffect b) -> {default Nothing cost : Maybe (Cost b)} -> Mode b

  -- one labeled option of an OUTCOME vote ([CR#701.38b]): a `word` with no rules meaning (cosmetic — it appears
  -- on the card and players say it aloud) paired with the `effect` that runs if it wins. Its mechanical identity
  -- is its POSITION in the `Outcomes` list (what `tiebreak` indexes); `word` is for rendering/communication.
  namespace VoteOption
    public export
    data VoteOption : Ctx -> Type where
      MkVoteOption : (word : String) -> (effect : OneShotEffect b) -> VoteOption b

  -- a vote's BALLOT — the two CR option kinds ([CR#701.38b]). `OverMatching`: each player votes for one object
  -- matching the predicate; the engine tallies and binds each WINNER (most votes — ties all included) as `It`,
  -- running `perWinner` once per winner (Council's Judgment = `OverMatching <nonland perm you don't control>
  -- (Act (Move It (ToZone Exile)))`). `Outcomes`: a fixed nonempty list of labeled outcomes; the winning option's
  -- effect runs, with `tiebreak` (an index into `options`, default the first) naming who wins a tie (Tyrant's
  -- Choice = torture). An out-of-range `tiebreak` is a senseless no-op (grammar-layer concern, not gated).
  namespace Ballot
    public export
    data Ballot : Ctx -> Type where
      OverMatching : (p : Predicate b AnObject) -> OneShotEffect (bindIt (loopOf p) b) -> Ballot b
      Outcomes : (options : List (VoteOption b)) -> {auto 0 ne : NonEmpty options} -> {default 0 tiebreak : Nat} -> Ballot b

  -- a continuous modification to a PLAYER attribute — the player-side twin of `ModificationOp`. `Raise`/`Lower`/
  -- `SetTo` adjust a count-valued attr (Exploration = `Raise LandPlaysPerTurn (^1)`); `NoMax` removes a cap
  -- (Reliquary Tower = `NoMax HandSizeLimit`, the "no maximum hand size" case — kept an op, not a `Maybe Count`
  -- value, since `PlayerStatOf` is a `Count` reader). Carried by `ModifyPlayer`.
  namespace PlayerMod
    public export
    data PlayerMod : Ctx -> Type where
      SetTo : PlayerAttr -> Count b -> PlayerMod b
      Raise : PlayerAttr -> Count b -> PlayerMod b
      Lower : PlayerAttr -> Count b -> PlayerMod b
      NoMax : PlayerAttr -> PlayerMod b

  -- A continuous modification a static ability applies to its subject.
  namespace Modification
    public export
    data Modification : Ctx -> Type where
      -- modify one CHARACTERISTIC axis with a (gated) `ModificationOp` ([CR#613]). The operation carries the
      -- layer/base-vs-current: "gets +2/+1" = `Alter Power (Up (^2))` + `Alter Toughness (Up (^1))`; "becomes
      -- blue" = `Alter Colors (Set [Blue])`; "is also an artifact" = `Alter Types (Add Artifact)`; "becomes
      -- an Island" = `Alter Subtypes (Add (^Island))`; "loses all creature types" = `Alter Subtypes (Set [])`;
      -- "base p/t are x/y" = `Alter Power (Set x)` + `Alter Toughness (Set y)` (a CDA `*/*` sets a dynamic
      -- `Count`). ONE mechanism; subsumes the former `ModifyPT`/`Set`/`AddType`/`AddSubtype`.
      Alter : (c : Characteristic) -> ModificationOp b c -> Modification b
      -- SEVERAL modifications as one ([CR#613] — "+2/+1 and gains flying"). The plural wrapper that lets
      -- `Modify` take a SINGLE `Modification`: a bare `List` never floats as an argument — it's always named
      -- (`ApplyAll`, like `Sequentially`/`And`/`Or`). `Modify This (ApplyAll [Alter Power (Up …), GrantAbility …])`.
      ApplyAll : List (Modification b) -> Modification b
      -- TEXT-CHANGE ([CR#612], a layer-3 mod): "replace all instances of one word with another of its
      -- class" — the eligible classes are listed; the two specific words are the player's resolution-time
      -- choice (engine-resolved, like `Choose`). Mind Bend = `ChangeText [ColorWords, BasicLandTypes]`.
      ChangeText : List TextWordClass -> Modification b
      LoseAbilities : Modification b                      -- "loses all abilities" (Humility-style)
      LoseKeyword : KeywordSpec b -> Modification b       -- SELECTIVE removal — "loses flying" (vs `LoseAbilities`' all-or-nothing)
      GainControl : Reference b APlayer -> Modification b         -- "[player] gains control"
      GrantAbility : Ability b -> Modification b
      -- gain ALL of another object's abilities at runtime (the set is resolved from `r`, not statically listed) —
      -- Necrotic Ooze (abilities of creature cards in graveyards), Conspicuous Snoop (top card of your library).
      InheritAbilities : Reference b AnObject -> Modification b
      -- "becomes a COPY of [r]" ([CR#707.2], layer 1 — copiable values). Alterations ("a copy, except it's
      -- a 4/4") are SEPARATE higher-layer mods (Continuously/Modify on the result), not bundled here.
      BecomeCopyOf : Reference b AnObject -> Modification b

  -- which flip(s)/roll(s) a `ReplaceRoll` discards, once the extras have been rolled
  -- ([CR#706.6] "if a player is instructed to ignore a roll... the player chooses one of those
  -- rolls to be ignored" when multiple tie for lowest). `IgnoreLowest` = automatically drop the
  -- lowest result, no choice (a "roll two, keep the higher" die effect); `IgnoreChosen n` = the
  -- player picks which n of the extras to discard (Krark's Thumb = flip two, ignore one of the
  -- player's choice, i.e. `IgnoreChosen 1` — matching the `ReplaceRoll` doc below).
  namespace IgnoreRule
    public export
    data IgnoreRule = IgnoreLowest | IgnoreChosen Nat

  -- A continuous effect a static (or `Continuously`) ability generates ([CR#611]):
  -- modify a subject, modify a whole filter (anthem), or REPLACE an event — a
  -- replacement effect is a continuous effect too ([CR#614]). Rust: the StaticEffect family.
  namespace StaticEffect
    public export
    data StaticEffect : Ctx -> Type where
      -- continuous modification of ONE subject ([CR#613]): "this gets +1/+1 and gains flying" =
      -- `Modify This [Alter Power (Up …), Alter Toughness (Up …), GrantAbility …]`. The subject is a SINGULAR
      -- `Reference` (`This`, a target, or `It` when iterated). Plurality is lifted OUT to `Each` (below) — there
      -- is no `Selection` here, and exactly ONE `Modification` (use `ApplyAll […]` for more than one). A
      -- per-subject mod reads the subject as `It` under the `Each` (Coat of Arms =
      -- `Modify It (ApplyAll [Alter Power (Up (CountOf (And [creature, SharesChar Subtypes It, Not (SameAs It)])))])`).
      Modify : Reference b AnObject -> Modification b -> StaticEffect b
      -- the PLAYER-side `Modify`: a continuous modification to a player's attribute (Exploration's extra land
      -- play, Reliquary Tower's no-maximum-hand-size). The player twin of `Modify`.
      ModifyPlayer : Reference b APlayer -> PlayerMod b -> StaticEffect b
      -- iterate a `Selection`, binding each element as `It`, applying a static effect to each — the STATIC twin
      -- of the one-shot `Each` ([CR#611]). Anthem = `Each (SelectAll (And [creature, ControlledBy you])) (Modify
      -- It […])`; fixed set = `Each (Union …) …`. Because a `StaticEffect` is re-evaluated each layer pass, the
      -- selection re-gathers continuously (a LIVE filter — creatures get the buff as they enter), unlike one-shot
      -- `Each` which fixes the set once. Shares the bare name `Each` with `OneShotEffect.Each` (its own namespace,
      -- disambiguated by type). Subsumes the former `Modify`-over-`Selection`/`ModifyAll` split.
      Each : (dom : Bindable b Many k) -> StaticEffect (bindIt (binderAnte dom) b) -> StaticEffect b
      -- continuous COST modification ([CR#118.7]): spells/abilities matching `of_` get the `change`.
      -- "Instant/sorcery spells you cast cost {1} less" = `CostModifier (And […, ControlledBy you]) (Reduce
      -- [^1])`; affinity is a SELF modifier `CostModifier (SameAs This) (ScaledBy (Reduce …) (CountOf …))`.
      CostModifier : Predicate b AnObject -> CostChange b -> StaticEffect b
      -- a DECLARED OPTIONAL COST on this object's own casting ([CR#118.8b,601.2b]) — the kicker/
      -- multikicker/buyback identity: one tag, read back by `PaidCost`/`TimesPaid`/`WasPaidWith`
      -- ([CR#702.33d..702.33e,607]). `repeatable` = multikicker's "any number of times"
      -- ([CR#702.33c]). Replaces `CostChange.Optional` (the untagged, unreadable flag).
      -- Rust: StaticEffect::CostOption(OptionalCost{components, tag, repeatable}).
      CostOption : (tag : String) -> List (Cost b) -> {default False repeatable : Bool} -> StaticEffect b
      -- ALTERNATIVE PAYMENT of individual cost pips ([CR#601.2] — NOT mana production, NOT a `CostChange` reduction):
      -- "for each [pips] of THIS spell's total cost, you may [PayAct] rather than pay that mana." A static that
      -- functions while this is being cast ([CR#702.51a] convoke / [CR#702.66a] delve / [CR#702.126a] improvise; Avatar's
      -- waterbend bundles it into a named cost). The engine applies it at payment ([CR#601.2g..601.2h]); nothing enters the pool.
      PayPips : PipClass -> PayAct b -> StaticEffect b
      -- "if [event] would happen, do [effect] INSTEAD" — a replacement ([CR#614]). Empty body = a SKIP
      -- (a replacement that removes the event — e.g. "skip your draw step"). This is NOT a prohibition:
      -- the event still "would happen" and is intercepted; for "can't happen", use `CantHappen` below.
      Replaces : (q : EventQuery b) -> OneShotEffect (bindEvent (eventQueryCaps q) (queryRoles q) b) -> {default Unlimited limit : ReplaceLimit b} -> StaticEffect b
      -- "[event] CAN'T happen" — a continuous PROHIBITION, semantically distinct from replacing-with-
      -- nothing: it's not a one-shot ([CR#614.5]) application, isn't ordered against other replacements
      -- ([CR#616]), and the event never "would happen". Indestructible = `CantHappen (MkEventQuery [Destroy]
      -- [this])`; Solemnity = `CantHappen (MkEventQuery [PutCounters] [])`. (Event-level; the deontic `cant` is
      -- its player-ACTION sibling — "can't attack".)
      CantHappen : EventQuery b -> StaticEffect b
      -- PAYLOAD replacement ([CR#616]): the event still happens, but its numeric amount becomes
      -- `newAmount` (a `Count` over the event body, so it can read `EventAmount`). Furnace of Rath =
      -- `ReplaceAmount (MkEventQuery [DealDamage Nothing] []) (Times EventAmount (^2))`. Takes the same `EventQuery` as
      -- `Replaces`/`Also`, gated so every queried kind carries an amount — `ReplaceAmount (MkEventQuery [Begins Cast] []) …`
      -- (a cast has no amount) is a TYPE ERROR; the query's facets add the non-kind conditions.
      ReplaceAmount : (q : EventQuery b) -> {auto 0 amt : eventQueryHasAmount q = True} -> (newAmount : Count (bindEvent (eventQueryCaps q) (queryRoles q) b)) -> StaticEffect b
      -- ROLL-MORE replacement ([CR#614.3] Krark's Thumb-family; [CR#706.6] the ignore-result
      -- semantics): "if you would roll/flip [q], instead roll/flip `extra` more and ignore
      -- [ignore]". Gated to `isRandomnessQuery` — a `RollDice`/`FlipCoin` query only, NEVER the
      -- planar die (it has no roll-more interaction, [CR#901.9d]) — so `ReplaceRoll` on a
      -- `RollPlanarDie` query is a TYPE ERROR, not a runtime no-op. Krark's Thumb = `ReplaceRoll
      -- (MkEventQuery [FlipCoin Nothing] [Actor you]) (^1) (IgnoreChosen 1)`.
      ReplaceRoll : (q : EventQuery b) -> {auto 0 rnd : isRandomnessQuery q = True} -> (extra : Count b) -> (ignore : IgnoreRule) -> StaticEffect b
      -- a static OUTCOME suppressor: the matching players can't lose / can't win ([CR#104.2b,104.3e]). Platinum
      -- Angel = `OutcomeGate CantLose you` + `OutcomeGate CantWin opponent`. (Distinct from `CantHappen` —
      -- game-loss isn't a replaceable event — and from a deontic `cant` — it's not a player action.)
      OutcomeGate : OutcomeGateKind -> Predicate b APlayer -> StaticEffect b
      -- ADDITIVE replacement ([CR#614.13] "as well as"): when [event] happens it STILL happens, but
      -- [effect] also runs. An Aura enters attached via `Also thisEnters (Act (Attach This host))`.
      Also : (q : EventQuery b) -> OneShotEffect (bindEvent (eventQueryCaps q) (queryRoles q) b) -> StaticEffect b
      -- TRIGGER MULTIPLICATION ([CR#603.2d]): a matching triggered ability triggers `extra` ADDITIONAL times.
      -- NOT a copy ([CR#707.10]) — each instance chooses its OWN modes/targets, and multipliers ADD rather
      -- than compound (two Panharmonicons → 3×, not 4×). `cause` is the EVENT whose resulting triggers are
      -- multiplied (an artifact/creature ETB); `affected` filters the affected ability's SOURCE permanent,
      -- defaulting to "you control" (Panharmonicon/Teysa) — override for any/opponent doublers. Panharmonicon
      -- = `TriggerMultiplier (MkEventQuery [ZoneChanged Nothing (Just Battlefield)] [Agent (Or [artifact, creature])]) (^1)`.
      TriggerMultiplier : (cause : EventQuery b) -> (extra : Count b) -> {default (ControlledBy (SameAs You)) affected : Predicate b AnObject} -> StaticEffect b
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
      -- "you may cast THIS for [costs] from [from]" ([CR#118.9]) — the alternative-cost permission (a base SWAP,
      -- distinct from `CostModifier`'s base modify; that distinction is carried HERE, by the consumer, so the
      -- cost list needs no `AltCost` wrapper). `costs` is the swapped-in cost ([] = "without paying its mana
      -- cost"). `from` defaults to Hand; a non-default zone is the cast-from-zone family ([CR#702.34] flashback =
      -- `{from = [Graveyard]}`; escape/jump-start add a rider). Force of Will = `MayCastFor [Do (ChangeLife You (Down (^1))), …]`.
      -- `tag` (default `Nothing`) NAMES the alt cost so a rider can ask `WasCastWith tag` ("if its dash cost was
      -- paid", Dash/Evoke/Blitz); `when` (default `Nothing`) is an AVAILABILITY guard ([CR#702.76a],[CR#702.137a])
      -- — the permission only exists if the condition holds (Prowl = combat damage dealt; Spectacle = an opponent
      -- lost life this turn), unlike an unconditional alt cost.
      MayCastFor : List (Cost b) -> {default [Hand] from : List Zone} -> {default Nothing tag : Maybe (KeywordSpec b)} -> {default Nothing when : Maybe (Condition b)} -> StaticEffect b
      -- "you may cast THIS face down for [cost]" ([CR#702.37]) — an alternative cast that ALSO turns the
      -- object face down; the engine then applies the global [CR#708.2] 2/2-colorless-vanilla override.
      CastFaceDown : Cost b -> StaticEffect b
      -- a duration-bounded ZONE CHANGE — "exile UNTIL ~", the modern `Banishing Light` form. Under an enclosing
      -- duration (`Continuously (UntilEvent …)` / `While …`), `r` moves `from`→`to`; when that duration ends it
      -- returns `from`. BOTH edges are real `ZoneChanged` events, so the returned object is a NEW object
      -- ([CR#400.7] — counters vanish, Auras fall off, tokens don't return; matches the official rulings). This is
      -- NOT layer-style zone "membership" (which would preserve identity) and NOT a delayed trigger.
      Relocate : Reference b AnObject -> (from : Zone) -> (to : Zone) -> StaticEffect b
      -- the inner continuous effect applies only WHILE the condition holds ([CR#604.3]) — a conditional static
      -- ("gets +1/+1 as long as …"), RE-EVALUATED continuously (toggles with the condition). DISTINCT from the
      -- `Duration.ForAsLongAs` of a resolution-created effect, whose affected set is FIXED at the start
      -- ([CR#611.2c] "works differently than a continuous effect from a static ability") — NOT a redundancy.
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
      Priced : PricedTiming -> Cost b -> Deed b -> StaticEffect b

  -- A keyword as it sits on a permanent ([CR#702]): either `Bare` — an engine-PRIMITIVE keyword
  -- the grammar can't desugar (FirstStrike/DoubleStrike/Deathtouch/Trample = damage pipeline;
  -- Vigilance = attack event-edit) — or a `Composite` of its tag + the `Ability`s it desugars to:
  -- Flying/Defender/Shroud/Hexproof/Menace → a `cant` (Menace's is SET-level, `BlockedBy`); Reach → `[]` (a flag flying's clause reads, no
  -- ability of its own); Flash → a `Can (Casts …) {window = AsInstant}` (cast at instant speed).
  -- `Keyword` wraps it; `keyword` (Macros) builds it.
  namespace KeywordAbility
    public export
    data KeywordAbility : Ctx -> Type where
      Bare      : KeywordSpec b -> KeywordAbility b
      Composite : KeywordSpec b -> List (Ability b) -> KeywordAbility b

  -- An ability, INDEXED by its context `b`. A card's top-level abilities are `Ability Base`
  -- (source bound, no targets); a keyword desugaring can be `Ability b` so its clause may
  -- reference an anaphor — "protection from the CHOSEN color/player" (Mother of Runes).
  namespace Ability
    public export
    data Ability : Ctx -> Type where
      Spell : OneShotEffect b -> Ability b
      Keyword : KeywordAbility b -> Ability b
      -- "{cost}: {effect}" — an activated ability ([CR#602]). `window` is its activation timing
      -- (instant by default; `AsSorcery` = "activate only as a sorcery"); `limits` are the
      -- use-frequency caps. A loyalty ability is `{window = AsSorcery, limits = [LoyaltyOncePerTurn]}`
      -- ([CR#606.3,306.5d] — shared across every loyalty ability of the permanent, not per-ability).
      -- `from` = the zone(s) the ability FUNCTIONS in ([CR#113.6b], default `[Battlefield]`): Cycling/
      -- Channel/Forecast from `[Hand]`, Embalm/Unearth from `[Graveyard]` (mirrors engine `ActivatedAbility.from`).
      -- `activationGuard` = an extra LEGALITY condition ([CR#602.5,702.142a], "activate only if …" — Boast's
      -- "attacked this turn", Metalcraft, "if you control 3+ Caves"). It gates ACTIVATABILITY, unlike an `If` in
      -- the body (which leaves the cost payable then fizzles); default `Nothing`.
      Activated : Cost b -> OneShotEffect b -> {default AsInstant window : Timing} -> {default [] limits : List UsageLimit} -> {default [Battlefield] from : List Zone} -> {default Nothing activationGuard : Maybe (Condition b)} -> Ability b
      -- a triggered ability: when `event` fires, resolve `effect`. `limits` are use-frequency caps —
      -- "triggers only once each turn" = `{limits = [OncePerTurn]}` — the same `UsageLimit` list
      -- `Activated` carries. `from` = the zone(s) it functions in ([CR#113.6b], default `[Battlefield]`): a
      -- graveyard/hand trigger (Madness, "while in your graveyard") sets `[Graveyard]`/`[Hand]`. Rust: Ability::Triggered.
      Triggered : (q : EventQuery b) -> OneShotEffect (bindEvent (eventQueryCaps q) (queryRoles q) b) -> {default [] limits : List UsageLimit} -> {default [Battlefield] from : List Zone} -> Ability b
      -- a TURN-BASED action ([CR#703]) intrinsic to the bearer: at `phase`, perform `effect` automatically —
      -- no stack, unlike `Triggered`. The Saga lore-increment ([CR#714.3c], conferred by the Saga subtype):
      -- `TurnBased (MainPhase PreCombat) (Act (PutCounters loreCounter (^1) This))`. (Was the old `PropTurnBased`.)
      TurnBased : PhaseStep -> OneShotEffect b -> Ability b
      -- (Retired `Enchant`: the engine has no dedicated aura ability — "enchant X" is a `Can (Enact Attach …)`
      --  PERMISSION (attaching is default-forbidden, so the aura ENABLES it), enters-attached an `Also`,
      --  falls-off an `Sba`. No subtype special-casing.)
      -- a static continuous ability — modifications, anthems, AND replacements live in `StaticEffect`.
      -- `from` = the zone(s) the ability FUNCTIONS in ([CR#113.6b], default `[Battlefield]`): a graveyard/hand
      -- static (Riftstone Portal's land-mana from the graveyard, Satoru hand-ninjutsu enablers) sets
      -- `[Graveyard]`/`[Hand]`. Unifies with `from` on `Activated`/`Triggered` — function-zone is ability-level.
      Static : StaticEffect b -> {default [Battlefield] from : List Zone} -> Ability b
      -- "[cost]: turn This face up" ([CR#708.9]) — a SPECIAL action (not stack-using), not an `Activated`
      -- ability. Pays [cost], removes `FaceDown`. The face-up cost of `morph`/`disguise`. `onTurnUp` is an
      -- optional effect that fires AS it's turned face up "this way" ([CR#702.37b]) — Megamorph's "+1/+1
      -- counter if its megamorph cost was paid" rides the face-up ability, not a separate trigger.
      TurnFaceUp : Cost b -> {default Nothing onTurnUp : Maybe (OneShotEffect b)} -> Ability b
      -- "As ~ enters, choose a [d]" ([CR#614.12]) for a VALUE choice: a single ability that makes the
      -- as-enters choice and SCOPES it to the abilities that read it — those nest at `bindChosen d b` (so
      -- `OfChosen`/`ChosenIs` resolve), while the card's other abilities (and its whole printed face) stay
      -- at `b`, untouched. `d` is value-only now; a chosen ENTITY goes through `AsEntersChoosing`.
      AsEnters : (d : ChooseDomain) -> {auto 0 ok : ModeDomainOk d} -> List (Ability (bindChosen d b)) -> Ability b
      -- "As ~ enters, choose a [filtered ENTITY]" ([CR#614.12]) — the game-entity twin of `AsEnters`. The
      -- `Predicate b k` is the choosable set (Clone's "a creature", lost when this was the unconstrained
      -- `AnObjectChoice`); `k` is gated to object/player by `ChoiceRefKindOk`. The chosen entity binds
      -- `chosenRefKind k`, read back by `ChosenObject`/`ChosenPlayer` in the nested abilities.
      AsEntersChoosing : (k : RefKind) -> {auto 0 ok : ChoiceRefKindOk k} -> Predicate b k -> List (Ability (bindChosenRef k b)) -> Ability b

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
DefaultValue (Characteristics b) where
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

-- the (total) card-type category of a subtype ([CR#205.3g..205.3q]) — reads the owning-category field.
public export
subtypeCategory : Subtype -> Category
subtypeCategory (MkSubtype cat _ _) = cat

-- [CR#205.3d]: a subtype's governing card type must be among the card's types.
-- A category admits a GROUP of types (`categoryTypes` — creature types on
-- Creature or Kindred, spell types on Instant or Sorcery), so the card need
-- only carry AT LEAST ONE of them. The proof is demanded at `Normal`, so
-- `types`/`subtypes` stay plain fields the `^ { … := … }` builder can still set.
public export
SubtypesOk : Characteristics Base -> Type
SubtypesOk c = All (\s => Any (\t => Elem t (categoryTypes (subtypeCategory s))) (types c)) (subtypes c)

-- How a multi-faced card's two faces are arranged ([CR#712] transforming/modal DFC, [CR#709] split,
-- [CR#715] adventurer, [CR#710] flip). The LAYOUT carries the access rules; both faces are full faces.
namespace FaceLayout
  public export
  data FaceLayout = Transforming | ModalDFC | Split | Adventure | Flip

namespace Card
  public export
  data Card : Type where
    Normal : (c : Characteristics Base) -> {auto 0 ok : SubtypesOk c} -> {auto 0 wf : CharacteristicsOk c} -> Card
    -- a TWO-faced card: `front` (the primary/default face) and `back`, arranged per `layout`. Each face
    -- is a full `Face` with its own well-formedness ([CR#712.8] each face has its own characteristics);
    -- transform / cast-the-other-face is the engine's job — the grammar just holds both faces.
    TwoFaced : (layout : FaceLayout) -> (front : Face) -> (back : Face) ->
               {auto 0 okF : SubtypesOk front} -> {auto 0 wfF : CharacteristicsOk front} ->
               {auto 0 okB : SubtypesOk back} -> {auto 0 wfB : CharacteristicsOk back} -> Card

-- What a counter / subtype / type CONFERS on its bearer is just a list of `Ability`s — the engine's
-- mechanism for intrinsic behavior with NO subtype special-casing (`This` = the bearer). There is no
-- `Property` wrapper: a conferral IS an ability (`Static (Modify …)` for a continuous self-mod, `Static
-- (Sba …)` for an SBA, `TurnBased …` for a turn-based action, a plain `Static`/keyword otherwise).
-- Open + name-keyed like `Subtype`; conferrals ride the value's third field, projected below.

-- what a COUNTER KIND confers on its bearer — just its conferral field (parallel to `subtypeConfers`).
-- +1/+1 and −1/−1 carry their OWN P/T pump (a `Static (Modify …)`), baked into the `p1p1`/`m1m1` helpers.
public export
counterConfers : CounterKind -> List (Ability Base)
counterConfers (MkCounterKind _ _ cs) = cs

-- Terse category constructors for the common (non-conferring) subtypes: `creatureType "Bear"`,
-- `landType "Island"`, etc. build an open subtype name owned by the given category, conferring
-- nothing. Named `*Type` (not bare `creature`/`land`) so they never collide with the `hasType`
-- filter helpers (`creature : Predicate …`).
public export
creatureType : String -> Subtype
creatureType n = MkSubtype Creature n []
public export
enchantmentType : String -> Subtype
enchantmentType n = MkSubtype Enchantment n []
public export
artifactType : String -> Subtype
artifactType n = MkSubtype Artifact n []
public export
landType : String -> Subtype
landType n = MkSubtype Land n []
public export
battleType : String -> Subtype
battleType n = MkSubtype Battle n []
public export
planeswalkerType : String -> Subtype
planeswalkerType n = MkSubtype Planeswalker n []
public export
spellType : String -> Subtype
spellType n = MkSubtype Spell n []

-- the conferring subtypes, with their intrinsic rules baked into the conferral field. The Aura
-- falls-off SBA ([CR#704.5m], a `Static (Sba …)`) lives here — a shared rule, not a per-card static, and
-- never a subtype `if`-branch. (Its sibling `saga` is defined below, after the counter helpers it needs.)
public export
aura : Subtype
aura = MkSubtype Enchantment "Aura" [Static (Sba (Not (LegallyAttached This)) (Act (Move This (ToZone Graveyard))))]
-- Hand-authoring helper VALUES for the curated counter kinds / designations (parallel to the subtype
-- `aura`/`saga` helpers) — one per member the closed enums used to carry, so every hand-authored
-- reference names a helper, never a bare constructor. The `String` is the RON registry name; the `Scope`
-- is the carrier. Confers is `[]` except `p1p1`/`m1m1`, which bake their P/T pump (the ex-`counterConfers`
-- approximation, self-reference avoided via an inline literal — correcting it to the real
-- `Continuous`/`StateBased` flavor belongs to `idris-sba-not-a-static-ability`).
public export
loyaltyCounter : CounterKind
loyaltyCounter = MkCounterKind Object "LoyaltyCounter" []
public export
fateCounter : CounterKind
fateCounter = MkCounterKind Object "FateCounter" []
public export
chargeCounter : CounterKind
chargeCounter = MkCounterKind Object "ChargeCounter" []
public export
levelCounter : CounterKind
levelCounter = MkCounterKind Object "LevelCounter" []
public export
loreCounter : CounterKind
loreCounter = MkCounterKind Object "LoreCounter" []
public export
stunCounter : CounterKind
stunCounter = MkCounterKind Object "StunCounter" []
public export
shieldCounter : CounterKind
shieldCounter = MkCounterKind Object "ShieldCounter" []
public export
poison : CounterKind
poison = MkCounterKind Player "Poison" []
public export
energy : CounterKind
energy = MkCounterKind Player "Energy" []
public export
experience : CounterKind
experience = MkCounterKind Player "Experience" []
public export
p1p1 : CounterKind
p1p1 = MkCounterKind Object "P1P1Counter"
         [Static (Modify This (ApplyAll
            [ Alter Power     (Up (CountersOn (MkCounterKind Object "P1P1Counter" []) This))
            , Alter Toughness (Up (CountersOn (MkCounterKind Object "P1P1Counter" []) This)) ]))]
public export
m1m1 : CounterKind
m1m1 = MkCounterKind Object "M1M1Counter"
         [Static (Modify This (ApplyAll
            [ Alter Power     (Down (CountersOn (MkCounterKind Object "M1M1Counter" []) This))
            , Alter Toughness (Down (CountersOn (MkCounterKind Object "M1M1Counter" []) This)) ]))]
public export
monarch : Designation
monarch = MkDesignation Player "Monarch" []
public export
theInitiative : Designation
theInitiative = MkDesignation Player "TheInitiative" []
public export
citysBlessing : Designation
citysBlessing = MkDesignation Player "CitysBlessing" []
public export
monstrous : Designation
monstrous = MkDesignation Object "Monstrous" []
public export
goaded : Designation
goaded = MkDesignation Object "Goaded" []
public export
renowned : Designation
renowned = MkDesignation Object "Renowned" []
public export
suspected : Designation
suspected = MkDesignation Object "Suspected" []
public export
saddled : Designation
saddled = MkDesignation Object "Saddled" []
public export
solved : Designation
solved = MkDesignation Object "Solved" []

-- the Saga subtype (the other conferring subtype besides `aura` above): its lore-increment ([CR#714.3c],
-- a `TurnBased` action) is baked into its conferral field — a shared rule, never a subtype `if`-branch.
-- Defined here (after the counter helpers) because it references the `loreCounter` helper.
public export
saga : Subtype
saga = MkSubtype Enchantment "Saga" [TurnBased (MainPhase PreCombat) (Act (PutCounters loreCounter (^1) This))]

-- what a SUBTYPE confers on its bearer — just its conferral field.
public export
subtypeConfers : Subtype -> List (Ability Base)
subtypeConfers (MkSubtype _ _ cs) = cs

-- what a card TYPE confers on its bearer (parallel to `subtypeConfers`). A Planeswalker or Battle CREATES
-- a deontic permitting creatures to attack IT ([CR#508.1] — attackability is a granted permission, not a
-- hardcoded target list), using `Enact Attack` with the permanent itself as the object (patient) defender.
public export
typeConfers : Type_ -> List (Ability b)
typeConfers Planeswalker = [Static (Can (Enact Attack (HasChar Types Creature) (SameAs This)))]
typeConfers Battle       = [Static (Can (Enact Attack (HasChar Types Creature) (SameAs This)))]
typeConfers _            = []

-- ===========================================================================
-- EMITTER-TARGET POSITIONAL HELPERS. The RON→Idris re-emit gate
-- (`deckmaste_plugin::idris_emit`) authors card expressions mechanically and
-- MUST NOT write curly-brace named-argument syntax (`{field = val}`). The
-- constructors below carry ergonomic `{default …}` params for HAND authoring
-- (Cards/Macros/Spec keep using them terse, untouched); these thin aliases
-- expose each emitter-overridden param POSITIONALLY so the generated
-- expression is a plain fully-parenthesized application. Purely additive —
-- no existing signature or default changes.
-- ===========================================================================

public export
topFrom : Count b -> Reference b APlayer -> Selection b AnObject
topFrom c w = TopOfLibrary c {whose = w}

public export
bottomFrom : Count b -> Reference b APlayer -> Selection b AnObject
bottomFrom c w = BottomOfLibrary c {whose = w}

public export
chooseOneBy : Reference b APlayer -> Predicate b k -> Bindable b One k
chooseOneBy by p = ChooseOne {by} p

public export
chooseBy : Reference b APlayer -> Quantity b -> Predicate b k -> Bindable b Many k
chooseBy by q p = Choose {by} q p

public export
moveAttacking : Reference b AnObject -> (d : Destination b) -> {auto 0 dOk : DestinationOk d}
             -> Maybe (Reference b APlayer) -> Action b
moveAttacking r d ea = Move r d {enteringAttacking = ea}

-- `dealDamageFrom`/`drawBy`/`gainLifeBy`/`discardBy`/`loseLifeBy`/`setLifeToBy`/`sacrificeBy`/
-- `shuffleBy` are GONE: they existed only to expose an emitter-overridden actor POSITIONALLY
-- past a `{default You actor}` implicit — now that the action-role-reshape merge (2026-08-01)
-- strips every role default (Law 2), the raw constructors (`DealDamage`/`DrawCard`/`ChangeLife`/
-- `Sacrifice`/`Shuffle`) already take that argument positionally, so the emitter calls them
-- directly. `discardBy` in particular has no raw constructor left to alias — the bare `Discard`
-- verb is gone; discard is always the `Composite (Discard by n) body` shape.

public export
addManaFull : Reference b APlayer -> Count b -> ProducedMana b -> List (ManaRider b) -> Action b
addManaFull a amt pm rs = AddMana a amt pm {riders = rs}

public export
createTokenAttacking : Reference b APlayer -> Count b -> (c : Characteristics b) -> {auto 0 wf : CharacteristicsOk c}
                    -> Maybe (Reference b APlayer) -> Action b
createTokenAttacking a n c ea = Create a n c {enteringAttacking = ea}

public export
costOptionRep : String -> List (Cost b) -> Bool -> StaticEffect b
costOptionRep t cs r = CostOption t cs {repeatable = r}

public export
triggerMultiplierFor : (cause : EventQuery b) -> Count b -> Predicate b AnObject -> StaticEffect b
triggerMultiplierFor c e a = TriggerMultiplier c e {affected = a}

public export
mayCastForFrom : List (Cost b) -> List Zone -> StaticEffect b
mayCastForFrom cs z = MayCastFor cs {from = z}

public export
canWindow : Deed b -> Maybe Timing -> StaticEffect b
canWindow d w = Can d {window = w}

public export
ifElse : Condition b -> (thenDo : OneShotEffect b) -> Maybe (OneShotEffect b) -> OneShotEffect b
ifElse c t e = If c t {otherwise = e}

public export
mayWith : (effect : OneShotEffect b) -> Maybe (OneShotEffect (intro effect b))
       -> Maybe (OneShotEffect b) -> OneShotEffect b
mayWith e did notd = May e {ifDid = did} {ifNot = notd}

-- The collapsed `MayPay`/`MustPay` shape ([CR#118.12a]): `who` MAY pay `cost`;
-- if they do -> `ifDid` (typed over the payment, like an `AdditionalCost`
-- body); if not -> `ifNot`. `who` is required (Law 2, no default) — mirrors
-- Rust's `May.who`, unlike `May`'s OWN implicit default (every other `May`
-- stays implicitly You). The MustPay punisher is `ifDid = Nothing`; a
-- branchless `May (Pay cost)` is `ifDid = Nothing, ifNot = Nothing` — one
-- smart constructor for all three, replacing the old `mayPayFull`/`mustPayBy`.
public export
mayPayCostBy : Reference b APlayer -> (cost : Cost b)
            -> Maybe (OneShotEffect (bindEvent (costCaps cost) (costRoles (costCaps cost)) b))
            -> Maybe (OneShotEffect b) -> OneShotEffect b
mayPayCostBy w cost did notd = May (Act (Pay cost)) {who = w} {ifDid = did} {ifNot = notd}

public export
mkChooseSpecRep : Quantity b -> Bool -> ChooseSpec b
mkChooseSpecRep q r = MkChooseSpec q {repeats = r}

public export
mkModeCost : (effect : OneShotEffect b) -> Maybe (Cost b) -> Mode b
mkModeCost e c = MkMode e {cost = c}

public export
activatedFull : Cost b -> OneShotEffect b -> Timing -> List UsageLimit -> List Zone
             -> Maybe (Condition b) -> Ability b
activatedFull c e w l f g = Activated c e {window = w} {limits = l} {from = f} {activationGuard = g}

public export
triggeredFull : (q : EventQuery b)
             -> OneShotEffect (bindEvent (eventQueryCaps q) (queryRoles q) b)
             -> List UsageLimit -> List Zone -> Ability b
triggeredFull q e l f = Triggered q e {limits = l} {from = f}

public export
replacesLimit : (q : EventQuery b)
             -> OneShotEffect (bindEvent (eventQueryCaps q) (queryRoles q) b)
             -> ReplaceLimit b -> StaticEffect b
replacesLimit q e l = Replaces q e {limit = l}

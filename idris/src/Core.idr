||| Core grammar of the toy MTG card model: characteristics, the `Bindings`
||| typestate (what references are in scope), and the filter / reference /
||| selection / action / effect / ability trees. Kept deliberately brief.
module Core

import public Data.Vect
import public Data.Nat
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
data SimpleManaSymbol
  = Generic Nat
  | Specific (Maybe Color)

public export
data ManaSymbol
  = Simple SimpleManaSymbol
  | Hybrid SimpleManaSymbol Color
  | Variable

public export
implementation Cast Nat ManaSymbol where
  cast = Simple . Generic

public export
implementation Cast Integer ManaSymbol where
  cast = cast . cast {to=Nat}

public export
implementation Cast Color ManaSymbol where
  cast = Simple . Specific . Just

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
  = Bear | Rat | Spider | Human | Knight | Goblin | Elf | Zombie | Elemental  -- creature types
public export
data EnchantmentSubtype
  = Aura
public export
data ArtifactSubtype
  = Equipment
public export
data LandSubtype
  = Island
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
implementation Cast CreatureSubtype Subtype where
  cast = CreatureSub
public export
implementation Cast EnchantmentSubtype Subtype where
  cast = EnchantmentSub
public export
implementation Cast LandSubtype Subtype where
  cast = LandSub
public export
implementation Cast ArtifactSubtype Subtype where
  cast = ArtifactSub
public export
implementation Cast BattleSubtype Subtype where
  cast = BattleSub

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

public export
data Cmp = Equal | GreaterEq | LessEq | Greater | Less

-- What kind of object a filter matches ([CR#109.3]). Rust: ObjectKind.
public export
data ObjectKind = IsCard | IsEmblem | IsPlayerKind | IsSpell | IsToken | IsAbility

-- Supertypes ([CR#205.4a]); independent of card type and subtype.
public export
data Supertype = Basic | Legendary | Ongoing | Snow | World

-- A kind of counter ([CR#122]). The TYPE is `CounterKind` — bare `Counter` is taken
-- by the spell-countering `Action`.
public export
data CounterKind = Loyalty | Fate | Charge | P1P1 | M1M1

-- Activation restrictions on an activated ability ([CR#602.5]). Loyalty abilities are
-- `[SorcerySpeed, OncePerTurn]`.
public export
data Restriction = SorcerySpeed | OncePerTurn | OncePerGame

-- Whether a `Reference` denotes an object or a player ([CR#109.1]). One reference
-- language, indexed by this — strict on the kind where it matters, lax where it doesn't.
public export
data Ent = AnObject | APlayer

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

public export
data PhaseStep
  = BeginningPhase BeginningStep
  | MainPhase Nat
  | CombatPhase CombatStep
  | EndingPhase EndingStep

public export
implementation Cast BeginningStep PhaseStep where
  cast = BeginningPhase
public export
implementation Cast CombatStep PhaseStep where
  cast = CombatPhase
public export
implementation Cast EndingStep PhaseStep where
  cast = EndingPhase

-- A history-lookback / timing scope for an `EventQuery`. Rust: Window.
public export
data Window = ThisGame | ThisTurn | LastTurn | ThisCombat | ThisStep

-- What KIND of event an `EventQuery` matches. `ZoneChanged`/`BeginStep` carry data;
-- "dies" = ZoneChanged (Just Battlefield) (Just Graveyard). (`Drew`/`DealtDamage` are
-- past-tense to avoid clashing with the `Action` verbs `Draw`/`DealDamage`.)
public export
data EventKind
  = Cast | Sacrificed | Drew | Discarded | DealtDamage
  | ZoneChanged (Maybe Zone) (Maybe Zone)
  | BeginStep PhaseStep

-- `Bindings`: the typestate of what references are in scope. Its fields are
-- PROJECTIONS we write constraints against; it grows as the model binds roles.
public export
record Bindings where
  constructor MkBindings
  targetCount : Nat
  thatBound   : Bool   -- a `With`-bound group (`That`)
  itBound     : Bool   -- a `ForEach`-bound element (`It`)
  eventBound  : Bool   -- inside a trigger/replacement/delayed body (`EventObject`/`EventActor`)

-- The bindings a resolving spell starts in: nothing bound yet.
public export
Base : Bindings
Base = MkBindings 0 False False False

-- Each sets one field, reconstructing `MkBindings` explicitly so a projection of a
-- bind result reduces definitionally even for abstract `b` (record-update sugar
-- has no get-after-set law for an abstract record).
public export
bindTargets : Nat -> Bindings -> Bindings
bindTargets n b = MkBindings n (thatBound b) (itBound b) (eventBound b)

public export
unbindTargets : Bindings -> Bindings
unbindTargets b = MkBindings 0 (thatBound b) (itBound b) (eventBound b)

public export
bindThat : Bindings -> Bindings
bindThat b = MkBindings (targetCount b) True (itBound b) (eventBound b)

public export
bindIt : Bindings -> Bindings
bindIt b = MkBindings (targetCount b) (thatBound b) True (eventBound b)

-- entering a trigger/replacement/delayed body, where the event's object/player are bound.
public export
bindEvent : Bindings -> Bindings
bindEvent b = MkBindings (targetCount b) (thatBound b) (itBound b) True

-- "target n is a legal reference in bindings b".
public export
ValidTarget : Nat -> Bindings -> Type
ValidTarget n b = LTE (S n) (targetCount b)

-- A keyword ability ([CR#702]). Rust: Ability::Keyword(KeywordAbility). Defined
-- before the predicate block so a `Predicate` can ask `HasKeyword`.
public export
data KeywordAbility : Bindings -> Type where
  Flying : KeywordAbility b
  FirstStrike : KeywordAbility b
  DoubleStrike : KeywordAbility b
  Deathtouch : KeywordAbility b
  Reach : KeywordAbility b
  Trample : KeywordAbility b
  Vigilance : KeywordAbility b
  Flash : KeywordAbility b

-- Reference / Count / Predicate / Condition / EventQuery are one mutually recursive
-- language. A PREDICATE is an object test — its candidate is IMPLICIT, so there's
-- NO `Subject` reference or `bindSubject` gate. A *filter* IS a `Predicate`. A
-- `Condition` is a closed/game-state test that reaches objects only via
-- `Matches : Reference -> Predicate` (or `exists`/`unique`, below). Combinators:
-- predicates use `AllOf`/`OneOf`/`IsNot`; conditions use `And`/`Or`/`Not`.
mutual
  -- A REFERENCE to a single game entity, indexed by `Ent` (object vs player). One
  -- reference language now: object-refs and player-refs together, strict on the kind
  -- where it matters (`StatOf` needs `AnObject`, `LifeTotal` needs `APlayer`) and lax
  -- where it doesn't (`SameAs`, damage). A target's kind FLEXES — `AnObject` by default,
  -- `APlayer` where a player op forces it.
  public export
  data Reference : Bindings -> Ent -> Type where
    -- the source object; always available [CR#113.7].
    This : Reference b AnObject
    -- a target; kind flexes (default `AnObject`, becomes `APlayer` where a player op forces it).
    GetTarget : (n : Nat) -> {auto prf : ValidTarget n b} -> {default AnObject k : Ent} -> Reference b k
    -- the unique object matching a predicate.
    Only : Predicate b -> Reference b AnObject
    -- the host this is attached to ("enchanted creature"); and its inverse.
    AttachHostOf : Reference b AnObject -> Reference b AnObject
    AttachedTo : Reference b AnObject -> Reference b AnObject
    -- the `ForEach`-bound element ("it"); gated by `itBound`.
    It : {auto prf : itBound b = True} -> Reference b AnObject
    -- the triggering event's object ("that card"); gated by `eventBound` ([CR#608.2g]).
    EventObject : {auto prf : eventBound b = True} -> Reference b AnObject
    -- PLAYERS (the old `PlayerRef`, folded in here):
    You : Reference b APlayer                            -- controller of this ability [CR#109.5]
    Opponent : Reference b APlayer                       -- an opponent [CR#102.1]; single-opponent for now
    ControllerOf : Reference b AnObject -> Reference b APlayer   -- the controller of an object
    OwnerOf : Reference b AnObject -> Reference b APlayer        -- the owner of an object [CR#108.3]
    EventActor : {auto prf : eventBound b = True} -> Reference b APlayer  -- the event's player ("that player")
    EachPlayer : Reference b APlayer      -- FLAG: PLURAL (Stage 2 dissolves this into a player `Selection`)
    EachOpponent : Reference b APlayer    -- FLAG: plural

  -- A numeric value ([CR#107.3]). `Literal` is a bare number; the rest read the game
  -- state — object counts, stats, counters, life/hand totals, event tallies, arithmetic.
  public export
  data Count : Bindings -> Type where
    Literal : Nat -> Count b                  -- a bare number
    X : Count b                               -- the chosen {X} value
    CountOf : Predicate b -> Count b          -- how many objects match a predicate
    StatOf : Reference b AnObject -> Stat -> Count b     -- an object's power/toughness/etc.
    EventCount : EventQuery b -> Count b      -- how many matching events occurred (window is in the query)
    CountersOn : CounterKind -> Reference b AnObject -> Count b   -- number of [kind] counters on r
    LifeTotal : Reference b APlayer -> Count b           -- a player's life total
    HandSize : Reference b APlayer -> Count b            -- cards in a player's hand
    Plus  : Count b -> Count b -> Count b                -- arithmetic on values
    Minus : Count b -> Count b -> Count b
    Times : Count b -> Count b -> Count b
    HalfUp : Count b -> Count b                          -- "half, rounded up"
    HalfDown : Count b -> Count b
    ThatMuch : Count b                                   -- FLAG: amount-anaphora (the preceding amount; ungated)

  -- A PREDICATE: a test on a single IMPLICIT candidate object — i.e. a *filter*.
  -- The atoms read the candidate's characteristics; `SameAs r` tests identity.
  public export
  data Predicate : Bindings -> Type where
    HasType : Type_ -> Predicate b
    HasSupertype : Supertype -> Predicate b
    HasSubtype : Subtype -> Predicate b
    HasColor : Color -> Predicate b
    IsKind : ObjectKind -> Predicate b
    InZone : Zone -> Predicate b
    HasKeyword : KeywordAbility b -> Predicate b
    SameAs : Reference b k -> Predicate b      -- the candidate IS r (ANY kind; "another" = IsNot (SameAs This))
    SameName : Reference b AnObject -> Predicate b   -- shares a name with r ("named [its own name]" = SameName This)
    WasCastFrom : Zone -> Predicate b          -- the object was cast from this zone (cast provenance)
    ExiledBy : Reference b AnObject -> Predicate b   -- set aside by r's effect ("cards exiled by this" = ExiledBy
                                               -- This); the engine holds the association ([CR#607] linked abilities)
    HasName : String -> Predicate b            -- named a specific card (tutors / token names)
    HasCounter : CounterKind -> Predicate b    -- has ≥1 of this counter ("without a fate counter" = IsNot (HasCounter Fate))
    ControlledBy : Reference b APlayer -> Predicate b   -- "creature you control" = AllOf [HasType Creature, ControlledBy You]
    OwnedBy : Reference b APlayer -> Predicate b
    WasKicked : Predicate b                    -- FLAG: kicker as a boolean flag on the object (no cost-mode model)
    -- combinators (distinct from `Condition`'s And/Or/Not):
    AllOf : List (Predicate b) -> Predicate b
    OneOf : List (Predicate b) -> Predicate b
    IsNot : Predicate b -> Predicate b        -- negation

  -- A CLOSED / game-state test ([CR#603.4]); reaches objects only via `Matches`
  -- (apply a `Predicate` to a named `Reference`) or `exists`/`unique` (below).
  public export
  data Condition : Bindings -> Type where
    Matches : Reference b AnObject -> Predicate b -> Condition b   -- does object r satisfy the predicate
    Compare : Count b -> Cmp -> Count b -> Condition b
    TurnOf : Reference b APlayer -> Condition b   -- it's this player's turn (`yourTurn = TurnOf You`)
    During : PhaseStep -> Condition b
    And : List (Condition b) -> Condition b
    Or : List (Condition b) -> Condition b
    Not : Condition b -> Condition b

  -- A query OVER EVENTS: the matcher for triggers, `EventCount`, and durations — the
  -- event analog of `Predicate`. Facets conjoin via `Query`; `Join`/`Except` are
  -- or/not. `SourceMatches` embeds the object language; `Within`/`DuringStep`/
  -- `DuringTurn` are the timing facets ("not during your turn" = `Except (DuringTurn You)`).
  public export
  data EventQuery : Bindings -> Type where
    KindIs        : EventKind -> EventQuery b
    SourceMatches : Predicate b -> EventQuery b
    ActorIs       : Reference b APlayer -> EventQuery b
    Within        : Window -> EventQuery b
    DuringStep    : PhaseStep -> EventQuery b
    DuringTurn    : Reference b APlayer -> EventQuery b
    Query  : List (EventQuery b) -> EventQuery b   -- AND
    Join   : List (EventQuery b) -> EventQuery b   -- OR
    Except : EventQuery b -> EventQuery b          -- NOT


-- "it's your turn" — the common specialization of `TurnOf`.
public export
yourTurn : Condition b
yourTurn = TurnOf You

-- `exists`/`unique`: a predicate matches ≥1 / exactly-1 object. DERIVED from
-- `CountOf` + `Compare`, not primitive constructors. (`CountOf` takes a `Predicate`,
-- so `exists (During …)` is now a TYPE error, not a degenerate term.)
public export
exists : Predicate b -> Condition b
exists p = Compare (CountOf p) Greater (Literal 0)

public export
unique : Predicate b -> Condition b
unique p = Compare (CountOf p) Equal (Literal 1)

public export
implementation Cast Nat (Count b) where
  cast = Literal
public export
implementation Cast Integer (Count b) where
  cast = Literal . cast {to=Nat}

-- Integer literals + `+`/`*` sugar for the value language (so `power := Just 2` and
-- `SetPT 1 1` typecheck; `Plus`/`Times` back the operators).
public export
implementation Num (Count b) where
  (+) = Plus
  (*) = Times
  fromInteger = Literal . cast {to=Nat}

-- A SIGNED change to a value (layer-7c "+N/+N" P/T modifications). `cast` builds it from
-- an Integer (`cast 2` = Up 2, `cast (-1)` = Down 1); use `Up`/`Down` for dynamic deltas.
public export
data Delta : Bindings -> Type where
  Up   : Count b -> Delta b   -- "+N"
  Down : Count b -> Delta b   -- "−N"

public export
implementation Cast Integer (Delta b) where
  cast n = if n >= 0 then Up (cast n) else Down (cast (negate n))

-- A game-result effect ([CR#104]). Its own category above `Action` — a game-ender
-- isn't just another verb; `Effect`'s `Conclude` wraps it.
public export
data Outcome : Bindings -> Type where
  WinGame  : Reference b APlayer -> Outcome b
  LoseGame : Reference b APlayer -> Outcome b

-- Where a card goes in a library ([CR#401]). `FromTop (Literal 0)` = on top.
public export
data LibraryPosition : Bindings -> Type where
  FromTop    : Count b -> LibraryPosition b
  FromBottom : Count b -> LibraryPosition b

-- A continuous effect's lifetime ([CR#611.2]). Rust: Duration. (Above `Action` so a
-- duration-bounded verb like `ExileUntil` can name it.)
public export
data Duration : Bindings -> Type where
  UntilEndOfTurn : Duration b
  UntilEvent : EventQuery b -> Duration b
  ForAsLongAs : Condition b -> Duration b
  Permanent : Duration b                       -- rest of game (Rust: EndOfGame)

-- A cardinality spec for a choice ([CR#107.3]). Rust: Quantity.
public export
data Quantity : Bindings -> Type where
  Range : Maybe (Count b) -> Maybe (Count b) -> Quantity b

-- `Range lo hi`: `Nothing` bound = unbounded that side. A bare numeral is the
-- EXACTLY case (`Range (Just n) (Just n)`); the helpers below name the rest.
public export
implementation Cast Integer (Quantity b) where
  cast n = let k = Literal (cast {to=Nat} n) in Range (Just k) (Just k)

public export
atLeast : Count b -> Quantity b
atLeast n = Range (Just n) Nothing

public export
atMost : Count b -> Quantity b
atMost n = Range Nothing (Just n)

public export
between : Count b -> Count b -> Quantity b
between lo hi = Range (Just lo) (Just hi)

public export
anyNumber : Quantity b
anyNumber = Range Nothing Nothing

public export
data TargetSpec : Bindings -> Type where
  Target : Nat -> Predicate b -> TargetSpec b

-- A resolution-time GROUP / choice. `Reference` is single-GameObject only, so
-- the plural anaphor lives HERE, not there. Mirrors Rust `Selection`.
public export
data Selection : Bindings -> Type where
  -- the matching objects as one set (Rust: Selection::Filter). A single object is
  -- just `SelectAll (SameAs r)`, so there's no dedicated ref-to-selection variant.
  SelectAll : Predicate b -> Selection b
  -- the whole ordered group bound by an enclosing `With`. Rust: Selection::Those.
  That : {auto prf : thatBound b = True} -> Selection b
  -- (choosing is interactive — it's a `Bindable`, not a `Selection`. No distributive
  -- `Each` operand either: distribution is `ForEach`, the set is `SelectAll`.)
  -- a random quantity of the matching objects. Rust: Selection::Random.
  Random : Quantity b -> Predicate b -> Selection b
  -- the top n cards of a library (default: yours). Rust: Selection::TopOfLibrary.
  TopOfLibrary : (count : Count b) -> {default You whose : Reference b APlayer} -> Selection b
  -- the bottom n cards of a library (positional reference beyond the top). Rust: Selection::BottomOfLibrary.
  BottomOfLibrary : (count : Count b) -> {default You whose : Reference b APlayer} -> Selection b

-- A token's characteristics ([CR#111.1]). Vanilla — FLAG: token abilities/keywords
-- aren't modeled here (most common tokens are vanilla creatures).
public export
record TokenSpec where
  constructor MkToken
  tokName : String
  tokTypes : List Type_
  tokSubtypes : List Subtype
  tokColors : List Color
  tokPower : Count Base
  tokToughness : Count Base

-- The verbs ([CR#701]). `Effect::Act` wraps these. Object verbs carry an object
-- `source` (default `This`); player verbs an `actor : Reference b APlayer` (default `You`).
public export
data Action : Bindings -> Type where
  -- deal damage to a `Selection`; source object is the agent ([CR#120.1]).
  DealDamage : {default This source : Reference b AnObject} -> Selection b -> Count b -> Action b
  -- a plain zone change [CR#400.7]; owner-relative, control implicit.
  Move : Selection b -> Zone -> Action b
  -- exile a selection UNTIL a duration ends, then return it — the duration-bounded
  -- "exile until ~" form ([CR#603.6e]), NOT a leave-triggered return (see Oblivion Ring).
  ExileUntil : Selection b -> Duration b -> Action b
  -- destroy [CR#701.8] / counter a stack object [CR#701.6a]. (Return-to-hand is just
  -- `Move … Hand` — `Move` is owner-relative — so there's no dedicated bounce verb.)
  Destroy : Selection b -> Action b
  Counter : Selection b -> Action b
  -- tap / untap [CR#701.26]; attach / unattach [CR#701.3].
  Tap : Selection b -> Action b
  Untap : Selection b -> Action b
  Attach : (what : Selection b) -> (to : Selection b) -> Action b
  Unattach : Selection b -> Action b
  -- a player verb: the `actor` draws n cards. Rust: PlayerAction::Draw(Count).
  Draw : {default You actor : Reference b APlayer} -> Count b -> Action b
  -- the `actor` gains n life. Rust: PlayerAction::GainLife(Count).
  GainLife : {default You actor : Reference b APlayer} -> Count b -> Action b
  -- put a selection into its owner's library at a position ([CR#401]).
  PutIntoLibrary : Selection b -> LibraryPosition b -> Action b
  -- put / clear counters ([CR#122]). `RemoveAllCounters` clears every counter of a kind.
  PutCounters : CounterKind -> Count b -> Selection b -> Action b
  RemoveAllCounters : CounterKind -> Selection b -> Action b
  -- player verbs: discard / lose life; and a chooser-verb where a player sacrifices.
  Discard : {default You actor : Reference b APlayer} -> Count b -> Action b
  LoseLife : {default You actor : Reference b APlayer} -> Count b -> Action b
  Sacrifices : Reference b APlayer -> Predicate b -> Action b   -- "[player] sacrifices a [pred]" (they choose which)
  -- keyword actions / further verbs ([CR#701]). The interactive bits (reorder, search
  -- choice, copy characteristics) are the engine's; the grammar names the verb.
  Scry : Count b -> Action b                            -- look at top n, reorder / bottom some
  Surveil : Count b -> Action b
  Fight : (x : Selection b) -> (y : Selection b) -> Action b   -- each deals damage equal to its power to the other
  Reveal : Selection b -> Action b
  Shuffle : {default You actor : Reference b APlayer} -> Action b
  CreateToken : Count b -> TokenSpec -> Action b        -- FLAG: vanilla token spec (no abilities)
  CopySpell : Selection b -> Action b                   -- "copy target spell" — FLAG: copy semantics deferred to engine
  AddMana : {default You actor : Reference b APlayer} -> ManaCost -> Action b   -- "add {G}" (mana ability effect); pool/paying is engine

-- What a binder (`With`) binds as `That`: a QUERY of existing objects, a PRODUCER
-- (an `Action` run for effect, binding its product), or a CHOICE (a player picks).
-- The grammar only names the role; the ENGINE resolves `That` to the live (reminted
-- or gone) object, so `MovedRef`/lki/became is a runtime concern, NOT modeled here.
public export
data Bindable : Bindings -> Type where
  Existing : Selection b -> Bindable b  -- bind existing objects (a plain selection)
  Produce : Action b -> Bindable b      -- run the action, bind its product (the moved object) as `That`
  -- `by` chooses a `Quantity` of objects matching the filter; the chosen are bound as
  -- `That`. Choosing is interactive, so it lives here, not in `Selection`. Rust: Selection::Choose.
  Choose : {default You by : Reference b APlayer} -> Quantity b -> Predicate b -> Bindable b
  -- `by` searches `whose`'s `from`-zones (one or more — "library and/or graveyard") for
  -- matching cards, bound as `That` — like `Choose`, but from (hidden) zones the engine
  -- reveals/shuffles. Search ANOTHER player's via `whose`; the found card's destination
  -- is a following owner-routed `Move That …`. Rust: Selection::Search.
  Search : {default You by : Reference b APlayer} -> {default You whose : Reference b APlayer} -> {default [Library] from : List Zone} -> Quantity b -> Predicate b -> Bindable b

-- A cost paid to activate an ability ([CR#118,602]). `Costs` conjoins components;
-- `TapSelf`/`Sacrifice`/… read `This` (the ability's source). Rust: Cost.
public export
data Cost : Bindings -> Type where
  Mana      : ManaCost -> Cost b                 -- "{4}"
  TapSelf   : Cost b                             -- "{T}"
  UntapSelf : Cost b                             -- "{Q}"
  PayLife   : Count b -> Cost b                  -- "Pay N life"
  Sacrifice : Selection b -> Cost b              -- "Sacrifice this" = Sacrifice (SelectAll (SameAs This))
  AddCounters    : CounterKind -> Count b -> Cost b   -- a loyalty "+N" cost (put N counters on This)
  RemoveCounters : CounterKind -> Count b -> Cost b   -- a loyalty "−N" cost (remove N from This)
  Costs     : List (Cost b) -> Cost b            -- all components together

-- How many modes to choose, for a modal effect ([CR#700.2]). Rust: ChooseSpec.
public export
data ChooseSpec : Bindings -> Type where
  MkChooseSpec : (count : Count b) -> {default False upTo : Bool} -> {default False repeats : Bool} -> ChooseSpec b

-- Effects, continuous effects, and abilities are mutually recursive: a one-shot
-- can CREATE a continuous effect (`Continuously`), a static ability can grant an
-- ability, and an ability wraps an effect.
mutual
  public export
  data Effect : Bindings -> Type where
    Sequence : (List (Effect b)) -> Effect b
    Targeted : (Vect n (TargetSpec b)) -> Effect (bindTargets n b) -> Effect b
    -- binds `that` as `That` for `body`. `that` may PRODUCE a moved object. Rust: Effect::With.
    With : Bindable b -> Effect (bindThat b) -> Effect b
    -- a single intrinsic instruction (the verb compartment). Rust: Effect::Act.
    Act : Action b -> Effect b
    -- end the game (or a player's part in it) — the `Outcome` compartment. Rust: Effect::Conclude.
    Conclude : Outcome b -> Effect b
    -- "you may [effect]", with optional "if you do / if you don't". Rust: Effect::May.
    May : (effect : Effect b) -> {default Nothing ifDid : Maybe (Effect b)} -> {default Nothing ifNot : Maybe (Effect b)} -> Effect b
    -- "if [cond], [thenDo]; otherwise [else]". Rust: Effect::If.
    If : Condition b -> (thenDo : Effect b) -> {default Nothing otherwise : Maybe (Effect b)} -> Effect b
    -- "[effect] unless [who] pays [cost]" (CostComponent; ManaCost stand-in). Rust: Effect::Unless.
    Unless : (effect : Effect b) -> {default You actor : Reference b APlayer} -> ManaCost -> Effect b
    -- create a continuous effect for a duration ([CR#611.2]). Rust: Effect::Continuously.
    Continuously : StaticEffect b -> Duration b -> Effect b
    -- choose modes, then apply them ([CR#700.2]). Rust: Effect::Modal.
    Modal : ChooseSpec b -> List (Mode b) -> Effect b
    -- "for each [domain], [body]" — binds each element as `It`. The distributive
    -- primitive (subsumes the old `Selection::Each`). Rust: Effect::ForEach.
    ForEach : Selection b -> Effect (bindIt b) -> Effect b
    -- "when you do [the preceding], [effect]" — a reflexive trigger. It NESTS, so
    -- `That`/targets stay in scope; no event-scanning sibling. Rust: Effect::Reflexive.
    Reflexive : Effect b -> Effect b
    -- schedule `body` for `event`; `unbindTargets` keeps `That`, drops targets. Rust: Effect::Delayed.
    Delayed : EventQuery b -> Effect (bindEvent (unbindTargets b)) -> Effect b

  -- one option of a modal effect: an effect plus an optional extra cost. Rust: Mode.
  public export
  data Mode : Bindings -> Type where
    MkMode : (effect : Effect b) -> {default Nothing cost : Maybe ManaCost} -> Mode b

  -- A continuous modification a static ability applies to its subject.
  public export
  data Modification : Bindings -> Type where
    ModifyPT : Delta b -> Delta b -> Modification b     -- "gets +x/+y" (SIGNED, layer 7c — Up/Down)
    SetPT : Count b -> Count b -> Modification b         -- "base p/t are x/y" (layer 7b; x/y may be dynamic — CDA `*/*`)
    AddType : Type_ -> Modification b                   -- "is also a [type]"
    AddSubtype : Subtype -> Modification b              -- "becomes an Island" (adds the subtype)
    LoseAbilities : Modification b                      -- "loses all abilities" (Humility-style)
    GainControl : Reference b APlayer -> Modification b         -- "[player] gains control"
    GrantAbility : Ability -> Modification b

  -- A continuous effect a static (or `Continuously`) ability generates ([CR#611]):
  -- modify a subject, modify a whole filter (anthem), or REPLACE an event — a
  -- replacement effect is a continuous effect too ([CR#614]). Rust: the StaticEffect family.
  public export
  data StaticEffect : Bindings -> Type where
    Modify : Reference b AnObject -> List (Modification b) -> StaticEffect b
    ModifyAll : Predicate b -> List (Modification b) -> StaticEffect b   -- anthem: "each [filter] gets [mods]"
    -- "if [event] would happen, do [effect] instead" — the card names only the
    -- replacement (empty = a pure skip); the engine skips the original + handles edges.
    Replaces : EventQuery b -> Effect (bindEvent b) -> StaticEffect b
    -- the inner continuous effect applies only WHILE the condition holds ([CR#604.3]) —
    -- a conditional static ("gets +1/+1 as long as …").
    While : Condition b -> StaticEffect b -> StaticEffect b

  -- A castable spell resolves in `Base`: source bound, no top-level targets.
  public export
  data Ability : Type where
    Spell : Effect Base -> Ability
    Keyword : KeywordAbility Base -> Ability
    -- "{cost}: {effect}" — an activated ability ([CR#602]). `limits` are the activation
    -- restrictions (a loyalty ability is `{limits = [SorcerySpeed, OncePerTurn]}`).
    Activated : Cost Base -> Effect Base -> {default [] limits : List Restriction} -> Ability
    -- a triggered ability: when `event` fires, resolve `effect`. Rust: Ability::Triggered.
    Triggered : EventQuery Base -> Effect (bindEvent Base) -> Ability
    -- "Enchant <filter>": what this Aura may attach to ([CR#702.5]).
    Enchant : Predicate Base -> Ability
    -- a static continuous ability — modifications, anthems, AND replacements live in `StaticEffect`.
    Static : StaticEffect Base -> Ability

public export
record Face where
  constructor MkFace
  name : String
  manaCost : ManaCost
  types : List Type_
  supertypes : List Supertype
  subtypes : List Subtype
  abilities : List Ability
  power : Maybe (Count Base)
  toughness : Maybe (Count Base)
  loyalty : Maybe (Count Base)
  defense : Maybe (Count Base)

public export
interface DefaultValue a where
  defaultValue : a

public export
fromDefault : (DefaultValue a) => (a -> a) -> a
fromDefault b = b defaultValue

public export
implementation DefaultValue Face where
  defaultValue = MkFace
    { name = ""
    , manaCost = []
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
-- fields the `fromDefault { … := … }` builder can still set.
public export
SubtypesOk : Face -> Type
SubtypesOk b = All (\s => Elem (subtypeCategory s) (types b)) (subtypes b)

public export
data Card : Type where
  Normal : (b : Face) -> {auto ok : SubtypesOk b} -> Card

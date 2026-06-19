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
data Stat = Power | Toughness

public export
data Cmp = Equal | GreaterEq | LessEq | Greater | Less

-- What kind of object a filter matches ([CR#109.3]). Rust: ObjectKind.
public export
data ObjectKind = IsCard | IsEmblem | IsPlayerKind | IsSpell | IsToken

-- Supertypes ([CR#205.4a]); independent of card type and subtype.
public export
data Supertype = Basic | Legendary | Ongoing | Snow | World

-- A kind of counter ([CR#122]). The TYPE is `CounterKind` — bare `Counter` is taken
-- by the spell-countering `Action`.
public export
data CounterKind = Loyalty | Fate | Charge | P1P1 | M1M1

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

-- The bindings a resolving spell starts in: nothing bound yet.
public export
Base : Bindings
Base = MkBindings 0 False False

-- Each sets one field, reconstructing `MkBindings` explicitly so a projection of a
-- bind result reduces definitionally even for abstract `b` (record-update sugar
-- has no get-after-set law for an abstract record).
public export
bindTargets : Nat -> Bindings -> Bindings
bindTargets n b = MkBindings n (thatBound b) (itBound b)

public export
unbindTargets : Bindings -> Bindings
unbindTargets b = MkBindings 0 (thatBound b) (itBound b)

public export
bindThat : Bindings -> Bindings
bindThat b = MkBindings (targetCount b) True (itBound b)

public export
bindIt : Bindings -> Bindings
bindIt b = MkBindings (targetCount b) (thatBound b) True

-- "target n is a legal reference in bindings b".
public export
ValidTarget : Nat -> Bindings -> Type
ValidTarget n b = LTE (S n) (targetCount b)

-- A keyword ability ([CR#702]). Rust: Ability::Keyword(KeywordAbility). Defined
-- before the filter block so a `Filter` can ask `HasKeyword`.
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

-- Reference / Count / Predicate / Condition / PlayerRef are one mutually recursive
-- language. A PREDICATE is an object test — its candidate is IMPLICIT, so there's
-- NO `Subject` reference or `bindSubject` gate. A *filter* IS a `Predicate`. A
-- `Condition` is a closed/game-state test that reaches objects only via
-- `Matches : Reference -> Predicate` (or `exists`/`unique`, below). Combinators:
-- predicates use `AllOf`/`OneOf`/`IsNot`; conditions use `And`/`Or`/`Not`.
mutual
  -- A single GAME OBJECT. Player specifiers live in `PlayerRef`, not here.
  public export
  data Reference : Bindings -> Type where
    -- the source; always available — every spell/ability has one [CR#113.7].
    This : Reference b
    -- DEMANDS the bindings bound at least an (n+1)-th target.
    GetTarget : (n : Nat) -> {auto prf : ValidTarget n b} -> Reference b
    -- the unique object matching a predicate.
    Only : Predicate b -> Reference b
    -- the permanent R is attached to ("enchanted creature"); and its inverse.
    AttachHostOf : Reference b -> Reference b
    AttachedTo : Reference b -> Reference b
    -- the element bound by an enclosing `ForEach` ("it"). Gated by `itBound`.
    It : {auto prf : itBound b = True} -> Reference b
    -- FLAG: the triggering event's object ("that card"). UNGATED — should only be
    -- available inside a Triggered/Delayed/Replaces body; no typestate for that yet.
    EventObject : Reference b

  -- A numeric value ([CR#107.3]). `Literal` is a bare number; the rest read the
  -- game state. (EventCount/EventSum/CounterCount/Min/ThatMuch deferred.)
  public export
  data Count : Bindings -> Type where
    Literal : Nat -> Count b                  -- a bare number
    X : Count b                               -- the chosen {X} value
    CountOf : Predicate b -> Count b          -- how many objects match a predicate
    StatOf : Reference b -> Stat -> Count b   -- a referenced object's power/toughness
    EventCount : EventQuery b -> Count b      -- how many matching events occurred (window is in the query)

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
    SameAs : Reference b -> Predicate b        -- the candidate IS r ("another" = IsNot (SameAs This))
    SameName : Reference b -> Predicate b      -- shares a name with r ("named [its own name]" = SameName This)
    WasCastFrom : Zone -> Predicate b          -- the object was cast from this zone (cast provenance)
    HasCounter : CounterKind -> Predicate b    -- has ≥1 of this counter ("without a fate counter" = IsNot (HasCounter Fate))
    ControlledBy : PlayerRef b -> Predicate b  -- "creature you control" = AllOf [HasType Creature, ControlledBy You]
    OwnedBy : PlayerRef b -> Predicate b
    WasKicked : Predicate b                    -- FLAG: kicker as a boolean flag on the object (no cost-mode model)
    -- combinators (distinct from `Condition`'s And/Or/Not):
    AllOf : List (Predicate b) -> Predicate b
    OneOf : List (Predicate b) -> Predicate b
    IsNot : Predicate b -> Predicate b        -- negation

  -- A CLOSED / game-state test ([CR#603.4]); reaches objects only via `Matches`
  -- (apply a `Predicate` to a named `Reference`) or `exists`/`unique` (below).
  public export
  data Condition : Bindings -> Type where
    Matches : Reference b -> Predicate b -> Condition b   -- does r satisfy the predicate
    Compare : Count b -> Cmp -> Count b -> Condition b
    TurnOf : PlayerRef b -> Condition b   -- it's this player's turn (`yourTurn = TurnOf You`)
    During : PhaseStep -> Condition b
    And : List (Condition b) -> Condition b
    Or : List (Condition b) -> Condition b
    Not : Condition b -> Condition b

  -- A PLAYER specifier (objects live in `Reference`). In the block so `TurnOf` can name it.
  public export
  data PlayerRef : Bindings -> Type where
    You : PlayerRef b                          -- controller of this ability ([CR#109.5])
    Opponent : PlayerRef b                     -- an opponent ([CR#102.1]); single-opponent for now
    ControllerOf : Reference b -> PlayerRef b  -- the controller of a referenced object
    OwnerOf : Reference b -> PlayerRef b       -- the owner of a referenced object ([CR#108.3])
    EachPlayer : PlayerRef b                   -- FLAG: a PLURAL player specifier ("each player")
    EachOpponent : PlayerRef b                 -- FLAG: plural
    TargetedPlayer : (n : Nat) -> {auto prf : ValidTarget n b} -> PlayerRef b  -- FLAG: nth target read as a player

  -- A query OVER EVENTS: the matcher for triggers, `EventCount`, and durations — the
  -- event analog of `Predicate`. Facets conjoin via `Query`; `Join`/`Except` are
  -- or/not. `SourceMatches` embeds the object language; `Within`/`DuringStep`/
  -- `DuringTurn` are the timing facets ("not during your turn" = `Except (DuringTurn You)`).
  public export
  data EventQuery : Bindings -> Type where
    KindIs        : EventKind -> EventQuery b
    SourceMatches : Predicate b -> EventQuery b
    ActorIs       : PlayerRef b -> EventQuery b
    Within        : Window -> EventQuery b
    DuringStep    : PhaseStep -> EventQuery b
    DuringTurn    : PlayerRef b -> EventQuery b
    Query  : List (EventQuery b) -> EventQuery b   -- AND
    Join   : List (EventQuery b) -> EventQuery b   -- OR
    Except : EventQuery b -> EventQuery b          -- NOT


-- A *filter* is just a `Predicate` — the candidate is the predicate's IMPLICIT
-- argument, so there's no `Where`/`Subject`/`bindSubject`. The alias is kept only
-- so selection/target/event signatures read "filter" rather than "predicate".
public export
Filter : Bindings -> Type
Filter b = Predicate b

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

-- A game-result effect ([CR#104]). Its own category above `Action` — a game-ender
-- isn't just another verb; `Effect`'s `Conclude` wraps it.
public export
data Outcome : Bindings -> Type where
  WinGame  : PlayerRef b -> Outcome b
  LoseGame : PlayerRef b -> Outcome b

-- Where a card goes in a library ([CR#401]). `FromTop (Literal 0)` = on top.
public export
data LibraryPosition : Bindings -> Type where
  FromTop    : Count b -> LibraryPosition b
  FromBottom : Count b -> LibraryPosition b

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
  Target : Nat -> Filter b -> TargetSpec b

-- A resolution-time GROUP / choice. `Reference` is single-GameObject only, so
-- the plural anaphor lives HERE, not there. Mirrors Rust `Selection`.
public export
data Selection : Bindings -> Type where
  -- the matching objects as one set (Rust: Selection::Filter). A single object is
  -- just `SelectAll (SameAs r)`, so there's no dedicated ref-to-selection variant.
  SelectAll : Filter b -> Selection b
  -- the whole ordered group bound by an enclosing `With`. Rust: Selection::Those.
  That : {auto prf : thatBound b = True} -> Selection b
  -- (choosing is interactive — it's a `Bindable`, not a `Selection`. No distributive
  -- `Each` operand either: distribution is `ForEach`, the set is `SelectAll`.)
  -- a random quantity of the matching objects. Rust: Selection::Random.
  Random : Quantity b -> Filter b -> Selection b
  -- the top n cards of a library (default: yours). Rust: Selection::TopOfLibrary.
  TopOfLibrary : (count : Count b) -> {default You whose : PlayerRef b} -> Selection b

-- The verbs ([CR#701]). `Effect::Act` wraps these. Object verbs carry an object
-- `source` (default `This`); player verbs an `actor : PlayerRef` (default `You`).
public export
data Action : Bindings -> Type where
  -- deal damage to a `Selection`; source object is the agent ([CR#120.1]).
  DealDamage : {default This source : Reference b} -> Selection b -> Count b -> Action b
  -- a plain zone change [CR#400.7]; owner-relative, control implicit.
  Move : Selection b -> Zone -> Action b
  -- destroy [CR#701.8] / return to hand / counter a stack object [CR#701.6a].
  Destroy : Selection b -> Action b
  ReturnToHand : Selection b -> Action b
  Counter : Selection b -> Action b
  -- tap / untap [CR#701.26]; attach / unattach [CR#701.3].
  Tap : Selection b -> Action b
  Untap : Selection b -> Action b
  Attach : (what : Selection b) -> (to : Selection b) -> Action b
  Unattach : Selection b -> Action b
  -- a player verb: the `actor` draws n cards. Rust: PlayerAction::Draw(Count).
  Draw : {default You actor : PlayerRef b} -> Count b -> Action b
  -- the `actor` gains n life. Rust: PlayerAction::GainLife(Count).
  GainLife : {default You actor : PlayerRef b} -> Count b -> Action b
  -- put a selection into its owner's library at a position ([CR#401]).
  PutIntoLibrary : Selection b -> LibraryPosition b -> Action b
  -- put / clear counters ([CR#122]). `RemoveAllCounters` clears every counter of a kind.
  PutCounters : CounterKind -> Count b -> Selection b -> Action b
  RemoveAllCounters : CounterKind -> Selection b -> Action b
  -- player verbs: discard / lose life; and a chooser-verb where a player sacrifices.
  Discard : {default You actor : PlayerRef b} -> Count b -> Action b
  LoseLife : {default You actor : PlayerRef b} -> Count b -> Action b
  Sacrifices : PlayerRef b -> Predicate b -> Action b   -- "[player] sacrifices a [pred]" (they choose which)

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
  Choose : {default You by : PlayerRef b} -> Quantity b -> Filter b -> Bindable b

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

-- A continuous effect's lifetime ([CR#611.2]). Rust: Duration.
public export
data Duration : Bindings -> Type where
  UntilEndOfTurn : Duration b
  UntilEvent : EventQuery b -> Duration b
  ForAsLongAs : Condition b -> Duration b
  Permanent : Duration b                       -- rest of game (Rust: EndOfGame)

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
    Unless : (effect : Effect b) -> {default You who : PlayerRef b} -> ManaCost -> Effect b
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
    Delayed : EventQuery b -> Effect (unbindTargets b) -> Effect b

  -- one option of a modal effect: an effect plus an optional extra cost. Rust: Mode.
  public export
  data Mode : Bindings -> Type where
    MkMode : (effect : Effect b) -> {default Nothing cost : Maybe ManaCost} -> Mode b

  -- A continuous modification a static ability applies to its subject.
  public export
  data Modification : Bindings -> Type where
    PlusPT : Int -> Int -> Modification b               -- "gets +x/+y"
    SetPT : Int -> Int -> Modification b                -- "base power/toughness are x/y" (layer 7b)
    AddType : Type_ -> Modification b                   -- "is also a [type]"
    AddSubtype : Subtype -> Modification b              -- "becomes an Island" (adds the subtype)
    LoseAbilities : Modification b                      -- "loses all abilities" (Humility-style)
    GainControl : PlayerRef b -> Modification b         -- "[player] gains control"
    GrantAbility : Ability -> Modification b

  -- A static (continuous) effect: `subject` gets the modifications. Rust: Ability::Static.
  public export
  data StaticEffect : Bindings -> Type where
    Modify : Reference b -> List (Modification b) -> StaticEffect b
    ModifyAll : Filter b -> List (Modification b) -> StaticEffect b   -- anthem: "each [filter] gets [mods]"

  -- A castable spell resolves in `Base`: source bound, no top-level targets.
  public export
  data Ability
    = Spell (Effect Base)
    | Keyword (KeywordAbility Base)
    -- "{cost}: {effect}" — an activated ability ([CR#602]). Rust: Ability::Activated.
    | Activated (Cost Base) (Effect Base)
    -- a triggered ability: when `event` fires, resolve `effect`. Rust: Ability::Triggered.
    | Triggered (EventQuery Base) (Effect Base)
    -- "Enchant <filter>": what this Aura may attach to. Rust: the Enchant keyword [CR#702.5].
    | Enchant (Filter Base)
    -- a static continuous ability. Rust: Ability::Static.
    | Static (StaticEffect Base)
    -- "if [event] would happen, do [effect] instead" — a replacement ([CR#614]). The
    -- card only specifies the replacement effect (empty = a pure skip); the engine
    -- handles skipping the original and the rest of the rules, rules-accurately.
    | Replaces (EventQuery Base) (Effect Base)

public export
record Face where
  constructor MkFace
  name : String
  manaCost : ManaCost
  types : List Type_
  supertypes : List Supertype
  subtypes : List Subtype
  abilities : List Ability
  power : Maybe Int
  toughness : Maybe Int
  loyalty : Maybe Int
  defense : Maybe Int

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

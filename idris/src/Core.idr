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
data ManaSymbol
  = Simple SimpleManaSymbol
  | Hybrid SimpleManaSymbol Color
  | Variable

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
  = Bear | Rat | Spider | Human | Knight | Goblin | Elf | Zombie | Elemental | Wall | Spirit  -- creature types
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

-- A timing WINDOW — the speed at which an action is allowed: `InstantWindow` (any time you have
-- priority) or `SorceryWindow` (your main phase, empty stack — [CR#601.3,602.5d]). The ONE timing
-- notion, shared by a deontic `Can (Casts …)` (Flash widens to `InstantWindow`, [CR#702.8a]) and
-- by `Activated` (instant by default; "activate only as a sorcery" narrows to `SorceryWindow`).
public export
data TimingWindow = InstantWindow | SorceryWindow

-- Activation USE-LIMITS on an activated ability ([CR#602.5b]) — frequency caps, NOT timing (that's
-- `TimingWindow` above; the two used to overlap on a `SorcerySpeed` constructor). A loyalty ability
-- is `{window = SorceryWindow, limits = [OncePerTurn]}`.
public export
data Restriction = OncePerTurn | OncePerGame

-- Whether a `Reference` denotes an object or a player ([CR#109.1]). One reference
-- language, indexed by this — strict on the kind where it matters, lax where it doesn't.
-- `Anything` is the union kind for "any target" ([CR#115.4]) — an object OR a player;
-- only lax ops (damage) accept it, so it can't be read as a definite object/player.
public export
data RefKind = Empty | AnObject | APlayer | Anything

-- The JOIN on `RefKind` (least upper bound): `Empty` is the identity (bottom), like-with-like is
-- itself, two distinct kinds widen to `Anything` (the top) — so `(RefKind, \/, Empty)` is a
-- bounded join-semilattice. Spelled `\/` (a lattice join), not `||` — it isn't boolean OR.
-- `OneOf` folds it over its arms' kinds (base `Empty`) to COMPUTE a union's kind — what
-- retires `Widen`; an empty union folds to `Empty` (a vacuous predicate, matches nothing).
public export
(\/) : RefKind -> RefKind -> RefKind
(\/) Empty x = x
(\/) x Empty = x
(\/) AnObject AnObject = AnObject
(\/) APlayer APlayer = APlayer
(\/) _ _ = Anything

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
  targetKinds : List RefKind     -- one `RefKind` per target slot (the slot's kind, from its filter)
  thatKind    : Maybe RefKind    -- a `With`-bound group's element kind (`That`), if bound
  itKind      : Maybe RefKind    -- a `ForEach`-bound element's kind (`It`), if bound
  eventBound  : Bool         -- inside a trigger/replacement/delayed body (`EventObject`/`EventActor`)

-- The bindings a resolving spell starts in: nothing bound yet.
public export
Base : Bindings
Base = MkBindings [] Nothing Nothing False

-- Each sets one field, reconstructing `MkBindings` explicitly so a projection of a
-- bind result reduces definitionally even for abstract `b` (record-update sugar
-- has no get-after-set law for an abstract record).
public export
bindTargets : List RefKind -> Bindings -> Bindings
bindTargets ks b = MkBindings ks (thatKind b) (itKind b) (eventBound b)

public export
unbindTargets : Bindings -> Bindings
unbindTargets b = MkBindings [] (thatKind b) (itKind b) (eventBound b)

public export
bindThat : RefKind -> Bindings -> Bindings
bindThat k b = MkBindings (targetKinds b) (Just k) (itKind b) (eventBound b)

public export
bindIt : RefKind -> Bindings -> Bindings
bindIt k b = MkBindings (targetKinds b) (thatKind b) (Just k) (eventBound b)

-- entering a trigger/replacement/delayed body, where the event's object/player are bound.
public export
bindEvent : Bindings -> Bindings
bindEvent b = MkBindings (targetKinds b) (thatKind b) (itKind b) True

-- KeywordSpec / Reference / Count / Predicate / Condition / EventQuery are one mutually
-- recursive language. A PREDICATE is an object test — its candidate is IMPLICIT. A `Condition`
-- is a closed/game-state test reaching objects via `Matches`/`exists`/`unique`. Combinators:
-- predicates use `AllOf`/`OneOf`/`IsNot`; conditions use `And`/`Or`/`Not`.
mutual
  -- A KEYWORD's tag + params ([CR#702]) — the "name" side of a keyword. In this block so
  -- `HasKeyword` can read it and `Hexproof`'s "from" filter can be a `Predicate` (which may name
  -- an anaphor — "from the CHOSEN color"). `keyword` (Macros) desugars a spec into its full `Ability`
  -- (a `Composite`): the deontic ones (Flying/Defender/Shroud/Hexproof) get a `Cant`; the rest
  -- (FirstStrike/Deathtouch/Trample = damage; Vigilance = event-edit; Reach/Flash = flag/window)
  -- carry no clause. (Menace omitted: set-level.)
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
    Defender : KeywordSpec b
    Shroud : KeywordSpec b
    Menace : KeywordSpec b
    Hexproof : Maybe (Predicate b AnObject) -> KeywordSpec b   -- "from [filter]" — a SOURCE predicate (objects); "from a player" = ControlledBy that player
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
    -- the `ForEach`-bound element ("it"); its kind is the loop domain's (`itKind`).
    It : {auto prf : itKind b = Just k} -> Reference b k
    -- the triggering event's object ("that card"); gated by `eventBound` ([CR#608.2g]).
    EventObject : {auto prf : eventBound b = True} -> Reference b AnObject
    -- PLAYERS (the old `PlayerRef`, folded in here):
    You : Reference b APlayer                            -- controller of this ability [CR#109.5]
    ControllerOf : Reference b AnObject -> Reference b APlayer   -- the controller of an object
    OwnerOf : Reference b AnObject -> Reference b APlayer        -- the owner of an object [CR#108.3]
    EventActor : {auto prf : eventBound b = True} -> Reference b APlayer  -- the event's player ("that player")

  -- A numeric value ([CR#107.3]). `Literal` is a bare number; the rest read the game
  -- state — object counts, stats, counters, life/hand totals, event tallies, arithmetic.
  public export
  data Count : Bindings -> Type where
    Literal : Nat -> Count b                  -- a bare number
    X : Count b                               -- the chosen {X} value
    CountOf : Predicate b k -> Count b        -- how many entities match a predicate
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
  data Predicate : Bindings -> RefKind -> Type where
    HasType : Type_ -> Predicate b AnObject
    HasSupertype : Supertype -> Predicate b AnObject
    HasSubtype : Subtype -> Predicate b AnObject
    HasColor : Color -> Predicate b AnObject
    IsKind : ObjectKind -> Predicate b AnObject
    InZone : Zone -> Predicate b AnObject
    HasKeyword : KeywordSpec b -> Predicate b AnObject
    SameAs : Reference b k -> Predicate b k    -- the candidate IS r (same kind; "another" = IsNot (SameAs This))
    SameName : Reference b AnObject -> Predicate b AnObject   -- shares a name with r ("named [its own name]" = SameName This)
    WasCastFrom : Zone -> Predicate b AnObject -- the object was cast from this zone (cast provenance)
    ExiledBy : Reference b AnObject -> Predicate b AnObject   -- set aside by r's effect ("cards exiled by this" = ExiledBy
                                               -- This); the engine holds the association ([CR#607] linked abilities)
    HasName : String -> Predicate b AnObject   -- named a specific card (tutors / token names)
    HasCounter : CounterKind -> Predicate b AnObject   -- has ≥1 of this counter ("without a fate counter" = IsNot (HasCounter Fate))
    ControlledBy : Predicate b APlayer -> Predicate b AnObject   -- controller MATCHES a player-pred: "you control" = ControlledBy you, "an opponent controls" = ControlledBy opponent
    OwnedBy : Predicate b APlayer -> Predicate b AnObject
    WasKicked : Predicate b AnObject           -- FLAG: kicker as a boolean flag on the object (no cost-mode model)
    -- `Anyone` is the player top-predicate ("any player" — a person, hence `APlayer`).
    Anyone : Predicate b APlayer
    -- combinators (distinct from `Condition`'s And/Or/Not). `AllOf` (AND) is same-kind — a
    -- candidate is ONE kind, so all conjuncts share it. `OneOf` (OR/union) is HETEROGENEOUS:
    -- its arms may differ in kind and the result kind is their JOIN (`foldr (\/) Empty` over
    -- the arms' kinds), so a OneOf mixing object and player predicates is `Anything` — no
    -- `Widen`/`AnyOf`. "Any target" = `OneOf [creature…, Anyone]`; an empty `OneOf` is `Empty`.
    AllOf : List (Predicate b k) -> Predicate b k
    OneOf : {ks : List RefKind} -> All (Predicate b) ks -> Predicate b (foldr (\/) Empty ks)
    IsNot : Predicate b k -> Predicate b k     -- negation

  -- A CLOSED / game-state test ([CR#603.4]); reaches objects only via `Matches`
  -- (apply a `Predicate` to a named `Reference`) or `exists`/`unique` (below).
  public export
  data Condition : Bindings -> Type where
    Matches : Reference b k -> Predicate b k -> Condition b   -- does r satisfy the (same-kind) predicate
    Compare : Count b -> Cmp -> Count b -> Condition b
    TurnOf : Predicate b APlayer -> Condition b   -- it's a (matching) player's turn (`yourTurn = TurnOf (SameAs You)`)
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
    SourceMatches : Predicate b AnObject -> EventQuery b
    ActorIs       : Predicate b APlayer -> EventQuery b   -- the event's actor matches a player-pred (you / opponent)
    Within        : Window -> EventQuery b
    DuringStep    : PhaseStep -> EventQuery b
    DuringTurn    : Predicate b APlayer -> EventQuery b   -- the turn's player matches a player-pred
    Query  : List (EventQuery b) -> EventQuery b   -- AND
    Join   : List (EventQuery b) -> EventQuery b   -- OR
    Except : EventQuery b -> EventQuery b          -- NOT

  -- A cardinality spec for a choice ([CR#107.3]). In the mutual block so `Selection` can use it.
  public export
  data Quantity : Bindings -> Type where
    Range : Maybe (Count b) -> Maybe (Count b) -> Quantity b

  -- A resolution-time GROUP / choice. In the mutual block because `Single` (a `Reference`)
  -- demotes it. `GetTargets n` = the n-th target slot's targets (`GetTarget` demotes to one).
  public export
  data Selection : Bindings -> RefKind -> Type where
    SelectAll : Predicate b k -> Selection b k                  -- every match (a group)
    That : {auto prf : thatKind b = Just k} -> Selection b k    -- the `With`-bound group
    GetTargets : (n : Nat) -> {auto prf : InBounds n (targetKinds b)} -> Selection b (index n (targetKinds b))
    Random : Quantity b -> Predicate b k -> Selection b k
    TopOfLibrary : (count : Count b) -> {default You whose : Reference b APlayer} -> Selection b AnObject
    BottomOfLibrary : (count : Count b) -> {default You whose : Reference b APlayer} -> Selection b AnObject


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
between : Count b -> Count b -> Quantity b
between lo hi = Range (Just lo) (Just hi)

public export
anyNumber : Quantity b
anyNumber = Range Nothing Nothing

-- A target slot's `Quantity` must permit ≥1 target ([CR#115.1] — a slot can't target nothing).
-- Guards the UPPER bound: a statically-zero max ("up to 0") is rejected; "up to N>0" (lower 0) is fine.
public export
NonZeroTargets : Quantity b -> Type
NonZeroTargets (Range _ (Just (Literal Z))) = Void
NonZeroTargets _ = ()

public export
data TargetSpec : Bindings -> RefKind -> Type where
  -- a target slot: a NON-ZERO `Quantity` of targets matching the predicate (`Target (^1)` = one;
  -- `Target (between (^1) (^2))` = "one or two"). The slot's targets are `GetTargets n` (a group);
  -- `GetTarget n` demotes a single-target slot to a `Reference`.
  Target : (q : Quantity b) -> {auto 0 prf : NonZeroTargets q} -> Predicate b k -> TargetSpec b k

-- the n-th target as a single `Reference` (the common case) — sugar that demotes the slot's
-- targets via `Single`. A plural slot (`Target (between …)`) uses `GetTargets` directly.
public export
GetTarget : (n : Nat) -> {auto prf : InBounds n (targetKinds b)} -> Reference b (index n (targetKinds b))
GetTarget n = Single (GetTargets n)

-- "the unique object matching a predicate" — sugar: the sole element of `SelectAll p`.
public export
Only : Predicate b AnObject -> Reference b AnObject
Only p = Single (SelectAll p)

-- A cost paid to activate an ability ([CR#118,602]). `Costs` conjoins components;
-- `TapSelf`/`Sacrifice`/… read `This` (the ability's source). Rust: Cost.
public export
data Cost : Bindings -> Type where
  Mana      : ManaCost -> Cost b                 -- "{4}"
  TapSelf   : Cost b                             -- "{T}"
  UntapSelf : Cost b                             -- "{Q}"
  PayLife   : Count b -> Cost b                  -- "Pay N life"
  Sacrifice : Reference b AnObject -> Cost b              -- "Sacrifice this" = Sacrifice This
  AddCounters    : CounterKind -> Count b -> Cost b   -- a loyalty "+N" cost (put N counters on This)
  RemoveCounters : CounterKind -> Count b -> Cost b   -- a loyalty "−N" cost (remove N from This)
  Scaled    : Count b -> Cost b -> Cost b         -- the cost paid once per unit of the count ("{2} for each X" = Scaled (CountOf X) (Mana [promote 2]))
  Costs     : List (Cost b) -> Cost b            -- all components together

-- How many modes to choose, for a modal effect ([CR#700.2]). Rust: ChooseSpec.
public export
data ChooseSpec : Bindings -> Type where
  MkChooseSpec : (count : Count b) -> {default False upTo : Bool} -> {default False repeats : Bool} -> ChooseSpec b

-- A DEONTIC clause's carrier: a game ACTION a player may attempt ([CR#101.2,601.3] the deontic
-- layer) — distinct from the resolving `Action` verbs. Each names its participants; the CR's
-- "where ⟨pred⟩" qualifier rides the variable participant (`who`/`blocker`/`source`). The
-- polarities `Cant`/`Must`/`Gate`/`Toll` (in `StaticEffect`) wrap a `Deed`. BOUNDARY [CR#614.17]:
-- this is choice-LEGALITY ("can't attack"); event-edits ("doesn't tap", "can't be regenerated",
-- "can't lose") are `Replaces`/SBA, NOT a `Cant`.
public export
data Deed : Bindings -> Type where
  Attacks    : (who : Predicate b AnObject) -> {default Anyone whom : Predicate b APlayer} -> Deed b
  Blocks     : (blocker : Predicate b AnObject) -> (attacker : Predicate b AnObject) -> Deed b
  -- SET-LEVEL block ([CR#509.1c],[CR#702.111b]): "[attacker] is blocked by a DECLARED set of `size`
  -- creatures" (a block, so size ≥ 1 by construction). `Cant (BlockedBy This …)` constrains the
  -- WHOLE blocker set, not one blocker at a time — Menace = `Cant (BlockedBy (SameAs This) (^1))`
  -- (forbid the lone blocker; 0 = unblocked and 2+ stay legal). [CR#509.1c] judges the whole set.
  BlockedBy  : (attacker : Predicate b AnObject) -> (size : Quantity b) -> Deed b
  -- "[object] is targeted BY a source matching `by`"; `by` defaults to any spell or ability.
  BeTargeted : (object : Predicate b AnObject) -> {default (OneOf [IsKind IsSpell, IsKind IsAbility]) by : Predicate b AnObject} -> Deed b
  Casts      : (who : Predicate b APlayer) -> (what : Predicate b AnObject) -> Deed b
  Activates  : (who : Predicate b APlayer) -> (what : Predicate b AnObject) -> Deed b

-- One big mutual block: `Ability → OneShotEffect → Action → CreateToken → Characteristics` is a
-- cycle, so `Characteristics`/`Action`/`Bindable` join the effect/ability block below. (The leaf
-- `Cost`/`ChooseSpec`/`Deed` stay OUT — they only reach into block 1.)
mutual
  -- The printable CHARACTERISTICS of an object ([CR#109.3]) — shared by a card `Face`
  -- (`Characteristics Base`) and a created token (`Characteristics b`, so a token's P/T can be a
  -- `Count b`: "an X/X where X = [a value known at creation]"). `colors` is the explicit color (a
  -- color indicator [CR#107.4a] / a token's printed color); a card's color-FROM-MANA is derived.
  public export
  record Characteristics (b : Bindings) where
    constructor MkCharacteristics
    name : String
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

  -- The verbs ([CR#701]). `Effect::Act` wraps these. Object verbs carry an object
  -- `source` (default `This`); player verbs an `actor : Reference b APlayer` (default `You`).
  public export
  data Action : Bindings -> Type where
    -- deal damage to ONE recipient ([CR#120.1] — damage is to a single object/player per event);
    -- `source` object is the agent. "Deals N to EACH …" is a `ForEach` over the recipients.
    DealDamage : {default This source : Reference b AnObject} -> Reference b k -> Count b -> Action b
    -- "N damage divided as you choose among [a group]" ([CR#120.1] — Electrolyze). The split is the
    -- engine's; the grammar names the total and the recipient set (a `Selection`, ≥1 each).
    DealDamageDivided : {default This source : Reference b AnObject} -> Count b -> Selection b k -> Action b
    -- a plain zone change [CR#400.7]; owner-relative, control implicit.
    Move : Reference b AnObject -> Zone -> Action b
    -- exile a selection UNTIL a duration ends, then return it — the duration-bounded
    -- "exile until ~" form ([CR#603.6e]), NOT a leave-triggered return (see Oblivion Ring).
    ExileUntil : Reference b AnObject -> Duration b -> Action b
    -- destroy [CR#701.8] / counter a stack object [CR#701.6a]. (Return-to-hand is just
    -- `Move … Hand` — `Move` is owner-relative — so there's no dedicated bounce verb.)
    Destroy : Reference b AnObject -> Action b
    Counter : Reference b AnObject -> Action b
    -- tap / untap [CR#701.26]; attach / unattach [CR#701.3].
    Tap : Reference b AnObject -> Action b
    Untap : Reference b AnObject -> Action b
    Attach : (what : Reference b AnObject) -> (to : Reference b AnObject) -> Action b
    Unattach : Reference b AnObject -> Action b
    -- a player verb: the `actor` draws n cards. Rust: PlayerAction::Draw(Count).
    Draw : {default You actor : Reference b APlayer} -> Count b -> Action b
    -- the `actor` gains n life. Rust: PlayerAction::GainLife(Count).
    GainLife : {default You actor : Reference b APlayer} -> Count b -> Action b
    -- put a selection into its owner's library at a position ([CR#401]).
    PutIntoLibrary : Reference b AnObject -> LibraryPosition b -> Action b
    -- put / clear counters ([CR#122]). `RemoveAllCounters` clears every counter of a kind.
    PutCounters : CounterKind -> Count b -> Reference b AnObject -> Action b
    RemoveAllCounters : CounterKind -> Reference b AnObject -> Action b
    -- player verbs: discard / lose life; and a chooser-verb where a player sacrifices.
    Discard : {default You actor : Reference b APlayer} -> Count b -> Action b
    LoseLife : {default You actor : Reference b APlayer} -> Count b -> Action b
    Sacrifices : Reference b APlayer -> Predicate b AnObject -> Action b   -- "[player] sacrifices a [pred]" (they choose which)
    -- keyword actions / further verbs ([CR#701]). The interactive bits (reorder, search
    -- choice, copy characteristics) are the engine's; the grammar names the verb.
    Scry : Count b -> Action b                            -- look at top n, reorder / bottom some
    Surveil : Count b -> Action b
    Fight : (x : Reference b AnObject) -> (y : Reference b AnObject) -> Action b   -- each deals damage equal to its power to the other
    Reveal : Reference b AnObject -> Action b
    Shuffle : {default You actor : Reference b APlayer} -> Action b
    CreateToken : Count b -> Characteristics b -> Action b   -- the token's full characteristics (P/T may be a `Count b`)
    CopySpell : Reference b AnObject -> Action b                   -- "copy target spell" — FLAG: copy semantics deferred to engine
    AddMana : {default You actor : Reference b APlayer} -> ManaCost -> Action b   -- "add {G}" (mana ability effect); pool/paying is engine

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
    Modal : ChooseSpec b -> List (Mode b) -> OneShotEffect b
    -- "for each [domain], [body]" — binds each element as `It`. The distributive
    -- primitive (subsumes the old `Selection::Each`). Rust: Effect::ForEach.
    ForEach : Selection b k -> OneShotEffect (bindIt k b) -> OneShotEffect b
    -- "when you do [the preceding], [effect]" — a reflexive trigger. It NESTS, so
    -- `That`/targets stay in scope; no event-scanning sibling. Rust: Effect::Reflexive.
    Reflexive : OneShotEffect b -> OneShotEffect b
    -- schedule `body` for `event`; `unbindTargets` keeps `That`, drops targets. Rust: Effect::Delayed.
    Delayed : EventQuery b -> OneShotEffect (bindEvent (unbindTargets b)) -> OneShotEffect b

  -- one option of a modal effect: an effect plus an optional extra cost. Rust: Mode.
  public export
  data Mode : Bindings -> Type where
    MkMode : (effect : OneShotEffect b) -> {default Nothing cost : Maybe (Cost b)} -> Mode b

  -- A continuous modification a static ability applies to its subject.
  public export
  data Modification : Bindings -> Type where
    ModifyPT : Delta b -> Delta b -> Modification b     -- "gets +x/+y" (SIGNED, layer 7c — Up/Down)
    SetPT : Count b -> Count b -> Modification b         -- "base p/t are x/y" (layer 7b; x/y may be dynamic — CDA `*/*`)
    AddType : Type_ -> Modification b                   -- "is also a [type]"
    AddSubtype : Subtype -> Modification b              -- "becomes an Island" (adds the subtype)
    LoseAbilities : Modification b                      -- "loses all abilities" (Humility-style)
    GainControl : Reference b APlayer -> Modification b         -- "[player] gains control"
    GrantAbility : Ability b -> Modification b

  -- A continuous effect a static (or `Continuously`) ability generates ([CR#611]):
  -- modify a subject, modify a whole filter (anthem), or REPLACE an event — a
  -- replacement effect is a continuous effect too ([CR#614]). Rust: the StaticEffect family.
  public export
  data StaticEffect : Bindings -> Type where
    Modify : Reference b AnObject -> List (Modification b) -> StaticEffect b
    ModifyAll : Predicate b AnObject -> List (Modification b) -> StaticEffect b   -- anthem: "each [filter] gets [mods]"
    -- "if [event] would happen, do [effect] instead" — the card names only the
    -- replacement (empty = a pure skip); the engine skips the original + handles edges.
    Replaces : EventQuery b -> OneShotEffect (bindEvent b) -> StaticEffect b
    -- the inner continuous effect applies only WHILE the condition holds ([CR#604.3]) —
    -- a conditional static ("gets +1/+1 as long as …").
    While : Condition b -> StaticEffect b -> StaticEffect b
    -- DEONTIC clauses over a `Deed` (choice-legality, [CR#101.2]): the permission FLOOR (`Can`, the
    -- deontic "may" — named `Can` to pair with `Cant` and avoid the one-shot `May`), a can't, a
    -- must, or a cost-gate. The engine arbitrates Cant-beats-Can ([CR#101.2]); the grammar only
    -- records the clauses. `Gate`'s price is paid at declaration (never compulsory, [CR#508.1d]);
    -- `Toll`'s is punished downstream (ward, [CR#702.21a]). Cost comes FIRST. These gate CHOICES —
    -- the §6 sibling of `Replaces` (event-edits), never conflated with it.
    --  • `Can` — the permission floor made explicit ([CR#101.2,601.3]). A `Can (Casts …)` carries a
    --    `window`; Flash widens it to `InstantWindow` ([CR#702.8a] — a wider window, NOT an as-though).
    --  • `AsThough` — a scoped COUNTERFACTUAL premise ([CR#609.4]) wrapping a clause: "[clause]
    --    treated as though [condition] held." "attack as though it didn't have defender" =
    --    `AsThough (Matches This (IsNot (HasKeyword Defender))) (Can (Attacks (SameAs This)))`.
    -- (Window-NARROWING `Only` is the `window : TimingWindow` on `Activated` — `SorceryWindow`; the
    -- as-though of a deed-INTERNAL participant — "as though the BLOCKER's attacker lacked flying" — is still deferred.)
    Can  : Deed b -> {default Nothing window : Maybe TimingWindow} -> StaticEffect b
    AsThough : Condition b -> StaticEffect b -> StaticEffect b
    Cant : Deed b -> StaticEffect b
    Must : Deed b -> StaticEffect b
    Gate : Cost b -> Deed b -> StaticEffect b
    Toll : Cost b -> Deed b -> StaticEffect b

  -- A keyword as it sits on a permanent ([CR#702]): either `Bare` — an engine-PRIMITIVE keyword
  -- the grammar can't desugar (FirstStrike/DoubleStrike/Deathtouch/Trample = damage pipeline;
  -- Vigilance = attack event-edit) — or a `Composite` of its tag + the `Ability`s it desugars to:
  -- Flying/Defender/Shroud/Hexproof/Menace → a `Cant` (Menace's is SET-level, `BlockedBy`); Reach → `[]` (a flag flying's clause reads, no
  -- ability of its own); Flash → a `Can (Casts …) {window = InstantWindow}` (cast at instant speed).
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
    -- (instant by default; `SorceryWindow` = "activate only as a sorcery"); `limits` are the
    -- use-frequency caps. A loyalty ability is `{window = SorceryWindow, limits = [OncePerTurn]}`.
    Activated : Cost b -> OneShotEffect b -> {default InstantWindow window : TimingWindow} -> {default [] limits : List Restriction} -> Ability b
    -- a triggered ability: when `event` fires, resolve `effect`. Rust: Ability::Triggered.
    Triggered : EventQuery b -> OneShotEffect (bindEvent b) -> Ability b
    -- "Enchant <filter>": what this Aura may attach to ([CR#702.5]).
    Enchant : Predicate b AnObject -> Ability b
    -- a static continuous ability — modifications, anthems, AND replacements live in `StaticEffect`.
    Static : StaticEffect b -> Ability b

-- A card's printed face is just `Characteristics` at the empty bindings.
public export
Face : Type
Face = Characteristics Base

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
    { name = ""
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

public export
data Card : Type where
  Normal : (c : Characteristics Base) -> {auto ok : SubtypesOk c} -> Card

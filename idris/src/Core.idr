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

-- `Bindings`: the typestate of what references are in scope. Its fields are
-- PROJECTIONS we write constraints against; it grows as the model binds roles.
public export
record Bindings where
  constructor MkBindings
  targetCount  : Nat
  thatBound    : Bool   -- a `With`-bound group (`That`)
  itBound      : Bool   -- a `ForEach`-bound element (`It`)
  subjectBound : Bool   -- the candidate inside a filter (`Subject`)

-- The bindings a resolving spell starts in: nothing bound yet.
public export
Base : Bindings
Base = MkBindings 0 False False False

-- Record-update sugar keeps these indices INVERTIBLE for the unifier (so a
-- polymorphic macro's `b` is inferable at use sites). The discrete-binder proofs
-- (`thatBound`/`itBound`/target) only arise in concrete contexts and reduce there.
public export
bindTargets : Nat -> Bindings -> Bindings
bindTargets n b = { targetCount := n } b

public export
unbindTargets : Bindings -> Bindings
unbindTargets b = { targetCount := 0 } b

public export
bindThat : Bindings -> Bindings
bindThat b = { thatBound := True } b

public export
bindIt : Bindings -> Bindings
bindIt b = { itBound := True } b

-- A filter context supplies the candidate (`Subject`). Reconstructed explicitly
-- (not record-update sugar) so `subjectBound (bindSubject b)` reduces to True
-- DEFINITIONALLY even for abstract `b` — the gate proof the polymorphic macros need.
public export
bindSubject : Bindings -> Bindings
bindSubject b = MkBindings (targetCount b) (thatBound b) (itBound b) True

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

-- Reference / Count / Condition are one mutually recursive predicate language.
-- A *filter* is just a `Condition` with the candidate (`Subject`) in scope, tagged
-- by the `Filter`/`Where` newtype below. The handful of atoms that read a
-- `Reference` (`HasType`, `HasColor`, …) are the irreducible primitives;
-- combinators (`AllOf`/`OneOf`/`Not`) and `Exists` build the rest.
mutual
  -- A single GAME OBJECT. Player specifiers live in `PlayerRef`, not here.
  public export
  data Reference : Bindings -> Type where
    -- the source; always available — every spell/ability has one [CR#113.7].
    This : Reference b
    -- DEMANDS the bindings bound at least an (n+1)-th target.
    GetTarget : (n : Nat) -> {auto prf : ValidTarget n b} -> Reference b
    -- the unique object matching a predicate.
    Only : Condition (bindSubject b) -> Reference b
    -- the permanent R is attached to ("enchanted creature"); and its inverse.
    AttachHostOf : Reference b -> Reference b
    AttachedTo : Reference b -> Reference b
    -- the element bound by an enclosing `ForEach` ("it"). Gated by `itBound`.
    It : {auto prf : itBound b = True} -> Reference b
    -- the candidate being tested in a filter. GATED by `subjectBound` so a CLOSED
    -- `Condition` (a triggered intervening-if, `If`/`Unless`) can't use it. The
    -- gate is discharged in filter contexts by the `subjectBoundAfterBind` %hint.
    Subject : {auto prf : subjectBound b = True} -> Reference b

  -- A numeric value ([CR#107.3]). `Literal` is a bare number; the rest read the
  -- game state. (EventCount/EventSum/CounterCount/Min/ThatMuch deferred.)
  public export
  data Count : Bindings -> Type where
    Literal : Nat -> Count b                          -- a bare number
    X : Count b                                       -- the chosen {X} value
    CountOf : Condition (bindSubject b) -> Count b    -- how many objects match a predicate
    StatOf : Reference b -> Stat -> Count b           -- a referenced object's power/toughness

  -- THE predicate language ([CR#603.4]). A *filter* is a `Condition` with `Subject`
  -- bound (the `Filter` alias); a closed `Condition b` is an intervening-"if".
  public export
  data Condition : Bindings -> Type where
    -- atoms: read a referenced object (the handful of irreducible primitives).
    HasType : Reference b -> Type_ -> Condition b
    HasSubtype : Reference b -> Subtype -> Condition b
    HasColor : Reference b -> Color -> Condition b
    OfKind : Reference b -> ObjectKind -> Condition b
    InZone : Reference b -> Zone -> Condition b
    HasKeyword : Reference b -> KeywordAbility b -> Condition b
    SameObject : Reference b -> Reference b -> Condition b
    -- numeric, quantifier, and game-state predicates:
    Compare : Count b -> Cmp -> Count b -> Condition b
    Exists : Condition (bindSubject b) -> Condition b   -- ∃ object satisfying a predicate
    YourTurn : Condition b
    DuringPhase : PhaseStep -> Condition b
    -- combinators:
    AllOf : List (Condition b) -> Condition b
    OneOf : List (Condition b) -> Condition b
    Not : Condition b -> Condition b


-- A *filter* is a `Condition` with `Subject` in scope (`bindSubject b`), tagged.
-- Its sole constructor IS `Where` — exactly the old `Filter::Where` bridge: every
-- filter is a `Where` around a subject-condition. The newtype (vs a bare alias)
-- exists only so `b` is injectively inferable at use sites (the polymorphic macros).
-- A closed `Condition b` (a triggered intervening-if) is the same language sans `Subject`.
public export
data Filter : Bindings -> Type where
  Where : Condition (bindSubject b) -> Filter b

public export
unFilter : Filter b -> Condition (bindSubject b)
unFilter (Where c) = c

public export
implementation Cast Nat (Count b) where
  cast = Literal
public export
implementation Cast Integer (Count b) where
  cast = Literal . cast {to=Nat}

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

-- A PLAYER specifier (split out from `Reference`, which is objects-only).
public export
data PlayerRef : Bindings -> Type where
  You : PlayerRef b                            -- controller of this ability ([CR#109.5])
  Opponent : PlayerRef b                        -- an opponent ([CR#102.1]); single-opponent for now
  ControllerOf : Reference b -> PlayerRef b     -- the controller of a referenced object
  OwnerOf : Reference b -> PlayerRef b          -- the owner of a referenced object ([CR#108.3])

public export
data TargetSpec : Bindings -> Type where
  Target : Nat -> Filter b -> TargetSpec b

-- A resolution-time GROUP / choice. `Reference` is single-GameObject only, so
-- the plural anaphor lives HERE, not there. Mirrors Rust `Selection`.
public export
data Selection : Bindings -> Type where
  -- the matching objects as one set (Rust: Selection::Filter). A single object is
  -- just `SelectAll (IsRef r)`, so there's no dedicated ref-to-selection variant.
  SelectAll : Filter b -> Selection b
  -- the whole ordered group bound by an enclosing `With`. Rust: Selection::Those.
  That : {auto prf : thatBound b = True} -> Selection b
  -- a quantity of untargeted choices at resolution. Rust: Selection::Choose(Qty, Filter).
  SelectChoose : Quantity b -> Filter b -> Selection b
  -- (no distributive `Each` operand: distribution is `ForEach`, the set is `SelectAll`.)
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

-- What a binder (`With`) binds as `That`: a QUERY of existing objects, or a
-- PRODUCER — an `Action` run for effect, binding its product. The grammar only
-- names the role; the ENGINE resolves `That` to the live (reminted or gone)
-- object, so `MovedRef`/lki/became is a runtime concern, NOT modeled here.
public export
data Bindable : Bindings -> Type where
  Query   : Selection b -> Bindable b   -- bind existing objects (a plain selection)
  Produce : Action b -> Bindable b      -- run the action, bind its product (the moved object) as `That`

-- Trigger conditions a triggered/delayed ability waits for. Rust: the `Event` enum.
public export
data Event : Bindings -> Type where
  MovedTo : Zone -> Filter b -> Event b           -- matching object enters `Zone`
  BeginningOfEndStep : Event b                     -- "at the beginning of the next end step"
  OnStep : PhaseStep -> Event b
  PutIntoGraveyard : Filter b -> Event b           -- battlefield → graveyard ("dies"-style)

-- A continuous effect's lifetime ([CR#611.2]). Rust: Duration.
public export
data Duration : Bindings -> Type where
  UntilEndOfTurn : Duration b
  UntilEvent : Event b -> Duration b
  ForAsLongAs : Condition b -> Duration b
  Permanent : Duration b                       -- rest of game (Rust: EndOfGame)

-- How many modes to choose, for a modal effect ([CR#700.2]). Rust: ChooseSpec.
public export
data ChooseSpec : Bindings -> Type where
  Choose : (count : Count b) -> {default False upTo : Bool} -> {default False repeats : Bool} -> ChooseSpec b

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
    Delayed : Event b -> Effect (unbindTargets b) -> Effect b

  -- one option of a modal effect: an effect plus an optional extra cost. Rust: Mode.
  public export
  data Mode : Bindings -> Type where
    MkMode : (effect : Effect b) -> {default Nothing cost : Maybe ManaCost} -> Mode b

  -- A continuous modification a static ability applies to its subject.
  public export
  data Modification : Bindings -> Type where
    PlusPT : Int -> Int -> Modification b               -- "gets +x/+y"
    GrantAbility : Ability -> Modification b

  -- A static (continuous) effect: `subject` gets the modifications. Rust: Ability::Static.
  public export
  data StaticEffect : Bindings -> Type where
    Modify : Reference b -> List (Modification b) -> StaticEffect b

  -- A castable spell resolves in `Base`: source bound, no top-level targets.
  public export
  data Ability
    = Spell (Effect Base)
    | Keyword (KeywordAbility Base)
    -- a triggered ability: when `event` fires, resolve `effect`. Rust: Ability::Triggered.
    | Triggered (Event Base) (Effect Base)
    -- "Enchant <filter>": what this Aura may attach to. Rust: the Enchant keyword [CR#702.5].
    | Enchant (Filter Base)
    -- a static continuous ability. Rust: Ability::Static.
    | Static (StaticEffect Base)

public export
record Face where
  constructor MkFace
  name : String
  manaCost : ManaCost
  types : List Type_
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

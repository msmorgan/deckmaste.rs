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
  = Aura            -- enchantment type
public export
data ArtifactSubtype
  = Equipment       -- artifact type
public export
data LandSubtype
  = Island          -- land type
public export
data BattleSubtype
  = Siege           -- battle type

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

-- `Bindings` is a plain record so its fields are PROJECTIONS (targetCount, …)
-- we can write constraints against; it grows as the model binds more roles.
public export
record Bindings where
  constructor MkBindings
  targetCount : Nat
  thatBound   : Bool   -- is a `With`-bound group in scope? (gates `That`)

-- The bindings a resolving spell starts in: no targets, no group bound yet.
public export
Base : Bindings
Base = MkBindings 0 False

-- A binder PROVIDES n targets to the bindings its body runs in.
public export
bindTargets : Nat -> Bindings -> Bindings
bindTargets n b = { targetCount := n } b

public export
unbindTargets : Bindings -> Bindings
unbindTargets b = { targetCount := 0 } b

-- `With` PROVIDES a bound group (the `That` anaphor) to its body's bindings.
public export
bindThat : Bindings -> Bindings
bindThat b = { thatBound := True } b

-- "target n is a legal reference in bindings b". One place to enrich later
-- (e.g. class-matching once slots are typed); for now: the bindings bound enough.
public export
ValidTarget : Nat -> Bindings -> Type
ValidTarget n b = LTE (S n) (targetCount b)

mutual
  public export
  data Filter : Bindings -> Type where
    IsAll : (List (Filter b)) -> Filter b
    IsAny : (List (Filter b)) -> Filter b
    IsPlayer : Filter b
    IsInZone : Zone -> Filter b
    IsType : Type_ -> Filter b
    -- "requires at least one target bound" — a DEMAND, not a pin.
    IsTargeted : {auto prf : ValidTarget Z b} -> Filter b
    IsRef : Reference b -> Filter b
    -- negation, for "another" (≠ a referenced object). Rust: Filter::Not.
    IsNot : Filter b -> Filter b

  -- A single GAME OBJECT. Player specifiers live in `PlayerRef`, not here.
  public export
  data Reference : Bindings -> Type where
    -- the source; always available — every spell/ability has one [CR#113.7].
    This : Reference b
    -- polymorphic in b; DEMANDS the bindings bound at least an (n+1)-th target.
    GetTarget : (n : Nat) -> {auto prf : ValidTarget n b} -> Reference b
    Only : Filter b -> Reference b
    -- the permanent R is attached to — an Aura's host ("enchanted creature"). Rust: AttachHostOf.
    AttachHostOf : Reference b -> Reference b
    -- the attachment ON R — inverse of AttachHostOf. Rust: Reference::AttachedTo.
    AttachedTo : Reference b -> Reference b

-- A PLAYER specifier (split out from `Reference`, which is objects-only). Used
-- as the `actor` of player actions and as a controller/owner derivation.
public export
data PlayerRef : Bindings -> Type where
  You : PlayerRef b                            -- controller of this ability ([CR#109.5])
  Opponent : PlayerRef b                        -- an opponent ([CR#102.1]); single-opponent for now
  ControllerOf : Reference b -> PlayerRef b     -- the controller of a referenced object
  OwnerOf : Reference b -> PlayerRef b          -- the owner of a referenced object ([CR#108.3])

public export
data TargetSpec : Bindings -> Type where
  Target : Nat -> Filter b -> TargetSpec b

public export
data Stat = Power | Toughness

-- A numeric value ([CR#107.3]). `Literal` is a bare number; the rest read the
-- game state. (EventCount/EventSum/CounterCount/Min/ThatMuch deferred.)
public export
data Count : Bindings -> Type where
  Literal : Nat -> Count b                  -- a bare number
  X : Count b                               -- the chosen {X} value
  CountOf : Filter b -> Count b             -- cardinality of a filter match ("for each")
  StatOf : Reference b -> Stat -> Count b   -- a referenced object's power/toughness

public export
implementation Cast Nat (Count b) where
  cast = Literal
public export
implementation Cast Integer (Count b) where
  cast = Literal . cast {to=Nat}

-- A cardinality spec for a choice ([CR#107.3]). Rust: Quantity.
public export
data Quantity : Bindings -> Type where
  Exactly : Count b -> Quantity b
  AtLeast : Count b -> Quantity b
  AtMost : Count b -> Quantity b
  Between : Count b -> Count b -> Quantity b
  AnyNumber : Quantity b

public export
implementation Cast Integer (Quantity b) where
  cast = Exactly . Literal . cast {to=Nat}

-- A resolution-time GROUP / choice. `Reference` is single-GameObject only, so
-- the plural anaphor lives HERE, not there. Mirrors Rust `Selection`.
public export
data Selection : Bindings -> Type where
  SelectFilter : Filter b -> Selection b      -- the set matching a filter
  SelectRef : Reference b -> Selection b   -- one object as a singleton group (Rust: Selection::Ref)
  -- the whole ordered group bound by an enclosing `With`. Rust: Selection::Those.
  That : {auto prf : thatBound b = True} -> Selection b
  -- a quantity of untargeted choices at resolution. Rust: Selection::Choose(Qty, Filter).
  SelectChoose : Quantity b -> Filter b -> Selection b
  -- every matching object, one at a time — distributive "each". Rust: Selection::Each.
  Each : Filter b -> Selection b
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

public export
data Cmp = Equal | GreaterEq | LessEq | Greater | Less

-- A truth-valued test ([CR#603.4]-style intervening "if", etc.). Rust: Condition.
-- (Happened/LegallyAttached/DamagedByDeathtouch deferred — need Window/specifics.)
public export
data Condition : Bindings -> Type where
  Compare : Count b -> Cmp -> Count b -> Condition b   -- numeric test
  Exists : Filter b -> Condition b                      -- ≥1 object matches
  Is : Reference b -> Filter b -> Condition b           -- the referenced object matches
  YourTurn : Condition b
  DuringPhase : PhaseStep -> Condition b
  AllOf : List (Condition b) -> Condition b
  OneOf : List (Condition b) -> Condition b
  Not : Condition b -> Condition b

-- Trigger conditions a triggered/delayed ability waits for. Rust: the `Event`
-- enum (ZoneMove{to:Battlefield} for ETB; BeginningOf(Ending(End), …)).
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

-- A keyword ability ([CR#702]). Rust: Ability::Keyword(KeywordAbility).
public export
data KeywordAbility : Bindings -> Type where
  Flying : KeywordAbility b
  FirstStrike : KeywordAbility b
  DoubleStrike : KeywordAbility b
  Deathtouch : KeywordAbility b
  Reach : KeywordAbility b
  Trample : KeywordAbility b
  Vigilance : KeywordAbility b

-- Effects, continuous effects, and abilities are mutually recursive: a one-shot
-- can CREATE a continuous effect (`Continuously`), a static ability can grant an
-- ability, and an ability wraps an effect.
mutual
  public export
  data Effect : Bindings -> Type where
    Sequence : (List (Effect b)) -> Effect b
    Targeted : (Vect n (TargetSpec b)) -> Effect (bindTargets n b) -> Effect b
    -- binds `that` as `That` for `body`. `that` may PRODUCE a moved object (an
    -- Action), so "exile X, then act on That" is one binder. Rust: Effect::With.
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
    -- "when you do [the preceding], [effect]" — a reflexive trigger. It NESTS here,
    -- so `That`/targets stay in scope; no event-scanning sibling. Rust: Effect::Reflexive.
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

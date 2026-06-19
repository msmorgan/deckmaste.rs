||| Core grammar of the toy MTG card model: characteristics, the `Bindings`
||| typestate (what references are in scope), and the filter / reference /
||| selection / effect / ability trees. Kept deliberately brief — one file.
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

  public export
  data Reference : Bindings -> Type where
    -- the source; always available — every spell/ability has one [CR#113.7].
    This : Reference b
    -- polymorphic in b; DEMANDS the bindings bound at least an (n+1)-th target.
    GetTarget : (n : Nat) -> {auto prf : ValidTarget n b} -> Reference b
    Only : Filter b -> Reference b
    -- the permanent R is attached to — an Aura's host ("enchanted creature"). Rust: AttachHostOf.
    AttachHostOf : Reference b -> Reference b

public export
data TargetSpec : Bindings -> Type where
  Target : Nat -> Filter b -> TargetSpec b

-- A resolution-time GROUP / choice. `Reference` is single-GameObject only, so
-- the plural anaphor lives HERE, not there. Mirrors Rust `Selection`.
public export
data Selection : Bindings -> Type where
  SelectFilter : Filter b -> Selection b      -- the set matching a filter
  SelectRef : Reference b -> Selection b   -- one object as a singleton group (Rust: Selection::Ref)
  -- the whole ordered group bound by an enclosing `With`. Rust: Selection::Those.
  That : {auto prf : thatBound b = True} -> Selection b
  -- choose exactly n matching objects at resolution. Rust: Selection::Choose(Qty, Filter).
  SelectChoose : Nat -> Filter b -> Selection b

-- What a binder (`With`) binds as `That`: a QUERY of existing objects, or a
-- PRODUCER — a zone change run for effect, binding its product. The grammar only
-- names the role; the ENGINE resolves `That` to the live (reminted or gone)
-- object, so `MovedRef`/lki/became is a runtime concern, NOT modeled here.
public export
data Bindable : Bindings -> Type where
  Query   : Selection b -> Bindable b         -- bind existing objects (a plain selection)
  Produce : Selection b -> Zone -> Bindable b -- move the selection to Zone, bind the product as `That`

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

-- Trigger conditions a triggered/delayed ability waits for. Rust: the `Event`
-- enum (ZoneMove{to:Battlefield} for ETB; BeginningOf(Ending(End), …)).
public export
data Event : Bindings -> Type where
  MovedTo : Zone -> Filter b -> Event b           -- matching object enters `Zone`
  BeginningOfEndStep : Event b                     -- "at the beginning of the next end step"
  OnStep : PhaseStep -> Event b
  PutIntoGraveyard : Filter b -> Event b           -- battlefield → graveyard ("dies"-style)

public export
data Effect : Bindings -> Type where
  Sequence : (List (Effect b)) -> Effect b
  Targeted : (Vect n (TargetSpec b)) -> Effect (bindTargets n b) -> Effect b
  -- binds `that` as `That` for `body`. `that` may PRODUCE a moved object (a zone
  -- change), so "exile X, then act on That" is one binder. Rust: Effect::With.
  With : Bindable b -> Effect (bindThat b) -> Effect b
  -- damage goes to a `Selection` (a group), faithful to Rust `DealDamage(Selection, Count)`:
  -- a single target is `SelectRef (GetTarget n)`; "each creature" is `SelectFilter …`.
  Damage : {default This source : Reference b} -> (to : Selection b) -> Nat -> Effect b
  -- a plain zone change [CR#400.7]; destination is owner-relative, control implicit. Rust: Action::Move.
  Move : Selection b -> Zone -> Effect b
  -- schedule `body` for when `event` fires. `unbindTargets` clears the targets
  -- (stale post-move) but KEEPS bound anaphora like `That` — captured at registration;
  -- the engine decides whether the object is still findable. Rust: Effect::Delayed.
  Delayed : Event b -> Effect (unbindTargets b) -> Effect b
  -- a player draws n cards (implicit `You`). Rust: PlayerAction::Draw(Count).
  Draw : Nat -> Effect b

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

mutual
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

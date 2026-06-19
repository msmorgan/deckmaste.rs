module Main

import Data.Vect
import Data.Nat
import Data.List.Elem
import Data.List.Quantifiers

data Color
  = White
  | Blue
  | Black
  | Red
  | Green

data SimpleManaSymbol
  = Generic Nat
  | Specific (Maybe Color)

data ManaSymbol
  = Simple SimpleManaSymbol
  | Hybrid SimpleManaSymbol Color
  | Variable

implementation Cast Nat ManaSymbol where
  cast = Simple . Generic

implementation Cast Integer ManaSymbol where
  cast = cast . cast {to=Nat}

implementation Cast Color ManaSymbol where
  cast = Simple . Specific . Just

ManaCost : Type
ManaCost = List ManaSymbol

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
data CreatureSubtype
  = Bear | Rat | Spider | Human | Knight | Goblin | Elf | Zombie | Elemental  -- creature types
data EnchantmentSubtype
  = Aura            -- enchantment type
data ArtifactSubtype
  = Equipment       -- artifact type
data LandSubtype
  = Island          -- land type
data BattleSubtype
  = Siege           -- battle type

data Subtype
  = CreatureSub CreatureSubtype
  | EnchantmentSub EnchantmentSubtype
  | ArtifactSub ArtifactSubtype
  | LandSub LandSubtype
  | BattleSub BattleSubtype

implementation Cast CreatureSubtype Subtype where
  cast = CreatureSub
implementation Cast EnchantmentSubtype Subtype where
  cast = EnchantmentSub
implementation Cast LandSubtype Subtype where
  cast = LandSub
implementation Cast ArtifactSubtype Subtype where
  cast = ArtifactSub
implementation Cast BattleSubtype Subtype where
  cast = BattleSub

subtypeCategory : Subtype -> Type_
subtypeCategory (CreatureSub _) = Creature
subtypeCategory (EnchantmentSub _) = Enchantment
subtypeCategory (ArtifactSub _) = Artifact
subtypeCategory (LandSub _) = Land
subtypeCategory (BattleSub _) = Battle

-- The frame is a plain record so its fields are PROJECTIONS (targetCount, …)
-- we can write constraints against; it grows as the model binds more roles.
record Frame where
  constructor MkFrame
  targetCount : Nat
  thatBound   : Bool   -- is a `With`-bound group in scope? (gates `That`)

-- The frame a resolving spell starts in: no targets, no group bound yet.
base : Frame
base = MkFrame 0 False

-- A binder PROVIDES n targets to the frame its body runs in.
bindTargets : Nat -> Frame -> Frame
bindTargets n f = { targetCount := n } f

-- `With` PROVIDES a bound group (the `That` anaphor) to its body's frame.
bindThat : Frame -> Frame
bindThat f = { thatBound := True } f

-- "target n is a legal reference in frame f". One place to enrich later
-- (e.g. class-matching once slots are typed); for now: the frame bound enough.
ValidTarget : Nat -> Frame -> Type
ValidTarget n f = LTE (S n) (targetCount f)

mutual
  data Filter : Frame -> Type where
    IsAll : (List (Filter f)) -> Filter f
    IsAny : (List (Filter f)) -> Filter f
    IsPlayer : Filter f
    IsInZone : Zone -> Filter f
    IsType : Type_ -> Filter f
    -- "requires at least one target bound" — a DEMAND, not a pin.
    IsTargeted : {auto prf : ValidTarget Z f} -> Filter f
    IsRef : Reference f -> Filter f
    -- negation, for "another" (≠ a referenced object). Rust: Filter::Not.
    IsNot : Filter f -> Filter f

  data Reference : Frame -> Type where
    -- the source; always available — every spell/ability has one [CR#113.7].
    This : Reference f
    You : Reference f
    -- polymorphic in f; DEMANDS the frame bound at least an (n+1)-th target.
    GetTarget : (n : Nat) -> {auto prf : ValidTarget n f} -> Reference f
    Only : Filter f -> Reference f
    -- a single object remembered under a key by `Noting` ("that card"). Rust:
    -- Reference::Linked. Ungated — a runtime-resolved anaphor, not a positional binding.
    Linked : String -> Reference f
    -- the permanent R is attached to — an Aura's host ("enchanted creature"). Rust: AttachHostOf.
    AttachHostOf : Reference f -> Reference f

data TargetSpec : Frame -> Type where
  Target : Nat -> Filter f -> TargetSpec f

-- A resolution-time GROUP / choice. `Reference` is single-GameObject only, so
-- the plural anaphor lives HERE, not there. Mirrors Rust `Selection`.
data Selection : Frame -> Type where
  SelectFilter : Filter f -> Selection f      -- the set matching a filter
  SelectRef : Reference f -> Selection f   -- one object as a singleton group (Rust: Selection::Ref)
  -- the whole ordered group bound by an enclosing `With`. Rust: Selection::Those.
  That : {auto prf : thatBound f = True} -> Selection f
  -- choose exactly n matching objects at resolution. Rust: Selection::Choose(Qty, Filter).
  SelectChoose : Nat -> Filter f -> Selection f

-- Trigger conditions a triggered/delayed ability waits for. Rust: the `Event`
-- enum (ZoneMove{to:Battlefield} for ETB; BeginningOf(Ending(End), …)).
data Event
  = Enters (Filter Main.base)   -- something matching the filter enters the battlefield
  | BeginningOfEndStep     -- "at the beginning of the next end step"
  | PutIntoGraveyard (Filter Main.base)  -- matching object goes battlefield → graveyard ("dies"-style)

data Effect : Frame -> Type where
  Sequence : (List (Effect f)) -> Effect f
  Targeted : (Vect n (TargetSpec f)) -> Effect (bindTargets n f) -> Effect f
  -- binds the WHOLE `selection` as `That` for `body`; never distributes. Rust: Effect::With.
  With : Selection f -> Effect (bindThat f) -> Effect f
  -- damage goes to a `Selection` (a group), faithful to Rust `DealDamage(Selection, Count)`:
  -- a single target is `SelectRef (GetTarget n)`; "each creature" is `SelectFilter …`.
  Damage : (source : Reference f) -> (to : Selection f) -> Nat -> Effect f
  -- a plain zone change [CR#400.7]; destination is owner-relative, control implicit. Rust: Action::Move.
  Move : Selection f -> Zone -> Effect f
  -- run `effect`, remembering the objects it moved under `key` (for a later `Linked`). Rust: Effect::Noting.
  Noting : String -> Effect f -> Effect f
  -- schedule `body` for when `event` fires. Body runs in a FRESH `base` frame: the
  -- current targets do NOT survive — only `Linked` anaphora cross. Rust: Effect::Delayed.
  Delayed : Event -> Effect Main.base -> Effect f
  -- a player draws n cards (implicit `You`). Rust: PlayerAction::Draw(Count).
  Draw : Nat -> Effect f

-- A keyword ability ([CR#702]). Rust: Ability::Keyword(KeywordAbility).
data KeywordAbility
  = Flying
  | FirstStrike
  | DoubleStrike
  | Deathtouch
  | Reach
  | Trample
  | Vigilance

-- A continuous modification a static ability applies to its subject.
data Modification
  = PlusPT Int Int               -- "gets +x/+y"
  | GrantsKeyword KeywordAbility  -- "has <keyword>"

-- A static (continuous) effect: `subject` gets the modifications. Rust: Ability::Static.
data StaticEffect : Frame -> Type where
  Affecting : Reference f -> List Modification -> StaticEffect f

-- A castable spell resolves in `base`: source bound, no top-level targets.
-- (qualified `Main.base` so it isn't auto-bound as a fresh implicit)
data Ability
  = Spell (Effect Main.base)
  | Keyword KeywordAbility
  -- a triggered ability: when `event` fires, resolve `effect`. Rust: Ability::Triggered.
  | Triggered Event (Effect Main.base)
  -- "Enchant <filter>": what this Aura may attach to. Rust: the Enchant keyword [CR#702.5].
  | Enchant (Filter Main.base)
  -- a static continuous ability. Rust: Ability::Static.
  | Static (StaticEffect Main.base)

permanent : Filter f
permanent = IsInZone Battlefield

anyTarget : TargetSpec f
anyTarget = Target 1 $ IsAny
  [ IsPlayer
  , IsAll [permanent, IsType Battle]
  , IsAll [permanent, IsType Creature]
  , IsAll [permanent, IsType Planeswalker]
  ]

playerOrPlaneswalker : TargetSpec f
playerOrPlaneswalker = Target 1 $ IsAny
  [ IsPlayer
  , IsAll [permanent, IsType Planeswalker]
  ]

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

interface DefaultValue a where
  defaultValue : a

fromDefault : (DefaultValue a) => (a -> a) -> a
fromDefault f = f defaultValue

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
SubtypesOk : Face -> Type
SubtypesOk f = All (\s => Elem (subtypeCategory s) (types f)) (subtypes f)

data Card : Type where
  Normal : (f : Face) -> {auto ok : SubtypesOk f} -> Card

LightningBolt : Card
LightningBolt = Normal $ fromDefault
  { name := "Lightning Bolt"
  , manaCost := [cast Red]
  , types := [Instant]
  , abilities :=
      [ Spell (Targeted [anyTarget]
          (Damage This (SelectRef (GetTarget 0)) 3)
        )
      ]
  }

-- Vanilla creature: no abilities, just power/toughness. No new data variant.
GrizzlyBears : Card
GrizzlyBears = Normal $ fromDefault
  { name := "Grizzly Bears"
  , manaCost := [cast 1, cast Green]
  , types := [Creature]
  , subtypes := [cast Bear]
  , power := Just 2
  , toughness := Just 2
  }

-- French vanilla: a single keyword ability.
TyphoidRats : Card
TyphoidRats = Normal $ fromDefault
  { name := "Typhoid Rats"
  , manaCost := [cast Black]
  , types := [Creature]
  , subtypes := [cast Rat]
  , abilities := [Keyword Deathtouch]
  , power := Just 1
  , toughness := Just 1
  }

GiantSpider : Card
GiantSpider = Normal $ fromDefault
  { name := "Giant Spider"
  , manaCost := [cast 3, cast Green]
  , types := [Creature]
  , subtypes := [cast Spider]
  , abilities := [Keyword Reach]
  , power := Just 2
  , toughness := Just 4
  }

-- Untargeted group damage: `Damage` to a `SelectFilter`, no `Targeted` wrapper.
Pyroclasm : Card
Pyroclasm = Normal $ fromDefault
  { name := "Pyroclasm"
  , manaCost := [cast 1, cast Red]
  , types := [Sorcery]
  , abilities :=
      [ Spell (Damage This (SelectFilter (IsType Creature)) 2)
      ]
  }

-- TRICKY: ETB trigger that exiles "another target permanent" (a battlefield
-- object that is not This), then a DELAYED trigger returns it next end step.
-- The delayed body runs in a fresh `base` frame — the target is gone; only the
-- `Linked "exiled"` anaphor crosses (mirrors the "returns as a new object" ruling).
Flickerwisp : Card
Flickerwisp = Normal $ fromDefault
  { name := "Flickerwisp"
  , manaCost := [cast 1, cast White, cast White]
  , types := [Creature]
  , subtypes := [cast Elemental]
  , abilities :=
      [ Keyword Flying
      , Triggered (Enters (IsRef This)) $
          Targeted [Target 1 (IsAll [permanent, IsNot (IsRef This)])] $
            Sequence
              [ Noting "exiled" (Move (SelectRef (GetTarget 0)) Exile)
              , Delayed BeginningOfEndStep
                  (Move (SelectRef (Linked "exiled")) Battlefield)
              ]
      ]
  , power := Just 3
  , toughness := Just 1
  }

-- TRICKY: Draw, then choose two cards from hand and move them onto the library.
-- Faithful to Rust `Sequence([Draw(3), PutInLibrary(Choose(2, hand), top)])`
-- (no `With`/`That` — Brainstorm uses a `Choose`, not the ordered anaphor).
Brainstorm : Card
Brainstorm = Normal $ fromDefault
  { name := "Brainstorm"
  , manaCost := [cast Blue]
  , types := [Instant]
  , abilities :=
      [ Spell $ Sequence
          [ Draw 3
          , Move (SelectChoose 2 (IsInZone Hand)) Library
          ]
      ]
  }


-- TRICKY: an Aura. `Enchant` says what it attaches to; a `Static` ability buffs
-- the host (`AttachHostOf This`) with +2/+0 and trample; a graveyard trigger
-- returns it to hand. (Aura is an enchantment subtype, so it's allowed here.)
Rancor : Card
Rancor = Normal $ fromDefault
  { name := "Rancor"
  , manaCost := [cast Green]
  , types := [Enchantment]
  , subtypes := [cast Aura]
  , abilities :=
      [ Enchant (IsType Creature)
      , Static (Affecting (AttachHostOf This) [PlusPT 2 0, GrantsKeyword Trample])
      , Triggered (PutIntoGraveyard (IsRef This)) (Move (SelectRef This) Hand)
      ]
  }

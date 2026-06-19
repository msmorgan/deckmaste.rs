||| Card encodings. Each is a `Core.Card` built with the `fromDefault { … }`
||| builder, using `Core` constructors and `Macros` templates. Verbs go through
||| `Act` (the verb compartment); a binder that produces a moved object wraps
||| the producing `Action` in `Produce`.
module Cards

import Core
import Macros

export
LightningBolt : Card
LightningBolt = Normal $ fromDefault
  { name := "Lightning Bolt"
  , manaCost := [cast Red]
  , types := [Instant]
  , abilities :=
      [ Spell (Targeted [anyTarget]
          (Act (DealDamage (SelectAll (isRef (GetTarget 0))) (cast 3)))
        )
      ]
  }

-- Vanilla creature: no abilities, just power/toughness. No new data variant.
export
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
export
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

export
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

-- Untargeted group damage: `DealDamage` to a `SelectAll`, no `Targeted`.
export
Pyroclasm : Card
Pyroclasm = Normal $ fromDefault
  { name := "Pyroclasm"
  , manaCost := [cast 1, cast Red]
  , types := [Sorcery]
  , abilities :=
      [ Spell (Act (DealDamage (SelectAll (creature)) (cast 2)))
      ]
  }

-- TRICKY: ETB trigger exiles "another target permanent", binding it as `That`; a
-- DELAYED trigger returns `That` next end step. `unbindTargets` drops the target
-- (stale post-move) but KEEPS the captured `That` — no key, no MovedRef. The
-- engine resolves `That` to the reminted (or gone) object [CR#400.7].
export
Flickerwisp : Card
Flickerwisp = Normal $ fromDefault
  { name := "Flickerwisp"
  , manaCost := [cast 1, cast White, cast White]
  , types := [Creature]
  , subtypes := [cast Elemental]
  , abilities :=
      [ Keyword Flying
      , Triggered (MovedTo Battlefield (isRef This)) $
          Targeted [Target 1 (allF [permanent, notF (isRef This)])] $
            With (Produce (Move (SelectAll (isRef (GetTarget 0))) Exile)) $  -- exile the target, bind `That`
              Delayed BeginningOfEndStep
                (Act (Move That Battlefield))                        -- return `That` (captured; target gone)
      ]
  , power := Just 3
  , toughness := Just 1
  }

-- TRICKY: Draw, then choose two cards from hand and move them onto the library.
-- Faithful to Rust `Sequence([Draw(3), PutInLibrary(Choose(2, hand), top)])`
-- (no `With`/`That` — Brainstorm uses a `Choose`, not the ordered anaphor).
export
Brainstorm : Card
Brainstorm = Normal $ fromDefault
  { name := "Brainstorm"
  , manaCost := [cast Blue]
  , types := [Instant]
  , abilities :=
      [ Spell $ Sequence
          [ Act (Draw (cast 3))
          , Act (Move (Choose (cast 2) (inHand)) Library)
          ]
      ]
  }

-- TRICKY: an Aura. `Enchant` says what it attaches to; a `Static` ability buffs
-- the host (`AttachHostOf This`) with +2/+0 and trample; a graveyard trigger
-- returns it to hand. (Aura is an enchantment subtype, so it's allowed here.)
export
Rancor : Card
Rancor = Normal $ fromDefault
  { name := "Rancor"
  , manaCost := [cast Green]
  , types := [Enchantment]
  , subtypes := [cast Aura]
  , abilities :=
      [ Enchant (allF [permanent, creature])
      , Static (Modify (AttachHostOf This)
          [ PlusPT 2 0
          , GrantAbility (Keyword Trample)
          ])
      , Triggered
          (PutIntoGraveyard (isRef This))
          (Act (Move (SelectAll (isRef This)) Hand))
      ]
  }

-- TRICKY: Cloudshift — exile→return in ONE resolution (the pure [CR#400.7j] case).
-- `With (Produce (Move …))` binds the exiled object as `That`; the body returns it.
export
Cloudshift : Card
Cloudshift = Normal $ fromDefault
  { name := "Cloudshift"
  , manaCost := [cast White]
  , types := [Instant]
  , abilities :=
      [ Spell $ Targeted [Target 1 (allF [permanent, creature])] $
          With (Produce (Move (SelectAll (isRef (GetTarget 0))) Exile)) $
            Act (Move That Battlefield)
      ]
  }

-- TRICKY: Through the Breach — put a creature onto the battlefield (binding it as
-- `That`), then a DELAYED trigger sacrifices `That` at the next end step. The
-- captured `That` is the acceptance test: if the engine can't still find the
-- object at fire time (it was blinked away), the sacrifice does nothing.
-- ("may" and "gains haste" omitted for focus.)
export
ThroughTheBreach : Card
ThroughTheBreach = Normal $ fromDefault
  { name := "Through the Breach"
  , manaCost := [cast 4, cast Red]
  , types := [Instant]
  , abilities :=
      [ Spell $
          With (Produce (Move (Choose (cast 1) (allF [inHand, creature])) Battlefield)) $
            Delayed BeginningOfEndStep
              (Act (Move That Graveyard))
      ]
  }

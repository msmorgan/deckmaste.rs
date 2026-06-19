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
          (Act (DealDamage (SelectAll (SameAs (GetTarget 0))) (cast 3)))
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
      , Triggered (Query [KindIs (ZoneChanged Nothing (Just Battlefield)), SourceMatches (SameAs This)]) $
          Targeted [Target 1 (AllOf [permanent, IsNot (SameAs This)])] $
            With (Produce (Move (SelectAll (SameAs (GetTarget 0))) Exile)) $  -- exile the target, bind `That`
              Delayed nextEndStep
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
      [ Enchant (AllOf [permanent, creature])
      , Static (Modify (AttachHostOf This)
          [ PlusPT 2 0
          , GrantAbility (Keyword Trample)
          ])
      , Triggered
          (Query [KindIs (ZoneChanged (Just Battlefield) (Just Graveyard)), SourceMatches (SameAs This)])
          (Act (Move (SelectAll (SameAs This)) Hand))
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
      [ Spell $ Targeted [Target 1 (AllOf [permanent, creature])] $
          With (Produce (Move (SelectAll (SameAs (GetTarget 0))) Exile)) $
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
          With (Produce (Move (Choose (cast 1) (AllOf [inHand, creature])) Battlefield)) $
            Delayed nextEndStep
              (Act (Move That Graveyard))
      ]
  }

-- TRICKY: Approach of the Second Sun — an alternate WIN CONDITION gated on game
-- history. `EventCount` (log-derived) counts this game's prior casts of this same
-- spell; ≥2 (the current cast is itself logged) ⇒ you win. Otherwise burrow it 7th
-- from the top and gain 7. Exercises Outcome / EventQuery / SameName / WasCastFrom /
-- positional library / GainLife.
export
ApproachOfTheSecondSun : Card
ApproachOfTheSecondSun = Normal $ fromDefault
  { name := "Approach of the Second Sun"
  , manaCost := [cast 6, cast White]
  , types := [Sorcery]
  , abilities :=
      [ Spell $
          If (And [ Matches This (WasCastFrom Hand)
                  , Compare (EventCount (Query [ KindIs Cast
                                               , ActorIs You
                                               , SourceMatches (SameName This)
                                               , Within ThisGame ]))
                            GreaterEq (Literal 2) ])
             (Conclude (WinGame You))
             { otherwise = Just (Sequence
                 [ Act (PutIntoLibrary (SelectAll (SameAs This)) (FromTop (cast 6)))
                 , Act (GainLife (cast 7)) ]) }
      ]
  }

-- Two ACTIVATED abilities + a cost algebra + counters. `{4},{T}:` fate-counter a
-- permanent; `{5},{T},Sacrifice this:` wrath everything not fate-protected, then
-- clear the counters. Exercises Activated / Cost (mana+tap+sacrifice) / CounterKind /
-- HasCounter / PutCounters / RemoveAllCounters.
export
OblivionStone : Card
OblivionStone = Normal $ fromDefault
  { name := "Oblivion Stone"
  , manaCost := [cast 3]
  , types := [Artifact]
  , abilities :=
      [ Activated (Costs [Mana [cast 4], TapSelf])
          (Targeted [Target 1 permanent]
            (Act (PutCounters Fate (Literal 1) (SelectAll (SameAs (GetTarget 0))))))
      , Activated (Costs [Mana [cast 5], TapSelf, Sacrifice (SelectAll (SameAs This))])
          (Sequence
            [ Act (Destroy (SelectAll (AllOf [permanent, IsNot (HasType Land), IsNot (HasCounter Fate)])))
            , Act (RemoveAllCounters Fate (SelectAll permanent)) ])
      ]
  }

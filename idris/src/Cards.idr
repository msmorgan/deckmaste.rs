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

-- TRICKY: Draw 3, then choose two cards from hand and put them on the library. The
-- `Choose` is a `Bindable` — it binds the chosen cards as `That`; the body moves `That`.
export
Brainstorm : Card
Brainstorm = Normal $ fromDefault
  { name := "Brainstorm"
  , manaCost := [cast Blue]
  , types := [Instant]
  , abilities :=
      [ Spell $ Sequence
          [ Act (Draw (cast 3))
          , With (Choose (cast 2) inHand) (Act (Move That Library))
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
          With (Choose (cast 1) (AllOf [inHand, creature])) $
            Sequence
              [ Act (Move That Battlefield)
              , Delayed nextEndStep (Act (Move That Graveyard)) ]
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

-- A clean ANTHEM: a static `ModifyAll` over "creatures you control". Exercises
-- ModifyAll + ControlledBy (the controller predicate).
export
GloriousAnthem : Card
GloriousAnthem = Normal $ fromDefault
  { name := "Glorious Anthem"
  , manaCost := [cast 1, cast White, cast White]
  , types := [Enchantment]
  , abilities :=
      [ Static (ModifyAll (AllOf [HasType Creature, ControlledBy You]) [PlusPT 1 1]) ]
  }

-- Liliana of the Veil — the "planeswalkers are pure composite" thesis: loyalty
-- abilities are Activated abilities whose cost adds/removes Loyalty counters, and the
-- printed loyalty (3) is "enters with 3 Loyalty counters" (Face.loyalty).
-- FLAGS: the once-per-turn / sorcery-speed use-limit is NOT modeled; "each player" and
-- "target player" use the dubious EachPlayer / TargetedPlayer; the −6 pile ultimate is
-- OMITTED (unrepresentable without pile-division); the "Liliana" planeswalker subtype
-- is omitted (no planeswalker-subtype enum).
export
LilianaOfTheVeil : Card
LilianaOfTheVeil = Normal $ fromDefault
  { name := "Liliana of the Veil"
  , manaCost := [cast 1, cast Black, cast Black]
  , types := [Planeswalker]
  , supertypes := [Legendary]
  , loyalty := Just 3
  , abilities :=
      [ Activated (AddCounters Loyalty (Literal 1))
          (Act (Discard {actor = EachPlayer} (cast 1)))
      , Activated (RemoveCounters Loyalty (Literal 2))
          (Targeted [Target 1 (IsKind IsPlayerKind)]
            (Act (Sacrifices (TargetedPlayer 0) creature)))
      ]
  }

-- Tide Shaper — layers + kicker, heavily flagged. The kicked ETB makes a target land
-- an Island for a duration (AddSubtype + ForAsLongAs). FLAGS: kicker is the WasKicked
-- boolean (no cost-mode model); the "+1/+1 as long as an opponent controls an Island"
-- static is OMITTED (no conditional statics); Merfolk/Wizard subtypes omitted.
export
TideShaper : Card
TideShaper = Normal $ fromDefault
  { name := "Tide Shaper"
  , manaCost := [cast Blue]
  , types := [Creature]
  , abilities :=
      [ Triggered (Query [KindIs (ZoneChanged Nothing (Just Battlefield)), SourceMatches (SameAs This)])
          (If (Matches This WasKicked)
              (Targeted [Target 1 (HasType Land)]
                (Continuously (Modify (GetTarget 0) [AddSubtype (cast Island)])
                              (ForAsLongAs (Matches This (InZone Battlefield))))))
      ]
  , power := Just 1
  , toughness := Just 1
  }

-- Necropotence — "skip your draw step" is a replacement whose effect is nothing (the
-- engine skips the step); the discard trigger exiles the discarded card; the pay-life
-- ability draws into exile, deferred to your end step. FLAGS (grammar, not engine): the
-- "that card" anaphora uses the UNGATED `EventObject`; "face down" isn't modeled.
export
Necropotence : Card
Necropotence = Normal $ fromDefault
  { name := "Necropotence"
  , manaCost := [cast Black, cast Black, cast Black]
  , types := [Enchantment]
  , abilities :=
      [ Static (Replaces (Query [KindIs (BeginStep (BeginningPhase DrawStep)), DuringTurn You]) (Sequence []))
      , Triggered (Query [KindIs Discarded, ActorIs You])
          (Act (Move (SelectAll (SameAs EventObject)) Exile))
      , Activated (PayLife (Literal 1))
          (With (Produce (Move (TopOfLibrary (Literal 1)) Exile))
            (Delayed nextEndStep (Act (Move That Hand))))
      ]
  }

-- Notion Thief — replace an opponent's draw with "you draw a card" instead; the card
-- just names the replacement, the engine handles the opponent skipping their draw.
-- FLAG (grammar): "except the first draw each draw step" is approximated as `Except
-- (DuringStep draw-step)` — we have no ordinal facet to say "the first".
export
NotionThief : Card
NotionThief = Normal $ fromDefault
  { name := "Notion Thief"
  , manaCost := [cast 2, cast Blue, cast Black]
  , types := [Creature]
  , abilities :=
      [ Keyword Flash
      , Static (Replaces (Query [ KindIs Drew, ActorIs Opponent
                                , Except (Query [DuringStep (BeginningPhase DrawStep)]) ])
          (Act (Draw {actor = You} (cast 1))))
      ]
  , power := Just 3
  , toughness := Just 1
  }

-- Banishing Light — the O-Ring "exile until this leaves" pattern, fully representable
-- with no new machinery. The ETB exiles a target (an opponent's nonland permanent, via
-- the ControlledBy predicate) and binds it as `That`; a DELAYED trigger keyed on THIS
-- leaving the battlefield returns `That`. The "until" is just a Delayed on the
-- leave-event — same Produce/That/Delayed shape as Flickerwisp.
export
BanishingLight : Card
BanishingLight = Normal $ fromDefault
  { name := "Banishing Light"
  , manaCost := [cast 2, cast White]
  , types := [Enchantment]
  , abilities :=
      [ Triggered (Query [KindIs (ZoneChanged Nothing (Just Battlefield)), SourceMatches (SameAs This)]) $
          Targeted [Target 1 (AllOf [permanent, IsNot (HasType Land), ControlledBy Opponent])] $
            With (Produce (Move (SelectAll (SameAs (GetTarget 0))) Exile)) $
              Delayed (Query [KindIs (ZoneChanged (Just Battlefield) Nothing), SourceMatches (SameAs This)])
                (Act (Move That Battlefield))
      ]
  }

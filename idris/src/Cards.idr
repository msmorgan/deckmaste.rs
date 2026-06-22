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
  , manaCost := [^Red]
  , types := [Instant]
  , abilities :=
      [ Spell $
          Targeted [anyTarget] $
          (Act (DealDamage (GetTarget 0) (^3)))
      ]
  }

-- Untargeted group damage: a `ForEach` over all creatures, dealing to each `It` (no `Targeted`).
export
Pyroclasm : Card
Pyroclasm = Normal $ fromDefault
  { name := "Pyroclasm"
  , manaCost := [^1, ^Red]
  , types := [Sorcery]
  , abilities :=
      [ Spell (ForEach (SelectAll creature) (Act (DealDamage It (^2))))
      ]
  }

-- Vanilla creature: no abilities, just power/toughness. No new data variant.
export
GrizzlyBears : Card
GrizzlyBears = Normal $ fromDefault
  { name := "Grizzly Bears"
  , manaCost := [^1, ^Green]
  , types := [Creature]
  , subtypes := [^Bear]
  , power := Just 2
  , toughness := Just 2
  }

-- French vanilla: a single keyword ability.
export
TyphoidRats : Card
TyphoidRats = Normal $ fromDefault
  { name := "Typhoid Rats"
  , manaCost := [^Black]
  , types := [Creature]
  , subtypes := [^Rat]
  , abilities := [keyword Deathtouch]
  , power := Just 1
  , toughness := Just 1
  }

export
GiantSpider : Card
GiantSpider = Normal $ fromDefault
  { name := "Giant Spider"
  , manaCost := [^3, ^Green]
  , types := [Creature]
  , subtypes := [^Spider]
  , abilities := [keyword Reach]
  , power := Just 2
  , toughness := Just 4
  }

-- TRICKY: ETB trigger exiles "another target permanent", binding it as `That`; a
-- DELAYED trigger returns `That` next end step. `unbindTargets` drops the target
-- (stale post-move) but KEEPS the captured `That` — no key, no MovedRef. The
-- engine resolves `That` to the reminted (or gone) object [CR#400.7].
export
Flickerwisp : Card
Flickerwisp = Normal $ fromDefault
  { name := "Flickerwisp"
  , manaCost := [^1, ^White, ^White]
  , types := [Creature]
  , subtypes := [^Elemental]
  , abilities :=
      [ keyword Flying
      , Triggered (Query [KindIs (ZoneChanged Nothing (Just Battlefield)), SourceMatches (SameAs This)]) $
          Targeted [Target (^1) (AllOf [permanent, IsNot (SameAs This)])] $
            With (Produce (Move ((GetTarget 0)) Exile)) $  -- exile the target, bind `That`
              Delayed nextEndStep
                (ForEach That (Act (Move It Battlefield)))                        -- return `That` (captured; target gone)
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
  , manaCost := [^Blue]
  , types := [Instant]
  , abilities :=
      [ Spell $
          Sequence
          [ Act (Draw (^3))
          , With (Choose (^2) inHand) (ForEach That (Act (Move It Library)))
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
  , manaCost := [^Green]
  , types := [Enchantment]
  , subtypes := [^Aura]
  , abilities :=
      [ Enchant (AllOf [permanent, creature])
      , Static (Modify (AttachHostOf This)
          [ ModifyPT (^2) (^0)
          , GrantAbility (keyword Trample)
          ])
      , Triggered
          (Query [ KindIs (ZoneChanged (Just Battlefield) (Just Graveyard))
                 , SourceMatches (SameAs This)
                 ])
          (Act (Move (This) Hand))
      ]
  }

-- TRICKY: Cloudshift — exile→return in ONE resolution (the pure [CR#400.7j] case).
-- `With (Produce (Move …))` binds the exiled object as `That`; the body returns it.
export
Cloudshift : Card
Cloudshift = Normal $ fromDefault
  { name := "Cloudshift"
  , manaCost := [^White]
  , types := [Instant]
  , abilities :=
      [ Spell $
          Targeted [ Target (^1) (AllOf [ permanent, creature
                                     , ControlledBy you
                                     ])
                   ] $
          With (Produce (Move ((GetTarget 0)) Exile)) $
          ForEach That (Act (Move It Battlefield))
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
  , manaCost := [^4, ^Red]
  , types := [Instant]
  , abilities :=
      [ Spell $
          With (Choose (^1) (AllOf [inHand, creature])) $
            Sequence
              [ ForEach That (Act (Move It Battlefield))
              , Delayed nextEndStep (ForEach That (Act (Move It Graveyard))) ]
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
  , manaCost := [^6, ^White]
  , types := [Sorcery]
  , abilities :=
      [ Spell $
          If (And [ Matches This (WasCastFrom Hand)
                  , Compare (EventCount (Query [ KindIs Cast
                                               , ActorIs you
                                               , SourceMatches (SameName This)
                                               , Within ThisGame ]))
                            GreaterEq (Literal 2) ])
             (Conclude (WinGame You))
             { otherwise = Just (Sequence
                 [ Act (PutIntoLibrary (This) (FromTop (^6)))
                 , Act (GainLife (^7)) ]) }
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
  , manaCost := [^3]
  , types := [Artifact]
  , abilities :=
      [ Activated (Costs [Mana [^4], TapSelf])
          (Targeted [Target (^1) permanent]
            (Act (PutCounters Fate (Literal 1) ((GetTarget 0)))))
      , Activated (Costs [Mana [^5], TapSelf, Sacrifice (This)])
          (Sequence
            [ ForEach (SelectAll (AllOf [permanent, IsNot (HasType Land), IsNot (HasCounter Fate)])) (Act (Destroy It))
            , ForEach (SelectAll permanent) (Act (RemoveAllCounters Fate It)) ])
      ]
  }

-- A clean ANTHEM: a static `ModifyAll` over "creatures you control". Exercises
-- ModifyAll + ControlledBy (the controller predicate).
export
GloriousAnthem : Card
GloriousAnthem = Normal $ fromDefault
  { name := "Glorious Anthem"
  , manaCost := [^1, ^White, ^White]
  , types := [Enchantment]
  , abilities :=
      [ Static (ModifyAll (AllOf [HasType Creature, ControlledBy you]) [ModifyPT (^1) (^1)]) ]
  }

-- Liliana of the Veil — "planeswalkers are pure composite": loyalty abilities are
-- Activated abilities whose cost adds/removes Loyalty counters, carrying [SorcerySpeed,
-- OncePerTurn] limits; the printed loyalty (3) is "enters with 3 Loyalty counters"
-- (Face.loyalty). "Each player" is `ForEach eachPlayer` (a player-`Selection`); "target
-- player" is a player-kinded target (`Anyone`), so `GetTarget 0` is `APlayer` with no
-- annotation. The −6 pile ultimate is OMITTED (no pile-division); the "Liliana"
-- planeswalker subtype is omitted (no planeswalker-subtype enum).
export
LilianaOfTheVeil : Card
LilianaOfTheVeil = Normal $ fromDefault
  { name := "Liliana of the Veil"
  , manaCost := [^1, ^Black, ^Black]
  , types := [Planeswalker]
  , supertypes := [Legendary]
  , loyalty := Just 3
  , abilities :=
      [ Activated (AddCounters Loyalty (Literal 1))
          (ForEach eachPlayer (Act (Discard {actor = It} (^1)))) {limits = [SorcerySpeed, OncePerTurn]}
      , Activated (RemoveCounters Loyalty (Literal 2))
          (Targeted [Target (^1) Anyone]
            (Act (Sacrifices (GetTarget 0) creature))) {limits = [SorcerySpeed, OncePerTurn]}
      ]
  }

-- Tide Shaper — layers + kicker. The kicked ETB makes a target land an Island for a
-- duration (AddSubtype + ForAsLongAs); a conditional static grants +1/+1 while an
-- opponent controls an Island (`While` + `exists`). FLAG: kicker is the WasKicked
-- boolean (no cost-mode model); Merfolk/Wizard subtypes omitted.
export
TideShaper : Card
TideShaper = Normal $ fromDefault
  { name := "Tide Shaper"
  , manaCost := [^Blue]
  , types := [Creature]
  , abilities :=
      [ Triggered (Query [KindIs (ZoneChanged Nothing (Just Battlefield)), SourceMatches (SameAs This)])
          (If (Matches This WasKicked)
              (Targeted [Target (^1) (HasType Land)]
                (Continuously (Modify (GetTarget 0) [AddSubtype (^Island)])
                              (ForAsLongAs (Matches This (InZone Battlefield))))))
      , Static (While (exists (AllOf [InZone Battlefield, HasSubtype (^Island), ControlledBy opponent]))
                      (Modify This [ModifyPT (^1) (^1)]))
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
  , manaCost := [^Black, ^Black, ^Black]
  , types := [Enchantment]
  , abilities :=
      [ Static (Replaces (Query [KindIs (BeginStep (BeginningPhase DrawStep)), DuringTurn you]) (Sequence []))
      , Triggered (Query [KindIs Discarded, ActorIs you])
          (Act (Move EventObject Exile))
      , Activated (PayLife (Literal 1))
          (ForEach (TopOfLibrary (Literal 1))
            (With (Produce (Move It Exile))
              (Delayed nextEndStep (ForEach That (Act (Move It Hand))))))
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
  , manaCost := [^2, ^Blue, ^Black]
  , types := [Creature]
  , abilities :=
      [ keyword Flash
      , Static (Replaces (Query [ KindIs Drew, ActorIs opponent
                                , Except (Query [DuringStep (BeginningPhase DrawStep)]) ])
          (Act (Draw {actor = You} (^1))))
      ]
  , power := Just 3
  , toughness := Just 1
  }

-- Oblivion Ring — TWO linked abilities, AS WRITTEN: (ETB) exile another target nonland
-- permanent; (LTB) return the cards exiled by this. The link is the `ExiledBy This`
-- predicate (the engine holds the exile association), so the abilities are independent
-- — no With/That fusion. Contrast Banishing Light's single "exile UNTIL" duration.
export
OblivionRing : Card
OblivionRing = Normal $ fromDefault
  { name := "Oblivion Ring"
  , manaCost := [^2, ^White]
  , types := [Enchantment]
  , abilities :=
      [ Triggered (Query [KindIs (ZoneChanged Nothing (Just Battlefield)), SourceMatches (SameAs This)])
          (Targeted [Target (^1) (AllOf [permanent, IsNot (HasType Land), IsNot (SameAs This)])]
            (Act (Move ((GetTarget 0)) Exile)))
      , Triggered (Query [KindIs (ZoneChanged (Just Battlefield) Nothing), SourceMatches (SameAs This)])
          (ForEach (SelectAll (ExiledBy This)) (Act (Move It Battlefield)))
      ]
  }

-- Banishing Light — ONE ability with a duration-bounded exile: "exile target nonland
-- permanent an opponent controls UNTIL this leaves" = `ExileUntil … (UntilEvent
-- (this leaves the battlefield))`. The "until" is a `Duration`, not a leave-trigger —
-- that's the rules difference from Oblivion Ring's two linked abilities.
export
BanishingLight : Card
BanishingLight = Normal $ fromDefault
  { name := "Banishing Light"
  , manaCost := [^2, ^White]
  , types := [Enchantment]
  , abilities :=
      [ Triggered (Query [KindIs (ZoneChanged Nothing (Just Battlefield)), SourceMatches (SameAs This)]) $
          Targeted [Target (^1) (AllOf [permanent, IsNot (HasType Land), ControlledBy opponent])] $
            Act (ExileUntil ((GetTarget 0))
                            (UntilEvent (Query [ KindIs (ZoneChanged (Just Battlefield) Nothing)
                                               , SourceMatches (SameAs This) ])))
      ]
  }

-- Donate — "Target player gains control of target permanent you control." The MIXED-kind
-- multi-target case: slot 0 is a player (`APlayer`), slot 1 an object (`AnObject`), so
-- `GetTarget 0`/`GetTarget 1` are strictly kinded by their own slots. The control shift is
-- a rest-of-game continuous effect (`Continuously … Permanent`).
export
Donate : Card
Donate = Normal $ fromDefault
  { name := "Donate"
  , manaCost := [^2, ^Blue]
  , types := [Sorcery]
  , abilities :=
      [ Spell (Targeted [ Target (^1) Anyone
                        , Target (^1) (AllOf [permanent, ControlledBy you]) ]
          (Continuously (Modify (GetTarget 1) [GainControl (GetTarget 0)]) Permanent))
      ]
  }

-- DEONTIC cards ----------------------------------------------------------------

-- Pacifism — "Enchanted creature can't attack or block." Two `Cant` clauses over the host
-- (`AttachHostOf This`): can't attack at all, and can't block any creature. Pure deontic.
export
Pacifism : Card
Pacifism = Normal $ fromDefault
  { name := "Pacifism"
  , manaCost := [^1, ^White]
  , types := [Enchantment]
  , subtypes := [^Aura]
  , abilities :=
      [ Enchant (AllOf [permanent, creature])
      , Static (Cant (Attacks (SameAs (AttachHostOf This))))
      , Static (Cant (Blocks (SameAs (AttachHostOf This)) creature))
      ]
  }

-- Juggernaut — "attacks each combat if able" (a `Must`) + "can't be blocked by Walls" (a
-- `Cant` on the blocker). The Juggernaut creature-subtype is omitted (not in the enum).
export
Juggernaut : Card
Juggernaut = Normal $ fromDefault
  { name := "Juggernaut"
  , manaCost := [^4]
  , types := [Artifact, Creature]
  , power := Just 5
  , toughness := Just 3
  , abilities :=
      [ Static (Must (Attacks (SameAs This)))
      , Static (Cant (Blocks (HasSubtype (^Wall)) (SameAs This)))
      ]
  }

-- Ghostly Prison — "Creatures can't attack you unless their controller pays {2} …" — a `Gate`
-- (cost FIRST): the attack is legal only if the toll is paid, never compulsory. The "{2} for
-- EACH attacking creature" scaling is now expressible via `Scaled (CountOf …) (Mana [^2])`,
-- but the count "creatures attacking you" needs an "is-attacking" combat-state predicate we
-- don't have yet — so this stays flat {2} for now.
export
GhostlyPrison : Card
GhostlyPrison = Normal $ fromDefault
  { name := "Ghostly Prison"
  , manaCost := [^2, ^White]
  , types := [Enchantment]
  , abilities :=
      [ Static (Gate (Mana [^2]) (Attacks creature {whom = you})) ]
  }

-- Wall of Omens — a DEONTIC KEYWORD card: `keyword Defender` expands to a `Composite` whose tag is
-- `Defender` and whose body is the can't-attack `Cant` clause — the meaning is intrinsic to the
-- keyword, not written on the card. (+ a plain ETB draw trigger.)
export
WallOfOmens : Card
WallOfOmens = Normal $ fromDefault
  { name := "Wall of Omens"
  , manaCost := [^1, ^White]
  , types := [Creature]
  , subtypes := [^Wall]
  , power := Just 0
  , toughness := Just 4
  , abilities :=
      [ keyword Defender
      , Triggered (Query [KindIs (ZoneChanged Nothing (Just Battlefield)), SourceMatches (SameAs This)])
          (Act (Draw (^1)))
      ]
  }

-- HIGH-COVERAGE cards (exercise multiple subsystems at once) -------------------

-- Mana Leak — the cost-payment DECIDER on a card: the spell's CONTROLLER `MustPay` {3}, OR ELSE
-- it's countered. Exercises MustPay / ControllerOf (the targeted spell's controller) / a spell target.
export
ManaLeak : Card
ManaLeak = Normal $ fromDefault
  { name := "Mana Leak"
  , manaCost := [^1, ^Blue]
  , types := [Instant]
  , abilities :=
      [ Spell (Targeted [Target (^1) (IsKind IsSpell)]
          (MustPay {actor = ControllerOf (GetTarget 0)} (Mana [^3])
            (Act (Counter ((GetTarget 0)))))) ]
  }

-- Invisible Stalker — a DEONTIC-KEYWORD creature: `keyword (Hexproof Nothing)` is a `Composite`
-- carrying its can't-be-targeted `Cant`; "can't be blocked" is a second `Cant` (no creature may
-- block it). (Rogue subtype omitted — not in the enum.)
export
InvisibleStalker : Card
InvisibleStalker = Normal $ fromDefault
  { name := "Invisible Stalker"
  , manaCost := [^1, ^Blue]
  , types := [Creature]
  , subtypes := [^Human]
  , power := Just 1
  , toughness := Just 1
  , abilities :=
      [ keyword (Hexproof Nothing)
      , Static (Cant (Blocks creature (SameAs This)))
      ]
  }

-- Cryptic Command — a MODAL spell ("choose two"), each mode its own effect (two with their own
-- targets). Exercises Modal / per-mode Targeted / Counter / Move / Tap / ControlledBy opponent.
export
CrypticCommand : Card
CrypticCommand = Normal $ fromDefault
  { name := "Cryptic Command"
  , manaCost := [^1, ^Blue, ^Blue, ^Blue]
  , types := [Instant]
  , abilities :=
      [ Spell (Modal (MkChooseSpec (^2))
          [ MkMode (Targeted [Target (^1) (IsKind IsSpell)] (Act (Counter ((GetTarget 0)))))
          , MkMode (Targeted [Target (^1) permanent] (Act (Move ((GetTarget 0)) Hand)))
          , MkMode (ForEach (SelectAll (AllOf [creature, ControlledBy opponent])) (Act (Tap It)))
          , MkMode (Act (Draw (^1)))
          ]) ]
  }

-- PLURAL targets + divided damage. "deals 2 damage divided as you choose among one or two target
-- creatures and/or players" — a single slot with a NON-ZERO range cardinality (1–2), referenced as
-- the GROUP `GetTargets 0` and fed to `DealDamageDivided`. Then an untargeted draw.
export
Electrolyze : Card
Electrolyze = Normal $ fromDefault
  { name := "Electrolyze"
  , manaCost := [^1, ^Blue, ^Red]
  , types := [Instant]
  , abilities :=
      [ Spell (Targeted [Target (between (^1) (^2)) (OneOf [creature, Anyone])]
          (Sequence
            [ Act (DealDamageDivided (^2) (GetTargets 0))
            , Act (Draw (^1)) ]))
      ]
  }

--:vim:sts=2 sw=2:

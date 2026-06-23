||| Card encodings. Each is a `Core.Card` built with the `^: { … }`
||| builder, using `Core` constructors and `Macros` templates. Verbs go through
||| `Act` (the verb compartment); a binder that produces a moved object wraps
||| the producing `Action` in `Produce`.
module Cards

import Core
import Macros

export
card_LightningBolt : Card
card_LightningBolt = Normal $ ^:
  { name := Just "Lightning Bolt"
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
card_Pyroclasm : Card
card_Pyroclasm = Normal $ ^:
  { name := Just "Pyroclasm"
  , manaCost := [^1, ^Red]
  , types := [Sorcery]
  , abilities :=
      [ Spell (ForEach (SelectAll creature) (Act (DealDamage It (^2))))
      ]
  }

-- Vanilla creature: no abilities, just power/toughness. No new data variant.
export
card_GrizzlyBears : Card
card_GrizzlyBears = Normal $ ^:
  { name := Just "Grizzly Bears"
  , manaCost := [^1, ^Green]
  , types := [Creature]
  , subtypes := [^Bear]
  , power := Just 2
  , toughness := Just 2
  }

-- French vanilla: a single keyword ability.
export
card_TyphoidRats : Card
card_TyphoidRats = Normal $ ^:
  { name := Just "Typhoid Rats"
  , manaCost := [^Black]
  , types := [Creature]
  , subtypes := [^Rat]
  , abilities := [keyword Deathtouch]
  , power := Just 1
  , toughness := Just 1
  }

export
card_GiantSpider : Card
card_GiantSpider = Normal $ ^:
  { name := Just "Giant Spider"
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
card_Flickerwisp : Card
card_Flickerwisp = Normal $ ^:
  { name := Just "Flickerwisp"
  , manaCost := [^1, ^White, ^White]
  , types := [Creature]
  , subtypes := [^Elemental]
  , abilities :=
      [ keyword Flying
      , Triggered (And [KindIs (ZoneChanged Nothing (Just Battlefield)), SourceMatches (SameAs This)]) $
          Targeted [Target (^1) (And [permanent, Not (SameAs This)])] $
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
card_Brainstorm : Card
card_Brainstorm = Normal $ ^:
  { name := Just "Brainstorm"
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
card_Rancor : Card
card_Rancor = Normal $ ^:
  { name := Just "Rancor"
  , manaCost := [^Green]
  , types := [Enchantment]
  , subtypes := [^Aura]
  , abilities :=
      [ Enchant (And [permanent, creature])
      , Static (Modify (AttachHostOf This)
          [ ModifyPT (^2) (^0)
          , GrantAbility (keyword Trample)
          ])
      , Triggered
          (And [ KindIs (ZoneChanged (Just Battlefield) (Just Graveyard))
                 , SourceMatches (SameAs This)
                 ])
          (Act (Move (This) Hand))
      ]
  }

-- TRICKY: Cloudshift — exile→return in ONE resolution (the pure [CR#400.7j] case).
-- `With (Produce (Move …))` binds the exiled object as `That`; the body returns it.
export
card_Cloudshift : Card
card_Cloudshift = Normal $ ^:
  { name := Just "Cloudshift"
  , manaCost := [^White]
  , types := [Instant]
  , abilities :=
      [ Spell $
          Targeted [ Target (^1) (And [ permanent, creature
                                     , ControlledBy you
                                     ])
                   ] $
          With (Produce (Move ((GetTarget 0)) Exile)) $
          ForEach That (Act (Move It Battlefield))
      ]
  }

-- TRICKY: Through the Breach — put a creature onto the battlefield (binding it as `That`); it GAINS
-- HASTE (now a grantable keyword — a continuous `GrantAbility (keyword Haste)` until end of turn);
-- then a DELAYED trigger sacrifices `That` at the next end step. The captured `That` is the
-- acceptance test: if the engine can't still find the object at fire time, the sacrifice does
-- nothing. (The alternative cast cost stays casting machinery, not a card-effect clause.)
export
card_ThroughTheBreach : Card
card_ThroughTheBreach = Normal $ ^:
  { name := Just "Through the Breach"
  , manaCost := [^4, ^Red]
  , types := [Instant]
  , abilities :=
      [ Spell $
          With (Choose (^1) (And [inHand, creature])) $
            Sequence
              [ ForEach That (Act (Move It Battlefield))
              , Continuously (Modify (Single That) [GrantAbility (keyword Haste)]) UntilEndOfTurn  -- "it gains haste"
              , Delayed nextEndStep (ForEach That (Act (Move It Graveyard))) ]
      ]
  }

-- TRICKY: Approach of the Second Sun — an alternate WIN CONDITION gated on game
-- history. `EventCount` (log-derived) counts this game's prior casts of this same
-- spell; ≥2 (the current cast is itself logged) ⇒ you win. Otherwise burrow it 7th
-- from the top and gain 7. Exercises Outcome / EventQuery / SameName / WasCastFrom /
-- positional library / GainLife.
export
card_ApproachOfTheSecondSun : Card
card_ApproachOfTheSecondSun = Normal $ ^:
  { name := Just "Approach of the Second Sun"
  , manaCost := [^6, ^White]
  , types := [Sorcery]
  , abilities :=
      [ Spell $
          If (And [ Matches This (WasCastFrom Hand)
                  , Compare (EventCount (And [ KindIs Cast
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
card_OblivionStone : Card
card_OblivionStone = Normal $ ^:
  { name := Just "Oblivion Stone"
  , manaCost := [^3]
  , types := [Artifact]
  , abilities :=
      [ Activated (Costs [Mana [^4], TapSelf])
          (Targeted [Target (^1) permanent]
            (Act (PutCounters Fate (Literal 1) ((GetTarget 0)))))
      , Activated (Costs [Mana [^5], TapSelf, Sacrifice (This)])
          (Sequence
            [ ForEach (SelectAll (And [permanent, Not (HasType Land), Not (HasCounter Fate)])) (Act (Destroy It))
            , ForEach (SelectAll permanent) (Act (RemoveAllCounters Fate It)) ])
      ]
  }

-- A clean ANTHEM: a static `ModifyAll` over "creatures you control". Exercises
-- ModifyAll + ControlledBy (the controller predicate).
export
card_GloriousAnthem : Card
card_GloriousAnthem = Normal $ ^:
  { name := Just "Glorious Anthem"
  , manaCost := [^1, ^White, ^White]
  , types := [Enchantment]
  , abilities :=
      [ Static (ModifyAll (And [HasType Creature, ControlledBy you]) [ModifyPT (^1) (^1)]) ]
  }

-- Liliana of the Veil — "planeswalkers are pure composite": loyalty abilities are
-- Activated abilities whose cost adds/removes Loyalty counters, carrying {window = SorceryWindow,
-- limits = [OncePerTurn]}; the printed loyalty (3) is "enters with 3 Loyalty counters"
-- (Face.loyalty). "Each player" is `ForEach eachPlayer` (a player-`Selection`); "target
-- player" is a player-kinded target (`Anyone`), so `GetTarget 0` is `APlayer` with no
-- annotation. The −6 pile ultimate is OMITTED (no pile-division); the "Liliana"
-- planeswalker subtype is omitted (no planeswalker-subtype enum).
export
card_LilianaOfTheVeil : Card
card_LilianaOfTheVeil = Normal $ ^:
  { name := Just "Liliana of the Veil"
  , manaCost := [^1, ^Black, ^Black]
  , types := [Planeswalker]
  , supertypes := [Legendary]
  , loyalty := Just 3
  , abilities :=
      [ Activated (AddCounters Loyalty (Literal 1))
          (ForEach eachPlayer (Act (Discard {actor = It} (^1)))) {window = SorceryWindow, limits = [OncePerTurn]}
      , Activated (RemoveCounters Loyalty (Literal 2))
          (Targeted [Target (^1) Anyone]
            (Act (Sacrifices (GetTarget 0) creature))) {window = SorceryWindow, limits = [OncePerTurn]}
      ]
  }

-- Tide Shaper — layers + kicker. The kicked ETB makes a target land an Island for a
-- duration (AddSubtype + ForAsLongAs); a conditional static grants +1/+1 while an
-- opponent controls an Island (`While` + `exists`). FLAG: kicker is the WasKicked
-- boolean (no cost-mode model).
export
card_TideShaper : Card
card_TideShaper = Normal $ ^:
  { name := Just "Tide Shaper"
  , manaCost := [^Blue]
  , types := [Creature]
  , subtypes := [^Merfolk, ^Wizard]
  , abilities :=
      [ Triggered (And [KindIs (ZoneChanged Nothing (Just Battlefield)), SourceMatches (SameAs This)])
          (If (Matches This WasKicked)
              (Targeted [Target (^1) (HasType Land)]
                (Continuously (Modify (GetTarget 0) [AddSubtype (^Island)])
                              (ForAsLongAs (Matches This (InZone Battlefield))))))
      , Static (While (exists (And [InZone Battlefield, HasSubtype (^Island), ControlledBy opponent]))
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
card_Necropotence : Card
card_Necropotence = Normal $ ^:
  { name := Just "Necropotence"
  , manaCost := [^Black, ^Black, ^Black]
  , types := [Enchantment]
  , abilities :=
      [ Static (Replaces (And [KindIs (BeginStep (BeginningPhase DrawStep)), DuringTurn you]) (Sequence []))
      , Triggered (And [KindIs Discard, ActorIs you])
          (Act (Move EventObject Exile))
      , Activated (PayLife (Literal 1))
          (ForEach (TopOfLibrary (Literal 1))
            (With (Produce (Move It Exile))
              (Delayed nextEndStep (ForEach That (Act (Move It Hand))))))
      ]
  }

-- Notion Thief — replace an opponent's draw with "you draw a card" instead. "Not the FIRST one
-- they draw in each of their draw steps" is now FAITHFUL via the ordinal `IsFirst` facet: only the
-- first draw-step draw is exempt, so a SECOND draw-step draw (or any draw outside it) is still
-- stolen — exactly as written. (Rogue subtype now in the enum.)
export
card_NotionThief : Card
card_NotionThief = Normal $ ^:
  { name := Just "Notion Thief"
  , manaCost := [^2, ^Blue, ^Black]
  , types := [Creature]
  , subtypes := [^Human, ^Rogue]
  , abilities :=
      [ keyword Flash
      , Static (Replaces (And [ KindIs Draw, ActorIs opponent
                                , Not (And [DuringStep (BeginningPhase DrawStep), IsFirst ThisStep]) ])
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
card_OblivionRing : Card
card_OblivionRing = Normal $ ^:
  { name := Just "Oblivion Ring"
  , manaCost := [^2, ^White]
  , types := [Enchantment]
  , abilities :=
      [ Triggered (And [KindIs (ZoneChanged Nothing (Just Battlefield)), SourceMatches (SameAs This)])
          (Targeted [Target (^1) (And [permanent, Not (HasType Land), Not (SameAs This)])]
            (Act (Move ((GetTarget 0)) Exile)))
      , Triggered (And [KindIs (ZoneChanged (Just Battlefield) Nothing), SourceMatches (SameAs This)])
          (ForEach (SelectAll (ExiledBy This)) (Act (Move It Battlefield)))
      ]
  }

-- Banishing Light — ONE ability with a duration-bounded exile: "exile target nonland
-- permanent an opponent controls UNTIL this leaves" = `ExileUntil … (UntilEvent
-- (this leaves the battlefield))`. The "until" is a `Duration`, not a leave-trigger —
-- that's the rules difference from Oblivion Ring's two linked abilities.
export
card_BanishingLight : Card
card_BanishingLight = Normal $ ^:
  { name := Just "Banishing Light"
  , manaCost := [^2, ^White]
  , types := [Enchantment]
  , abilities :=
      [ Triggered (And [KindIs (ZoneChanged Nothing (Just Battlefield)), SourceMatches (SameAs This)]) $
          Targeted [Target (^1) (And [permanent, Not (HasType Land), ControlledBy opponent])] $
            Act (ExileUntil ((GetTarget 0))
                            (UntilEvent (And [ KindIs (ZoneChanged (Just Battlefield) Nothing)
                                               , SourceMatches (SameAs This) ])))
      ]
  }

-- Donate — "Target player gains control of target permanent you control." The MIXED-kind
-- multi-target case: slot 0 is a player (`APlayer`), slot 1 an object (`AnObject`), so
-- `GetTarget 0`/`GetTarget 1` are strictly kinded by their own slots. The control shift is
-- a rest-of-game continuous effect (`Continuously … Permanent`).
export
card_Donate : Card
card_Donate = Normal $ ^:
  { name := Just "Donate"
  , manaCost := [^2, ^Blue]
  , types := [Sorcery]
  , abilities :=
      [ Spell (Targeted [ Target (^1) Anyone
                        , Target (^1) (And [permanent, ControlledBy you]) ]
          (Continuously (Modify (GetTarget 1) [GainControl (GetTarget 0)]) Permanent))
      ]
  }

-- DEONTIC cards ----------------------------------------------------------------

-- Pacifism — "Enchanted creature can't attack or block." Two `Cant` clauses over the host
-- (`AttachHostOf This`): can't attack at all, and can't block any creature. Pure deontic.
export
card_Pacifism : Card
card_Pacifism = Normal $ ^:
  { name := Just "Pacifism"
  , manaCost := [^1, ^White]
  , types := [Enchantment]
  , subtypes := [^Aura]
  , abilities :=
      [ Enchant (And [permanent, creature])
      , Static (Cant (Attacks (SameAs (AttachHostOf This))))
      , Static (Cant (Blocks (SameAs (AttachHostOf This)) creature))
      ]
  }

-- Juggernaut — "attacks each combat if able" (a `Must`) + "can't be blocked by Walls" (a
-- `Cant` on the blocker).
export
card_Juggernaut : Card
card_Juggernaut = Normal $ ^:
  { name := Just "Juggernaut"
  , manaCost := [^4]
  , types := [Artifact, Creature]
  , subtypes := [^Juggernaut]
  , power := Just 5
  , toughness := Just 3
  , abilities :=
      [ Static (Must (Attacks (SameAs This)))
      , Static (Cant (Blocks (HasSubtype (^Wall)) (SameAs This)))
      ]
  }

-- Ghostly Prison — "Creatures can't attack you unless their controller pays {2} for each creature
-- they control that's attacking you" — a `Gate` (cost FIRST; never compulsory). This is NOT a flat
-- approximation: the `Deed` is PER-ATTACKER (`Attacks creature {whom = you}`), so the Gate charges
-- {2} per attacker attacking you — N attackers ⇒ {2}N, exactly the printed cost. (No `Scaled` needed.)
export
card_GhostlyPrison : Card
card_GhostlyPrison = Normal $ ^:
  { name := Just "Ghostly Prison"
  , manaCost := [^2, ^White]
  , types := [Enchantment]
  , abilities :=
      [ Static (Gate (Mana [^2]) (Attacks creature {whom = you})) ]
  }

-- Wall of Omens — a DEONTIC KEYWORD card: `keyword Defender` expands to a `Composite` whose tag is
-- `Defender` and whose body is the can't-attack `Cant` clause — the meaning is intrinsic to the
-- keyword, not written on the card. (+ a plain ETB draw trigger.)
export
card_WallOfOmens : Card
card_WallOfOmens = Normal $ ^:
  { name := Just "Wall of Omens"
  , manaCost := [^1, ^White]
  , types := [Creature]
  , subtypes := [^Wall]
  , power := Just 0
  , toughness := Just 4
  , abilities :=
      [ keyword Defender
      , Triggered (And [KindIs (ZoneChanged Nothing (Just Battlefield)), SourceMatches (SameAs This)])
          (Act (Draw (^1)))
      ]
  }

-- HIGH-COVERAGE cards (exercise multiple subsystems at once) -------------------

-- Mana Leak — the cost-payment DECIDER on a card: the spell's CONTROLLER `MustPay` {3}, OR ELSE
-- it's countered. Exercises MustPay / ControllerOf (the targeted spell's controller) / a spell target.
export
card_ManaLeak : Card
card_ManaLeak = Normal $ ^:
  { name := Just "Mana Leak"
  , manaCost := [^1, ^Blue]
  , types := [Instant]
  , abilities :=
      [ Spell (Targeted [Target (^1) (IsKind IsSpell)]
          (MustPay {actor = ControllerOf (GetTarget 0)} (Mana [^3])
            (Act (Counter ((GetTarget 0)))))) ]
  }

-- Invisible Stalker — a DEONTIC-KEYWORD creature: `keyword (Hexproof Nothing)` is a `Composite`
-- carrying its can't-be-targeted `Cant`; "can't be blocked" is a second `Cant` (no creature may
-- block it).
export
card_InvisibleStalker : Card
card_InvisibleStalker = Normal $ ^:
  { name := Just "Invisible Stalker"
  , manaCost := [^1, ^Blue]
  , types := [Creature]
  , subtypes := [^Human, ^Rogue]
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
card_CrypticCommand : Card
card_CrypticCommand = Normal $ ^:
  { name := Just "Cryptic Command"
  , manaCost := [^1, ^Blue, ^Blue, ^Blue]
  , types := [Instant]
  , abilities :=
      [ Spell (Modal (MkChooseSpec (^2))
          [ MkMode (Targeted [Target (^1) (IsKind IsSpell)] (Act (Counter ((GetTarget 0)))))
          , MkMode (Targeted [Target (^1) permanent] (Act (Move ((GetTarget 0)) Hand)))
          , MkMode (ForEach (SelectAll (And [creature, ControlledBy opponent])) (Act (Tap It)))
          , MkMode (Act (Draw (^1)))
          ]) ]
  }

-- PLURAL targets + divided damage. "deals 2 damage divided as you choose among one or two target
-- creatures and/or players" — a single slot with a NON-ZERO range cardinality (1–2), referenced as
-- the GROUP `GetTargets 0` and fed to `DealDamageDivided`. Then an untargeted draw.
export
card_Electrolyze : Card
card_Electrolyze = Normal $ ^:
  { name := Just "Electrolyze"
  , manaCost := [^1, ^Blue, ^Red]
  , types := [Instant]
  , abilities :=
      [ Spell (Targeted [Target (between (^1) (^2)) (Or [creature, Anyone])]
          (Sequence
            [ Act (DealDamageDivided (^2) (GetTargets 0))
            , Act (Draw (^1)) ]))
      ]
  }

-- Flash made real: `keyword Flash` now carries a `Can`-cast-at-instant-speed (the deontic permission
-- floor). Pairs a Composite keyword (Flash → the `Can`) with a Bare one (Deathtouch). (Snake subtype
-- omitted — not in the enum.)
export
card_AmbushViper : Card
card_AmbushViper = Normal $ ^:
  { name := Just "Ambush Viper"
  , manaCost := [^1, ^Green]
  , types := [Creature]
  , abilities := [keyword Flash, keyword Deathtouch]
  , power := Just 2
  , toughness := Just 1
  }

-- Menace: a SET-LEVEL `Cant` (the whole blocker set must be ≥2, not a per-blocker check) — the
-- `BlockedBy` deed the doc flags as mandatory.
export
card_BoggartBrute : Card
card_BoggartBrute = Normal $ ^:
  { name := Just "Boggart Brute"
  , manaCost := [^2, ^Red]
  , types := [Creature]
  , subtypes := [^Goblin, ^Warrior]
  , abilities := [keyword Menace]
  , power := Just 3
  , toughness := Just 2
  }

-- Token ABILITIES (no longer vanilla-only): "create two 1/1 white Spirit creature tokens with
-- flying" — the `TokenSpec` now carries `[keyword Flying]`, mirroring a card face's abilities.
export
card_MidnightHaunting : Card
card_MidnightHaunting = Normal $ ^:
  { name := Just "Midnight Haunting"
  , manaCost := [^2, ^White]
  , types := [Instant]
  , abilities :=
      [ Spell (Act (CreateToken (^2)
          (^: { name := Just "Spirit", types := [Creature], subtypes := [^Spirit]
              , colors := [White], power := Just 1, toughness := Just 1
              , abilities := [keyword Flying] })))
      ]
  }

-- Leveler ([CR#711]) — "a list of conditional statics keyed on the level-counter count": a `Level`
-- counter added by the level-up ability, and one `While (CountersOn Level This in range)` tier per
-- band. NO new machinery beyond the `Level` counter — While/CountersOn/SetPT/GrantAbility did it all.
export
card_StudentOfWarfare : Card
card_StudentOfWarfare = Normal $ ^:
  { name := Just "Student of Warfare"
  , manaCost := [^White]
  , types := [Creature]
  , subtypes := [^Human, ^Knight]
  , power := Just 1
  , toughness := Just 1
  , abilities :=
      [ Activated (Mana [^White]) (Act (PutCounters Level (^1) This)) {window = SorceryWindow}  -- "level up only as a sorcery"
      , Static (While (And [ Compare (CountersOn Level This) GreaterEq (^2)
                           , Compare (CountersOn Level This) LessEq (^6) ])
          (Modify This [SetPT (^3) (^3), GrantAbility (keyword FirstStrike)]))            -- LEVEL 2–6: 3/3 first strike
      , Static (While (Compare (CountersOn Level This) GreaterEq (^7))
          (Modify This [SetPT (^4) (^4), GrantAbility (keyword DoubleStrike)]))           -- LEVEL 7+: 4/4 double strike
      ]
  }

--:vim:sts=2 sw=2:

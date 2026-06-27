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

-- Untargeted group damage: a `Each` over all creatures, dealing to each `It` (no `Targeted`).
export
card_Pyroclasm : Card
card_Pyroclasm = Normal $ ^:
  { name := Just "Pyroclasm"
  , manaCost := [^1, ^Red]
  , types := [Sorcery]
  , abilities :=
      [ Spell (Each (SelectAll creature) (Act (DealDamage It (^2))))
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
      , Triggered (thisEnters) $
          Targeted [Target (^1) (And [permanent, Not (SameAs This)])] $
            With (Produce (Move ((GetTarget 0)) (ToZone Exile))) $  -- exile the target, bind `That`
              Delayed nextEndStep
                (Each That (Act (Move It (ToZone Battlefield))))                        -- return `That` (captured; target gone)
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
          , With (Choose (^2) inHand) (Each That (Act (Move It (ToZone Library))))
          ]
      ]
  }

-- TRICKY: an Aura, the ENGINE's way (no dedicated `Enchant`): "enchant creature" is the PERMISSION
-- `Can (Enact Attach (SameAs This) creature)` — attaching is default-forbidden, so the aura ENABLES itself
-- to attach to creatures; a `Static` buffs the host (`AttachHostOf This`) with +2/+0 and trample; a
-- graveyard trigger returns it to hand. The intrinsic enters-attached / falls-off rules (`Also`/`Sba`) are
-- SHARED statics (engine: conferred by the Aura subtype) — see Spec.
export
card_Rancor : Card
card_Rancor = Normal $ ^:
  { name := Just "Rancor"
  , manaCost := [^Green]
  , types := [Enchantment]
  , subtypes := [^Aura]
  , abilities := enchant creature ++
      [ Static (Modify (SelectAll (SameAs (AttachHostOf This)))
          [ Alter Power (Up (^2))
          , GrantAbility (keyword Trample)
          ])
      , Triggered
          (MkQuery [ZoneChanged (Just Battlefield) (Just Graveyard)]
                   [Agent (SameAs This)])
          (Act (Move (This) (ToZone Hand)))
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
          With (Produce (Move ((GetTarget 0)) (ToZone Exile))) $
          Each That (Act (Move It (ToZone Battlefield)))
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
              [ Each That (Act (Move It (ToZone Battlefield)))
              , Continuously (Modify That [GrantAbility (keyword Haste)]) UntilEndOfTurn  -- "it gains haste"
              , Delayed nextEndStep (Each That (Act (Move It (ToZone Graveyard)))) ]
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
                  , Compare (CountEvents (MkQuery [Begins Cast]
                                               [ Actor you
                                               , Patient (SameName This)
                                               , Within ThisGame ]))
                            GreaterEq (Literal 2) ])
             (Conclude (WinGame You))
             { otherwise = Just (Sequence
                 [ Act (Move (This) (ToLibrary (FromTop (^6))))
                 , Act (GainLife (^7)) ]) }
      ]
  }

-- Two ACTIVATED abilities + a cost algebra + counters. `{4},{T}:` fate-counter a
-- permanent; `{5},{T},Sacrifice this:` wrath everything not fate-protected, then
-- clear the counters. Exercises Activated / Cost (mana+tap+sacrifice) / CounterKind /
-- HasCounter / PutCounters / RemoveCounters.
export
card_OblivionStone : Card
card_OblivionStone = Normal $ ^:
  { name := Just "Oblivion Stone"
  , manaCost := [^3]
  , types := [Artifact]
  , abilities :=
      [ Activated (Costs [Mana [^4], Do (Tap This)])
          (Targeted [Target (^1) permanent]
            (Act (PutCounters Fate (Literal 1) ((GetTarget 0)))))
      , Activated (Costs [Mana [^5], Do (Tap This), Do (Sacrifice You (SameAs This))])
          (Sequence
            [ Each (SelectAll (And [permanent, Not (HasType Land), Not (HasCounter Fate)])) (Act (Destroy It))
            , Each (SelectAll permanent) (Act (RemoveCounters Fate (CountersOn Fate It) It)) ])
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
      [ Static (Modify (SelectAll (And [HasType Creature, ControlledBy you])) [Alter Power (Up (^1)), Alter Toughness (Up (^1))]) ]
  }

-- Liliana of the Veil — "planeswalkers are pure composite": loyalty abilities are
-- Activated abilities whose cost adds/removes Loyalty counters, carrying {window = AsSorcery,
-- limits = [OncePerTurn]}; the printed loyalty (3) is "enters with 3 Loyalty counters"
-- (Face.loyalty). "Each player" is `Each eachPlayer` (a player-`Selection`); "target
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
      [ Activated (Do (PutCounters Loyalty (Literal 1) This))
          (Each eachPlayer (Act (Discard {actor = It} (^1)))) {window = AsSorcery, limits = [OncePerTurn]}
      , Activated (Do (RemoveCounters Loyalty (Literal 2) This))
          (Targeted [Target (^1) Anyone]
            (Act (Sacrifice (GetTarget 0) creature))) {window = AsSorcery, limits = [OncePerTurn]}
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
      [ Triggered (thisEnters)
          (If (Matches This WasKicked)
              (Targeted [Target (^1) (HasType Land)]
                (Continuously (Modify (SelectAll (SameAs (GetTarget 0))) [Alter Subtypes (Add (^Island))])
                              (ForAsLongAs (Matches This (InZone Battlefield))))))
      , Static (While (exists (And [InZone Battlefield, HasSubtype (^Island), ControlledBy opponent]))
                      (Modify (SelectAll (SameAs This)) [Alter Power (Up (^1)), Alter Toughness (Up (^1))]))
      ]
  , power := Just 1
  , toughness := Just 1
  }

-- Necropotence — "skip your draw step" is a replacement whose effect is nothing (the
-- engine skips the step); the discard trigger exiles the discarded card via `EventObject`
-- ("that card") — now GATED, and valid here because a `Discard` event supplies an object;
-- the pay-life ability draws into exile, deferred to your end step. ("face down" isn't modeled.)
export
card_Necropotence : Card
card_Necropotence = Normal $ ^:
  { name := Just "Necropotence"
  , manaCost := [^Black, ^Black, ^Black]
  , types := [Enchantment]
  , abilities :=
      [ Static (Replaces (MkQuery [BeginStep (BeginningPhase DrawStep)] [DuringTurn you]) (Sequence []))
      , Triggered (MkQuery [Discard] [Actor you])
          (Act (Move EventObject (ToZone Exile)))
      , Activated (Do (LoseLife (Literal 1)))
          (Each (TopOfLibrary (Literal 1))
            (With (Produce (Move It (ToZone Exile)))
              (Delayed nextEndStep (Each That (Act (Move It (ToZone Hand)))))))
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
      , Static (Replaces (MkQuery [Draw]
                                [ Actor opponent
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
      [ Triggered (thisEnters)
          (Targeted [Target (^1) (And [permanent, Not (HasType Land), Not (SameAs This)])]
            (Act (Move ((GetTarget 0)) (ToZone Exile))))
      , Triggered (MkQuery [ZoneChanged (Just Battlefield) Nothing] [Agent (SameAs This)])
          (Each (SelectAll (ExiledBy This)) (Act (Move It (ToZone Battlefield))))
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
      [ Triggered (thisEnters) $
          Targeted [Target (^1) (And [permanent, Not (HasType Land), ControlledBy opponent])] $
            Act (ExileUntil ((GetTarget 0))
                            (UntilEvent (MkQuery [ZoneChanged (Just Battlefield) Nothing]
                                               [Agent (SameAs This)])))
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
          (Continuously (Modify (SelectAll (SameAs (GetTarget 1))) [GainControl (GetTarget 0)]) Permanent))
      ]
  }

-- DEONTIC cards ----------------------------------------------------------------

-- Pacifism — an Aura: "enchant creature" (a `Can (Enact Attach …)` PERMISSION, since attaching is
-- default-forbidden), then two `cant` clauses over the host (`AttachHostOf This`): can't attack at all,
-- can't block any creature. Pure deontic.
export
card_Pacifism : Card
card_Pacifism = Normal $ ^:
  { name := Just "Pacifism"
  , manaCost := [^1, ^White]
  , types := [Enchantment]
  , subtypes := [^Aura]
  , abilities :=
      enchant creature ++
      [ Static (cant (Enact Attack (SameAs (AttachHostOf This)) Anyone))
      , Static (cant (Enact Block (SameAs (AttachHostOf This)) creature))
      ]
  }

-- Juggernaut — "attacks each combat if able" (a `must`) + "can't be blocked by Walls" (a
-- `cant` on the blocker).
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
      [ Static (must (Enact Attack (SameAs This) Anyone))
      , Static (cant (Enact Block (HasSubtype (^Wall)) (SameAs This)))
      ]
  }

-- Ghostly Prison — "Creatures can't attack you unless their controller pays {2} for each creature
-- they control that's attacking you" — a `Gate` (cost FIRST; never compulsory). This is NOT a flat
-- approximation: the `Deed` is PER-ATTACKER (`Enact Attack creature you`), so the Gate charges
-- {2} per attacker attacking you — N attackers ⇒ {2}N, exactly the printed cost. (No `Scaled` needed.)
export
card_GhostlyPrison : Card
card_GhostlyPrison = Normal $ ^:
  { name := Just "Ghostly Prison"
  , manaCost := [^2, ^White]
  , types := [Enchantment]
  , abilities :=
      [ Static (Priced AtDeclaration (Mana [^2]) (Enact Attack creature you)) ]
  }

-- Wall of Omens — a DEONTIC KEYWORD card: `keyword Defender` expands to a `Composite` whose tag is
-- `Defender` and whose body is the can't-attack `cant` clause — the meaning is intrinsic to the
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
      , Triggered (thisEnters)
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
-- carrying its can't-be-targeted `cant`; "can't be blocked" is a second `cant` (no creature may
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
      , Static (cant (Enact Block creature (SameAs This)))
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
          , MkMode (Targeted [Target (^1) permanent] (Act (Move ((GetTarget 0)) (ToZone Hand))))
          , MkMode (Each (SelectAll (And [creature, ControlledBy opponent])) (Act (Tap It)))
          , MkMode (Act (Draw (^1)))
          ]) ]
  }

-- PLURAL targets + divided damage. "deals 2 damage divided as you choose among one or two target
-- creatures and/or players" — a single slot with a NON-ZERO range cardinality (1–2), referenced as
-- the GROUP `GetTargets 0` and fed to the general `Distribute` (each element dealt its `Allotment`).
-- Then an untargeted draw.
export
card_Electrolyze : Card
card_Electrolyze = Normal $ ^:
  { name := Just "Electrolyze"
  , manaCost := [^1, ^Blue, ^Red]
  , types := [Instant]
  , abilities :=
      [ Spell (Targeted [Target (between (^1) (^2)) (Or [creature, Anyone])]
          (Sequence
            [ Distribute (^2) (GetTargets 0) (Act (DealDamage It Allotment))
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

-- Menace: a SET-LEVEL `cant` (the whole blocker set must be ≥2, not a per-blocker check) — the
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
      [ levelUp (Mana [^White])                                                                  -- "Level up {W}"
      , Static (While (And [ Compare (CountersOn Level This) GreaterEq (^2)
                           , Compare (CountersOn Level This) LessEq (^6) ])
          (Modify (SelectAll (SameAs This)) [Alter Power (Set (^3)), Alter Toughness (Set (^3)), GrantAbility (keyword FirstStrike)]))   -- LEVEL 2–6: 3/3 first strike
      , Static (While (Compare (CountersOn Level This) GreaterEq (^7))
          (Modify (SelectAll (SameAs This)) [Alter Power (Set (^4)), Alter Toughness (Set (^4)), GrantAbility (keyword DoubleStrike)]))   -- LEVEL 7+: 4/4 double strike
      ]
  }

-- Iona, Shield of Emeria — an AS-ENTERS value choice ([CR#614.12]): "choose a color", then a static
-- reads it. The choice is ONE ability — `AsEnters AColor [...]` — scoping only the static that reads
-- it (which nests at `bindChosen AColor Base`); `Flying` and the printed face stay at `Base`. "Spells
-- of the chosen color" is `And [IsKind IsSpell, OfChosen]` (`OfChosen` = "has the chosen color").
export
card_Iona : Card
card_Iona = Normal $ ^:
  { name := Just "Iona, Shield of Emeria"
  , manaCost := [^6, ^White, ^White, ^White]
  , types := [Creature]
  , supertypes := [Legendary]
  , subtypes := [^Angel]
  , abilities :=
      [ keyword Flying
      , AsEnters AColor
          [ Static (cant (Enact Cast opponent (And [IsKind IsSpell, OfChosen]))) ]
      ]
  , power := Just 7
  , toughness := Just 7
  }

-- Steely Resolve — the creature-type companion to Iona's color choice: an `AsEnters ACreatureType`
-- ability wraps the static that filters "creatures of the chosen type" via `OfChosen` (a `ModifyAll`
-- granting shroud). (Cavern of Souls is the other iconic creature-type chooser.)
export
card_SteelyResolve : Card
card_SteelyResolve = Normal $ ^:
  { name := Just "Steely Resolve"
  , manaCost := [^1, ^Green]
  , types := [Enchantment]
  , abilities :=
      [ AsEnters ACreatureType
          [ Static (Modify (SelectAll (And [creature, OfChosen])) [GrantAbility (keyword Shroud)]) ] ]
  }

-- Citadel Siege — the MODAL choose-on-enter case (Outpost Siege's class): an `AsEnters (AMode 2)`
-- ability wraps the two triggered abilities, each gated on `ChosenIs`. Both fire at begin-of-combat;
-- the unchosen mode's `If (ChosenIs …)` is inert — the toy gates the EFFECT (since `ChosenIs` is a
-- `Condition`), not the ability's existence. "That player controls" (Dragons) reads as
-- `ControlledBy opponent` — exact in two-player.
export
card_CitadelSiege : Card
card_CitadelSiege = Normal $ ^:
  { name := Just "Citadel Siege"
  , manaCost := [^3, ^White]
  , types := [Enchantment]
  , abilities :=
      [ AsEnters (AMode 2)
          [ -- Khans (0): begin combat on YOUR turn → two +1/+1 counters on a creature you control
            Triggered (MkQuery [BeginStep (CombatPhase BeginningOfCombatStep)] [DuringTurn you])
              (If (ChosenIs 0)
                  (Targeted [Target (^1) (And [creature, ControlledBy you])]
                    (Act (PutCounters P1P1 (^2) (GetTarget 0)))))
          , -- Dragons (1): begin combat on an OPPONENT's turn → tap a creature that opponent controls
            Triggered (MkQuery [BeginStep (CombatPhase BeginningOfCombatStep)] [DuringTurn opponent])
              (If (ChosenIs 1)
                  (Targeted [Target (^1) (And [creature, ControlledBy opponent])]
                    (Act (Tap (GetTarget 0)))))
          ]
      ]
  }

-- Outpost Siege — the namesake siege, now fully modeled (was substituted by Citadel Siege earlier).
-- The existing `TopOfLibrary` selection + `Enact Play` close Khans's impulse-draw: "exile the
-- top card; until end of turn you may play that card" = exile `Single (TopOfLibrary (^1))` (bind
-- `That`), then a continuous `Can (Plays …)` on `That`. Dragons pings on a creature you control leaving.
export
card_OutpostSiege : Card
card_OutpostSiege = Normal $ ^:
  { name := Just "Outpost Siege"
  , manaCost := [^3, ^Red]
  , types := [Enchantment]
  , abilities :=
      [ AsEnters (AMode 2)
          [ -- Khans (0): at your upkeep, exile the top card of your library; until eot you may play it
            Triggered (MkQuery [BeginStep (BeginningPhase UpkeepStep)] [DuringTurn you])
              (If (ChosenIs 0)
                  (With (Produce (Move (Single (TopOfLibrary (^1))) (ToZone Exile)))
                    (Continuously (Can (Enact Play you (SameAs (Single That)))) UntilEndOfTurn)))
          , -- Dragons (1): when a creature you control leaves the battlefield, deal 1 to any target
            Triggered (MkQuery [ZoneChanged (Just Battlefield) Nothing]
                           [Agent (And [creature, ControlledBy you])])
              (If (ChosenIs 1)
                  (Targeted [anyTarget] (Act (DealDamage (GetTarget 0) (^1)))))
          ]
      ]
  }

-- Cavern of Souls — the iconic creature-type chooser, now fully modeled. The {C} ability doesn't read
-- the choice, so it's a plain sibling; the second ability (which does) nests in `AsEnters
-- ACreatureType`. That one is RESTRICTED mana: `onlyToCast` allows it only for "a creature spell of
-- the chosen type" (`OfChosen`), and `confers` makes that spell uncounterable — the paid-for spell is
-- bound as `It`, so the rider reads `cant (Enact Counter spellOrAbility (SameAs It))`.
export
card_CavernOfSouls : Card
card_CavernOfSouls = Normal $ ^:
  { name := Just "Cavern of Souls"
  , types := [Land]
  , abilities :=
      [ Activated (Do (Tap This)) (Act (AddMana (^1) (^Colorless)))                          -- {T}: Add {C}
      , AsEnters ACreatureType
          [ Activated (Do (Tap This)) (Act (AddMana (^1) AnyColor
              { riders = [ SpendOnly (And [IsKind IsSpell, creature, OfChosen])
                         , GrantOnSpend (cant (Enact Counter spellOrAbility (SameAs It))) ] }))  -- {T}: any color — creature spell of the chosen type, uncounterable
          ]
      ]
  }

-- Wear // Tear — a SPLIT card ([CR#709]): two independent instant halves, each a full `Face`. The
-- `TwoFaced Split` holds both; "cast either half" is the engine's.
export
card_WearTear : Card
card_WearTear = TwoFaced Split
  (^: { name := Just "Wear"
      , manaCost := [^1, ^Red]
      , types := [Instant]
      , abilities := [ Spell (Targeted [Target (^1) (HasType Artifact)] (Act (Destroy (GetTarget 0)))) ]
      })
  (^: { name := Just "Tear"
      , manaCost := [^White]
      , types := [Instant]
      , abilities := [ Spell (Targeted [Target (^1) (HasType Enchantment)] (Act (Destroy (GetTarget 0)))) ]
      })

-- Brazen Borrower // Petty Theft — an ADVENTURE card ([CR#715]): a creature whose "adventure" half is
-- an instant. `TwoFaced Adventure` holds the creature `front` and the spell `back`; the cast-the-
-- adventure-then-exile-then-cast-the-creature flow is the engine's.
export
card_BrazenBorrower : Card
card_BrazenBorrower = TwoFaced Adventure
  (^: { name := Just "Brazen Borrower"
      , manaCost := [^1, ^Blue, ^Blue]
      , types := [Creature]
      , subtypes := [^Faerie, ^Rogue]
      , abilities :=
          [ keyword Flash
          , keyword Flying
          , Static (cant (Enact Block (SameAs This) (Not (HasKeyword Flying))))   -- "can block only creatures with flying"
          ]
      , power := Just 3
      , toughness := Just 1
      })
  (^: { name := Just "Petty Theft"
      , manaCost := [^1, ^Blue]
      , types := [Instant]
      , abilities := [ Spell (Targeted [Target (^1) (And [permanent, Not (HasType Land)])] (Act (Move (GetTarget 0) (ToZone Hand)))) ]
      })

-- Delver of Secrets // Insectile Aberration — a TRANSFORMING DFC ([CR#712]). The front's upkeep trigger
-- reads the top card (full-info, so the "look" is just the condition) and MAY reveal an instant/sorcery
-- to `Transform This` to the back. `TwoFaced Transforming` holds both faces.
export
card_DelverOfSecrets : Card
card_DelverOfSecrets = TwoFaced Transforming
  (^: { name := Just "Delver of Secrets"
      , manaCost := [^Blue]
      , types := [Creature]
      , subtypes := [^Human, ^Wizard]
      , abilities :=
          [ Triggered (MkQuery [BeginStep (BeginningPhase UpkeepStep)] [DuringTurn you])
              (If (Matches (Single (TopOfLibrary (^1))) (Or [HasType Instant, HasType Sorcery]))
                  (May (Sequence [ Act (Reveal (Single (TopOfLibrary (^1))))
                                 , Act (Transform This) ])))
          ]
      , power := Just 1
      , toughness := Just 1
      })
  (^: { name := Just "Insectile Aberration"
      , types := [Creature]
      , subtypes := [^Human, ^Insect]
      , abilities := [ keyword Flying ]
      , power := Just 3
      , toughness := Just 2
      })

-- Furnace of Rath — PAYLOAD replacement ([CR#616]): "if a source would deal damage, it deals double
-- instead." `ReplaceAmount` keeps the damage event but scales its amount to `Times ThatMuch (^2)`.
export
card_FurnaceOfRath : Card
card_FurnaceOfRath = Normal $ ^:
  { name := Just "Furnace of Rath"
  , manaCost := [^3, ^Red]
  , types := [Enchantment]
  , abilities := [ Static (ReplaceAmount (MkQuery [DealDamage Nothing] []) (Times ThatMuch (^2))) ]
  }

-- Doubling Season — two payload replacements: twice the tokens you'd create, and twice the counters
-- that'd be placed on a permanent you control. The token/counter EVENT-KINDS reuse the verb names.
export
card_DoublingSeason : Card
card_DoublingSeason = Normal $ ^:
  { name := Just "Doubling Season"
  , manaCost := [^4, ^Green]
  , types := [Enchantment]
  , abilities :=
      [ Static (ReplaceAmount (MkQuery [CreateToken] [Actor you]) (Times ThatMuch (^2)))
      , Static (ReplaceAmount (MkQuery [PutCounters] [Patient (ControlledBy you)]) (Times ThatMuch (^2)))
      ]
  }

-- Time Walk — "target player takes an extra turn after this one." `ExtraTurn {actor = …}` on the
-- targeted player ([CR#505]).
export
card_TimeWalk : Card
card_TimeWalk = Normal $ ^:
  { name := Just "Time Walk"
  , manaCost := [^1, ^Blue]
  , types := [Sorcery]
  , abilities := [ Spell (Targeted [Target (^1) Anyone] (Act (ExtraTurn {actor = GetTarget 0}))) ]
  }

-- Mindslaver — "{T}, Sacrifice Mindslaver: You control target player during that player's next turn."
-- `ControlPlayer` on the targeted player; the next-turn duration is the engine's ([CR#723]).
export
card_Mindslaver : Card
card_Mindslaver = Normal $ ^:
  { name := Just "Mindslaver"
  , manaCost := [^6]
  , types := [Artifact]
  , abilities :=
      [ Activated (Costs [Do (Tap This), Do (Sacrifice You (SameAs This))])
          (Targeted [Target (^1) Anyone] (Act (ControlPlayer (GetTarget 0)))) ]
  }

-- Mind Bend — TEXT-CHANGE ([CR#612]): "change the text of target permanent or spell by replacing one
-- color word or basic land type with another." A continuous `ChangeText` mod naming the operation +
-- the eligible word classes; the two specific words are the caster's resolution-time choice.
export
card_MindBend : Card
card_MindBend = Normal $ ^:
  { name := Just "Mind Bend"
  , manaCost := [^1, ^Blue]
  , types := [Sorcery]
  , abilities :=
      [ Spell (Targeted [Target (^1) (Or [permanent, IsKind IsSpell])]
          (Continuously (Modify (SelectAll (SameAs (GetTarget 0))) [ChangeText [ColorWords, BasicLandTypes]]) Permanent)) ]
  }

-- Flooded Strand — a FETCH LAND: {T}, pay 1 life, sacrifice it → search your library for a Plains or
-- Island card, put it onto the battlefield, then shuffle. (Fetches anything with those land types,
-- so duals/shocks too — hence the bare subtype filter, no `Basic` supertype.)
export
card_FloodedStrand : Card
card_FloodedStrand = Normal $ ^:
  { name := Just "Flooded Strand"
  , types := [Land]
  , abilities :=
      [ Activated (Costs [Do (Tap This), Do (LoseLife (^1)), Do (Sacrifice You (SameAs This))])
          (With (Search {from = [Library]} (^1) (Or [HasSubtype (^Plains), HasSubtype (^Island)]))
            (Sequence [ Each That (Act (Move It (ToZone Battlefield)))
                      , Act Shuffle ])) ]
  }

-- Aether Hub — a PLAYER-COUNTER (energy) demo. "You get {E}{E}" is `PutCounters Energy (^2) You`, which
-- typechecks ONLY because `counterScope Energy = APlayer` (energy on an object is a type error);
-- "Pay {E}" is `Do (RemoveCounters Energy (^1) You)` — energy rides `Do` like any cost-as-action
-- ([CR#118.3]); no dedicated `PayEnergy` verb. The dependent carrier puts the counter on the player by type.
export
card_AetherHub : Card
card_AetherHub = Normal $ ^:
  { name := Just "Aether Hub"
  , types := [Land]
  , abilities :=
      [ Triggered (thisEnters)
          (Act (PutCounters Energy (^2) You))                                  -- "you get {E}{E}"
      , Activated (Do (Tap This)) (Act (AddMana (^1) (^Colorless)))                         -- {T}: Add {C}
      , Activated (Costs [Do (Tap This), Do (RemoveCounters Energy (^1) You)]) (Act (AddMana (^1) AnyColor))  -- {T}, Pay {E}: add one mana of any color
      ]
  }

-- Thorn of the Black Rose — a PLAYER designation (monarch): ETB → "you become the monarch" =
-- `GrantDesignation Monarch You`, which typechecks because `designationScope Monarch = APlayer`.
export
card_ThornOfTheBlackRose : Card
card_ThornOfTheBlackRose = Normal $ ^:
  { name := Just "Thorn of the Black Rose"
  , manaCost := [^2, ^Black]
  , types := [Creature]
  , subtypes := [^Human]
  , abilities :=
      [ keyword Deathtouch
      , Triggered (thisEnters)
          (Act (GrantDesignation Monarch You))   -- "you become the monarch"
      ]
  , power := Just 1
  , toughness := Just 4
  }

-- Fleecemane Lion — an OBJECT designation (monstrous): Monstrosity grants it (`GrantDesignation
-- Monstrous This`), and the statics read it (`HasDesignation Monstrous`, an object test) to confer
-- hexproof AND indestructible while monstrous. Indestructible needs no new construct — it's `Replaces`
-- (the destroy of This) with `Sequence []` (a pure skip).
export
card_FleecemaneLion : Card
card_FleecemaneLion = Normal $ ^:
  { name := Just "Fleecemane Lion"
  , manaCost := [^Green, ^White]
  , types := [Creature]
  , subtypes := [^Cat]
  , abilities :=
      [ monstrosity (Mana [^3, ^Green, ^White]) (^1)                       -- Monstrosity 1
      , Static (While (Matches This (HasDesignation Monstrous))
          (Modify (SelectAll (SameAs This)) [ GrantAbility (keyword (Hexproof Nothing))
                       , GrantAbility (keyword Indestructible) ]))          -- while monstrous: hexproof + indestructible
      ]
  , power := Just 3
  , toughness := Just 3
  }

-- Goblin Electromancer — continuous COST reduction: "instant and sorcery spells you cast cost {1}
-- less" = a `CostModifier` over a spell filter carrying `Reduce`.
export
card_GoblinElectromancer : Card
card_GoblinElectromancer = Normal $ ^:
  { name := Just "Goblin Electromancer"
  , manaCost := [^Blue, ^Red]
  , types := [Creature]
  , subtypes := [^Goblin, ^Wizard]
  , abilities :=
      [ Static (CostModifier (And [Or [HasType Instant, HasType Sorcery], ControlledBy you])
          (Reduce [Mana [^1]])) ]
  , power := Just 2
  , toughness := Just 2
  }

-- Thalia, Guardian of Thraben — a TAXER: "noncreature spells cost {1} more" = `Increase`.
export
card_Thalia : Card
card_Thalia = Normal $ ^:
  { name := Just "Thalia, Guardian of Thraben"
  , manaCost := [^1, ^White]
  , types := [Creature]
  , supertypes := [Legendary]
  , subtypes := [^Human]
  , abilities :=
      [ keyword FirstStrike
      , Static (CostModifier (And [IsKind IsSpell, Not (HasType Creature)]) (Increase [Mana [^1]])) ]
  , power := Just 2
  , toughness := Just 1
  }

-- Frogmite — AFFINITY for artifacts: "this costs {1} less for each artifact you control." A SELF
-- `CostModifier` whose `Reduce` is `ScaledBy` the artifact count — affinity needs no own constructor.
export
card_Frogmite : Card
card_Frogmite = Normal $ ^:
  { name := Just "Frogmite"
  , manaCost := [^4]
  , types := [Artifact, Creature]
  , abilities :=
      [ Static (CostModifier (SameAs This)
          (ScaledBy (Reduce [Mana [^1]]) (CountMatching (And [HasType Artifact, ControlledBy you])))) ]
  , power := Just 2
  , toughness := Just 2
  }

-- Gaea's Cradle — VARIABLE mana: "{T}: Add {G} for each creature you control" = `AddMana (CountMatching …) (^Green)`.
export
card_GaeasCradle : Card
card_GaeasCradle = Normal $ ^:
  { name := Just "Gaea's Cradle"
  , types := [Land]
  , supertypes := [Legendary]
  , abilities :=
      [ Activated (Do (Tap This)) (Act (AddMana (CountMatching (And [permanent, creature, ControlledBy you])) (^Green))) ]
  }

-- Karametra's Acolyte — DEVOTION: "{T}: Add {G} equal to your devotion to green" ([CR#700.5]). Devotion
-- decomposes to `Aggregate SumOf` of each permanent-you-control's green-pip count — still a `Count`, so it
-- drops into mana production for free (the bespoke `Devotion` constructor is gone).
export
card_KarametrasAcolyte : Card
card_KarametrasAcolyte = Normal $ ^:
  { name := Just "Karametra's Acolyte"
  , manaCost := [^3, ^Green]
  , types := [Creature]
  , subtypes := [^Human]
  , abilities :=
      [ Activated (Do (Tap This))
          (Act (AddMana (Aggregate SumOf (eachOf (And [permanent, ControlledBy you])
                                                 (CountOf (ManaSymbols It (CountsAs Green)))))
                        (^Green))) ]
  , power := Just 1
  , toughness := Just 4
  }

-- Coat of Arms — the anthem SELF-REFERENCE that motivated Rust's `Subject`, here via `It`. "Each
-- creature gets +1/+1 for each OTHER creature that shares a creature type with it": `ModifyAll` binds
-- the anthem'd creature as `It`, and the per-subject `CountOf` already has its OWN implicit candidate
-- (the counted creature), so `It` names just the outer one. No `Subject`, no `Where`.
export
card_CoatOfArms : Card
card_CoatOfArms = Normal $ ^:
  { name := Just "Coat of Arms"
  , manaCost := [^2]
  , types := [Artifact]
  , abilities :=
      [ Static (Modify (SelectAll creature)
          [ Alter Power (Up (CountMatching (And [permanent, creature, SharesSubtype It, Not (SameAs It)])))
          , Alter Toughness (Up (CountMatching (And [permanent, creature, SharesSubtype It, Not (SameAs It)]))) ]) ]
  }

-- Platinum Angel — the OUTCOME gate: "you can't lose the game and your opponents can't win." Two
-- `OutcomeGate` statics — a dedicated channel, since game-loss is neither a deontic action nor a
-- replaceable event ([CR#104.3a]).
export
card_PlatinumAngel : Card
card_PlatinumAngel = Normal $ ^:
  { name := Just "Platinum Angel"
  , manaCost := [^7]
  , types := [Artifact, Creature]
  , subtypes := [^Angel]
  , abilities :=
      [ keyword Flying
      , Static (OutcomeGate CantLose you)
      , Static (OutcomeGate CantWin opponent)
      ]
  , power := Just 4
  , toughness := Just 4
  }

-- Darksteel Citadel — INDESTRUCTIBLE as a keyword (its `Composite` desugars to the `Replaces`-the-
-- destroy-with-`Sequence []` skip, so the `Replaces`-empty machinery subsumes Rust's `CantHappen`).
export
card_DarksteelCitadel : Card
card_DarksteelCitadel = Normal $ ^:
  { name := Just "Darksteel Citadel"
  , types := [Artifact, Land]
  , abilities :=
      [ keyword Indestructible                              -- Indestructible
      , Activated (Do (Tap This)) (Act (AddMana (^1) (^Colorless)))      -- {T}: Add {C}
      ]
  }

-- Mutagenic Growth — PHYREXIAN mana ({G/P}: pay {G} or 2 life): target creature gets +2/+2.
export
card_MutagenicGrowth : Card
card_MutagenicGrowth = Normal $ ^:
  { name := Just "Mutagenic Growth"
  , manaCost := [Phyrexian Green Nothing]
  , types := [Instant]
  , abilities :=
      [ Spell (Targeted [Target (^1) creature]
          (Continuously (Modify (SelectAll (SameAs (GetTarget 0))) [Alter Power (Up (^2)), Alter Toughness (Up (^2))]) UntilEndOfTurn)) ]
  }

-- Skred — SNOW mana ({S}): deals damage to target creature equal to the snow permanents you control.
export
card_Skred : Card
card_Skred = Normal $ ^:
  { name := Just "Skred"
  , manaCost := [SnowMana, ^Red]
  , types := [Sorcery]
  , abilities :=
      [ Spell (Targeted [Target (^1) creature]
          (Act (DealDamage (GetTarget 0) (CountMatching (And [permanent, HasSupertype Snow, ControlledBy you]))))) ]
  }

-- History of Benalia — a SAGA. The `Saga` subtype CONFERS the lore-increment (`subtypeConfers (^Saga)`
-- = a `TurnBased` ability adding a Lore counter each precombat main), so the card only spells its
-- CHAPTERS (triggered on the Lore count) + the final-chapter `Sba` sacrifice.
export
card_HistoryOfBenalia : Card
card_HistoryOfBenalia = Normal $ ^:
  { name := Just "History of Benalia"
  , manaCost := [^1, ^White]
  , types := [Enchantment]
  , subtypes := [^Saga]
  , abilities :=
      [ -- I, II — create a 2/2 white Knight
        Triggered (MkQuery [PutCounters] [Patient (SameAs This)])
          (If (Or [ Compare (CountersOn Lore This) Equal (^1)
                  , Compare (CountersOn Lore This) Equal (^2) ])
              (Act (CreateToken (^1)
                (^: { name := Just "Knight", types := [Creature], subtypes := [^Knight]
                    , colors := [White], power := Just 2, toughness := Just 2 }))))
      , -- III — Knights you control get +2/+1 until end of turn
        Triggered (MkQuery [PutCounters] [Patient (SameAs This)])
          (If (Compare (CountersOn Lore This) Equal (^3))
              (Continuously (Modify (SelectAll (And [HasSubtype (^Knight), ControlledBy you])) [Alter Power (Up (^2)), Alter Toughness (Up (^1))]) UntilEndOfTurn))
      , -- sacrifice after the final chapter ([CR#714.4])
        Static (Sba (Compare (CountersOn Lore This) GreaterEq (^3)) (Act (Move This (ToZone Graveyard))))
      ]
  }

-- Meddling Mage — NAME read-back: "as ~ enters, choose a card name; spells with the chosen name can't
-- be cast." `AsEnters AName` notes the name; `OfChosen` at that binding reads "has the chosen name"
-- (the same anaphor as the color/type cases — `AName` just joined `IsCharDomain`).
export
card_MeddlingMage : Card
card_MeddlingMage = Normal $ ^:
  { name := Just "Meddling Mage"
  , manaCost := [^White, ^Blue]
  , types := [Creature]
  , subtypes := [^Human, ^Wizard]
  , abilities :=
      [ AsEnters AName
          [ Static (cant (Enact Cast Anyone (And [IsKind IsSpell, OfChosen]))) ] ]
  , power := Just 2
  , toughness := Just 2
  }

-- Vodalian Illusionist — PHASING: "{2}, {T}: Target creature phases out." `PhaseOut` verb +
-- `PhasedOut` state; phasing back in is the engine's turn-based action.
export
card_VodalianIllusionist : Card
card_VodalianIllusionist = Normal $ ^:
  { name := Just "Vodalian Illusionist"
  , manaCost := [^1, ^Blue]
  , types := [Creature]
  , subtypes := [^Merfolk, ^Wizard]
  , abilities :=
      [ Activated (Costs [Mana [^2], Do (Tap This)])
          (Targeted [Target (^1) creature] (Act (PhaseOut (GetTarget 0)))) ]
  , power := Just 1
  , toughness := Just 1
  }

-- Smuggler's Copter — a VEHICLE with CREW (the aggregate-stat cost): "Crew 1" = tap creatures with
-- total power ≥ 1 to make this Vehicle an artifact creature until end of turn. + loot on attack/block
-- (the `Begins Attack`/`Begins Block` onset event).
export
card_SmugglersCopter : Card
card_SmugglersCopter = Normal $ ^:
  { name := Just "Smuggler's Copter"
  , manaCost := [^2]
  , types := [Artifact]
  , subtypes := [^Vehicle]
  , abilities :=
      [ keyword Flying
      , Triggered (MkQuery [Begins Attack, Begins Block] [Agent (SameAs This)])
          (May (Sequence [Act (Draw (^1)), Act (Discard (^1))]))   -- loot on attack or block
      , crew (^1)                                                  -- Crew 1
      ]
  , power := Just 3
  , toughness := Just 3
  }

-- Solemnity — "players can't get counters; counters can't be put on permanents." A PROHIBITION (not a
-- replace-with-nothing): `CantHappen` the counter-placement event — reaching counters because the
-- `PutCounters` event-kind exists.
export
card_Solemnity : Card
card_Solemnity = Normal $ ^:
  { name := Just "Solemnity"
  , manaCost := [^1, ^White]
  , types := [Enchantment]
  , abilities := [ Static (CantHappen (MkQuery [PutCounters] [])) ]
  }

-- Pine Walker — a clean vanilla MORPH creature: "{4}{G} 5/5; Morph {4}{G}." The `morph` macro is the
-- whole encoding — cast face down as a 2/2 for {3} + turn up for {4}{G}; the face-down 2/2 body is the
-- engine's global [CR#708.2] rule, not on the card.
export
card_PineWalker : Card
card_PineWalker = Normal $ ^:
  { name := Just "Pine Walker"
  , manaCost := [^4, ^Green]
  , types := [Creature]
  , subtypes := [^Elemental]
  , abilities := [ morph (Mana [^4, ^Green]) ]
  , power := Just 5
  , toughness := Just 5
  }

-- Cackling Counterpart — token COPY: "Create a token that's a copy of target creature you control."
-- `CreateTokenCopy` over a target; no alterations, so nothing layers on. (Flashback omitted — cast-from-
-- graveyard is a separate gap.)
export
card_CacklingCounterpart : Card
card_CacklingCounterpart = Normal $ ^:
  { name := Just "Cackling Counterpart"
  , manaCost := [^1, ^Blue, ^Blue]
  , types := [Instant]
  , abilities :=
      [ Spell (Targeted [Target (^1) (And [creature, ControlledBy you])]
          (Act (CreateTokenCopy (GetTarget 0)))) ]
  }

-- Tarmogoyf — the canonical CDA: "*/1+*, where * is the number of card types among cards in all
-- graveyards." No printed P/T (the fields are omitted); a `Static (Modify (SelectAll (SameAs This)) [Set …])` DEFINES them
-- from `typesInGraveyards` (a `CountDistinct OfCardType` over graveyards), toughness = power + 1. Real, not a stand-in.
export
card_Tarmogoyf : Card
card_Tarmogoyf = Normal $ ^:
  { name := Just "Tarmogoyf"
  , manaCost := [^1, ^Green]
  , types := [Creature]
  , abilities :=
      [ Static (Modify (SelectAll (SameAs This)) [ Alter Power (Set typesInGraveyards)
                            , Alter Toughness (Set (Plus typesInGraveyards (Literal 1))) ]) ]
  }

-- Drudge Skeletons — REGENERATION: "{B}: Regenerate this." The `regenerate` macro sets up the one-shot
-- this-turn shield (a `Replaces` of the next destroy with heal-tap-remove, `{limit = UpTo 1}`); the
-- heal-tap-remove is spelled from primitives (RemoveAllDamage / Tap / RemoveFromCombat).
export
card_DrudgeSkeletons : Card
card_DrudgeSkeletons = Normal $ ^:
  { name := Just "Drudge Skeletons"
  , manaCost := [^1, ^Black]
  , types := [Creature]
  , abilities := [ Activated (Mana [^Black]) regenerate ]
  , power := Just 1
  , toughness := Just 1
  }

-- White Knight — PROTECTION: "First strike; protection from black." The `protection (HasColor Black)`
-- macro is the whole DEBT bundle (can't be damaged/enchanted/blocked/targeted by black) in one clause.
export
card_WhiteKnight : Card
card_WhiteKnight = Normal $ ^:
  { name := Just "White Knight"
  , manaCost := [^White, ^White]
  , types := [Creature]
  , subtypes := [^Human, ^Knight]
  , abilities := [ keyword FirstStrike, protection (HasColor Black) ]
  , power := Just 2
  , toughness := Just 2
  }

-- Garza Zol, Plague Queen — the DAMAGE-PROVENANCE + COMBAT-flag exemplar. Ability 1 is a death trigger
-- gated on `DamagedBy This` (the turn-scoped "a creature dealt damage by ~ this turn"); ability 2 reads
-- the combat boolean now on the damage event — `DealDamage (Just True)` = "deals combat damage" — and the
-- draw is OPTIONAL (`May`), faithful to "you may draw a card".
export
card_GarzaZol : Card
card_GarzaZol = Normal $ ^:
  { name := Just "Garza Zol, Plague Queen"
  , manaCost := [^4, ^Blue, ^Black, ^Red]
  , types := [Creature]
  , supertypes := [Legendary]
  , subtypes := [^Vampire, ^Noble]
  , abilities :=
      [ keyword Flying
      , haste
      , -- "Whenever a creature dealt damage by ~ this turn dies, put a +1/+1 counter on ~."
        Triggered
          (MkQuery [ZoneChanged (Just Battlefield) (Just Graveyard)]
                   [Agent (And [creature, DamagedBy This])])
          (Act (PutCounters P1P1 (^1) This))
      , -- "Whenever ~ deals combat damage to a player, you may draw a card."
        Triggered
          (MkQuery [DealDamage (Just True)] [Agent (SameAs This), Patient Anyone])
          (May (Act (Draw (^1))))
      ]
  , power := Just 5
  , toughness := Just 5
  }

--:vim:sts=2 sw=2:

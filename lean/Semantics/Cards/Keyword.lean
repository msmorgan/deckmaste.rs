import Semantics
import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Cards.Trigger

/-!
# Semantics.Cards.Keyword

Port of `idris/src/Experimental/Cards/Keyword.idr`: the printed cards of the Keyword family
(keyword abilities, their parameters and grants) and the bench items beside them.

`battleSquadron` prints a `*/*` box: those slots are `none` under the printed-star ruling.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Cards

def jump : Action := gain (target isCreature) flying (some untilEndOfTurn)
theorem okJump : Action.check (DRS.ofList []) jump = [] := by decide
def gabrielAngelfire : Action :=
  sequentially
    [ choose (a (qualityFrom .ability
        (abilitiesAmong [flying, firstStrike, trample, .theWith rampage 3]))),
      gain thisCreature (thatAbility .theChoice) (some (untilEvent (beginningOf .upkeep (some you) nextTime))) ]
theorem okGabrielAngelfire : Action.check (DRS.ofList []) gabrielAngelfire = [] := by decide
def builtToSmash : Action :=
  sequentially
    [ get (target (and [isCreature, isAttacking])) (up (.lit 3)) (up (.lit 3)) (some untilEndOfTurn),
      doIf (matches_ it (and [isArtifact, isCreature]))
        (gain it trample (some untilEndOfTurn)) ]
theorem okBuiltToSmash : Action.check (DRS.ofList []) builtToSmash = [] := by decide

/-- A flying Thopter token. -/
def thopterToken : CharacteristicBundle :=
  { characteristics :=
    { types := [.artifact, .creature], subtypes := [thopter],
      text := [flying], power := stat 1, toughness := stat 1 } }

def aviationPioneer : Action := createToken thopterToken
theorem okAviationPioneer : Action.check (DRS.ofList []) aviationPioneer = [] := by decide
def fireNavyTrebuchet : Action :=
  createTappedAttacking (.lit 1)
    { characteristics :=
      { name := "Ballistic Boulder", types := [.artifact, .creature],
        subtypes := [construct], text := [flying],
        power := stat 2, toughness := stat 1 } }
theorem okFireNavyTrebuchet : Action.check (DRS.ofList []) fireNavyTrebuchet = [] := by decide

def rorixBladewing : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rorix Bladewing", cost := some [generic 3, pip .red, pip .red, pip .red],
      supertypes := [.legendary], types := [.creature], subtypes := [dragon],
      text := [flying, haste], power := stat 6, toughness := stat 5 } }

def yotianSoldier : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Yotian Soldier", cost := some [generic 3], types := [.artifact, .creature],
      subtypes := [soldier], text := [vigilance],
      power := stat 1, toughness := stat 4 } }

/-- Pym Particles -/
def pymParticlesGrant : Action :=
  sequentially
    [ gain (target isCreature) vigilance (some untilEndOfTurn),
      establishThroughout (cant any (block (that creature))) thisTurn ]
theorem okPymParticlesGrant : Action.check (DRS.ofList []) pymParticlesGrant = [] := by decide
def bladebrand : Action := gain (target isCreature) deathtouch (some untilEndOfTurn)
theorem okBladebrand : Action.check (DRS.ofList []) bladebrand = [] := by decide
def criticalHit : Action :=
  gain (target isCreature) doubleStrike (some untilEndOfTurn)
theorem okCriticalHit : Action.check (DRS.ofList []) criticalHit = [] := by decide
def lightningBlow : Action :=
  gain (target isCreature) firstStrike (some untilEndOfTurn)
theorem okLightningBlow : Action.check (DRS.ofList []) lightningBlow = [] := by decide
def deathByDragons : Action :=
  instruct (each (and [isPlayer, otherThan (target isPlayer)])) (createToken { characteristics :=
        { colors := [.red], types := [.creature], subtypes := [dragon],
          text := [flying], power := stat 5, toughness := stat 5 } })
theorem okDeathByDragons : Action.check (DRS.ofList []) deathByDragons = [] := by decide

/-- Concordant Crossroads -/
def concordantCrossroads : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Concordant Crossroads", cost := some [pip .green], supertypes := [.world],
      types := [.enchantment], text := [static (abilityGrant (allOf isCreature) haste)]
          } }

/-- Bristlepack Sentry -/
def bristlepackSentry : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bristlepack Sentry", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [plant, wolf],
      text :=
        [ defender,
          static (onlyWhile
            (can thisCreature attack
               (asThough := some
                  (holds (matches_ it (not (hasKeyword defender))))))
            (exists_ (and [ isCreature, youControl,
                             hasPower .atLeast (.lit 4) ]))) ],
      power := stat 3, toughness := stat 3 } }

def platinumAngel : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Platinum Angel", cost := some [generic 7], types := [.artifact, .creature],
      subtypes := [angel],
      text :=
        [ flying,
          static (cant you loseGame),
          static (cant yourOpponents winGame) ],
      power := stat 4, toughness := stat 4 } }

def abyssalPersecutor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Abyssal Persecutor", cost := some [generic 2, pip .black, pip .black],
      types := [.creature], subtypes := [demon],
      text :=
        [ flying, trample,
          static (cant you winGame),
          static (cant yourOpponents loseGame) ],
      power := stat 6, toughness := stat 6 } }

def smogElemental : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Smog Elemental", cost := some [generic 4, pip .black, pip .black],
      types := [.creature], subtypes := [elemental],
      text :=
        [ flying,
          static (getsPt
            (allOf (and [ isCreature, hasKeyword flying,
                           opponentsControl ]))
            (down (.lit 1)) (down (.lit 1))) ],
      power := stat 3, toughness := stat 3 } }

def exquisiteArchangel : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Exquisite Archangel", cost := some [generic 5, pip .white, pip .white],
      types := [.creature], subtypes := [angel],
      text :=
        [ flying,
          static (replace (would you loseGame) (instead (sequentially
              [ exile thisCreature,
                setLife (statOf (.playerStat .startingLifeTotal) you) ]))) ],
      power := stat 5, toughness := stat 5 } }

def adelbertSteiner : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Adelbert Steiner", cost := some [generic 1, pip .white], supertypes := [.legendary],
      types := [.creature], subtypes := [human, knight],
      text :=
        [ lifelink,
          static (getsPt thisCreature
            (up (forEach (.lit 1) (and [hasSubtype equipment, youControl])))
            (up (forEach (.lit 1) (and [hasSubtype equipment, youControl])))) ],
      power := stat 2, toughness := stat 1 } }

def inspiringStatuary : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Inspiring Statuary", cost := some [generic 3], types := [.artifact],
      text :=
        [ static (abilityGrant (allOf (and [non isArtifact, isSpell, castBy you])) improvise) ] } }

def chiefEngineer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chief Engineer", cost := some [generic 1, pip .blue], types := [.creature],
      subtypes := [vedalken, artificer],
      text := [static (abilityGrant (allOf (and [isArtifact, isSpell, castBy you])) convoke)],
      power := stat 1, toughness := stat 3 } }

def firesongAndSunspeaker : Ability :=
  static (abilityGrant
    (allOf (and [hasColor .red, isInstantOrSorcery, isSpell, youControl]))
    lifelink)
theorem okFiresongAndSunspeaker : Ability.check (DRS.ofList []) firesongAndSunspeaker = [] := by decide
def briaRiptideRogue : Ability :=
  static (abilityGrant (allOf (and [isCreature, youControl, otherThan thisCreature])) prowess)
theorem okBriaRiptideRogue : Ability.check (DRS.ofList []) briaRiptideRogue = [] := by decide
def narsetEnlightenedExile : Ability :=
  static (abilityGrant (allOf (and [isCreature, youControl])) prowess)
theorem okNarsetEnlightenedExile : Ability.check (DRS.ofList []) narsetEnlightenedExile = [] := by decide
def pontiffOfBlight : Ability :=
  static (abilityGrant (allOf (and [isCreature, youControl, otherThan thisCreature])) extort)
theorem okPontiffOfBlight : Ability.check (DRS.ofList []) pontiffOfBlight = [] := by decide

def prismariTheInspiration : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Prismari, the Inspiration", cost := some [generic 5, pip .blue, pip .red],
      supertypes := [.legendary], types := [.creature],
      subtypes := [elder, dragon],
      text :=
        [ flying,
          ward (payLife 5),
          static (abilityGrant (allOf (and [isInstantOrSorcery, isSpell, castBy you])) storm) ],
      power := stat 7, toughness := stat 7 } }

def tomakulHonorGuard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tomakul Honor Guard", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [human, soldier],
      text := [ward (mana [generic 2])], power := stat 3, toughness := stat 1 } }

def repentantBlacksmith : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Repentant Blacksmith", cost := some [generic 1, pip .white], types := [.creature],
      subtypes := [human],
      text := [protection (hasColor .red)], power := stat 1, toughness := stat 2 } }

def battleSquadron : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Battle Squadron", cost := some [generic 3, pip .red, pip .red], types := [.creature],
      subtypes := [goblin],
      text :=
        [ flying,
          static (ptDefinition thisCreature .bothEach (countOf (and [isCreature, youControl]))) ] } }

def eomerOfTheRiddermark : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Éomer of the Riddermark", cost := some [generic 4, pip .red],
      supertypes := [.legendary], types := [.creature],
      subtypes := [human, knight],
      text :=
        [ haste,
          triggeredIf (does thisCreature attack)
            (exists_ (and [ isCreature, youControl,
                             superlative .max (.stat .power) (and [isCreature, isPermanent]) ]))
            (createToken (creatureToken 1 1 [.white] [human, soldier])) ],
      power := stat 5, toughness := stat 4 } }

def angelicGift : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Angelic Gift", cost := some [generic 1, pip .white], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ enchant isCreature,
          when (does thisAura enter) (draw (.lit 1)),
          static (abilityGrant (enchanted creature) flying) ] }
              }

def consulsLieutenant : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Consul's Lieutenant", cost := some [pip .white, pip .white], types := [.creature],
      subtypes := [human, soldier],
      text :=
        [ firstStrike,
          renown (.lit 1),
          triggeredIf (does thisCreature attack) (matches_ it (hasDesignation renowned))
            (get (allOf (and [isCreature, isAttacking, youControl, otherThan
                thisCreature]))
              (up (.lit 1)) (up (.lit 1)) (some untilEndOfTurn)) ],
      power := stat 2, toughness := stat 1 } }

def secretsOfTheGoldenCity : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Secrets of the Golden City", cost := some [generic 1, pip .blue, pip .blue],
      types := [.sorcery],
      text :=
        [ ascend,
          spell (replace (draw (.lit 2))
            (doIf (matches_ you (hasDesignation citysBlessing)) (instead (draw (.lit 3))))) ] } }

def bombur : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bombur, Gentle Dreamer", cost := some [generic 2, pip .red],
      supertypes := [.legendary], types := [.creature],
      subtypes := [dwarf, bard],
      text :=
        [ storied,
          static (onlyUnless (doesntUntap thisCreature (some you))
            (matches_ you (hasDesignation enduringStory))) ],
      power := stat 5, toughness := stat 3 } }

def drachNyen : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Drach'Nyen", cost := some [generic 4, pip .black, pip .red],
      supertypes := [.legendary], types := [.artifact], subtypes := [equipment],
      text :=
        [ when (does thisEquipment enter) (exile (targets (upTo 1) isCreature)),
          static (conjunction
                    [ abilityGrant (equipped creature) menace,
              getsPt (equipped creature) (up (letter .x)) (up (.lit 0)),
              letterDefinition .x (powerOf (the (in_ (exiledWith thisEquipment)))) ]),
          equip (mana [generic 2]) ] } }

def colossalGraveReaver : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Colossal Grave-Reaver", cost := some [generic 6, pip .black, pip .green],
      types := [.creature], subtypes := [dragon],
      text :=
        [ flying,
          triggeredOr (does thisCreature enter) [does thisCreature attack] (mill (.lit 3)),
          whenever
            (does any
               (putInto (counted (atLeast 1) (and [isCard, isCreature])) yourGraveyard
                  (from_ := some yourLibrary)))
            (putOntoBattlefield (someOf (exactly 1) them)) ],
      power := stat 7, toughness := stat 6 } }

/-- Grimdancer -/
def grimdancer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Grimdancer", cost := some [generic 1, pip .black, pip .black], types := [.creature],
      subtypes := [nightmare],
      text :=
        [ static (enters thisCreature
            [withCounters (.lit 2)
              (distinctChosenKinds [menaceCounter, deathtouchCounter, lifelinkCounter]) .fresh]) ],
      power := stat 3, toughness := stat 3 } }

def divineVisitation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Divine Visitation", cost := some [generic 3, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ static (replace (would any (createObject anyAmount (token (copyOf (counted (atLeast 1) (and [isCreature, isToken]))) []) false (some you))) (instead (createToken
              { characteristics :=
                { colors := [.white], types := [.creature], subtypes := [angel],
                  text := [flying,
                           vigilance], power := stat 4, toughness := stat 4 } } (count :=
                                                                                   groupSize)))) ] } }

def adrixAndNev : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Adrix and Nev, Twincasters", cost := some [generic 2, pip .green, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [merfolk, wizard],
      text :=
        [ ward (mana [generic 2]),
          static (replace (would any (createObject anyAmount (token (copyOf (counted (atLeast 1) isToken)) []) false (some
              you))) (instead (create (times (.lit 2) groupSize) asThose []))) ],
      power := stat 2, toughness := stat 2 } }

def windZendikon : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Wind Zendikon", cost := some [pip .blue], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ enchant isLand,
          static (qualityChange (enchanted land) .sets
            (bundle
              { characteristics :=
                { colors := [.blue], types := [.creature], subtypes := [elemental],
                  text := [flying], power := stat 2, toughness := stat 2 } }
              (some .land))),
          when (does (enchanted land) die) (putInto (that card) hand) ] } }

def awakenTheBear : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Awaken the Bear", cost := some [generic 2, pip .green], types := [.instant],
      text :=
        [ spell
            (establishThroughout
               (conjunction [ getsPt (target isCreature) (up (.lit 3)) (up (.lit 3)),
                abilityGrant it trample ]) untilEndOfTurn) ] } }

def spidersilkArmor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Spidersilk Armor", cost := some [generic 2, pip .green], types := [.enchantment],
      text :=
        [ static (conjunction [ getsPt (allOf (and [isCreature, youControl])) (up (.lit 0)) (up
                  (.lit 1)),
              abilityGrant them reach ]) ] } }

def arcaneFlight : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Arcane Flight", cost := some [pip .blue], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ enchant isCreature,
          static (conjunction [ getsPt (enchanted creature) (up (.lit 1)) (up (.lit 1)),
              abilityGrant it flying ]) ] } }

def bootsOfSpeed : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Boots of Speed", cost := some [pip .red], types := [.artifact],
      subtypes := [equipment],
      text :=
        [ static (conjunction [ getsPt (equipped creature) (up (.lit 1)) (up (.lit 0)),
              abilityGrant it haste ]),
          equip (mana [generic 1]) ] } }

/-- "enchanted creature loses all abilities and is a <colors> <subtype> with base P/T" -/
def enchantedBecomesVanilla (colors : List Color) (subtype : String) (power toughness : Nat) :
    StaticSpec :=
  conjunction [ allAbilityLoss (enchanted creature),
      qualityChange it .sets
        (bundle { characteristics := { colors, types := [.creature], subtypes := [creatureType subtype] } }),
      getsPt it (setTo (.lit power)) (setTo (.lit toughness)) ]

def frogify : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Frogify", cost := some [generic 1, pip .blue], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ enchant isCreature,
          static (enchantedBecomesVanilla [.blue] "Frog" 1 1) ] } }

def darksteelMutation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Darksteel Mutation", cost := some [generic 1, pip .white], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ enchant isCreature,
          static (conjunction [ qualityChange (enchanted creature) .sets
                (bundle
                   { characteristics := { types := [.artifact, .creature], subtypes := [insect] }
                       }),
              getsBase it (.lit 0) (.lit 1),
              abilityGrant it indestructible,
              allAbilityLoss it ]) ] } }

def kenrithsTransformation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Kenrith's Transformation", cost := some [generic 1, pip .green],
      types := [.enchantment], subtypes := [aura],
      text :=
        [ enchant isCreature,
          when (does thisAura enter) (draw (.lit 1)),
          static (enchantedBecomesVanilla [.green] "Elk" 3 3) ] } }

def amphibianDownpour : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Amphibian Downpour", cost := some [generic 2, pip .blue], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ flash, storm,
          enchant isCreature,
          static (enchantedBecomesVanilla [.blue] "Frog" 1 1) ] } }

def lignify : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lignify", cost := some [generic 1, pip .green], types := [.kindred, .enchantment],
      subtypes := [treefolk, aura],
      text :=
        [ enchant isCreature,
          static (conjunction [ qualityChange (enchanted creature) .sets
                (bundle { characteristics := { subtypes := [treefolk] } }),
              getsBase it (.lit 0) (.lit 4),
              allAbilityLoss it ]) ] } }

def nefariousImp : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Nefarious Imp", cost := some [generic 2, pip .black], types := [.creature],
      subtypes := [imp],
      text :=
        [ flying,
          whenever
            (does (counted (atLeast 1) (and [isPermanent, youControl])) leave)
            (scry (.lit 1)) ],
      power := stat 2, toughness := stat 1 } }

/-- Ainok Tracker -/
def ainokTracker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ainok Tracker", cost := some [generic 5, pip .red], types := [.creature],
      subtypes := [dog, scout],
      text := [firstStrike, morph (mana [generic 4, pip .red])],
      power := stat 3, toughness := stat 3 } }

def irenicussVileDuplication : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Irenicus's Vile Duplication", cost := some [generic 3, pip .blue], types := [.sorcery],
      text :=
        [ spell (create (.lit 1)
            (copyOf (target (and [isCreature, youControl]))
              [alsoHas flying, isntLegendary])
            []) ] } }

def cacklingCounterpart : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cackling Counterpart", cost := some [generic 1, pip .blue, pip .blue],
      types := [.instant],
      text :=
        [ spell (create (.lit 1)
            (copyOf (target (and [isCreature, youControl]))) []),
          flashback (mana [generic 5, pip .blue, pip .blue]) ] } }

def chandrasSpitfire : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chandra's Spitfire", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [elemental],
      text :=
        [ flying,
          whenever (does any (dealDamage (to := anOpponent) (kind := .noncombatOnly)))
            (get thisCreature (up (.lit 3)) (up (.lit 0)) (some untilEndOfTurn)) ],
      power := stat 1, toughness := stat 3 } }

def livingHive : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Living Hive", cost := some [generic 6, pip .green, pip .green], types := [.creature],
      subtypes := [elemental, insect],
      text :=
        [ trample,
          whenever (does thisCreature (dealDamage (to := a isPlayer) (kind := .combatOnly)))
            (createToken (creatureToken 1 1 [.green] [insect]) (count := thatMuch)) ],
      power := stat 6, toughness := stat 6 } }

def giantCindermaw : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Giant Cindermaw", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [dinosaur, beast],
      text := [trample, static (cant (bare isPlayer) (gainLife anyAmount))],
      power := stat 4, toughness := stat 3 } }

def lushGrowth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lush Growth", cost := some [pip .green], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ enchant isLand,
          static (qualityChange (enchanted land) .sets
            (bundle
              { characteristics :=
                { subtypes := [mountain, forest, plains] } })) ] } }

def voiceOfAll : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Voice of All", cost := some [generic 2, pip .white, pip .white], types := [.creature],
      subtypes := [angel],
      text :=
        [ flying,
          static (entersChoosing thisCreature .color),
          static (abilityGrant thisCreature (protection (ofChosen .color))) ],
      power := stat 2, toughness := stat 2 } }

def wardSliver : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ward Sliver", cost := some [generic 4, pip .white], types := [.creature],
      subtypes := [sliver],
      text :=
        [ static (entersChoosing thisCreature .color),
          static (abilityGrant (allOf (hasSubtype sliver))
            (protection (ofChosen .color))) ],
      power := stat 2, toughness := stat 2 } }

def sanctuaryBlade : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sanctuary Blade", cost := some [generic 2], types := [.artifact],
      subtypes := [equipment],
      text :=
        [ static (attachChoosing thisEquipment .color),
          static (conjunction [ getsPt (equipped creature) (up (.lit 2)) (up (.lit 0)),
              abilityGrant (equipped creature)
                (protection (ofTheLastChosen .color)) ]),
          equip (mana [generic 3]) ] } }

/-- Sinister Strength -/
def sinisterStrength : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sinister Strength", cost := some [generic 1, pip .black], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ enchant isCreature,
          static (conjunction [ getsPt (enchanted creature) (up (.lit 3)) (up (.lit 1)),
              qualityChange it .sets (colored (.some [.black])) ]) ] } }

/-- Crimson Wisps -/
def crimsonWisps : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Crimson Wisps", cost := some [pip .red], types := [.instant],
      text :=
        [ spell (sequentially
            [ establishThroughout
                (conjunction [ qualityChange (target isCreature) .sets (colored (.some [.red])),
                    abilityGrant it haste ]) untilEndOfTurn,
              draw (.lit 1) ]) ] } }

/-- Ghoulflesh -/
def ghoulflesh : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ghoulflesh", cost := some [pip .black], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ enchant isCreature,
          static (conjunction
                    [ getsPt (enchanted creature) (down (.lit 1)) (down (.lit 1)),
              qualityChange it .adds
                (bundle
                   { characteristics := { colors := [.black], subtypes := [zombie] } }) ]) ] } }

/-- Blade of the Oni -/
def bladeOfTheOniStatic : StaticSpec :=
  conjunction [ getsBase (equipped creature) (.lit 5) (.lit 5),
      abilityGrant it menace,
      qualityChange it .adds
        (bundle { characteristics := { colors := [.black], subtypes := [demon] } }) ]
theorem okBladeOfTheOniStatic : StaticSpec.check (DRS.ofList []) bladeOfTheOniStatic = [] := by decide

def historyOfBenalia : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "History of Benalia", cost := some [generic 1, pip .white, pip .white],
      types := [.enchantment], subtypes := [saga],
      text :=
        [ when (chapterMark [1, 2])
            (createToken
              { characteristics :=
                { colors := [.white], types := [.creature], subtypes := [knight],
                  text := [vigilance], power := stat 2, toughness := stat 2 } }),
          when (chapterMark [3])
            (get (allOf (and [hasSubtype knight, youControl]))
              (up (.lit 2)) (up (.lit 1)) (some untilEndOfTurn)) ] } }

/-- Akroan Sergeant -/
def akroanSergeant : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Akroan Sergeant", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [human, soldier],
      text := [firstStrike, renown (.lit 1)], power := stat 2, toughness := stat 2 } }

def castCreatureSpellsFromTop : StaticSpec :=
  can you (cast (allOf (and [isSpell, isCreature])) (from_ := some onTop))
theorem okCastCreatureSpellsFromTop : StaticSpec.check (DRS.ofList []) castCreatureSpellsFromTop = [] := by decide

def garruksHorde : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Garruk's Horde", cost := some [generic 5, pip .green, pip .green], types := [.creature],
      subtypes := [beast],
      text :=
        [ trample,
          static (visibility .reveal you topOfLibrary),
          static castCreatureSpellsFromTop ],
      power := stat 7, toughness := stat 7 } }

def wanderingEye : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Wandering Eye", cost := some [generic 2, pip .blue], types := [.creature],
      subtypes := [illusion],
      text := [flying, static (visibility .reveal (bare isPlayer) wholeHand)],
      power := stat 1, toughness := stat 3 } }

def amorphousAxe : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Amorphous Axe", cost := some [generic 2], types := [.artifact],
      subtypes := [equipment],
      text :=
        [ static (conjunction [ getsPt (equipped creature) (up (.lit 3)) (up (.lit 0)),
              qualityChange (equipped creature) .adds (everyTypeOf
                  .creature) ]),
          equip (mana [generic 3]) ] } }

def runedStalactite : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Runed Stalactite", cost := some [generic 1], types := [.artifact],
      subtypes := [equipment],
      text :=
        [ static (conjunction [ getsPt (equipped creature) (up (.lit 1)) (up (.lit 1)),
              qualityChange (equipped creature) .adds (everyTypeOf
                  .creature) ]),
          equip (mana [generic 2]) ] } }

def arachnoform : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Arachnoform", cost := some [generic 1, pip .green], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ enchant isCreature,
          static (conjunction [ getsPt (enchanted creature) (up (.lit 2)) (up (.lit 2)),
              abilityGrant (enchanted creature) reach,
              qualityChange (enchanted creature) .adds (everyTypeOf
                  .creature) ]) ] } }

def nyleasPresence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Nylea's Presence", cost := some [generic 1, pip .green], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ enchant isLand,
          when (does thisAura enter) (draw (.lit 1)),
          static (qualityChange (enchanted land) .adds (everyTypeOf
              .basicLand)) ] } }

/-- Curse of Conformity -/
def curseOfConformity : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Curse of Conformity", cost := some [generic 4, pip .white], types := [.enchantment],
      subtypes := [aura, curse],
      text :=
        [ enchant isPlayer,
          static (conjunction [ getsBase (allOf (and [ isCreature, non (hasSupertype .legendary),
                               controlledBy (enchanted player) ])) (.lit 3) (.lit 3),
              qualityChange them .loses (everyTypeOf .creature) ]) ] } }

/-- Drumbellower -/
def drumbellower : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Drumbellower", cost := some [generic 2, pip .white], types := [.creature],
      subtypes := [spirit],
      text :=
        [ flying,
          static (untapsDuring (allOf (and [isCreature, youControl]))
            (some (each isOtherPlayer))) ],
      power := stat 2, toughness := stat 1 } }

def tayamLine : StaticSpec :=
  entersWithAdditionalCounters
    (each (and [isCreature, youControl, otherThan thisCreature])) (.lit 1)
    vigilanceCounter
theorem okTayamLine : StaticSpec.check (DRS.ofList []) tayamLine = [] := by decide

def clarionConqueror : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Clarion Conqueror", cost := some [generic 2, pip .white], types := [.creature],
      subtypes := [dragon],
      text :=
        [ flying,
          static (cant any (activate (allOf (and [ abilityHead .anyActivated,
                           abilityOf (allOf (or [isArtifact, isCreature, isPlaneswalker])) ])))) ],
      power := stat 3, toughness := stat 3 } }

/-- Kang the Conqueror -/
def kangPowerUpLock : StaticSpec :=
  cant any (activate (allOf (abilityHead (.keyword powerUp))))
theorem okKangPowerUpLock : StaticSpec.check (DRS.ofList []) kangPowerUpLock = [] := by decide

def vipersKiss : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Viper's Kiss", cost := some [pip .black], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ enchant isCreature,
          static (conjunction
                    [ getsPt (enchanted creature) (down (.lit 1)) (down (.lit 1)),
              cant any (activate (allOf (and [abilityHead .anyActivated, abilityOf it]))) ]) ] } }

def stupefyingTouch : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stupefying Touch", cost := some [generic 1, pip .blue], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ enchant isCreature,
          when (does thisAura enter) (draw (.lit 1)),
          static (cant any (activate (allOf (and [ abilityHead .anyActivated,
                           abilityOf (enchanted creature) ])))) ] } }

def linvalaKeeperOfSilence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Linvala, Keeper of Silence", cost := some [generic 2, pip .white, pip .white],
      supertypes := [.legendary], types := [.creature], subtypes := [angel],
      text :=
        [ flying,
          static (cant any (activate (allOf (and [ abilityHead .anyActivated,
                           abilityOf (allOf (and [isCreature, opponentsControl])) ])))) ],
      power := stat 3, toughness := stat 4 } }

def kyrenLegate : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Kyren Legate", cost := some [generic 1, pip .red], types := [.creature],
      subtypes := [goblin],
      text :=
        [ static (onlyWhile (altCost this_)
            (and
              [ exists_ (and [ isLand, hasSubtype plains,
                                anOpponentControls ]),
                exists_ (and [ isLand, hasSubtype mountain,
                                youControl ]) ])),
          haste ],
      power := stat 1, toughness := stat 1 } }

def polarKraken : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Polar Kraken", cost := some [generic 8, pip .blue, pip .blue, pip .blue],
      types := [.creature], subtypes := [kraken],
      text :=
        [ trample,
          static (entersTapped thisCreature),
          cumulativeUpkeep (perform (sacrifice (a isLand))) ],
      power := stat 11, toughness := stat 11 } }

def yavimayaAnts : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Yavimaya Ants", cost := some [generic 2, pip .green, pip .green], types := [.creature],
      subtypes := [insect],
      text := [trample, haste, cumulativeUpkeep (mana [pip .green, pip .green])],
      power := stat 5, toughness := stat 1 } }

def illusionaryForces : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Illusionary Forces", cost := some [generic 3, pip .blue], types := [.creature],
      subtypes := [illusion],
      text := [flying, cumulativeUpkeep (mana [pip .blue])],
      power := stat 4, toughness := stat 4 } }

def vexingSphinx : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vexing Sphinx", cost := some [generic 1, pip .blue, pip .blue], types := [.creature],
      subtypes := [sphinx],
      text :=
        [ flying,
          cumulativeUpkeep (perform (discard (a (cardIn yourHand)))),
          when (does thisCreature die) (draw (countersOn ageCounter it)) ],
      power := stat 4, toughness := stat 4 } }

def manaChains : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mana Chains", cost := some [pip .blue], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ enchant isCreature,
          static (abilityGrant (enchanted creature) (cumulativeUpkeep (mana
              [generic 1]))) ] } }

def dreamThief : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dream Thief", cost := some [generic 2, pip .blue], types := [.creature],
      subtypes := [faerie, rogue],
      text :=
        [ flying,
          when (does thisCreature enter)
            (doOnlyIf (draw (.lit 1))
              (happened (cast (a
                (and [isSpell, hasColor .blue,
                otherThan this_]))) you soFarThisTurn)) ],
      power := stat 2, toughness := stat 1 } }

def brightspearZealot : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Brightspear Zealot", cost := some [generic 2, pip .white], types := [.creature],
      subtypes := [human, soldier],
      text :=
        [ vigilance,
          static (onlyWhile (getsPt thisCreature (up (.lit 2)) (up (.lit 0)))
            (compareAmt (eventCount (cast (a isSpell)) you soFarThisTurn) .atLeast (.lit
              2))) ],
      power := stat 2, toughness := stat 4 } }

def deepwayNavigator : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Deepway Navigator", cost := some [pip .white, pip .blue], types := [.creature],
      subtypes := [merfolk, wizard],
      text :=
        [ flash,
          when (does thisCreature enter)
            (untap
              (each (and [ hasSubtype merfolk, youControl,
                            otherThan thisCreature ]))),
          static (onlyWhile
            (getsPt (allOf (and [hasSubtype merfolk, youControl]))
              (up (.lit 1)) (up (.lit 0)))
            (happened (attackWith (counted (atLeast 3)
              (hasSubtype merfolk))) you soFarThisTurn)) ],
      power := stat 2, toughness := stat 2 } }

def bloodfireEnforcers : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bloodfire Enforcers", cost := some [generic 3, pip .red], types := [.creature],
      subtypes := [human, monk],
      text :=
        [ static (onlyWhile
            (conjunction [abilityGrant thisCreature firstStrike, abilityGrant it
                trample])
            (and
              [ exists_ (and [isInstant, in_ yourGraveyard]),
                exists_ (and [isSorcery, in_ yourGraveyard]) ])) ],
      power := stat 5, toughness := stat 2 } }

def stormOfSouls : Action :=
  sequentially
    [ putOntoBattlefield (allOf (and [isCreature, in_ yourGraveyard])),
      become them
        { characteristics :=
          { subtypes := [spirit], text := [flying],
            power := stat 1, toughness := stat 1 } },
      exile this_ ]
theorem okStormOfSouls : Action.check (DRS.ofList []) stormOfSouls = [] := by decide

def answeredPrayers : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Answered Prayers", cost := some [generic 1, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ when (does (a (and [isCreature, youControl])) enter)
            (sequentially
              [ gainLife (.lit 1),
                doIf (not (matches_
                    thisEnchantment isCreature))
                  (become thisEnchantment
                    { characteristics :=
                      { types := [.creature], subtypes := [angel],
                        text := [flying], power := stat 3, toughness := stat 3 } }
                    (some untilEndOfTurn)) ]) ] } }

/-- Hate Mirage's middle two sentences -/
def hateMirageTokens : Action :=
  sequentially
    [ doForEach (targets (upTo 2) (and [isCreature, youDontControl]))
        (create (.lit 1) (copyOf it) []),
      gain thoseTokens haste ]
theorem okHateMirageTokens : Action.check (DRS.ofList []) hateMirageTokens = [] := by decide
def descentOfTheDragons : Action :=
  sequentially
    [ destroy (targets anyNumber isCreature),
      doForEach (theVerbed (.action "destroy") .many (some creature) .thisWay)
        (instruct (controllerOf it) (createToken { characteristics :=
              { colors := [.red], types := [.creature], subtypes := [dragon],
                text := [flying], power := stat 4, toughness := stat 4 } })) ]
theorem okDescentOfTheDragons : Action.check (DRS.ofList []) descentOfTheDragons = [] := by decide
/-- Brimaz, King of Oreskos -/
def brimazAttackToken : Ability :=
  whenever (does thisCreature attack)
    (create (.lit 1)
      (written
        { characteristics :=
          { colors := [.white], types := [.creature], subtypes := [cat, soldier],
            text := [vigilance], power := stat 1, toughness := stat 1 } })
      [entersAttacking none])
theorem okBrimazAttackToken : Ability.check (DRS.ofList []) brimazAttackToken = [] := by decide
/-- Sedris, the Traitor King -/
def sedrisTheTraitorKing : Ability :=
  static (abilityGrant (each (and [isCreature, in_ yourGraveyard]))
    (unearth (mana [generic 2, pip .black])))
theorem okSedrisTheTraitorKing : Ability.check (DRS.ofList []) sedrisTheTraitorKing = [] := by decide
/-- Grixis -/
def grixis : Ability :=
  static (abilityGrant
    (allOf (and [ or [hasColor .blue, hasColor .black, hasColor .red], isCreature,
                   in_ yourGraveyard ]))
    (unearth itsManaCost))
theorem okGrixis : Ability.check (DRS.ofList []) grixis = [] := by decide
/-- Dralnu, Lich Lord -/
def dralnuLichLord : Action :=
  gain (target (and [isInstantOrSorcery, in_ yourGraveyard]))
    (flashback itsManaCost) (some untilEndOfTurn)
theorem okDralnuLichLord : Action.check (DRS.ofList []) dralnuLichLord = [] := by decide
/-- Dregscape Zombie -/
def dregscapeZombie : Ability := unearth (mana [pip .black])
theorem okDregscapeZombie : Ability.check (DRS.ofList []) dregscapeZombie = [] := by decide
/-- Think Twice -/
def thinkTwice : Ability := flashback (mana [generic 2, pip .blue])
theorem okThinkTwice : Ability.check (DRS.ofList []) thinkTwice = [] := by decide
/-- Greater Mossdog -/
def greaterMossdog : Ability := dredge (.lit 3)
theorem okGreaterMossdog : Ability.check (DRS.ofList []) greaterMossdog = [] := by decide
/-- Raven's Crime -/
def ravensCrime : Ability := retrace
theorem okRavensCrime : Ability.check (DRS.ofList []) ravensCrime = [] := by decide
/-- Barkhide Mauler -/
def barkhideMauler : Ability := cycling (mana [generic 2])
theorem okBarkhideMauler : Ability.check (DRS.ofList []) barkhideMauler = [] := by decide
/-- Ninja of the New Moon -/
def ninjaOfTheNewMoon : Ability := ninjutsu (mana [generic 3, pip .black])
theorem okNinjaOfTheNewMoon : Ability.check (DRS.ofList []) ninjaOfTheNewMoon = [] := by decide
/-- Thunderous Wrath -/
def thunderousWrath : Ability := miracle (mana [pip .red])
theorem okThunderousWrath : Ability.check (DRS.ofList []) thunderousWrath = [] := by decide
/-- Bygone Colossus -/
def bygoneColossus : Ability := warp (mana [generic 3])
theorem okBygoneColossus : Ability.check (DRS.ofList []) bygoneColossus = [] := by decide
/-- Knight of Grace — "Hexproof from black" [CR#702.11d] -/
def knightOfGraceHexproofFromBlack : Ability := hexproofFrom (hasColor .black)
theorem okKnightOfGraceHexproofFromBlack :
    Ability.check (DRS.ofList []) knightOfGraceHexproofFromBlack = [] := by decide
/-- Eternal Dragon — "Plainscycling {2}" [CR#702.29e] -/
def eternalDragonPlainscycling : Ability :=
  typecycling (hasSubtype plains) (mana [generic 2])
theorem okEternalDragonPlainscycling : Ability.check (DRS.ofList []) eternalDragonPlainscycling = [] := by decide
/-- Nezumi Ronin -/
def nezumiRonin : Ability := bushido (.lit 1)
theorem okNezumiRonin : Ability.check (DRS.ofList []) nezumiRonin = [] := by decide
/-- Dragonlord Ojutai -/
def dragonlordOjutaiHexproof : Ability :=
  static (onlyWhile (abilityGrant thisCreature hexproof) (matches_ it isUntapped))
theorem okDragonlordOjutaiHexproof : Ability.check (DRS.ofList []) dragonlordOjutaiHexproof = [] := by decide
/-- Monoxa, Midway Manager -/
def monoxaRollTrigger : Ability :=
  whenever (does you (rollDice (watch := resultIn (atLeast 3))))
    (sequentially
      [ gain thisCreature firstStrike (some untilEndOfTurn),
        doIf (compareAmt (theOutcome .rollResult) .atLeast (.lit 4))
          (gain it menace (some untilEndOfTurn)),
        doIf (compareAmt (theOutcome .rollResult) .atLeast (.lit 5))
          (gain it lifelink (some untilEndOfTurn)) ])
theorem okMonoxaRollTrigger : Ability.check (DRS.ofList []) monoxaRollTrigger = [] := by decide
/-- Celebr-8000 -/
def celebr8000Doubles : Action :=
  sequentially
    [ rollDice (.lit 2) (.sides 6),
      doIf rolledDoubles (gain thisCreature doubleStrike (some untilEndOfTurn)) ]
theorem okCelebr8000Doubles : Action.check (DRS.ofList []) celebr8000Doubles = [] := by decide

/-- Krosan Druid -/
def krosanDruid : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Krosan Druid", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [centaur, druid],
      text :=
        [ kicker (mana [generic 4, pip .green]),
          triggeredIf (does thisCreature enter) (keywordCostPaid kicker
                                                           thisCreature)
            (gainLife (.lit 10)) ],
      power := stat 2, toughness := stat 3 } }

/-- Lightkeeper of Emeria -/
def lightkeeperOfEmeria : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lightkeeper of Emeria", cost := some [generic 3, pip .white], types := [.creature],
      subtypes := [angel],
      text :=
        [ multikicker (mana [pip .white]),
          flying,
          when (does thisCreature enter)
            (gainLife (times (.lit 2) (timesPaid (.byKeyword kicker) thisCreature))) ],
      power := stat 2, toughness := stat 4 } }

/-- Merfolk Falconer -/
def merfolkFalconer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Merfolk Falconer", cost := some [generic 3, pip .blue, pip .blue], types := [.creature],
      subtypes := [merfolk, wizard],
      text :=
        [ flying,
          whenever
            (does you (cast (a (compareOver isSpell (paidCostRead (.byKeyword kicker)
                none it) .atLeast (.lit 1)))))
            (scry (.lit 2)) ],
      power := stat 4, toughness := stat 4 } }

/-- Borrowed Malevolence -/
def borrowedMalevolence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Borrowed Malevolence", cost := some [pip .black], types := [.instant],
      text :=
        [ escalate (mana [generic 2]),
          spell (chooseModes (oneThrough 2)
            [ get (target isCreature) (up (.lit 1)) (up (.lit 1)) (some untilEndOfTurn),
              get (target isCreature) (down (.lit 1)) (down (.lit 1)) (some untilEndOfTurn) ]) ] } }

/-- New Perspectives -/
def newPerspectivesCyclingAltCost : Ability :=
  static (altCost (allOf (abilityHead (.keyword cycling))) (some (mana [generic 0])))
theorem okNewPerspectivesCyclingAltCost :
    Ability.check (DRS.ofList []) newPerspectivesCyclingAltCost = [] := by decide
/-- Thick-Skinned Goblin -/
def thickSkinnedGoblinEchoAltCost : Ability :=
  static (altCost
    (allOf (and [ abilityHead (.keyword echo),
                   abilityOf (allOf (and [isPermanent, youControl])) ]))
    (some (mana [generic 0])))
theorem okThickSkinnedGoblinEchoAltCost :
    Ability.check (DRS.ofList []) thickSkinnedGoblinEchoAltCost = [] := by decide
/-- Fumiko the Lowblood -/
def fumikoBushidoX : Ability :=
  static (conjunction [ abilityGrant thisCreature (bushido (letter .x)),
      letterDefinition .x (countOf isAttacking) ])
theorem okFumikoBushidoX : Ability.check (DRS.ofList []) fumikoBushidoX = [] := by decide

/-- Rafter Demon -/
def rafterDemon : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rafter Demon", cost := some [generic 2, pip .black, pip .red], types := [.creature],
      subtypes := [demon],
      text :=
        [ spectacle (mana [generic 3, pip .black, pip .red]),
          triggeredIf (does thisCreature enter)
            (keywordCostPaid spectacle thisCreature)
            (sequentially [instruct eachOpponent (choose (a (cardIn hand))), discard them]) ],
      power := stat 4, toughness := stat 2 } }

/-- Tourach, Dread Cantor -/
def tourachDiscardTrigger : Ability :=
  whenever (does anOpponent (discard (a
    isCard)))
    (putCounters (.lit 1) p1p1Counter thisCreature .many)
theorem okTourachDiscardTrigger : Ability.check (DRS.ofList []) tourachDiscardTrigger = [] := by decide

/-- Shimmering Glasskite -/
def shimmeringGlasskite : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shimmering Glasskite", cost := some [generic 3, pip .blue], types := [.creature],
      subtypes := [spirit],
      text :=
        [ flying,
          triggeredOnlyOnce
            (does (a (or [isSpell, abilityHead .anyOnStack])) (target thisCreature)) oncePerTurn
            (counter (that .stack)) ],
      power := stat 2, toughness := stat 3 } }

/-- Conqueror's Pledge -/
def conquerorsPledge : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Conqueror's Pledge", cost := some [generic 2, pip .white, pip .white, pip .white],
      types := [.sorcery],
      text :=
        [ kicker (mana [generic 6]),
          spell (replace
            (createToken (creatureToken 1 1 [.white] [kor, soldier]) (count := .lit 6))
            (doIf (keywordCostPaid kicker this_)
              (instead (create (.lit 12) asThose [])))) ] } }

/-- Soul of Emancipation -/
def soulOfEmancipation : Ability :=
  when (does thisCreature enter)
    (sequentially
      [ destroy (targets (upTo 3) (and [isPermanent, non isLand, otherThan this_])),
        doForEach (those permanent)
          (instruct (controllerOf (that permanent)) (createToken { characteristics :=
                { colors := [.white], types := [.creature], subtypes := [angel],
                  text := [flying], power := stat 3, toughness := stat 3 } })) ])
theorem okSoulOfEmancipation : Ability.check (DRS.ofList []) soulOfEmancipation = [] := by decide

/-- Tourach, Dread Cantor -/
def tourachDreadCantor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tourach, Dread Cantor", cost := some [generic 1, pip .black],
      supertypes := [.legendary], types := [.creature],
      subtypes := [human, cleric],
      text :=
        [ kicker (mana [pip .black, pip .black]),
          protection (hasColor .white),
          tourachDiscardTrigger,
          triggeredIf (does thisCreature enter) (keywordCostPaid kicker
                                                           thisCreature)
            (instruct (target isOpponent) (discard (countedAtRandom (exactly 2) isCard))) ],
      power := stat 2, toughness := stat 1 } }

/-- Talrand's Invocation -/
def talrandsInvocation : Action :=
  createToken
    { characteristics :=
      { colors := [.blue], types := [.creature], subtypes := [drake],
        text := [flying], power := stat 2, toughness := stat 2 } } (count := .lit 2)
theorem okTalrandsInvocation : Action.check (DRS.ofList []) talrandsInvocation = [] := by decide

/-- Predatory Wurm -/
def predatoryWurm : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Predatory Wurm", cost := some [generic 3, pip .green], types := [.creature],
      subtypes := [wurm],
      text :=
        [ vigilance,
          static (onlyWhile (getsPt thisCreature (up (.lit 2)) (up (.lit 2)))
            (exists_ (and [hasSubtype garruk, youControl]))) ],
      power := stat 4, toughness := stat 4 } }

/-- Tower Winder -/
def towerWinder : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tower Winder", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [snake],
      text :=
        [ reach, deathtouch,
          when (does thisCreature enter)
            (sequentially
              [ searchLibraryOrGraveyard (named (printedName "Command Tower")),
                reveal it, putInto it hand,
                doIf (happenedThisWay (searchLibraryFor anyNumber (and []))
                        you) shuffle ]) ],
      power := stat 1, toughness := stat 1 } }

/-- Aim High -/
def aimHigh : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aim High", cost := some [generic 1, pip .green], types := [.instant],
      text :=
        [ spell (sequentially
            [ untap (target isCreature),
              establishThroughout (conjunction
                                     [ getsPt (theVerbed (.action "untap") .one) (up (.lit 2))
                                         (up (.lit 2)),
                    abilityGrant (theVerbed (.action "untap") .one) reach ]) untilEndOfTurn ]) ] } }

/-- Harried Dronesmith -/
def harriedDronesmithToken : Action :=
  sequentially [createToken thopterToken, gainHaste itAsToken (some untilEndOfTurn)]
theorem okHarriedDronesmithToken : Action.check (DRS.ofList []) harriedDronesmithToken = [] := by decide

/-- Archfiend's Vessel -/
def archfiendsVessel : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Archfiend's Vessel", cost := some [pip .black], types := [.creature],
      subtypes := [human, cleric],
      text :=
        [ lifelink,
          triggeredIf (does thisCreature enter)
            (or
              [ did ⟨it, enter (from_ := some yourGraveyard), triggering⟩,
                matches_ it (and [castBy you, castFrom yourGraveyard]) ])
            (triggerReflexively (exile it)
              (createToken
                { characteristics :=
                  { colors := [.black], types := [.creature], subtypes := [demon],
                    text := [flying], power := stat 5, toughness := stat 5 } })) ],
      power := stat 1, toughness := stat 1 } }

/-- Containment Priest -/
def containmentPriest : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Containment Priest", cost := some [generic 1, pip .white], types := [.creature],
      subtypes := [human, cleric],
      text :=
        [ flash,
          static (replace
                    (would any
                       (putOntoBattlefield
                          (a (and [isCreature, isNontoken, not (castBy any)])))) (instead (exile it))) ],
      power := stat 2, toughness := stat 2 } }

/-- Veiling Oddity -/
def veilingOddityLine : Ability :=
  triggeredWhile (lastCounterRemoved timeCounter this_)
    (whileTrue (matches_ this_ (cardIn exileZone)))
    (establishThroughout (cant any (block (allOf isCreature))) thisTurn)
theorem okVeilingOddityLine : Ability.check (DRS.ofList []) veilingOddityLine = [] := by decide

def wizenedSnitches : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Wizened Snitches", cost := some [generic 3, pip .blue], types := [.creature],
      subtypes := [faerie, rogue],
      text := [flying, static (visibility .reveal (bare isPlayer) topOfLibrary)],
      power := stat 1, toughness := stat 3 } }

/-- Darkblade Agent -/
def darkbladeAgentGrants : Ability :=
  static (onlyWhile (conjunction [ abilityGrant thisCreature deathtouch,
        abilityGrant thisCreature
          (whenever
             (does thisCreature (dealDamage (to := a isPlayer) (kind := .combatOnly))) (draw
                                                                                                  (.lit 1))) ]) (happened (surveil anyAmount) you soFarThisTurn))
theorem okDarkbladeAgentGrants : Ability.check (DRS.ofList []) darkbladeAgentGrants = [] := by decide
/-- Frenzied Gorespawn -/
def frenziedGorespawnMenaceTrigger : Ability :=
  triggered (does (counted (atLeast 1) isCreature) (attack (some anOpponent)))
      (gain (those creature) menace (some untilEndOfTurn))
theorem okFrenziedGorespawnMenaceTrigger :
    Ability.check (DRS.ofList []) frenziedGorespawnMenaceTrigger = [] := by decide
/-- Pir, Imaginative Rascal's replacement -/
def pirDistributive : Ability :=
  static (replace
            (would any
               (putCounters anyAmount anyKind
                  (a (and [isPermanent, controlledBy (youOr (a isTeammate))])) .many false))
                      (instead (putCounters (plus thatMuch (.lit 1)) thoseCounters (that permanent) .many)))
theorem okPirDistributive : Ability.check (DRS.ofList []) pirDistributive = [] := by decide

/-- Frogmite -/
def frogmite : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Frogmite", cost := some [generic 4], types := [.artifact, .creature],
      subtypes := [frog], text := [affinity isArtifact],
      power := stat 2, toughness := stat 2 } }

/-- Iymrith, Desert Doom -/
def iymrithDesertDoom : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Iymrith, Desert Doom", cost := some [generic 3, pip .blue, pip .blue],
      supertypes := [.legendary], types := [.creature], subtypes := [dragon],
      text :=
        [ flying,
          static (onlyWhile (abilityGrant thisCreature (ward (mana [generic
              4])))
            (matches_ it isUntapped)),
          whenever (does thisCreature (dealDamage (to := a isPlayer) (kind := .combatOnly)))
            (sequentially
              [ draw (.lit 1),
                doIf (compareAmt (countOf (cardIn yourHand)) .less (.lit 3))
                  (draw theDifference) ]) ],
      power := stat 5, toughness := stat 5 } }

def protectionAbilities : KeywordTerm := .anyIn ⟨protection, none⟩
def landwalkAbilities : KeywordTerm := .anyIn ⟨landwalk, none⟩
/-- Shay Cormac -/
def wardAbilities : KeywordTerm := .anyIn ⟨ward, none⟩
/-- Tolaria, Shelkin Brownie -/
def bandsWithOtherAbilities : KeywordTerm := .anyIn ⟨bandsWithOther, none⟩
def protectionFromAnyColor : KeywordTerm := .anyIn ⟨protection, some .color⟩

/-- Cairn Wanderer -/
def cairnWanderer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cairn Wanderer", cost := some [generic 4, pip .black], types := [.creature],
      subtypes := [shapeshifter],
      text :=
        [ changeling,
          alsoForKeywords
            (static (onlyWhile (abilityGrant thisCreature flying)
              (exists_ (and [isCreature, in_ graveyard, hasKeyword flying]))))
            [ fear, firstStrike, doubleStrike, deathtouch, haste,
              landwalkAbilities, lifelink, protectionAbilities, reach, trample,
              shroud, vigilance ] ],
      power := stat 4, toughness := stat 4 } }

/-- Autarch Mammoth -/
def autarchMammoth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Autarch Mammoth", cost := some [generic 4, pip .green, pip .green], types := [.creature],
      subtypes := [elephant, mount],
      text := [autarchMammothLine, saddle (.lit 5)],
      power := stat 5, toughness := stat 5 } }

/-- Debris Beetle -/
def debrisBeetle : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Debris Beetle", cost := some [generic 2, pip .black, pip .green], types := [.artifact],
      subtypes := [vehicle],
      text :=
        [ trample,
          when (does thisVehicle enter)
            (sequentially [instruct eachOpponent (loseLife (.lit 3)), gainLife (.lit 3)]),
          crew (.lit 2) ],
      power := stat 6, toughness := stat 6 } }

/-- Pir, Imaginative Rascal -/
def pirImaginativeRascal : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Pir, Imaginative Rascal", cost := some [generic 2, pip .green],
      supertypes := [.legendary], types := [.creature], subtypes := [human],
      text :=
        [ partnerWith (named (printedName "Toothy, Imaginary Friend")),
          pirDistributive ],
      power := stat 1, toughness := stat 1 } }

/-- Market Gnome -/
def marketGnome : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Market Gnome", cost := some [pip .white], types := [.artifact, .creature],
      subtypes := [gnome],
      text :=
        [ when (does thisCreature die) (sequentially [gainLife (.lit 1), draw (.lit 1)]),
          triggeredWhile (does any (exile thisCreature))
            (whileDoing (does you (activate (a (abilityHead (.keyword craft))))))
            (sequentially [gainLife (.lit 1), draw (.lit 1)]) ],
      power := stat 0, toughness := stat 3 } }

/-- Escaped Shapeshifter -/
def escapedShapeshifter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Escaped Shapeshifter", cost := some [generic 3, pip .blue, pip .blue],
      types := [.creature], subtypes := [shapeshifter],
      text :=
        [ alsoForKeywords
            (static (onlyWhile (abilityGrant thisCreature flying)
              (exists_
                 (and [ isCreature, anOpponentControls, hasKeyword flying,
                               not (named (printedName "Escaped Shapeshifter")) ]))))
            [firstStrike, trample, protectionFromAnyColor] ],
      power := stat 3, toughness := stat 4 } }

/-- Ancestral Blade -/
def ancestralBlade : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ancestral Blade", cost := some [generic 1, pip .white], types := [.artifact],
      subtypes := [equipment],
      text :=
        [ when (does thisEquipment enter)
            (sequentially
              [ createToken (creatureToken 1 1 [.white] [soldier]),
                attachTo thisEquipment itAsToken ]),
          static (getsPt (equipped creature) (up (.lit 1)) (up (.lit 1))),
          equip (mana [generic 1]) ] } }

/-- Steelclaw Lance -/
def steelclawLance : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Steelclaw Lance", cost := some [pip .black, pip .red], types := [.artifact],
      subtypes := [equipment],
      text :=
        [ static (getsPt (equipped creature) (up (.lit 2)) (up (.lit 2))),
          equipQuality (hasSubtype knight) (mana [generic 1]),
          equip (mana [generic 3]) ] } }

/-- Commander's Plate's equip lines -/
def commandersPlateEquip : List Ability :=
  [ equipQuality (hasDesignation commander) (mana [generic 3]),
    equip (mana [generic 5]) ]
theorem okCommandersPlateEquip : Ability.checkText (DRS.ofList []) commandersPlateEquip = [] := by decide
/-- Luxior, Giada's Gift's equip lines -/
def luxiorEquipLines : List Ability :=
  [ equipQuality isPlaneswalker (mana [generic 1]),
    equip (mana [generic 3]) ]
theorem okLuxiorEquipLines : Ability.checkText (DRS.ofList []) luxiorEquipLines = [] := by decide

/-- Veiling Oddity -/
def veilingOddity : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Veiling Oddity", cost := some [generic 3, pip .blue], types := [.creature],
      subtypes := [illusion],
      text := [suspend (.lit 4) (mana [generic 1, pip .blue]), veilingOddityLine],
      power := stat 2, toughness := stat 3 } }

/-- Enormous Energy Blade -/
def enormousEnergyBlade : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Enormous Energy Blade", cost := some [generic 2, pip .black], types := [.artifact],
      subtypes := [equipment],
      text :=
        [ static (getsPt (equipped creature) (up (.lit 4)) (up (.lit 0))),
          whenever (does any (attachment .attached thisEquipment (some (a isCreature))))
              (tap (that creature)),
          equip (mana [generic 2]) ] } }

/-- Grafted Wargear -/
def graftedWargear : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Grafted Wargear", cost := some [generic 3], types := [.artifact],
      subtypes := [equipment],
      text :=
        [ static (getsPt (equipped creature) (up (.lit 3)) (up (.lit 2))),
          whenever (does any (attachment .unattached thisEquipment (some (a isPermanent))))
            (sacrifice (that permanent)),
          equip (mana [generic 0]) ] } }

/-- Black Ward -/
def blackWard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Black Ward", cost := some [pip .white], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ enchant isCreature,
          static (retention
            (abilityGrant (enchanted creature) (protection
                (hasColor .black)))
            thisAura) ] } }

/-- Cho-Manno's Blessing -/
def choMannosBlessing : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cho-Manno's Blessing", cost := some [pip .white, pip .white], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ flash,
          enchant isCreature,
          static (entersChoosing thisAura .color),
          static (retention
            (abilityGrant (enchanted creature) (protection
                (ofChosen .color)))
            thisAura) ] } }

/-- Pentarch Ward -/
def pentarchWard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Pentarch Ward", cost := some [generic 2, pip .white], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ enchant isCreature,
          static (entersChoosing thisAura .color),
          when (does thisAura enter) (draw (.lit 1)),
          static (retention
            (abilityGrant (enchanted creature) (protection
                (ofChosen .color)))
            thisAura) ] } }

/-- Benevolent Blessing -/
def benevolentBlessing : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Benevolent Blessing", cost := some [generic 1, pip .white], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ flash,
          enchant isCreature,
          static (entersChoosing thisAura .color),
          static (retention
            (abilityGrant (enchanted creature) (protection
                (ofChosen .color)))
            (allOf (and [ or [hasSubtype aura, hasSubtype equipment],
                           youControl, attachedTo it ]))) ] } }

/-- Wall of Shards -/
def wallOfShards : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Wall of Shards", cost := some [generic 1, pip .white], supertypes := [.snow],
      types := [.creature], subtypes := [wall],
      text :=
        [ defender, flying,
          cumulativeUpkeep (perform (instruct anOpponent (gainLife (.lit 1)))) ],
      power := stat 1, toughness := stat 8 } }

/-- Earthen Goo -/
def earthenGoo : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Earthen Goo", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [ooze],
      text :=
        [ trample,
          cumulativeUpkeep (either (mana [pip .red]) (mana [pip .green])),
          static (getsPt thisCreature
            (up (times (.lit 1) (countersOn ageCounter it)))
            (up (times (.lit 1) (countersOn ageCounter it)))) ],
      power := stat 2, toughness := stat 2 } }

/-- Mutagen Connoisseur -/
def mutagenConnoisseur : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mutagen Connoisseur", cost := some [generic 1, pip .green, pip .blue],
      types := [.creature], subtypes := [vedalken, mutant],
      text :=
        [ flying, vigilance,
          static (getsPt thisCreature
            (up (forEach (.lit 1) (and [isTransformed, youControl]))) (up (.lit 0))) ],
      power := stat 0, toughness := stat 5 } }

/-- Bedrock Tortoise -/
def bedrockTortoiseWindow : StaticSpec :=
  partScope .turn (some you) (abilityGrant (allOf (and [isCreature, youControl])) hexproof)
theorem okBedrockTortoiseWindow : StaticSpec.check (DRS.ofList []) bedrockTortoiseWindow = [] := by decide

/-- Battlegate Mimic -/
def battlegateMimic : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Battlegate Mimic", cost := some [generic 1, hybridPip .red .white], types := [.creature],
      subtypes := [shapeshifter],
      text :=
        [ whenever (does you (cast (a
          (and [isSpell, hasColor .red,
          hasColor .white]))))
            (establishThroughout (conjunction [ getsBase thisCreature (.lit 4) (.lit 2),
                  abilityGrant thisCreature firstStrike ]) untilEndOfTurn) ],
      power := stat 2, toughness := stat 1 } }

/-- Sphinx of Uthuun -/
def sphinxOfUthuun : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sphinx of Uthuun", cost := some [generic 5, pip .blue, pip .blue], types := [.creature],
      subtypes := [sphinx],
      text :=
        [ flying,
          when (does thisCreature enter)
            (sequentially
              [ reveal (topSlice (.lit 5)),
                instruct anOpponent (separateIntoPiles them 2),
                putInto onePile hand,
                putInto (theOther .pile) graveyard ]) ],
      power := stat 5, toughness := stat 6 } }

/-- Marit Lage -/
def maritLage : CharacteristicBundle :=
  { characteristics :=
    { name := "Marit Lage", colors := [.black], supertypes := [.legendary], types := [.creature],
      subtypes := [avatar], text := [flying, indestructible],
      power := stat 20, toughness := stat 20 } }

/-- Tuktuk the Explorer -/
def tuktukTheExplorer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tuktuk the Explorer", cost := some [generic 2, pip .red], supertypes := [.legendary],
      types := [.creature], subtypes := [goblin],
      text :=
        [ haste,
          when (does thisCreature die)
            (createToken
              { characteristics :=
                { name := "Tuktuk the Returned", supertypes := [.legendary],
                  types := [.artifact, .creature],
                  subtypes := [goblin, golem],
                  power := stat 5, toughness := stat 5 } }) ],
      power := stat 1, toughness := stat 1 } }

/-- Artificer's Assistant -/
def artificersAssistant : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Artificer's Assistant", cost := some [pip .blue], types := [.creature],
      subtypes := [bird],
      text :=
        [ flying,
          whenever (does you (cast (a
            (and [isHistoric, isSpell])))) (scry (.lit 1))
              ],
      power := stat 1, toughness := stat 1 } }

/-- Aya of Alexandria -/
def ayaOfAlexandria : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aya of Alexandria", cost := some [generic 2, pip .red, pip .white],
      supertypes := [.legendary], types := [.creature],
      subtypes := [human, assassin],
      text :=
        [ menace, lifelink,
          whenever
            (does (a (and [isHistoric, isCreature, youControl]))
               (dealDamage (to := a isPlayer) (kind := .combatOnly)))
            (createToken
              { characteristics :=
                { colors := [.black], types := [.creature], subtypes := [assassin],
                  text := [menace], power := stat 1, toughness := stat 1 } }) ],
      power := stat 4, toughness := stat 3 } }

/-- Indestructibility -/
def indestructibility : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Indestructibility", cost := some [generic 3, pip .white], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ enchant isPermanent,
          static (abilityGrant (enchanted permanent) indestructible) ] }
              }

/-- Seraphic Greatsword -/
def seraphicGreatsword : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Seraphic Greatsword", cost := some [generic 1, pip .white], types := [.artifact],
      subtypes := [equipment],
      text :=
        [ static (getsPt (equipped creature) (up (.lit 2)) (up (.lit 2))),
          whenever
            (does (equipped creature)
               (attack
                  (some
                     (the (and [isPlayer, superlative .max (.playerStat .lifeTotal) isPlayer])))))
            (create (.lit 1)
              (written
                { characteristics :=
                  { colors := [.white], types := [.creature], subtypes := [angel],
                    text := [flying], power := stat 4, toughness := stat 4 } })
              [entersAs .tapped, entersAttacking (some (that player))]),
          equip (mana [generic 4]) ] } }

def giantGrowth : Action :=
  get (target isCreature) (up (.lit 3)) (up (.lit 3)) (some untilEndOfTurn)
theorem okGiantGrowth : Action.check (DRS.ofList []) giantGrowth = [] := by decide
def amassZombiesTwo : Action := amass zombie (.lit 2)
theorem okAmassZombiesTwo : Action.check (DRS.ofList []) amassZombiesTwo = [] := by decide
/-- Avarice Totem -/
def avariceTotemExchange : Action :=
  exchange (controlOf thisArtifact (target (and [isPermanent, non isLand])))
theorem okAvariceTotemExchange : Action.check (DRS.ofList []) avariceTotemExchange = [] := by decide
/-- Arcanum Wings -- aura swap [CR#702.65a] -/
def arcanumWingsAuraSwap : Ability :=
  activated (mana [generic 2, pip .blue])
    (may
      (exchange (cardsAcross thisAura (a (and [hasSubtype aura, in_
          hand])))))
theorem okArcanumWingsAuraSwap : Ability.check (DRS.ofList []) arcanumWingsAuraSwap = [] := by decide
/-- Tovolar, Dire Overlord -/
def tovolarNightfall : Action := setGameDesignation night
theorem okTovolarNightfall : Action.check (DRS.ofList []) tovolarNightfall = [] := by decide
/-- Spin into Myth -/
def spinIntoMyth : Action :=
  sequentially [putInto (target isCreature) onTop, fateseal (.lit 2)]
theorem okSpinIntoMyth : Action.check (DRS.ofList []) spinIntoMyth = [] := by decide
/-- Pure // Simple -/
def pureHalf : Action := destroy (target (and [isPermanent, isMulticolored]))
theorem okPureHalf : Action.check (DRS.ofList []) pureHalf = [] := by decide
/-- Korlash -/
def grandeurDiscardCost : Cost :=
  perform (discard (a (and [named (printedName "Korlash, Heir to Blackblade"), otherThan this_, in_ yourHand])))
theorem okGrandeurDiscardCost : Cost.check (DRS.ofList []) grandeurDiscardCost = [] := by decide
/-- Gyruda, Doom of Depths -/
def gyrudaCompanion : Ability := companion (everyCardIs isCard (manaValueParity .even))
theorem okGyrudaCompanion : Ability.check (DRS.ofList []) gyrudaCompanion = [] := by decide
/-- Jegantha, the Wellspring -/
def jeganthaCompanion : Ability := companion (noCardIs isCard repeatedManaSymbol)
theorem okJeganthaCompanion : Ability.check (DRS.ofList []) jeganthaCompanion = [] := by decide
/-- Kaheera, the Orphanguard -/
def kaheeraCompanion : Ability :=
  companion
    (everyCardIs (and [isCreature, isCard])
      (anyOf [ aCharacteristic (hasSubtype cat),
                aCharacteristic (hasSubtype elemental),
                aCharacteristic (hasSubtype nightmare),
                aCharacteristic (hasSubtype dinosaur),
                aCharacteristic (hasSubtype beast) ]))
theorem okKaheeraCompanion : Ability.check (DRS.ofList []) kaheeraCompanion = [] := by decide
/-- Keruga, the Macrosage -/
def kerugaCompanion : Ability :=
  companion
    (everyCardIs isCard
      (anyOf [ aCharacteristic (hasManaValue .atLeast (.lit 3)),
                aCharacteristic isLand ]))
theorem okKerugaCompanion : Ability.check (DRS.ofList []) kerugaCompanion = [] := by decide
/-- Lurrus of the Dream-Den -/
def lurrusCompanion : Ability :=
  companion
    (everyCardIs isPermanentCard (aCharacteristic (hasManaValue .atMost (.lit 2))))
theorem okLurrusCompanion : Ability.check (DRS.ofList []) lurrusCompanion = [] := by decide
/-- Lutri, the Spellchaser -/
def lutriCompanion : Ability := companion (groupAgrees
                                             (withDifferentNames
                                                (allOf (and [non isLand, isCard]))))
theorem okLutriCompanion : Ability.check (DRS.ofList []) lutriCompanion = [] := by decide
/-- Obosh, the Preypiercer -/
def oboshCompanion : Ability :=
  companion (everyCardIs isCard (anyOf [manaValueParity .odd, aCharacteristic isLand]))
theorem okOboshCompanion : Ability.check (DRS.ofList []) oboshCompanion = [] := by decide
/-- Umori, the Collector -/
def umoriCompanion : Ability := companion
    (groupAgrees (agree .cardType .share (allOf (and [non isLand, isCard]))))
theorem okUmoriCompanion : Ability.check (DRS.ofList []) umoriCompanion = [] := by decide
/-- Yorion, Sky Nomad -/
def yorionCompanion : Ability := companion (deckSizeOverMinimum 20)
theorem okYorionCompanion : Ability.check (DRS.ofList []) yorionCompanion = [] := by decide
/-- Zirda, the Dawnwaker -/
def zirdaCompanion : Ability :=
  companion (everyCardIs isPermanentCard (hasAbilityOf .anyActivated))
theorem okZirdaCompanion : Ability.check (DRS.ofList []) zirdaCompanion = [] := by decide

/-- Deadpool, Trading Card -/
def deadpoolTradingCard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Deadpool, Trading Card", cost := some [generic 2, pip .black, pip .red],
      supertypes := [.legendary], types := [.creature],
      subtypes := [mutant, mercenary, hero],
      text :=
        [ static
            (replace (would thisCreature enter)
              (also
                (may
                  (establish
                    (textChange thisCreature
                      (exchangeWith (a (and [isCreature, otherThan this_])))))))),
          at_ (beginningOfYour .upkeep) (loseLife (.lit 3)),
          activated (compound [mana [generic 3], perform (sacrifice thisCreature)])
            (instruct (each isOtherPlayer) (draw (.lit 1))) ],
      power := stat 5, toughness := stat 3 } }

end Semantics.Cards

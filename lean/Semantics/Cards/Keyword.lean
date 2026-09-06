import Semantics
import Semantics.Macros
import Semantics.Check.Card
import Semantics.Cards.Trigger

/-!
# Semantics.Cards.Keyword

Port of `idris/src/Experimental/Cards/Keyword.idr`: the printed cards of the Keyword family
(keyword abilities, their parameters and grants) and the bench items beside them.

`battleSquadron` prints a `*/*` box: those slots are `none` under the printed-star ruling.
-/

open Semantics Semantics.Macros

namespace Semantics.Cards

def jump : Instruction := gain (target creature) (keyword "Flying") (some untilEndOfTurn)
theorem okJump : Instruction.check [] jump = [] := by decide
def gabrielAngelfire : Instruction :=
  .sequence
    [ choose (a (qualityFrom .ability
        (.abilitiesAmong [.the "Flying", .the "FirstStrike", .the "Trample", .theWith "Rampage" 3]))),
      gain thisCreature (.thatAbility .theChoice) (some (.until_ (.startOf .upkeep (some .you)))) ]
theorem okGabrielAngelfire : Instruction.check [] gabrielAngelfire = [] := by decide
def builtToSmash : Instruction :=
  .sequence
    [ get (target (.and [creature, attacking])) (.up (.lit 3)) (.up (.lit 3)) (some untilEndOfTurn),
      .doIf (itsA (.and [artifact, creature]))
        (gain it (keyword "Trample") (some untilEndOfTurn)) none ]
theorem okBuiltToSmash : Instruction.check [] builtToSmash = [] := by decide

/-- A flying Thopter token. -/
def thopterToken : CharacteristicBundle :=
  { characteristics :=
    { types := [.artifact, .creature], subtypes := [creatureType "Thopter"],
      text := [keyword "Flying"], power := stat 1, toughness := stat 1 } }

def aviationPioneer : Instruction := create (.lit 1) thopterToken
theorem okAviationPioneer : Instruction.check [] aviationPioneer = [] := by decide
def fireNavyTrebuchet : Instruction :=
  createTappedAttacking (.lit 1)
    { characteristics :=
      { name := "Ballistic Boulder", types := [.artifact, .creature],
        subtypes := [creatureType "Construct"], text := [keyword "Flying"],
        power := stat 2, toughness := stat 1 } }
theorem okFireNavyTrebuchet : Instruction.check [] fireNavyTrebuchet = [] := by decide

def rorixBladewing : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rorix Bladewing", cost := some [generic 3, pip .red, pip .red, pip .red],
      supertypes := [.legendary], types := [.creature], subtypes := [creatureType "Dragon"],
      text := [keyword "Flying", keyword "Haste"], power := stat 6, toughness := stat 5 } }

def yotianSoldier : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Yotian Soldier", cost := some [generic 3], types := [.artifact, .creature],
      subtypes := [creatureType "Soldier"], text := [keyword "Vigilance"],
      power := stat 1, toughness := stat 4 } }

/-- Pym Particles -/
def pymParticlesGrant : Instruction :=
  .sequence
    [ gain (target creature) (keyword "Vigilance") (some untilEndOfTurn),
      .establish (deontic (that (.type .creature)) .forbid [.core .block] .patient .noPatient)
        (some .thisTurn) ]
theorem okPymParticlesGrant : Instruction.check [] pymParticlesGrant = [] := by decide
def bladebrand : Instruction := gain (target creature) (keyword "Deathtouch") (some untilEndOfTurn)
theorem okBladebrand : Instruction.check [] bladebrand = [] := by decide
def criticalHit : Instruction :=
  gain (target creature) (keyword "DoubleStrike") (some untilEndOfTurn)
theorem okCriticalHit : Instruction.check [] criticalHit = [] := by decide
def lightningBlow : Instruction :=
  gain (target creature) (keyword "FirstStrike") (some untilEndOfTurn)
theorem okLightningBlow : Instruction.check [] lightningBlow = [] := by decide
def deathByDragons : Instruction :=
  .create (.lit 1)
    (.written
      { characteristics :=
        { colors := [.red], types := [.creature], subtypes := [creatureType "Dragon"],
          text := [keyword "Flying"], power := stat 5, toughness := stat 5 } })
    [] (agent := (each (.and [.anyPlayer, .otherThan (target .anyPlayer)])))
theorem okDeathByDragons : Instruction.check [] deathByDragons = [] := by decide

/-- Concordant Crossroads -/
def concordantCrossroads : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Concordant Crossroads", cost := some [pip .green], supertypes := [.world],
      types := [.enchantment], text := [.static (.abilityGrant (allOf creature) (keyword "Haste"))]
          } }

/-- Bristlepack Sentry -/
def bristlepackSentry : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bristlepack Sentry", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [creatureType "Plant", creatureType "Wolf"],
      text :=
        [ keyword "Defender",
          .static (onlyWhile
            (canDoAsThough thisCreature (.core .attack) (.not (.hasKeyword (.the "Defender"))))
            (exists_ (.and [ creature, .hasPossessor .controller .you,
                             .compare [.stat .power] .atLeast (.lit 4) ]))) ],
      power := stat 3, toughness := stat 3 } }

def platinumAngel : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Platinum Angel", cost := some [generic 7], types := [.artifact, .creature],
      subtypes := [creatureType "Angel"],
      text :=
        [ keyword "Flying",
          .static (playerCant (.core .loseGame) .you),
          .static (playerCant (.core .winGame) (.playerGroup .yourOpponents)) ],
      power := stat 4, toughness := stat 4 } }

def abyssalPersecutor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Abyssal Persecutor", cost := some [generic 2, pip .black, pip .black],
      types := [.creature], subtypes := [creatureType "Demon"],
      text :=
        [ keyword "Flying", keyword "Trample",
          .static (playerCant (.core .winGame) .you),
          .static (playerCant (.core .loseGame) (.playerGroup .yourOpponents)) ],
      power := stat 6, toughness := stat 6 } }

def smogElemental : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Smog Elemental", cost := some [generic 4, pip .black, pip .black],
      types := [.creature], subtypes := [creatureType "Elemental"],
      text :=
        [ keyword "Flying",
          .static (getsPt
            (allOf (.and [ creature, .hasKeyword (.the "Flying"),
                           .hasPossessor .controller (.playerGroup .yourOpponents) ]))
            (.down (.lit 1)) (.down (.lit 1))) ],
      power := stat 3, toughness := stat 3 } }

def exquisiteArchangel : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Exquisite Archangel", cost := some [generic 5, pip .white, pip .white],
      types := [.creature], subtypes := [creatureType "Angel"],
      text :=
        [ keyword "Flying",
          .static (.replacement (.losesGame .you) [] none
            (.sequence
              [ exile thisCreature,
                setLife (.statOf (.playerStat .startingLifeTotal) .you) (agent := .you) ])
            .repeatedly none) ],
      power := stat 5, toughness := stat 5 } }

def adelbertSteiner : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Adelbert Steiner", cost := some [generic 1, pip .white], supertypes := [.legendary],
      types := [.creature], subtypes := [creatureType "Human", creatureType "Knight"],
      text :=
        [ keyword "Lifelink",
          .static (getsPt thisCreature
            (.up (forEach 1 (.and [.hasSubtype (artifactType "Equipment"), .hasPossessor .controller .you])))
            (.up (forEach 1 (.and [.hasSubtype (artifactType "Equipment"), .hasPossessor .controller .you])))) ],
      power := stat 2, toughness := stat 1 } }

def inspiringStatuary : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Inspiring Statuary", cost := some [generic 3], types := [.artifact],
      text :=
        [ .static (.abilityGrant (allOf (.and [.not artifact, spell, castBy .you])) (keyword
            "Improvise")) ] } }

def chiefEngineer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chief Engineer", cost := some [generic 1, pip .blue], types := [.creature],
      subtypes := [creatureType "Vedalken", creatureType "Artificer"],
      text := [.static (.abilityGrant (allOf (.and [artifact, spell, castBy .you])) (keyword
          "Convoke"))],
      power := stat 1, toughness := stat 3 } }

def firesongAndSunspeaker : Ability :=
  .static (.abilityGrant
    (allOf (.and [.colorIs .red, instantOrSorcery, spell, .hasPossessor .controller .you]))
    (keyword "Lifelink"))
theorem okFiresongAndSunspeaker : Ability.check [] firesongAndSunspeaker = [] := by decide
def briaRiptideRogue : Ability :=
  .static (.abilityGrant (allOf (otherCreatureYouControl thisCreature)) (keyword "Prowess"))
theorem okBriaRiptideRogue : Ability.check [] briaRiptideRogue = [] := by decide
def narsetEnlightenedExile : Ability :=
  .static (.abilityGrant (allOf creatureYouControl) (keyword "Prowess"))
theorem okNarsetEnlightenedExile : Ability.check [] narsetEnlightenedExile = [] := by decide
def pontiffOfBlight : Ability :=
  .static (.abilityGrant (allOf (otherCreatureYouControl thisCreature)) (keyword "Extort"))
theorem okPontiffOfBlight : Ability.check [] pontiffOfBlight = [] := by decide

def prismariTheInspiration : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Prismari, the Inspiration", cost := some [generic 5, pip .blue, pip .red],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Elder", creatureType "Dragon"],
      text :=
        [ keyword "Flying",
          keywordCosting "Ward" (payLife .you 5),
          .static (.abilityGrant (allOf (.and [instantOrSorcery, spell, castBy .you])) storm) ],
      power := stat 7, toughness := stat 7 } }

def tomakulHonorGuard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tomakul Honor Guard", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Soldier"],
      text := [keywordCosting "Ward" (.mana [generic 2])], power := stat 3, toughness := stat 1 } }

def repentantBlacksmith : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Repentant Blacksmith", cost := some [generic 1, pip .white], types := [.creature],
      subtypes := [creatureType "Human"],
      text := [keywordQuality "Protection" (.colorIs .red)], power := stat 1, toughness := stat 2 } }

def battleSquadron : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Battle Squadron", cost := some [generic 3, pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Goblin"],
      text :=
        [ keyword "Flying",
          .static (.ptDefinition thisCreature .bothEach (countOf creatureYouControl)) ] } }

def eomerOfTheRiddermark : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Éomer of the Riddermark", cost := some [generic 4, pip .red],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Knight"],
      text :=
        [ keyword "Haste",
          triggeredIf (attacks thisCreature)
            (exists_ (.and [ creature, .hasPossessor .controller .you,
                             .superlative .max (.stat .power) (.and [creature, permanent]) ]))
            (create (.lit 1) (creatureToken 1 1 [.white] [creatureType "Human", creatureType "Soldier"])) ],
      power := stat 5, toughness := stat 4 } }

def angelicGift : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Angelic Gift", cost := some [generic 1, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          when (.enters thisAura none) (.draw (.lit 1) (agent := .you)),
          .static (.abilityGrant (.attachHost .enchanted (.type .creature)) (keyword "Flying")) ] }
              }

def consulsLieutenant : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Consul's Lieutenant", cost := some [pip .white, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Soldier"],
      text :=
        [ keyword "FirstStrike",
          renown 1,
          triggeredIf (attacks thisCreature) (.matches it (.hasDesignation "renowned" none))
            (get (allOf (.and [creature, attacking, .hasPossessor .controller .you, .otherThan
                thisCreature]))
              (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn)) ],
      power := stat 2, toughness := stat 1 } }

def secretsOfTheGoldenCity : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Secrets of the Golden City", cost := some [generic 1, pip .blue, pip .blue],
      types := [.sorcery],
      text :=
        [ keyword "Ascend",
          .spell none (.replace (.draw (.lit 2) (agent := .you))
            (.doIf (.matches .you (.hasDesignation "the city's blessing" none)) (.draw (.lit 3)
                (agent := .you))
              none)) ] } }

def bombur : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bombur, Gentle Dreamer", cost := some [generic 2, pip .red],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Dwarf", creatureType "Bard"],
      text :=
        [ keyword "Storied",
          .static (onlyUnless (doesntUntap thisCreature (some .you))
            (.matches .you (.hasDesignation "an enduring story" none))) ],
      power := stat 5, toughness := stat 3 } }

def drachNyen : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Drach'Nyen", cost := some [generic 4, pip .black, pip .red],
      supertypes := [.legendary], types := [.artifact], subtypes := [artifactType "Equipment"],
      text :=
        [ when (.enters thisEquipment none) (exile (.described (.target (upTo 1)) creature)),
          .static (.conjunction none
            [ .abilityGrant (.attachHost .equipped (.type .creature)) (keyword "Menace"),
              .modification (.attachHost .equipped (.type .creature)) .power (.up (.letter .x)),
              .modification (.attachHost .equipped (.type .creature)) .toughness (.up (.lit 0)),
              .letterDefinition .x (.statOf (.stat .power) (the (.exiledWith thisEquipment))) ]),
          keywordCosting "Equip" (.mana [generic 2]) ] } }

def colossalGraveReaver : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Colossal Grave-Reaver", cost := some [generic 6, pip .black, pip .green],
      types := [.creature], subtypes := [creatureType "Dragon"],
      text :=
        [ keyword "Flying",
          triggeredOr (.enters thisCreature none) [attacks thisCreature] (mill (.lit 3) .you (agent
              := .you)),
          whenever
            (putIntoFrom (counted (atLeast 1) (.and [creature, .inZone yourLibrary])) (graveyardOf .you)
              (.zones [yourLibrary]))
            (putOntoBattlefield (someOf (exactly 1) them)) ],
      power := stat 7, toughness := stat 6 } }

/-- Grimdancer -/
def grimdancer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Grimdancer", cost := some [generic 1, pip .black, pip .black], types := [.creature],
      subtypes := [creatureType "Nightmare"],
      text :=
        [ .static (.entryRider thisCreature
            (.withCounters (.lit 2)
              (.distinctChosen [.keyword "Menace", .keyword "Deathtouch", .keyword "Lifelink"]) .fresh)) ],
      power := stat 3, toughness := stat 3 } }

def divineVisitation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Divine Visitation", cost := some [generic 3, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ .static (.replacement
            (.tokensCreated (counted (atLeast 1) (.and [creature, .isToken])) false none (some .you))
            [] none
            (create .groupSize
              { characteristics :=
                { colors := [.white], types := [.creature], subtypes := [creatureType "Angel"],
                  text := [keyword "Flying", keyword "Vigilance"], power := stat 4, toughness := stat 4 } })
            .repeatedly none) ] } }

def adrixAndNev : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Adrix and Nev, Twincasters", cost := some [generic 2, pip .green, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Merfolk", creatureType "Wizard"],
      text :=
        [ keywordCosting "Ward" (.mana [generic 2]),
          .static (.replacement (.tokensCreated (counted (atLeast 1) .isToken) false none (some
              .you))
            [] none (.create (times (.lit 2) .groupSize) .asThose [] (agent := .you)) .repeatedly
                none) ],
      power := stat 2, toughness := stat 2 } }

def windZendikon : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Wind Zendikon", cost := some [pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" land,
          .static (.qualityChange (.attachHost .enchanted (.type .land)) .sets
            (.bundle
              { characteristics :=
                { colors := [.blue], types := [.creature], subtypes := [creatureType "Elemental"],
                  text := [keyword "Flying"], power := stat 2, toughness := stat 2 } }
              (some .land))),
          when (.dies (.attachHost .enchanted (.type .land))) (move (that .card) hand) ] } }

def awakenTheBear : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Awaken the Bear", cost := some [generic 2, pip .green], types := [.instant],
      text :=
        [ .spell none (.establish
            (.conjunction none
              [ .modification (target creature) .power (.up (.lit 3)),
                .modification (itsOther (target creature) (.up (.lit 3))) .toughness (.up (.lit 3)),
                .abilityGrant it (keyword "Trample") ])
            (some untilEndOfTurn)) ] } }

def spidersilkArmor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Spidersilk Armor", cost := some [generic 2, pip .green], types := [.enchantment],
      text :=
        [ .static (.conjunction none
            [ .modification (allOf creatureYouControl) .power (.up (.lit 0)),
              .modification (itsOther (allOf creatureYouControl) (.up (.lit 0))) .toughness (.up
                  (.lit 1)),
              .abilityGrant them (keyword "Reach") ]) ] } }

def arcaneFlight : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Arcane Flight", cost := some [pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          .static (.conjunction none
            [ .modification (.attachHost .enchanted (.type .creature)) .power (.up (.lit 1)),
              .modification (itsOther (.attachHost .enchanted (.type .creature)) (.up (.lit 1)))
                  .toughness
                (.up (.lit 1)),
              .abilityGrant it (keyword "Flying") ]) ] } }

def bootsOfSpeed : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Boots of Speed", cost := some [pip .red], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ .static (.conjunction none
            [ .modification (.attachHost .equipped (.type .creature)) .power (.up (.lit 1)),
              .modification (itsOther (.attachHost .equipped (.type .creature)) (.up (.lit 1)))
                  .toughness
                (.up (.lit 0)),
              .abilityGrant it (keyword "Haste") ]),
          keywordCosting "Equip" (.mana [generic 1]) ] } }

/-- "enchanted creature loses all abilities and is a <colors> <subtype> with base P/T" -/
def enchantedBecomesVanilla (colors : List Color) (subtype : String) (power toughness : Nat) :
    StaticSpec :=
  .conjunction none
    [ .allAbilityLoss (.attachHost .enchanted (.type .creature)) none,
      .qualityChange it .sets
        (.bundle { characteristics := { colors, types := [.creature], subtypes := [creatureType subtype] } }
          none),
      .modification it .power (.set (.lit power)),
      .modification (itsOther it (.set (.lit power))) .toughness (.set (.lit toughness)) ]

def frogify : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Frogify", cost := some [generic 1, pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          .static (enchantedBecomesVanilla [.blue] "Frog" 1 1) ] } }

def darksteelMutation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Darksteel Mutation", cost := some [generic 1, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          .static (.conjunction none
            [ .qualityChange (.attachHost .enchanted (.type .creature)) .sets
                (.bundle { characteristics := { types := [.artifact, .creature], subtypes := [creatureType "Insect"] } }
                  none),
              .modification it .power (.set (.lit 0)),
              .modification (itsOther it (.set (.lit 0))) .toughness (.set (.lit 1)),
              .abilityGrant it (keyword "Indestructible"),
              .allAbilityLoss it none ]) ] } }

def kenrithsTransformation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Kenrith's Transformation", cost := some [generic 1, pip .green],
      types := [.enchantment], subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          when (.enters thisAura none) (.draw (.lit 1) (agent := .you)),
          .static (enchantedBecomesVanilla [.green] "Elk" 3 3) ] } }

def amphibianDownpour : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Amphibian Downpour", cost := some [generic 2, pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keyword "Flash", storm,
          keywordSubject "Enchant" creature,
          .static (enchantedBecomesVanilla [.blue] "Frog" 1 1) ] } }

def lignify : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lignify", cost := some [generic 1, pip .green], types := [.kindred, .enchantment],
      subtypes := [creatureType "Treefolk", enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          .static (.conjunction none
            [ .qualityChange (.attachHost .enchanted (.type .creature)) .sets
                (.bundle { characteristics := { subtypes := [creatureType "Treefolk"] } } none),
              .modification it .power (.set (.lit 0)),
              .modification (itsOther it (.set (.lit 0))) .toughness (.set (.lit 4)),
              .allAbilityLoss it none ]) ] } }

def nefariousImp : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Nefarious Imp", cost := some [generic 2, pip .black], types := [.creature],
      subtypes := [creatureType "Imp"],
      text :=
        [ keyword "Flying",
          whenever
            (leavesBattlefield (counted (atLeast 1) (.and [permanent, .hasPossessor .controller .you])))
            (scry (.lit 1) (agent := .you)) ],
      power := stat 2, toughness := stat 1 } }

/-- Ainok Tracker -/
def ainokTracker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ainok Tracker", cost := some [generic 5, pip .red], types := [.creature],
      subtypes := [creatureType "Dog", creatureType "Scout"],
      text := [keyword "FirstStrike", keywordCosting "Morph" (.mana [generic 4, pip .red])],
      power := stat 3, toughness := stat 3 } }

def irenicussVileDuplication : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Irenicus's Vile Duplication", cost := some [generic 3, pip .blue], types := [.sorcery],
      text :=
        [ .spell none (.create (.lit 1)
            (.copyOf (target (.and [creature, .hasPossessor .controller .you]))
              [.ability (keyword "Flying"), .nonlegendary])
            [] (agent := .you)) ] } }

def cacklingCounterpart : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cackling Counterpart", cost := some [generic 1, pip .blue, pip .blue],
      types := [.instant],
      text :=
        [ .spell none (.create (.lit 1)
            (.copyOf (target (.and [creature, .hasPossessor .controller .you])) []) [] (agent :=
                .you)),
          keywordCosting "Flashback" (.mana [generic 5, pip .blue, pip .blue]) ] } }

def chandrasSpitfire : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chandra's Spitfire", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text :=
        [ keyword "Flying",
          whenever (.isDealtDamage .noncombatOnly (a .opponent))
            (get thisCreature (.up (.lit 3)) (.up (.lit 0)) (some untilEndOfTurn)) ],
      power := stat 1, toughness := stat 3 } }

def livingHive : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Living Hive", cost := some [generic 6, pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Elemental", creatureType "Insect"],
      text :=
        [ keyword "Trample",
          whenever (dealsCombatDamage thisCreature (a .anyPlayer))
            (create .thatMuch (creatureToken 1 1 [.green] [creatureType "Insect"])) ],
      power := stat 6, toughness := stat 6 } }

def giantCindermaw : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Giant Cindermaw", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [creatureType "Dinosaur", creatureType "Beast"],
      text := [keyword "Trample", .static (playerCant (.core .gainLife) (.playerGroup .allPlayers))],
      power := stat 4, toughness := stat 3 } }

def lushGrowth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lush Growth", cost := some [pip .green], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" land,
          .static (.qualityChange (.attachHost .enchanted (.type .land)) .sets
            (.bundle
              { characteristics :=
                { subtypes := [landType "Mountain", landType "Forest", landType "Plains"] } }
              none)) ] } }

def voiceOfAll : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Voice of All", cost := some [generic 2, pip .white, pip .white], types := [.creature],
      subtypes := [creatureType "Angel"],
      text :=
        [ keyword "Flying",
          .static (entersChoosing thisCreature .color),
          .static (.abilityGrant thisCreature (keywordQuality "Protection" (ofChosen .color))) ],
      power := stat 2, toughness := stat 2 } }

def wardSliver : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ward Sliver", cost := some [generic 4, pip .white], types := [.creature],
      subtypes := [creatureType "Sliver"],
      text :=
        [ .static (entersChoosing thisCreature .color),
          .static (.abilityGrant (allOf (.hasSubtype (creatureType "Sliver")))
            (keywordQuality "Protection" (ofChosen .color))) ],
      power := stat 2, toughness := stat 2 } }

def sanctuaryBlade : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sanctuary Blade", cost := some [generic 2], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ .static (attachChoosing thisEquipment .color),
          .static (.conjunction none
            [ .modification (.attachHost .equipped (.type .creature)) .power (.up (.lit 2)),
              .modification (itsOther (.attachHost .equipped (.type .creature)) (.up (.lit 2)))
                  .toughness
                (.up (.lit 0)),
              .abilityGrant (.attachHost .equipped (.type .creature))
                (keywordQuality "Protection" (ofTheLastChosen .color)) ]),
          keywordCosting "Equip" (.mana [generic 3]) ] } }

/-- Sinister Strength -/
def sinisterStrength : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sinister Strength", cost := some [generic 1, pip .black], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          .static (.conjunction none
            [ .modification (.attachHost .enchanted (.type .creature)) .power (.up (.lit 3)),
              .modification (itsOther (.attachHost .enchanted (.type .creature)) (.up (.lit 3)))
                  .toughness
                (.up (.lit 1)),
              .qualityChange it .sets (.colored (.some [.black])) ]) ] } }

/-- Crimson Wisps -/
def crimsonWisps : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Crimson Wisps", cost := some [pip .red], types := [.instant],
      text :=
        [ .spell none (.sequence
            [ .establish
                (.conjunction none
                  [ .qualityChange (target creature) .sets (.colored (.some [.red])),
                    .abilityGrant it (keyword "Haste") ])
                (some untilEndOfTurn),
              .draw (.lit 1) (agent := .you) ]) ] } }

/-- Ghoulflesh -/
def ghoulflesh : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ghoulflesh", cost := some [pip .black], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          .static (.conjunction none
            [ .modification (.attachHost .enchanted (.type .creature)) .power (.down (.lit 1)),
              .modification (itsOther (.attachHost .enchanted (.type .creature)) (.down (.lit 1)))
                  .toughness
                (.down (.lit 1)),
              .qualityChange it .adds
                (.bundle { characteristics := { colors := [.black], subtypes := [creatureType "Zombie"] } } none) ]) ] } }

/-- Blade of the Oni -/
def bladeOfTheOniStatic : StaticSpec :=
  .conjunction none
    [ .modification (.attachHost .equipped (.type .creature)) .power (.set (.lit 5)),
      .modification (itsOther (.attachHost .equipped (.type .creature)) (.set (.lit 5))) .toughness
        (.set (.lit 5)),
      .abilityGrant it (keyword "Menace"),
      .qualityChange it .adds
        (.bundle { characteristics := { colors := [.black], subtypes := [creatureType "Demon"] } } none) ]
theorem okBladeOfTheOniStatic : StaticSpec.check [] bladeOfTheOniStatic = [] := by decide

def historyOfBenalia : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "History of Benalia", cost := some [generic 1, pip .white, pip .white],
      types := [.enchantment], subtypes := [enchantmentType "Saga"],
      text :=
        [ when (.chapterMark [1, 2])
            (create (.lit 1)
              { characteristics :=
                { colors := [.white], types := [.creature], subtypes := [creatureType "Knight"],
                  text := [keyword "Vigilance"], power := stat 2, toughness := stat 2 } }),
          when (.chapterMark [3])
            (get (allOf (.and [.hasSubtype (creatureType "Knight"), .hasPossessor .controller
                .you]))
              (.up (.lit 2)) (.up (.lit 1)) (some untilEndOfTurn)) ] } }

/-- Akroan Sergeant -/
def akroanSergeant : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Akroan Sergeant", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Soldier"],
      text := [keyword "FirstStrike", renown 1], power := stat 2, toughness := stat 2 } }

def castCreatureSpellsFromTop : StaticSpec :=
  mayPlayDeed (.action "Cast") .you (allOf (.and [spell, creature])) none
    (.play (some onTop) none none false .itsOwnCost)
theorem okCastCreatureSpellsFromTop : StaticSpec.check [] castCreatureSpellsFromTop = [] := by decide

def garruksHorde : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Garruk's Horde", cost := some [generic 5, pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Beast"],
      text :=
        [ keyword "Trample",
          .static (.visibility .reveal .you .topOfLibrary),
          .static castCreatureSpellsFromTop ],
      power := stat 7, toughness := stat 7 } }

def wanderingEye : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Wandering Eye", cost := some [generic 2, pip .blue], types := [.creature],
      subtypes := [creatureType "Illusion"],
      text := [keyword "Flying", .static (.visibility .reveal (.playerGroup .allPlayers) .wholeHand)],
      power := stat 1, toughness := stat 3 } }

def amorphousAxe : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Amorphous Axe", cost := some [generic 2], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ .static (.conjunction none
            [ .modification (.attachHost .equipped (.type .creature)) .power (.up (.lit 3)),
              .modification (itsOther (.attachHost .equipped (.type .creature)) (.up (.lit 3)))
                  .toughness
                (.up (.lit 0)),
              .qualityChange (.attachHost .equipped (.type .creature)) .adds (.everyTypeOf
                  .creature) ]),
          keywordCosting "Equip" (.mana [generic 3]) ] } }

def runedStalactite : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Runed Stalactite", cost := some [generic 1], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ .static (.conjunction none
            [ .modification (.attachHost .equipped (.type .creature)) .power (.up (.lit 1)),
              .modification (itsOther (.attachHost .equipped (.type .creature)) (.up (.lit 1)))
                  .toughness
                (.up (.lit 1)),
              .qualityChange (.attachHost .equipped (.type .creature)) .adds (.everyTypeOf
                  .creature) ]),
          keywordCosting "Equip" (.mana [generic 2]) ] } }

def arachnoform : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Arachnoform", cost := some [generic 1, pip .green], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          .static (.conjunction none
            [ .modification (.attachHost .enchanted (.type .creature)) .power (.up (.lit 2)),
              .modification (itsOther (.attachHost .enchanted (.type .creature)) (.up (.lit 2)))
                  .toughness
                (.up (.lit 2)),
              .abilityGrant (.attachHost .enchanted (.type .creature)) (keyword "Reach"),
              .qualityChange (.attachHost .enchanted (.type .creature)) .adds (.everyTypeOf
                  .creature) ]) ] } }

def nyleasPresence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Nylea's Presence", cost := some [generic 1, pip .green], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" land,
          when (.enters thisAura none) (.draw (.lit 1) (agent := .you)),
          .static (.qualityChange (.attachHost .enchanted (.type .land)) .adds (.everyTypeOf
              .basicLand)) ] } }

/-- Curse of Conformity -/
def curseOfConformity : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Curse of Conformity", cost := some [generic 4, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura", enchantmentType "Curse"],
      text :=
        [ keywordSubject "Enchant" .anyPlayer,
          .static (.conjunction none
            [ .modification
                (allOf (.and [ creature, .not (.hasSupertype .legendary),
                               .hasPossessor .controller (.attachHost .enchanted .player) ]))
                .power (.set (.lit 3)),
              .modification
                (itsOther
                  (allOf (.and [ creature, .not (.hasSupertype .legendary),
                                 .hasPossessor .controller (.attachHost .enchanted .player) ]))
                  (.set (.lit 3)))
                .toughness (.set (.lit 3)),
              .qualityChange them .loses (.everyTypeOf .creature) ]) ] } }

/-- Drumbellower -/
def drumbellower : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Drumbellower", cost := some [generic 2, pip .white], types := [.creature],
      subtypes := [creatureType "Spirit"],
      text :=
        [ keyword "Flying",
          .static (untapsDuring (allOf (.and [creature, .hasPossessor .controller .you]))
            (some (each otherPlayer))) ],
      power := stat 2, toughness := stat 1 } }

def tayamLine : StaticSpec :=
  entersWithAdditionalCounters
    (each (.and [creature, .hasPossessor .controller .you, .otherThan thisCreature])) (.lit 1)
    (.keyword "Vigilance")
theorem okTayamLine : StaticSpec.check [] tayamLine = [] := by decide

def clarionConqueror : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Clarion Conqueror", cost := some [generic 2, pip .white], types := [.creature],
      subtypes := [creatureType "Dragon"],
      text :=
        [ keyword "Flying",
          .static (objectCant (.action "Activate")
            (allOf (.and [ .abilityHead .anyActivated,
                           .abilityOf (allOf (.or [artifact, creature, .hasType .planeswalker])) ]))) ],
      power := stat 3, toughness := stat 3 } }

/-- Kang the Conqueror -/
def kangPowerUpLock : StaticSpec :=
  objectCant (.action "Activate") (allOf (.abilityHead (.keyword "PowerUp")))
theorem okKangPowerUpLock : StaticSpec.check [] kangPowerUpLock = [] := by decide

def vipersKiss : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Viper's Kiss", cost := some [pip .black], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          .static (.conjunction none
            [ .modification (.attachHost .enchanted (.type .creature)) .power (.down (.lit 1)),
              .modification (itsOther (.attachHost .enchanted (.type .creature)) (.down (.lit 1)))
                  .toughness
                (.down (.lit 1)),
              objectCant (.action "Activate")
                (allOf (.and [.abilityHead .anyActivated, .abilityOf it])) ]) ] } }

def stupefyingTouch : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stupefying Touch", cost := some [generic 1, pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          when (.enters thisAura none) (.draw (.lit 1) (agent := .you)),
          .static (objectCant (.action "Activate")
            (allOf (.and [ .abilityHead .anyActivated,
                           .abilityOf (.attachHost .enchanted (.type .creature)) ]))) ] } }

def linvalaKeeperOfSilence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Linvala, Keeper of Silence", cost := some [generic 2, pip .white, pip .white],
      supertypes := [.legendary], types := [.creature], subtypes := [creatureType "Angel"],
      text :=
        [ keyword "Flying",
          .static (objectCant (.action "Activate")
            (allOf (.and [ .abilityHead .anyActivated,
                           .abilityOf (allOf (.and [creature, .hasPossessor .controller (.playerGroup .yourOpponents)])) ]))) ],
      power := stat 3, toughness := stat 4 } }

def kyrenLegate : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Kyren Legate", cost := some [generic 1, pip .red], types := [.creature],
      subtypes := [creatureType "Goblin"],
      text :=
        [ .static (onlyWhile (.altCost .this none)
            (.and
              [ exists_ (.and [ land, .hasSubtype (landType "Plains"),
                                .hasPossessor .controller anOpponent ]),
                exists_ (.and [ land, .hasSubtype (landType "Mountain"),
                                .hasPossessor .controller .you ]) ])),
          keyword "Haste" ],
      power := stat 1, toughness := stat 1 } }

def polarKraken : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Polar Kraken", cost := some [generic 8, pip .blue, pip .blue, pip .blue],
      types := [.creature], subtypes := [creatureType "Kraken"],
      text :=
        [ keyword "Trample",
          .static (entersTapped thisCreature),
          cumulativeUpkeep (.perform (sacrifice (a land) (agent := .you))) ],
      power := stat 11, toughness := stat 11 } }

def yavimayaAnts : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Yavimaya Ants", cost := some [generic 2, pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Insect"],
      text := [keyword "Trample", keyword "Haste", cumulativeUpkeep (.mana [pip .green, pip .green])],
      power := stat 5, toughness := stat 1 } }

def illusionaryForces : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Illusionary Forces", cost := some [generic 3, pip .blue], types := [.creature],
      subtypes := [creatureType "Illusion"],
      text := [keyword "Flying", cumulativeUpkeep (.mana [pip .blue])],
      power := stat 4, toughness := stat 4 } }

def vexingSphinx : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vexing Sphinx", cost := some [generic 1, pip .blue, pip .blue], types := [.creature],
      subtypes := [creatureType "Sphinx"],
      text :=
        [ keyword "Flying",
          cumulativeUpkeep (.perform (discard (a (.inZone hand)) (agent := .you))),
          when (.dies thisCreature) (.draw (countersOn (.named "Age") it) (agent := .you)) ],
      power := stat 4, toughness := stat 4 } }

def manaChains : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mana Chains", cost := some [pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          .static (.abilityGrant (.attachHost .enchanted (.type .creature)) (cumulativeUpkeep (.mana
              [generic 1]))) ] } }

def dreamThief : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dream Thief", cost := some [generic 2, pip .blue], types := [.creature],
      subtypes := [creatureType "Faerie", creatureType "Rogue"],
      text :=
        [ keyword "Flying",
          when (.enters thisCreature none)
            (.doOnlyIf (.draw (.lit 1) (agent := .you))
              (happenedInvolving .spellCast .you .thisTurn
                (a (.and [spell, .colorIs .blue, .otherThan .this])))
              none) ],
      power := stat 2, toughness := stat 1 } }

def brightspearZealot : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Brightspear Zealot", cost := some [generic 2, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Soldier"],
      text :=
        [ keyword "Vigilance",
          .static (onlyWhile (getsPt thisCreature (.up (.lit 2)) (.up (.lit 0)))
            (.compareAmt (eventCountInvolving .spellCast .you .thisTurn (a spell)) .atLeast (.lit 2))) ],
      power := stat 2, toughness := stat 4 } }

def deepwayNavigator : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Deepway Navigator", cost := some [pip .white, pip .blue], types := [.creature],
      subtypes := [creatureType "Merfolk", creatureType "Wizard"],
      text :=
        [ keyword "Flash",
          when (.enters thisCreature none)
            (.setStatus .untapped
              (each (.and [ .hasSubtype (creatureType "Merfolk"), .hasPossessor .controller .you,
                            .otherThan thisCreature ]))),
          .static (onlyWhile
            (getsPt (allOf (.and [.hasSubtype (creatureType "Merfolk"), .hasPossessor .controller .you]))
              (.up (.lit 1)) (.up (.lit 0)))
            (happenedInvolving .attackDeclaration .you .thisTurn
              (counted (atLeast 3) (.hasSubtype (creatureType "Merfolk"))))) ],
      power := stat 2, toughness := stat 2 } }

def bloodfireEnforcers : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bloodfire Enforcers", cost := some [generic 3, pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Monk"],
      text :=
        [ .static (onlyWhile
            (.conjunction none [.abilityGrant thisCreature (keyword "FirstStrike"), .abilityGrant it
                (keyword "Trample")])
            (.and
              [ exists_ (.and [instant, .inZone (graveyardOf .you)]),
                exists_ (.and [sorcery, .inZone (graveyardOf .you)]) ])) ],
      power := stat 5, toughness := stat 2 } }

def stormOfSouls : Instruction :=
  .sequence
    [ move (allOf (.and [creature, .inZone (graveyardOf .you)])) battlefield,
      become them
        { characteristics :=
          { subtypes := [creatureType "Spirit"], text := [keyword "Flying"],
            power := stat 1, toughness := stat 1 } }
        none,
      exile .this ]
theorem okStormOfSouls : Instruction.check [] stormOfSouls = [] := by decide

def answeredPrayers : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Answered Prayers", cost := some [generic 1, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ when (.enters (a creatureYouControl) none)
            (.sequence
              [ gainLife (.lit 1) (agent := .you),
                .doIf (.not (.matches thisEnchantment creature))
                  (become (itCondSubject [] (.not (.matches thisEnchantment creature)))
                    { characteristics :=
                      { types := [.creature], subtypes := [creatureType "Angel"],
                        text := [keyword "Flying"], power := stat 3, toughness := stat 3 } }
                    (some untilEndOfTurn))
                  none ]) ] } }

/-- Hate Mirage's middle two sentences -/
def hateMirageTokens : Instruction :=
  .sequence
    [ .doForEach (.described (.target (upTo 2)) creatureYouDontControl)
        (.create (.lit 1) (.copyOf it []) [] (agent := .you)),
      gain (those .token) (keyword "Haste") none ]
theorem okHateMirageTokens : Instruction.check [] hateMirageTokens = [] := by decide
def descentOfTheDragons : Instruction :=
  .sequence
    [ destroy (.described (.target anyNumber) creature),
      .doForEach (theVerbed (.action "Destroy") (.type .creature) .thisWay .many)
        (.create (.lit 1)
          (.written
            { characteristics :=
              { colors := [.red], types := [.creature], subtypes := [creatureType "Dragon"],
                text := [keyword "Flying"], power := stat 4, toughness := stat 4 } })
          [] (agent := (controllerOf it))) ]
theorem okDescentOfTheDragons : Instruction.check [] descentOfTheDragons = [] := by decide
/-- Brimaz, King of Oreskos -/
def brimazAttackToken : Ability :=
  whenever (attacks thisCreature)
    (.create (.lit 1)
      (.written
        { characteristics :=
          { colors := [.white], types := [.creature], subtypes := [creatureType "Cat", creatureType "Soldier"],
            text := [keyword "Vigilance"], power := stat 1, toughness := stat 1 } })
      [.entersAttacking none] (agent := .you))
theorem okBrimazAttackToken : Ability.check [] brimazAttackToken = [] := by decide
/-- Sedris, the Traitor King -/
def sedrisTheTraitorKing : Ability :=
  .static (.abilityGrant (each (.and [creature, .inZone (graveyardOf .you)]))
    (keywordCosting "Unearth" (.mana [generic 2, pip .black])))
theorem okSedrisTheTraitorKing : Ability.check [] sedrisTheTraitorKing = [] := by decide
/-- Grixis -/
def grixis : Ability :=
  .static (.abilityGrant
    (allOf (.and [ .or [.colorIs .blue, .colorIs .black, .colorIs .red], creature,
                   .inZone (graveyardOf .you) ]))
    (keywordCosting "Unearth" .itsManaCost))
theorem okGrixis : Ability.check [] grixis = [] := by decide
/-- Dralnu, Lich Lord -/
def dralnuLichLord : Instruction :=
  gain (target (.and [instantOrSorcery, .inZone (graveyardOf .you)]))
    (keywordCosting "Flashback" .itsManaCost) (some untilEndOfTurn)
theorem okDralnuLichLord : Instruction.check [] dralnuLichLord = [] := by decide
/-- Dregscape Zombie -/
def dregscapeZombie : Ability := keywordCosting "Unearth" (.mana [pip .black])
theorem okDregscapeZombie : Ability.check [] dregscapeZombie = [] := by decide
/-- Think Twice -/
def thinkTwice : Ability := keywordCosting "Flashback" (.mana [generic 2, pip .blue])
theorem okThinkTwice : Ability.check [] thinkTwice = [] := by decide
/-- Greater Mossdog -/
def greaterMossdog : Ability := keywordNumber "Dredge" (.lit 3)
theorem okGreaterMossdog : Ability.check [] greaterMossdog = [] := by decide
/-- Raven's Crime -/
def ravensCrime : Ability := keyword "Retrace"
theorem okRavensCrime : Ability.check [] ravensCrime = [] := by decide
/-- Barkhide Mauler -/
def barkhideMauler : Ability := keywordCosting "Cycling" (.mana [generic 2])
theorem okBarkhideMauler : Ability.check [] barkhideMauler = [] := by decide
/-- Ninja of the New Moon -/
def ninjaOfTheNewMoon : Ability := keywordCosting "Ninjutsu" (.mana [generic 3, pip .black])
theorem okNinjaOfTheNewMoon : Ability.check [] ninjaOfTheNewMoon = [] := by decide
/-- Thunderous Wrath -/
def thunderousWrath : Ability := keywordCosting "Miracle" (.mana [pip .red])
theorem okThunderousWrath : Ability.check [] thunderousWrath = [] := by decide
/-- Bygone Colossus -/
def bygoneColossus : Ability := keywordCosting "Warp" (.mana [generic 3])
theorem okBygoneColossus : Ability.check [] bygoneColossus = [] := by decide
/-- Knight of Grace — "Hexproof from black" [CR#702.11d] -/
def knightOfGraceHexproofFromBlack : Ability := keywordQuality "Hexproof" (.colorIs .black)
theorem okKnightOfGraceHexproofFromBlack :
    Ability.check [] knightOfGraceHexproofFromBlack = [] := by decide
/-- Eternal Dragon — "Plainscycling {2}" [CR#702.29e] -/
def eternalDragonPlainscycling : Ability :=
  keywordQualityCosting "Cycling" (.hasSubtype (landType "Plains")) (.mana [generic 2])
theorem okEternalDragonPlainscycling : Ability.check [] eternalDragonPlainscycling = [] := by decide
/-- Nezumi Ronin -/
def nezumiRonin : Ability := keywordNumber "Bushido" (.lit 1)
theorem okNezumiRonin : Ability.check [] nezumiRonin = [] := by decide
/-- Dragonlord Ojutai -/
def dragonlordOjutaiHexproof : Ability :=
  .static (onlyWhile (.abilityGrant thisCreature (keyword "Hexproof")) (.matches it untapped))
theorem okDragonlordOjutaiHexproof : Ability.check [] dragonlordOjutaiHexproof = [] := by decide
/-- Monoxa, Midway Manager -/
def monoxaRollTrigger : Ability :=
  whenever (youRollResultIn (atLeast 3))
    (.sequence
      [ gain thisCreature (keyword "FirstStrike") (some untilEndOfTurn),
        doIf (.compareAmt (.theOutcome .rollResult) .atLeast (.lit 4))
          (gain it (keyword "Menace") (some untilEndOfTurn)),
        doIf (.compareAmt (.theOutcome .rollResult) .atLeast (.lit 5))
          (gain it (keyword "Lifelink") (some untilEndOfTurn)) ])
theorem okMonoxaRollTrigger : Ability.check [] monoxaRollTrigger = [] := by decide
/-- Celebr-8000 -/
def celebr8000Doubles : Instruction :=
  .sequence
    [ rollDice 2 6 (agent := .you),
      doIf .rolledDoubles (gain thisCreature (keyword "DoubleStrike") (some untilEndOfTurn)) ]
theorem okCelebr8000Doubles : Instruction.check [] celebr8000Doubles = [] := by decide

/-- Krosan Druid -/
def krosanDruid : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Krosan Druid", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [creatureType "Centaur", creatureType "Druid"],
      text :=
        [ keywordCosting "Kicker" (.mana [generic 4, pip .green]),
          triggeredIf (.enters thisCreature none) (costWasPaid (.byKeyword "Kicker") none thisCreature)
            (gainLife (.lit 10) (agent := .you)) ],
      power := stat 2, toughness := stat 3 } }

/-- Lightkeeper of Emeria -/
def lightkeeperOfEmeria : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lightkeeper of Emeria", cost := some [generic 3, pip .white], types := [.creature],
      subtypes := [creatureType "Angel"],
      text :=
        [ keywordCosting "Multikicker" (.mana [pip .white]),
          keyword "Flying",
          when (.enters thisCreature none)
            (gainLife (times (.lit 2) (timesPaid (.byKeyword "Kicker") thisCreature)) (agent :=
                .you)) ],
      power := stat 2, toughness := stat 4 } }

/-- Merfolk Falconer -/
def merfolkFalconer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Merfolk Falconer", cost := some [generic 3, pip .blue, pip .blue], types := [.creature],
      subtypes := [creatureType "Merfolk", creatureType "Wizard"],
      text :=
        [ keyword "Flying",
          whenever
            (.casts .you
              (a (.compareOver spell (paidCostRead (.byKeyword "Kicker") none it) .atLeast (.lit 1)))
              none)
            (scry (.lit 2) (agent := .you)) ],
      power := stat 4, toughness := stat 4 } }

/-- Borrowed Malevolence -/
def borrowedMalevolence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Borrowed Malevolence", cost := some [pip .black], types := [.instant],
      text :=
        [ keywordCosting "Escalate" (.mana [generic 2]),
          .spell none (chooseModes (.range (some 1) (some 2))
            [ get (target creature) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn),
              get (target creature) (.down (.lit 1)) (.down (.lit 1)) (some untilEndOfTurn) ]) ] } }

/-- New Perspectives -/
def newPerspectivesCyclingAltCost : Ability :=
  .static (.altCost (allOf (.abilityHead (.keyword "Cycling"))) (some (.mana [generic 0])))
theorem okNewPerspectivesCyclingAltCost :
    Ability.check [] newPerspectivesCyclingAltCost = [] := by decide
/-- Thick-Skinned Goblin -/
def thickSkinnedGoblinEchoAltCost : Ability :=
  .static (.altCost
    (allOf (.and [ .abilityHead (.keyword "Echo"),
                   .abilityOf (allOf (.and [permanent, .hasPossessor .controller .you])) ]))
    (some (.mana [generic 0])))
theorem okThickSkinnedGoblinEchoAltCost :
    Ability.check [] thickSkinnedGoblinEchoAltCost = [] := by decide
/-- Fumiko the Lowblood -/
def fumikoBushidoX : Ability :=
  .static (.conjunction none
    [ .abilityGrant thisCreature (keywordNumber "Bushido" (.letter .x)),
      .letterDefinition .x (countOf attacking) ])
theorem okFumikoBushidoX : Ability.check [] fumikoBushidoX = [] := by decide

/-- Rafter Demon -/
def rafterDemon : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rafter Demon", cost := some [generic 2, pip .black, pip .red], types := [.creature],
      subtypes := [creatureType "Demon"],
      text :=
        [ keywordCosting "Spectacle" (.mana [generic 3, pip .black, pip .red]),
          triggeredIf (.enters thisCreature none)
            (costWasPaid (.byKeyword "Spectacle") none thisCreature)
            (discard (a (.inZone hand)) (agent := (each .opponent))) ],
      power := stat 4, toughness := stat 2 } }

/-- Tourach, Dread Cantor -/
def tourachDiscardTrigger : Ability :=
  whenever (.verbedEvent (some anOpponent) (.action "Discard") (some (a .isCard)) none)
    (.putCounters (.lit 1) (.printed plusOnePlusOne) thisCreature)
theorem okTourachDiscardTrigger : Ability.check [] tourachDiscardTrigger = [] := by decide

/-- Shimmering Glasskite -/
def shimmeringGlasskite : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shimmering Glasskite", cost := some [generic 3, pip .blue], types := [.creature],
      subtypes := [creatureType "Spirit"],
      text :=
        [ keyword "Flying",
          triggeredOnlyOnce
            (.becomesTarget thisCreature (a (.or [spell, .abilityHead .anyOnStack]))) .oncePerTurn
            (.counterSpell (that .stack)) ],
      power := stat 2, toughness := stat 3 } }

/-- Conqueror's Pledge -/
def conquerorsPledge : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Conqueror's Pledge", cost := some [generic 2, pip .white, pip .white, pip .white],
      types := [.sorcery],
      text :=
        [ keywordCosting "Kicker" (.mana [generic 6]),
          .spell none (.replace
            (create (.lit 6) (creatureToken 1 1 [.white] [creatureType "Kor", creatureType "Soldier"]))
            (.doIf (costWasPaid (.byKeyword "Kicker") none .this)
              (.create (.lit 12) .asThose [] (agent := .you)) none)) ] } }

/-- Soul of Emancipation -/
def soulOfEmancipation : Ability :=
  when (.enters thisCreature none)
    (.sequence
      [ destroy (.described (.target (upTo 3)) (.and [permanent, .not land, .otherThan .this])),
        .doForEach (those .permanent)
          (.create (.lit 1)
            (.written
              { characteristics :=
                { colors := [.white], types := [.creature], subtypes := [creatureType "Angel"],
                  text := [keyword "Flying"], power := stat 3, toughness := stat 3 } })
            [] (agent := (controllerOf (that .permanent)))) ])
theorem okSoulOfEmancipation : Ability.check [] soulOfEmancipation = [] := by decide

/-- Tourach, Dread Cantor -/
def tourachDreadCantor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tourach, Dread Cantor", cost := some [generic 1, pip .black],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Cleric"],
      text :=
        [ keywordCosting "Kicker" (.mana [pip .black, pip .black]),
          keywordQuality "Protection" (.colorIs .white),
          tourachDiscardTrigger,
          triggeredIf (.enters thisCreature none) (costWasPaid (.byKeyword "Kicker") none thisCreature)
            (discard (countedAtRandom (exactly 2) (.inZone hand)) (agent := (target .opponent))) ],
      power := stat 2, toughness := stat 1 } }

/-- Talrand's Invocation -/
def talrandsInvocation : Instruction :=
  create (.lit 2)
    { characteristics :=
      { colors := [.blue], types := [.creature], subtypes := [creatureType "Drake"],
        text := [keyword "Flying"], power := stat 2, toughness := stat 2 } }
theorem okTalrandsInvocation : Instruction.check [] talrandsInvocation = [] := by decide

/-- Predatory Wurm -/
def predatoryWurm : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Predatory Wurm", cost := some [generic 3, pip .green], types := [.creature],
      subtypes := [creatureType "Wurm"],
      text :=
        [ keyword "Vigilance",
          .static (onlyWhile (getsPt thisCreature (.up (.lit 2)) (.up (.lit 2)))
            (exists_ (.and [.hasSubtype (planeswalkerType "Garruk"), .hasPossessor .controller .you]))) ],
      power := stat 4, toughness := stat 4 } }

/-- Tower Winder -/
def towerWinder : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tower Winder", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [creatureType "Snake"],
      text :=
        [ keyword "Reach", keyword "Deathtouch",
          when (.enters thisCreature none)
            (.sequence
              [ searchLibraryOrGraveyard (.named (.printed "Command Tower")),
                revealIt, move foundCard hand,
                .doIf (happenedAt (.verbedAct (.action "Search")) .you .thisWay yourLibrary) shuffle
                  none ]) ],
      power := stat 1, toughness := stat 1 } }

/-- Aim High -/
def aimHigh : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aim High", cost := some [generic 1, pip .green], types := [.instant],
      text :=
        [ .spell none (.sequence
            [ untap (target creature),
              .establish
                (.conjunction none
                  [ .modification (itVerbed (.action "Untap")) .power (.up (.lit 2)),
                    .modification (itVerbed (.action "Untap")) .toughness (.up (.lit 2)),
                    .abilityGrant (itVerbed (.action "Untap")) (keyword "Reach") ])
                (some untilEndOfTurn) ]) ] } }

/-- Harried Dronesmith -/
def harriedDronesmithToken : Instruction :=
  .sequence [create (.lit 1) thopterToken, gainHaste itAsToken (some untilEndOfTurn)]
theorem okHarriedDronesmithToken : Instruction.check [] harriedDronesmithToken = [] := by decide

/-- Archfiend's Vessel -/
def archfiendsVessel : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Archfiend's Vessel", cost := some [pip .black], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Cleric"],
      text :=
        [ keyword "Lifelink",
          triggeredIf (.enters thisCreature none)
            (.or
              [ .happened it (.mk .entry .triggering (some (.fromZones (.zones [graveyardOf .you]) none))),
                .matches it (.and [castBy .you, .castFrom (graveyardOf .you)]) ])
            (.triggerReflexively (exile it)
              (create (.lit 1)
                { characteristics :=
                  { colors := [.black], types := [.creature], subtypes := [creatureType "Demon"],
                    text := [keyword "Flying"], power := stat 5, toughness := stat 5 } })) ],
      power := stat 1, toughness := stat 1 } }

/-- Containment Priest -/
def containmentPriest : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Containment Priest", cost := some [generic 1, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Cleric"],
      text :=
        [ keyword "Flash",
          .static (.replacement (.enters (a (.and [creature, nontoken, .not .wasCast])) none) []
              none
            (exile it) .repeatedly none) ],
      power := stat 2, toughness := stat 2 } }

/-- Veiling Oddity -/
def veilingOddityLine : Ability :=
  triggeredWhile (lastCounterRemoved (.named "Time") .this)
    (.whileTrue (.matches .this (.inZone exileZone)))
    (.establish (deontic (allOf creature) .forbid [.core .block] .patient .noPatient)
      (some .thisTurn))
theorem okVeilingOddityLine : Ability.check [] veilingOddityLine = [] := by decide

def wizenedSnitches : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Wizened Snitches", cost := some [generic 3, pip .blue], types := [.creature],
      subtypes := [creatureType "Faerie", creatureType "Rogue"],
      text := [keyword "Flying", .static (.visibility .reveal (.playerGroup .allPlayers) .topOfLibrary)],
      power := stat 1, toughness := stat 3 } }

/-- Darkblade Agent -/
def darkbladeAgentGrants : Ability :=
  .static (.conditional
    (.conjunction none
      [ .abilityGrant thisCreature (keyword "Deathtouch"),
        .abilityGrant thisCreature
          (whenever (dealsCombatDamage thisCreature (a .anyPlayer)) (.draw (.lit 1) (agent :=
              .you))) ])
    (happened (.verbedAct (.action "Surveil")) .you .thisTurn) .asLongAs)
theorem okDarkbladeAgentGrants : Ability.check [] darkbladeAgentGrants = [] := by decide
/-- Frenzied Gorespawn -/
def frenziedGorespawnMenaceTrigger : Ability :=
  .triggered (.combat .attackerOf (counted (atLeast 1) creature) (some anOpponent)) [] none [] none
    none none (gain (those (.type .creature)) (keyword "Menace") (some untilEndOfTurn))
theorem okFrenziedGorespawnMenaceTrigger :
    Ability.check [] frenziedGorespawnMenaceTrigger = [] := by decide
/-- Pir, Imaginative Rascal's replacement -/
def pirDistributive : Ability :=
  .static (.replacement
    (bareCounterEvent .put .many
      (a (.and [permanent, .hasPossessor .controller (.playerGroup .yourTeam)])))
    [] none (.putCounters (plus .thatMuch (.lit 1)) .those (that .permanent)) .repeatedly none)
theorem okPirDistributive : Ability.check [] pirDistributive = [] := by decide

/-- Frogmite -/
def frogmite : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Frogmite", cost := some [generic 4], types := [.artifact, .creature],
      subtypes := [creatureType "Frog"], text := [keywordQuality "Affinity" artifact],
      power := stat 2, toughness := stat 2 } }

/-- Iymrith, Desert Doom -/
def iymrithDesertDoom : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Iymrith, Desert Doom", cost := some [generic 3, pip .blue, pip .blue],
      supertypes := [.legendary], types := [.creature], subtypes := [creatureType "Dragon"],
      text :=
        [ keyword "Flying",
          .static (onlyWhile (.abilityGrant thisCreature (keywordCosting "Ward" (.mana [generic
              4])))
            (.matches it untapped)),
          whenever (dealsCombatDamage thisCreature (a .anyPlayer))
            (.sequence
              [ .draw (.lit 1) (agent := .you),
                .doIf (.compareAmt (countOf (.inZone (handOf .you))) .less (.lit 3))
                  (.draw .theDifference (agent := .you)) none ]) ],
      power := stat 5, toughness := stat 5 } }

def protectionAbilities : KeywordTerm := .anyIn ⟨"Protection", none⟩
def landwalkAbilities : KeywordTerm := .anyIn ⟨"Landwalk", none⟩
/-- Shay Cormac -/
def wardAbilities : KeywordTerm := .anyIn ⟨"Ward", none⟩
/-- Tolaria, Shelkin Brownie -/
def bandsWithOtherAbilities : KeywordTerm := .anyIn ⟨"BandsWithOther", none⟩
def protectionFromAnyColor : KeywordTerm := .anyIn ⟨"Protection", some .color⟩

/-- Cairn Wanderer -/
def cairnWanderer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cairn Wanderer", cost := some [generic 4, pip .black], types := [.creature],
      subtypes := [creatureType "Shapeshifter"],
      text :=
        [ keyword "Changeling",
          .alsoForKeywords
            (.static (onlyWhile (.abilityGrant thisCreature (keyword "Flying"))
              (exists_ (.and [creature, .inZone graveyard, .hasKeyword (.the "Flying")]))))
            [ .the "Fear", .the "FirstStrike", .the "DoubleStrike", .the "Deathtouch", .the "Haste",
              landwalkAbilities, .the "Lifelink", protectionAbilities, .the "Reach", .the "Trample",
              .the "Shroud", .the "Vigilance" ] ],
      power := stat 4, toughness := stat 4 } }

/-- Autarch Mammoth -/
def autarchMammoth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Autarch Mammoth", cost := some [generic 4, pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Elephant", creatureType "Mount"],
      text := [autarchMammothLine, keywordNumber "Saddle" (.lit 5)],
      power := stat 5, toughness := stat 5 } }

/-- Debris Beetle -/
def debrisBeetle : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Debris Beetle", cost := some [generic 2, pip .black, pip .green], types := [.artifact],
      subtypes := [artifactType "Vehicle"],
      text :=
        [ keyword "Trample",
          when (.enters thisVehicle none)
            (.sequence [loseLife (.lit 3) (agent := (each .opponent)), gainLife (.lit 3) (agent :=
                .you)]),
          keywordNumber "Crew" (.lit 2) ],
      power := stat 6, toughness := stat 6 } }

/-- Pir, Imaginative Rascal -/
def pirImaginativeRascal : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Pir, Imaginative Rascal", cost := some [generic 2, pip .green],
      supertypes := [.legendary], types := [.creature], subtypes := [creatureType "Human"],
      text :=
        [ keywordQuality "PartnerWith" (.named (.printed "Toothy, Imaginary Friend")),
          pirDistributive ],
      power := stat 1, toughness := stat 1 } }

/-- Market Gnome -/
def marketGnome : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Market Gnome", cost := some [pip .white], types := [.artifact, .creature],
      subtypes := [creatureType "Gnome"],
      text :=
        [ when (.dies thisCreature) (.sequence [gainLife (.lit 1) (agent := .you), .draw (.lit 1)
            (agent := .you)]),
          triggeredWhile (.verbedEvent none (.action "Exile") (some thisCreature) none)
            (.whileDoing (.activates .you (a (.abilityHead (.keyword "Craft")))))
            (.sequence [gainLife (.lit 1) (agent := .you), .draw (.lit 1) (agent := .you)]) ],
      power := stat 0, toughness := stat 3 } }

/-- Escaped Shapeshifter -/
def escapedShapeshifter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Escaped Shapeshifter", cost := some [generic 3, pip .blue, pip .blue],
      types := [.creature], subtypes := [creatureType "Shapeshifter"],
      text :=
        [ .alsoForKeywords
            (.static (onlyWhile (.abilityGrant thisCreature (keyword "Flying"))
              (exists_ (.and [ creature, .hasPossessor .controller anOpponent, .hasKeyword (.the "Flying"),
                               .not (.named (.printed "Escaped Shapeshifter")) ]))))
            [.the "FirstStrike", .the "Trample", protectionFromAnyColor] ],
      power := stat 3, toughness := stat 4 } }

/-- Ancestral Blade -/
def ancestralBlade : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ancestral Blade", cost := some [generic 1, pip .white], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ when (.enters thisEquipment none)
            (.sequence
              [ create (.lit 1) (creatureToken 1 1 [.white] [creatureType "Soldier"]),
                .attachTo thisEquipment itAsToken ]),
          .static (getsPt (.attachHost .equipped (.type .creature)) (.up (.lit 1)) (.up (.lit 1))),
          keywordCosting "Equip" (.mana [generic 1]) ] } }

/-- Steelclaw Lance -/
def steelclawLance : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Steelclaw Lance", cost := some [pip .black, pip .red], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ .static (getsPt (.attachHost .equipped (.type .creature)) (.up (.lit 2)) (.up (.lit 2))),
          keywordQualityCosting "Equip" (.hasSubtype (creatureType "Knight")) (.mana [generic 1]),
          keywordCosting "Equip" (.mana [generic 3]) ] } }

/-- Commander's Plate's equip lines -/
def commandersPlateEquip : List Ability :=
  [ keywordQualityCosting "Equip" (.hasDesignation "commander" none) (.mana [generic 3]),
    keywordCosting "Equip" (.mana [generic 5]) ]
theorem okCommandersPlateEquip : Ability.checkText [] commandersPlateEquip = [] := by decide
/-- Luxior, Giada's Gift's equip lines -/
def luxiorEquipLines : List Ability :=
  [ keywordQualityCosting "Equip" (.hasType .planeswalker) (.mana [generic 1]),
    keywordCosting "Equip" (.mana [generic 3]) ]
theorem okLuxiorEquipLines : Ability.checkText [] luxiorEquipLines = [] := by decide

/-- Veiling Oddity -/
def veilingOddity : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Veiling Oddity", cost := some [generic 3, pip .blue], types := [.creature],
      subtypes := [creatureType "Illusion"],
      text := [keywordNumberCosting "Suspend" (.lit 4) (.mana [generic 1, pip .blue]), veilingOddityLine],
      power := stat 2, toughness := stat 3 } }

/-- Enormous Energy Blade -/
def enormousEnergyBlade : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Enormous Energy Blade", cost := some [generic 2, pip .black], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ .static (getsPt (.attachHost .equipped (.type .creature)) (.up (.lit 4)) (.up (.lit 0))),
          whenever (.attachment .attached thisEquipment (a creature)) (tap (that (.type .creature))),
          keywordCosting "Equip" (.mana [generic 2]) ] } }

/-- Grafted Wargear -/
def graftedWargear : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Grafted Wargear", cost := some [generic 3], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ .static (getsPt (.attachHost .equipped (.type .creature)) (.up (.lit 3)) (.up (.lit 2))),
          whenever (.attachment .unattached thisEquipment (a permanent))
            (sacrifice (that .permanent) (agent := .you)),
          keywordCosting "Equip" (.mana [generic 0]) ] } }

/-- Black Ward -/
def blackWard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Black Ward", cost := some [pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          .static (.retention
            (.abilityGrant (.attachHost .enchanted (.type .creature)) (keywordQuality "Protection"
                (.colorIs .black)))
            thisAura) ] } }

/-- Cho-Manno's Blessing -/
def choMannosBlessing : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cho-Manno's Blessing", cost := some [pip .white, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keyword "Flash",
          keywordSubject "Enchant" creature,
          .static (entersChoosing thisAura .color),
          .static (.retention
            (.abilityGrant (.attachHost .enchanted (.type .creature)) (keywordQuality "Protection"
                (ofChosen .color)))
            thisAura) ] } }

/-- Pentarch Ward -/
def pentarchWard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Pentarch Ward", cost := some [generic 2, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          .static (entersChoosing thisAura .color),
          when (.enters thisAura none) (.draw (.lit 1) (agent := .you)),
          .static (.retention
            (.abilityGrant (.attachHost .enchanted (.type .creature)) (keywordQuality "Protection"
                (ofChosen .color)))
            thisAura) ] } }

/-- Benevolent Blessing -/
def benevolentBlessing : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Benevolent Blessing", cost := some [generic 1, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keyword "Flash",
          keywordSubject "Enchant" creature,
          .static (entersChoosing thisAura .color),
          .static (.retention
            (.abilityGrant (.attachHost .enchanted (.type .creature)) (keywordQuality "Protection"
                (ofChosen .color)))
            (allOf (.and [ .or [.hasSubtype (enchantmentType "Aura"), .hasSubtype (artifactType "Equipment")],
                           .hasPossessor .controller .you, .attachedTo it ]))) ] } }

/-- Wall of Shards -/
def wallOfShards : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Wall of Shards", cost := some [generic 1, pip .white], supertypes := [.snow],
      types := [.creature], subtypes := [creatureType "Wall"],
      text :=
        [ keyword "Defender", keyword "Flying",
          cumulativeUpkeep (.perform (gainLife (.lit 1) (agent := anOpponent))) ],
      power := stat 1, toughness := stat 8 } }

/-- Earthen Goo -/
def earthenGoo : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Earthen Goo", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [creatureType "Ooze"],
      text :=
        [ keyword "Trample",
          cumulativeUpkeep (.either (.mana [pip .red]) (.mana [pip .green])),
          .static (getsPt thisCreature
            (.up (times (.lit 1) (countersOn (.named "Age") it)))
            (.up (times (.lit 1) (countersOn (.named "Age") it)))) ],
      power := stat 2, toughness := stat 2 } }

/-- Mutagen Connoisseur -/
def mutagenConnoisseur : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mutagen Connoisseur", cost := some [generic 1, pip .green, pip .blue],
      types := [.creature], subtypes := [creatureType "Vedalken", creatureType "Mutant"],
      text :=
        [ keyword "Flying", keyword "Vigilance",
          .static (getsPt thisCreature
            (.up (forEach 1 (.and [.isTransformed, .hasPossessor .controller .you]))) (.up (.lit 0))) ],
      power := stat 0, toughness := stat 5 } }

/-- Bedrock Tortoise -/
def bedrockTortoiseWindow : StaticSpec :=
  .partScope .turn (some .you) (.abilityGrant (allOf creatureYouControl) (keyword "Hexproof"))
theorem okBedrockTortoiseWindow : StaticSpec.check [] bedrockTortoiseWindow = [] := by decide

/-- Battlegate Mimic -/
def battlegateMimic : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Battlegate Mimic", cost := some [generic 1, hybridPip .red .white], types := [.creature],
      subtypes := [creatureType "Shapeshifter"],
      text :=
        [ whenever (.casts .you (a (.and [spell, .colorIs .red, .colorIs .white])) none)
            (.establish
              (.conjunction none
                [ .modification thisCreature .power (.set (.lit 4)),
                  .modification (itsOther thisCreature (.set (.lit 4))) .toughness (.set (.lit 2)),
                  .abilityGrant thisCreature (keyword "FirstStrike") ])
              (some untilEndOfTurn)) ],
      power := stat 2, toughness := stat 1 } }

/-- Sphinx of Uthuun -/
def sphinxOfUthuun : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sphinx of Uthuun", cost := some [generic 5, pip .blue, pip .blue], types := [.creature],
      subtypes := [creatureType "Sphinx"],
      text :=
        [ keyword "Flying",
          when (.enters thisCreature none)
            (.sequence
              [ revealCards (topSlice (.lit 5)),
                .separateIntoPiles them 2 [] (agent := anOpponent),
                move onePile hand,
                move (theOther .pile) graveyard ]) ],
      power := stat 5, toughness := stat 6 } }

/-- Marit Lage -/
def maritLage : CharacteristicBundle :=
  { characteristics :=
    { name := "Marit Lage", colors := [.black], supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Avatar"], text := [keyword "Flying", keyword "Indestructible"],
      power := stat 20, toughness := stat 20 } }

/-- Tuktuk the Explorer -/
def tuktukTheExplorer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tuktuk the Explorer", cost := some [generic 2, pip .red], supertypes := [.legendary],
      types := [.creature], subtypes := [creatureType "Goblin"],
      text :=
        [ keyword "Haste",
          when (.dies thisCreature)
            (create (.lit 1)
              { characteristics :=
                { name := "Tuktuk the Returned", supertypes := [.legendary],
                  types := [.artifact, .creature],
                  subtypes := [creatureType "Goblin", creatureType "Golem"],
                  power := stat 5, toughness := stat 5 } }) ],
      power := stat 1, toughness := stat 1 } }

/-- Artificer's Assistant -/
def artificersAssistant : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Artificer's Assistant", cost := some [pip .blue], types := [.creature],
      subtypes := [creatureType "Bird"],
      text :=
        [ keyword "Flying",
          whenever (.casts .you (a (.and [historic, spell])) none) (scry (.lit 1) (agent := .you))
              ],
      power := stat 1, toughness := stat 1 } }

/-- Aya of Alexandria -/
def ayaOfAlexandria : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aya of Alexandria", cost := some [generic 2, pip .red, pip .white],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Assassin"],
      text :=
        [ keyword "Menace", keyword "Lifelink",
          whenever
            (dealsCombatDamage (a (.and [historic, creature, .hasPossessor .controller .you])) (a .anyPlayer))
            (create (.lit 1)
              { characteristics :=
                { colors := [.black], types := [.creature], subtypes := [creatureType "Assassin"],
                  text := [keyword "Menace"], power := stat 1, toughness := stat 1 } }) ],
      power := stat 4, toughness := stat 3 } }

/-- Indestructibility -/
def indestructibility : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Indestructibility", cost := some [generic 3, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" permanent,
          .static (.abilityGrant (.attachHost .enchanted .permanent) (keyword "Indestructible")) ] }
              }

/-- Seraphic Greatsword -/
def seraphicGreatsword : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Seraphic Greatsword", cost := some [generic 1, pip .white], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ .static (getsPt (.attachHost .equipped (.type .creature)) (.up (.lit 2)) (.up (.lit 2))),
          whenever
            (attacksPlayer (.attachHost .equipped (.type .creature))
              (the (.and [.anyPlayer, .superlative .max (.playerStat .lifeTotal) .anyPlayer])))
            (.create (.lit 1)
              (.written
                { characteristics :=
                  { colors := [.white], types := [.creature], subtypes := [creatureType "Angel"],
                    text := [keyword "Flying"], power := stat 4, toughness := stat 4 } })
              [.entersAs .tapped, .entersAttacking (some (that .player))] (agent := .you)),
          keywordCosting "Equip" (.mana [generic 4]) ] } }

def giantGrowth : Instruction :=
  get (target creature) (.up (.lit 3)) (.up (.lit 3)) (some untilEndOfTurn)
theorem okGiantGrowth : Instruction.check [] giantGrowth = [] := by decide
def amassZombiesTwo : Instruction := amass "Zombie" 2
theorem okAmassZombiesTwo : Instruction.check [] amassZombiesTwo = [] := by decide
/-- Avarice Totem -/
def avariceTotemExchange : Instruction :=
  .exchange (.controlOf thisArtifact (target (.and [permanent, .not land])))
theorem okAvariceTotemExchange : Instruction.check [] avariceTotemExchange = [] := by decide
/-- Arcanum Wings -- aura swap [CR#702.65a] -/
def arcanumWingsAuraSwap : Ability :=
  activated (.mana [generic 2, pip .blue])
    (offer
      (.exchange (.cardsAcross thisAura (a (.and [.hasSubtype (enchantmentType "Aura"), .inZone
          hand])))) (agent := .you))
theorem okArcanumWingsAuraSwap : Ability.check [] arcanumWingsAuraSwap = [] := by decide
/-- Tovolar, Dire Overlord -/
def tovolarNightfall : Instruction := .setGameDesignation "night"
theorem okTovolarNightfall : Instruction.check [] tovolarNightfall = [] := by decide
/-- Spin into Myth -/
def spinIntoMyth : Instruction :=
  .sequence [move (target creature) onTop, fateseal anOpponent (.lit 2) (agent := .you)]
theorem okSpinIntoMyth : Instruction.check [] spinIntoMyth = [] := by decide
/-- Pure // Simple -/
def pureHalf : Instruction := destroy (target (.and [permanent, multicolored]))
theorem okPureHalf : Instruction.check [] pureHalf = [] := by decide
/-- Korlash -/
def grandeurDiscardCost : Cost :=
  .perform (discard
    (a (.and [.named (.printed "Korlash, Heir to Blackblade"), .otherThan .this, .inZone hand]))
        (agent := .you))
theorem okGrandeurDiscardCost : Cost.check [] grandeurDiscardCost = [] := by decide
/-- Gyruda, Doom of Depths -/
def gyrudaCompanion : Ability := companion (.everyCardIs .isCard (.manaValueParity .even))
theorem okGyrudaCompanion : Ability.check [] gyrudaCompanion = [] := by decide
/-- Jegantha, the Wellspring -/
def jeganthaCompanion : Ability := companion (.noCardIs .isCard .repeatedManaSymbol)
theorem okJeganthaCompanion : Ability.check [] jeganthaCompanion = [] := by decide
/-- Kaheera, the Orphanguard -/
def kaheeraCompanion : Ability :=
  companion
    (.everyCardIs (.and [creature, .isCard])
      (.anyOf [ .aCharacteristic (.hasSubtype (creatureType "Cat")),
                .aCharacteristic (.hasSubtype (creatureType "Elemental")),
                .aCharacteristic (.hasSubtype (creatureType "Nightmare")),
                .aCharacteristic (.hasSubtype (creatureType "Dinosaur")),
                .aCharacteristic (.hasSubtype (creatureType "Beast")) ]))
theorem okKaheeraCompanion : Ability.check [] kaheeraCompanion = [] := by decide
/-- Keruga, the Macrosage -/
def kerugaCompanion : Ability :=
  companion
    (.everyCardIs .isCard
      (.anyOf [ .aCharacteristic (.compare [.stat .manaValue] .atLeast (.lit 3)),
                .aCharacteristic land ]))
theorem okKerugaCompanion : Ability.check [] kerugaCompanion = [] := by decide
/-- Lurrus of the Dream-Den -/
def lurrusCompanion : Ability :=
  companion
    (.everyCardIs permanentCard (.aCharacteristic (.compare [.stat .manaValue] .atMost (.lit 2))))
theorem okLurrusCompanion : Ability.check [] lurrusCompanion = [] := by decide
/-- Lutri, the Spellchaser -/
def lutriCompanion : Ability := companion (.cardsDiffer (.and [.not land, .isCard]) .cardName)
theorem okLutriCompanion : Ability.check [] lutriCompanion = [] := by decide
/-- Obosh, the Preypiercer -/
def oboshCompanion : Ability :=
  companion (.everyCardIs .isCard (.anyOf [.manaValueParity .odd, .aCharacteristic land]))
theorem okOboshCompanion : Ability.check [] oboshCompanion = [] := by decide
/-- Umori, the Collector -/
def umoriCompanion : Ability := companion (.cardsShare (.and [.not land, .isCard]) .cardType)
theorem okUmoriCompanion : Ability.check [] umoriCompanion = [] := by decide
/-- Yorion, Sky Nomad -/
def yorionCompanion : Ability := companion (.deckSizeOverMinimum 20)
theorem okYorionCompanion : Ability.check [] yorionCompanion = [] := by decide
/-- Zirda, the Dawnwaker -/
def zirdaCompanion : Ability :=
  companion (.everyCardIs permanentCard (.hasAbilityOf .anyActivated))
theorem okZirdaCompanion : Ability.check [] zirdaCompanion = [] := by decide

/-- Deadpool, Trading Card -/
def deadpoolTradingCard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Deadpool, Trading Card", cost := some [generic 2, pip .black, pip .red],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Mutant", creatureType "Mercenary", creatureType "Hero"],
      text :=
        [ .static (.replacement (.enters thisCreature none) [] none
            (offer (.exchange (.textBoxes thisCreature (a (.and [creature, .otherThan .this]))))
                (agent := .you))
            .nextTimeOnly none),
          at_ (.beginningOf .the .upkeep (.byPlayer .you)) (loseLife (.lit 3) (agent := .you)),
          activated (.compound [.mana [generic 3], .perform (sacrifice thisCreature (agent :=
              .you))])
            (.draw (.lit 1) (agent := (each otherPlayer))) ],
      power := stat 5, toughness := stat 3 } }

end Semantics.Cards

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
  Primitives.Instruction.sequentially
    [ choose (a (qualityFrom .ability
        (Primitives.ChoiceDomain.abilitiesAmong [.the "Flying", .the "FirstStrike", .the "Trample", .theWith "Rampage" 3]))),
      gain thisCreature (Primitives.Ability.thatAbility .theChoice) (some (Primitives.Duration.until_ (Primitives.DurationEnd.startOf .upkeep (some Primitives.NounPhrase.you)))) ]
theorem okGabrielAngelfire : Instruction.check [] gabrielAngelfire = [] := by decide
def builtToSmash : Instruction :=
  Primitives.Instruction.sequentially
    [ get (target (Primitives.Predicate.and [creature, attacking])) (Primitives.Delta.up (.lit 3)) (Primitives.Delta.up (.lit 3)) (some untilEndOfTurn),
      Primitives.Instruction.doIf (itsA (Primitives.Predicate.and [artifact, creature]))
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
  Primitives.Instruction.sequentially
    [ gain (target creature) (keyword "Vigilance") (some untilEndOfTurn),
      Primitives.Instruction.establish (deontic (that (.type .creature)) Primitives.Compulsion.forbid [.core .block] .patient Primitives.DeonticPatient.noPatient)
        (some Primitives.Duration.thisTurn) ]
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
  Primitives.Instruction.create (.lit 1)
    (Primitives.TokenSpec.written
      { characteristics :=
        { colors := [.red], types := [.creature], subtypes := [creatureType "Dragon"],
          text := [keyword "Flying"], power := stat 5, toughness := stat 5 } })
    [] (agent := (each (Primitives.Predicate.and [Primitives.Predicate.anyPlayer, Primitives.Predicate.otherThan (target Primitives.Predicate.anyPlayer)])))
theorem okDeathByDragons : Instruction.check [] deathByDragons = [] := by decide

/-- Concordant Crossroads -/
def concordantCrossroads : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Concordant Crossroads", cost := some [pip .green], supertypes := [.world],
      types := [.enchantment], text := [Primitives.Ability.static (Primitives.StaticSpec.abilityGrant (allOf creature) (keyword "Haste"))]
          } }

/-- Bristlepack Sentry -/
def bristlepackSentry : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bristlepack Sentry", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [creatureType "Plant", creatureType "Wolf"],
      text :=
        [ keyword "Defender",
          Primitives.Ability.static (onlyWhile
            (canDoAsThough thisCreature (.core .attack) (Primitives.Predicate.not (Primitives.Predicate.hasKeyword (.the "Defender"))))
            (exists_ (Primitives.Predicate.and [ creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you,
                             Primitives.Predicate.compare [.stat .power] .atLeast (.lit 4) ]))) ],
      power := stat 3, toughness := stat 3 } }

def platinumAngel : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Platinum Angel", cost := some [generic 7], types := [.artifact, .creature],
      subtypes := [creatureType "Angel"],
      text :=
        [ keyword "Flying",
          Primitives.Ability.static (playerCant (.core .loseGame) Primitives.NounPhrase.you),
          Primitives.Ability.static (playerCant (.core .winGame) (Primitives.NounPhrase.playerGroup .yourOpponents)) ],
      power := stat 4, toughness := stat 4 } }

def abyssalPersecutor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Abyssal Persecutor", cost := some [generic 2, pip .black, pip .black],
      types := [.creature], subtypes := [creatureType "Demon"],
      text :=
        [ keyword "Flying", keyword "Trample",
          Primitives.Ability.static (playerCant (.core .winGame) Primitives.NounPhrase.you),
          Primitives.Ability.static (playerCant (.core .loseGame) (Primitives.NounPhrase.playerGroup .yourOpponents)) ],
      power := stat 6, toughness := stat 6 } }

def smogElemental : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Smog Elemental", cost := some [generic 4, pip .black, pip .black],
      types := [.creature], subtypes := [creatureType "Elemental"],
      text :=
        [ keyword "Flying",
          Primitives.Ability.static (getsPt
            (allOf (Primitives.Predicate.and [ creature, Primitives.Predicate.hasKeyword (.the "Flying"),
                           Primitives.Predicate.hasPossessor .controller (Primitives.NounPhrase.playerGroup .yourOpponents) ]))
            (Primitives.Delta.down (.lit 1)) (Primitives.Delta.down (.lit 1))) ],
      power := stat 3, toughness := stat 3 } }

def exquisiteArchangel : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Exquisite Archangel", cost := some [generic 5, pip .white, pip .white],
      types := [.creature], subtypes := [creatureType "Angel"],
      text :=
        [ keyword "Flying",
          Primitives.Ability.static (Primitives.StaticSpec.replacement (Primitives.GameEvent.losesGame Primitives.NounPhrase.you) [] none
            (Primitives.Instruction.sequentially
              [ exile thisCreature,
                setLife (Primitives.Amount.statOf (.playerStat .startingLifeTotal) Primitives.NounPhrase.you) (agent := Primitives.NounPhrase.you) ])
            .repeatedly none) ],
      power := stat 5, toughness := stat 5 } }

def adelbertSteiner : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Adelbert Steiner", cost := some [generic 1, pip .white], supertypes := [.legendary],
      types := [.creature], subtypes := [creatureType "Human", creatureType "Knight"],
      text :=
        [ keyword "Lifelink",
          Primitives.Ability.static (getsPt thisCreature
            (Primitives.Delta.up (forEach 1 (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (artifactType "Equipment"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))
            (Primitives.Delta.up (forEach 1 (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (artifactType "Equipment"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))) ],
      power := stat 2, toughness := stat 1 } }

def inspiringStatuary : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Inspiring Statuary", cost := some [generic 3], types := [.artifact],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.abilityGrant (allOf (Primitives.Predicate.and [Primitives.Predicate.not artifact, spell, castBy Primitives.NounPhrase.you])) (keyword
            "Improvise")) ] } }

def chiefEngineer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chief Engineer", cost := some [generic 1, pip .blue], types := [.creature],
      subtypes := [creatureType "Vedalken", creatureType "Artificer"],
      text := [Primitives.Ability.static (Primitives.StaticSpec.abilityGrant (allOf (Primitives.Predicate.and [artifact, spell, castBy Primitives.NounPhrase.you])) (keyword
          "Convoke"))],
      power := stat 1, toughness := stat 3 } }

def firesongAndSunspeaker : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.abilityGrant
    (allOf (Primitives.Predicate.and [Primitives.Predicate.colorIs .red, instantOrSorcery, spell, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
    (keyword "Lifelink"))
theorem okFiresongAndSunspeaker : Ability.check [] firesongAndSunspeaker = [] := by decide
def briaRiptideRogue : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.abilityGrant (allOf (otherCreatureYouControl thisCreature)) (keyword "Prowess"))
theorem okBriaRiptideRogue : Ability.check [] briaRiptideRogue = [] := by decide
def narsetEnlightenedExile : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.abilityGrant (allOf creatureYouControl) (keyword "Prowess"))
theorem okNarsetEnlightenedExile : Ability.check [] narsetEnlightenedExile = [] := by decide
def pontiffOfBlight : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.abilityGrant (allOf (otherCreatureYouControl thisCreature)) (keyword "Extort"))
theorem okPontiffOfBlight : Ability.check [] pontiffOfBlight = [] := by decide

def prismariTheInspiration : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Prismari, the Inspiration", cost := some [generic 5, pip .blue, pip .red],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Elder", creatureType "Dragon"],
      text :=
        [ keyword "Flying",
          keywordCosting "Ward" (payLife Primitives.NounPhrase.you 5),
          Primitives.Ability.static (Primitives.StaticSpec.abilityGrant (allOf (Primitives.Predicate.and [instantOrSorcery, spell, castBy Primitives.NounPhrase.you])) storm) ],
      power := stat 7, toughness := stat 7 } }

def tomakulHonorGuard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tomakul Honor Guard", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Soldier"],
      text := [keywordCosting "Ward" (Primitives.Cost.mana [generic 2])], power := stat 3, toughness := stat 1 } }

def repentantBlacksmith : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Repentant Blacksmith", cost := some [generic 1, pip .white], types := [.creature],
      subtypes := [creatureType "Human"],
      text := [keywordQuality "Protection" (Primitives.Predicate.colorIs .red)], power := stat 1, toughness := stat 2 } }

def battleSquadron : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Battle Squadron", cost := some [generic 3, pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Goblin"],
      text :=
        [ keyword "Flying",
          Primitives.Ability.static (Primitives.StaticSpec.ptDefinition thisCreature .bothEach (countOf creatureYouControl)) ] } }

def eomerOfTheRiddermark : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Éomer of the Riddermark", cost := some [generic 4, pip .red],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Knight"],
      text :=
        [ keyword "Haste",
          triggeredIf (attacks thisCreature)
            (exists_ (Primitives.Predicate.and [ creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you,
                             Primitives.Predicate.superlative .max (.stat .power) (Primitives.Predicate.and [creature, permanent]) ]))
            (create (.lit 1) (creatureToken 1 1 [.white] [creatureType "Human", creatureType "Soldier"])) ],
      power := stat 5, toughness := stat 4 } }

def angelicGift : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Angelic Gift", cost := some [generic 1, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          when (Primitives.GameEvent.enters thisAura none) (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)),
          Primitives.Ability.static (Primitives.StaticSpec.abilityGrant (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (keyword "Flying")) ] }
              }

def consulsLieutenant : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Consul's Lieutenant", cost := some [pip .white, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Soldier"],
      text :=
        [ keyword "FirstStrike",
          renown 1,
          triggeredIf (attacks thisCreature) (Primitives.Condition.matches it (Primitives.Predicate.hasDesignation "renowned" none))
            (get (allOf (Primitives.Predicate.and [creature, attacking, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, Primitives.Predicate.otherThan
                thisCreature]))
              (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1)) (some untilEndOfTurn)) ],
      power := stat 2, toughness := stat 1 } }

def secretsOfTheGoldenCity : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Secrets of the Golden City", cost := some [generic 1, pip .blue, pip .blue],
      types := [.sorcery],
      text :=
        [ keyword "Ascend",
          Primitives.Ability.spell none (Primitives.Instruction.replace (Primitives.Instruction.draw (.lit 2) (agent := Primitives.NounPhrase.you))
            (Primitives.Instruction.doIf (Primitives.Condition.matches Primitives.NounPhrase.you (Primitives.Predicate.hasDesignation "the city's blessing" none)) (Primitives.Instruction.draw (.lit 3)
                (agent := Primitives.NounPhrase.you))
              none)) ] } }

def bombur : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bombur, Gentle Dreamer", cost := some [generic 2, pip .red],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Dwarf", creatureType "Bard"],
      text :=
        [ keyword "Storied",
          Primitives.Ability.static (onlyUnless (doesntUntap thisCreature (some Primitives.NounPhrase.you))
            (Primitives.Condition.matches Primitives.NounPhrase.you (Primitives.Predicate.hasDesignation "an enduring story" none))) ],
      power := stat 5, toughness := stat 3 } }

def drachNyen : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Drach'Nyen", cost := some [generic 4, pip .black, pip .red],
      supertypes := [.legendary], types := [.artifact], subtypes := [artifactType "Equipment"],
      text :=
        [ when (Primitives.GameEvent.enters thisEquipment none) (exile (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 1)) creature)),
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ Primitives.StaticSpec.abilityGrant (Primitives.NounPhrase.attachHost .equipped (.type .creature)) (keyword "Menace"),
              Primitives.StaticSpec.modification (Primitives.NounPhrase.attachHost .equipped (.type .creature)) .power (Primitives.Delta.up (Primitives.Amount.letter .x)),
              Primitives.StaticSpec.modification (Primitives.NounPhrase.attachHost .equipped (.type .creature)) .toughness (Primitives.Delta.up (.lit 0)),
              Primitives.StaticSpec.letterDefinition .x (Primitives.Amount.statOf (.stat .power) (the (Primitives.Predicate.exiledWith thisEquipment))) ]),
          keywordCosting "Equip" (Primitives.Cost.mana [generic 2]) ] } }

def colossalGraveReaver : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Colossal Grave-Reaver", cost := some [generic 6, pip .black, pip .green],
      types := [.creature], subtypes := [creatureType "Dragon"],
      text :=
        [ keyword "Flying",
          triggeredOr (Primitives.GameEvent.enters thisCreature none) [attacks thisCreature] (mill (.lit 3) Primitives.NounPhrase.you (agent
              := Primitives.NounPhrase.you)),
          whenever
            (putIntoFrom (counted (atLeast 1) (Primitives.Predicate.and [Primitives.Predicate.isCard, creature])) (graveyardOf Primitives.NounPhrase.you)
              (Primitives.EventSource.zones [yourLibrary]))
            (putOntoBattlefield (someOf (exactly 1) them)) ],
      power := stat 7, toughness := stat 6 } }

/-- Grimdancer -/
def grimdancer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Grimdancer", cost := some [generic 1, pip .black, pip .black], types := [.creature],
      subtypes := [creatureType "Nightmare"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.entryRider thisCreature
            (Primitives.TokenRider.withCounters (.lit 2)
              (Primitives.CounterKindSource.distinctChosen [.keyword "Menace", .keyword "Deathtouch", .keyword "Lifelink"]) .fresh)) ],
      power := stat 3, toughness := stat 3 } }

def divineVisitation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Divine Visitation", cost := some [generic 3, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.replacement
            (Primitives.GameEvent.tokensCreated (counted (atLeast 1) (Primitives.Predicate.and [creature, Primitives.Predicate.isToken])) false none (some Primitives.NounPhrase.you))
            [] none
            (create Primitives.Amount.groupSize
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
        [ keywordCosting "Ward" (Primitives.Cost.mana [generic 2]),
          Primitives.Ability.static (Primitives.StaticSpec.replacement (Primitives.GameEvent.tokensCreated (counted (atLeast 1) Primitives.Predicate.isToken) false none (some
              Primitives.NounPhrase.you))
            [] none (Primitives.Instruction.create (times (.lit 2) Primitives.Amount.groupSize) Primitives.TokenSpec.asThose [] (agent := Primitives.NounPhrase.you)) .repeatedly
                none) ],
      power := stat 2, toughness := stat 2 } }

def windZendikon : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Wind Zendikon", cost := some [pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" land,
          Primitives.Ability.static (Primitives.StaticSpec.qualityChange (Primitives.NounPhrase.attachHost .enchanted (.type .land)) .sets
            (Primitives.QualityPayload.bundle
              { characteristics :=
                { colors := [.blue], types := [.creature], subtypes := [creatureType "Elemental"],
                  text := [keyword "Flying"], power := stat 2, toughness := stat 2 } }
              (some .land))),
          when (Primitives.GameEvent.dies (Primitives.NounPhrase.attachHost .enchanted (.type .land))) (move (that .card) hand) ] } }

def awakenTheBear : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Awaken the Bear", cost := some [generic 2, pip .green], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.establish
            (Primitives.StaticSpec.conjunction none
              [ Primitives.StaticSpec.modification (target creature) .power (Primitives.Delta.up (.lit 3)),
                Primitives.StaticSpec.modification (itsOther (target creature) (Primitives.Delta.up (.lit 3))) .toughness (Primitives.Delta.up (.lit 3)),
                Primitives.StaticSpec.abilityGrant it (keyword "Trample") ])
            (some untilEndOfTurn)) ] } }

def spidersilkArmor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Spidersilk Armor", cost := some [generic 2, pip .green], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ Primitives.StaticSpec.modification (allOf creatureYouControl) .power (Primitives.Delta.up (.lit 0)),
              Primitives.StaticSpec.modification (itsOther (allOf creatureYouControl) (Primitives.Delta.up (.lit 0))) .toughness (Primitives.Delta.up
                  (.lit 1)),
              Primitives.StaticSpec.abilityGrant them (keyword "Reach") ]) ] } }

def arcaneFlight : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Arcane Flight", cost := some [pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ Primitives.StaticSpec.modification (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) .power (Primitives.Delta.up (.lit 1)),
              Primitives.StaticSpec.modification (itsOther (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (Primitives.Delta.up (.lit 1)))
                  .toughness
                (Primitives.Delta.up (.lit 1)),
              Primitives.StaticSpec.abilityGrant it (keyword "Flying") ]) ] } }

def bootsOfSpeed : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Boots of Speed", cost := some [pip .red], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ Primitives.StaticSpec.modification (Primitives.NounPhrase.attachHost .equipped (.type .creature)) .power (Primitives.Delta.up (.lit 1)),
              Primitives.StaticSpec.modification (itsOther (Primitives.NounPhrase.attachHost .equipped (.type .creature)) (Primitives.Delta.up (.lit 1)))
                  .toughness
                (Primitives.Delta.up (.lit 0)),
              Primitives.StaticSpec.abilityGrant it (keyword "Haste") ]),
          keywordCosting "Equip" (Primitives.Cost.mana [generic 1]) ] } }

/-- "enchanted creature loses all abilities and is a <colors> <subtype> with base P/T" -/
def enchantedBecomesVanilla (colors : List Color) (subtype : String) (power toughness : Nat) :
    StaticSpec :=
  Primitives.StaticSpec.conjunction none
    [ Primitives.StaticSpec.allAbilityLoss (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) none,
      Primitives.StaticSpec.qualityChange it .sets
        (Primitives.QualityPayload.bundle { characteristics := { colors, types := [.creature], subtypes := [creatureType subtype] } }
          none),
      Primitives.StaticSpec.modification it .power (Primitives.Delta.set (.lit power)),
      Primitives.StaticSpec.modification (itsOther it (Primitives.Delta.set (.lit power))) .toughness (Primitives.Delta.set (.lit toughness)) ]

def frogify : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Frogify", cost := some [generic 1, pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          Primitives.Ability.static (enchantedBecomesVanilla [.blue] "Frog" 1 1) ] } }

def darksteelMutation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Darksteel Mutation", cost := some [generic 1, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ Primitives.StaticSpec.qualityChange (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) .sets
                (Primitives.QualityPayload.bundle { characteristics := { types := [.artifact, .creature], subtypes := [creatureType "Insect"] } }
                  none),
              Primitives.StaticSpec.modification it .power (Primitives.Delta.set (.lit 0)),
              Primitives.StaticSpec.modification (itsOther it (Primitives.Delta.set (.lit 0))) .toughness (Primitives.Delta.set (.lit 1)),
              Primitives.StaticSpec.abilityGrant it (keyword "Indestructible"),
              Primitives.StaticSpec.allAbilityLoss it none ]) ] } }

def kenrithsTransformation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Kenrith's Transformation", cost := some [generic 1, pip .green],
      types := [.enchantment], subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          when (Primitives.GameEvent.enters thisAura none) (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)),
          Primitives.Ability.static (enchantedBecomesVanilla [.green] "Elk" 3 3) ] } }

def amphibianDownpour : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Amphibian Downpour", cost := some [generic 2, pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keyword "Flash", storm,
          keywordSubject "Enchant" creature,
          Primitives.Ability.static (enchantedBecomesVanilla [.blue] "Frog" 1 1) ] } }

def lignify : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lignify", cost := some [generic 1, pip .green], types := [.kindred, .enchantment],
      subtypes := [creatureType "Treefolk", enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ Primitives.StaticSpec.qualityChange (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) .sets
                (Primitives.QualityPayload.bundle { characteristics := { subtypes := [creatureType "Treefolk"] } } none),
              Primitives.StaticSpec.modification it .power (Primitives.Delta.set (.lit 0)),
              Primitives.StaticSpec.modification (itsOther it (Primitives.Delta.set (.lit 0))) .toughness (Primitives.Delta.set (.lit 4)),
              Primitives.StaticSpec.allAbilityLoss it none ]) ] } }

def nefariousImp : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Nefarious Imp", cost := some [generic 2, pip .black], types := [.creature],
      subtypes := [creatureType "Imp"],
      text :=
        [ keyword "Flying",
          whenever
            (leavesBattlefield (counted (atLeast 1) (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))
            (scry (.lit 1) (agent := Primitives.NounPhrase.you)) ],
      power := stat 2, toughness := stat 1 } }

/-- Ainok Tracker -/
def ainokTracker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ainok Tracker", cost := some [generic 5, pip .red], types := [.creature],
      subtypes := [creatureType "Dog", creatureType "Scout"],
      text := [keyword "FirstStrike", keywordCosting "Morph" (Primitives.Cost.mana [generic 4, pip .red])],
      power := stat 3, toughness := stat 3 } }

def irenicussVileDuplication : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Irenicus's Vile Duplication", cost := some [generic 3, pip .blue], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.create (.lit 1)
            (Primitives.TokenSpec.copyOf (target (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
              [Primitives.CopyExcept.ability (keyword "Flying"), Primitives.CopyExcept.nonlegendary])
            [] (agent := Primitives.NounPhrase.you)) ] } }

def cacklingCounterpart : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cackling Counterpart", cost := some [generic 1, pip .blue, pip .blue],
      types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.create (.lit 1)
            (Primitives.TokenSpec.copyOf (target (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) []) [] (agent :=
                Primitives.NounPhrase.you)),
          keywordCosting "Flashback" (Primitives.Cost.mana [generic 5, pip .blue, pip .blue]) ] } }

def chandrasSpitfire : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chandra's Spitfire", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text :=
        [ keyword "Flying",
          whenever (Primitives.GameEvent.isDealtDamage .noncombatOnly (a Primitives.Predicate.opponent))
            (get thisCreature (Primitives.Delta.up (.lit 3)) (Primitives.Delta.up (.lit 0)) (some untilEndOfTurn)) ],
      power := stat 1, toughness := stat 3 } }

def livingHive : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Living Hive", cost := some [generic 6, pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Elemental", creatureType "Insect"],
      text :=
        [ keyword "Trample",
          whenever (dealsCombatDamage thisCreature (a Primitives.Predicate.anyPlayer))
            (create Primitives.Amount.thatMuch (creatureToken 1 1 [.green] [creatureType "Insect"])) ],
      power := stat 6, toughness := stat 6 } }

def giantCindermaw : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Giant Cindermaw", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [creatureType "Dinosaur", creatureType "Beast"],
      text := [keyword "Trample", Primitives.Ability.static (playerCant (.core .gainLife) (Primitives.NounPhrase.playerGroup .allPlayers))],
      power := stat 4, toughness := stat 3 } }

def lushGrowth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lush Growth", cost := some [pip .green], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" land,
          Primitives.Ability.static (Primitives.StaticSpec.qualityChange (Primitives.NounPhrase.attachHost .enchanted (.type .land)) .sets
            (Primitives.QualityPayload.bundle
              { characteristics :=
                { subtypes := [landType "Mountain", landType "Forest", landType "Plains"] } }
              none)) ] } }

def voiceOfAll : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Voice of All", cost := some [generic 2, pip .white, pip .white], types := [.creature],
      subtypes := [creatureType "Angel"],
      text :=
        [ keyword "Flying",
          Primitives.Ability.static (entersChoosing thisCreature .color),
          Primitives.Ability.static (Primitives.StaticSpec.abilityGrant thisCreature (keywordQuality "Protection" (ofChosen .color))) ],
      power := stat 2, toughness := stat 2 } }

def wardSliver : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ward Sliver", cost := some [generic 4, pip .white], types := [.creature],
      subtypes := [creatureType "Sliver"],
      text :=
        [ Primitives.Ability.static (entersChoosing thisCreature .color),
          Primitives.Ability.static (Primitives.StaticSpec.abilityGrant (allOf (Primitives.Predicate.hasSubtype (creatureType "Sliver")))
            (keywordQuality "Protection" (ofChosen .color))) ],
      power := stat 2, toughness := stat 2 } }

def sanctuaryBlade : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sanctuary Blade", cost := some [generic 2], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ Primitives.Ability.static (attachChoosing thisEquipment .color),
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ Primitives.StaticSpec.modification (Primitives.NounPhrase.attachHost .equipped (.type .creature)) .power (Primitives.Delta.up (.lit 2)),
              Primitives.StaticSpec.modification (itsOther (Primitives.NounPhrase.attachHost .equipped (.type .creature)) (Primitives.Delta.up (.lit 2)))
                  .toughness
                (Primitives.Delta.up (.lit 0)),
              Primitives.StaticSpec.abilityGrant (Primitives.NounPhrase.attachHost .equipped (.type .creature))
                (keywordQuality "Protection" (ofTheLastChosen .color)) ]),
          keywordCosting "Equip" (Primitives.Cost.mana [generic 3]) ] } }

/-- Sinister Strength -/
def sinisterStrength : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sinister Strength", cost := some [generic 1, pip .black], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ Primitives.StaticSpec.modification (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) .power (Primitives.Delta.up (.lit 3)),
              Primitives.StaticSpec.modification (itsOther (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (Primitives.Delta.up (.lit 3)))
                  .toughness
                (Primitives.Delta.up (.lit 1)),
              Primitives.StaticSpec.qualityChange it .sets (Primitives.QualityPayload.colored (.some [.black])) ]) ] } }

/-- Crimson Wisps -/
def crimsonWisps : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Crimson Wisps", cost := some [pip .red], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ Primitives.Instruction.establish
                (Primitives.StaticSpec.conjunction none
                  [ Primitives.StaticSpec.qualityChange (target creature) .sets (Primitives.QualityPayload.colored (.some [.red])),
                    Primitives.StaticSpec.abilityGrant it (keyword "Haste") ])
                (some untilEndOfTurn),
              Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you) ]) ] } }

/-- Ghoulflesh -/
def ghoulflesh : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ghoulflesh", cost := some [pip .black], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ Primitives.StaticSpec.modification (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) .power (Primitives.Delta.down (.lit 1)),
              Primitives.StaticSpec.modification (itsOther (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (Primitives.Delta.down (.lit 1)))
                  .toughness
                (Primitives.Delta.down (.lit 1)),
              Primitives.StaticSpec.qualityChange it .adds
                (Primitives.QualityPayload.bundle { characteristics := { colors := [.black], subtypes := [creatureType "Zombie"] } } none) ]) ] } }

/-- Blade of the Oni -/
def bladeOfTheOniStatic : StaticSpec :=
  Primitives.StaticSpec.conjunction none
    [ Primitives.StaticSpec.modification (Primitives.NounPhrase.attachHost .equipped (.type .creature)) .power (Primitives.Delta.set (.lit 5)),
      Primitives.StaticSpec.modification (itsOther (Primitives.NounPhrase.attachHost .equipped (.type .creature)) (Primitives.Delta.set (.lit 5))) .toughness
        (Primitives.Delta.set (.lit 5)),
      Primitives.StaticSpec.abilityGrant it (keyword "Menace"),
      Primitives.StaticSpec.qualityChange it .adds
        (Primitives.QualityPayload.bundle { characteristics := { colors := [.black], subtypes := [creatureType "Demon"] } } none) ]
theorem okBladeOfTheOniStatic : StaticSpec.check [] bladeOfTheOniStatic = [] := by decide

def historyOfBenalia : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "History of Benalia", cost := some [generic 1, pip .white, pip .white],
      types := [.enchantment], subtypes := [enchantmentType "Saga"],
      text :=
        [ when (Primitives.GameEvent.chapterMark [1, 2])
            (create (.lit 1)
              { characteristics :=
                { colors := [.white], types := [.creature], subtypes := [creatureType "Knight"],
                  text := [keyword "Vigilance"], power := stat 2, toughness := stat 2 } }),
          when (Primitives.GameEvent.chapterMark [3])
            (get (allOf (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (creatureType "Knight"), Primitives.Predicate.hasPossessor .controller
                Primitives.NounPhrase.you]))
              (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 1)) (some untilEndOfTurn)) ] } }

/-- Akroan Sergeant -/
def akroanSergeant : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Akroan Sergeant", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Soldier"],
      text := [keyword "FirstStrike", renown 1], power := stat 2, toughness := stat 2 } }

def castCreatureSpellsFromTop : StaticSpec :=
  mayPlayDeed (.action "Cast") Primitives.NounPhrase.you (allOf (Primitives.Predicate.and [spell, creature])) none
    (Primitives.DeonticRider.play (some onTop) none none false Primitives.PlayPayment.itsOwnCost)
theorem okCastCreatureSpellsFromTop : StaticSpec.check [] castCreatureSpellsFromTop = [] := by decide

def garruksHorde : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Garruk's Horde", cost := some [generic 5, pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Beast"],
      text :=
        [ keyword "Trample",
          Primitives.Ability.static (Primitives.StaticSpec.visibility .reveal Primitives.NounPhrase.you Primitives.VisibleThing.topOfLibrary),
          Primitives.Ability.static castCreatureSpellsFromTop ],
      power := stat 7, toughness := stat 7 } }

def wanderingEye : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Wandering Eye", cost := some [generic 2, pip .blue], types := [.creature],
      subtypes := [creatureType "Illusion"],
      text := [keyword "Flying", Primitives.Ability.static (Primitives.StaticSpec.visibility .reveal (Primitives.NounPhrase.playerGroup .allPlayers) Primitives.VisibleThing.wholeHand)],
      power := stat 1, toughness := stat 3 } }

def amorphousAxe : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Amorphous Axe", cost := some [generic 2], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ Primitives.StaticSpec.modification (Primitives.NounPhrase.attachHost .equipped (.type .creature)) .power (Primitives.Delta.up (.lit 3)),
              Primitives.StaticSpec.modification (itsOther (Primitives.NounPhrase.attachHost .equipped (.type .creature)) (Primitives.Delta.up (.lit 3)))
                  .toughness
                (Primitives.Delta.up (.lit 0)),
              Primitives.StaticSpec.qualityChange (Primitives.NounPhrase.attachHost .equipped (.type .creature)) .adds (Primitives.QualityPayload.everyTypeOf
                  .creature) ]),
          keywordCosting "Equip" (Primitives.Cost.mana [generic 3]) ] } }

def runedStalactite : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Runed Stalactite", cost := some [generic 1], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ Primitives.StaticSpec.modification (Primitives.NounPhrase.attachHost .equipped (.type .creature)) .power (Primitives.Delta.up (.lit 1)),
              Primitives.StaticSpec.modification (itsOther (Primitives.NounPhrase.attachHost .equipped (.type .creature)) (Primitives.Delta.up (.lit 1)))
                  .toughness
                (Primitives.Delta.up (.lit 1)),
              Primitives.StaticSpec.qualityChange (Primitives.NounPhrase.attachHost .equipped (.type .creature)) .adds (Primitives.QualityPayload.everyTypeOf
                  .creature) ]),
          keywordCosting "Equip" (Primitives.Cost.mana [generic 2]) ] } }

def arachnoform : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Arachnoform", cost := some [generic 1, pip .green], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ Primitives.StaticSpec.modification (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) .power (Primitives.Delta.up (.lit 2)),
              Primitives.StaticSpec.modification (itsOther (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (Primitives.Delta.up (.lit 2)))
                  .toughness
                (Primitives.Delta.up (.lit 2)),
              Primitives.StaticSpec.abilityGrant (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (keyword "Reach"),
              Primitives.StaticSpec.qualityChange (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) .adds (Primitives.QualityPayload.everyTypeOf
                  .creature) ]) ] } }

def nyleasPresence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Nylea's Presence", cost := some [generic 1, pip .green], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" land,
          when (Primitives.GameEvent.enters thisAura none) (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)),
          Primitives.Ability.static (Primitives.StaticSpec.qualityChange (Primitives.NounPhrase.attachHost .enchanted (.type .land)) .adds (Primitives.QualityPayload.everyTypeOf
              .basicLand)) ] } }

/-- Curse of Conformity -/
def curseOfConformity : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Curse of Conformity", cost := some [generic 4, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura", enchantmentType "Curse"],
      text :=
        [ keywordSubject "Enchant" Primitives.Predicate.anyPlayer,
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ Primitives.StaticSpec.modification
                (allOf (Primitives.Predicate.and [ creature, Primitives.Predicate.not (Primitives.Predicate.hasSupertype .legendary),
                               Primitives.Predicate.hasPossessor .controller (Primitives.NounPhrase.attachHost .enchanted .player) ]))
                .power (Primitives.Delta.set (.lit 3)),
              Primitives.StaticSpec.modification
                (itsOther
                  (allOf (Primitives.Predicate.and [ creature, Primitives.Predicate.not (Primitives.Predicate.hasSupertype .legendary),
                                 Primitives.Predicate.hasPossessor .controller (Primitives.NounPhrase.attachHost .enchanted .player) ]))
                  (Primitives.Delta.set (.lit 3)))
                .toughness (Primitives.Delta.set (.lit 3)),
              Primitives.StaticSpec.qualityChange them .loses (Primitives.QualityPayload.everyTypeOf .creature) ]) ] } }

/-- Drumbellower -/
def drumbellower : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Drumbellower", cost := some [generic 2, pip .white], types := [.creature],
      subtypes := [creatureType "Spirit"],
      text :=
        [ keyword "Flying",
          Primitives.Ability.static (untapsDuring (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
            (some (each otherPlayer))) ],
      power := stat 2, toughness := stat 1 } }

def tayamLine : StaticSpec :=
  entersWithAdditionalCounters
    (each (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, Primitives.Predicate.otherThan thisCreature])) (.lit 1)
    (.keyword "Vigilance")
theorem okTayamLine : StaticSpec.check [] tayamLine = [] := by decide

def clarionConqueror : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Clarion Conqueror", cost := some [generic 2, pip .white], types := [.creature],
      subtypes := [creatureType "Dragon"],
      text :=
        [ keyword "Flying",
          Primitives.Ability.static (objectCant (.action "Activate")
            (allOf (Primitives.Predicate.and [ Primitives.Predicate.abilityHead .anyActivated,
                           Primitives.Predicate.abilityOf (allOf (Primitives.Predicate.or [artifact, creature, Primitives.Predicate.hasType .planeswalker])) ]))) ],
      power := stat 3, toughness := stat 3 } }

/-- Kang the Conqueror -/
def kangPowerUpLock : StaticSpec :=
  objectCant (.action "Activate") (allOf (Primitives.Predicate.abilityHead (.keyword "PowerUp")))
theorem okKangPowerUpLock : StaticSpec.check [] kangPowerUpLock = [] := by decide

def vipersKiss : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Viper's Kiss", cost := some [pip .black], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ Primitives.StaticSpec.modification (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) .power (Primitives.Delta.down (.lit 1)),
              Primitives.StaticSpec.modification (itsOther (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (Primitives.Delta.down (.lit 1)))
                  .toughness
                (Primitives.Delta.down (.lit 1)),
              objectCant (.action "Activate")
                (allOf (Primitives.Predicate.and [Primitives.Predicate.abilityHead .anyActivated, Primitives.Predicate.abilityOf it])) ]) ] } }

def stupefyingTouch : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stupefying Touch", cost := some [generic 1, pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          when (Primitives.GameEvent.enters thisAura none) (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)),
          Primitives.Ability.static (objectCant (.action "Activate")
            (allOf (Primitives.Predicate.and [ Primitives.Predicate.abilityHead .anyActivated,
                           Primitives.Predicate.abilityOf (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) ]))) ] } }

def linvalaKeeperOfSilence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Linvala, Keeper of Silence", cost := some [generic 2, pip .white, pip .white],
      supertypes := [.legendary], types := [.creature], subtypes := [creatureType "Angel"],
      text :=
        [ keyword "Flying",
          Primitives.Ability.static (objectCant (.action "Activate")
            (allOf (Primitives.Predicate.and [ Primitives.Predicate.abilityHead .anyActivated,
                           Primitives.Predicate.abilityOf (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller (Primitives.NounPhrase.playerGroup .yourOpponents)])) ]))) ],
      power := stat 3, toughness := stat 4 } }

def kyrenLegate : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Kyren Legate", cost := some [generic 1, pip .red], types := [.creature],
      subtypes := [creatureType "Goblin"],
      text :=
        [ Primitives.Ability.static (onlyWhile (Primitives.StaticSpec.altCost Primitives.NounPhrase.this none)
            (Primitives.Condition.and
              [ exists_ (Primitives.Predicate.and [ land, Primitives.Predicate.hasSubtype (landType "Plains"),
                                Primitives.Predicate.hasPossessor .controller anOpponent ]),
                exists_ (Primitives.Predicate.and [ land, Primitives.Predicate.hasSubtype (landType "Mountain"),
                                Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you ]) ])),
          keyword "Haste" ],
      power := stat 1, toughness := stat 1 } }

def polarKraken : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Polar Kraken", cost := some [generic 8, pip .blue, pip .blue, pip .blue],
      types := [.creature], subtypes := [creatureType "Kraken"],
      text :=
        [ keyword "Trample",
          Primitives.Ability.static (entersTapped thisCreature),
          cumulativeUpkeep (Primitives.Cost.perform (sacrifice (a land) (agent := Primitives.NounPhrase.you))) ],
      power := stat 11, toughness := stat 11 } }

def yavimayaAnts : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Yavimaya Ants", cost := some [generic 2, pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Insect"],
      text := [keyword "Trample", keyword "Haste", cumulativeUpkeep (Primitives.Cost.mana [pip .green, pip .green])],
      power := stat 5, toughness := stat 1 } }

def illusionaryForces : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Illusionary Forces", cost := some [generic 3, pip .blue], types := [.creature],
      subtypes := [creatureType "Illusion"],
      text := [keyword "Flying", cumulativeUpkeep (Primitives.Cost.mana [pip .blue])],
      power := stat 4, toughness := stat 4 } }

def vexingSphinx : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vexing Sphinx", cost := some [generic 1, pip .blue, pip .blue], types := [.creature],
      subtypes := [creatureType "Sphinx"],
      text :=
        [ keyword "Flying",
          cumulativeUpkeep (Primitives.Cost.perform (discard (a (Primitives.Predicate.inZone hand)) (agent := Primitives.NounPhrase.you))),
          when (Primitives.GameEvent.dies thisCreature) (Primitives.Instruction.draw (countersOn (.named "Age") it) (agent := Primitives.NounPhrase.you)) ],
      power := stat 4, toughness := stat 4 } }

def manaChains : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mana Chains", cost := some [pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          Primitives.Ability.static (Primitives.StaticSpec.abilityGrant (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (cumulativeUpkeep (Primitives.Cost.mana
              [generic 1]))) ] } }

def dreamThief : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dream Thief", cost := some [generic 2, pip .blue], types := [.creature],
      subtypes := [creatureType "Faerie", creatureType "Rogue"],
      text :=
        [ keyword "Flying",
          when (Primitives.GameEvent.enters thisCreature none)
            (Primitives.Instruction.doOnlyIf (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you))
              (happened (Primitives.GameEvent.casts (relative .player) (some (a
                (Primitives.Predicate.and [spell, Primitives.Predicate.colorIs .blue,
                Primitives.Predicate.otherThan Primitives.NounPhrase.this]))) none)
                Primitives.NounPhrase.you .thisTurn)
              none) ],
      power := stat 2, toughness := stat 1 } }

def brightspearZealot : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Brightspear Zealot", cost := some [generic 2, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Soldier"],
      text :=
        [ keyword "Vigilance",
          Primitives.Ability.static (onlyWhile (getsPt thisCreature (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 0)))
            (Primitives.Condition.compareAmt (eventCount (Primitives.GameEvent.casts (relative
              .player) (some (a spell)) none) Primitives.NounPhrase.you .thisTurn) .atLeast (.lit
              2))) ],
      power := stat 2, toughness := stat 4 } }

def deepwayNavigator : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Deepway Navigator", cost := some [pip .white, pip .blue], types := [.creature],
      subtypes := [creatureType "Merfolk", creatureType "Wizard"],
      text :=
        [ keyword "Flash",
          when (Primitives.GameEvent.enters thisCreature none)
            (Primitives.Instruction.setStatus .untapped
              (each (Primitives.Predicate.and [ Primitives.Predicate.hasSubtype (creatureType "Merfolk"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you,
                            Primitives.Predicate.otherThan thisCreature ]))),
          Primitives.Ability.static (onlyWhile
            (getsPt (allOf (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (creatureType "Merfolk"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
              (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 0)))
            (happened (Primitives.GameEvent.attacksWith (relative .player) none (counted (atLeast 3)
              (Primitives.Predicate.hasSubtype (creatureType "Merfolk")))) Primitives.NounPhrase.you
              .thisTurn)) ],
      power := stat 2, toughness := stat 2 } }

def bloodfireEnforcers : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bloodfire Enforcers", cost := some [generic 3, pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Monk"],
      text :=
        [ Primitives.Ability.static (onlyWhile
            (Primitives.StaticSpec.conjunction none [Primitives.StaticSpec.abilityGrant thisCreature (keyword "FirstStrike"), Primitives.StaticSpec.abilityGrant it
                (keyword "Trample")])
            (Primitives.Condition.and
              [ exists_ (Primitives.Predicate.and [instant, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)]),
                exists_ (Primitives.Predicate.and [sorcery, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)]) ])) ],
      power := stat 5, toughness := stat 2 } }

def stormOfSouls : Instruction :=
  Primitives.Instruction.sequentially
    [ move (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)])) battlefield,
      become them
        { characteristics :=
          { subtypes := [creatureType "Spirit"], text := [keyword "Flying"],
            power := stat 1, toughness := stat 1 } }
        none,
      exile Primitives.NounPhrase.this ]
theorem okStormOfSouls : Instruction.check [] stormOfSouls = [] := by decide

def answeredPrayers : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Answered Prayers", cost := some [generic 1, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ when (Primitives.GameEvent.enters (a creatureYouControl) none)
            (Primitives.Instruction.sequentially
              [ gainLife (.lit 1) (agent := Primitives.NounPhrase.you),
                Primitives.Instruction.doIf (Primitives.Condition.not (Primitives.Condition.matches thisEnchantment creature))
                  (become (itCondSubject (Primitives.Condition.not (Primitives.Condition.matches thisEnchantment creature)))
                    { characteristics :=
                      { types := [.creature], subtypes := [creatureType "Angel"],
                        text := [keyword "Flying"], power := stat 3, toughness := stat 3 } }
                    (some untilEndOfTurn))
                  none ]) ] } }

/-- Hate Mirage's middle two sentences -/
def hateMirageTokens : Instruction :=
  Primitives.Instruction.sequentially
    [ Primitives.Instruction.doForEach (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 2)) creatureYouDontControl)
        (Primitives.Instruction.create (.lit 1) (Primitives.TokenSpec.copyOf it []) [] (agent := Primitives.NounPhrase.you)),
      gain (those .token) (keyword "Haste") none ]
theorem okHateMirageTokens : Instruction.check [] hateMirageTokens = [] := by decide
def descentOfTheDragons : Instruction :=
  Primitives.Instruction.sequentially
    [ destroy (Primitives.NounPhrase.described (Primitives.DetPhrase.target anyNumber) creature),
      Primitives.Instruction.doForEach (theVerbed (.action "Destroy") (.type .creature) .thisWay .many)
        (Primitives.Instruction.create (.lit 1)
          (Primitives.TokenSpec.written
            { characteristics :=
              { colors := [.red], types := [.creature], subtypes := [creatureType "Dragon"],
                text := [keyword "Flying"], power := stat 4, toughness := stat 4 } })
          [] (agent := (controllerOf it))) ]
theorem okDescentOfTheDragons : Instruction.check [] descentOfTheDragons = [] := by decide
/-- Brimaz, King of Oreskos -/
def brimazAttackToken : Ability :=
  whenever (attacks thisCreature)
    (Primitives.Instruction.create (.lit 1)
      (Primitives.TokenSpec.written
        { characteristics :=
          { colors := [.white], types := [.creature], subtypes := [creatureType "Cat", creatureType "Soldier"],
            text := [keyword "Vigilance"], power := stat 1, toughness := stat 1 } })
      [Primitives.TokenRider.entersAttacking none] (agent := Primitives.NounPhrase.you))
theorem okBrimazAttackToken : Ability.check [] brimazAttackToken = [] := by decide
/-- Sedris, the Traitor King -/
def sedrisTheTraitorKing : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.abilityGrant (each (Primitives.Predicate.and [creature, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)]))
    (keywordCosting "Unearth" (Primitives.Cost.mana [generic 2, pip .black])))
theorem okSedrisTheTraitorKing : Ability.check [] sedrisTheTraitorKing = [] := by decide
/-- Grixis -/
def grixis : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.abilityGrant
    (allOf (Primitives.Predicate.and [ Primitives.Predicate.or [Primitives.Predicate.colorIs .blue, Primitives.Predicate.colorIs .black, Primitives.Predicate.colorIs .red], creature,
                   Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you) ]))
    (keywordCosting "Unearth" Primitives.Cost.itsManaCost))
theorem okGrixis : Ability.check [] grixis = [] := by decide
/-- Dralnu, Lich Lord -/
def dralnuLichLord : Instruction :=
  gain (target (Primitives.Predicate.and [instantOrSorcery, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)]))
    (keywordCosting "Flashback" Primitives.Cost.itsManaCost) (some untilEndOfTurn)
theorem okDralnuLichLord : Instruction.check [] dralnuLichLord = [] := by decide
/-- Dregscape Zombie -/
def dregscapeZombie : Ability := keywordCosting "Unearth" (Primitives.Cost.mana [pip .black])
theorem okDregscapeZombie : Ability.check [] dregscapeZombie = [] := by decide
/-- Think Twice -/
def thinkTwice : Ability := keywordCosting "Flashback" (Primitives.Cost.mana [generic 2, pip .blue])
theorem okThinkTwice : Ability.check [] thinkTwice = [] := by decide
/-- Greater Mossdog -/
def greaterMossdog : Ability := keywordNumber "Dredge" (.lit 3)
theorem okGreaterMossdog : Ability.check [] greaterMossdog = [] := by decide
/-- Raven's Crime -/
def ravensCrime : Ability := keyword "Retrace"
theorem okRavensCrime : Ability.check [] ravensCrime = [] := by decide
/-- Barkhide Mauler -/
def barkhideMauler : Ability := keywordCosting "Cycling" (Primitives.Cost.mana [generic 2])
theorem okBarkhideMauler : Ability.check [] barkhideMauler = [] := by decide
/-- Ninja of the New Moon -/
def ninjaOfTheNewMoon : Ability := keywordCosting "Ninjutsu" (Primitives.Cost.mana [generic 3, pip .black])
theorem okNinjaOfTheNewMoon : Ability.check [] ninjaOfTheNewMoon = [] := by decide
/-- Thunderous Wrath -/
def thunderousWrath : Ability := keywordCosting "Miracle" (Primitives.Cost.mana [pip .red])
theorem okThunderousWrath : Ability.check [] thunderousWrath = [] := by decide
/-- Bygone Colossus -/
def bygoneColossus : Ability := keywordCosting "Warp" (Primitives.Cost.mana [generic 3])
theorem okBygoneColossus : Ability.check [] bygoneColossus = [] := by decide
/-- Knight of Grace — "Hexproof from black" [CR#702.11d] -/
def knightOfGraceHexproofFromBlack : Ability := keywordQuality "Hexproof" (Primitives.Predicate.colorIs .black)
theorem okKnightOfGraceHexproofFromBlack :
    Ability.check [] knightOfGraceHexproofFromBlack = [] := by decide
/-- Eternal Dragon — "Plainscycling {2}" [CR#702.29e] -/
def eternalDragonPlainscycling : Ability :=
  keywordQualityCosting "Cycling" (Primitives.Predicate.hasSubtype (landType "Plains")) (Primitives.Cost.mana [generic 2])
theorem okEternalDragonPlainscycling : Ability.check [] eternalDragonPlainscycling = [] := by decide
/-- Nezumi Ronin -/
def nezumiRonin : Ability := keywordNumber "Bushido" (.lit 1)
theorem okNezumiRonin : Ability.check [] nezumiRonin = [] := by decide
/-- Dragonlord Ojutai -/
def dragonlordOjutaiHexproof : Ability :=
  Primitives.Ability.static (onlyWhile (Primitives.StaticSpec.abilityGrant thisCreature (keyword "Hexproof")) (Primitives.Condition.matches it untapped))
theorem okDragonlordOjutaiHexproof : Ability.check [] dragonlordOjutaiHexproof = [] := by decide
/-- Monoxa, Midway Manager -/
def monoxaRollTrigger : Ability :=
  whenever (youRollResultIn (atLeast 3))
    (Primitives.Instruction.sequentially
      [ gain thisCreature (keyword "FirstStrike") (some untilEndOfTurn),
        doIf (Primitives.Condition.compareAmt (Primitives.Amount.theOutcome .rollResult) .atLeast (.lit 4))
          (gain it (keyword "Menace") (some untilEndOfTurn)),
        doIf (Primitives.Condition.compareAmt (Primitives.Amount.theOutcome .rollResult) .atLeast (.lit 5))
          (gain it (keyword "Lifelink") (some untilEndOfTurn)) ])
theorem okMonoxaRollTrigger : Ability.check [] monoxaRollTrigger = [] := by decide
/-- Celebr-8000 -/
def celebr8000Doubles : Instruction :=
  Primitives.Instruction.sequentially
    [ rollDice 2 6 (agent := Primitives.NounPhrase.you),
      doIf Primitives.Condition.rolledDoubles (gain thisCreature (keyword "DoubleStrike") (some untilEndOfTurn)) ]
theorem okCelebr8000Doubles : Instruction.check [] celebr8000Doubles = [] := by decide

/-- Krosan Druid -/
def krosanDruid : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Krosan Druid", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [creatureType "Centaur", creatureType "Druid"],
      text :=
        [ keywordCosting "Kicker" (Primitives.Cost.mana [generic 4, pip .green]),
          triggeredIf (Primitives.GameEvent.enters thisCreature none) (costWasPaid (.byKeyword "Kicker") none thisCreature)
            (gainLife (.lit 10) (agent := Primitives.NounPhrase.you)) ],
      power := stat 2, toughness := stat 3 } }

/-- Lightkeeper of Emeria -/
def lightkeeperOfEmeria : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lightkeeper of Emeria", cost := some [generic 3, pip .white], types := [.creature],
      subtypes := [creatureType "Angel"],
      text :=
        [ keywordCosting "Multikicker" (Primitives.Cost.mana [pip .white]),
          keyword "Flying",
          when (Primitives.GameEvent.enters thisCreature none)
            (gainLife (times (.lit 2) (timesPaid (.byKeyword "Kicker") thisCreature)) (agent :=
                Primitives.NounPhrase.you)) ],
      power := stat 2, toughness := stat 4 } }

/-- Merfolk Falconer -/
def merfolkFalconer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Merfolk Falconer", cost := some [generic 3, pip .blue, pip .blue], types := [.creature],
      subtypes := [creatureType "Merfolk", creatureType "Wizard"],
      text :=
        [ keyword "Flying",
          whenever
            (Primitives.GameEvent.casts Primitives.NounPhrase.you
              (some (a (Primitives.Predicate.compareOver spell (paidCostRead (.byKeyword "Kicker")
                none it) .atLeast (.lit 1))))
              none)
            (scry (.lit 2) (agent := Primitives.NounPhrase.you)) ],
      power := stat 4, toughness := stat 4 } }

/-- Borrowed Malevolence -/
def borrowedMalevolence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Borrowed Malevolence", cost := some [pip .black], types := [.instant],
      text :=
        [ keywordCosting "Escalate" (Primitives.Cost.mana [generic 2]),
          Primitives.Ability.spell none (chooseModes (Primitives.Quantity.range (some 1) (some 2))
            [ get (target creature) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1)) (some untilEndOfTurn),
              get (target creature) (Primitives.Delta.down (.lit 1)) (Primitives.Delta.down (.lit 1)) (some untilEndOfTurn) ]) ] } }

/-- New Perspectives -/
def newPerspectivesCyclingAltCost : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.altCost (allOf (Primitives.Predicate.abilityHead (.keyword "Cycling"))) (some (Primitives.Cost.mana [generic 0])))
theorem okNewPerspectivesCyclingAltCost :
    Ability.check [] newPerspectivesCyclingAltCost = [] := by decide
/-- Thick-Skinned Goblin -/
def thickSkinnedGoblinEchoAltCost : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.altCost
    (allOf (Primitives.Predicate.and [ Primitives.Predicate.abilityHead (.keyword "Echo"),
                   Primitives.Predicate.abilityOf (allOf (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) ]))
    (some (Primitives.Cost.mana [generic 0])))
theorem okThickSkinnedGoblinEchoAltCost :
    Ability.check [] thickSkinnedGoblinEchoAltCost = [] := by decide
/-- Fumiko the Lowblood -/
def fumikoBushidoX : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.conjunction none
    [ Primitives.StaticSpec.abilityGrant thisCreature (keywordNumber "Bushido" (Primitives.Amount.letter .x)),
      Primitives.StaticSpec.letterDefinition .x (countOf attacking) ])
theorem okFumikoBushidoX : Ability.check [] fumikoBushidoX = [] := by decide

/-- Rafter Demon -/
def rafterDemon : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rafter Demon", cost := some [generic 2, pip .black, pip .red], types := [.creature],
      subtypes := [creatureType "Demon"],
      text :=
        [ keywordCosting "Spectacle" (Primitives.Cost.mana [generic 3, pip .black, pip .red]),
          triggeredIf (Primitives.GameEvent.enters thisCreature none)
            (costWasPaid (.byKeyword "Spectacle") none thisCreature)
            (discard (a (Primitives.Predicate.inZone hand)) (agent := (each Primitives.Predicate.opponent))) ],
      power := stat 4, toughness := stat 2 } }

/-- Tourach, Dread Cantor -/
def tourachDiscardTrigger : Ability :=
  whenever (Primitives.GameEvent.verbedEvent (some anOpponent) (.action "Discard") (some (a
    Primitives.Predicate.isCard)) none none)
    (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne) thisCreature)
theorem okTourachDiscardTrigger : Ability.check [] tourachDiscardTrigger = [] := by decide

/-- Shimmering Glasskite -/
def shimmeringGlasskite : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shimmering Glasskite", cost := some [generic 3, pip .blue], types := [.creature],
      subtypes := [creatureType "Spirit"],
      text :=
        [ keyword "Flying",
          triggeredOnlyOnce
            (Primitives.GameEvent.becomesTarget thisCreature (a (Primitives.Predicate.or [spell, Primitives.Predicate.abilityHead .anyOnStack]))) Primitives.UsageLimit.oncePerTurn
            (Primitives.Instruction.counterSpell (that .stack)) ],
      power := stat 2, toughness := stat 3 } }

/-- Conqueror's Pledge -/
def conquerorsPledge : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Conqueror's Pledge", cost := some [generic 2, pip .white, pip .white, pip .white],
      types := [.sorcery],
      text :=
        [ keywordCosting "Kicker" (Primitives.Cost.mana [generic 6]),
          Primitives.Ability.spell none (Primitives.Instruction.replace
            (create (.lit 6) (creatureToken 1 1 [.white] [creatureType "Kor", creatureType "Soldier"]))
            (Primitives.Instruction.doIf (costWasPaid (.byKeyword "Kicker") none Primitives.NounPhrase.this)
              (Primitives.Instruction.create (.lit 12) Primitives.TokenSpec.asThose [] (agent := Primitives.NounPhrase.you)) none)) ] } }

/-- Soul of Emancipation -/
def soulOfEmancipation : Ability :=
  when (Primitives.GameEvent.enters thisCreature none)
    (Primitives.Instruction.sequentially
      [ destroy (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 3)) (Primitives.Predicate.and [permanent, Primitives.Predicate.not land, Primitives.Predicate.otherThan Primitives.NounPhrase.this])),
        Primitives.Instruction.doForEach (those .permanent)
          (Primitives.Instruction.create (.lit 1)
            (Primitives.TokenSpec.written
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
        [ keywordCosting "Kicker" (Primitives.Cost.mana [pip .black, pip .black]),
          keywordQuality "Protection" (Primitives.Predicate.colorIs .white),
          tourachDiscardTrigger,
          triggeredIf (Primitives.GameEvent.enters thisCreature none) (costWasPaid (.byKeyword "Kicker") none thisCreature)
            (discard (countedAtRandom (exactly 2) (Primitives.Predicate.inZone hand)) (agent := (target Primitives.Predicate.opponent))) ],
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
          Primitives.Ability.static (onlyWhile (getsPt thisCreature (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 2)))
            (exists_ (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (planeswalkerType "Garruk"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))) ],
      power := stat 4, toughness := stat 4 } }

/-- Tower Winder -/
def towerWinder : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tower Winder", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [creatureType "Snake"],
      text :=
        [ keyword "Reach", keyword "Deathtouch",
          when (Primitives.GameEvent.enters thisCreature none)
            (Primitives.Instruction.sequentially
              [ searchLibraryOrGraveyard (Primitives.Predicate.named (Primitives.NameSource.printed "Command Tower")),
                revealIt, move foundCard hand,
                Primitives.Instruction.doIf (happened (Primitives.GameEvent.verbedEvent (some
                  (relative .player)) (.action "Search") none none (some yourLibrary))
                  Primitives.NounPhrase.you .thisWay) shuffle
                  none ]) ],
      power := stat 1, toughness := stat 1 } }

/-- Aim High -/
def aimHigh : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aim High", cost := some [generic 1, pip .green], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ untap (target creature),
              Primitives.Instruction.establish
                (Primitives.StaticSpec.conjunction none
                  [ Primitives.StaticSpec.modification (itVerbed (.action "Untap")) .power (Primitives.Delta.up (.lit 2)),
                    Primitives.StaticSpec.modification (itVerbed (.action "Untap")) .toughness (Primitives.Delta.up (.lit 2)),
                    Primitives.StaticSpec.abilityGrant (itVerbed (.action "Untap")) (keyword "Reach") ])
                (some untilEndOfTurn) ]) ] } }

/-- Harried Dronesmith -/
def harriedDronesmithToken : Instruction :=
  Primitives.Instruction.sequentially [create (.lit 1) thopterToken, gainHaste itAsToken (some untilEndOfTurn)]
theorem okHarriedDronesmithToken : Instruction.check [] harriedDronesmithToken = [] := by decide

/-- Archfiend's Vessel -/
def archfiendsVessel : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Archfiend's Vessel", cost := some [pip .black], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Cleric"],
      text :=
        [ keyword "Lifelink",
          triggeredIf (Primitives.GameEvent.enters thisCreature none)
            (Primitives.Condition.or
              [ Primitives.Condition.happened it (Primitives.LookbackClause.mk
                (Primitives.GameEvent.enters (relative .object) (some (Primitives.EventSource.zones
                [graveyardOf Primitives.NounPhrase.you]))) .triggering),
                Primitives.Condition.matches it (Primitives.Predicate.and [castBy Primitives.NounPhrase.you, Primitives.Predicate.castFrom (graveyardOf Primitives.NounPhrase.you)]) ])
            (Primitives.Instruction.triggerReflexively (exile it)
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
          Primitives.Ability.static (Primitives.StaticSpec.replacement (Primitives.GameEvent.enters (a (Primitives.Predicate.and [creature, nontoken, Primitives.Predicate.not Primitives.Predicate.wasCast])) none) []
              none
            (exile it) .repeatedly none) ],
      power := stat 2, toughness := stat 2 } }

/-- Veiling Oddity -/
def veilingOddityLine : Ability :=
  triggeredWhile (lastCounterRemoved (.named "Time") Primitives.NounPhrase.this)
    (Primitives.Concurrent.whileTrue (Primitives.Condition.matches Primitives.NounPhrase.this (Primitives.Predicate.inZone exileZone)))
    (Primitives.Instruction.establish (deontic (allOf creature) Primitives.Compulsion.forbid [.core .block] .patient Primitives.DeonticPatient.noPatient)
      (some Primitives.Duration.thisTurn))
theorem okVeilingOddityLine : Ability.check [] veilingOddityLine = [] := by decide

def wizenedSnitches : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Wizened Snitches", cost := some [generic 3, pip .blue], types := [.creature],
      subtypes := [creatureType "Faerie", creatureType "Rogue"],
      text := [keyword "Flying", Primitives.Ability.static (Primitives.StaticSpec.visibility .reveal (Primitives.NounPhrase.playerGroup .allPlayers) Primitives.VisibleThing.topOfLibrary)],
      power := stat 1, toughness := stat 3 } }

/-- Darkblade Agent -/
def darkbladeAgentGrants : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.conditional
    (Primitives.StaticSpec.conjunction none
      [ Primitives.StaticSpec.abilityGrant thisCreature (keyword "Deathtouch"),
        Primitives.StaticSpec.abilityGrant thisCreature
          (whenever (dealsCombatDamage thisCreature (a Primitives.Predicate.anyPlayer)) (Primitives.Instruction.draw (.lit 1) (agent :=
              Primitives.NounPhrase.you))) ])
    (happened (Primitives.GameEvent.verbedEvent (some (relative .player)) (.action "Surveil") none
      none none) Primitives.NounPhrase.you .thisTurn) .asLongAs)
theorem okDarkbladeAgentGrants : Ability.check [] darkbladeAgentGrants = [] := by decide
/-- Frenzied Gorespawn -/
def frenziedGorespawnMenaceTrigger : Ability :=
  Primitives.Ability.triggered (Primitives.GameEvent.combat .attackerOf (counted (atLeast 1) creature) (some anOpponent)) [] none [] none
    none none (gain (those (.type .creature)) (keyword "Menace") (some untilEndOfTurn))
theorem okFrenziedGorespawnMenaceTrigger :
    Ability.check [] frenziedGorespawnMenaceTrigger = [] := by decide
/-- Pir, Imaginative Rascal's replacement -/
def pirDistributive : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.replacement
    (bareCounterEvent .put .many
      (a (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .controller (Primitives.NounPhrase.playerGroup .yourTeam)])))
    [] none (Primitives.Instruction.putCounters (plus Primitives.Amount.thatMuch (.lit 1)) Primitives.CounterKindSource.those (that .permanent)) .repeatedly none)
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
          Primitives.Ability.static (onlyWhile (Primitives.StaticSpec.abilityGrant thisCreature (keywordCosting "Ward" (Primitives.Cost.mana [generic
              4])))
            (Primitives.Condition.matches it untapped)),
          whenever (dealsCombatDamage thisCreature (a Primitives.Predicate.anyPlayer))
            (Primitives.Instruction.sequentially
              [ Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you),
                Primitives.Instruction.doIf (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.inZone (handOf Primitives.NounPhrase.you))) .less (.lit 3))
                  (Primitives.Instruction.draw Primitives.Amount.theDifference (agent := Primitives.NounPhrase.you)) none ]) ],
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
          Primitives.Ability.alsoForKeywords
            (Primitives.Ability.static (onlyWhile (Primitives.StaticSpec.abilityGrant thisCreature (keyword "Flying"))
              (exists_ (Primitives.Predicate.and [creature, Primitives.Predicate.inZone graveyard, Primitives.Predicate.hasKeyword (.the "Flying")]))))
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
          when (Primitives.GameEvent.enters thisVehicle none)
            (Primitives.Instruction.sequentially [loseLife (.lit 3) (agent := (each Primitives.Predicate.opponent)), gainLife (.lit 3) (agent :=
                Primitives.NounPhrase.you)]),
          keywordNumber "Crew" (.lit 2) ],
      power := stat 6, toughness := stat 6 } }

/-- Pir, Imaginative Rascal -/
def pirImaginativeRascal : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Pir, Imaginative Rascal", cost := some [generic 2, pip .green],
      supertypes := [.legendary], types := [.creature], subtypes := [creatureType "Human"],
      text :=
        [ keywordQuality "PartnerWith" (Primitives.Predicate.named (Primitives.NameSource.printed "Toothy, Imaginary Friend")),
          pirDistributive ],
      power := stat 1, toughness := stat 1 } }

/-- Market Gnome -/
def marketGnome : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Market Gnome", cost := some [pip .white], types := [.artifact, .creature],
      subtypes := [creatureType "Gnome"],
      text :=
        [ when (Primitives.GameEvent.dies thisCreature) (Primitives.Instruction.sequentially [gainLife (.lit 1) (agent := Primitives.NounPhrase.you), Primitives.Instruction.draw (.lit 1)
            (agent := Primitives.NounPhrase.you)]),
          triggeredWhile (Primitives.GameEvent.verbedEvent none (.action "Exile") (some
            thisCreature) none none)
            (Primitives.Concurrent.whileDoing (Primitives.GameEvent.activates Primitives.NounPhrase.you (a (Primitives.Predicate.abilityHead (.keyword "Craft")))))
            (Primitives.Instruction.sequentially [gainLife (.lit 1) (agent := Primitives.NounPhrase.you), Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)]) ],
      power := stat 0, toughness := stat 3 } }

/-- Escaped Shapeshifter -/
def escapedShapeshifter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Escaped Shapeshifter", cost := some [generic 3, pip .blue, pip .blue],
      types := [.creature], subtypes := [creatureType "Shapeshifter"],
      text :=
        [ Primitives.Ability.alsoForKeywords
            (Primitives.Ability.static (onlyWhile (Primitives.StaticSpec.abilityGrant thisCreature (keyword "Flying"))
              (exists_ (Primitives.Predicate.and [ creature, Primitives.Predicate.hasPossessor .controller anOpponent, Primitives.Predicate.hasKeyword (.the "Flying"),
                               Primitives.Predicate.not (Primitives.Predicate.named (Primitives.NameSource.printed "Escaped Shapeshifter")) ]))))
            [.the "FirstStrike", .the "Trample", protectionFromAnyColor] ],
      power := stat 3, toughness := stat 4 } }

/-- Ancestral Blade -/
def ancestralBlade : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ancestral Blade", cost := some [generic 1, pip .white], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ when (Primitives.GameEvent.enters thisEquipment none)
            (Primitives.Instruction.sequentially
              [ create (.lit 1) (creatureToken 1 1 [.white] [creatureType "Soldier"]),
                Primitives.Instruction.attachTo thisEquipment itAsToken ]),
          Primitives.Ability.static (getsPt (Primitives.NounPhrase.attachHost .equipped (.type .creature)) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1))),
          keywordCosting "Equip" (Primitives.Cost.mana [generic 1]) ] } }

/-- Steelclaw Lance -/
def steelclawLance : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Steelclaw Lance", cost := some [pip .black, pip .red], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ Primitives.Ability.static (getsPt (Primitives.NounPhrase.attachHost .equipped (.type .creature)) (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 2))),
          keywordQualityCosting "Equip" (Primitives.Predicate.hasSubtype (creatureType "Knight")) (Primitives.Cost.mana [generic 1]),
          keywordCosting "Equip" (Primitives.Cost.mana [generic 3]) ] } }

/-- Commander's Plate's equip lines -/
def commandersPlateEquip : List Ability :=
  [ keywordQualityCosting "Equip" (Primitives.Predicate.hasDesignation "commander" none) (Primitives.Cost.mana [generic 3]),
    keywordCosting "Equip" (Primitives.Cost.mana [generic 5]) ]
theorem okCommandersPlateEquip : Ability.checkText [] commandersPlateEquip = [] := by decide
/-- Luxior, Giada's Gift's equip lines -/
def luxiorEquipLines : List Ability :=
  [ keywordQualityCosting "Equip" (Primitives.Predicate.hasType .planeswalker) (Primitives.Cost.mana [generic 1]),
    keywordCosting "Equip" (Primitives.Cost.mana [generic 3]) ]
theorem okLuxiorEquipLines : Ability.checkText [] luxiorEquipLines = [] := by decide

/-- Veiling Oddity -/
def veilingOddity : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Veiling Oddity", cost := some [generic 3, pip .blue], types := [.creature],
      subtypes := [creatureType "Illusion"],
      text := [keywordNumberCosting "Suspend" (.lit 4) (Primitives.Cost.mana [generic 1, pip .blue]), veilingOddityLine],
      power := stat 2, toughness := stat 3 } }

/-- Enormous Energy Blade -/
def enormousEnergyBlade : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Enormous Energy Blade", cost := some [generic 2, pip .black], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ Primitives.Ability.static (getsPt (Primitives.NounPhrase.attachHost .equipped (.type .creature)) (Primitives.Delta.up (.lit 4)) (Primitives.Delta.up (.lit 0))),
          whenever (Primitives.GameEvent.attachment .attached thisEquipment (a creature)) (tap (that (.type .creature))),
          keywordCosting "Equip" (Primitives.Cost.mana [generic 2]) ] } }

/-- Grafted Wargear -/
def graftedWargear : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Grafted Wargear", cost := some [generic 3], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ Primitives.Ability.static (getsPt (Primitives.NounPhrase.attachHost .equipped (.type .creature)) (Primitives.Delta.up (.lit 3)) (Primitives.Delta.up (.lit 2))),
          whenever (Primitives.GameEvent.attachment .unattached thisEquipment (a permanent))
            (sacrifice (that .permanent) (agent := Primitives.NounPhrase.you)),
          keywordCosting "Equip" (Primitives.Cost.mana [generic 0]) ] } }

/-- Black Ward -/
def blackWard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Black Ward", cost := some [pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          Primitives.Ability.static (Primitives.StaticSpec.retention
            (Primitives.StaticSpec.abilityGrant (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (keywordQuality "Protection"
                (Primitives.Predicate.colorIs .black)))
            thisAura) ] } }

/-- Cho-Manno's Blessing -/
def choMannosBlessing : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cho-Manno's Blessing", cost := some [pip .white, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keyword "Flash",
          keywordSubject "Enchant" creature,
          Primitives.Ability.static (entersChoosing thisAura .color),
          Primitives.Ability.static (Primitives.StaticSpec.retention
            (Primitives.StaticSpec.abilityGrant (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (keywordQuality "Protection"
                (ofChosen .color)))
            thisAura) ] } }

/-- Pentarch Ward -/
def pentarchWard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Pentarch Ward", cost := some [generic 2, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          Primitives.Ability.static (entersChoosing thisAura .color),
          when (Primitives.GameEvent.enters thisAura none) (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)),
          Primitives.Ability.static (Primitives.StaticSpec.retention
            (Primitives.StaticSpec.abilityGrant (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (keywordQuality "Protection"
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
          Primitives.Ability.static (entersChoosing thisAura .color),
          Primitives.Ability.static (Primitives.StaticSpec.retention
            (Primitives.StaticSpec.abilityGrant (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (keywordQuality "Protection"
                (ofChosen .color)))
            (allOf (Primitives.Predicate.and [ Primitives.Predicate.or [Primitives.Predicate.hasSubtype (enchantmentType "Aura"), Primitives.Predicate.hasSubtype (artifactType "Equipment")],
                           Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, Primitives.Predicate.attachedTo it ]))) ] } }

/-- Wall of Shards -/
def wallOfShards : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Wall of Shards", cost := some [generic 1, pip .white], supertypes := [.snow],
      types := [.creature], subtypes := [creatureType "Wall"],
      text :=
        [ keyword "Defender", keyword "Flying",
          cumulativeUpkeep (Primitives.Cost.perform (gainLife (.lit 1) (agent := anOpponent))) ],
      power := stat 1, toughness := stat 8 } }

/-- Earthen Goo -/
def earthenGoo : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Earthen Goo", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [creatureType "Ooze"],
      text :=
        [ keyword "Trample",
          cumulativeUpkeep (Primitives.Cost.either (Primitives.Cost.mana [pip .red]) (Primitives.Cost.mana [pip .green])),
          Primitives.Ability.static (getsPt thisCreature
            (Primitives.Delta.up (times (.lit 1) (countersOn (.named "Age") it)))
            (Primitives.Delta.up (times (.lit 1) (countersOn (.named "Age") it)))) ],
      power := stat 2, toughness := stat 2 } }

/-- Mutagen Connoisseur -/
def mutagenConnoisseur : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mutagen Connoisseur", cost := some [generic 1, pip .green, pip .blue],
      types := [.creature], subtypes := [creatureType "Vedalken", creatureType "Mutant"],
      text :=
        [ keyword "Flying", keyword "Vigilance",
          Primitives.Ability.static (getsPt thisCreature
            (Primitives.Delta.up (forEach 1 (Primitives.Predicate.and [Primitives.Predicate.isTransformed, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))) (Primitives.Delta.up (.lit 0))) ],
      power := stat 0, toughness := stat 5 } }

/-- Bedrock Tortoise -/
def bedrockTortoiseWindow : StaticSpec :=
  Primitives.StaticSpec.partScope .turn (some Primitives.NounPhrase.you) (Primitives.StaticSpec.abilityGrant (allOf creatureYouControl) (keyword "Hexproof"))
theorem okBedrockTortoiseWindow : StaticSpec.check [] bedrockTortoiseWindow = [] := by decide

/-- Battlegate Mimic -/
def battlegateMimic : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Battlegate Mimic", cost := some [generic 1, hybridPip .red .white], types := [.creature],
      subtypes := [creatureType "Shapeshifter"],
      text :=
        [ whenever (Primitives.GameEvent.casts Primitives.NounPhrase.you (some (a
          (Primitives.Predicate.and [spell, Primitives.Predicate.colorIs .red,
          Primitives.Predicate.colorIs .white]))) none)
            (Primitives.Instruction.establish
              (Primitives.StaticSpec.conjunction none
                [ Primitives.StaticSpec.modification thisCreature .power (Primitives.Delta.set (.lit 4)),
                  Primitives.StaticSpec.modification (itsOther thisCreature (Primitives.Delta.set (.lit 4))) .toughness (Primitives.Delta.set (.lit 2)),
                  Primitives.StaticSpec.abilityGrant thisCreature (keyword "FirstStrike") ])
              (some untilEndOfTurn)) ],
      power := stat 2, toughness := stat 1 } }

/-- Sphinx of Uthuun -/
def sphinxOfUthuun : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sphinx of Uthuun", cost := some [generic 5, pip .blue, pip .blue], types := [.creature],
      subtypes := [creatureType "Sphinx"],
      text :=
        [ keyword "Flying",
          when (Primitives.GameEvent.enters thisCreature none)
            (Primitives.Instruction.sequentially
              [ revealCards (topSlice (.lit 5)),
                Primitives.Instruction.separateIntoPiles them 2 [] (agent := anOpponent),
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
          when (Primitives.GameEvent.dies thisCreature)
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
          whenever (Primitives.GameEvent.casts Primitives.NounPhrase.you (some (a
            (Primitives.Predicate.and [historic, spell]))) none) (scry (.lit 1) (agent :=
            Primitives.NounPhrase.you))
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
            (dealsCombatDamage (a (Primitives.Predicate.and [historic, creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) (a Primitives.Predicate.anyPlayer))
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
          Primitives.Ability.static (Primitives.StaticSpec.abilityGrant (Primitives.NounPhrase.attachHost .enchanted .permanent) (keyword "Indestructible")) ] }
              }

/-- Seraphic Greatsword -/
def seraphicGreatsword : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Seraphic Greatsword", cost := some [generic 1, pip .white], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ Primitives.Ability.static (getsPt (Primitives.NounPhrase.attachHost .equipped (.type .creature)) (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 2))),
          whenever
            (attacksPlayer (Primitives.NounPhrase.attachHost .equipped (.type .creature))
              (the (Primitives.Predicate.and [Primitives.Predicate.anyPlayer, Primitives.Predicate.superlative .max (.playerStat .lifeTotal) Primitives.Predicate.anyPlayer])))
            (Primitives.Instruction.create (.lit 1)
              (Primitives.TokenSpec.written
                { characteristics :=
                  { colors := [.white], types := [.creature], subtypes := [creatureType "Angel"],
                    text := [keyword "Flying"], power := stat 4, toughness := stat 4 } })
              [Primitives.TokenRider.entersAs .tapped, Primitives.TokenRider.entersAttacking (some (that .player))] (agent := Primitives.NounPhrase.you)),
          keywordCosting "Equip" (Primitives.Cost.mana [generic 4]) ] } }

def giantGrowth : Instruction :=
  get (target creature) (Primitives.Delta.up (.lit 3)) (Primitives.Delta.up (.lit 3)) (some untilEndOfTurn)
theorem okGiantGrowth : Instruction.check [] giantGrowth = [] := by decide
def amassZombiesTwo : Instruction := amass "Zombie" 2
theorem okAmassZombiesTwo : Instruction.check [] amassZombiesTwo = [] := by decide
/-- Avarice Totem -/
def avariceTotemExchange : Instruction :=
  Primitives.Instruction.exchange (Primitives.Exchanged.controlOf thisArtifact (target (Primitives.Predicate.and [permanent, Primitives.Predicate.not land])))
theorem okAvariceTotemExchange : Instruction.check [] avariceTotemExchange = [] := by decide
/-- Arcanum Wings -- aura swap [CR#702.65a] -/
def arcanumWingsAuraSwap : Ability :=
  activated (Primitives.Cost.mana [generic 2, pip .blue])
    (offer
      (Primitives.Instruction.exchange (Primitives.Exchanged.cardsAcross thisAura (a (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (enchantmentType "Aura"), Primitives.Predicate.inZone
          hand])))) (agent := Primitives.NounPhrase.you))
theorem okArcanumWingsAuraSwap : Ability.check [] arcanumWingsAuraSwap = [] := by decide
/-- Tovolar, Dire Overlord -/
def tovolarNightfall : Instruction := Primitives.Instruction.setGameDesignation "night"
theorem okTovolarNightfall : Instruction.check [] tovolarNightfall = [] := by decide
/-- Spin into Myth -/
def spinIntoMyth : Instruction :=
  Primitives.Instruction.sequentially [move (target creature) onTop, fateseal anOpponent (.lit 2) (agent := Primitives.NounPhrase.you)]
theorem okSpinIntoMyth : Instruction.check [] spinIntoMyth = [] := by decide
/-- Pure // Simple -/
def pureHalf : Instruction := destroy (target (Primitives.Predicate.and [permanent, multicolored]))
theorem okPureHalf : Instruction.check [] pureHalf = [] := by decide
/-- Korlash -/
def grandeurDiscardCost : Cost :=
  Primitives.Cost.perform (discard
    (a (Primitives.Predicate.and [Primitives.Predicate.named (Primitives.NameSource.printed "Korlash, Heir to Blackblade"), Primitives.Predicate.otherThan Primitives.NounPhrase.this, Primitives.Predicate.inZone hand]))
        (agent := Primitives.NounPhrase.you))
theorem okGrandeurDiscardCost : Cost.check [] grandeurDiscardCost = [] := by decide
/-- Gyruda, Doom of Depths -/
def gyrudaCompanion : Ability := companion (Primitives.DeckCondition.everyCardIs Primitives.Predicate.isCard (Primitives.DeckTrait.manaValueParity .even))
theorem okGyrudaCompanion : Ability.check [] gyrudaCompanion = [] := by decide
/-- Jegantha, the Wellspring -/
def jeganthaCompanion : Ability := companion (Primitives.DeckCondition.noCardIs Primitives.Predicate.isCard Primitives.DeckTrait.repeatedManaSymbol)
theorem okJeganthaCompanion : Ability.check [] jeganthaCompanion = [] := by decide
/-- Kaheera, the Orphanguard -/
def kaheeraCompanion : Ability :=
  companion
    (Primitives.DeckCondition.everyCardIs (Primitives.Predicate.and [creature, Primitives.Predicate.isCard])
      (Primitives.DeckTrait.anyOf [ Primitives.DeckTrait.aCharacteristic (Primitives.Predicate.hasSubtype (creatureType "Cat")),
                Primitives.DeckTrait.aCharacteristic (Primitives.Predicate.hasSubtype (creatureType "Elemental")),
                Primitives.DeckTrait.aCharacteristic (Primitives.Predicate.hasSubtype (creatureType "Nightmare")),
                Primitives.DeckTrait.aCharacteristic (Primitives.Predicate.hasSubtype (creatureType "Dinosaur")),
                Primitives.DeckTrait.aCharacteristic (Primitives.Predicate.hasSubtype (creatureType "Beast")) ]))
theorem okKaheeraCompanion : Ability.check [] kaheeraCompanion = [] := by decide
/-- Keruga, the Macrosage -/
def kerugaCompanion : Ability :=
  companion
    (Primitives.DeckCondition.everyCardIs Primitives.Predicate.isCard
      (Primitives.DeckTrait.anyOf [ Primitives.DeckTrait.aCharacteristic (Primitives.Predicate.compare [.stat .manaValue] .atLeast (.lit 3)),
                Primitives.DeckTrait.aCharacteristic land ]))
theorem okKerugaCompanion : Ability.check [] kerugaCompanion = [] := by decide
/-- Lurrus of the Dream-Den -/
def lurrusCompanion : Ability :=
  companion
    (Primitives.DeckCondition.everyCardIs permanentCard (Primitives.DeckTrait.aCharacteristic (Primitives.Predicate.compare [.stat .manaValue] .atMost (.lit 2))))
theorem okLurrusCompanion : Ability.check [] lurrusCompanion = [] := by decide
/-- Lutri, the Spellchaser -/
def lutriCompanion : Ability := companion (Primitives.DeckCondition.cardsDiffer (Primitives.Predicate.and [Primitives.Predicate.not land, Primitives.Predicate.isCard]) .cardName)
theorem okLutriCompanion : Ability.check [] lutriCompanion = [] := by decide
/-- Obosh, the Preypiercer -/
def oboshCompanion : Ability :=
  companion (Primitives.DeckCondition.everyCardIs Primitives.Predicate.isCard (Primitives.DeckTrait.anyOf [Primitives.DeckTrait.manaValueParity .odd, Primitives.DeckTrait.aCharacteristic land]))
theorem okOboshCompanion : Ability.check [] oboshCompanion = [] := by decide
/-- Umori, the Collector -/
def umoriCompanion : Ability := companion (Primitives.DeckCondition.cardsShare (Primitives.Predicate.and [Primitives.Predicate.not land, Primitives.Predicate.isCard]) .cardType)
theorem okUmoriCompanion : Ability.check [] umoriCompanion = [] := by decide
/-- Yorion, Sky Nomad -/
def yorionCompanion : Ability := companion (Primitives.DeckCondition.deckSizeOverMinimum 20)
theorem okYorionCompanion : Ability.check [] yorionCompanion = [] := by decide
/-- Zirda, the Dawnwaker -/
def zirdaCompanion : Ability :=
  companion (Primitives.DeckCondition.everyCardIs permanentCard (Primitives.DeckTrait.hasAbilityOf .anyActivated))
theorem okZirdaCompanion : Ability.check [] zirdaCompanion = [] := by decide

/-- Deadpool, Trading Card -/
def deadpoolTradingCard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Deadpool, Trading Card", cost := some [generic 2, pip .black, pip .red],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Mutant", creatureType "Mercenary", creatureType "Hero"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.replacement (Primitives.GameEvent.enters thisCreature none) [] none
            (offer (Primitives.Instruction.exchange (Primitives.Exchanged.textBoxes thisCreature (a (Primitives.Predicate.and [creature, Primitives.Predicate.otherThan Primitives.NounPhrase.this]))))
                (agent := Primitives.NounPhrase.you))
            .nextTimeOnly none),
          at_ (Primitives.GameEvent.beginningOf .the .upkeep (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you)) (loseLife (.lit 3) (agent := Primitives.NounPhrase.you)),
          activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 3], Primitives.Cost.perform (sacrifice thisCreature (agent :=
              Primitives.NounPhrase.you))])
            (Primitives.Instruction.draw (.lit 1) (agent := (each otherPlayer))) ],
      power := stat 5, toughness := stat 3 } }

end Semantics.Cards

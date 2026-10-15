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

def jump : Instruction := gain (target creature) (keyword (.named "flying")) (some untilEndOfTurn)
theorem okJump : Instruction.check (DRS.ofList []) jump = [] := by decide
def gabrielAngelfire : Instruction :=
  Primitives.Instruction.sequentially
    [ choose (a (qualityFrom .ability
        (Primitives.ChoiceDomain.abilitiesAmong [.the (.named "flying"), .the (.named "firstStrike"), .the (.named "trample"), .theWith (.named "rampage") 3]))),
      gain thisCreature (Primitives.Ability.thatAbility .theChoice) (some (untilEvent (Primitives.GameEvent.beginningOf .upkeep (some Primitives.NounPhrase.you) (Primitives.TurnRef.next none)))) ]
theorem okGabrielAngelfire : Instruction.check (DRS.ofList []) gabrielAngelfire = [] := by decide
def builtToSmash : Instruction :=
  Primitives.Instruction.sequentially
    [ get (target (Primitives.Predicate.and [creature, attacking])) (Primitives.Delta.up (.lit 3)) (Primitives.Delta.up (.lit 3)) (some untilEndOfTurn),
      Primitives.Instruction.doIf (Primitives.Condition.matches it (Primitives.Predicate.and [artifact, creature]))
        (gain it (keyword (.named "trample")) (some untilEndOfTurn)) none ]
theorem okBuiltToSmash : Instruction.check (DRS.ofList []) builtToSmash = [] := by decide

/-- A flying Thopter token. -/
def thopterToken : CharacteristicBundle :=
  { characteristics :=
    { types := [.artifact, .creature], subtypes := [creatureType "Thopter"],
      text := [keyword (.named "flying")], power := stat 1, toughness := stat 1 } }

def aviationPioneer : Instruction := create (.lit 1) thopterToken
theorem okAviationPioneer : Instruction.check (DRS.ofList []) aviationPioneer = [] := by decide
def fireNavyTrebuchet : Instruction :=
  createTappedAttacking (.lit 1)
    { characteristics :=
      { name := "Ballistic Boulder", types := [.artifact, .creature],
        subtypes := [creatureType "Construct"], text := [keyword (.named "flying")],
        power := stat 2, toughness := stat 1 } }
theorem okFireNavyTrebuchet : Instruction.check (DRS.ofList []) fireNavyTrebuchet = [] := by decide

def rorixBladewing : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rorix Bladewing", cost := some [generic 3, pip .red, pip .red, pip .red],
      supertypes := [.legendary], types := [.creature], subtypes := [creatureType "Dragon"],
      text := [keyword (.named "flying"), keyword (.named "haste")], power := stat 6, toughness := stat 5 } }

def yotianSoldier : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Yotian Soldier", cost := some [generic 3], types := [.artifact, .creature],
      subtypes := [creatureType "Soldier"], text := [keyword (.named "vigilance")],
      power := stat 1, toughness := stat 4 } }

/-- Pym Particles -/
def pymParticlesGrant : Instruction :=
  Primitives.Instruction.sequentially
    [ gain (target creature) (keyword (.named "vigilance")) (some untilEndOfTurn),
      Primitives.Instruction.establish (Primitives.StaticSpec.cant Primitives.Subject.any (Primitives.Instruction.block (that (.type .creature))) { rider := Primitives.DeonticRider.noRider })
        (some thisTurn) ]
theorem okPymParticlesGrant : Instruction.check (DRS.ofList []) pymParticlesGrant = [] := by decide
def bladebrand : Instruction := gain (target creature) (keyword (.named "deathtouch")) (some untilEndOfTurn)
theorem okBladebrand : Instruction.check (DRS.ofList []) bladebrand = [] := by decide
def criticalHit : Instruction :=
  gain (target creature) (keyword (.named "doubleStrike")) (some untilEndOfTurn)
theorem okCriticalHit : Instruction.check (DRS.ofList []) criticalHit = [] := by decide
def lightningBlow : Instruction :=
  gain (target creature) (keyword (.named "firstStrike")) (some untilEndOfTurn)
theorem okLightningBlow : Instruction.check (DRS.ofList []) lightningBlow = [] := by decide
def deathByDragons : Instruction :=
  instruct (each (Primitives.Predicate.and [Primitives.Predicate.anyPlayer, Primitives.Predicate.otherThan (target Primitives.Predicate.anyPlayer)])) (Primitives.Instruction.create (.lit 1)
    (Primitives.TokenSpec.written
      { characteristics :=
        { colors := [.red], types := [.creature], subtypes := [creatureType "Dragon"],
          text := [keyword (.named "flying")], power := stat 5, toughness := stat 5 } })
    [])
theorem okDeathByDragons : Instruction.check (DRS.ofList []) deathByDragons = [] := by decide

/-- Concordant Crossroads -/
def concordantCrossroads : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Concordant Crossroads", cost := some [pip .green], supertypes := [.world],
      types := [.enchantment], text := [Primitives.Ability.static (Primitives.StaticSpec.abilityGrant (allOf creature) (keyword (.named "haste")))]
          } }

/-- Bristlepack Sentry -/
def bristlepackSentry : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bristlepack Sentry", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [creatureType "Plant", creatureType "Wolf"],
      text :=
        [ keyword (.named "defender"),
          Primitives.Ability.static (onlyWhile
            (canDoAsThough thisCreature (.core .attack) (Primitives.Predicate.not (Primitives.Predicate.hasKeyword (.the (.named "defender")))))
            (exists_ (Primitives.Predicate.and [ creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you,
                             Primitives.Predicate.compare [.stat .power] .atLeast (.lit 4) ]))) ],
      power := stat 3, toughness := stat 3 } }

def platinumAngel : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Platinum Angel", cost := some [generic 7], types := [.artifact, .creature],
      subtypes := [creatureType "Angel"],
      text :=
        [ keyword (.named "flying"),
          Primitives.Ability.static (playerCant (.core .loseGame) Primitives.NounPhrase.you),
          Primitives.Ability.static (playerCant (.core .winGame) (Primitives.NounPhrase.playerGroup .yourOpponents)) ],
      power := stat 4, toughness := stat 4 } }

def abyssalPersecutor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Abyssal Persecutor", cost := some [generic 2, pip .black, pip .black],
      types := [.creature], subtypes := [creatureType "Demon"],
      text :=
        [ keyword (.named "flying"), keyword (.named "trample"),
          Primitives.Ability.static (playerCant (.core .winGame) Primitives.NounPhrase.you),
          Primitives.Ability.static (playerCant (.core .loseGame) (Primitives.NounPhrase.playerGroup .yourOpponents)) ],
      power := stat 6, toughness := stat 6 } }

def smogElemental : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Smog Elemental", cost := some [generic 4, pip .black, pip .black],
      types := [.creature], subtypes := [creatureType "Elemental"],
      text :=
        [ keyword (.named "flying"),
          Primitives.Ability.static (getsPt
            (allOf (Primitives.Predicate.and [ creature, Primitives.Predicate.hasKeyword (.the (.named "flying")),
                           Primitives.Predicate.hasPossessor .controller (Primitives.NounPhrase.playerGroup .yourOpponents) ]))
            (Primitives.Delta.down (.lit 1)) (Primitives.Delta.down (.lit 1))) ],
      power := stat 3, toughness := stat 3 } }

def exquisiteArchangel : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Exquisite Archangel", cost := some [generic 5, pip .white, pip .white],
      types := [.creature], subtypes := [creatureType "Angel"],
      text :=
        [ keyword (.named "flying"),
          Primitives.Ability.static (Primitives.StaticSpec.instead (Primitives.WouldClause.would (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Instruction.conclude .loseGame)) (Primitives.Instruction.sequentially
              [ exile thisCreature,
                setLife (Primitives.Amount.statOf (.playerStat .startingLifeTotal) Primitives.NounPhrase.you) ]) none .repeatedly none) ],
      power := stat 5, toughness := stat 5 } }

def adelbertSteiner : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Adelbert Steiner", cost := some [generic 1, pip .white], supertypes := [.legendary],
      types := [.creature], subtypes := [creatureType "Human", creatureType "Knight"],
      text :=
        [ keyword (.named "lifelink"),
          Primitives.Ability.static (getsPt thisCreature
            (Primitives.Delta.up (forEach 1 (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (artifactType "Equipment"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))
            (Primitives.Delta.up (forEach 1 (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (artifactType "Equipment"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))) ],
      power := stat 2, toughness := stat 1 } }

def inspiringStatuary : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Inspiring Statuary", cost := some [generic 3], types := [.artifact],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.abilityGrant (allOf (Primitives.Predicate.and [Primitives.Predicate.not artifact, spell, castBy Primitives.NounPhrase.you])) (keyword
            (.named "improvise"))) ] } }

def chiefEngineer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chief Engineer", cost := some [generic 1, pip .blue], types := [.creature],
      subtypes := [creatureType "Vedalken", creatureType "Artificer"],
      text := [Primitives.Ability.static (Primitives.StaticSpec.abilityGrant (allOf (Primitives.Predicate.and [artifact, spell, castBy Primitives.NounPhrase.you])) (keyword
          (.named "convoke")))],
      power := stat 1, toughness := stat 3 } }

def firesongAndSunspeaker : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.abilityGrant
    (allOf (Primitives.Predicate.and [Primitives.Predicate.colorIs .red, instantOrSorcery, spell, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
    (keyword (.named "lifelink")))
theorem okFiresongAndSunspeaker : Ability.check (DRS.ofList []) firesongAndSunspeaker = [] := by decide
def briaRiptideRogue : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.abilityGrant (allOf (otherCreatureYouControl thisCreature)) (keyword (.named "prowess")))
theorem okBriaRiptideRogue : Ability.check (DRS.ofList []) briaRiptideRogue = [] := by decide
def narsetEnlightenedExile : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.abilityGrant (allOf creatureYouControl) (keyword (.named "prowess")))
theorem okNarsetEnlightenedExile : Ability.check (DRS.ofList []) narsetEnlightenedExile = [] := by decide
def pontiffOfBlight : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.abilityGrant (allOf (otherCreatureYouControl thisCreature)) (keyword (.named "extort")))
theorem okPontiffOfBlight : Ability.check (DRS.ofList []) pontiffOfBlight = [] := by decide

def prismariTheInspiration : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Prismari, the Inspiration", cost := some [generic 5, pip .blue, pip .red],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Elder", creatureType "Dragon"],
      text :=
        [ keyword (.named "flying"),
          keywordCosting (.named "ward") (payLife 5),
          Primitives.Ability.static (Primitives.StaticSpec.abilityGrant (allOf (Primitives.Predicate.and [instantOrSorcery, spell, castBy Primitives.NounPhrase.you])) storm) ],
      power := stat 7, toughness := stat 7 } }

def tomakulHonorGuard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tomakul Honor Guard", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Soldier"],
      text := [keywordCosting (.named "ward") (Primitives.Cost.mana [generic 2])], power := stat 3, toughness := stat 1 } }

def repentantBlacksmith : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Repentant Blacksmith", cost := some [generic 1, pip .white], types := [.creature],
      subtypes := [creatureType "Human"],
      text := [keywordQuality (.named "protection") (Primitives.Predicate.colorIs .red)], power := stat 1, toughness := stat 2 } }

def battleSquadron : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Battle Squadron", cost := some [generic 3, pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Goblin"],
      text :=
        [ keyword (.named "flying"),
          Primitives.Ability.static (Primitives.StaticSpec.ptDefinition thisCreature .bothEach (countOf creatureYouControl)) ] } }

def eomerOfTheRiddermark : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Éomer of the Riddermark", cost := some [generic 4, pip .red],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Knight"],
      text :=
        [ keyword (.named "haste"),
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
        [ keywordSubject (.named "enchant") creature,
          when (Primitives.GameEvent.enters thisAura none) (Primitives.Instruction.draw (.lit 1)),
          Primitives.Ability.static (Primitives.StaticSpec.abilityGrant (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (keyword (.named "flying"))) ] }
              }

def consulsLieutenant : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Consul's Lieutenant", cost := some [pip .white, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Soldier"],
      text :=
        [ keyword (.named "firstStrike"),
          renown 1,
          triggeredIf (attacks thisCreature) (Primitives.Condition.matches it (Primitives.Predicate.hasDesignation (.named "renowned") none))
            (get (allOf (Primitives.Predicate.and [creature, attacking, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, Primitives.Predicate.otherThan
                thisCreature]))
              (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1)) (some untilEndOfTurn)) ],
      power := stat 2, toughness := stat 1 } }

def secretsOfTheGoldenCity : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Secrets of the Golden City", cost := some [generic 1, pip .blue, pip .blue],
      types := [.sorcery],
      text :=
        [ keyword (.named "ascend"),
          Primitives.Ability.spell none (Primitives.Instruction.replace (Primitives.Instruction.draw (.lit 2))
            (Primitives.Instruction.doIf (Primitives.Condition.matches Primitives.NounPhrase.you (Primitives.Predicate.hasDesignation (.named "citysBlessing") none)) (Primitives.Instruction.draw (.lit 3))
              none)) ] } }

def bombur : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bombur, Gentle Dreamer", cost := some [generic 2, pip .red],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Dwarf", creatureType "Bard"],
      text :=
        [ keyword (.named "storied"),
          Primitives.Ability.static (onlyUnless (doesntUntap thisCreature (some Primitives.NounPhrase.you))
            (Primitives.Condition.matches Primitives.NounPhrase.you (Primitives.Predicate.hasDesignation (.named "enduringStory") none))) ],
      power := stat 5, toughness := stat 3 } }

def drachNyen : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Drach'Nyen", cost := some [generic 4, pip .black, pip .red],
      supertypes := [.legendary], types := [.artifact], subtypes := [artifactType "Equipment"],
      text :=
        [ when (Primitives.GameEvent.enters thisEquipment none) (exile (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 1)) creature)),
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ Primitives.StaticSpec.abilityGrant (Primitives.NounPhrase.attachHost .equipped (.type .creature)) (keyword (.named "menace")),
              getsPt (Primitives.NounPhrase.attachHost .equipped (.type .creature)) (Primitives.Delta.up (Primitives.Amount.letter .x)) (Primitives.Delta.up (.lit 0)),
              Primitives.StaticSpec.letterDefinition .x (Primitives.Amount.statOf (.stat .power) (the (Primitives.Predicate.exiledWith thisEquipment))) ]),
          keywordCosting (.named "equip") (Primitives.Cost.mana [generic 2]) ] } }

def colossalGraveReaver : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Colossal Grave-Reaver", cost := some [generic 6, pip .black, pip .green],
      types := [.creature], subtypes := [creatureType "Dragon"],
      text :=
        [ keyword (.named "flying"),
          triggeredOr (Primitives.GameEvent.enters thisCreature none) [attacks thisCreature] (mill (.lit 3) Primitives.NounPhrase.you),
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
            (Primitives.EntryRider.withCounters (.lit 2)
              (Primitives.CounterKindSource.distinctChosen [.named "menaceCounter", .named "deathtouchCounter", .named "lifelinkCounter"]) .fresh)) ],
      power := stat 3, toughness := stat 3 } }

def divineVisitation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Divine Visitation", cost := some [generic 3, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.instead (Primitives.WouldClause.would Primitives.Subject.any (Primitives.Instruction.createObject Primitives.Amount.anyAmount (Primitives.CreationSpec.token (Primitives.TokenSpec.copyOf (counted (atLeast 1) (Primitives.Predicate.and [creature, Primitives.Predicate.isToken])) []) []) false (some Primitives.NounPhrase.you))) (create Primitives.Amount.groupSize
              { characteristics :=
                { colors := [.white], types := [.creature], subtypes := [creatureType "Angel"],
                  text := [keyword (.named "flying"), keyword (.named "vigilance")], power := stat 4, toughness := stat 4 } }) none .repeatedly none) ] } }

def adrixAndNev : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Adrix and Nev, Twincasters", cost := some [generic 2, pip .green, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Merfolk", creatureType "Wizard"],
      text :=
        [ keywordCosting (.named "ward") (Primitives.Cost.mana [generic 2]),
          Primitives.Ability.static (Primitives.StaticSpec.instead (Primitives.WouldClause.would Primitives.Subject.any (Primitives.Instruction.createObject Primitives.Amount.anyAmount (Primitives.CreationSpec.token (Primitives.TokenSpec.copyOf (counted (atLeast 1) Primitives.Predicate.isToken) []) []) false (some
              Primitives.NounPhrase.you))) (Primitives.Instruction.create (times (.lit 2) Primitives.Amount.groupSize) Primitives.TokenSpec.asThose []) none .repeatedly none) ],
      power := stat 2, toughness := stat 2 } }

def windZendikon : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Wind Zendikon", cost := some [pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") land,
          Primitives.Ability.static (Primitives.StaticSpec.qualityChange (Primitives.NounPhrase.attachHost .enchanted (.type .land)) .sets
            (Primitives.QualityPayload.bundle
              { characteristics :=
                { colors := [.blue], types := [.creature], subtypes := [creatureType "Elemental"],
                  text := [keyword (.named "flying")], power := stat 2, toughness := stat 2 } }
              (some .land))),
          when (Primitives.GameEvent.dies (Primitives.NounPhrase.attachHost .enchanted (.type .land))) (move (that .card) hand) ] } }

def awakenTheBear : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Awaken the Bear", cost := some [generic 2, pip .green], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.establish
            (Primitives.StaticSpec.conjunction none
              [ getsPt (target creature) (Primitives.Delta.up (.lit 3)) (Primitives.Delta.up (.lit 3)),
                Primitives.StaticSpec.abilityGrant it (keyword (.named "trample")) ])
            (some untilEndOfTurn)) ] } }

def spidersilkArmor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Spidersilk Armor", cost := some [generic 2, pip .green], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ getsPt (allOf creatureYouControl) (Primitives.Delta.up (.lit 0)) (Primitives.Delta.up
                  (.lit 1)),
              Primitives.StaticSpec.abilityGrant them (keyword (.named "reach")) ]) ] } }

def arcaneFlight : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Arcane Flight", cost := some [pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") creature,
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ getsPt (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1)),
              Primitives.StaticSpec.abilityGrant it (keyword (.named "flying")) ]) ] } }

def bootsOfSpeed : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Boots of Speed", cost := some [pip .red], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ getsPt (Primitives.NounPhrase.attachHost .equipped (.type .creature)) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 0)),
              Primitives.StaticSpec.abilityGrant it (keyword (.named "haste")) ]),
          keywordCosting (.named "equip") (Primitives.Cost.mana [generic 1]) ] } }

/-- "enchanted creature loses all abilities and is a <colors> <subtype> with base P/T" -/
def enchantedBecomesVanilla (colors : List Color) (subtype : String) (power toughness : Nat) :
    StaticSpec :=
  Primitives.StaticSpec.conjunction none
    [ Primitives.StaticSpec.allAbilityLoss (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) none,
      Primitives.StaticSpec.qualityChange it .sets
        (Primitives.QualityPayload.bundle { characteristics := { colors, types := [.creature], subtypes := [creatureType subtype] } }
          none),
      getsPt it (Primitives.Delta.set (.lit power)) (Primitives.Delta.set (.lit toughness)) ]

def frogify : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Frogify", cost := some [generic 1, pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") creature,
          Primitives.Ability.static (enchantedBecomesVanilla [.blue] "Frog" 1 1) ] } }

def darksteelMutation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Darksteel Mutation", cost := some [generic 1, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") creature,
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ Primitives.StaticSpec.qualityChange (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) .sets
                (Primitives.QualityPayload.bundle { characteristics := { types := [.artifact, .creature], subtypes := [creatureType "Insect"] } }
                  none),
              getsPt it (Primitives.Delta.set (.lit 0)) (Primitives.Delta.set (.lit 1)),
              Primitives.StaticSpec.abilityGrant it (keyword (.named "indestructible")),
              Primitives.StaticSpec.allAbilityLoss it none ]) ] } }

def kenrithsTransformation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Kenrith's Transformation", cost := some [generic 1, pip .green],
      types := [.enchantment], subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") creature,
          when (Primitives.GameEvent.enters thisAura none) (Primitives.Instruction.draw (.lit 1)),
          Primitives.Ability.static (enchantedBecomesVanilla [.green] "Elk" 3 3) ] } }

def amphibianDownpour : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Amphibian Downpour", cost := some [generic 2, pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keyword (.named "flash"), storm,
          keywordSubject (.named "enchant") creature,
          Primitives.Ability.static (enchantedBecomesVanilla [.blue] "Frog" 1 1) ] } }

def lignify : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lignify", cost := some [generic 1, pip .green], types := [.kindred, .enchantment],
      subtypes := [creatureType "Treefolk", enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") creature,
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ Primitives.StaticSpec.qualityChange (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) .sets
                (Primitives.QualityPayload.bundle { characteristics := { subtypes := [creatureType "Treefolk"] } } none),
              getsPt it (Primitives.Delta.set (.lit 0)) (Primitives.Delta.set (.lit 4)),
              Primitives.StaticSpec.allAbilityLoss it none ]) ] } }

def nefariousImp : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Nefarious Imp", cost := some [generic 2, pip .black], types := [.creature],
      subtypes := [creatureType "Imp"],
      text :=
        [ keyword (.named "flying"),
          whenever
            (leavesBattlefield (counted (atLeast 1) (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))
            (scry (.lit 1)) ],
      power := stat 2, toughness := stat 1 } }

/-- Ainok Tracker -/
def ainokTracker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ainok Tracker", cost := some [generic 5, pip .red], types := [.creature],
      subtypes := [creatureType "Dog", creatureType "Scout"],
      text := [keyword (.named "firstStrike"), keywordCosting (.named "morph") (Primitives.Cost.mana [generic 4, pip .red])],
      power := stat 3, toughness := stat 3 } }

def irenicussVileDuplication : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Irenicus's Vile Duplication", cost := some [generic 3, pip .blue], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.create (.lit 1)
            (Primitives.TokenSpec.copyOf (target (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
              [Primitives.CopyExcept.ability (keyword (.named "flying")), Primitives.CopyExcept.nonlegendary])
            []) ] } }

def cacklingCounterpart : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cackling Counterpart", cost := some [generic 1, pip .blue, pip .blue],
      types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.create (.lit 1)
            (Primitives.TokenSpec.copyOf (target (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) []) []),
          keywordCosting (.named "flashback") (Primitives.Cost.mana [generic 5, pip .blue, pip .blue]) ] } }

def chandrasSpitfire : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chandra's Spitfire", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text :=
        [ keyword (.named "flying"),
          whenever (Primitives.GameEvent.isDealtDamage .noncombatOnly (a Primitives.Predicate.opponent))
            (get thisCreature (Primitives.Delta.up (.lit 3)) (Primitives.Delta.up (.lit 0)) (some untilEndOfTurn)) ],
      power := stat 1, toughness := stat 3 } }

def livingHive : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Living Hive", cost := some [generic 6, pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Elemental", creatureType "Insect"],
      text :=
        [ keyword (.named "trample"),
          whenever (dealsCombatDamage thisCreature (a Primitives.Predicate.anyPlayer))
            (create Primitives.Amount.thatMuch (creatureToken 1 1 [.green] [creatureType "Insect"])) ],
      power := stat 6, toughness := stat 6 } }

def giantCindermaw : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Giant Cindermaw", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [creatureType "Dinosaur", creatureType "Beast"],
      text := [keyword (.named "trample"), Primitives.Ability.static (playerCant (.core .gainLife) (Primitives.NounPhrase.playerGroup .allPlayers))],
      power := stat 4, toughness := stat 3 } }

def lushGrowth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lush Growth", cost := some [pip .green], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") land,
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
        [ keyword (.named "flying"),
          Primitives.Ability.static (entersChoosing thisCreature .color),
          Primitives.Ability.static (Primitives.StaticSpec.abilityGrant thisCreature (keywordQuality (.named "protection") (ofChosen .color))) ],
      power := stat 2, toughness := stat 2 } }

def wardSliver : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ward Sliver", cost := some [generic 4, pip .white], types := [.creature],
      subtypes := [creatureType "Sliver"],
      text :=
        [ Primitives.Ability.static (entersChoosing thisCreature .color),
          Primitives.Ability.static (Primitives.StaticSpec.abilityGrant (allOf (Primitives.Predicate.hasSubtype (creatureType "Sliver")))
            (keywordQuality (.named "protection") (ofChosen .color))) ],
      power := stat 2, toughness := stat 2 } }

def sanctuaryBlade : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sanctuary Blade", cost := some [generic 2], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ Primitives.Ability.static (attachChoosing thisEquipment .color),
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ getsPt (Primitives.NounPhrase.attachHost .equipped (.type .creature)) (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 0)),
              Primitives.StaticSpec.abilityGrant (Primitives.NounPhrase.attachHost .equipped (.type .creature))
                (keywordQuality (.named "protection") (ofTheLastChosen .color)) ]),
          keywordCosting (.named "equip") (Primitives.Cost.mana [generic 3]) ] } }

/-- Sinister Strength -/
def sinisterStrength : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sinister Strength", cost := some [generic 1, pip .black], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") creature,
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ getsPt (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (Primitives.Delta.up (.lit 3)) (Primitives.Delta.up (.lit 1)),
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
                    Primitives.StaticSpec.abilityGrant it (keyword (.named "haste")) ])
                (some untilEndOfTurn),
              Primitives.Instruction.draw (.lit 1) ]) ] } }

/-- Ghoulflesh -/
def ghoulflesh : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ghoulflesh", cost := some [pip .black], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") creature,
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ getsPt (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (Primitives.Delta.down (.lit 1)) (Primitives.Delta.down (.lit 1)),
              Primitives.StaticSpec.qualityChange it .adds
                (Primitives.QualityPayload.bundle { characteristics := { colors := [.black], subtypes := [creatureType "Zombie"] } } none) ]) ] } }

/-- Blade of the Oni -/
def bladeOfTheOniStatic : StaticSpec :=
  Primitives.StaticSpec.conjunction none
    [ getsPt (Primitives.NounPhrase.attachHost .equipped (.type .creature)) (Primitives.Delta.set (.lit 5)) (Primitives.Delta.set (.lit 5)),
      Primitives.StaticSpec.abilityGrant it (keyword (.named "menace")),
      Primitives.StaticSpec.qualityChange it .adds
        (Primitives.QualityPayload.bundle { characteristics := { colors := [.black], subtypes := [creatureType "Demon"] } } none) ]
theorem okBladeOfTheOniStatic : StaticSpec.check (DRS.ofList []) bladeOfTheOniStatic = [] := by decide

def historyOfBenalia : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "History of Benalia", cost := some [generic 1, pip .white, pip .white],
      types := [.enchantment], subtypes := [enchantmentType "Saga"],
      text :=
        [ when (Primitives.GameEvent.chapterMark [1, 2])
            (create (.lit 1)
              { characteristics :=
                { colors := [.white], types := [.creature], subtypes := [creatureType "Knight"],
                  text := [keyword (.named "vigilance")], power := stat 2, toughness := stat 2 } }),
          when (Primitives.GameEvent.chapterMark [3])
            (get (allOf (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (creatureType "Knight"), Primitives.Predicate.hasPossessor .controller
                Primitives.NounPhrase.you]))
              (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 1)) (some untilEndOfTurn)) ] } }

/-- Akroan Sergeant -/
def akroanSergeant : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Akroan Sergeant", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Soldier"],
      text := [keyword (.named "firstStrike"), renown 1], power := stat 2, toughness := stat 2 } }

def castCreatureSpellsFromTop : StaticSpec :=
  mayPlayDeed (.action "cast") Primitives.NounPhrase.you (allOf (Primitives.Predicate.and [spell, creature])) none
    (Primitives.DeonticRider.play (some onTop) none none false Primitives.PlayPayment.itsOwnCost)
theorem okCastCreatureSpellsFromTop : StaticSpec.check (DRS.ofList []) castCreatureSpellsFromTop = [] := by decide

def garruksHorde : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Garruk's Horde", cost := some [generic 5, pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Beast"],
      text :=
        [ keyword (.named "trample"),
          Primitives.Ability.static (Primitives.StaticSpec.visibility .reveal Primitives.NounPhrase.you Primitives.VisibleThing.topOfLibrary),
          Primitives.Ability.static castCreatureSpellsFromTop ],
      power := stat 7, toughness := stat 7 } }

def wanderingEye : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Wandering Eye", cost := some [generic 2, pip .blue], types := [.creature],
      subtypes := [creatureType "Illusion"],
      text := [keyword (.named "flying"), Primitives.Ability.static (Primitives.StaticSpec.visibility .reveal (Primitives.NounPhrase.playerGroup .allPlayers) Primitives.VisibleThing.wholeHand)],
      power := stat 1, toughness := stat 3 } }

def amorphousAxe : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Amorphous Axe", cost := some [generic 2], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ getsPt (Primitives.NounPhrase.attachHost .equipped (.type .creature)) (Primitives.Delta.up (.lit 3)) (Primitives.Delta.up (.lit 0)),
              Primitives.StaticSpec.qualityChange (Primitives.NounPhrase.attachHost .equipped (.type .creature)) .adds (Primitives.QualityPayload.everyTypeOf
                  .creature) ]),
          keywordCosting (.named "equip") (Primitives.Cost.mana [generic 3]) ] } }

def runedStalactite : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Runed Stalactite", cost := some [generic 1], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ getsPt (Primitives.NounPhrase.attachHost .equipped (.type .creature)) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1)),
              Primitives.StaticSpec.qualityChange (Primitives.NounPhrase.attachHost .equipped (.type .creature)) .adds (Primitives.QualityPayload.everyTypeOf
                  .creature) ]),
          keywordCosting (.named "equip") (Primitives.Cost.mana [generic 2]) ] } }

def arachnoform : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Arachnoform", cost := some [generic 1, pip .green], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") creature,
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ getsPt (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 2)),
              Primitives.StaticSpec.abilityGrant (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (keyword (.named "reach")),
              Primitives.StaticSpec.qualityChange (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) .adds (Primitives.QualityPayload.everyTypeOf
                  .creature) ]) ] } }

def nyleasPresence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Nylea's Presence", cost := some [generic 1, pip .green], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") land,
          when (Primitives.GameEvent.enters thisAura none) (Primitives.Instruction.draw (.lit 1)),
          Primitives.Ability.static (Primitives.StaticSpec.qualityChange (Primitives.NounPhrase.attachHost .enchanted (.type .land)) .adds (Primitives.QualityPayload.everyTypeOf
              .basicLand)) ] } }

/-- Curse of Conformity -/
def curseOfConformity : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Curse of Conformity", cost := some [generic 4, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura", enchantmentType "Curse"],
      text :=
        [ keywordSubject (.named "enchant") Primitives.Predicate.anyPlayer,
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ getsPt (allOf (Primitives.Predicate.and [ creature, Primitives.Predicate.not (Primitives.Predicate.hasSupertype .legendary),
                               Primitives.Predicate.hasPossessor .controller (Primitives.NounPhrase.attachHost .enchanted .player) ])) (Primitives.Delta.set (.lit 3)) (Primitives.Delta.set (.lit 3)),
              Primitives.StaticSpec.qualityChange them .loses (Primitives.QualityPayload.everyTypeOf .creature) ]) ] } }

/-- Drumbellower -/
def drumbellower : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Drumbellower", cost := some [generic 2, pip .white], types := [.creature],
      subtypes := [creatureType "Spirit"],
      text :=
        [ keyword (.named "flying"),
          Primitives.Ability.static (untapsDuring (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
            (some (each otherPlayer))) ],
      power := stat 2, toughness := stat 1 } }

def tayamLine : StaticSpec :=
  entersWithAdditionalCounters
    (each (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, Primitives.Predicate.otherThan thisCreature])) (.lit 1)
    (.named "vigilanceCounter")
theorem okTayamLine : StaticSpec.check (DRS.ofList []) tayamLine = [] := by decide

def clarionConqueror : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Clarion Conqueror", cost := some [generic 2, pip .white], types := [.creature],
      subtypes := [creatureType "Dragon"],
      text :=
        [ keyword (.named "flying"),
          Primitives.Ability.static (objectCant (.action "activate")
            (allOf (Primitives.Predicate.and [ Primitives.Predicate.abilityHead .anyActivated,
                           Primitives.Predicate.abilityOf (allOf (Primitives.Predicate.or [artifact, creature, Primitives.Predicate.hasType .planeswalker])) ]))) ],
      power := stat 3, toughness := stat 3 } }

/-- Kang the Conqueror -/
def kangPowerUpLock : StaticSpec :=
  objectCant (.action "activate") (allOf (Primitives.Predicate.abilityHead (.keyword (.named "powerUp"))))
theorem okKangPowerUpLock : StaticSpec.check (DRS.ofList []) kangPowerUpLock = [] := by decide

def vipersKiss : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Viper's Kiss", cost := some [pip .black], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") creature,
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ getsPt (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (Primitives.Delta.down (.lit 1)) (Primitives.Delta.down (.lit 1)),
              objectCant (.action "activate")
                (allOf (Primitives.Predicate.and [Primitives.Predicate.abilityHead .anyActivated, Primitives.Predicate.abilityOf it])) ]) ] } }

def stupefyingTouch : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stupefying Touch", cost := some [generic 1, pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") creature,
          when (Primitives.GameEvent.enters thisAura none) (Primitives.Instruction.draw (.lit 1)),
          Primitives.Ability.static (objectCant (.action "activate")
            (allOf (Primitives.Predicate.and [ Primitives.Predicate.abilityHead .anyActivated,
                           Primitives.Predicate.abilityOf (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) ]))) ] } }

def linvalaKeeperOfSilence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Linvala, Keeper of Silence", cost := some [generic 2, pip .white, pip .white],
      supertypes := [.legendary], types := [.creature], subtypes := [creatureType "Angel"],
      text :=
        [ keyword (.named "flying"),
          Primitives.Ability.static (objectCant (.action "activate")
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
          keyword (.named "haste") ],
      power := stat 1, toughness := stat 1 } }

def polarKraken : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Polar Kraken", cost := some [generic 8, pip .blue, pip .blue, pip .blue],
      types := [.creature], subtypes := [creatureType "Kraken"],
      text :=
        [ keyword (.named "trample"),
          Primitives.Ability.static (entersTapped thisCreature),
          cumulativeUpkeep (Primitives.Cost.perform (sacrifice (a land))) ],
      power := stat 11, toughness := stat 11 } }

def yavimayaAnts : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Yavimaya Ants", cost := some [generic 2, pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Insect"],
      text := [keyword (.named "trample"), keyword (.named "haste"), cumulativeUpkeep (Primitives.Cost.mana [pip .green, pip .green])],
      power := stat 5, toughness := stat 1 } }

def illusionaryForces : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Illusionary Forces", cost := some [generic 3, pip .blue], types := [.creature],
      subtypes := [creatureType "Illusion"],
      text := [keyword (.named "flying"), cumulativeUpkeep (Primitives.Cost.mana [pip .blue])],
      power := stat 4, toughness := stat 4 } }

def vexingSphinx : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vexing Sphinx", cost := some [generic 1, pip .blue, pip .blue], types := [.creature],
      subtypes := [creatureType "Sphinx"],
      text :=
        [ keyword (.named "flying"),
          cumulativeUpkeep (Primitives.Cost.perform (discard (a (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.inZone yourHand])))),
          when (Primitives.GameEvent.dies thisCreature) (Primitives.Instruction.draw (countersOn (.named "ageCounter") it)) ],
      power := stat 4, toughness := stat 4 } }

def manaChains : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mana Chains", cost := some [pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") creature,
          Primitives.Ability.static (Primitives.StaticSpec.abilityGrant (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (cumulativeUpkeep (Primitives.Cost.mana
              [generic 1]))) ] } }

def dreamThief : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dream Thief", cost := some [generic 2, pip .blue], types := [.creature],
      subtypes := [creatureType "Faerie", creatureType "Rogue"],
      text :=
        [ keyword (.named "flying"),
          when (Primitives.GameEvent.enters thisCreature none)
            (Primitives.Instruction.doOnlyIf (Primitives.Instruction.draw (.lit 1))
              (happened (Primitives.Instruction.procedure .cast ((a
                (Primitives.Predicate.and [spell, Primitives.Predicate.colorIs .blue,
                Primitives.Predicate.otherThan Primitives.NounPhrase.this]))) Primitives.PlayPayment.itsOwnCost none none none) (Primitives.Subject.actor Primitives.NounPhrase.you) soFarThisTurn)
              none) ],
      power := stat 2, toughness := stat 1 } }

def brightspearZealot : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Brightspear Zealot", cost := some [generic 2, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Soldier"],
      text :=
        [ keyword (.named "vigilance"),
          Primitives.Ability.static (onlyWhile (getsPt thisCreature (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 0)))
            (Primitives.Condition.compareAmt (eventCount (Primitives.Instruction.procedure .cast ((a spell)) Primitives.PlayPayment.itsOwnCost none none none) (Primitives.Subject.actor Primitives.NounPhrase.you) soFarThisTurn) .atLeast (.lit
              2))) ],
      power := stat 2, toughness := stat 4 } }

def deepwayNavigator : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Deepway Navigator", cost := some [pip .white, pip .blue], types := [.creature],
      subtypes := [creatureType "Merfolk", creatureType "Wizard"],
      text :=
        [ keyword (.named "flash"),
          when (Primitives.GameEvent.enters thisCreature none)
            (Primitives.Instruction.setStatus .untapped
              (each (Primitives.Predicate.and [ Primitives.Predicate.hasSubtype (creatureType "Merfolk"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you,
                            Primitives.Predicate.otherThan thisCreature ]))),
          Primitives.Ability.static (onlyWhile
            (getsPt (allOf (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (creatureType "Merfolk"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
              (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 0)))
            (happened (Primitives.Instruction.attackWith (counted (atLeast 3)
              (Primitives.Predicate.hasSubtype (creatureType "Merfolk"))) none) (Primitives.Subject.actor Primitives.NounPhrase.you) soFarThisTurn)) ],
      power := stat 2, toughness := stat 2 } }

def bloodfireEnforcers : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bloodfire Enforcers", cost := some [generic 3, pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Monk"],
      text :=
        [ Primitives.Ability.static (onlyWhile
            (Primitives.StaticSpec.conjunction none [Primitives.StaticSpec.abilityGrant thisCreature (keyword (.named "firstStrike")), Primitives.StaticSpec.abilityGrant it
                (keyword (.named "trample"))])
            (Primitives.Condition.and
              [ exists_ (Primitives.Predicate.and [instant, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)]),
                exists_ (Primitives.Predicate.and [sorcery, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)]) ])) ],
      power := stat 5, toughness := stat 2 } }

def stormOfSouls : Instruction :=
  Primitives.Instruction.sequentially
    [ move (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)])) battlefield,
      become them
        { characteristics :=
          { subtypes := [creatureType "Spirit"], text := [keyword (.named "flying")],
            power := stat 1, toughness := stat 1 } }
        none,
      exile Primitives.NounPhrase.this ]
theorem okStormOfSouls : Instruction.check (DRS.ofList []) stormOfSouls = [] := by decide

def answeredPrayers : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Answered Prayers", cost := some [generic 1, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ when (Primitives.GameEvent.enters (a creatureYouControl) none)
            (Primitives.Instruction.sequentially
              [ gainLife (.lit 1),
                Primitives.Instruction.doIf (Primitives.Condition.not (Primitives.Condition.matches
                    thisEnchantment creature))
                  (become thisEnchantment
                    { characteristics :=
                      { types := [.creature], subtypes := [creatureType "Angel"],
                        text := [keyword (.named "flying")], power := stat 3, toughness := stat 3 } }
                    (some untilEndOfTurn))
                  none ]) ] } }

/-- Hate Mirage's middle two sentences -/
def hateMirageTokens : Instruction :=
  Primitives.Instruction.sequentially
    [ Primitives.Instruction.doForEach (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 2)) creatureYouDontControl)
        (Primitives.Instruction.create (.lit 1) (Primitives.TokenSpec.copyOf it []) []),
      gain (those .token) (keyword (.named "haste")) none ]
theorem okHateMirageTokens : Instruction.check (DRS.ofList []) hateMirageTokens = [] := by decide
def descentOfTheDragons : Instruction :=
  Primitives.Instruction.sequentially
    [ destroy (Primitives.NounPhrase.described (Primitives.DetPhrase.target anyNumber) creature),
      Primitives.Instruction.doForEach (theVerbed (.action "destroy") .many (some (.type .creature)) .thisWay)
        (instruct (controllerOf it) (Primitives.Instruction.create (.lit 1)
          (Primitives.TokenSpec.written
            { characteristics :=
              { colors := [.red], types := [.creature], subtypes := [creatureType "Dragon"],
                text := [keyword (.named "flying")], power := stat 4, toughness := stat 4 } })
          [])) ]
theorem okDescentOfTheDragons : Instruction.check (DRS.ofList []) descentOfTheDragons = [] := by decide
/-- Brimaz, King of Oreskos -/
def brimazAttackToken : Ability :=
  whenever (attacks thisCreature)
    (Primitives.Instruction.create (.lit 1)
      (Primitives.TokenSpec.written
        { characteristics :=
          { colors := [.white], types := [.creature], subtypes := [creatureType "Cat", creatureType "Soldier"],
            text := [keyword (.named "vigilance")], power := stat 1, toughness := stat 1 } })
      [Primitives.EntryRider.entersAttacking none])
theorem okBrimazAttackToken : Ability.check (DRS.ofList []) brimazAttackToken = [] := by decide
/-- Sedris, the Traitor King -/
def sedrisTheTraitorKing : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.abilityGrant (each (Primitives.Predicate.and [creature, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)]))
    (keywordCosting (.named "unearth") (Primitives.Cost.mana [generic 2, pip .black])))
theorem okSedrisTheTraitorKing : Ability.check (DRS.ofList []) sedrisTheTraitorKing = [] := by decide
/-- Grixis -/
def grixis : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.abilityGrant
    (allOf (Primitives.Predicate.and [ Primitives.Predicate.or [Primitives.Predicate.colorIs .blue, Primitives.Predicate.colorIs .black, Primitives.Predicate.colorIs .red], creature,
                   Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you) ]))
    (keywordCosting (.named "unearth") Primitives.Cost.itsManaCost))
theorem okGrixis : Ability.check (DRS.ofList []) grixis = [] := by decide
/-- Dralnu, Lich Lord -/
def dralnuLichLord : Instruction :=
  gain (target (Primitives.Predicate.and [instantOrSorcery, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)]))
    (keywordCosting (.named "flashback") Primitives.Cost.itsManaCost) (some untilEndOfTurn)
theorem okDralnuLichLord : Instruction.check (DRS.ofList []) dralnuLichLord = [] := by decide
/-- Dregscape Zombie -/
def dregscapeZombie : Ability := keywordCosting (.named "unearth") (Primitives.Cost.mana [pip .black])
theorem okDregscapeZombie : Ability.check (DRS.ofList []) dregscapeZombie = [] := by decide
/-- Think Twice -/
def thinkTwice : Ability := keywordCosting (.named "flashback") (Primitives.Cost.mana [generic 2, pip .blue])
theorem okThinkTwice : Ability.check (DRS.ofList []) thinkTwice = [] := by decide
/-- Greater Mossdog -/
def greaterMossdog : Ability := keywordNumber (.named "dredge") (.lit 3)
theorem okGreaterMossdog : Ability.check (DRS.ofList []) greaterMossdog = [] := by decide
/-- Raven's Crime -/
def ravensCrime : Ability := keyword (.named "retrace")
theorem okRavensCrime : Ability.check (DRS.ofList []) ravensCrime = [] := by decide
/-- Barkhide Mauler -/
def barkhideMauler : Ability := keywordCosting (.named "cycling") (Primitives.Cost.mana [generic 2])
theorem okBarkhideMauler : Ability.check (DRS.ofList []) barkhideMauler = [] := by decide
/-- Ninja of the New Moon -/
def ninjaOfTheNewMoon : Ability := keywordCosting (.named "ninjutsu") (Primitives.Cost.mana [generic 3, pip .black])
theorem okNinjaOfTheNewMoon : Ability.check (DRS.ofList []) ninjaOfTheNewMoon = [] := by decide
/-- Thunderous Wrath -/
def thunderousWrath : Ability := keywordCosting (.named "miracle") (Primitives.Cost.mana [pip .red])
theorem okThunderousWrath : Ability.check (DRS.ofList []) thunderousWrath = [] := by decide
/-- Bygone Colossus -/
def bygoneColossus : Ability := keywordCosting (.named "warp") (Primitives.Cost.mana [generic 3])
theorem okBygoneColossus : Ability.check (DRS.ofList []) bygoneColossus = [] := by decide
/-- Knight of Grace — "Hexproof from black" [CR#702.11d] -/
def knightOfGraceHexproofFromBlack : Ability := keywordQuality (.named "hexproof") (Primitives.Predicate.colorIs .black)
theorem okKnightOfGraceHexproofFromBlack :
    Ability.check (DRS.ofList []) knightOfGraceHexproofFromBlack = [] := by decide
/-- Eternal Dragon — "Plainscycling {2}" [CR#702.29e] -/
def eternalDragonPlainscycling : Ability :=
  keywordQualityCosting (.named "cycling") (Primitives.Predicate.hasSubtype (landType "Plains")) (Primitives.Cost.mana [generic 2])
theorem okEternalDragonPlainscycling : Ability.check (DRS.ofList []) eternalDragonPlainscycling = [] := by decide
/-- Nezumi Ronin -/
def nezumiRonin : Ability := keywordNumber (.named "bushido") (.lit 1)
theorem okNezumiRonin : Ability.check (DRS.ofList []) nezumiRonin = [] := by decide
/-- Dragonlord Ojutai -/
def dragonlordOjutaiHexproof : Ability :=
  Primitives.Ability.static (onlyWhile (Primitives.StaticSpec.abilityGrant thisCreature (keyword (.named "hexproof"))) (Primitives.Condition.matches it untapped))
theorem okDragonlordOjutaiHexproof : Ability.check (DRS.ofList []) dragonlordOjutaiHexproof = [] := by decide
/-- Monoxa, Midway Manager -/
def monoxaRollTrigger : Ability :=
  whenever (youRollResultIn (atLeast 3))
    (Primitives.Instruction.sequentially
      [ gain thisCreature (keyword (.named "firstStrike")) (some untilEndOfTurn),
        doIf (Primitives.Condition.compareAmt (Primitives.Amount.theOutcome .rollResult) .atLeast (.lit 4))
          (gain it (keyword (.named "menace")) (some untilEndOfTurn)),
        doIf (Primitives.Condition.compareAmt (Primitives.Amount.theOutcome .rollResult) .atLeast (.lit 5))
          (gain it (keyword (.named "lifelink")) (some untilEndOfTurn)) ])
theorem okMonoxaRollTrigger : Ability.check (DRS.ofList []) monoxaRollTrigger = [] := by decide
/-- Celebr-8000 -/
def celebr8000Doubles : Instruction :=
  Primitives.Instruction.sequentially
    [ rollDice 2 6,
      doIf Primitives.Condition.rolledDoubles (gain thisCreature (keyword (.named "doubleStrike")) (some untilEndOfTurn)) ]
theorem okCelebr8000Doubles : Instruction.check (DRS.ofList []) celebr8000Doubles = [] := by decide

/-- Krosan Druid -/
def krosanDruid : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Krosan Druid", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [creatureType "Centaur", creatureType "Druid"],
      text :=
        [ keywordCosting (.named "kicker") (Primitives.Cost.mana [generic 4, pip .green]),
          triggeredIf (Primitives.GameEvent.enters thisCreature none) (costWasPaid (.byKeyword (.named "kicker")) none thisCreature)
            (gainLife (.lit 10)) ],
      power := stat 2, toughness := stat 3 } }

/-- Lightkeeper of Emeria -/
def lightkeeperOfEmeria : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lightkeeper of Emeria", cost := some [generic 3, pip .white], types := [.creature],
      subtypes := [creatureType "Angel"],
      text :=
        [ keywordCosting (.named "multikicker") (Primitives.Cost.mana [pip .white]),
          keyword (.named "flying"),
          when (Primitives.GameEvent.enters thisCreature none)
            (gainLife (times (.lit 2) (timesPaid (.byKeyword (.named "kicker")) thisCreature))) ],
      power := stat 2, toughness := stat 4 } }

/-- Merfolk Falconer -/
def merfolkFalconer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Merfolk Falconer", cost := some [generic 3, pip .blue, pip .blue], types := [.creature],
      subtypes := [creatureType "Merfolk", creatureType "Wizard"],
      text :=
        [ keyword (.named "flying"),
          whenever
            (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Instruction.procedure .cast ((a (Primitives.Predicate.compareOver spell (paidCostRead (.byKeyword (.named "kicker"))
                none it) .atLeast (.lit 1)))) Primitives.PlayPayment.itsOwnCost none none none))
            (scry (.lit 2)) ],
      power := stat 4, toughness := stat 4 } }

/-- Borrowed Malevolence -/
def borrowedMalevolence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Borrowed Malevolence", cost := some [pip .black], types := [.instant],
      text :=
        [ keywordCosting (.named "escalate") (Primitives.Cost.mana [generic 2]),
          Primitives.Ability.spell none (chooseModes (Primitives.Quantity.range (some 1) (some 2))
            [ get (target creature) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1)) (some untilEndOfTurn),
              get (target creature) (Primitives.Delta.down (.lit 1)) (Primitives.Delta.down (.lit 1)) (some untilEndOfTurn) ]) ] } }

/-- New Perspectives -/
def newPerspectivesCyclingAltCost : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.altCost (allOf (Primitives.Predicate.abilityHead (.keyword (.named "cycling")))) (some (Primitives.Cost.mana [generic 0])))
theorem okNewPerspectivesCyclingAltCost :
    Ability.check (DRS.ofList []) newPerspectivesCyclingAltCost = [] := by decide
/-- Thick-Skinned Goblin -/
def thickSkinnedGoblinEchoAltCost : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.altCost
    (allOf (Primitives.Predicate.and [ Primitives.Predicate.abilityHead (.keyword (.named "echo")),
                   Primitives.Predicate.abilityOf (allOf (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) ]))
    (some (Primitives.Cost.mana [generic 0])))
theorem okThickSkinnedGoblinEchoAltCost :
    Ability.check (DRS.ofList []) thickSkinnedGoblinEchoAltCost = [] := by decide
/-- Fumiko the Lowblood -/
def fumikoBushidoX : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.conjunction none
    [ Primitives.StaticSpec.abilityGrant thisCreature (keywordNumber (.named "bushido") (Primitives.Amount.letter .x)),
      Primitives.StaticSpec.letterDefinition .x (countOf attacking) ])
theorem okFumikoBushidoX : Ability.check (DRS.ofList []) fumikoBushidoX = [] := by decide

/-- Rafter Demon -/
def rafterDemon : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rafter Demon", cost := some [generic 2, pip .black, pip .red], types := [.creature],
      subtypes := [creatureType "Demon"],
      text :=
        [ keywordCosting (.named "spectacle") (Primitives.Cost.mana [generic 3, pip .black, pip .red]),
          triggeredIf (Primitives.GameEvent.enters thisCreature none)
            (costWasPaid (.byKeyword (.named "spectacle")) none thisCreature)
            (Primitives.Instruction.sequentially [instruct (each Primitives.Predicate.opponent) (choose (a (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.inZone hand]))), discard them]) ],
      power := stat 4, toughness := stat 2 } }

/-- Tourach, Dread Cantor -/
def tourachDiscardTrigger : Ability :=
  whenever (Primitives.GameEvent.does (Primitives.Subject.actor anOpponent) (discard ((a
    Primitives.Predicate.isCard))))
    (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter) thisCreature .many true)
theorem okTourachDiscardTrigger : Ability.check (DRS.ofList []) tourachDiscardTrigger = [] := by decide

/-- Shimmering Glasskite -/
def shimmeringGlasskite : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shimmering Glasskite", cost := some [generic 3, pip .blue], types := [.creature],
      subtypes := [creatureType "Spirit"],
      text :=
        [ keyword (.named "flying"),
          triggeredOnlyOnce
            (Primitives.GameEvent.does (Primitives.Subject.actor (a (Primitives.Predicate.or [spell, Primitives.Predicate.abilityHead .anyOnStack]))) (Primitives.Instruction.target thisCreature)) oncePerTurn
            (Primitives.Instruction.counterSpell (that .stack)) ],
      power := stat 2, toughness := stat 3 } }

/-- Conqueror's Pledge -/
def conquerorsPledge : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Conqueror's Pledge", cost := some [generic 2, pip .white, pip .white, pip .white],
      types := [.sorcery],
      text :=
        [ keywordCosting (.named "kicker") (Primitives.Cost.mana [generic 6]),
          Primitives.Ability.spell none (Primitives.Instruction.replace
            (create (.lit 6) (creatureToken 1 1 [.white] [creatureType "Kor", creatureType "Soldier"]))
            (Primitives.Instruction.doIf (costWasPaid (.byKeyword (.named "kicker")) none Primitives.NounPhrase.this)
              (Primitives.Instruction.create (.lit 12) Primitives.TokenSpec.asThose []) none)) ] } }

/-- Soul of Emancipation -/
def soulOfEmancipation : Ability :=
  when (Primitives.GameEvent.enters thisCreature none)
    (Primitives.Instruction.sequentially
      [ destroy (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 3)) (Primitives.Predicate.and [permanent, Primitives.Predicate.not land, Primitives.Predicate.otherThan Primitives.NounPhrase.this])),
        Primitives.Instruction.doForEach (those .permanent)
          (instruct (controllerOf (that .permanent)) (Primitives.Instruction.create (.lit 1)
            (Primitives.TokenSpec.written
              { characteristics :=
                { colors := [.white], types := [.creature], subtypes := [creatureType "Angel"],
                  text := [keyword (.named "flying")], power := stat 3, toughness := stat 3 } })
            [])) ])
theorem okSoulOfEmancipation : Ability.check (DRS.ofList []) soulOfEmancipation = [] := by decide

/-- Tourach, Dread Cantor -/
def tourachDreadCantor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tourach, Dread Cantor", cost := some [generic 1, pip .black],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Cleric"],
      text :=
        [ keywordCosting (.named "kicker") (Primitives.Cost.mana [pip .black, pip .black]),
          keywordQuality (.named "protection") (Primitives.Predicate.colorIs .white),
          tourachDiscardTrigger,
          triggeredIf (Primitives.GameEvent.enters thisCreature none) (costWasPaid (.byKeyword (.named "kicker")) none thisCreature)
            (instruct (target Primitives.Predicate.opponent) (Actor.discard (countedAtRandom (exactly 2) Primitives.Predicate.isCard))) ],
      power := stat 2, toughness := stat 1 } }

/-- Talrand's Invocation -/
def talrandsInvocation : Instruction :=
  create (.lit 2)
    { characteristics :=
      { colors := [.blue], types := [.creature], subtypes := [creatureType "Drake"],
        text := [keyword (.named "flying")], power := stat 2, toughness := stat 2 } }
theorem okTalrandsInvocation : Instruction.check (DRS.ofList []) talrandsInvocation = [] := by decide

/-- Predatory Wurm -/
def predatoryWurm : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Predatory Wurm", cost := some [generic 3, pip .green], types := [.creature],
      subtypes := [creatureType "Wurm"],
      text :=
        [ keyword (.named "vigilance"),
          Primitives.Ability.static (onlyWhile (getsPt thisCreature (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 2)))
            (exists_ (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (planeswalkerType "Garruk"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))) ],
      power := stat 4, toughness := stat 4 } }

/-- Tower Winder -/
def towerWinder : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tower Winder", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [creatureType "Snake"],
      text :=
        [ keyword (.named "reach"), keyword (.named "deathtouch"),
          when (Primitives.GameEvent.enters thisCreature none)
            (Primitives.Instruction.sequentially
              [ searchLibraryOrGraveyard (Primitives.Predicate.named (Primitives.NameSource.printed "Command Tower")),
                revealCards (theVerbed (.action "search") .one), move (theVerbed (.action "search") .one) hand,
                Primitives.Instruction.doIf (happenedThisWay (Primitives.Instruction.search (Primitives.SearchScope.oneZone Primitives.ZoneExpr.anyZone) anyNumber (Primitives.Predicate.and [])) (Primitives.Subject.actor Primitives.NounPhrase.you)) shuffle
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
                  [ getsPt (theVerbed (.action "untap") .one) (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 2)),
                    Primitives.StaticSpec.abilityGrant (theVerbed (.action "untap") .one) (keyword (.named "reach")) ])
                (some untilEndOfTurn) ]) ] } }

/-- Harried Dronesmith -/
def harriedDronesmithToken : Instruction :=
  Primitives.Instruction.sequentially [create (.lit 1) thopterToken, gainHaste itAsToken (some untilEndOfTurn)]
theorem okHarriedDronesmithToken : Instruction.check (DRS.ofList []) harriedDronesmithToken = [] := by decide

/-- Archfiend's Vessel -/
def archfiendsVessel : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Archfiend's Vessel", cost := some [pip .black], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Cleric"],
      text :=
        [ keyword (.named "lifelink"),
          triggeredIf (Primitives.GameEvent.enters thisCreature none)
            (Primitives.Condition.or
              [ Primitives.Condition.did (⟨Primitives.Subject.any, (Primitives.Instruction.move it (Primitives.EventSource.zones
                [graveyardOf Primitives.NounPhrase.you]) (some battlefield) .arrival none []), Primitives.LookbackSpan.triggering⟩),
                Primitives.Condition.matches it (Primitives.Predicate.and [castBy Primitives.NounPhrase.you, Primitives.Predicate.castFrom (graveyardOf Primitives.NounPhrase.you)]) ])
            (Primitives.Instruction.triggerReflexively (exile it)
              (create (.lit 1)
                { characteristics :=
                  { colors := [.black], types := [.creature], subtypes := [creatureType "Demon"],
                    text := [keyword (.named "flying")], power := stat 5, toughness := stat 5 } })) ],
      power := stat 1, toughness := stat 1 } }

/-- Containment Priest -/
def containmentPriest : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Containment Priest", cost := some [generic 1, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Cleric"],
      text :=
        [ keyword (.named "flash"),
          Primitives.Ability.static (Primitives.StaticSpec.instead (Primitives.WouldClause.would Primitives.Subject.any (Primitives.Instruction.move (a (Primitives.Predicate.and [creature, nontoken, Primitives.Predicate.not (Primitives.Predicate.castBy Primitives.NounPhrase.any none)])) Primitives.EventSource.anywhere (some battlefield) .arrival none [])) (exile it) none .repeatedly none) ],
      power := stat 2, toughness := stat 2 } }

/-- Veiling Oddity -/
def veilingOddityLine : Ability :=
  triggeredWhile (lastCounterRemoved (.named "timeCounter") Primitives.NounPhrase.this)
    (whileTrue (Primitives.Condition.matches Primitives.NounPhrase.this (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.inZone exileZone])))
    (Primitives.Instruction.establish (Primitives.StaticSpec.cant Primitives.Subject.any (Primitives.Instruction.block (allOf creature)) { rider := Primitives.DeonticRider.noRider })
      (some thisTurn))
theorem okVeilingOddityLine : Ability.check (DRS.ofList []) veilingOddityLine = [] := by decide

def wizenedSnitches : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Wizened Snitches", cost := some [generic 3, pip .blue], types := [.creature],
      subtypes := [creatureType "Faerie", creatureType "Rogue"],
      text := [keyword (.named "flying"), Primitives.Ability.static (Primitives.StaticSpec.visibility .reveal (Primitives.NounPhrase.playerGroup .allPlayers) Primitives.VisibleThing.topOfLibrary)],
      power := stat 1, toughness := stat 3 } }

/-- Darkblade Agent -/
def darkbladeAgentGrants : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.conditional
    (Primitives.StaticSpec.conjunction none
      [ Primitives.StaticSpec.abilityGrant thisCreature (keyword (.named "deathtouch")),
        Primitives.StaticSpec.abilityGrant thisCreature
          (whenever (dealsCombatDamage thisCreature (a Primitives.Predicate.anyPlayer)) (Primitives.Instruction.draw (.lit 1))) ])
    (happened (surveil Primitives.Amount.anyAmount) (Primitives.Subject.actor Primitives.NounPhrase.you) soFarThisTurn) .asLongAs)
theorem okDarkbladeAgentGrants : Ability.check (DRS.ofList []) darkbladeAgentGrants = [] := by decide
/-- Frenzied Gorespawn -/
def frenziedGorespawnMenaceTrigger : Ability :=
  Primitives.Ability.triggered (Primitives.GameEvent.does (Primitives.Subject.actor (counted (atLeast 1) creature)) (Primitives.Instruction.attack (some anOpponent))) [] none [] none
    none none (gain (those (.type .creature)) (keyword (.named "menace")) (some untilEndOfTurn))
theorem okFrenziedGorespawnMenaceTrigger :
    Ability.check (DRS.ofList []) frenziedGorespawnMenaceTrigger = [] := by decide
/-- Pir, Imaginative Rascal's replacement -/
def pirDistributive : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.instead (Primitives.WouldClause.would Primitives.Subject.any (Primitives.Instruction.putCounters Primitives.Amount.anyAmount Primitives.CounterKindSource.anyKind (a (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .controller (Primitives.NounPhrase.playerGroup .yourTeam)])) .many false)) (Primitives.Instruction.putCounters (plus Primitives.Amount.thatMuch (.lit 1)) Primitives.CounterKindSource.those (that .permanent) .many true) none .repeatedly none)
theorem okPirDistributive : Ability.check (DRS.ofList []) pirDistributive = [] := by decide

/-- Frogmite -/
def frogmite : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Frogmite", cost := some [generic 4], types := [.artifact, .creature],
      subtypes := [creatureType "Frog"], text := [keywordQuality (.named "affinity") artifact],
      power := stat 2, toughness := stat 2 } }

/-- Iymrith, Desert Doom -/
def iymrithDesertDoom : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Iymrith, Desert Doom", cost := some [generic 3, pip .blue, pip .blue],
      supertypes := [.legendary], types := [.creature], subtypes := [creatureType "Dragon"],
      text :=
        [ keyword (.named "flying"),
          Primitives.Ability.static (onlyWhile (Primitives.StaticSpec.abilityGrant thisCreature (keywordCosting (.named "ward") (Primitives.Cost.mana [generic
              4])))
            (Primitives.Condition.matches it untapped)),
          whenever (dealsCombatDamage thisCreature (a Primitives.Predicate.anyPlayer))
            (Primitives.Instruction.sequentially
              [ Primitives.Instruction.draw (.lit 1),
                Primitives.Instruction.doIf (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.inZone (handOf Primitives.NounPhrase.you)])) .less (.lit 3))
                  (Primitives.Instruction.draw Primitives.Amount.theDifference) none ]) ],
      power := stat 5, toughness := stat 5 } }

def protectionAbilities : KeywordTerm := .anyIn ⟨(.named "protection"), none⟩
def landwalkAbilities : KeywordTerm := .anyIn ⟨(.named "landwalk"), none⟩
/-- Shay Cormac -/
def wardAbilities : KeywordTerm := .anyIn ⟨(.named "ward"), none⟩
/-- Tolaria, Shelkin Brownie -/
def bandsWithOtherAbilities : KeywordTerm := .anyIn ⟨(.named "bandsWithOther"), none⟩
def protectionFromAnyColor : KeywordTerm := .anyIn ⟨(.named "protection"), some .color⟩

/-- Cairn Wanderer -/
def cairnWanderer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cairn Wanderer", cost := some [generic 4, pip .black], types := [.creature],
      subtypes := [creatureType "Shapeshifter"],
      text :=
        [ keyword (.named "changeling"),
          Primitives.Ability.alsoForKeywords
            (Primitives.Ability.static (onlyWhile (Primitives.StaticSpec.abilityGrant thisCreature (keyword (.named "flying")))
              (exists_ (Primitives.Predicate.and [creature, Primitives.Predicate.inZone graveyard, Primitives.Predicate.hasKeyword (.the (.named "flying"))]))))
            [ .the (.named "fear"), .the (.named "firstStrike"), .the (.named "doubleStrike"), .the (.named "deathtouch"), .the (.named "haste"),
              landwalkAbilities, .the (.named "lifelink"), protectionAbilities, .the (.named "reach"), .the (.named "trample"),
              .the (.named "shroud"), .the (.named "vigilance") ] ],
      power := stat 4, toughness := stat 4 } }

/-- Autarch Mammoth -/
def autarchMammoth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Autarch Mammoth", cost := some [generic 4, pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Elephant", creatureType "Mount"],
      text := [autarchMammothLine, keywordNumber (.named "saddle") (.lit 5)],
      power := stat 5, toughness := stat 5 } }

/-- Debris Beetle -/
def debrisBeetle : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Debris Beetle", cost := some [generic 2, pip .black, pip .green], types := [.artifact],
      subtypes := [artifactType "Vehicle"],
      text :=
        [ keyword (.named "trample"),
          when (Primitives.GameEvent.enters thisVehicle none)
            (Primitives.Instruction.sequentially [instruct (each Primitives.Predicate.opponent) (loseLife (.lit 3)), gainLife (.lit 3)]),
          keywordNumber (.named "crew") (.lit 2) ],
      power := stat 6, toughness := stat 6 } }

/-- Pir, Imaginative Rascal -/
def pirImaginativeRascal : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Pir, Imaginative Rascal", cost := some [generic 2, pip .green],
      supertypes := [.legendary], types := [.creature], subtypes := [creatureType "Human"],
      text :=
        [ keywordQuality (.named "partnerWith") (Primitives.Predicate.named (Primitives.NameSource.printed "Toothy, Imaginary Friend")),
          pirDistributive ],
      power := stat 1, toughness := stat 1 } }

/-- Market Gnome -/
def marketGnome : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Market Gnome", cost := some [pip .white], types := [.artifact, .creature],
      subtypes := [creatureType "Gnome"],
      text :=
        [ when (Primitives.GameEvent.dies thisCreature) (Primitives.Instruction.sequentially [gainLife (.lit 1), Primitives.Instruction.draw (.lit 1)]),
          triggeredWhile (Primitives.GameEvent.does Primitives.Subject.any (exile ((thisCreature))))
            (whileDoing (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Instruction.procedure .activate (a (Primitives.Predicate.abilityHead (.keyword (.named "craft")))) Primitives.PlayPayment.itsOwnCost none none none)))
            (Primitives.Instruction.sequentially [gainLife (.lit 1), Primitives.Instruction.draw (.lit 1)]) ],
      power := stat 0, toughness := stat 3 } }

/-- Escaped Shapeshifter -/
def escapedShapeshifter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Escaped Shapeshifter", cost := some [generic 3, pip .blue, pip .blue],
      types := [.creature], subtypes := [creatureType "Shapeshifter"],
      text :=
        [ Primitives.Ability.alsoForKeywords
            (Primitives.Ability.static (onlyWhile (Primitives.StaticSpec.abilityGrant thisCreature (keyword (.named "flying")))
              (exists_ (Primitives.Predicate.and [ creature, Primitives.Predicate.hasPossessor .controller anOpponent, Primitives.Predicate.hasKeyword (.the (.named "flying")),
                               Primitives.Predicate.not (Primitives.Predicate.named (Primitives.NameSource.printed "Escaped Shapeshifter")) ]))))
            [.the (.named "firstStrike"), .the (.named "trample"), protectionFromAnyColor] ],
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
          keywordCosting (.named "equip") (Primitives.Cost.mana [generic 1]) ] } }

/-- Steelclaw Lance -/
def steelclawLance : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Steelclaw Lance", cost := some [pip .black, pip .red], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ Primitives.Ability.static (getsPt (Primitives.NounPhrase.attachHost .equipped (.type .creature)) (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 2))),
          keywordQualityCosting (.named "equip") (Primitives.Predicate.hasSubtype (creatureType "Knight")) (Primitives.Cost.mana [generic 1]),
          keywordCosting (.named "equip") (Primitives.Cost.mana [generic 3]) ] } }

/-- Commander's Plate's equip lines -/
def commandersPlateEquip : List Ability :=
  [ keywordQualityCosting (.named "equip") (Primitives.Predicate.hasDesignation (.named "commander") none) (Primitives.Cost.mana [generic 3]),
    keywordCosting (.named "equip") (Primitives.Cost.mana [generic 5]) ]
theorem okCommandersPlateEquip : Ability.checkText [] commandersPlateEquip = [] := by decide
/-- Luxior, Giada's Gift's equip lines -/
def luxiorEquipLines : List Ability :=
  [ keywordQualityCosting (.named "equip") (Primitives.Predicate.hasType .planeswalker) (Primitives.Cost.mana [generic 1]),
    keywordCosting (.named "equip") (Primitives.Cost.mana [generic 3]) ]
theorem okLuxiorEquipLines : Ability.checkText [] luxiorEquipLines = [] := by decide

/-- Veiling Oddity -/
def veilingOddity : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Veiling Oddity", cost := some [generic 3, pip .blue], types := [.creature],
      subtypes := [creatureType "Illusion"],
      text := [keywordNumberCosting (.named "suspend") (.lit 4) (Primitives.Cost.mana [generic 1, pip .blue]), veilingOddityLine],
      power := stat 2, toughness := stat 3 } }

/-- Enormous Energy Blade -/
def enormousEnergyBlade : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Enormous Energy Blade", cost := some [generic 2, pip .black], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ Primitives.Ability.static (getsPt (Primitives.NounPhrase.attachHost .equipped (.type .creature)) (Primitives.Delta.up (.lit 4)) (Primitives.Delta.up (.lit 0))),
          whenever (Primitives.GameEvent.does Primitives.Subject.any (Primitives.Instruction.attachment .attached thisEquipment (some (a creature)))) (tap (that (.type .creature))),
          keywordCosting (.named "equip") (Primitives.Cost.mana [generic 2]) ] } }

/-- Grafted Wargear -/
def graftedWargear : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Grafted Wargear", cost := some [generic 3], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ Primitives.Ability.static (getsPt (Primitives.NounPhrase.attachHost .equipped (.type .creature)) (Primitives.Delta.up (.lit 3)) (Primitives.Delta.up (.lit 2))),
          whenever (Primitives.GameEvent.does Primitives.Subject.any (Primitives.Instruction.attachment .unattached thisEquipment (some (a permanent))))
            (sacrifice (that .permanent)),
          keywordCosting (.named "equip") (Primitives.Cost.mana [generic 0]) ] } }

/-- Black Ward -/
def blackWard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Black Ward", cost := some [pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") creature,
          Primitives.Ability.static (Primitives.StaticSpec.retention
            (Primitives.StaticSpec.abilityGrant (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (keywordQuality (.named "protection")
                (Primitives.Predicate.colorIs .black)))
            thisAura) ] } }

/-- Cho-Manno's Blessing -/
def choMannosBlessing : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cho-Manno's Blessing", cost := some [pip .white, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keyword (.named "flash"),
          keywordSubject (.named "enchant") creature,
          Primitives.Ability.static (entersChoosing thisAura .color),
          Primitives.Ability.static (Primitives.StaticSpec.retention
            (Primitives.StaticSpec.abilityGrant (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (keywordQuality (.named "protection")
                (ofChosen .color)))
            thisAura) ] } }

/-- Pentarch Ward -/
def pentarchWard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Pentarch Ward", cost := some [generic 2, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") creature,
          Primitives.Ability.static (entersChoosing thisAura .color),
          when (Primitives.GameEvent.enters thisAura none) (Primitives.Instruction.draw (.lit 1)),
          Primitives.Ability.static (Primitives.StaticSpec.retention
            (Primitives.StaticSpec.abilityGrant (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (keywordQuality (.named "protection")
                (ofChosen .color)))
            thisAura) ] } }

/-- Benevolent Blessing -/
def benevolentBlessing : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Benevolent Blessing", cost := some [generic 1, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keyword (.named "flash"),
          keywordSubject (.named "enchant") creature,
          Primitives.Ability.static (entersChoosing thisAura .color),
          Primitives.Ability.static (Primitives.StaticSpec.retention
            (Primitives.StaticSpec.abilityGrant (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (keywordQuality (.named "protection")
                (ofChosen .color)))
            (allOf (Primitives.Predicate.and [ Primitives.Predicate.or [Primitives.Predicate.hasSubtype (enchantmentType "Aura"), Primitives.Predicate.hasSubtype (artifactType "Equipment")],
                           Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, Primitives.Predicate.attachedTo it ]))) ] } }

/-- Wall of Shards -/
def wallOfShards : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Wall of Shards", cost := some [generic 1, pip .white], supertypes := [.snow],
      types := [.creature], subtypes := [creatureType "Wall"],
      text :=
        [ keyword (.named "defender"), keyword (.named "flying"),
          cumulativeUpkeep (Primitives.Cost.perform (instruct anOpponent (gainLife (.lit 1)))) ],
      power := stat 1, toughness := stat 8 } }

/-- Earthen Goo -/
def earthenGoo : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Earthen Goo", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [creatureType "Ooze"],
      text :=
        [ keyword (.named "trample"),
          cumulativeUpkeep (Primitives.Cost.either (Primitives.Cost.mana [pip .red]) (Primitives.Cost.mana [pip .green])),
          Primitives.Ability.static (getsPt thisCreature
            (Primitives.Delta.up (times (.lit 1) (countersOn (.named "ageCounter") it)))
            (Primitives.Delta.up (times (.lit 1) (countersOn (.named "ageCounter") it)))) ],
      power := stat 2, toughness := stat 2 } }

/-- Mutagen Connoisseur -/
def mutagenConnoisseur : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mutagen Connoisseur", cost := some [generic 1, pip .green, pip .blue],
      types := [.creature], subtypes := [creatureType "Vedalken", creatureType "Mutant"],
      text :=
        [ keyword (.named "flying"), keyword (.named "vigilance"),
          Primitives.Ability.static (getsPt thisCreature
            (Primitives.Delta.up (forEach 1 (Primitives.Predicate.and [Primitives.Predicate.isTransformed, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))) (Primitives.Delta.up (.lit 0))) ],
      power := stat 0, toughness := stat 5 } }

/-- Bedrock Tortoise -/
def bedrockTortoiseWindow : StaticSpec :=
  Primitives.StaticSpec.partScope .turn (some Primitives.NounPhrase.you) (Primitives.StaticSpec.abilityGrant (allOf creatureYouControl) (keyword (.named "hexproof")))
theorem okBedrockTortoiseWindow : StaticSpec.check (DRS.ofList []) bedrockTortoiseWindow = [] := by decide

/-- Battlegate Mimic -/
def battlegateMimic : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Battlegate Mimic", cost := some [generic 1, hybridPip .red .white], types := [.creature],
      subtypes := [creatureType "Shapeshifter"],
      text :=
        [ whenever (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Instruction.procedure .cast ((a
          (Primitives.Predicate.and [spell, Primitives.Predicate.colorIs .red,
          Primitives.Predicate.colorIs .white]))) Primitives.PlayPayment.itsOwnCost none none none))
            (Primitives.Instruction.establish
              (Primitives.StaticSpec.conjunction none
                [ getsPt thisCreature (Primitives.Delta.set (.lit 4)) (Primitives.Delta.set (.lit 2)),
                  Primitives.StaticSpec.abilityGrant thisCreature (keyword (.named "firstStrike")) ])
              (some untilEndOfTurn)) ],
      power := stat 2, toughness := stat 1 } }

/-- Sphinx of Uthuun -/
def sphinxOfUthuun : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sphinx of Uthuun", cost := some [generic 5, pip .blue, pip .blue], types := [.creature],
      subtypes := [creatureType "Sphinx"],
      text :=
        [ keyword (.named "flying"),
          when (Primitives.GameEvent.enters thisCreature none)
            (Primitives.Instruction.sequentially
              [ revealCards (topSlice (.lit 5)),
                instruct anOpponent (Primitives.Instruction.separateIntoPiles them 2 []),
                move onePile hand,
                move (theOther .pile) graveyard ]) ],
      power := stat 5, toughness := stat 6 } }

/-- Marit Lage -/
def maritLage : CharacteristicBundle :=
  { characteristics :=
    { name := "Marit Lage", colors := [.black], supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Avatar"], text := [keyword (.named "flying"), keyword (.named "indestructible")],
      power := stat 20, toughness := stat 20 } }

/-- Tuktuk the Explorer -/
def tuktukTheExplorer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tuktuk the Explorer", cost := some [generic 2, pip .red], supertypes := [.legendary],
      types := [.creature], subtypes := [creatureType "Goblin"],
      text :=
        [ keyword (.named "haste"),
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
        [ keyword (.named "flying"),
          whenever (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Instruction.procedure .cast ((a
            (Primitives.Predicate.and [historic, spell]))) Primitives.PlayPayment.itsOwnCost none none none)) (scry (.lit 1))
              ],
      power := stat 1, toughness := stat 1 } }

/-- Aya of Alexandria -/
def ayaOfAlexandria : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aya of Alexandria", cost := some [generic 2, pip .red, pip .white],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Assassin"],
      text :=
        [ keyword (.named "menace"), keyword (.named "lifelink"),
          whenever
            (dealsCombatDamage (a (Primitives.Predicate.and [historic, creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) (a Primitives.Predicate.anyPlayer))
            (create (.lit 1)
              { characteristics :=
                { colors := [.black], types := [.creature], subtypes := [creatureType "Assassin"],
                  text := [keyword (.named "menace")], power := stat 1, toughness := stat 1 } }) ],
      power := stat 4, toughness := stat 3 } }

/-- Indestructibility -/
def indestructibility : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Indestructibility", cost := some [generic 3, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") permanent,
          Primitives.Ability.static (Primitives.StaticSpec.abilityGrant (Primitives.NounPhrase.attachHost .enchanted .permanent) (keyword (.named "indestructible"))) ] }
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
                    text := [keyword (.named "flying")], power := stat 4, toughness := stat 4 } })
              [Primitives.EntryRider.entersAs .tapped, Primitives.EntryRider.entersAttacking (some (that .player))]),
          keywordCosting (.named "equip") (Primitives.Cost.mana [generic 4]) ] } }

def giantGrowth : Instruction :=
  get (target creature) (Primitives.Delta.up (.lit 3)) (Primitives.Delta.up (.lit 3)) (some untilEndOfTurn)
theorem okGiantGrowth : Instruction.check (DRS.ofList []) giantGrowth = [] := by decide
def amassZombiesTwo : Instruction := amass "Zombie" 2
theorem okAmassZombiesTwo : Instruction.check (DRS.ofList []) amassZombiesTwo = [] := by decide
/-- Avarice Totem -/
def avariceTotemExchange : Instruction :=
  Primitives.Instruction.exchange (Primitives.Exchanged.controlOf thisArtifact (target (Primitives.Predicate.and [permanent, Primitives.Predicate.not land])))
theorem okAvariceTotemExchange : Instruction.check (DRS.ofList []) avariceTotemExchange = [] := by decide
/-- Arcanum Wings -- aura swap [CR#702.65a] -/
def arcanumWingsAuraSwap : Ability :=
  activated (Primitives.Cost.mana [generic 2, pip .blue])
    (offer
      (Primitives.Instruction.exchange (Primitives.Exchanged.cardsAcross thisAura (a (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (enchantmentType "Aura"), Primitives.Predicate.inZone
          hand])))))
theorem okArcanumWingsAuraSwap : Ability.check (DRS.ofList []) arcanumWingsAuraSwap = [] := by decide
/-- Tovolar, Dire Overlord -/
def tovolarNightfall : Instruction := Primitives.Instruction.setGameDesignation (.named "night")
theorem okTovolarNightfall : Instruction.check (DRS.ofList []) tovolarNightfall = [] := by decide
/-- Spin into Myth -/
def spinIntoMyth : Instruction :=
  Primitives.Instruction.sequentially [move (target creature) onTop, fateseal anOpponent (.lit 2)]
theorem okSpinIntoMyth : Instruction.check (DRS.ofList []) spinIntoMyth = [] := by decide
/-- Pure // Simple -/
def pureHalf : Instruction := destroy (target (Primitives.Predicate.and [permanent, multicolored]))
theorem okPureHalf : Instruction.check (DRS.ofList []) pureHalf = [] := by decide
/-- Korlash -/
def grandeurDiscardCost : Cost :=
  Primitives.Cost.perform (discard (a (Primitives.Predicate.and [Primitives.Predicate.named (Primitives.NameSource.printed "Korlash, Heir to Blackblade"), Primitives.Predicate.otherThan Primitives.NounPhrase.this, Primitives.Predicate.inZone yourHand])))
theorem okGrandeurDiscardCost : Cost.check (DRS.ofList []) grandeurDiscardCost = [] := by decide
/-- Gyruda, Doom of Depths -/
def gyrudaCompanion : Ability := companion (Primitives.DeckCondition.everyCardIs Primitives.Predicate.isCard (Primitives.DeckTrait.manaValueParity .even))
theorem okGyrudaCompanion : Ability.check (DRS.ofList []) gyrudaCompanion = [] := by decide
/-- Jegantha, the Wellspring -/
def jeganthaCompanion : Ability := companion (Primitives.DeckCondition.noCardIs Primitives.Predicate.isCard Primitives.DeckTrait.repeatedManaSymbol)
theorem okJeganthaCompanion : Ability.check (DRS.ofList []) jeganthaCompanion = [] := by decide
/-- Kaheera, the Orphanguard -/
def kaheeraCompanion : Ability :=
  companion
    (Primitives.DeckCondition.everyCardIs (Primitives.Predicate.and [creature, Primitives.Predicate.isCard])
      (Primitives.DeckTrait.anyOf [ Primitives.DeckTrait.aCharacteristic (Primitives.Predicate.hasSubtype (creatureType "Cat")),
                Primitives.DeckTrait.aCharacteristic (Primitives.Predicate.hasSubtype (creatureType "Elemental")),
                Primitives.DeckTrait.aCharacteristic (Primitives.Predicate.hasSubtype (creatureType "Nightmare")),
                Primitives.DeckTrait.aCharacteristic (Primitives.Predicate.hasSubtype (creatureType "Dinosaur")),
                Primitives.DeckTrait.aCharacteristic (Primitives.Predicate.hasSubtype (creatureType "Beast")) ]))
theorem okKaheeraCompanion : Ability.check (DRS.ofList []) kaheeraCompanion = [] := by decide
/-- Keruga, the Macrosage -/
def kerugaCompanion : Ability :=
  companion
    (Primitives.DeckCondition.everyCardIs Primitives.Predicate.isCard
      (Primitives.DeckTrait.anyOf [ Primitives.DeckTrait.aCharacteristic (Primitives.Predicate.compare [.stat .manaValue] .atLeast (.lit 3)),
                Primitives.DeckTrait.aCharacteristic land ]))
theorem okKerugaCompanion : Ability.check (DRS.ofList []) kerugaCompanion = [] := by decide
/-- Lurrus of the Dream-Den -/
def lurrusCompanion : Ability :=
  companion
    (Primitives.DeckCondition.everyCardIs permanentCard (Primitives.DeckTrait.aCharacteristic (Primitives.Predicate.compare [.stat .manaValue] .atMost (.lit 2))))
theorem okLurrusCompanion : Ability.check (DRS.ofList []) lurrusCompanion = [] := by decide
/-- Lutri, the Spellchaser -/
def lutriCompanion : Ability := companion (Primitives.DeckCondition.groupAgrees (Primitives.NounPhrase.agree .cardName .differ (allOf (Primitives.Predicate.and [Primitives.Predicate.not land, Primitives.Predicate.isCard]))))
theorem okLutriCompanion : Ability.check (DRS.ofList []) lutriCompanion = [] := by decide
/-- Obosh, the Preypiercer -/
def oboshCompanion : Ability :=
  companion (Primitives.DeckCondition.everyCardIs Primitives.Predicate.isCard (Primitives.DeckTrait.anyOf [Primitives.DeckTrait.manaValueParity .odd, Primitives.DeckTrait.aCharacteristic land]))
theorem okOboshCompanion : Ability.check (DRS.ofList []) oboshCompanion = [] := by decide
/-- Umori, the Collector -/
def umoriCompanion : Ability := companion (Primitives.DeckCondition.groupAgrees (Primitives.NounPhrase.agree .cardType .share (allOf (Primitives.Predicate.and [Primitives.Predicate.not land, Primitives.Predicate.isCard]))))
theorem okUmoriCompanion : Ability.check (DRS.ofList []) umoriCompanion = [] := by decide
/-- Yorion, Sky Nomad -/
def yorionCompanion : Ability := companion (Primitives.DeckCondition.deckSizeOverMinimum 20)
theorem okYorionCompanion : Ability.check (DRS.ofList []) yorionCompanion = [] := by decide
/-- Zirda, the Dawnwaker -/
def zirdaCompanion : Ability :=
  companion (Primitives.DeckCondition.everyCardIs permanentCard (Primitives.DeckTrait.hasAbilityOf .anyActivated))
theorem okZirdaCompanion : Ability.check (DRS.ofList []) zirdaCompanion = [] := by decide

/-- Deadpool, Trading Card -/
def deadpoolTradingCard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Deadpool, Trading Card", cost := some [generic 2, pip .black, pip .red],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Mutant", creatureType "Mercenary", creatureType "Hero"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.instead (Primitives.WouldClause.would Primitives.Subject.any (Primitives.Instruction.move thisCreature Primitives.EventSource.anywhere (some battlefield) .arrival none [])) (offer (Primitives.Instruction.establish (Primitives.StaticSpec.textChange thisCreature (Primitives.TextChange.exchangeWith (a (Primitives.Predicate.and [creature, Primitives.Predicate.otherThan Primitives.NounPhrase.this])))) none)) none .nextTimeOnly none),
          at_ (Primitives.GameEvent.beginningOf .upkeep (some Primitives.NounPhrase.you) (Primitives.TurnRef.each none)) (loseLife (.lit 3)),
          activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 3], Primitives.Cost.perform (sacrifice thisCreature)])
            (instruct (each otherPlayer) (Primitives.Instruction.draw (.lit 1))) ],
      power := stat 5, toughness := stat 3 } }

end Semantics.Cards

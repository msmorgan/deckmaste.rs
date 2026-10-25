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

def jump : Action := gain (target isCreature) flying untilEndOfTurn
theorem okJump : Action.check (DRS.ofList []) jump = [] := by decide
def gabrielAngelfire : Action :=
  sequentially
    [ choose (a (qualityFrom .ability
        (abilitiesAmong [flying, firstStrike, trample, .theWith rampage 3]))),
      gain thisCreature (thatAbility .theChoice) (untilEvent (beginningOf .upkeep (some you) nextTime)) ]
theorem okGabrielAngelfire : Action.check (DRS.ofList []) gabrielAngelfire = [] := by decide
def builtToSmash : Action :=
  sequentially
    [ get (target (and [isCreature, isAttacking])) (up 3) (up 3) untilEndOfTurn,
      doIf (is it (and [isArtifact, isCreature]))
        (gain it trample untilEndOfTurn) ]
theorem okBuiltToSmash : Action.check (DRS.ofList []) builtToSmash = [] := by decide

/-- A flying Thopter token. -/
def thopterToken : CharacteristicBundle :=
  { characteristics :=
    { types := [.artifact, .creature], subtypes := [thopter],
      text := [flying], power := stat 1, toughness := stat 1 } }

def aviationPioneer : Action := createToken thopterToken
theorem okAviationPioneer : Action.check (DRS.ofList []) aviationPioneer = [] := by decide
def fireNavyTrebuchet : Action :=
  createTappedAttacking 1
    { characteristics :=
      { name := "Ballistic Boulder", types := [.artifact, .creature],
        subtypes := [construct], text := [flying],
        power := stat 2, toughness := stat 1 } }
theorem okFireNavyTrebuchet : Action.check (DRS.ofList []) fireNavyTrebuchet = [] := by decide

def rorixBladewing : Spelled := spelled <| singleFacedCard
  { name := "Rorix Bladewing", cost := [generic 3, pip .red, pip .red, pip .red],
    supertypes := [.legendary], types := [.creature], subtypes := [dragon],
    text := [flying, haste], power := stat 6, toughness := stat 5 }

def yotianSoldier : Spelled := spelled <| singleFacedCard
  { name := "Yotian Soldier", cost := [generic 3], types := [.artifact, .creature],
    subtypes := [soldier], text := [vigilance],
    power := stat 1, toughness := stat 4 }

/-- Pym Particles -/
def pymParticlesGrant : Action :=
  sequentially
    [ gain (target isCreature) vigilance untilEndOfTurn,
      establishThroughout (cant any (block (that creature))) thisTurn ]
theorem okPymParticlesGrant : Action.check (DRS.ofList []) pymParticlesGrant = [] := by decide
def bladebrand : Action := gain (target isCreature) deathtouch untilEndOfTurn
theorem okBladebrand : Action.check (DRS.ofList []) bladebrand = [] := by decide
def criticalHit : Action :=
  gain (target isCreature) doubleStrike untilEndOfTurn
theorem okCriticalHit : Action.check (DRS.ofList []) criticalHit = [] := by decide
def lightningBlow : Action :=
  gain (target isCreature) firstStrike untilEndOfTurn
theorem okLightningBlow : Action.check (DRS.ofList []) lightningBlow = [] := by decide
def deathByDragons : Action :=
  instruct (each (and [isPlayer, otherThan (target isPlayer)])) (createToken { characteristics :=
        { colors := [.red], types := [.creature], subtypes := [dragon],
          text := [flying], power := stat 5, toughness := stat 5 } })
theorem okDeathByDragons : Action.check (DRS.ofList []) deathByDragons = [] := by decide

/-- Concordant Crossroads -/
def concordantCrossroads : Spelled := spelled <| singleFacedCard
  { name := "Concordant Crossroads", cost := [pip .green], supertypes := [.world],
    types := [.enchantment], text := [static (abilityGrant (allOf isCreature) haste)]
        }

/-- Bristlepack Sentry -/
def bristlepackSentry : Spelled := spelled <| singleFacedCard
  { name := "Bristlepack Sentry", cost := [generic 1, pip .green], types := [.creature],
    subtypes := [plant, wolf],
    text :=
      [ defender,
        static (onlyWhile
          (can thisCreature attack
             (asThough := some
                (holds (is it (not (hasKeyword defender))))))
          (exists_ (and [ isCreature, youControl,
                           hasPower .atLeast 4 ]))) ],
    power := stat 3, toughness := stat 3 }

def platinumAngel : Spelled := spelled <| singleFacedCard
  { name := "Platinum Angel", cost := [generic 7], types := [.artifact, .creature],
    subtypes := [angel],
    text :=
      [ flying,
        static (cant you loseGame),
        static (cant yourOpponents winGame) ],
    power := stat 4, toughness := stat 4 }

def abyssalPersecutor : Spelled := spelled <| singleFacedCard
  { name := "Abyssal Persecutor", cost := [generic 2, pip .black, pip .black],
    types := [.creature], subtypes := [demon],
    text :=
      [ flying, trample,
        static (cant you winGame),
        static (cant yourOpponents loseGame) ],
    power := stat 6, toughness := stat 6 }

def smogElemental : Spelled := spelled <| singleFacedCard
  { name := "Smog Elemental", cost := [generic 4, pip .black, pip .black],
    types := [.creature], subtypes := [elemental],
    text :=
      [ flying,
        static (getsPt
          (allOf (and [ isCreature, hasKeyword flying,
                         opponentsControl ]))
          (down 1) (down 1)) ],
    power := stat 3, toughness := stat 3 }

def exquisiteArchangel : Spelled := spelled <| singleFacedCard
  { name := "Exquisite Archangel", cost := [generic 5, pip .white, pip .white],
    types := [.creature], subtypes := [angel],
    text :=
      [ flying,
        static (replace (would you loseGame) (instead (sequentially
            [ exile thisCreature,
              setLife (statOf (.playerStat .startingLifeTotal) you) ]))) ],
    power := stat 5, toughness := stat 5 }

def adelbertSteiner : Spelled := spelled <| singleFacedCard
  { name := "Adelbert Steiner", cost := [generic 1, pip .white], supertypes := [.legendary],
    types := [.creature], subtypes := [human, knight],
    text :=
      [ lifelink,
        static (getsPt thisCreature
          (up (forEach 1 (and [equipment, youControl])))
          (up (forEach 1 (and [equipment, youControl])))) ],
    power := stat 2, toughness := stat 1 }

def inspiringStatuary : Spelled := spelled <| singleFacedCard
  { name := "Inspiring Statuary", cost := [generic 3], types := [.artifact],
    text :=
      [ static (abilityGrant (allOf (and [non isArtifact, isSpell,
          did you (cast which) thisObject])) improvise) ] }

def chiefEngineer : Spelled := spelled <| singleFacedCard
  { name := "Chief Engineer", cost := [generic 1, pip .blue], types := [.creature],
    subtypes := [vedalken, artificer],
    text := [static (abilityGrant (allOf (and [isArtifact, isSpell,
        did you (cast which) thisObject])) convoke)],
    power := stat 1, toughness := stat 3 }

def firesongAndSunspeaker : Ability :=
  static (abilityGrant
    (allOf (and [hasColor .red, isInstantOrSorcery, isSpell, youControl]))
    lifelink)
theorem okFiresongAndSunspeaker : Ability.check (DRS.ofList []) firesongAndSunspeaker = [] := by decide
def briaRiptideRogue : Ability :=
  static (abilityGrant (allOf (and [isCreature, youControl, isOther])) prowess)
theorem okBriaRiptideRogue : Ability.check (DRS.ofList []) briaRiptideRogue = [] := by decide
def narsetEnlightenedExile : Ability :=
  static (abilityGrant (allOf (and [isCreature, youControl])) prowess)
theorem okNarsetEnlightenedExile : Ability.check (DRS.ofList []) narsetEnlightenedExile = [] := by decide
def pontiffOfBlight : Ability :=
  static (abilityGrant (allOf (and [isCreature, youControl, isOther])) extort)
theorem okPontiffOfBlight : Ability.check (DRS.ofList []) pontiffOfBlight = [] := by decide

def prismariTheInspiration : Spelled := spelled <| singleFacedCard
  { name := "Prismari, the Inspiration", cost := [generic 5, pip .blue, pip .red],
    supertypes := [.legendary], types := [.creature],
    subtypes := [elder, dragon],
    text :=
      [ flying,
        ward (payLife 5),
        static (abilityGrant (allOf (and [isInstantOrSorcery, isSpell,
            did you (cast which) thisObject])) storm) ],
    power := stat 7, toughness := stat 7 }

def tomakulHonorGuard : Spelled := spelled <| singleFacedCard
  { name := "Tomakul Honor Guard", cost := [generic 1, pip .green], types := [.creature],
    subtypes := [human, soldier],
    text := [ward [generic 2]], power := stat 3, toughness := stat 1 }

def repentantBlacksmith : Spelled := spelled <| singleFacedCard
  { name := "Repentant Blacksmith", cost := [generic 1, pip .white], types := [.creature],
    subtypes := [human],
    text := [protection (hasColor .red)], power := stat 1, toughness := stat 2 }

def battleSquadron : Spelled := spelled <| singleFacedCard
  { name := "Battle Squadron", cost := [generic 3, pip .red, pip .red], types := [.creature],
    subtypes := [goblin],
    text :=
      [ flying,
        static (ptDefinition thisCreature .bothEach (countOf (and [isCreature, youControl]))) ] }

def eomerOfTheRiddermark : Spelled := spelled <| singleFacedCard
  { name := "Éomer of the Riddermark", cost := [generic 4, pip .red],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, knight],
    text :=
      [ haste,
        whenever (does thisCreature attack)
          (if_ := some (exists_ (and [ isCreature, youControl,
                           superlative .max (.stat .power) (and [isCreature, isPermanent]) ])))
          (createToken (creatureToken 1 1 [.white] [human, soldier])) ],
    power := stat 5, toughness := stat 4 }

def angelicGift : Spelled := spelled <| singleFacedCard
  { name := "Angelic Gift", cost := [generic 1, pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        when (does thisAura enter) (draw 1),
        static (abilityGrant (enchanted creature) flying) ] }

def consulsLieutenant : Spelled := spelled <| singleFacedCard
  { name := "Consul's Lieutenant", cost := [pip .white, pip .white], types := [.creature],
    subtypes := [human, soldier],
    text :=
      [ firstStrike,
        renown 1,
        whenever (does thisCreature attack) (if_ := is it (hasDesignation renowned))
          (get (allOf (and [isCreature, isAttacking, youControl, isOther]))
            (up 1) (up 1) untilEndOfTurn) ],
    power := stat 2, toughness := stat 1 }

def secretsOfTheGoldenCity : Spelled := spelled <| singleFacedCard
  { name := "Secrets of the Golden City", cost := [generic 1, pip .blue, pip .blue],
    types := [.sorcery],
    text :=
      [ ascend,
        spell (replace (draw 2)
          (doIf (is you (hasDesignation citysBlessing)) (instead (draw 3)))) ] }

def bombur : Spelled := spelled <| singleFacedCard
  { name := "Bombur, Gentle Dreamer", cost := [generic 2, pip .red],
    supertypes := [.legendary], types := [.creature],
    subtypes := [dwarf, bard],
    text :=
      [ storied,
        static (onlyUnless (doesntUntap thisCreature you)
          (is you (hasDesignation enduringStory))) ],
    power := stat 5, toughness := stat 3 }

def drachNyen : Spelled := spelled <| singleFacedCard
  { name := "Drach'Nyen", cost := [generic 4, pip .black, pip .red],
    supertypes := [.legendary], types := [.artifact], subtypes := [equipment],
    text :=
      [ when (does thisEquipment enter) (exile (targets (upTo 1) isCreature)),
        static (conjunction
                  [ abilityGrant (equipped creature) menace,
            getsPt (equipped creature) (up (letter .x)) (up 0),
            letterDefinition .x (powerOf (the (in_ (exiledWith thisEquipment)))) ]),
        equip [generic 2] ] }

def colossalGraveReaver : Spelled := spelled <| singleFacedCard
  { name := "Colossal Grave-Reaver", cost := [generic 6, pip .black, pip .green],
    types := [.creature], subtypes := [dragon],
    text :=
      [ flying,
        whenever (does thisCreature enter) (or_ := [does thisCreature attack]) (mill 3),
        whenever
          (does any
             (putInto (counted (orMore 1) (and [isCard, isCreature])) yourGraveyard
                (from_ := yourLibrary)))
          (putOntoBattlefield (someOf (exactly 1) them)) ],
    power := stat 7, toughness := stat 6 }

/-- Grimdancer -/
def grimdancer : Spelled := spelled <| singleFacedCard
  { name := "Grimdancer", cost := [generic 1, pip .black, pip .black], types := [.creature],
    subtypes := [nightmare],
    text :=
      [ static (enters thisCreature
          [withCounters 2
            (distinctChosenKinds [menaceCounter, deathtouchCounter, lifelinkCounter]) .fresh]) ],
    power := stat 3, toughness := stat 3 }

def divineVisitation : Spelled := spelled <| singleFacedCard
  { name := "Divine Visitation", cost := [generic 3, pip .white, pip .white],
    types := [.enchantment],
    text :=
      [ static (replace (would any (createObject anyAmount (token (copyOf (counted (orMore 1) (and [isCreature, isToken]))) []) you)) (instead (createToken
            { characteristics :=
              { colors := [.white], types := [.creature], subtypes := [angel],
                text := [flying,
                         vigilance], power := stat 4, toughness := stat 4 } } (count :=
                                                                                 groupSize)))) ] }

def adrixAndNev : Spelled := spelled <| singleFacedCard
  { name := "Adrix and Nev, Twincasters", cost := [generic 2, pip .green, pip .blue],
    supertypes := [.legendary], types := [.creature],
    subtypes := [merfolk, wizard],
    text :=
      [ ward [generic 2],
        static (replace (would any (createObject anyAmount (token (copyOf (counted (orMore 1) isToken)) []) (some
            you))) (instead (create (times 2 groupSize) asThose []))) ],
    power := stat 2, toughness := stat 2 }

def windZendikon : Spelled := spelled <| singleFacedCard
  { name := "Wind Zendikon", cost := [pip .blue], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isLand,
        static (qualityChange (enchanted land) .sets
          (bundle
            { characteristics :=
              { colors := [.blue], types := [.creature], subtypes := [elemental],
                text := [flying], power := stat 2, toughness := stat 2 } }
            (some .land))),
        when (does (enchanted land) die) (returnToHand (that card)) ] }

def awakenTheBear : Spelled := spelled <| singleFacedCard
  { name := "Awaken the Bear", cost := [generic 2, pip .green], types := [.instant],
    text :=
      [ spell
          (establishFor (target isCreature)
            [getsPt it (up 3) (up 3), abilityGrant it trample]
            untilEndOfTurn) ] }

def spidersilkArmor : Spelled := spelled <| singleFacedCard
  { name := "Spidersilk Armor", cost := [generic 2, pip .green], types := [.enchantment],
    text :=
      [ static (conjunction [ getsPt (allOf (and [isCreature, youControl])) (up 0) (up
                1),
            abilityGrant them reach ]) ] }

def arcaneFlight : Spelled := spelled <| singleFacedCard
  { name := "Arcane Flight", cost := [pip .blue], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (conjunction [ getsPt (enchanted creature) (up 1) (up 1),
            abilityGrant it flying ]) ] }

def bootsOfSpeed : Spelled := spelled <| singleFacedCard
  { name := "Boots of Speed", cost := [pip .red], types := [.artifact],
    subtypes := [equipment],
    text :=
      [ static (conjunction [ getsPt (equipped creature) (up 1) (up 0),
            abilityGrant it haste ]),
        equip [generic 1] ] }

/-- "enchanted creature loses all abilities and is a <colors> <subtype> with base P/T" -/
def enchantedBecomesVanilla (colors : List Color) (subtype : Subtype) (power toughness : Nat) :
    StaticSpec :=
  conjunction [ allAbilityLoss (enchanted creature),
      qualityChange it .sets
        (bundle { characteristics := { colors, types := [.creature], subtypes := [subtype] } }),
      getsPt it (setTo (.lit power)) (setTo (.lit toughness)) ]

def frogify : Spelled := spelled <| singleFacedCard
  { name := "Frogify", cost := [generic 1, pip .blue], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (enchantedBecomesVanilla [.blue] frog 1 1) ] }

def darksteelMutation : Spelled := spelled <| singleFacedCard
  { name := "Darksteel Mutation", cost := [generic 1, pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (conjunction [ qualityChange (enchanted creature) .sets
              (bundle
                 { characteristics := { types := [.artifact, .creature], subtypes := [insect] }
                     }),
            getsBase it 0 1,
            abilityGrant it indestructible,
            allAbilityLoss it ]) ] }

def kenrithsTransformation : Spelled := spelled <| singleFacedCard
  { name := "Kenrith's Transformation", cost := [generic 1, pip .green],
    types := [.enchantment], subtypes := [aura],
    text :=
      [ enchant isCreature,
        when (does thisAura enter) (draw 1),
        static (enchantedBecomesVanilla [.green] elk 3 3) ] }

def amphibianDownpour : Spelled := spelled <| singleFacedCard
  { name := "Amphibian Downpour", cost := [generic 2, pip .blue], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ flash, storm,
        enchant isCreature,
        static (enchantedBecomesVanilla [.blue] frog 1 1) ] }

def lignify : Spelled := spelled <| singleFacedCard
  { name := "Lignify", cost := [generic 1, pip .green], types := [.kindred, .enchantment],
    subtypes := [treefolk, aura],
    text :=
      [ enchant isCreature,
        static (conjunction [ qualityChange (enchanted creature) .sets
              (bundle { characteristics := { subtypes := [treefolk] } }),
            getsBase it 0 4,
            allAbilityLoss it ]) ] }

def nefariousImp : Spelled := spelled <| singleFacedCard
  { name := "Nefarious Imp", cost := [generic 2, pip .black], types := [.creature],
    subtypes := [imp],
    text :=
      [ flying,
        whenever
          (does (counted (orMore 1) (and [isPermanent, youControl])) leave)
          (scry 1) ],
    power := stat 2, toughness := stat 1 }

/-- Ainok Tracker -/
def ainokTracker : Spelled := spelled <| singleFacedCard
  { name := "Ainok Tracker", cost := [generic 5, pip .red], types := [.creature],
    subtypes := [dog, scout],
    text := [firstStrike, morph [generic 4, pip .red]],
    power := stat 3, toughness := stat 3 }

def irenicussVileDuplication : Spelled := spelled <| singleFacedCard
  { name := "Irenicus's Vile Duplication", cost := [generic 3, pip .blue], types := [.sorcery],
    text :=
      [ spell (create 1
          (copyOf (target (and [isCreature, youControl]))
            [alsoHas flying, isntLegendary])
          []) ] }

def cacklingCounterpart : Spelled := spelled <| singleFacedCard
  { name := "Cackling Counterpart", cost := [generic 1, pip .blue, pip .blue],
    types := [.instant],
    text :=
      [ spell (create 1
          (copyOf (target (and [isCreature, youControl]))) []),
        flashback [generic 5, pip .blue, pip .blue] ] }

def chandrasSpitfire : Spelled := spelled <| singleFacedCard
  { name := "Chandra's Spitfire", cost := [generic 2, pip .red], types := [.creature],
    subtypes := [elemental],
    text :=
      [ flying,
        whenever (does any (dealDamage (to := anOpponent) (kind := some .noncombat)))
          (get thisCreature (up 3) (up 0) untilEndOfTurn) ],
    power := stat 1, toughness := stat 3 }

def livingHive : Spelled := spelled <| singleFacedCard
  { name := "Living Hive", cost := [generic 6, pip .green, pip .green], types := [.creature],
    subtypes := [elemental, insect],
    text :=
      [ trample,
        whenever (does thisCreature (dealDamage (to := a isPlayer) (kind := some .combat)))
          (createToken (creatureToken 1 1 [.green] [insect]) (count := thatMuch)) ],
    power := stat 6, toughness := stat 6 }

def giantCindermaw : Spelled := spelled <| singleFacedCard
  { name := "Giant Cindermaw", cost := [generic 2, pip .red], types := [.creature],
    subtypes := [dinosaur, beast],
    text := [trample, static (cant (bare isPlayer) (gainLife anyAmount))],
    power := stat 4, toughness := stat 3 }

def lushGrowth : Spelled := spelled <| singleFacedCard
  { name := "Lush Growth", cost := [pip .green], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isLand,
        static (qualityChange (enchanted land) .sets
          (bundle
            { characteristics :=
              { subtypes := [mountain, forest, plains] } })) ] }

def voiceOfAll : Spelled := spelled <| singleFacedCard
  { name := "Voice of All", cost := [generic 2, pip .white, pip .white], types := [.creature],
    subtypes := [angel],
    text :=
      [ flying,
        static (entersChoosing thisCreature .color),
        static (abilityGrant thisCreature (protection (ofChosen .color))) ],
    power := stat 2, toughness := stat 2 }

def wardSliver : Spelled := spelled <| singleFacedCard
  { name := "Ward Sliver", cost := [generic 4, pip .white], types := [.creature],
    subtypes := [sliver],
    text :=
      [ static (entersChoosing thisCreature .color),
        static (abilityGrant (allOf sliver)
          (protection (ofChosen .color))) ],
    power := stat 2, toughness := stat 2 }

def sanctuaryBlade : Spelled := spelled <| singleFacedCard
  { name := "Sanctuary Blade", cost := [generic 2], types := [.artifact],
    subtypes := [equipment],
    text :=
      [ static (attachChoosing thisEquipment .color),
        static (conjunction [ getsPt (equipped creature) (up 2) (up 0),
            abilityGrant (equipped creature)
              (protection (ofTheLastChosen .color)) ]),
        equip [generic 3] ] }

/-- Sinister Strength -/
def sinisterStrength : Spelled := spelled <| singleFacedCard
  { name := "Sinister Strength", cost := [generic 1, pip .black], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (conjunction [ getsPt (enchanted creature) (up 3) (up 1),
            qualityChange it .sets (colored (.some [.black])) ]) ] }

/-- Crimson Wisps -/
def crimsonWisps : Spelled := spelled <| singleFacedCard
  { name := "Crimson Wisps", cost := [pip .red], types := [.instant],
    text :=
      [ spell (sequentially
          [ establishFor (target isCreature)
              [qualityChange it .sets (colored (.some [.red])), abilityGrant it haste]
              untilEndOfTurn,
            draw 1 ]) ] }

/-- Ghoulflesh -/
def ghoulflesh : Spelled := spelled <| singleFacedCard
  { name := "Ghoulflesh", cost := [pip .black], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (conjunction
                  [ getsPt (enchanted creature) (down 1) (down 1),
            qualityChange it .adds
              (bundle
                 { characteristics := { colors := [.black], subtypes := [zombie] } }) ]) ] }

/-- Blade of the Oni -/
def bladeOfTheOniStatic : StaticSpec :=
  conjunction [ getsBase (equipped creature) 5 5,
      abilityGrant it menace,
      qualityChange it .adds
        (bundle { characteristics := { colors := [.black], subtypes := [demon] } }) ]
theorem okBladeOfTheOniStatic : StaticSpec.check (DRS.ofList []) bladeOfTheOniStatic = [] := by decide

def historyOfBenalia : Spelled := spelled <| singleFacedCard
  { name := "History of Benalia", cost := [generic 1, pip .white, pip .white],
    types := [.enchantment], subtypes := [saga],
    text :=
      [ when (chapterMark [1, 2])
          (createToken
            { characteristics :=
              { colors := [.white], types := [.creature], subtypes := [knight],
                text := [vigilance], power := stat 2, toughness := stat 2 } }),
        when (chapterMark [3])
          (get (allOf (and [knight, youControl]))
            (up 2) (up 1) untilEndOfTurn) ] }

/-- Akroan Sergeant -/
def akroanSergeant : Spelled := spelled <| singleFacedCard
  { name := "Akroan Sergeant", cost := [generic 2, pip .red], types := [.creature],
    subtypes := [human, soldier],
    text := [firstStrike, renown 1], power := stat 2, toughness := stat 2 }

def castCreatureSpellsFromTop : StaticSpec :=
  can you (cast (allOf (and [isSpell, isCreature])) (from_ := onTop))
theorem okCastCreatureSpellsFromTop : StaticSpec.check (DRS.ofList []) castCreatureSpellsFromTop = [] := by decide

def garruksHorde : Spelled := spelled <| singleFacedCard
  { name := "Garruk's Horde", cost := [generic 5, pip .green, pip .green], types := [.creature],
    subtypes := [beast],
    text :=
      [ trample,
        static (visibility .reveal you topOfLibrary),
        static castCreatureSpellsFromTop ],
    power := stat 7, toughness := stat 7 }

def wanderingEye : Spelled := spelled <| singleFacedCard
  { name := "Wandering Eye", cost := [generic 2, pip .blue], types := [.creature],
    subtypes := [illusion],
    text := [flying, static (visibility .reveal (bare isPlayer) wholeHand)],
    power := stat 1, toughness := stat 3 }

def amorphousAxe : Spelled := spelled <| singleFacedCard
  { name := "Amorphous Axe", cost := [generic 2], types := [.artifact],
    subtypes := [equipment],
    text :=
      [ static (conjunction [ getsPt (equipped creature) (up 3) (up 0),
            qualityChange (equipped creature) .adds (everyTypeOf
                .creature) ]),
        equip [generic 3] ] }

def runedStalactite : Spelled := spelled <| singleFacedCard
  { name := "Runed Stalactite", cost := [generic 1], types := [.artifact],
    subtypes := [equipment],
    text :=
      [ static (conjunction [ getsPt (equipped creature) (up 1) (up 1),
            qualityChange (equipped creature) .adds (everyTypeOf
                .creature) ]),
        equip [generic 2] ] }

def arachnoform : Spelled := spelled <| singleFacedCard
  { name := "Arachnoform", cost := [generic 1, pip .green], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (conjunction [ getsPt (enchanted creature) (up 2) (up 2),
            abilityGrant (enchanted creature) reach,
            qualityChange (enchanted creature) .adds (everyTypeOf
                .creature) ]) ] }

def nyleasPresence : Spelled := spelled <| singleFacedCard
  { name := "Nylea's Presence", cost := [generic 1, pip .green], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isLand,
        when (does thisAura enter) (draw 1),
        static (qualityChange (enchanted land) .adds (everyTypeOf
            .basicLand)) ] }

/-- Curse of Conformity -/
def curseOfConformity : Spelled := spelled <| singleFacedCard
  { name := "Curse of Conformity", cost := [generic 4, pip .white], types := [.enchantment],
    subtypes := [aura, curse],
    text :=
      [ enchant isPlayer,
        static (conjunction [ getsBase (allOf (and [ isCreature, non (hasSupertype .legendary),
                             controlledBy (enchanted player) ])) 3 3,
            qualityChange them .loses (everyTypeOf .creature) ]) ] }

/-- Drumbellower -/
def drumbellower : Spelled := spelled <| singleFacedCard
  { name := "Drumbellower", cost := [generic 2, pip .white], types := [.creature],
    subtypes := [spirit],
    text :=
      [ flying,
        static (untapDuring (allOf (and [isCreature, youControl]))
          (each isOtherPlayer)) ],
    power := stat 2, toughness := stat 1 }

def tayamLine : StaticSpec :=
  entersWithAdditionalCounters
    (each (and [isCreature, youControl, isOther])) 1
    vigilanceCounter
theorem okTayamLine : StaticSpec.check (DRS.ofList []) tayamLine = [] := by decide

def clarionConqueror : Spelled := spelled <| singleFacedCard
  { name := "Clarion Conqueror", cost := [generic 2, pip .white], types := [.creature],
    subtypes := [dragon],
    text :=
      [ flying,
        static (cant any (activate (allOf (and [ abilityHead .anyActivated,
                         abilityOf (allOf (or [isArtifact, isCreature, isPlaneswalker])) ])))) ],
    power := stat 3, toughness := stat 3 }

/-- Kang the Conqueror -/
def kangPowerUpLock : StaticSpec :=
  cant any (activate (allOf (abilityHead (.keyword powerUp))))
theorem okKangPowerUpLock : StaticSpec.check (DRS.ofList []) kangPowerUpLock = [] := by decide

def vipersKiss : Spelled := spelled <| singleFacedCard
  { name := "Viper's Kiss", cost := [pip .black], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (conjunction
                  [ getsPt (enchanted creature) (down 1) (down 1),
            cant any (activate (allOf (and [abilityHead .anyActivated, abilityOf it]))) ]) ] }

def stupefyingTouch : Spelled := spelled <| singleFacedCard
  { name := "Stupefying Touch", cost := [generic 1, pip .blue], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        when (does thisAura enter) (draw 1),
        static (cant any (activate (allOf (and [ abilityHead .anyActivated,
                         abilityOf (enchanted creature) ])))) ] }

def linvalaKeeperOfSilence : Spelled := spelled <| singleFacedCard
  { name := "Linvala, Keeper of Silence", cost := [generic 2, pip .white, pip .white],
    supertypes := [.legendary], types := [.creature], subtypes := [angel],
    text :=
      [ flying,
        static (cant any (activate (allOf (and [ abilityHead .anyActivated,
                         abilityOf (allOf (and [isCreature, opponentsControl])) ])))) ],
    power := stat 3, toughness := stat 4 }

def kyrenLegate : Spelled := spelled <| singleFacedCard
  { name := "Kyren Legate", cost := [generic 1, pip .red], types := [.creature],
    subtypes := [goblin],
    text :=
      [ static (onlyWhile (altCost this_)
          (and
            [ exists_ (and [ isLand, plains,
                              anOpponentControls ]),
              exists_ (and [ isLand, mountain,
                              youControl ]) ])),
        haste ],
    power := stat 1, toughness := stat 1 }

def polarKraken : Spelled := spelled <| singleFacedCard
  { name := "Polar Kraken", cost := [generic 8, pip .blue, pip .blue, pip .blue],
    types := [.creature], subtypes := [kraken],
    text :=
      [ trample,
        static (entersTapped thisCreature),
        cumulativeUpkeep (sacrifice (a isLand)) ],
    power := stat 11, toughness := stat 11 }

def yavimayaAnts : Spelled := spelled <| singleFacedCard
  { name := "Yavimaya Ants", cost := [generic 2, pip .green, pip .green], types := [.creature],
    subtypes := [insect],
    text := [trample, haste, cumulativeUpkeep [pip .green, pip .green]],
    power := stat 5, toughness := stat 1 }

def illusionaryForces : Spelled := spelled <| singleFacedCard
  { name := "Illusionary Forces", cost := [generic 3, pip .blue], types := [.creature],
    subtypes := [illusion],
    text := [flying, cumulativeUpkeep [pip .blue]],
    power := stat 4, toughness := stat 4 }

def vexingSphinx : Spelled := spelled <| singleFacedCard
  { name := "Vexing Sphinx", cost := [generic 1, pip .blue, pip .blue], types := [.creature],
    subtypes := [sphinx],
    text :=
      [ flying,
        cumulativeUpkeep (discard (a (cardIn yourHand))),
        when (does thisCreature die) (draw (countersOn ageCounter it)) ],
    power := stat 4, toughness := stat 4 }

def manaChains : Spelled := spelled <| singleFacedCard
  { name := "Mana Chains", cost := [pip .blue], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (abilityGrant (enchanted creature) (cumulativeUpkeep (mana
            [generic 1]))) ] }

def dreamThief : Spelled := spelled <| singleFacedCard
  { name := "Dream Thief", cost := [generic 2, pip .blue], types := [.creature],
    subtypes := [faerie, rogue],
    text :=
      [ flying,
        when (does thisCreature enter)
          (doOnlyIf (draw 1)
            (did you (cast (a
              (and [isSpell, hasColor .blue,
              isOther]))) soFarThisTurn)) ],
    power := stat 2, toughness := stat 1 }

def brightspearZealot : Spelled := spelled <| singleFacedCard
  { name := "Brightspear Zealot", cost := [generic 2, pip .white], types := [.creature],
    subtypes := [human, soldier],
    text :=
      [ vigilance,
        static (onlyWhile (getsPt thisCreature (up 2) (up 0))
          (compareAmt (eventCount (cast (a isSpell)) you soFarThisTurn) .atLeast 2)) ],
    power := stat 2, toughness := stat 4 }

def deepwayNavigator : Spelled := spelled <| singleFacedCard
  { name := "Deepway Navigator", cost := [pip .white, pip .blue], types := [.creature],
    subtypes := [merfolk, wizard],
    text :=
      [ flash,
        when (does thisCreature enter)
          (untap
            (each (and [ merfolk, youControl,
                          isOther ]))),
        static (onlyWhile
          (getsPt (allOf (and [merfolk, youControl]))
            (up 1) (up 0))
          (did you (attackWith (counted (orMore 3)
            merfolk)) soFarThisTurn)) ],
    power := stat 2, toughness := stat 2 }

def bloodfireEnforcers : Spelled := spelled <| singleFacedCard
  { name := "Bloodfire Enforcers", cost := [generic 3, pip .red], types := [.creature],
    subtypes := [human, monk],
    text :=
      [ static (onlyWhile
          (conjunction [abilityGrant thisCreature firstStrike, abilityGrant it
              trample])
          (and
            [ exists_ (and [isInstant, in_ yourGraveyard]),
              exists_ (and [isSorcery, in_ yourGraveyard]) ])) ],
    power := stat 5, toughness := stat 2 }

def stormOfSouls : Action :=
  sequentially
    [ returnToBattlefield (allOf (and [isCreature, in_ yourGraveyard])) (from_ := some yourGraveyard),
      become them
        { characteristics :=
          { subtypes := [spirit], text := [flying],
            power := stat 1, toughness := stat 1 } },
      exile this_ ]
theorem okStormOfSouls : Action.check (DRS.ofList []) stormOfSouls = [] := by decide

def answeredPrayers : Spelled := spelled <| singleFacedCard
  { name := "Answered Prayers", cost := [generic 1, pip .white, pip .white],
    types := [.enchantment],
    text :=
      [ when (does (a (and [isCreature, youControl])) enter)
          (sequentially
            [ gainLife 1,
              doIf (not (is
                  thisEnchantment isCreature))
                (become thisEnchantment
                  { characteristics :=
                    { types := [.creature], subtypes := [angel],
                      text := [flying], power := stat 3, toughness := stat 3 } }
                  untilEndOfTurn) ]) ] }

/-- Hate Mirage's middle two sentences -/
def hateMirageTokens : Action :=
  sequentially
    [ doForEach (targets (upTo 2) (and [isCreature, youDontControl]))
        (create 1 (copyOf it) []),
      gain thoseTokens haste ]
theorem okHateMirageTokens : Action.check (DRS.ofList []) hateMirageTokens = [] := by decide
def descentOfTheDragons : Action :=
  sequentially
    [ destroy (targets anyNumber isCreature),
      doForEach (each (and [isCreature, was (destroy which) thisWay]))
        (instruct (controllerOf it) (createToken { characteristics :=
              { colors := [.red], types := [.creature], subtypes := [dragon],
                text := [flying], power := stat 4, toughness := stat 4 } })) ]
theorem okDescentOfTheDragons : Action.check (DRS.ofList []) descentOfTheDragons = [] := by decide
/-- Brimaz, King of Oreskos -/
def brimazAttackToken : Ability :=
  whenever (does thisCreature attack)
    (create 1
      (written
        { characteristics :=
          { colors := [.white], types := [.creature], subtypes := [cat, soldier],
            text := [vigilance], power := stat 1, toughness := stat 1 } })
      [entersAttacking none])
theorem okBrimazAttackToken : Ability.check (DRS.ofList []) brimazAttackToken = [] := by decide
/-- Sedris, the Traitor King -/
def sedrisTheTraitorKing : Ability :=
  static (abilityGrant (each (and [isCreature, in_ yourGraveyard]))
    (unearth [generic 2, pip .black]))
theorem okSedrisTheTraitorKing : Ability.check (DRS.ofList []) sedrisTheTraitorKing = [] := by decide
/-- Grixis -/
def grixis : Ability :=
  static (abilityGrant
    (allOf (and [ andOr [hasColor .blue, hasColor .black, hasColor .red], isCreature,
                   in_ yourGraveyard ]))
    (unearth itsManaCost))
theorem okGrixis : Ability.check (DRS.ofList []) grixis = [] := by decide
/-- Dralnu, Lich Lord -/
def dralnuLichLord : Action :=
  gain (target (and [isInstantOrSorcery, in_ yourGraveyard]))
    (flashback itsManaCost) untilEndOfTurn
theorem okDralnuLichLord : Action.check (DRS.ofList []) dralnuLichLord = [] := by decide
/-- Dregscape Zombie -/
def dregscapeZombie : Ability := unearth [pip .black]
theorem okDregscapeZombie : Ability.check (DRS.ofList []) dregscapeZombie = [] := by decide
/-- Think Twice -/
def thinkTwice : Ability := flashback [generic 2, pip .blue]
theorem okThinkTwice : Ability.check (DRS.ofList []) thinkTwice = [] := by decide
/-- Greater Mossdog -/
def greaterMossdog : Ability := dredge 3
theorem okGreaterMossdog : Ability.check (DRS.ofList []) greaterMossdog = [] := by decide
/-- Raven's Crime -/
def ravensCrime : Ability := retrace
theorem okRavensCrime : Ability.check (DRS.ofList []) ravensCrime = [] := by decide
/-- Barkhide Mauler -/
def barkhideMauler : Ability := cycling [generic 2]
theorem okBarkhideMauler : Ability.check (DRS.ofList []) barkhideMauler = [] := by decide
/-- Ninja of the New Moon -/
def ninjaOfTheNewMoon : Ability := ninjutsu [generic 3, pip .black]
theorem okNinjaOfTheNewMoon : Ability.check (DRS.ofList []) ninjaOfTheNewMoon = [] := by decide
/-- Thunderous Wrath -/
def thunderousWrath : Ability := miracle [pip .red]
theorem okThunderousWrath : Ability.check (DRS.ofList []) thunderousWrath = [] := by decide
/-- Bygone Colossus -/
def bygoneColossus : Ability := warp [generic 3]
theorem okBygoneColossus : Ability.check (DRS.ofList []) bygoneColossus = [] := by decide

/-- Ragavan, Nimble Pilferer. Dash's "return the permanent this spell becomes" is a Resolution
Delayed Trigger [CR#702.109a,608.3g]. -/
def ragavanNimblePilferer : Spelled := spelled <| singleFacedCard
  { name := "Ragavan, Nimble Pilferer", cost := [pip .red], supertypes := [.legendary],
    types := [.creature], subtypes := [monkey, pirate],
    text :=
      [ whenever (does thisCreature (dealDamage (to := a isPlayer) (kind := some .combat)))
          (sequentially
            [ createToken treasureToken,
              exile (slice .top 1 (libraryOf (that player))),
              establishThroughout (can you (cast (that card))) untilEndOfTurn ]),
        dash [generic 1, pip .red] ],
    power := stat 2, toughness := stat 1 }
theorem okRagavanNimblePilferer : ragavanNimblePilferer.card.check = [] := ragavanNimblePilferer.ok

/-- Tenacious Underdog. Cast from the graveyard "using its blitz ability", the spell takes blitz's
alternative cost, so blitz's cost was paid and its Resolution Delayed Trigger sacrifices the
permanent [CR#702.152a,601.2b,608.3g]. -/
def tenaciousUnderdog : Spelled := spelled <| singleFacedCard
  { name := "Tenacious Underdog", cost := [generic 1, pip .black], types := [.creature],
    subtypes := [human, warrior],
    text :=
      [ blitz (compound [[generic 2, pip .black, pip .black], payLife 2]),
        static (can you
          (cast this_ (from_ := yourGraveyard) (payment := usingKeyword blitz))) ],
    power := stat 3, toughness := stat 2 }
theorem okTenaciousUnderdog : tenaciousUnderdog.card.check = [] := tenaciousUnderdog.ok

/-- Starbreach Whale. Warp's "exile the permanent this spell becomes" is a Resolution Delayed
Trigger [CR#702.185a,608.3g]. -/
def starbreachWhale : Spelled := spelled <| singleFacedCard
  { name := "Starbreach Whale", cost := [generic 4, pip .blue], types := [.creature],
    subtypes := [whale],
    text :=
      [ flying,
        when (does thisCreature enter) (surveil 2),
        warp [generic 1, pip .blue] ],
    power := stat 3, toughness := stat 5 }
theorem okStarbreachWhale : starbreachWhale.card.check = [] := starbreachWhale.ok
/-- Knight of Grace — "Hexproof from black" [CR#702.11d] -/
def knightOfGraceHexproofFromBlack : Ability := hexproofFrom (hasColor .black)
theorem okKnightOfGraceHexproofFromBlack :
    Ability.check (DRS.ofList []) knightOfGraceHexproofFromBlack = [] := by decide
/-- Eternal Dragon — "Plainscycling {2}" [CR#702.29e] -/
def eternalDragonPlainscycling : Ability :=
  typecycling plains [generic 2]
theorem okEternalDragonPlainscycling : Ability.check (DRS.ofList []) eternalDragonPlainscycling = [] := by decide
/-- Nezumi Ronin -/
def nezumiRonin : Ability := bushido 1
theorem okNezumiRonin : Ability.check (DRS.ofList []) nezumiRonin = [] := by decide
/-- Dragonlord Ojutai -/
def dragonlordOjutaiHexproof : Ability :=
  static (onlyWhile (abilityGrant thisCreature hexproof) (is it isUntapped))
theorem okDragonlordOjutaiHexproof : Ability.check (DRS.ofList []) dragonlordOjutaiHexproof = [] := by decide
/-- Monoxa, Midway Manager -/
def monoxaRollTrigger : Ability :=
  whenever (does you (rollDice (watch := resultIn (orMore 3))))
    (sequentially
      [ gain thisCreature firstStrike untilEndOfTurn,
        doIf (compareAmt (theOutcome .rollResult) .atLeast 4)
          (gain it menace untilEndOfTurn),
        doIf (compareAmt (theOutcome .rollResult) .atLeast 5)
          (gain it lifelink untilEndOfTurn) ])
theorem okMonoxaRollTrigger : Ability.check (DRS.ofList []) monoxaRollTrigger = [] := by decide
/-- Celebr-8000 -/
def celebr8000Doubles : Action :=
  sequentially
    [ rollDice 2 (.sides 6),
      doIf rolledDoubles (gain thisCreature doubleStrike untilEndOfTurn) ]
theorem okCelebr8000Doubles : Action.check (DRS.ofList []) celebr8000Doubles = [] := by decide

/-- Krosan Druid -/
def krosanDruid : Spelled := spelled <| singleFacedCard
  { name := "Krosan Druid", cost := [generic 2, pip .green], types := [.creature],
    subtypes := [centaur, druid],
    text :=
      [ kicker [generic 4, pip .green],
        when (does thisCreature enter) (if_ := some (keywordCostPaid kicker
                                                         thisCreature))
          (gainLife 10) ],
    power := stat 2, toughness := stat 3 }

/-- Lightkeeper of Emeria -/
def lightkeeperOfEmeria : Spelled := spelled <| singleFacedCard
  { name := "Lightkeeper of Emeria", cost := [generic 3, pip .white], types := [.creature],
    subtypes := [angel],
    text :=
      [ multikicker [pip .white],
        flying,
        when (does thisCreature enter)
          (gainLife (times 2 (timesPaid (.byKeyword kicker) thisCreature))) ],
    power := stat 2, toughness := stat 4 }

/-- Merfolk Falconer -/
def merfolkFalconer : Spelled := spelled <| singleFacedCard
  { name := "Merfolk Falconer", cost := [generic 3, pip .blue, pip .blue], types := [.creature],
    subtypes := [merfolk, wizard],
    text :=
      [ flying,
        whenever
          (does you (cast (a (compareOver isSpell (paidCostRead (.byKeyword kicker)
              none it) .atLeast 1))))
          (scry 2) ],
    power := stat 4, toughness := stat 4 }

/-- Borrowed Malevolence -/
def borrowedMalevolence : Spelled := spelled <| singleFacedCard
  { name := "Borrowed Malevolence", cost := [pip .black], types := [.instant],
    text :=
      [ escalate [generic 2],
        spell (chooseModes (oneThrough 2)
          [ get (target isCreature) (up 1) (up 1) untilEndOfTurn,
            get (target isCreature) (down 1) (down 1) untilEndOfTurn ]) ] }

/-- New Perspectives: "As long as you have seven or more cards in hand, you may pay {0} rather
than pay cycling costs." Cycling is an activated ability [CR#702.29a], and an alternative cost
replaces an activation cost as it replaces a spell's mana cost [CR#118.9,602.2b]. -/
def newPerspectivesCyclingAltCost : Ability :=
  static (onlyWhile (altCost (allOf (abilityHead (.keyword cycling))) [generic 0])
    (compareAmt (countOf (cardIn yourHand)) .atLeast 7))
theorem okNewPerspectivesCyclingAltCost :
    Ability.check (DRS.ofList []) newPerspectivesCyclingAltCost = [] := by decide
/-- Whether an ability is an alternative cost that applies only "as long as" a condition holds. -/
def altCostAsLongAs : Ability → Bool
  | .static (.conditional (.altCost ..) _ .asLongAs) => true
  | _ => false
/-- The condition-less spelling drops the printed "as long as you have seven or more cards in
hand": the card's alternative cost sits under a `conditional … .asLongAs`, which that spelling
lacks. -/
theorem badNewPerspectivesWithoutCondition :
    altCostAsLongAs newPerspectivesCyclingAltCost = true ∧
      altCostAsLongAs (static (altCost (allOf (abilityHead (.keyword cycling))) [generic 0]))
        = false := by decide
def newPerspectives : Spelled := spelled <| singleFacedCard
  { name := "New Perspectives", cost := [generic 5, pip .blue], types := [.enchantment],
    text :=
      [ when (does thisEnchantment enter) (draw 3),
        newPerspectivesCyclingAltCost ] }

/-- Thick-Skinned Goblin: "You may pay {0} rather than pay the echo cost for permanents you
control." Echo's cost is paid while its triggered ability resolves [CR#702.30a], neither a
spell's mana cost nor an activation cost, so it is no alternative cost: the payment of echo's
cost (the `keyword` field) is the event a replacement watches, and paying {0} is what may happen
in its place, the shape an "instead" effect takes [CR#614.1a]. Declining the `may` leaves the
echo cost to be paid. -/
def thickSkinnedGoblinEchoPayment : Ability :=
  static (replace
    (would you (pay anyCost .once .paid (some (allOf (and [isPermanent, youControl])))
      (some echo)))
    (instead (may (pay [generic 0] .once))))
theorem okThickSkinnedGoblinEchoPayment :
    Ability.check (DRS.ofList []) thickSkinnedGoblinEchoPayment = [] := by decide
def thickSkinnedGoblin : Spelled := spelled <| singleFacedCard
  { name := "Thick-Skinned Goblin", cost := [generic 1, pip .red], types := [.creature],
    subtypes := [goblin, shaman],
    text :=
      [ thickSkinnedGoblinEchoPayment,
        activated [pip .red]
          (gain thisCreature (protection (hasColor .red)) untilEndOfTurn) ],
    power := stat 2, toughness := stat 1 }
/-- Fumiko the Lowblood -/
def fumikoBushidoX : Ability :=
  static (conjunction [ abilityGrant thisCreature (bushido (letter .x)),
      letterDefinition .x (countOf isAttacking) ])
theorem okFumikoBushidoX : Ability.check (DRS.ofList []) fumikoBushidoX = [] := by decide

/-- Rafter Demon -/
def rafterDemon : Spelled := spelled <| singleFacedCard
  { name := "Rafter Demon", cost := [generic 2, pip .black, pip .red], types := [.creature],
    subtypes := [demon],
    text :=
      [ spectacle [generic 3, pip .black, pip .red],
        when (does thisCreature enter)
          (if_ := keywordCostPaid spectacle thisCreature)
          (instruct eachOpponent (discard (a (cardIn hand)))) ],
    power := stat 4, toughness := stat 2 }

/-- Tourach, Dread Cantor -/
def tourachDiscardTrigger : Ability :=
  whenever (does anOpponent (discard (a
    isCard)))
    (putCounters 1 p1p1Counter thisCreature)
theorem okTourachDiscardTrigger : Ability.check (DRS.ofList []) tourachDiscardTrigger = [] := by decide

/-- Shimmering Glasskite -/
def shimmeringGlasskite : Spelled := spelled <| singleFacedCard
  { name := "Shimmering Glasskite", cost := [generic 3, pip .blue], types := [.creature],
    subtypes := [spirit],
    text :=
      [ flying,
        whenever
          (nthOccurrence (.nth 1)
            (does (a (or [isSpell, abilityHead .anyOnStack])) (target thisCreature)) (some .turn))
          (counter (that .stack)) ],
    power := stat 2, toughness := stat 3 }

/-- Conqueror's Pledge -/
def conquerorsPledge : Spelled := spelled <| singleFacedCard
  { name := "Conqueror's Pledge", cost := [generic 2, pip .white, pip .white, pip .white],
    types := [.sorcery],
    text :=
      [ kicker [generic 6],
        spell (replace
          (createToken (creatureToken 1 1 [.white] [kor, soldier]) (count := 6))
          (doIf (keywordCostPaid kicker this_)
            (instead (create 12 asThose [])))) ] }

/-- Soul of Emancipation -/
def soulOfEmancipation : Ability :=
  when (does thisCreature enter)
    (sequentially
      [ destroy (targets (upTo 3) (and [isPermanent, non isLand, isOther])),
        doForEach (those permanent)
          (instruct (controllerOf (that permanent)) (createToken { characteristics :=
                { colors := [.white], types := [.creature], subtypes := [angel],
                  text := [flying], power := stat 3, toughness := stat 3 } })) ])
theorem okSoulOfEmancipation : Ability.check (DRS.ofList []) soulOfEmancipation = [] := by decide

/-- Tourach, Dread Cantor -/
def tourachDreadCantor : Spelled := spelled <| singleFacedCard
  { name := "Tourach, Dread Cantor", cost := [generic 1, pip .black],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, cleric],
    text :=
      [ kicker [pip .black, pip .black],
        protection (hasColor .white),
        tourachDiscardTrigger,
        when (does thisCreature enter) (if_ := some (keywordCostPaid kicker
                                                         thisCreature))
          (instruct (target isOpponent) (discard (countedAtRandom (exactly 2) isCard))) ],
    power := stat 2, toughness := stat 1 }

/-- Talrand's Invocation -/
def talrandsInvocation : Action :=
  createToken
    { characteristics :=
      { colors := [.blue], types := [.creature], subtypes := [drake],
        text := [flying], power := stat 2, toughness := stat 2 } } (count := 2)
theorem okTalrandsInvocation : Action.check (DRS.ofList []) talrandsInvocation = [] := by decide

/-- Predatory Wurm -/
def predatoryWurm : Spelled := spelled <| singleFacedCard
  { name := "Predatory Wurm", cost := [generic 3, pip .green], types := [.creature],
    subtypes := [wurm],
    text :=
      [ vigilance,
        static (onlyWhile (getsPt thisCreature (up 2) (up 2))
          (exists_ (and [garruk, youControl]))) ],
    power := stat 4, toughness := stat 4 }

/-- Tower Winder -/
def towerWinder : Spelled := spelled <| singleFacedCard
  { name := "Tower Winder", cost := [generic 1, pip .green], types := [.creature],
    subtypes := [snake],
    text :=
      [ reach, deathtouch,
        when (does thisCreature enter)
          (sequentially
            [ searchLibraryOrGraveyard (a (named "Command Tower")),
              reveal it, putInto it hand,
              doIf (did you (search yourLibrary) thisWay) shuffle ]) ],
    power := stat 1, toughness := stat 1 }

/-- Aim High -/
def aimHigh : Spelled := spelled <| singleFacedCard
  { name := "Aim High", cost := [generic 1, pip .green], types := [.instant],
    text :=
      [ spell (sequentially
          [ untap (target isCreature),
            establishFor it [getsPt it (up 2) (up 2), abilityGrant it reach]
              untilEndOfTurn ]) ] }

/-- Harried Dronesmith -/
def harriedDronesmithToken : Action :=
  sequentially [createToken thopterToken, gainHaste thatToken untilEndOfTurn]
theorem okHarriedDronesmithToken : Action.check (DRS.ofList []) harriedDronesmithToken = [] := by decide

/-- Archfiend's Vessel -/
def archfiendsVessel : Spelled := spelled <| singleFacedCard
  { name := "Archfiend's Vessel", cost := [pip .black], types := [.creature],
    subtypes := [human, cleric],
    text :=
      [ lifelink,
        when (does thisCreature enter)
          (if_ := some (or
            [ did it (enter (from_ := yourGraveyard)) triggering,
              is it (did you (cast which (from_ := yourGraveyard)) thisObject) ]))
          (doIfDone (exile it)
            (some <| createToken
              { characteristics :=
                { colors := [.black], types := [.creature], subtypes := [demon],
                  text := [flying], power := stat 5, toughness := stat 5 } })) ],
    power := stat 1, toughness := stat 1 }

/-- Containment Priest -/
def containmentPriest : Spelled := spelled <| singleFacedCard
  { name := "Containment Priest", cost := [generic 1, pip .white], types := [.creature],
    subtypes := [human, cleric],
    text :=
      [ flash,
        static (replace
                  (would (a (and [isCreature, isNontoken,
                      not (did any (cast which) thisObject)])) enter)
                  (instead (exile it))) ],
    power := stat 2, toughness := stat 2 }

/-- Veiling Oddity -/
def veilingOddityLine : Ability :=
  when (lastCounterRemoved timeCounter this_)
    (while_ := whileTrue (is this_ (cardIn exileZone)))
    (establishThroughout (cant any (block (allOf isCreature))) thisTurn)
theorem okVeilingOddityLine : Ability.check (DRS.ofList []) veilingOddityLine = [] := by decide

def wizenedSnitches : Spelled := spelled <| singleFacedCard
  { name := "Wizened Snitches", cost := [generic 3, pip .blue], types := [.creature],
    subtypes := [faerie, rogue],
    text := [flying, static (visibility .reveal (bare isPlayer) topOfLibrary)],
    power := stat 1, toughness := stat 3 }

/-- Darkblade Agent -/
def darkbladeAgentGrants : Ability :=
  static (onlyWhile (conjunction [ abilityGrant thisCreature deathtouch,
        abilityGrant thisCreature
          (whenever
             (does thisCreature (dealDamage (to := a isPlayer) (kind := some .combat))) (draw
                                                                                                  1)) ]) (did you (surveil anyAmount) soFarThisTurn))
theorem okDarkbladeAgentGrants : Ability.check (DRS.ofList []) darkbladeAgentGrants = [] := by decide
/-- Frenzied Gorespawn -/
def frenziedGorespawnMenaceTrigger : Ability :=
  whenever (does (counted (orMore 1) isCreature) (attack anOpponent))
      (gain (those creature) menace untilEndOfTurn)
theorem okFrenziedGorespawnMenaceTrigger :
    Ability.check (DRS.ofList []) frenziedGorespawnMenaceTrigger = [] := by decide
/-- Pir, Imaginative Rascal's replacement -/
def pirDistributive : Ability :=
  static (replace
            (would any
               (putCounters anyAmount anyKind
                  (a (and [isPermanent, controlledBy (youOr (a isTeammate))]))))
                      (instead (putCounters (plus thatMuch 1) thoseCounters (that permanent))))
theorem okPirDistributive : Ability.check (DRS.ofList []) pirDistributive = [] := by decide

/-- Frogmite -/
def frogmite : Spelled := spelled <| singleFacedCard
  { name := "Frogmite", cost := [generic 4], types := [.artifact, .creature],
    subtypes := [frog], text := [affinity isArtifact],
    power := stat 2, toughness := stat 2 }

/-- Iymrith, Desert Doom -/
def iymrithDesertDoom : Spelled := spelled <| singleFacedCard
  { name := "Iymrith, Desert Doom", cost := [generic 3, pip .blue, pip .blue],
    supertypes := [.legendary], types := [.creature], subtypes := [dragon],
    text :=
      [ flying,
        static (onlyWhile (abilityGrant thisCreature (ward [generic
            4]))
          (is it isUntapped)),
        whenever (does thisCreature (dealDamage (to := a isPlayer) (kind := some .combat)))
          (sequentially
            [ draw 1,
              doIf (compareAmt (countOf (cardIn yourHand)) .less 3)
                (draw theDifference) ]) ],
    power := stat 5, toughness := stat 5 }

def protectionAbilities : KeywordTerm := .anyIn ⟨protection, none⟩
def landwalkAbilities : KeywordTerm := .anyIn ⟨landwalk, none⟩
/-- Hexproof and each "hexproof from [quality]" [CR#702.11d]. -/
def hexproofAbilities : KeywordTerm := .anyIn ⟨hexproof, none⟩
/-- Shay Cormac -/
def wardAbilities : KeywordTerm := .anyIn ⟨ward, none⟩
/-- Tolaria, Shelkin Brownie -/
def bandsWithOtherAbilities : KeywordTerm := .anyIn ⟨bandsWithOther, none⟩
def protectionFromAnyColor : KeywordTerm := .anyIn ⟨protection, some .color⟩

/-- Cairn Wanderer -/
def cairnWanderer : Spelled := spelled <| singleFacedCard
  { name := "Cairn Wanderer", cost := [generic 4, pip .black], types := [.creature],
    subtypes := [shapeshifter],
    text :=
      [ changeling,
        alsoForKeywords
          (fun k => static (onlyWhile
            (keywordGrant thisCreature k (and [isCreature, in_ graveyard, hasKeyword k]))
            (exists_ (and [isCreature, in_ graveyard, hasKeyword k]))))
          [ flying, fear, firstStrike, doubleStrike, deathtouch, haste,
            landwalkAbilities, lifelink, protectionAbilities, reach, trample,
            shroud, vigilance ] ],
    power := stat 4, toughness := stat 4 }

/-- Autarch Mammoth -/
def autarchMammoth : Spelled := spelled <| singleFacedCard
  { name := "Autarch Mammoth", cost := [generic 4, pip .green, pip .green], types := [.creature],
    subtypes := [elephant, mount],
    text := [autarchMammothLine, saddle 5],
    power := stat 5, toughness := stat 5 }

/-- Debris Beetle -/
def debrisBeetle : Spelled := spelled <| singleFacedCard
  { name := "Debris Beetle", cost := [generic 2, pip .black, pip .green], types := [.artifact],
    subtypes := [vehicle],
    text :=
      [ trample,
        when (does thisVehicle enter)
          (sequentially [instruct eachOpponent (loseLife 3), gainLife 3]),
        crew 2 ],
    power := stat 6, toughness := stat 6 }

/-- Pir, Imaginative Rascal -/
def pirImaginativeRascal : Spelled := spelled <| singleFacedCard
  { name := "Pir, Imaginative Rascal", cost := [generic 2, pip .green],
    supertypes := [.legendary], types := [.creature], subtypes := [human],
    text :=
      [ partnerWith (named "Toothy, Imaginary Friend"),
        pirDistributive ],
    power := stat 1, toughness := stat 1 }

/-- Market Gnome -/
def marketGnome : Spelled := spelled <| singleFacedCard
  { name := "Market Gnome", cost := [pip .white], types := [.artifact, .creature],
    subtypes := [gnome],
    text :=
      [ when (does thisCreature die) (sequentially [gainLife 1, draw 1]),
        when (does any (exile thisCreature))
          (while_ := whileDoing (does you (activate (a (abilityHead (.keyword craft))))))
          (sequentially [gainLife 1, draw 1]) ],
    power := stat 0, toughness := stat 3 }

/-- Escaped Shapeshifter -/
def escapedShapeshifter : Spelled := spelled <| singleFacedCard
  { name := "Escaped Shapeshifter", cost := [generic 3, pip .blue, pip .blue],
    types := [.creature], subtypes := [shapeshifter],
    text :=
      [ alsoForKeywords
          (fun k => static (onlyWhile
            (keywordGrant thisCreature k
              (and [ isCreature, anOpponentControls, hasKeyword k,
                     not (named "Escaped Shapeshifter") ]))
            (exists_
              (and [ isCreature, anOpponentControls, hasKeyword k,
                     not (named "Escaped Shapeshifter") ]))))
          [flying, firstStrike, trample, protectionFromAnyColor] ],
    power := stat 3, toughness := stat 4 }

/-- Ancestral Blade -/
def ancestralBlade : Spelled := spelled <| singleFacedCard
  { name := "Ancestral Blade", cost := [generic 1, pip .white], types := [.artifact],
    subtypes := [equipment],
    text :=
      [ when (does thisEquipment enter)
          (sequentially
            [ createToken (creatureToken 1 1 [.white] [soldier]),
              attachTo thisEquipment thatToken ]),
        static (getsPt (equipped creature) (up 1) (up 1)),
        equip [generic 1] ] }

/-- Steelclaw Lance -/
def steelclawLance : Spelled := spelled <| singleFacedCard
  { name := "Steelclaw Lance", cost := [pip .black, pip .red], types := [.artifact],
    subtypes := [equipment],
    text :=
      [ static (getsPt (equipped creature) (up 2) (up 2)),
        equipQuality knight [generic 1],
        equip [generic 3] ] }

/-- Commander's Plate's equip lines -/
def commandersPlateEquip : List Ability :=
  [ equipQuality (hasDesignation commander) [generic 3],
    equip [generic 5] ]
theorem okCommandersPlateEquip : Ability.checkText (DRS.ofList []) commandersPlateEquip = [] := by decide
/-- Luxior, Giada's Gift's equip lines -/
def luxiorEquipLines : List Ability :=
  [ equipQuality isPlaneswalker [generic 1],
    equip [generic 3] ]
theorem okLuxiorEquipLines : Ability.checkText (DRS.ofList []) luxiorEquipLines = [] := by decide

/-- Veiling Oddity -/
def veilingOddity : Spelled := spelled <| singleFacedCard
  { name := "Veiling Oddity", cost := [generic 3, pip .blue], types := [.creature],
    subtypes := [illusion],
    text := [suspend 4 [generic 1, pip .blue], veilingOddityLine],
    power := stat 2, toughness := stat 3 }

/-- Enormous Energy Blade -/
def enormousEnergyBlade : Spelled := spelled <| singleFacedCard
  { name := "Enormous Energy Blade", cost := [generic 2, pip .black], types := [.artifact],
    subtypes := [equipment],
    text :=
      [ static (getsPt (equipped creature) (up 4) (up 0)),
        whenever (does any (attachment .attached thisEquipment (a isCreature)))
            (tap (that creature)),
        equip [generic 2] ] }

/-- Grafted Wargear -/
def graftedWargear : Spelled := spelled <| singleFacedCard
  { name := "Grafted Wargear", cost := [generic 3], types := [.artifact],
    subtypes := [equipment],
    text :=
      [ static (getsPt (equipped creature) (up 3) (up 2)),
        whenever (does any (attachment .unattached thisEquipment (a isPermanent)))
          (sacrifice (that permanent)),
        equip [generic 0] ] }

/-- Black Ward -/
def blackWard : Spelled := spelled <| singleFacedCard
  { name := "Black Ward", cost := [pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (retention
          (abilityGrant (enchanted creature) (protection
              (hasColor .black)))
          thisAura) ] }

/-- White Ward -/
def whiteWard : Spelled := spelled <| singleFacedCard
  { name := "White Ward", cost := [pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (retention
          (abilityGrant (enchanted creature) (protection (hasColor .white)))
          thisAura) ] }

/-- Blue Ward -/
def blueWard : Spelled := spelled <| singleFacedCard
  { name := "Blue Ward", cost := [pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (retention
          (abilityGrant (enchanted creature) (protection (hasColor .blue)))
          thisAura) ] }

/-- Red Ward -/
def redWard : Spelled := spelled <| singleFacedCard
  { name := "Red Ward", cost := [pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (retention
          (abilityGrant (enchanted creature) (protection (hasColor .red)))
          thisAura) ] }

/-- Green Ward -/
def greenWard : Spelled := spelled <| singleFacedCard
  { name := "Green Ward", cost := [pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (retention
          (abilityGrant (enchanted creature) (protection (hasColor .green)))
          thisAura) ] }

/-- Cho-Manno's Blessing -/
def choMannosBlessing : Spelled := spelled <| singleFacedCard
  { name := "Cho-Manno's Blessing", cost := [pip .white, pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ flash,
        enchant isCreature,
        static (entersChoosing thisAura .color),
        static (retention
          (abilityGrant (enchanted creature) (protection
              (ofChosen .color)))
          thisAura) ] }

/-- Pentarch Ward -/
def pentarchWard : Spelled := spelled <| singleFacedCard
  { name := "Pentarch Ward", cost := [generic 2, pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (entersChoosing thisAura .color),
        when (does thisAura enter) (draw 1),
        static (retention
          (abilityGrant (enchanted creature) (protection
              (ofChosen .color)))
          thisAura) ] }

/-- Benevolent Blessing -/
def benevolentBlessing : Spelled := spelled <| singleFacedCard
  { name := "Benevolent Blessing", cost := [generic 1, pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ flash,
        enchant isCreature,
        static (entersChoosing thisAura .color),
        static (retention
          (abilityGrant (enchanted creature) (protection
              (ofChosen .color)))
          (allOf (and [ or [aura, equipment],
                         youControl, attachedTo it ]))) ] }

/-- Spectra Ward -/
def spectraWard : Spelled := spelled <| singleFacedCard
  { name := "Spectra Ward", cost := [generic 3, pip .white, pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (conjunction
          [ getsPt (enchanted creature) (up 2) (up 2),
            retention (abilityGrant (enchanted creature) (protection (eachKind .color)))
              (allOf aura) ]
          (subject := some (enchanted creature))) ] }

/-- Pledge of Loyalty -/
def pledgeOfLoyalty : Spelled := spelled <| singleFacedCard
  { name := "Pledge of Loyalty", cost := [generic 1, pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (retention
          (abilityGrant (enchanted creature)
            (protection (eachKind .color (some (allOf (and [isPermanent, youControl]))))))
          thisAura) ] }

/-- Wall of Shards -/
def wallOfShards : Spelled := spelled <| singleFacedCard
  { name := "Wall of Shards", cost := [generic 1, pip .white], supertypes := [.snow],
    types := [.creature], subtypes := [wall],
    text :=
      [ defender, flying,
        cumulativeUpkeep (instruct anOpponent (gainLife 1)) ],
    power := stat 1, toughness := stat 8 }

/-- Earthen Goo -/
def earthenGoo : Spelled := spelled <| singleFacedCard
  { name := "Earthen Goo", cost := [generic 2, pip .red], types := [.creature],
    subtypes := [ooze],
    text :=
      [ trample,
        cumulativeUpkeep (either [pip .red] [pip .green]),
        static (getsPt thisCreature
          (up (times 1 (countersOn ageCounter it)))
          (up (times 1 (countersOn ageCounter it)))) ],
    power := stat 2, toughness := stat 2 }

/-- Mutagen Connoisseur -/
def mutagenConnoisseur : Spelled := spelled <| singleFacedCard
  { name := "Mutagen Connoisseur", cost := [generic 1, pip .green, pip .blue],
    types := [.creature], subtypes := [vedalken, mutant],
    text :=
      [ flying, vigilance,
        static (getsPt thisCreature
          (up (forEach 1 (and [isTransformed, youControl]))) (up 0)) ],
    power := stat 0, toughness := stat 5 }

/-- Bedrock Tortoise -/
def bedrockTortoiseWindow : StaticSpec :=
  partScope .turn you (abilityGrant (allOf (and [isCreature, youControl])) hexproof)
theorem okBedrockTortoiseWindow : StaticSpec.check (DRS.ofList []) bedrockTortoiseWindow = [] := by decide

/-- Battlegate Mimic -/
def battlegateMimic : Spelled := spelled <| singleFacedCard
  { name := "Battlegate Mimic", cost := [generic 1, hybridPip .red .white], types := [.creature],
    subtypes := [shapeshifter],
    text :=
      [ whenever (does you (cast (a
        (and [isSpell, hasColor .red,
        hasColor .white]))))
          (establishThroughout (conjunction [ getsBase thisCreature 4 2,
                abilityGrant thisCreature firstStrike ]) untilEndOfTurn) ],
    power := stat 2, toughness := stat 1 }

/-- Sphinx of Uthuun -/
def sphinxOfUthuun : Spelled := spelled <| singleFacedCard
  { name := "Sphinx of Uthuun", cost := [generic 5, pip .blue, pip .blue], types := [.creature],
    subtypes := [sphinx],
    text :=
      [ flying,
        when (does thisCreature enter)
          (sequentially
            [ reveal (topSlice 5),
              instruct anOpponent (separateIntoPiles them 2),
              putInto onePile hand,
              putInto (theOther .pile) graveyard ]) ],
    power := stat 5, toughness := stat 6 }

/-- Marit Lage -/
def maritLage : CharacteristicBundle :=
  { characteristics :=
    { name := "Marit Lage", colors := [.black], supertypes := [.legendary], types := [.creature],
      subtypes := [avatar], text := [flying, indestructible],
      power := stat 20, toughness := stat 20 } }

/-- Tuktuk the Explorer -/
def tuktukTheExplorer : Spelled := spelled <| singleFacedCard
  { name := "Tuktuk the Explorer", cost := [generic 2, pip .red], supertypes := [.legendary],
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
    power := stat 1, toughness := stat 1 }

/-- Artificer's Assistant -/
def artificersAssistant : Spelled := spelled <| singleFacedCard
  { name := "Artificer's Assistant", cost := [pip .blue], types := [.creature],
    subtypes := [bird],
    text :=
      [ flying,
        whenever (does you (cast (a
          (and [isHistoric, isSpell])))) (scry 1)
            ],
    power := stat 1, toughness := stat 1 }

/-- Aya of Alexandria -/
def ayaOfAlexandria : Spelled := spelled <| singleFacedCard
  { name := "Aya of Alexandria", cost := [generic 2, pip .red, pip .white],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, assassin],
    text :=
      [ menace, lifelink,
        whenever
          (does (a (and [isHistoric, isCreature, youControl]))
             (dealDamage (to := a isPlayer) (kind := some .combat)))
          (createToken
            { characteristics :=
              { colors := [.black], types := [.creature], subtypes := [assassin],
                text := [menace], power := stat 1, toughness := stat 1 } }) ],
    power := stat 4, toughness := stat 3 }

/-- Indestructibility -/
def indestructibility : Spelled := spelled <| singleFacedCard
  { name := "Indestructibility", cost := [generic 3, pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isPermanent,
        static (abilityGrant (enchanted permanent) indestructible) ] }

/-- Seraphic Greatsword -/
def seraphicGreatsword : Spelled := spelled <| singleFacedCard
  { name := "Seraphic Greatsword", cost := [generic 1, pip .white], types := [.artifact],
    subtypes := [equipment],
    text :=
      [ static (getsPt (equipped creature) (up 2) (up 2)),
        whenever
          (does (equipped creature)
             (attack
                (some
                   (a (and [isPlayer, superlative .max (.playerStat .lifeTotal) isPlayer])))))
          (create 1
            (written
              { characteristics :=
                { colors := [.white], types := [.creature], subtypes := [angel],
                  text := [flying], power := stat 4, toughness := stat 4 } })
            [entersAs .tapped, entersAttacking (that player)]),
        equip [generic 4] ] }

def giantGrowth : Action :=
  get (target isCreature) (up 3) (up 3) untilEndOfTurn
theorem okGiantGrowth : Action.check (DRS.ofList []) giantGrowth = [] := by decide
def amassZombiesTwo : Action := amass zombie 2
theorem okAmassZombiesTwo : Action.check (DRS.ofList []) amassZombiesTwo = [] := by decide
/-- Avarice Totem -/
def avariceTotemExchange : Action :=
  exchange (controlOf thisArtifact (target (and [isPermanent, non isLand])))
theorem okAvariceTotemExchange : Action.check (DRS.ofList []) avariceTotemExchange = [] := by decide
/-- Arcanum Wings -- aura swap [CR#702.65a] -/
def arcanumWingsAuraSwap : Ability :=
  activated [generic 2, pip .blue]
    (may
      (exchange (cardsAcross thisAura (a (and [aura, in_
          hand])))))
theorem okArcanumWingsAuraSwap : Ability.check (DRS.ofList []) arcanumWingsAuraSwap = [] := by decide
/-- Tovolar, Dire Overlord -/
def tovolarNightfall : Action := setGameDesignation night
theorem okTovolarNightfall : Action.check (DRS.ofList []) tovolarNightfall = [] := by decide
/-- Spin into Myth -/
def spinIntoMyth : Action :=
  sequentially [putInto (target isCreature) onTop, fateseal 2]
theorem okSpinIntoMyth : Action.check (DRS.ofList []) spinIntoMyth = [] := by decide
/-- Pure // Simple -/
def pureHalf : Action := destroy (target (and [isPermanent, isMulticolored]))
theorem okPureHalf : Action.check (DRS.ofList []) pureHalf = [] := by decide
/-- Korlash -/
def grandeurDiscardCost : Cost :=
  (discard (a (and [named "Korlash, Heir to Blackblade", isOther, in_ yourHand])))
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
      (anyOf [ aCharacteristic cat,
                aCharacteristic elemental,
                aCharacteristic nightmare,
                aCharacteristic dinosaur,
                aCharacteristic beast ]))
theorem okKaheeraCompanion : Ability.check (DRS.ofList []) kaheeraCompanion = [] := by decide
/-- Keruga, the Macrosage -/
def kerugaCompanion : Ability :=
  companion
    (everyCardIs isCard
      (anyOf [ aCharacteristic (hasManaValue .atLeast 3),
                aCharacteristic isLand ]))
theorem okKerugaCompanion : Ability.check (DRS.ofList []) kerugaCompanion = [] := by decide
/-- Lurrus of the Dream-Den -/
def lurrusCompanion : Ability :=
  companion
    (everyCardIs isPermanentCard (aCharacteristic (hasManaValue .atMost 2)))
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
def deadpoolTradingCard : Spelled := spelled <| singleFacedCard
  { name := "Deadpool, Trading Card", cost := [generic 2, pip .black, pip .red],
    supertypes := [.legendary], types := [.creature],
    subtypes := [mutant, mercenary, hero],
    text :=
      [ static
          (replace (would thisCreature enter)
            (alongside
              (may
                (establish
                  (textChange thisCreature
                    (exchangeWith (a (and [isCreature, isOther])))))))),
        at_ (beginningOfYour .upkeep) (loseLife 3),
        activated (compound [[generic 3], sacrifice thisCreature])
          (instruct (each isOtherPlayer) (draw 1)) ],
    power := stat 5, toughness := stat 3 }

/-- Frenzy Sliver -/
def frenzySliver : Spelled := spelled <| singleFacedCard
  { name := "Frenzy Sliver", cost := [generic 1, pip .black], types := [.creature],
    subtypes := [sliver],
    text := [static (abilityGrant (allOf (and [sliver, isCreature])) (frenzy 1))],
    power := stat 1, toughness := stat 1 }

/-- Eagle Vision -/
def eagleVision : Spelled := spelled <| singleFacedCard
  { name := "Eagle Vision", cost := [generic 4, pip .blue], types := [.sorcery],
    text := [freerunning [generic 1, pip .blue], spell (draw 3)] }

/-- Briarblade Adept -/
def briarbladeAdept : Spelled := spelled <| singleFacedCard
  { name := "Briarblade Adept", cost := [generic 4, pip .black], types := [.creature],
    subtypes := [elf, assassin],
    text :=
      [ whenever (does thisCreature attack)
          (get (target (and [isCreature, opponentsControl])) (down 1) (down 1) untilEndOfTurn),
        encore [generic 3, pip .black] ],
    power := stat 3, toughness := stat 4 }

/-- Blaring Captain -/
def blaringCaptain : Spelled := spelled <| singleFacedCard
  { name := "Blaring Captain", cost := [generic 3, pip .black], types := [.creature],
    subtypes := [azra, warrior],
    text :=
      [ partnerWith (named "Blaring Recruiter"),
        whenever (does thisCreature attack)
          (get (allOf (and [warrior, isAttacking])) (up 1) (up 1) untilEndOfTurn) ],
    power := stat 2, toughness := stat 2 }

/-- Eyetwitch -/
def eyetwitch : Spelled := spelled <| singleFacedCard
  { name := "Eyetwitch", cost := [pip .black], types := [.creature], subtypes := [eye, bat],
    text := [flying, when (does thisCreature die) learn],
    power := stat 1, toughness := stat 1 }

/-- Countersculpt -/
def countersculpt : Spelled := spelled <| singleFacedCard
  { name := "Countersculpt", cost := [pip .blue, pip .blue], types := [.instant],
    text :=
      [ static (addedCost (or [behold jace, [generic 1]]) false),
        spell (sequentially [counter (target isSpell), empowerJace 1]) ] }

/-- Germination Practicum -/
def germinationPracticum : Spelled := spelled <| singleFacedCard
  { name := "Germination Practicum", cost := [generic 3, pip .green, pip .green],
    types := [.sorcery], subtypes := [lesson],
    text :=
      [ spell (putCounters 2 p1p1Counter (each (and [isCreature, youControl]))),
        paradigm ] }

/-- Out of Bounds -/
def outOfBounds : Spelled := spelled <| singleFacedCard
  { name := "Out of Bounds", cost := [generic 3, pip .blue], types := [.instant],
    text := [assist, spell (counter (target isSpell))] }

/-- Mourning Patrol // Morning Apparition -/
def mourningPatrol : Spelled := spelled <| .transforming
  { characteristics :=
    { name := "Mourning Patrol", cost := [generic 2, pip .white], types := [.creature],
      subtypes := [human, soldier], text := [vigilance, disturb [generic 3, pip .white]],
      power := stat 2, toughness := stat 3 } }
  { characteristics :=
    { name := "Morning Apparition", colors := [.white], types := [.creature],
      subtypes := [spirit, soldier],
      text :=
        [ flying, vigilance,
          static (replace (would any (putInto thisCreature graveyard)) (instead (exile it))) ],
      power := stat 2, toughness := stat 1 } }

/-- Nissa, Ascended Animist -/
def nissaAscendedAnimist : Spelled := spelled <| singleFacedCard
  { name := "Nissa, Ascended Animist",
    cost := [generic 3, pip .green, pip .green, phyrexianPip .green, phyrexianPip .green],
    supertypes := [.legendary], types := [.planeswalker], subtypes := [nissa],
    text :=
      [ compleated,
        activated (loyaltySymbol (up 1))
          (sequentially
            [ createToken (creatureTokenOf (letter .x) (letter .x) [.green] [phyrexian, horror]),
              define .x (statOf (.stat .loyalty) thisPlaneswalker) ]),
        activated (loyaltySymbol (down 1)) (destroy (target (or [isArtifact, isEnchantment]))),
        activated (loyaltySymbol (down 7))
          (establishFor (allOf (and [isCreature, youControl]))
            [ getsPt them (up (forEach 1 (and [forest, youControl])))
                (up (forEach 1 (and [forest, youControl]))),
              abilityGrant them trample ]
            untilEndOfTurn) ],
    loyalty := stat 7 }

end Semantics.Cards

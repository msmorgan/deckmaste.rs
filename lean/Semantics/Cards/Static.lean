import Semantics
import Semantics.Macros
import Semantics.Check.Card
import Semantics.Cards.Choice
import Semantics.Cards.Trigger

/-!
# Semantics.Cards.Static

Port of `idris/src/Experimental/Cards/Static.idr`: the printed cards of the Static family
(continuous clauses: pumps, definitions, ascriptions, locks, alternative costs, visibility) and
the bench items beside them.

`maro`, `peopleOfTheWoods` and `cultivatorColossus` print `*` boxes: those slots are `none`
under the printed-star ruling, and the characteristic-defining clauses set them.
-/

open Semantics Semantics.Macros

namespace Semantics.Cards

def forkedBolt : Instruction :=
  dealsDivided .this (.lit 2) (.described (.target (oneThrough 2)) anyTarget)
theorem okForkedBolt : Instruction.check [] forkedBolt = [] := by decide
def thoughtReflection : Ability :=
  .static (.intercepts (.draws .you) [] none (.draw .you (.lit 2)) .repeatedly none)
theorem okThoughtReflection : Ability.check [] thoughtReflection = [] := by decide
def jorKadeen : Ability :=
  .static (onlyWhile (getsPt (allOf creatureYouControl) (.up (.lit 3)) (.up (.lit 0)))
    (.compareAmt (countOf (.and [artifact, .hasPossessor .controller .you])) .atLeast (.lit 3)))
theorem okJorKadeen : Ability.check [] jorKadeen = [] := by decide
def abandonedOutpost : Ability := .static (entersTapped thisLand)
theorem okAbandonedOutpost : Ability.check [] abandonedOutpost = [] := by decide
/-- Time Vault -/
def timeVaultLock : Ability := .static (doesntUntap thisArtifact (some .you))
theorem okTimeVaultLock : Ability.check [] timeVaultLock = [] := by decide
def hymnOfRebirth : Instruction :=
  putOntoBattlefieldUnderYourControl (target (.and [creature, .inZone graveyard]))
theorem okHymnOfRebirth : Instruction.check [] hymnOfRebirth = [] := by decide
def counterspell : Instruction := .counterSpell (target spell)
theorem okCounterspell : Instruction.check [] counterspell = [] := by decide

def anthemOfChampions : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Anthem of Champions", cost := some [pip .green, pip .white], types := [.enchantment],
      text := [.static (getsPt (allOf creatureYouControl) (.up (.lit 1)) (.up (.lit 1)))] } }

def adantoVanguard : Ability :=
  .static (onlyWhile (getsPt thisCreature (.up (.lit 2)) (.up (.lit 0)))
    (.matches thisCreature attacking))
theorem okAdantoVanguard : Ability.check [] adantoVanguard = [] := by decide
/-- Nowhere to Run -/
def nowhereToRunTargetLine : StaticSpec :=
  canBeTargetedAsThough (allOf creatureYourOpponentsControl)
    (allOf (.or [spell, .abilityHead .anyOnStack])) (.not (.hasKeyword (.the "Hexproof")))
theorem okNowhereToRunTargetLine : StaticSpec.check [] nowhereToRunTargetLine = [] := by decide
/-- Hithlain Rope -/
def hithlainRopeSacrificeLock : StaticSpec := objectCant (.action "Sacrifice") .this
theorem okHithlainRopeSacrificeLock : StaticSpec.check [] hithlainRopeSacrificeLock = [] := by
  decide
/-- Lich's Mastery -/
def lichsMasteryGate : Ability := .static (playerCant (.core .loseGame) .you)
theorem okLichsMasteryGate : Ability.check [] lichsMasteryGate = [] := by decide
def theGoldenThrone : Ability :=
  .static (.intercepts (.losesGame .you) [] none
    (.sequentially [exile thisArtifact, lifeBecomes .you (.lit 1)]) .repeatedly none)
theorem okTheGoldenThrone : Ability.check [] theGoldenThrone = [] := by decide
def stunningReversal : Ability :=
  .spell none (.continuously
    (.intercepts (.losesGame .you) [] none
      (.sequentially [.draw .you (.lit 7), lifeBecomes .you (.lit 1)]) .nextTimeOnly none)
    (some .thisTurn))
theorem okStunningReversal : Ability.check [] stunningReversal = [] := by decide
def pathOfBravery : Ability :=
  .static (onlyWhile (getsPt (allOf creatureYouControl) (.up (.lit 1)) (.up (.lit 1)))
    (.compareAmt (lifeTotalOf .you) .atLeast (.statOf (.playerStat .startingLifeTotal) .you)))
theorem okPathOfBravery : Ability.check [] pathOfBravery = [] := by decide

def deathsShadow : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Death's Shadow", cost := some [pip .black], types := [.creature],
      subtypes := [creatureType "Avatar"],
      text :=
        [ .static (.andAlso none
            [ .modify thisCreature .power (.down (.letter .x)),
              .modify thisCreature .toughness (.down (.letter .x)),
              .definesLetter .x (lifeTotalOf .you) ]) ],
      power := stat 13, toughness := stat 13 } }

def spontaneousMutation : Ability :=
  .static (.andAlso none
    [ .modify (.attachHost .enchanted (.type .creature)) .power (.down (.letter .x)),
      .modify (.attachHost .enchanted (.type .creature)) .toughness (.down (.lit 0)),
      .definesLetter .x (countOf (.inZone (graveyardOf .you))) ])
theorem okSpontaneousMutation : Ability.check [] spontaneousMutation = [] := by decide

def maro : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Maro", cost := some [generic 2, pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text := [.static (.definesPt thisCreature .bothEach (countOf (.inZone (handOf .you))))] } }

def peopleOfTheWoods : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "People of the Woods", cost := some [pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Human"],
      text :=
        [ .static (.definesPt thisCreature .toughnessAlone
            (countOf (.and [.hasSubtype (landType "Forest"), .hasPossessor .controller .you]))) ],
      power := stat 1 } }

def scourgeOfTheSkyclaves : Ability :=
  .static (.definesPt thisCreature .bothEach
    (minus (.lit 20) (aggregate .max (.playerStat .lifeTotal) .anyPlayer)))
theorem okScourgeOfTheSkyclaves : Ability.check [] scourgeOfTheSkyclaves = [] := by decide
def aettirAndPriwen : Ability :=
  .static (.andAlso none
    [ .modify (.attachHost .equipped (.type .creature)) .power (.set (.letter .x)),
      .modify (.attachHost .equipped (.type .creature)) .toughness (.set (.letter .x)),
      .definesLetter .x (lifeTotalOf .you) ])
theorem okAettirAndPriwen : Ability.check [] aettirAndPriwen = [] := by decide

def diminish : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Diminish", cost := some [pip .blue], types := [.instant],
      text :=
        [ .spell none (.continuously (getsBase (target creature) (.lit 1) (.lit 1))
            (some untilEndOfTurn)) ] } }

def cycleOfLife : Instruction :=
  .continuously (getsBase (target (.and [creature, castBy .you])) (.lit 0) (.lit 1))
    (some (.until_ (.startOf .upkeep (some .you))))
theorem okCycleOfLife : Instruction.check [] cycleOfLife = [] := by decide

def aboutFace : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "About Face", cost := some [pip .red], types := [.instant],
      text :=
        [ .spell none (.continuously (.switchesPt (target creature)) (some untilEndOfTurn)) ] } }

def roilingHorror : Ability :=
  .static (.definesPt thisCreature .bothEach
    (minus (lifeTotalOf .you)
      (lifeTotalOf
        (a (.and [.opponent, .superlative .max (.playerStat .lifeTotal) .opponent])))))
theorem okRoilingHorror : Ability.check [] roilingHorror = [] := by decide

/-- Katara, the Fearless -/
def kataraTheFearless : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Katara, the Fearless", cost := some [pip .green, pip .white, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Warrior", creatureType "Ally"],
      text :=
        [ .static (.triggersAdditionally
            (.triggers
              (a (.and [ .abilityHead .anyTriggered,
                         .abilityOf (a (.and [ .hasSubtype (creatureType "Ally"),
                                               .hasPossessor .controller .you ])) ])))
            (exactly 1)) ],
      power := stat 3, toughness := stat 3 } }

/-- Rain of Gore -/
def rainOfGore : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rain of Gore", cost := some [pip .black, pip .red], types := [.enchantment],
      text :=
        [ .static (.intercepts
            (.causes (.source (a (.or [spell, .abilityHead .anyOnStack])))
              (.lifeChanges (controllerOf it) .up))
            [] none (losesLife (that .player) .thatMuch) .repeatedly none) ] } }

/-- Master Chef -/
def masterChefGrantedAbility : Ability :=
  .static (entersWithAdditionalCounters thisCreature (.lit 1) plusOnePlusOne)
theorem okMasterChefGrantedAbility : Ability.check [] masterChefGrantedAbility = [] := by decide

def anointedProcession : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Anointed Procession", cost := some [generic 3, pip .white], types := [.enchantment],
      text :=
        [ .static (.intercepts (tokensCreatedByEffectUnder (counted (atLeast 1) .isToken) .you)
            [] none (.create .you (times (.lit 2) .groupSize) .asThose []) .repeatedly none) ] } }

def naturalAffinity : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Natural Affinity", cost := some [generic 2, pip .green], types := [.instant],
      text :=
        [ .spell none (.continuously
            (.becomes (allOf land) .sets
              (.bundle { characteristics := { types := [.creature], power := stat 2, toughness := stat 2 } }
                (some .land)))
            (some untilEndOfTurn)) ] } }

def turnToFrog : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Turn to Frog", cost := some [generic 1, pip .blue], types := [.instant],
      text :=
        [ .spell none (.continuously
            (.andAlso none
              [ .losesAllAbilities (target creature) none,
                .becomes it .sets
                  (.bundle { characteristics := { colors := [.blue], subtypes := [creatureType "Frog"] } } none),
                .modify it .power (.set (.lit 1)),
                .modify (itsOther it (.set (.lit 1))) .toughness (.set (.lit 1)) ])
            (some untilEndOfTurn)) ] } }

def humility : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Humility", cost := some [generic 2, pip .white, pip .white], types := [.enchantment],
      text :=
        [ .static (.andAlso none
            [ .losesAllAbilities (allOf creature) none,
              .modify them .power (.set (.lit 1)),
              .modify (itsOther them (.set (.lit 1))) .toughness (.set (.lit 1)) ]) ] } }

/-- Tahngarth, First Mate -/
def tahngarthAttacksThatJoin : Instruction := .becomesAttacking thisCreature (some thatJoin)
theorem okTahngarthAttacksThatJoin :
    Instruction.check (Instruction.intro (GameEvent.intro [] tahngarthHeader) tahngarthChoosesDefender)
      tahngarthAttacksThatJoin = [] := by
  decide

def mindlockOrb : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mindlock Orb", cost := some [generic 3, pip .blue], types := [.artifact],
      text := [.static (playerCant (.action "Search") (.playerGroup .allPlayers))] } }

def silence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Silence", cost := some [pip .white], types := [.instant],
      text :=
        [ .spell none (.continuously (playerCant (.action "Cast") (.playerGroup .yourOpponents))
            (some .thisTurn)) ] } }

def shadowOfDoubt : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shadow of Doubt", cost := some [hybridPip .blue .black, hybridPip .blue .black],
      types := [.instant],
      text :=
        [ .spell none (.sequentially
            [ .continuously (playerCant (.action "Search") (.playerGroup .allPlayers))
                (some .thisTurn),
              .draw .you (.lit 1) ]) ] } }

def omenMachineDraw : Ability := .static (playerCant (.core .draw) (.playerGroup .allPlayers))
theorem okOmenMachineDraw : Ability.check [] omenMachineDraw = [] := by decide
def solfataraLandLock : Instruction :=
  .continuously (playerCant (.action "Play") (target .anyPlayer)) (some .thisTurn)
theorem okSolfataraLandLock : Instruction.check [] solfataraLandLock = [] := by decide

/-- "each nonbasic land is a <type>": a moon effect. -/
def nonbasicLandsAre (type : String) : StaticSpec :=
  .becomes (allOf (.and [land, .not (.hasSupertype .basic)])) .sets
    (.bundle { characteristics := { subtypes := [landType type] } } none)

def bloodMoon : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blood Moon", cost := some [generic 2, pip .red], types := [.enchantment],
      text := [.static (nonbasicLandsAre "Mountain")] } }

def magusOfTheMoon : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Magus of the Moon", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text := [.static (nonbasicLandsAre "Mountain")], power := stat 2, toughness := stat 2 } }

def harbingerOfTheSeas : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Harbinger of the Seas", cost := some [generic 1, pip .blue, pip .blue],
      types := [.creature], subtypes := [creatureType "Merfolk", creatureType "Wizard"],
      text := [.static (nonbasicLandsAre "Island")], power := stat 2, toughness := stat 2 } }

def yavimayaCradleOfGrowth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Yavimaya, Cradle of Growth", supertypes := [.legendary], types := [.land],
      text :=
        [ .static (.becomes (allOf land) .adds
            (.bundle { characteristics := { subtypes := [landType "Forest"] } } none)) ] } }

def rallyTheRanks : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rally the Ranks", cost := some [generic 1, pip .white], types := [.enchantment],
      text :=
        [ .static (entersChoosing thisEnchantment (.subtype .creature)),
          .static (getsPt
            (allOf (.and [creature, .hasPossessor .controller .you, ofChosen (.subtype .creature)]))
            (.up (.lit 1)) (.up (.lit 1))) ] } }

def sharedTriumph : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shared Triumph", cost := some [generic 1, pip .white], types := [.enchantment],
      text :=
        [ .static (entersChoosing thisEnchantment (.subtype .creature)),
          .static (getsPt (allOf (.and [creature, ofChosen (.subtype .creature)]))
            (.up (.lit 1)) (.up (.lit 1))) ] } }

def hallOfTriumph : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hall of Triumph", cost := some [generic 3], supertypes := [.legendary],
      types := [.artifact],
      text :=
        [ .static (entersChoosing thisArtifact .color),
          .static (getsPt (allOf (.and [creature, .hasPossessor .controller .you, ofChosen .color]))
            (.up (.lit 1)) (.up (.lit 1))) ] } }

def engineeredPlague : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Engineered Plague", cost := some [generic 2, pip .black], types := [.enchantment],
      text :=
        [ .static (entersChoosing thisEnchantment (.subtype .creature)),
          .static (getsPt (allOf (.and [creature, ofChosen (.subtype .creature)]))
            (.down (.lit 1)) (.down (.lit 1))) ] } }

/-- Volrath's Laboratory -/
def volrathsLaboratoryChoice : StaticSpec :=
  .andAlso none
    [entersChoosing thisArtifact .color, entersChoosing thisArtifact (.subtype .creature)]
theorem okVolrathsLaboratoryChoice : StaticSpec.check [] volrathsLaboratoryChoice = [] := by decide
/-- Call to Arms -/
def callToArmsChoice : StaticSpec :=
  .andAlso none
    [ entersChoosing thisEnchantment .color,
      entersChoosingPlayer thisEnchantment (some (.players .opponent)) ]
theorem okCallToArmsChoice : StaticSpec.check [] callToArmsChoice = [] := by decide

def encroachingMycosynth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Encroaching Mycosynth", cost := some [generic 3, pip .blue], types := [.artifact],
      text :=
        [ .static (.alsoOffBattlefield
            (.becomes (allOf (.and [permanent, .not land, .hasPossessor .controller .you])) .adds
              (.bundle { characteristics := { types := [.artifact] } } none))) ] } }

/-- Darkest Hour -/
def darkestHour : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Darkest Hour", cost := some [pip .black], types := [.enchantment],
      text := [.static (.becomes (allOf creature) .sets (.colored (.some [.black])))] } }

/-- Thran Lens -/
def thranLens : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Thran Lens", cost := some [generic 2], types := [.artifact],
      text := [.static (.becomes (allOf permanent) .sets (.colored (.some [])))] } }

/-- Ghostflame Sliver -/
def ghostflameSliver : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ghostflame Sliver", cost := some [pip .black, pip .red], types := [.creature],
      subtypes := [creatureType "Sliver"],
      text :=
        [ .static (.becomes (allOf (.hasSubtype (creatureType "Sliver"))) .sets (.colored (.some []))) ],
      power := stat 2, toughness := stat 2 } }

/-- Nightcreep -/
def nightcreep : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Nightcreep", cost := some [pip .black, pip .black], types := [.instant],
      text :=
        [ .spell none (.continuously
            (.andAlso none
              [ .becomes (allOf creature) .sets (.colored (.some [.black])),
                .becomes (allOf land) .sets
                  (.bundle { characteristics := { subtypes := [landType "Swamp"] } } none) ])
            (some untilEndOfTurn)) ] } }

/-- Celestial Dawn -/
def celestialDawnAscriptions : List Ability :=
  [ .static (.becomes (allOf (.and [land, .hasPossessor .controller .you])) .sets
      (.bundle { characteristics := { subtypes := [landType "Plains"] } } none)),
    .static (.alsoOffBattlefield
      (.becomes (allOf (.and [permanent, .not land, .hasPossessor .controller .you])) .sets
        (.colored (.some [.white])))) ]
theorem okCelestialDawnAscriptions : Ability.checkText [] celestialDawnAscriptions = [] := by decide

/-- Transguild Courier -/
def transguildCourier : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Transguild Courier", cost := some [generic 4], types := [.artifact, .creature],
      subtypes := [creatureType "Golem"],
      text := [.static (.becomes thisCreature .sets (.colored .every))],
      power := stat 3, toughness := stat 3 } }

/-- Booby Trap -/
def boobyTrapNameChoice : StaticSpec :=
  entersChoosingFrom thisArtifact .cardName (.nameOfCard (.not (.and [.hasSupertype .basic, land])))
theorem okBoobyTrapNameChoice : StaticSpec.check [] boobyTrapNameChoice = [] := by decide

def dovinsVeto : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dovin's Veto", cost := some [pip .white, pip .blue], types := [.instant],
      text :=
        [ .static (objectCant (.action "Counter") .this),
          .spell none (.counterSpell (target (.and [spell, .not creature]))) ] } }

def goblinSpy : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Goblin Spy", cost := some [pip .red], types := [.creature],
      subtypes := [creatureType "Goblin", creatureType "Rogue"],
      text := [.static (.visibility .reveal .you .topOfLibrary)],
      power := stat 1, toughness := stat 1 } }

def futureSight : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Future Sight", cost := some [generic 2, pip .blue, pip .blue, pip .blue],
      types := [.enchantment],
      text :=
        [ .static (.visibility .reveal .you .topOfLibrary),
          .static playLandsAndCastSpellsFromTop ] } }

def magusOfTheFuture : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Magus of the Future", cost := some [generic 2, pip .blue, pip .blue, pip .blue],
      types := [.creature], subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ .static (.visibility .reveal .you .topOfLibrary),
          .static playLandsAndCastSpellsFromTop ],
      power := stat 2, toughness := stat 3 } }

def telepathy : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Telepathy", cost := some [pip .blue], types := [.enchantment],
      text := [.static (.visibility .reveal (.playerGroup .yourOpponents) .wholeHand)] } }

def assembleThePlayers : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Assemble the Players", cost := some [generic 1, pip .white], types := [.enchantment],
      text :=
        [ .static (.visibility .lookAt .you .topOfLibrary),
          .static castSmallCreatureFromTopOnceEachTurn ] } }

def prismaticOmen : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Prismatic Omen", cost := some [generic 1, pip .green], types := [.enchantment],
      text :=
        [ .static (.becomes (allOf (.and [land, .hasPossessor .controller .you])) .adds
            (.everyTypeOf .basicLand)) ] } }

def mistformUltimus : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mistform Ultimus", cost := some [generic 3, pip .blue], supertypes := [.legendary],
      types := [.creature], subtypes := [creatureType "Illusion"],
      text := [.static (.becomes thisCreature .adds (.everyTypeOf .creature))],
      power := stat 3, toughness := stat 3 } }

def volatileClaws : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Volatile Claws", cost := some [generic 2, pip .red], types := [.instant],
      text :=
        [ .spell none (.continuously
            (.andAlso none
              [ .modify (allOf (.and [creature, .hasPossessor .controller .you])) .power (.up (.lit 2)),
                .modify (itsOther (allOf (.and [creature, .hasPossessor .controller .you])) (.up (.lit 2)))
                  .toughness (.up (.lit 0)),
                .becomes (allOf (.and [creature, .hasPossessor .controller .you])) .adds
                  (.everyTypeOf .creature) ])
            (some untilEndOfTurn)) ] } }

/-- Nameless Inversion -/
def namelessInversionBody : Instruction :=
  .continuously
    (.andAlso none
      [ .modify (target creature) .power (.up (.lit 3)),
        .modify (itsOther (target creature) (.up (.lit 3))) .toughness (.down (.lit 3)),
        .becomes it .loses (.everyTypeOf .creature) ])
    (some untilEndOfTurn)
theorem okNamelessInversionBody : Instruction.check [] namelessInversionBody = [] := by decide
/-- Ego Erasure -/
def egoErasureBody : Instruction :=
  .continuously
    (.andAlso none
      [ .modify (allOf (.and [creature, .hasPossessor .controller (target .anyPlayer)])) .power
          (.down (.lit 2)),
        .modify (itsOther (allOf (.and [creature, .hasPossessor .controller (target .anyPlayer)]))
            (.down (.lit 2)))
          .toughness (.up (.lit 0)),
        .becomes them .loses (.everyTypeOf .creature) ])
    (some untilEndOfTurn)
theorem okEgoErasureBody : Instruction.check [] egoErasureBody = [] := by decide
/-- Lithoform Blight -/
def lithoformBlightLoss : StaticSpec :=
  .andAlso none
    [ .becomes (.attachHost .enchanted (.type .land)) .loses (.everyTypeOf .land),
      .losesAllAbilities it none ]
theorem okLithoformBlightLoss : StaticSpec.check [] lithoformBlightLoss = [] := by decide

/-- Energybending -/
def energybending : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Energybending", cost := some [generic 2], types := [.instant],
      subtypes := [spellType "Lesson"],
      text :=
        [ .spell none (.sequentially
            [ .continuously
                (.becomes (allOf (.and [land, .hasPossessor .controller .you])) .adds
                  (.everyTypeOf .basicLand))
                (some untilEndOfTurn),
              .draw .you (.lit 1) ]) ] } }

/-- Seedborn Muse -/
def seedbornMuse : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Seedborn Muse", cost := some [generic 3, pip .green, pip .green],
      types := [.creature], subtypes := [creatureType "Spirit"],
      text :=
        [ .static (untapsDuring (allOf (.hasPossessor .controller .you)) (some (each otherPlayer))) ],
      power := stat 2, toughness := stat 4 } }

/-- Unwinding Clock -/
def unwindingClock : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Unwinding Clock", cost := some [generic 4], types := [.artifact],
      text :=
        [ .static (untapsDuring (allOf (.and [artifact, .hasPossessor .controller .you]))
            (some (each otherPlayer))) ] } }

/-- Thousand Moons Infantry -/
def thousandMoonsInfantry : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Thousand Moons Infantry", cost := some [generic 2, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Soldier"],
      text := [.static (untapsDuring thisCreature (some (each otherPlayer)))],
      power := stat 2, toughness := stat 4 } }

/-- Blatant Thievery -/
def blatantThievery : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blatant Thievery", cost := some [generic 4, pip .blue, pip .blue, pip .blue],
      types := [.sorcery],
      text :=
        [ .spell none (.forEachOf (each .opponent)
            (.continuously
              (.gainsControl .you (target (.hasPossessor .controller (that .player)))) none)) ] } }

/-- "activated abilities of <p> can't be activated" -/
def cantActivateAbilitiesOf (p : Predicate) : StaticSpec :=
  objectCant (.action "Activate")
    (allOf (.and [.abilityHead .anyActivated, .abilityOf (allOf p)]))

def cursedTotem : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cursed Totem", cost := some [generic 2], types := [.artifact],
      text := [.static (cantActivateAbilitiesOf creature)] } }

def nullRod : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Null Rod", cost := some [generic 2], types := [.artifact],
      text := [.static (cantActivateAbilitiesOf artifact)] } }

def stonySilence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stony Silence", cost := some [generic 1, pip .white], types := [.enchantment],
      text := [.static (cantActivateAbilitiesOf artifact)] } }

def dampingMatrix : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Damping Matrix", cost := some [generic 3], types := [.artifact],
      text :=
        [ .static (objectCant (.action "Activate")
            (allOf (.and [ .abilityHead .anyActivated,
                           .abilityOf (allOf (.or [artifact, creature])),
                           .not .isManaAbility ]))) ] } }

def crash : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Crash", cost := some [generic 2, pip .red], types := [.instant],
      text :=
        [ .static (.altCost .this
            (some (.perform (sacrifice .you (a (.and [land, .hasSubtype (landType "Mountain")])))))),
          .spell none (destroy (target artifact)) ] } }

def moggSalvage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mogg Salvage", cost := some [generic 2, pip .red], types := [.instant],
      text :=
        [ .static (onlyWhile (.altCost .this none)
            (.and
              [ exists_ (.and [ land, .hasSubtype (landType "Island"),
                                .hasPossessor .controller anOpponent ]),
                exists_ (.and [ land, .hasSubtype (landType "Mountain"),
                                .hasPossessor .controller .you ]) ])),
          .spell none (destroy (target artifact)) ] } }

def abolish : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Abolish", cost := some [generic 1, pip .white, pip .white], types := [.instant],
      text :=
        [ .static (.altCost .this
            (some (.perform (discard .you (a (.and [.hasSubtype (landType "Plains"), .inZone hand])))))),
          .spell none (destroy (target (.or [artifact, enchantment]))) ] } }

def gush : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gush", cost := some [generic 4, pip .blue], types := [.instant],
      text :=
        [ .static (.altCost .this
            (some (.perform (move
              (counted (exactly 2)
                (.and [land, .hasSubtype (landType "Island"), .hasPossessor .controller .you]))
              hand)))),
          .spell none (.draw .you (.lit 2)) ] } }

def sunscour : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sunscour", cost := some [generic 5, pip .white, pip .white], types := [.sorcery],
      text :=
        [ .static (.altCost .this
            (some (.perform (exiles .you
              (counted (exactly 2) (.and [.colorIs .white, .inZone (handOf .you)])))))),
          .spell none (destroy (allOf creature)) ] } }

def massacre : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Massacre", cost := some [generic 2, pip .black, pip .black], types := [.sorcery],
      text :=
        [ .static (onlyWhile (.altCost .this none)
            (.and
              [ exists_ (.and [ land, .hasSubtype (landType "Plains"),
                                .hasPossessor .controller anOpponent ]),
                exists_ (.and [ land, .hasSubtype (landType "Swamp"),
                                .hasPossessor .controller .you ]) ])),
          .spell none (gets (allOf creature) (.down (.lit 2)) (.down (.lit 2)) (some untilEndOfTurn)) ] } }

def rouse : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rouse", cost := some [generic 1, pip .black], types := [.instant],
      text :=
        [ .static (onlyWhile (.altCost .this (some (payLife .you 2)))
            (exists_ (.and [land, .hasSubtype (landType "Swamp"), .hasPossessor .controller .you]))),
          .spell none (gets (target creature) (.up (.lit 2)) (.up (.lit 0)) (some untilEndOfTurn)) ] } }

def thwart : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Thwart", cost := some [generic 2, pip .blue, pip .blue], types := [.instant],
      text :=
        [ .static (.altCost .this
            (some (.perform (move
              (counted (exactly 3)
                (.and [land, .hasSubtype (landType "Island"), .hasPossessor .controller .you]))
              hand)))),
          .spell none (.counterSpell (target spell)) ] } }

def theLadyOfOtariaLine : StaticSpec :=
  .altCost .this
    (some (.perform (.setStatus .tapped
      (counted (exactly 3)
        (.and [ creature, .hasSubtype (creatureType "Dwarf"), .hasPossessor .controller .you,
                untapped ])))))
theorem okTheLadyOfOtariaLine : StaticSpec.check [] theLadyOfOtariaLine = [] := by decide

/-- Clergy of the Holy Nimbus -/
def clergyOfTheHolyNimbus : Ability :=
  .static (.intercepts (.verbedEvent none (.action "Destroy") (some thisCreature) none) [] none
    (.regenerate it) .repeatedly none)
theorem okClergyOfTheHolyNimbus : Ability.check [] clergyOfTheHolyNimbus = [] := by decide
/-- Rampant Frogantua -/
def rampantFrogantuaPump : Ability :=
  .static (getsPt thisCreature
    (.up (forEach 10 (.and [.anyPlayer, happenedTo .gameLoss .thisGame])))
    (.up (forEach 10 (.and [.anyPlayer, happenedTo .gameLoss .thisGame]))))
theorem okRampantFrogantuaPump : Ability.check [] rampantFrogantuaPump = [] := by decide
/-- Maskwood Nexus -/
def maskwoodNexusTypes : Ability :=
  .static (.alsoOffBattlefield (.becomes (allOf creatureYouControl) .adds (.everyTypeOf .creature)))
theorem okMaskwoodNexusTypes : Ability.check [] maskwoodNexusTypes = [] := by decide
/-- Luxior, Giada's Gift -/
def luxiorEquippedPermanent : Ability :=
  .static (.becomes (.attachHost .equipped .permanent) .adds
    (.bundle { characteristics := { types := [.creature] } } none))
theorem okLuxiorEquippedPermanent : Ability.check [] luxiorEquippedPermanent = [] := by decide
/-- Nahiri, the Unforgiving's compleated reminder -/
def nahiriCompleatedEntry : Ability :=
  .static (entersWithFewerCounters thisPlaneswalker (.lit 2) (.named "Loyalty"))
theorem okNahiriCompleatedEntry : Ability.check [] nahiriCompleatedEntry = [] := by decide
/-- Nimble Mongoose -/
def nimbleMongoose : Ability :=
  abilityWord "threshold"
    (.static (onlyWhile (getsPt thisCreature (.up (.lit 2)) (.up (.lit 2)))
      (.compareAmt (countOf (.inZone (graveyardOf .you))) .atLeast (.lit 7))))
theorem okNimbleMongoose : Ability.check [] nimbleMongoose = [] := by decide
/-- Anax, Hardened in the Forge -/
def anaxPowerDefinition : Ability :=
  .static (.definesPt thisCreature .powerAlone (.devotion .you (.lit .red) none))
theorem okAnaxPowerDefinition : Ability.check [] anaxPowerDefinition = [] := by decide
/-- Aspect of Wolf -/
def aspectOfWolf : Ability :=
  .static (.andAlso none
    [ .modify (.attachHost .enchanted (.type .creature)) .power (.up (.letter .x)),
      .modify (.attachHost .enchanted (.type .creature)) .toughness (.up (.letter .y)),
      .definesLetter .x (.half .down
        (countOf (.and [.hasSubtype (landType "Forest"), .hasPossessor .controller .you]))),
      .definesLetter .y (.half .up
        (countOf (.and [.hasSubtype (landType "Forest"), .hasPossessor .controller .you]))) ])
theorem okAspectOfWolf : Ability.check [] aspectOfWolf = [] := by decide
/-- Lhurgoyf -/
def lhurgoyfDefinition : Ability :=
  .static (.andAlso none
    [ .definesPt thisCreature .powerAlone (countOf (.and [creature, .inZone graveyard])),
      .definesPt thisCreature .toughnessAlone (plus .thatMuch (.lit 1)) ])
theorem okLhurgoyfDefinition : Ability.check [] lhurgoyfDefinition = [] := by decide

/-- Invigorate -/
def invigorate : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Invigorate", cost := some [generic 2, pip .green], types := [.instant],
      text :=
        [ .static (onlyWhile (.altCost .this (some (.perform (gainsLife anOpponent (.lit 3)))))
            (exists_ (.and [land, .hasSubtype (landType "Forest"), .hasPossessor .controller .you]))),
          .spell none (gets (target creature) (.up (.lit 4)) (.up (.lit 4)) (some untilEndOfTurn)) ] } }

/-- Deflecting Swat -/
def deflectingSwatCommanderAltCost : Ability :=
  .static (onlyWhile (.altCost .this none)
    (exists_ (.and [.hasDesignation "commander" none, .hasPossessor .controller .you])))
theorem okDeflectingSwatCommanderAltCost :
    Ability.check [] deflectingSwatCommanderAltCost = [] := by decide

/-- Fist of Suns -/
def fistOfSuns : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fist of Suns", cost := some [generic 3], types := [.artifact],
      text :=
        [ .static (.altCost (allOf (.and [spell, castBy .you]))
            (some (.mana [pip .white, pip .blue, pip .black, pip .red, pip .green]))) ] } }

/-- Rooftop Storm -/
def rooftopStorm : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rooftop Storm", cost := some [generic 5, pip .blue], types := [.enchantment],
      text :=
        [ .static (.altCost
            (allOf (.and [.hasSubtype (creatureType "Zombie"), creature, spell, castBy .you]))
            (some (.mana [generic 0]))) ] } }

/-- Voltage Surge's declaration -/
def voltageSurgeAddedCost : Ability :=
  .static (.addedCost (.perform (sacrifice .you (a artifact))) true)
theorem okVoltageSurgeAddedCost : Ability.check [] voltageSurgeAddedCost = [] := by decide
/-- Tarmogoyf -/
def tarmogoyfDefinition : Ability :=
  .static (.andAlso none
    [ .definesPt thisCreature .powerAlone
        (.distinctCount .cardType (allOf (.inZone (graveyardOf (.playerGroup .allPlayers))))),
      .definesPt thisCreature .toughnessAlone (plus .thatMuch (.lit 1)) ])
theorem okTarmogoyfDefinition : Ability.check [] tarmogoyfDefinition = [] := by decide
/-- Consuming Blob's definition -/
def consumingBlobDefinition : Ability :=
  .static (.andAlso none
    [ .definesPt thisCreature .powerAlone
        (.distinctCount .cardType (allOf (.inZone (graveyardOf .you)))),
      .definesPt thisCreature .toughnessAlone (plus .thatMuch (.lit 1)) ])
theorem okConsumingBlobDefinition : Ability.check [] consumingBlobDefinition = [] := by decide
/-- Nighthawk Scavenger's definition -/
def nighthawkScavengerDefinition : Ability :=
  .static (.definesPt thisCreature .powerAlone
    (plus (.lit 1)
      (.distinctCount .cardType (allOf (.inZone (graveyardOf (.playerGroup .yourOpponents)))))))
theorem okNighthawkScavengerDefinition : Ability.check [] nighthawkScavengerDefinition = [] := by
  decide
/-- Faeburrow Elder's pump -/
def faeburrowElderPump : Ability :=
  .static (getsPt thisCreature
    (.up (times (.lit 1)
      (.distinctCount .color (allOf (.and [permanent, .hasPossessor .controller .you])))))
    (.up (times (.lit 1)
      (.distinctCount .color (allOf (.and [permanent, .hasPossessor .controller .you]))))))
theorem okFaeburrowElderPump : Ability.check [] faeburrowElderPump = [] := by decide
/-- Bonds of Faith -/
def bondsOfFaithPump : Ability :=
  .static (onlyWhile
    (getsPt (.attachHost .enchanted (.type .creature)) (.up (.lit 2)) (.up (.lit 2)))
    (.matches it (.hasSubtype (creatureType "Human"))))
theorem okBondsOfFaithPump : Ability.check [] bondsOfFaithPump = [] := by decide

/-- Field of Dreams -/
def fieldOfDreams : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Field of Dreams", cost := some [pip .blue], supertypes := [.world],
      types := [.enchantment],
      text := [.static (.visibility .reveal (.playerGroup .allPlayers) .topOfLibrary)] } }

/-- Lantern of Insight -/
def lanternOfInsightRider : Ability :=
  .static (.visibility .reveal (.playerGroup .allPlayers) .topOfLibrary)
theorem okLanternOfInsightRider : Ability.check [] lanternOfInsightRider = [] := by decide

/-- Revelation -/
def revelation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Revelation", cost := some [pip .green], supertypes := [.world], types := [.enchantment],
      text := [.static (.visibility .reveal (.playerGroup .allPlayers) .wholeHand)] } }

/-- Phyrexian Unlife -/
def phyrexianUnlifeImmunity : Ability := .static (.noLossFromZeroLife .you)
theorem okPhyrexianUnlifeImmunity : Ability.check [] phyrexianUnlifeImmunity = [] := by decide
/-- Umbris, Fear Manifest -/
def umbrisPump : Ability :=
  .static (getsPt thisCreature
    (.up (forEach 1
      (.and [.isCard, .hasPossessor .owner (.playerGroup .yourOpponents), .inZone exileZone])))
    (.up (forEach 1
      (.and [.isCard, .hasPossessor .owner (.playerGroup .yourOpponents), .inZone exileZone]))))
theorem okUmbrisPump : Ability.check [] umbrisPump = [] := by decide
/-- Turbulent Fen -/
def turbulentFen : Ability :=
  .static (onlyUnless (entersTapped thisLand)
    (.compareAmt (countOf (.and [land, .hasPossessor .controller (.playerGroup .yourOpponents)]))
      .atLeast (.lit 8)))
theorem okTurbulentFen : Ability.check [] turbulentFen = [] := by decide
/-- Bastion Protector -/
def bastionProtectorPump : Ability :=
  .static (getsPt
    (allOf (.and [creature, .hasDesignation "commander" none, .hasPossessor .controller .you]))
    (.up (.lit 2)) (.up (.lit 2)))
theorem okBastionProtectorPump : Ability.check [] bastionProtectorPump = [] := by decide
/-- Luxior, Giada's Gift -/
def luxiorTypeSetting : Ability :=
  .static (.andAlso none
    [ .becomes (.attachHost .equipped .permanent) .loses
        (.bundle { characteristics := { types := [.planeswalker] } } none),
      .becomes it .adds (.bundle { characteristics := { types := [.creature] } } none) ])
theorem okLuxiorTypeSetting : Ability.check [] luxiorTypeSetting = [] := by decide
/-- Kasmina, Enigma Sage -/
def kasminaLoyaltySharing : StaticSpec :=
  .gainsAbilitiesOf
    (allOf (.and [.hasType .planeswalker, .hasPossessor .controller .you, .otherThan .this]))
    [.loyalty] .this none
theorem okKasminaLoyaltySharing : StaticSpec.check [] kasminaLoyaltySharing = [] := by decide
/-- Nicol Bolas, Dragon-God -/
def nicolBolasDragonGodSharing : StaticSpec :=
  .gainsAbilitiesOf .this [.loyalty]
    (allOf (.and [.hasType .planeswalker, .otherThan .this, .inZone battlefield])) none
theorem okNicolBolasDragonGodSharing : StaticSpec.check [] nicolBolasDragonGodSharing = [] := by
  decide
/-- Myr Welder -/
def myrWelderBorrowedAbilities : StaticSpec :=
  .gainsAbilitiesOf thisCreature [.anyActivated] (allOf (.exiledWith .this)) none
theorem okMyrWelderBorrowedAbilities : StaticSpec.check [] myrWelderBorrowedAbilities = [] := by
  decide
/-- Sharkey, Tyrant of the Shire -/
def sharkeyBorrowedLandAbilities : StaticSpec :=
  .gainsAbilitiesOf .this [.anyActivated]
    (allOf (.and [land, .hasPossessor .controller (.playerGroup .yourOpponents)]))
    (some .isManaAbility)
theorem okSharkeyBorrowedLandAbilities : StaticSpec.check [] sharkeyBorrowedLandAbilities = [] := by
  decide

/-- Skill Borrower -/
def skillBorrower : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Skill Borrower", cost := some [generic 2, pip .blue], types := [.artifact, .creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ .static (.visibility .reveal .you .topOfLibrary),
          .static (onlyWhile
            (.gainsAbilitiesOf thisCreature [.anyActivated] (topSlice (.lit 1)) none)
            (.matches (topSlice (.lit 1)) (.or [artifact, creature]))) ],
      power := stat 1, toughness := stat 3 } }

/-- Emissary of Grudges -/
def emissaryOfGrudgesEntry : StaticSpec :=
  entersChoosingPlayerSecretly thisCreature (some (.players .opponent))
theorem okEmissaryOfGrudgesEntry : StaticSpec.check [] emissaryOfGrudgesEntry = [] := by decide

def aerialAssault : Instruction := destroy (target (.and [creature, tapped]))
theorem okAerialAssault : Instruction.check [] aerialAssault = [] := by decide
def asphyxiate : Instruction := destroy (target (.and [creature, untapped]))
theorem okAsphyxiate : Instruction.check [] asphyxiate = [] := by decide
def vindicate : Instruction := destroy (target permanent)
theorem okVindicate : Instruction.check [] vindicate = [] := by decide

def counterspellCard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Counterspell", cost := some [pip .blue, pip .blue], types := [.instant],
      text := [.spell none counterspell] } }

def hymnOfRebirthCard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hymn of Rebirth", cost := some [generic 3, pip .green, pip .white],
      types := [.sorcery], text := [.spell none hymnOfRebirth] } }

def chandrasPyrohelixCard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chandra's Pyrohelix", cost := some [generic 1, pip .red], types := [.instant],
      text := [.spell none forkedBolt] } }

/-- Smite -/
def smite : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Smite", cost := some [pip .white], types := [.instant],
      text := [.spell none (destroy (target (.and [creature, blocked])))] } }

def eerieUltimatum : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Eerie Ultimatum",
      cost := some [pip .white, pip .white, pip .black, pip .black, pip .black, pip .green, pip .green],
      types := [.sorcery],
      text :=
        [ .spell none (move
            (withDifferentNames
              (counted anyNumber (.and [permanentCard, .inZone (graveyardOf .you)])))
            battlefield) ] } }

/-- Gray Merchant of Asphodel -/
def grayMerchantDrain : Instruction :=
  .sequentially
    [losesLife (each .opponent) (.letter .x), .define .x (.devotion .you (.lit .black) none)]
theorem okGrayMerchantDrain : Instruction.check [] grayMerchantDrain = [] := by decide

end Semantics.Cards

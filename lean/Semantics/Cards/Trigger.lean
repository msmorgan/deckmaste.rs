import Semantics
import Semantics.Macros
import Semantics.Check.Card
import Semantics.Cards.Anaphora

/-!
# Semantics.Cards.Trigger

Port of `idris/src/Experimental/Cards/Trigger.idr`: the printed cards of the Trigger family
(headers, intervening-ifs, joined and alternative headers, delayed and replacement shapes) and
the bench items beside them.
-/

open Semantics Semantics.Macros

namespace Semantics.Cards

def gracefulReprieve : Instruction :=
  delayedWithin (.dies (target creature)) .thisTurn (move (that .card) battlefield)
theorem okGracefulReprieve : Instruction.check [] gracefulReprieve = [] := by decide

def cloudkinSeer : Ability := when (.enters thisCreature none) (.draw .you (.lit 1))
theorem okCloudkinSeer : Ability.check [] cloudkinSeer = [] := by decide
def promiseOfTomorrow : Ability := whenever (.dies (a creatureYouControl)) (exile it)
theorem okPromiseOfTomorrow : Ability.check [] promiseOfTomorrow = [] := by decide
def libraryLarcenist : Ability := whenever (attacks thisCreature) (.draw .you (.lit 1))
theorem okLibraryLarcenist : Ability.check [] libraryLarcenist = [] := by decide
def jhessianThief : Ability :=
  whenever (dealsCombatDamage thisCreature (a .anyPlayer)) (.draw .you (.lit 1))
theorem okJhessianThief : Ability.check [] jhessianThief = [] := by decide
def scholarOfStars : Ability :=
  triggeredIf (.enters thisCreature none)
    (exists_ (.and [artifact, .hasPossessor .controller .you])) (.draw .you (.lit 1))
theorem okScholarOfStars : Ability.check [] scholarOfStars = [] := by decide

def miserysShadow : Ability :=
  .static (.intercepts (.dies (a (.and [creature, .hasPossessor .controller (a .opponent)]))) []
    none (exile it) .repeatedly none)
theorem okMiserysShadow : Ability.check [] miserysShadow = [] := by decide

def beastWhisperer : Ability :=
  whenever (.casts .you (a (.and [creature, spell])) none) (.draw .you (.lit 1))
theorem okBeastWhisperer : Ability.check [] beastWhisperer = [] := by decide
def mesmericOrb : Ability :=
  whenever (.statusEvent (a permanent) .untapped)
    (mills (controllerOf (that .permanent)) (.lit 1) they)
theorem okMesmericOrb : Ability.check [] mesmericOrb = [] := by decide
def secretPlans : Ability :=
  whenever (.statusEvent (a (.and [permanent, .hasPossessor .controller .you])) .faceUp)
    (.draw .you (.lit 1))
theorem okSecretPlans : Ability.check [] secretPlans = [] := by decide

/-- Teferi's Imp -/
def teferisImpPhasesOut : Ability :=
  whenever (.statusEvent thisCreature .phasedOut) (discard .you (a (.inZone hand)))
theorem okTeferisImpPhasesOut : Ability.check [] teferisImpPhasesOut = [] := by decide
/-- Teferi's Imp -/
def teferisImpPhasesIn : Ability :=
  whenever (.statusEvent thisCreature .phasedIn) (.draw .you (.lit 1))
theorem okTeferisImpPhasesIn : Ability.check [] teferisImpPhasesIn = [] := by decide

/-- Oubliette -/
def oubliette : Ability :=
  when (.enters thisEnchantment none)
    (phasesOutUntil (target creature) (leavesBattlefield thisEnchantment))
theorem okOubliette : Ability.check [] oubliette = [] := by decide

def shimmeringEfreet : Ability :=
  whenever (.statusEvent thisCreature .phasedIn) (.setStatus .phasedOut (target creature))
theorem okShimmeringEfreet : Ability.check [] shimmeringEfreet = [] := by decide
def hollowhengeSpirit : Ability :=
  when (.enters thisCreature none)
    (.removeFromCombat (target (.and [creature, .or [attacking, blocking]])))
theorem okHollowhengeSpirit : Ability.check [] hollowhengeSpirit = [] := by decide
def netcasterSpider : Ability :=
  whenever (blocks thisCreature (some (a (.and [creature, .hasKeyword (.the "Flying")]))))
    (gets thisCreature (.up (.lit 2)) (.up (.lit 0)) (some untilEndOfTurn))
theorem okNetcasterSpider : Ability.check [] netcasterSpider = [] := by decide
def viashinoWeaponsmith : Ability :=
  whenever (becomesBlocked thisCreature (some (a creature)))
    (gets thisCreature (.up (.lit 2)) (.up (.lit 2)) (some untilEndOfTurn))
theorem okViashinoWeaponsmith : Ability.check [] viashinoWeaponsmith = [] := by decide
def somberwaldAlpha : Ability :=
  whenever (becomesBlocked (a creatureYouControl) none)
    (gets it (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn))
theorem okSomberwaldAlpha : Ability.check [] somberwaldAlpha = [] := by decide
def vertigoSpawn : Ability :=
  whenever (blocks thisCreature (some (a creature)))
    (.sequentially
      [ .setStatus .tapped (that (.type .creature)),
        .doesntUntapNext (that (.type .creature)) (.lit 1) ])
theorem okVertigoSpawn : Ability.check [] vertigoSpawn = [] := by decide
def orneryDilophosaur : Ability :=
  triggeredIf (attacks thisCreature)
    (exists_ (.and [ creature, .hasPossessor .controller .you,
                     .compare [.stat .power] .atLeast (.lit 4) ]))
    (gets thisCreature (.up (.lit 2)) (.up (.lit 2)) (some untilEndOfTurn))
theorem okOrneryDilophosaur : Ability.check [] orneryDilophosaur = [] := by decide
def incisorGlider : Ability :=
  triggeredIf (attacks thisCreature)
    (.compareAmt (countersOn (.named "Poison") (a .opponent)) .atLeast (.lit 3))
    (gets (allOf creatureYouControl) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn))
theorem okIncisorGlider : Ability.check [] incisorGlider = [] := by decide
def stormFleetSpy : Ability :=
  triggeredIf (.enters thisCreature none) (happened .attackDeclaration .you .thisTurn)
    (.draw .you (.lit 1))
theorem okStormFleetSpy : Ability.check [] stormFleetSpy = [] := by decide
def loanShark : Ability :=
  triggeredIf (.enters thisCreature none)
    (.compareAmt (eventCount .spellCast .you .thisTurn) .atLeast (.lit 2)) (.draw .you (.lit 1))
theorem okLoanShark : Ability.check [] loanShark = [] := by decide

def forceOfDespair : Instruction :=
  destroy (allOf (.and [creature, happenedTo .entry .thisTurn]))
theorem okForceOfDespair : Instruction.check [] forceOfDespair = [] := by decide
def cradleToGrave : Instruction :=
  destroy (target (.and [creature, .not (.colorIs .black), happenedTo .entry .thisTurn]))
theorem okCradleToGrave : Instruction.check [] cradleToGrave = [] := by decide

def aragornKingOfGondor : Ability :=
  when (.enters thisCreature none) (.gainsDesignation .you "the monarch" .instructed none)
theorem okAragornKingOfGondor : Ability.check [] aragornKingOfGondor = [] := by decide
def firmamentSage : Ability := whenever (.gameBecomes "night") (.draw .you (.lit 1))
theorem okFirmamentSage : Ability.check [] firmamentSage = [] := by decide
def deeprootWarrior : Ability :=
  whenever (becomesBlocked thisCreature none)
    (gets it (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn))
theorem okDeeprootWarrior : Ability.check [] deeprootWarrior = [] := by decide
def borderlandMarauder : Ability :=
  whenever (attacks thisCreature) (gets it (.up (.lit 2)) (.up (.lit 0)) (some untilEndOfTurn))
theorem okBorderlandMarauder : Ability.check [] borderlandMarauder = [] := by decide
def lichsMasteryLoss : Ability :=
  when (leavesBattlefield thisEnchantment) (.concludes .loseGame .you)
theorem okLichsMasteryLoss : Ability.check [] lichsMasteryLoss = [] := by decide
def phageTheUntouchable : Ability :=
  whenever (dealsCombatDamage thisCreature (a .anyPlayer)) (.concludes .loseGame (that .player))
theorem okPhageTheUntouchable : Ability.check [] phageTheUntouchable = [] := by decide
def elderscaleWurm : Ability :=
  triggeredIf (.enters thisCreature none) (.compareAmt (lifeTotalOf .you) .less (.lit 7))
    (lifeBecomes .you (.lit 7))
theorem okElderscaleWurm : Ability.check [] elderscaleWurm = [] := by decide
/-- Krang, Master Mind -/
def krang : Ability :=
  triggeredIf (.enters thisCreature none)
    (.compareAmt (countOf (.inZone (handOf .you))) .less (.lit 4)) (.draw .you .theDifference)
theorem okKrang : Ability.check [] krang = [] := by decide

def carrionGrub : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Carrion Grub", cost := some [generic 3, pip .black], types := [.creature],
      subtypes := [creatureType "Insect"],
      text :=
        [ .static (.andAlso none
            [ .modify thisCreature .power (.up (.letter .x)),
              .modify thisCreature .toughness (.up (.lit 0)),
              .definesLetter .x
                (aggregate .max (.stat .power) (.and [creature, .inZone (graveyardOf .you)])) ]),
          when (.enters thisCreature none) (mills .you (.lit 4) .you) ],
      power := stat 0, toughness := stat 5 } }

/-- Once Upon a Time -/
def onceUponATimeFirstCast : Predicate := nthCastBy (.nth 1) .you (.within .thisGame)
theorem okOnceUponATimeFirstCast : Predicate.check .object [] onceUponATimeFirstCast = [] := by
  decide

def youngPyromancer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Young Pyromancer", cost := some [generic 1, pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Shaman"],
      text :=
        [ whenever (.casts .you (a (.and [instantOrSorcery, spell])) none)
            (.create .you (.lit 1)
              (.written (creatureToken 1 1 [.red] [creatureType "Elemental"])) []) ],
      power := stat 2, toughness := stat 1 } }

def archivistOfGondor : Ability :=
  triggeredIf (dealsCombatDamage yourCommander (a .anyPlayer)) (thereIsNo "the monarch")
    (.gainsDesignation .you "the monarch" .instructed none)
theorem okArchivistOfGondor : Ability.check [] archivistOfGondor = [] := by decide

def hissingMiasma : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hissing Miasma", cost := some [generic 1, pip .black, pip .black],
      types := [.enchantment],
      text :=
        [ whenever (attacksPlayer (a creature) .you) (losesLife (controllerOf it) (.lit 1)) ] } }

def orimsPrayer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Orim's Prayer", cost := some [generic 1, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ whenever (attacksPlayer (counted (atLeast 1) creature) .you)
            (gainsLife .you (forEach 1 (.and [attacking, creature]))) ] } }

def lesserGargadon : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lesser Gargadon", cost := some [generic 2, pip .red, pip .red],
      types := [.creature], subtypes := [creatureType "Beast"],
      text :=
        [ triggeredOr (attacks thisCreature) [blocks thisCreature none]
            (sacrifice .you (a land)) ],
      power := stat 6, toughness := stat 4 } }

/-- Chub Toad -/
def chubToad : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chub Toad", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [creatureType "Frog"],
      text :=
        [ triggeredOr (blocks thisCreature none) [becomesBlocked thisCreature none]
            (gets it (.up (.lit 2)) (.up (.lit 2)) (some untilEndOfTurn)) ],
      power := stat 1, toughness := stat 1 } }

/-- Giggling Skitterspike -/
def gigglingSkitterspikeTriggers : List GameEvent :=
  [blocks thisCreature none, .becomesTarget thisCreature (a spell)]
theorem okGigglingSkitterspikeTriggers :
    GameEvent.checkAll [] gigglingSkitterspikeTriggers = [] := by decide

/-- Naban, Dean of Iteration -/
def nabanDeanOfIteration : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Naban, Dean of Iteration", cost := some [generic 1, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ .static (.triggersAdditionally
            (.causes
              (.event
                (.enters (a (.and [ .hasSubtype (creatureType "Wizard"),
                                    .hasPossessor .controller .you ])) none))
              (.triggers
                (a (.and [ .abilityHead .anyTriggered,
                           .abilityOf (a (.and [permanent, .hasPossessor .controller .you])) ]))))
            (exactly 1)) ],
      power := stat 2, toughness := stat 1 } }

/-- Bloom Hulk -/
def bloomHulk : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bloom Hulk", cost := some [generic 3, pip .green], types := [.creature],
      subtypes := [creatureType "Plant", creatureType "Elemental"],
      text := [when (.enters thisCreature none) proliferate],
      power := stat 4, toughness := stat 4 } }

/-- Stalwart Successor -/
def stalwartSuccessorHeader : GameEvent := bareCounterEvent .put .many (a creatureYouControl)
theorem okStalwartSuccessorHeader : GameEvent.check [] stalwartSuccessorHeader = [] := by decide

/-- Hollowmurk Siege -/
def hollowmurkSiegeSultai : Ability :=
  triggeredOnlyOnce (bareCounterEvent .put .one (a creatureYouControl)) .oncePerTurn
    (.draw .you (.lit 1))
theorem okHollowmurkSiegeSultai : Ability.check [] hollowmurkSiegeSultai = [] := by decide

/-- Clone -/
def clone : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Clone", cost := some [generic 3, pip .blue], types := [.creature],
      subtypes := [creatureType "Shapeshifter"],
      text := [.static (.entersRider thisCreature (.asCopyOf true (a creature) []))],
      power := stat 0, toughness := stat 0 } }

/-- Quicksilver Gargantuan -/
def quicksilverGargantuan : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Quicksilver Gargantuan", cost := some [generic 5, pip .blue, pip .blue],
      types := [.creature], subtypes := [creatureType "Shapeshifter"],
      text :=
        [ .static (.entersRider thisCreature
            (.asCopyOf true (a creature) [.pt (.lit 7) (.lit 7)])) ],
      power := stat 7, toughness := stat 7 } }

/-- Sculpting Steel -/
def sculptingSteel : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sculpting Steel", cost := some [generic 3], types := [.artifact],
      text := [.static (.entersRider thisArtifact (.asCopyOf true (a artifact) []))] } }

/-- Sakashima's Student -/
def sakashimasStudentCopy : Ability :=
  .static (.entersRider thisCreature
    (.asCopyOf true (a creature) [.types [] [creatureType "Ninja"]]))
theorem okSakashimasStudentCopy : Ability.check [] sakashimasStudentCopy = [] := by decide
/-- Chameleon, Master of Disguise -/
def chameleonCopy : Ability :=
  .static (.entersRider thisCreature
    (.asCopyOf true (a (.and [creature, .hasPossessor .controller .you]))
      [.name "Chameleon, Master of Disguise"]))
theorem okChameleonCopy : Ability.check [] chameleonCopy = [] := by decide

def nefariousLichGain : Ability :=
  .static (.intercepts (.lifeChanges .you .up) [] none (.draw .you .thatMuch) .repeatedly none)
theorem okNefariousLichGain : Ability.check [] nefariousLichGain = [] := by decide

def piousWarrior : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Pious Warrior", cost := some [generic 3, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Rebel", creatureType "Warrior"],
      text := [whenever (.isDealtDamage .combatOnly thisCreature) (gainsLife .you .thatMuch)],
      power := stat 2, toughness := stat 3 } }

def sanguineBond : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sanguine Bond", cost := some [generic 3, pip .black, pip .black],
      types := [.enchantment],
      text := [whenever (.lifeChanges .you .up) (losesLife (target .opponent) .thatMuch)] } }

def exquisiteBlood : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Exquisite Blood", cost := some [generic 4, pip .black], types := [.enchantment],
      text := [whenever (.lifeChanges anOpponent .down) (gainsLife .you .thatMuch)] } }

/-- Agate-Blade Assassin -/
def agateBladeAssassin : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Agate-Blade Assassin", cost := some [generic 1, pip .black], types := [.creature],
      subtypes := [creatureType "Lizard", creatureType "Assassin"],
      text :=
        [ whenever (attacks thisCreature)
            (.sequentially
              [losesLife (.combatPlayer .defending) (.lit 1), gainsLife .you (.lit 1)]) ],
      power := stat 1, toughness := stat 3 } }

/-- Bard, Heir of Girion -/
def bardHeirOfGirion : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bard, Heir of Girion", cost := some [generic 2, pip .white, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Archer"],
      text :=
        [ keyword "Reach", keyword "Vigilance",
          .static (getsPt (allOf (otherCreatureYouControl thisCreature)) (.up (.lit 1)) (.up (.lit 1))),
          whenever (attacks .you) (.draw .you (.lit 1)) ],
      power := stat 4, toughness := stat 4 } }

/-- Fiend Binder -/
def fiendBinder : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fiend Binder", cost := some [generic 3, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Soldier"],
      text :=
        [ whenever (attacks thisCreature)
            (.setStatus .tapped
              (target (.and [creature, .hasPossessor .controller (.combatPlayer .defending)]))) ],
      power := stat 3, toughness := stat 2 } }

/-- Souls of the Faultless -/
def soulsOfTheFaultlessDrain : Ability :=
  whenever (.isDealtDamage .combatOnly thisCreature)
    (losesLife (.combatPlayer .attacking) .thatMuch)
theorem okSoulsOfTheFaultlessDrain : Ability.check [] soulsOfTheFaultlessDrain = [] := by decide

def unstableShapeshifter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Unstable Shapeshifter", cost := some [generic 3, pip .blue], types := [.creature],
      subtypes := [creatureType "Shapeshifter"],
      text :=
        [ whenever (.enters (a (.and [creature, .otherThan .this])) none)
            (.continuously (.becomesCopy thisCreature (that (.type .creature)) [.thisAbility])
              none) ],
      power := stat 0, toughness := stat 1 } }

def prismRing : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Prism Ring", cost := some [generic 1], types := [.artifact],
      text :=
        [ .static (entersChoosing thisArtifact .color),
          whenever (.casts .you (a (.and [spell, ofChosen .color])) none) (gainsLife .you (.lit 1)) ] } }

/-- Aisling Leprechaun -/
def aislingLeprechaun : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aisling Leprechaun", cost := some [pip .green], types := [.creature],
      subtypes := [creatureType "Faerie"],
      text :=
        [ triggeredOr (blocks thisCreature (some (a creature)))
            [becomesBlocked thisCreature (some (a creature))]
            (becomesColor (that (.type .creature)) (.some [.green]) none) ],
      power := stat 1, toughness := stat 1 } }

def avenShrine : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aven Shrine", cost := some [generic 1, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ whenever (.casts (a .anyPlayer) (a spell) none)
            (.sequentially
              [ gainsLife they (.letter .x),
                .define .x (countOf (.and [.inZone graveyard, .named (.sameAs (that .spell))])) ]) ] } }

def chromeReplicator : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chrome Replicator", cost := some [generic 5], types := [.artifact, .creature],
      subtypes := [creatureType "Construct"],
      text :=
        [ triggeredIf (.enters thisCreature none)
            (.exists_ (withTheSameName
              (counted (atLeast 2)
                (.and [permanent, .not land, nontoken, .hasPossessor .controller .you]))))
            (create (.lit 1)
              { characteristics :=
                { types := [.artifact, .creature], subtypes := [creatureType "Construct"],
                  power := stat 4, toughness := stat 4 } }) ],
      power := stat 4, toughness := stat 4 } }

def admiralsOrder : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Admiral's Order", cost := some [generic 1, pip .blue, pip .blue], types := [.instant],
      text :=
        [ .static (onlyWhile (.altCost .this (some (.mana [pip .blue])))
            (happened .attackDeclaration .you .thisTurn)),
          .spell none (.counterSpell (target spell)) ] } }

def fyndhornDruid : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fyndhorn Druid", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [creatureType "Elf", creatureType "Druid"],
      text :=
        [ triggeredIf (.dies thisCreature) (happened .blockedDeclaration it .thisTurn)
            (gainsLife .you (.lit 4)) ],
      power := stat 2, toughness := stat 2 } }

def kamiOfTerribleSecrets : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Kami of Terrible Secrets", cost := some [generic 3, pip .black], types := [.creature],
      subtypes := [creatureType "Spirit"],
      text :=
        [ triggeredIf (.enters thisCreature none)
            (.and [ exists_ (.and [artifact, .hasPossessor .controller .you]),
                    exists_ (.and [enchantment, .hasPossessor .controller .you]) ])
            (.sequentially [.draw .you (.lit 1), gainsLife .you (.lit 1)]) ],
      power := stat 3, toughness := stat 4 } }

def dreadSlaver : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dread Slaver", cost := some [generic 3, pip .black, pip .black],
      types := [.creature], subtypes := [creatureType "Zombie", creatureType "Horror"],
      text :=
        [ whenever
            (.dies (a (.and [creature, happenedToInvolving .damageTaken .thisTurn thisCreature])))
            (.sequentially
              [ putOntoBattlefieldUnderYourControl it,
                becomes (that (.type .creature))
                  { characteristics := { colors := [.black], subtypes := [creatureType "Zombie"] } }
                  none ]) ],
      power := stat 3, toughness := stat 5 } }

/-- Steppe Lynx -/
def steppeLynx : Ability :=
  abilityWord "landfall"
    (whenever (.enters (a (.and [land, .hasPossessor .controller .you])) none)
      (gets thisCreature (.up (.lit 2)) (.up (.lit 2)) (some untilEndOfTurn)))
theorem okSteppeLynx : Ability.check [] steppeLynx = [] := by decide
/-- Owlbear -/
def owlbear : Ability :=
  flavorWord "Keen Senses" (when (.enters thisCreature none) (.draw .you (.lit 1)))
theorem okOwlbear : Ability.check [] owlbear = [] := by decide
/-- Netherese Puzzle-Ward -/
def netheresePuzzleWardIllumination : Ability :=
  whenever youRollHighestNatural (.draw .you (.lit 1))
theorem okNetheresePuzzleWardIllumination :
    Ability.check [] netheresePuzzleWardIllumination = [] := by decide
/-- Farideh, Devil's Chosen -/
def faridehResultRead : Ability :=
  whenever (.rollsDice .you .many none .anyResult)
    (.sequentially
      [ gains thisCreature (keyword "Flying") (some untilEndOfTurn),
        gains thisCreature (keyword "Menace") (some untilEndOfTurn),
        if_ (.anyResultIs .atLeast (.lit 10)) (.draw .you (.lit 1)) ])
theorem okFaridehResultRead : Ability.check [] faridehResultRead = [] := by decide
/-- Jaws of Defeat -/
def jawsOfDefeat : Ability :=
  whenever (.enters (a creatureYouControl) none)
    (losesLife (target .opponent)
      (.arith .differenceBetween (.statOf (.stat .power) (that (.type .creature)))
        (.statOf (.stat .toughness) (that (.type .creature)))))
theorem okJawsOfDefeat : Ability.check [] jawsOfDefeat = [] := by decide
/-- Defiling Daemogoth -/
def defilingDaemogothDrain : Instruction :=
  .sequentially
    [losesLife (each .opponent) (.letter .x), .define .x (eventSum .lifeGain .you .thisTurn)]
theorem okDefilingDaemogothDrain : Instruction.check [] defilingDaemogothDrain = [] := by decide
def skullsporeNexusTrigger : Ability :=
  whenever (.dies (counted (atLeast 1) (.and [nontoken, creatureYouControl])))
    (create (.lit 1)
      (creatureTokenOf (.aggregate .sum (.stat .power) (those .card))
        (.aggregate .sum (.stat .power) (those .card))
        [.green] [creatureType "Fungus", creatureType "Dinosaur"]))
theorem okSkullsporeNexusTrigger : Ability.check [] skullsporeNexusTrigger = [] := by decide
/-- Wavebreak Hippocamp -/
def wavebreakHippocamp : Ability :=
  triggeredOnlyDuring (.nthOccurrence (.nth 1) none (.casts .you (a spell) none))
    (.duringPart .turn (some (each .opponent))) (.draw .you (.lit 1))
theorem okWavebreakHippocamp : Ability.check [] wavebreakHippocamp = [] := by decide
/-- Midnight Clock -/
def midnightClockHeader : GameEvent :=
  .nthOccurrence (.nth 12) none (counterEvent .put (.named "Hour") .one thisArtifact)
theorem okMidnightClockHeader : GameEvent.check [] midnightClockHeader = [] := by decide
/-- Political Triumph -/
def politicalTriumphHeader : GameEvent :=
  .nthOccurrence (.nth 4) none (counterEvent .put (.named "Plan") .one thisEnchantment)
theorem okPoliticalTriumphHeader : GameEvent.check [] politicalTriumphHeader = [] := by decide
/-- Thought Lash -/
def thoughtLashTrigger : Ability :=
  when (.paysCost (some (a .anyPlayer)) .unpaid thisEnchantment "CumulativeUpkeep")
    (exiles (that .player) (each (.inZone (libraryOf they))))
theorem okThoughtLashTrigger : Ability.check [] thoughtLashTrigger = [] := by decide
/-- Heart of Bogardan -/
def heartOfBogardanHeader : GameEvent :=
  .paysCost (some (a .anyPlayer)) .unpaid thisEnchantment "CumulativeUpkeep"
theorem okHeartOfBogardanHeader : GameEvent.check [] heartOfBogardanHeader = [] := by decide
/-- Balduvian Fallen -/
def balduvianFallenHeader : GameEvent := .paysCost none .paid thisCreature "CumulativeUpkeep"
theorem okBalduvianFallenHeader : GameEvent.check [] balduvianFallenHeader = [] := by decide
/-- Stormscape Battlemage -/
def stormscapeBattlemageFirstKicker : Ability :=
  triggeredIf (.enters thisCreature none)
    (costWasPaid (.byNthKeyword (.nth 1) "Kicker") none thisCreature) (gainsLife .you (.lit 3))
theorem okStormscapeBattlemageFirstKicker :
    Ability.check [] stormscapeBattlemageFirstKicker = [] := by decide
/-- All-Seeing Arbiter -/
def allSeeingArbiterHeader : GameEvent :=
  .verbedEvent (some .you) (.action "Discard") (some (a (.inZone hand))) none
theorem okAllSeeingArbiterHeader : GameEvent.check [] allSeeingArbiterHeader = [] := by decide

/-- Liliana's Caress -/
def lilianasCaress : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Liliana's Caress", cost := some [generic 1, pip .black], types := [.enchantment],
      text :=
        [ whenever (.verbedEvent (some anOpponent) (.action "Discard") (some (a (.inZone hand))) none)
            (losesLife they (.lit 2)) ] } }

/-- Scheming Aspirant -/
def schemingAspirant : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Scheming Aspirant", cost := some [generic 1, pip .black], types := [.creature],
      subtypes := [creatureType "Phyrexian", creatureType "Advisor"],
      text :=
        [ whenever (.verbedEvent (some .you) (.action "Proliferate") none none)
            (.sequentially [losesLife (each .opponent) (.lit 2), gainsLife .you (.lit 2)]) ],
      power := stat 1, toughness := stat 3 } }

/-- Reciprocate -/
def reciprocate : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Reciprocate", cost := some [pip .white], types := [.instant],
      text :=
        [ .spell none (exile
            (target (.and [creature, happenedToInvolving .damageDealing .thisTurn .you]))) ] } }

/-- Tahngarth, First Mate -/
def tahngarthHeader : GameEvent := .attacksWith anOpponent none (counted (atLeast 1) creature)
theorem okTahngarthHeader : GameEvent.check [] tahngarthHeader = [] := by decide
/-- Myth Unbound -/
def mythUnboundTrigger : Ability :=
  whenever (putIntoFrom yourCommander commandZone .anywhere) (.draw .you (.lit 1))
theorem okMythUnboundTrigger : Ability.check [] mythUnboundTrigger = [] := by decide

/-- Commander's Insignia -/
def commandersInsignia : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Commander's Insignia", cost := some [generic 2, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ .static (getsPt (allOf creatureYouControl)
            (.up (eventCountFrom .spellCast .you .thisGame yourCommander (.zones [commandZone])))
            (.up (eventCountFrom .spellCast .you .thisGame yourCommander (.zones [commandZone])))) ] } }

/-- Faith's Reward -/
def faithsReward : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Faith's Reward", cost := some [generic 3, pip .white], types := [.instant],
      text :=
        [ .spell none (move
            (allOf (.and [ permanentCard, .inZone (graveyardOf .you),
                           .happenedTo (.mk .placement .thisTurn
                             (some (.fromZones (.zones [battlefield]) none))) ]))
            battlefield) ] } }

/-- Oscorp Industries -/
def oscorpIndustriesReturn : Ability :=
  when (.enters thisLand (some (.zones [graveyard]))) (losesLife .you (.lit 2))
theorem okOscorpIndustriesReturn : Ability.check [] oscorpIndustriesReturn = [] := by decide
def theLostAndTheDamnedEntryArm : GameEvent :=
  .enters (a (.and [land, .hasPossessor .controller .you])) (some (.anywhereBut [handOf .you]))
theorem okTheLostAndTheDamnedEntryArm :
    GameEvent.check [] theLostAndTheDamnedEntryArm = [] := by decide
/-- Forsaken Wastes -/
def forsakenWastesTargeted : Ability :=
  whenever (.becomesTarget thisEnchantment (a spell))
    (losesLife (controllerOf (that .spell)) (.lit 5))
theorem okForsakenWastesTargeted : Ability.check [] forsakenWastesTargeted = [] := by decide

/-- Fblthp, the Lost -/
def fblthp : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fblthp, the Lost", cost := some [generic 1, pip .blue],
      supertypes := [.legendary], types := [.creature], subtypes := [creatureType "Homunculus"],
      text :=
        [ when (.enters .this none)
            (.insteadOf (.draw .you (.lit 1))
              (.if_ (.or
                  [ .happened it (.mk .entry .triggering
                      (some (.fromZones (.zones [yourLibrary]) none))),
                    .matches it (.castFrom yourLibrary) ])
                (.draw .you (.lit 2)) none)),
          when (.becomesTarget .this (a spell)) (shuffleInto .you .this) ],
      power := stat 1, toughness := stat 1 } }

/-- Frost Walker -/
def frostWalker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Frost Walker", cost := some [generic 1, pip .blue], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text :=
        [ when (.becomesTarget thisCreature (a (.or [spell, .abilityHead .anyOnStack])))
            (sacrificeIt .you) ],
      power := stat 4, toughness := stat 1 } }

/-- Loaming Shaman -/
def loamingShaman : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Loaming Shaman", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [creatureType "Centaur", creatureType "Shaman"],
      text :=
        [ when (.enters thisCreature none)
            (shuffleInto (target .anyPlayer)
              (.described (.target anyNumber) (.inZone (graveyardOf they)))) ],
      power := stat 3, toughness := stat 2 } }

/-- Debris Beetle -/
def debrisBeetleTrigger : Ability :=
  when (.enters thisVehicle none)
    (.sequentially [losesLife (each .opponent) (.lit 3), gainsLife .you (.lit 3)])
theorem okDebrisBeetleTrigger : Ability.check [] debrisBeetleTrigger = [] := by decide
/-- Roads Go Ever, Ever On's chapters II and III -/
def roadsGoEverEverOnChapters : Ability :=
  when (.chapterMark [2, 3]) (move (a (.and [.isCard, .exiledWith thisSaga])) hand)
theorem okRoadsGoEverEverOnChapters : Ability.check [] roadsGoEverEverOnChapters = [] := by decide
/-- Wurmwall Sweeper -/
def wurmwallSweeperTrigger : Ability := when (.enters thisSpacecraft none) (surveil .you (.lit 2))
theorem okWurmwallSweeperTrigger : Ability.check [] wurmwallSweeperTrigger = [] := by decide
/-- Case of the Crimson Pulse -/
def caseOfTheCrimsonPulseTrigger : Ability :=
  when (.enters thisCase none)
    (.sequentially [discard .you (a (.inZone hand)), .draw .you (.lit 2)])
theorem okCaseOfTheCrimsonPulseTrigger : Ability.check [] caseOfTheCrimsonPulseTrigger = [] := by
  decide

/-- Ferocious Pup -/
def ferociousPup : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ferocious Pup", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [creatureType "Wolf"],
      text :=
        [ when (.enters thisCreature none)
            (create (.lit 1) (creatureToken 2 2 [.green] [creatureType "Wolf"])) ],
      power := stat 0, toughness := stat 1 } }

/-- Agency Outfitter's search -/
def agencyOutfitterSearch : Instruction :=
  .sequentially
    [ may .you
        (.sequentially
          [ searchZonesOf .you (exactly 1)
              (.or [.named (.printed "Magnifying Glass"), .named (.printed "Thinking Cap")]),
            putOntoBattlefield foundCard ]),
      .if_ (happenedAt (.verbedAct (.action "Search")) .you .thisWay yourLibrary) shuffle none ]
theorem okAgencyOutfitterSearch : Instruction.check [] agencyOutfitterSearch = [] := by decide

/-- Trouble in Pairs' -/
def troubleInPairsArms : Ability :=
  .triggered (.attacksWith anOpponent (some .you) (counted (atLeast 2) creature))
    [ .nthOccurrence (.nth 2) (some .turn) (.draws (a .opponent)),
      .nthOccurrence (.nth 2) (some .turn) (.casts (a .opponent) (a spell) none) ]
    none [] none none none (.draw .you (.lit 1))
theorem okTroubleInPairsArms : Ability.check [] troubleInPairsArms = [] := by decide

/-- Bioplasm -/
def bioplasm : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bioplasm", cost := some [generic 3, pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Ooze"],
      text :=
        [ whenever (attacks thisCreature)
            (.sequentially
              [ exile (topSlice (.lit 1)),
                .if_ bioplasmCardTest
                  (gets thisCreature
                    (.up (.statOf (.stat .power)
                      (theVerbed (.action "Exile") (.typedCard .creature) .attributive .one)))
                    (.up (.statOf (.stat .toughness) (itVerbed (.action "Exile"))))
                    (some .thisTurn))
                  none ]) ],
      power := stat 4, toughness := stat 4 } }

/-- Hostile Investigator -/
def hostileInvestigatorHeader : GameEvent :=
  .verbedEvent (some (counted (atLeast 1) .anyPlayer)) (.action "Discard")
    (some (counted (atLeast 1) .isCard)) none
theorem okHostileInvestigatorHeader : GameEvent.check [] hostileInvestigatorHeader = [] := by decide

/-- Hallowed Moonlight -/
def hallowedMoonlight : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hallowed Moonlight", cost := some [generic 1, pip .white], types := [.instant],
      text :=
        [ .spell none (.sequentially
            [ ifWouldInstead (.enters (a (.and [creature, .not .wasCast])) none) (exile it)
                (some untilEndOfTurn),
              .draw .you (.lit 1) ]) ] } }

/-- Gather Specimens -/
def gatherSpecimens : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gather Specimens", cost := some [generic 3, pip .blue, pip .blue, pip .blue],
      types := [.instant],
      text :=
        [ .spell none (.continuously
            (.entersRider (a (.and [creature, .hasPossessor .controller anOpponent])) (.under .you))
            (some .thisTurn)) ] } }

/-- Don't Blink's replacement, without its written agent -/
def dontBlinkReplacement : Instruction :=
  .continuously
    (.intercepts (.enters (counted (atLeast 1) creature) (some (.zones [exileZone])))
      [.enters (counted (atLeast 1) (.and [creature, .castFrom exileZone])) none] none
      (shuffleInto .you them) .repeatedly none)
    (some untilEndOfTurn)
theorem okDontBlinkReplacement : Instruction.check [] dontBlinkReplacement = [] := by decide

/-- Seasoned Warrenguard -/
def seasonedWarrenguard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Seasoned Warrenguard", cost := some [pip .white], types := [.creature],
      subtypes := [creatureType "Rabbit", creatureType "Warrior"],
      text :=
        [ triggeredWhile (attacks thisCreature)
            (.whileTrue (exists_ (.and [.isToken, .hasPossessor .controller .you])))
            (gets thisCreature (.up (.lit 2)) (.up (.lit 0)) (some untilEndOfTurn)) ],
      power := stat 1, toughness := stat 2 } }

/-- Brazen Blademaster -/
def brazenBlademaster : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Brazen Blademaster", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [creatureType "Orc", creatureType "Pirate"],
      text :=
        [ triggeredWhile (attacks thisCreature)
            (.whileTrue
              (.compareAmt (countOf (.and [artifact, .hasPossessor .controller .you])) .atLeast
                (.lit 2)))
            (gets it (.up (.lit 2)) (.up (.lit 1)) (some untilEndOfTurn)) ],
      power := stat 2, toughness := stat 3 } }

/-- Autarch Mammoth -/
def autarchMammothLine : Ability :=
  triggeredJoined (.enters thisCreature none)
    [ joinedHeadWhile (attacks thisCreature)
        (.whileTrue (.matches thisCreature (.hasDesignation "saddled" none))) ]
    (create (.lit 1) (creatureToken 3 3 [.green] [creatureType "Elephant"]))
theorem okAutarchMammothLine : Ability.check [] autarchMammothLine = [] := by decide
/-- Altar of the Brood -/
def altarOfTheBrood : Ability :=
  whenever (.enters (a (.and [permanent, .hasPossessor .controller .you, .otherThan .this])) none)
    (mills (each .opponent) (.lit 1) they)
theorem okAltarOfTheBrood : Ability.check [] altarOfTheBrood = [] := by decide
/-- Foul Emissary -/
def foulEmissaryLine : Ability :=
  triggeredWhile (.verbedEvent (some .you) (.action "Sacrifice") (some thisCreature) none)
    (.whileDoing (.casts .you (a (.and [spell, .hasKeyword (.the "Emerge")])) none))
    (create (.lit 1) (creatureToken 3 2 [] [creatureType "Eldrazi", creatureType "Horror"]))
theorem okFoulEmissaryLine : Ability.check [] foulEmissaryLine = [] := by decide

/-- Bramble Elemental -/
def brambleElemental : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bramble Elemental", cost := some [generic 3, pip .green, pip .green],
      types := [.creature], subtypes := [creatureType "Elemental"],
      text :=
        [ whenever (.attachment .attached (a (.hasSubtype (enchantmentType "Aura"))) thisCreature)
            (create (.lit 2) (creatureToken 1 1 [.green] [creatureType "Saproling"])) ],
      power := stat 4, toughness := stat 4 } }

/-- Stone Haven Outfitter -/
def stoneHavenOutfitter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stone Haven Outfitter", cost := some [generic 1, pip .white], types := [.creature],
      subtypes := [creatureType "Kor", creatureType "Artificer", creatureType "Ally"],
      text :=
        [ .static (getsPt (allOf (.and [creatureYouControl, .isAttached (some .equipped)]))
            (.up (.lit 1)) (.up (.lit 1))),
          whenever (.dies (a (.and [creatureYouControl, .isAttached (some .equipped)])))
            (.draw .you (.lit 1)) ],
      power := stat 2, toughness := stat 2 } }

/-- Cloud, Ex-SOLDIER -/
def cloudExSoldierAttach : Ability :=
  when (.enters thisCreature none)
    (attachToIt
      (.described (.target (upTo 1))
        (.and [.hasSubtype (artifactType "Equipment"), .hasPossessor .controller .you])))
theorem okCloudExSoldierAttach : Ability.check [] cloudExSoldierAttach = [] := by decide
/-- Akiri, Fearless Voyager -/
def akiriEquippedAttackers : Ability :=
  whenever
    (.attacksWith .you (some (a .anyPlayer))
      (counted (atLeast 1) (.and [creatureYouControl, .isAttached (some .equipped)])))
    (.draw .you (.lit 1))
theorem okAkiriEquippedAttackers : Ability.check [] akiriEquippedAttackers = [] := by decide
/-- Vexing Bauble -/
def vexingBaubleTrigger : Ability :=
  triggeredIf (.casts (a .anyPlayer) (a spell) none) (noManaSpentToCast it)
    (.counterSpell (that .spell))
theorem okVexingBaubleTrigger : Ability.check [] vexingBaubleTrigger = [] := by decide

/-- Void Mirror -/
def voidMirror : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Void Mirror", cost := some [generic 2], types := [.artifact],
      text :=
        [ triggeredIf (.casts (a .anyPlayer) (a spell) none) (noColoredManaSpentToCast it)
            (.counterSpell (that .spell)) ] } }

/-- Blood Sun -/
def bloodSun : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blood Sun", cost := some [generic 2, pip .red], types := [.enchantment],
      text :=
        [ when (.enters thisEnchantment none) (.draw .you (.lit 1)),
          .static (.losesAllAbilities (allOf land) (some .isManaAbility)) ] } }

/-- Promise of Bunrei -/
def promiseOfBunrei : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Promise of Bunrei", cost := some [generic 2, pip .white], types := [.enchantment],
      text :=
        [ when (.dies (a creatureYouControl))
            (.ifDone (sacrifice .you thisEnchantment)
              (some (create (.lit 4) (creatureToken 1 1 [] [creatureType "Spirit"]))) none) ] } }

/-- Grave Peril -/
def gravePeril : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Grave Peril", cost := some [generic 1, pip .black], types := [.enchantment],
      text :=
        [ when (.enters (a (.and [creature, .not (.colorIs .black)])) none)
            (.ifDone (sacrifice .you thisEnchantment) (some (destroy (that (.type .creature))))
              none) ] } }

/-- Blood Reckoning -/
def bloodReckoning : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blood Reckoning", cost := some [generic 3, pip .black], types := [.enchantment],
      text :=
        [ whenever
            (attacksPlayer (a creature)
              (youOr (a (.and [.hasType .planeswalker, .hasPossessor .controller .you]))))
            (losesLife (controllerOf (that (.type .creature))) (.lit 1)) ] } }

/-- Jeskai Ascendancy's first trigger -/
def jeskaiAscendancyPump : Ability :=
  whenever (.casts .you (a (.and [spell, .not creature])) none)
    (.sequentially
      [ gets (bare creatureYouControl) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn),
        untap (those (.type .creature)) ])
theorem okJeskaiAscendancyPump : Ability.check [] jeskaiAscendancyPump = [] := by decide
def herosDemise : Instruction := destroy (target (.and [creature, .hasSupertype .legendary]))
theorem okHerosDemise : Instruction.check [] herosDemise = [] := by decide

def repayInKind : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Repay in Kind", cost := some [generic 5, pip .black, pip .black], types := [.sorcery],
      text :=
        [ .spell none
            (lifeBecomes (each .anyPlayer) (aggregate .min (.playerStat .lifeTotal) .anyPlayer)) ] } }

/-- Squelch -/
def squelch : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Squelch", cost := some [generic 1, pip .blue], types := [.instant],
      text :=
        [ .spell none (.sequentially
            [.counterSpell (target (.abilityHead .anyActivated)), .draw .you (.lit 1)]) ] } }

theorem bioplasmTwoCandidates : countOnes .object bioplasmAfterExile = 2 := by decide

end Semantics.Cards

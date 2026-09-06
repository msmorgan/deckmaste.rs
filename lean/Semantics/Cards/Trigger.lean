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
  delayWithin (.dies (target creature)) .thisTurn (move (that .card) battlefield)
theorem okGracefulReprieve : Instruction.check [] gracefulReprieve = [] := by decide

def cloudkinSeer : Ability := when (.enters thisCreature none) (.draw (.lit 1) (agent := .you))
theorem okCloudkinSeer : Ability.check [] cloudkinSeer = [] := by decide
def promiseOfTomorrow : Ability := whenever (.dies (a creatureYouControl)) (exile it)
theorem okPromiseOfTomorrow : Ability.check [] promiseOfTomorrow = [] := by decide
def libraryLarcenist : Ability := whenever (attacks thisCreature) (.draw (.lit 1) (agent := .you))
theorem okLibraryLarcenist : Ability.check [] libraryLarcenist = [] := by decide
def jhessianThief : Ability :=
  whenever (dealsCombatDamage thisCreature (a .anyPlayer)) (.draw (.lit 1) (agent := .you))
theorem okJhessianThief : Ability.check [] jhessianThief = [] := by decide
def scholarOfStars : Ability :=
  triggeredIf (.enters thisCreature none)
    (exists_ (.and [artifact, .hasPossessor .controller .you])) (.draw (.lit 1) (agent := .you))
theorem okScholarOfStars : Ability.check [] scholarOfStars = [] := by decide

def miserysShadow : Ability :=
  .static (.replacement (.dies (a (.and [creature, .hasPossessor .controller (a .opponent)]))) []
    none (exile it) .repeatedly none)
theorem okMiserysShadow : Ability.check [] miserysShadow = [] := by decide

def beastWhisperer : Ability :=
  whenever (.casts .you (a (.and [creature, spell])) none) (.draw (.lit 1) (agent := .you))
theorem okBeastWhisperer : Ability.check [] beastWhisperer = [] := by decide
def mesmericOrb : Ability :=
  whenever (.statusEvent (a permanent) .untapped)
    (mill (.lit 1) they (agent := (controllerOf (that .permanent))))
theorem okMesmericOrb : Ability.check [] mesmericOrb = [] := by decide
def secretPlans : Ability :=
  whenever (.statusEvent (a (.and [permanent, .hasPossessor .controller .you])) .faceUp)
    (.draw (.lit 1) (agent := .you))
theorem okSecretPlans : Ability.check [] secretPlans = [] := by decide

/-- Teferi's Imp -/
def teferisImpPhasesOut : Ability :=
  whenever (.statusEvent thisCreature .phasedOut) (discard (a (.inZone hand)) (agent := .you))
theorem okTeferisImpPhasesOut : Ability.check [] teferisImpPhasesOut = [] := by decide
/-- Teferi's Imp -/
def teferisImpPhasesIn : Ability :=
  whenever (.statusEvent thisCreature .phasedIn) (.draw (.lit 1) (agent := .you))
theorem okTeferisImpPhasesIn : Ability.check [] teferisImpPhasesIn = [] := by decide

/-- Oubliette -/
def oubliette : Ability :=
  when (.enters thisEnchantment none)
    (phaseOutUntil (target creature) (leavesBattlefield thisEnchantment))
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
    (get thisCreature (.up (.lit 2)) (.up (.lit 0)) (some untilEndOfTurn))
theorem okNetcasterSpider : Ability.check [] netcasterSpider = [] := by decide
def viashinoWeaponsmith : Ability :=
  whenever (becomesBlocked thisCreature (some (a creature)))
    (get thisCreature (.up (.lit 2)) (.up (.lit 2)) (some untilEndOfTurn))
theorem okViashinoWeaponsmith : Ability.check [] viashinoWeaponsmith = [] := by decide
def somberwaldAlpha : Ability :=
  whenever (becomesBlocked (a creatureYouControl) none)
    (get it (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn))
theorem okSomberwaldAlpha : Ability.check [] somberwaldAlpha = [] := by decide
def vertigoSpawn : Ability :=
  whenever (blocks thisCreature (some (a creature)))
    (.sequence
      [ .setStatus .tapped (that (.type .creature)),
        .skipUntap (that (.type .creature)) (.lit 1) ])
theorem okVertigoSpawn : Ability.check [] vertigoSpawn = [] := by decide
def orneryDilophosaur : Ability :=
  triggeredIf (attacks thisCreature)
    (exists_ (.and [ creature, .hasPossessor .controller .you,
                     .compare [.stat .power] .atLeast (.lit 4) ]))
    (get thisCreature (.up (.lit 2)) (.up (.lit 2)) (some untilEndOfTurn))
theorem okOrneryDilophosaur : Ability.check [] orneryDilophosaur = [] := by decide
def incisorGlider : Ability :=
  triggeredIf (attacks thisCreature)
    (.compareAmt (countersOn (.named "Poison") (a .opponent)) .atLeast (.lit 3))
    (get (allOf creatureYouControl) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn))
theorem okIncisorGlider : Ability.check [] incisorGlider = [] := by decide
def stormFleetSpy : Ability :=
  triggeredIf (.enters thisCreature none) (happened .attackDeclaration .you .thisTurn)
    (.draw (.lit 1) (agent := .you))
theorem okStormFleetSpy : Ability.check [] stormFleetSpy = [] := by decide
def loanShark : Ability :=
  triggeredIf (.enters thisCreature none)
    (.compareAmt (eventCount .spellCast .you .thisTurn) .atLeast (.lit 2)) (.draw (.lit 1) (agent :=
        .you))
theorem okLoanShark : Ability.check [] loanShark = [] := by decide

def forceOfDespair : Instruction :=
  destroy (allOf (.and [creature, happenedTo .entry .thisTurn]))
theorem okForceOfDespair : Instruction.check [] forceOfDespair = [] := by decide
def cradleToGrave : Instruction :=
  destroy (target (.and [creature, .not (.colorIs .black), happenedTo .entry .thisTurn]))
theorem okCradleToGrave : Instruction.check [] cradleToGrave = [] := by decide

def aragornKingOfGondor : Ability :=
  when (.enters thisCreature none) (.gainDesignation .you "the monarch" .instructed none)
theorem okAragornKingOfGondor : Ability.check [] aragornKingOfGondor = [] := by decide
def firmamentSage : Ability := whenever (.gameBecomes "night") (.draw (.lit 1) (agent := .you))
theorem okFirmamentSage : Ability.check [] firmamentSage = [] := by decide
def deeprootWarrior : Ability :=
  whenever (becomesBlocked thisCreature none)
    (get it (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn))
theorem okDeeprootWarrior : Ability.check [] deeprootWarrior = [] := by decide
def borderlandMarauder : Ability :=
  whenever (attacks thisCreature) (get it (.up (.lit 2)) (.up (.lit 0)) (some untilEndOfTurn))
theorem okBorderlandMarauder : Ability.check [] borderlandMarauder = [] := by decide
def lichsMasteryLoss : Ability :=
  when (leavesBattlefield thisEnchantment) (.conclude .loseGame (agent := .you))
theorem okLichsMasteryLoss : Ability.check [] lichsMasteryLoss = [] := by decide
def phageTheUntouchable : Ability :=
  whenever (dealsCombatDamage thisCreature (a .anyPlayer)) (.conclude .loseGame (agent := (that
      .player)))
theorem okPhageTheUntouchable : Ability.check [] phageTheUntouchable = [] := by decide
def elderscaleWurm : Ability :=
  triggeredIf (.enters thisCreature none) (.compareAmt (lifeTotalOf .you) .less (.lit 7))
    (setLife (.lit 7) (agent := .you))
theorem okElderscaleWurm : Ability.check [] elderscaleWurm = [] := by decide
/-- Krang, Master Mind -/
def krang : Ability :=
  triggeredIf (.enters thisCreature none)
    (.compareAmt (countOf (.inZone (handOf .you))) .less (.lit 4)) (.draw .theDifference (agent :=
        .you))
theorem okKrang : Ability.check [] krang = [] := by decide

def carrionGrub : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Carrion Grub", cost := some [generic 3, pip .black], types := [.creature],
      subtypes := [creatureType "Insect"],
      text :=
        [ .static (.conjunction none
            [ .modification thisCreature .power (.up (.letter .x)),
              .modification thisCreature .toughness (.up (.lit 0)),
              .letterDefinition .x
                (aggregate .max (.stat .power) (.and [creature, .inZone (graveyardOf .you)])) ]),
          when (.enters thisCreature none) (mill (.lit 4) .you (agent := .you)) ],
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
            (.create (.lit 1)
              (.written (creatureToken 1 1 [.red] [creatureType "Elemental"])) [] (agent := .you))
                  ],
      power := stat 2, toughness := stat 1 } }

def archivistOfGondor : Ability :=
  triggeredIf (dealsCombatDamage yourCommander (a .anyPlayer)) (thereIsNo "the monarch")
    (.gainDesignation .you "the monarch" .instructed none)
theorem okArchivistOfGondor : Ability.check [] archivistOfGondor = [] := by decide

def hissingMiasma : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hissing Miasma", cost := some [generic 1, pip .black, pip .black],
      types := [.enchantment],
      text :=
        [ whenever (attacksPlayer (a creature) .you) (loseLife (.lit 1) (agent := (controllerOf
            it))) ] } }

def orimsPrayer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Orim's Prayer", cost := some [generic 1, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ whenever (attacksPlayer (counted (atLeast 1) creature) .you)
            (gainLife (forEach 1 (.and [attacking, creature])) (agent := .you)) ] } }

def lesserGargadon : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lesser Gargadon", cost := some [generic 2, pip .red, pip .red],
      types := [.creature], subtypes := [creatureType "Beast"],
      text :=
        [ triggeredOr (attacks thisCreature) [blocks thisCreature none]
            (sacrifice (a land) (agent := .you)) ],
      power := stat 6, toughness := stat 4 } }

/-- Chub Toad -/
def chubToad : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chub Toad", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [creatureType "Frog"],
      text :=
        [ triggeredOr (blocks thisCreature none) [becomesBlocked thisCreature none]
            (get it (.up (.lit 2)) (.up (.lit 2)) (some untilEndOfTurn)) ],
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
        [ .static (.additionalTriggers
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
    (.draw (.lit 1) (agent := .you))
theorem okHollowmurkSiegeSultai : Ability.check [] hollowmurkSiegeSultai = [] := by decide

/-- Clone -/
def clone : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Clone", cost := some [generic 3, pip .blue], types := [.creature],
      subtypes := [creatureType "Shapeshifter"],
      text := [.static (.entryRider thisCreature (.asCopyOf true (a creature) []))],
      power := stat 0, toughness := stat 0 } }

/-- Quicksilver Gargantuan -/
def quicksilverGargantuan : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Quicksilver Gargantuan", cost := some [generic 5, pip .blue, pip .blue],
      types := [.creature], subtypes := [creatureType "Shapeshifter"],
      text :=
        [ .static (.entryRider thisCreature
            (.asCopyOf true (a creature) [.pt (.lit 7) (.lit 7)])) ],
      power := stat 7, toughness := stat 7 } }

/-- Sculpting Steel -/
def sculptingSteel : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sculpting Steel", cost := some [generic 3], types := [.artifact],
      text := [.static (.entryRider thisArtifact (.asCopyOf true (a artifact) []))] } }

/-- Sakashima's Student -/
def sakashimasStudentCopy : Ability :=
  .static (.entryRider thisCreature
    (.asCopyOf true (a creature) [.types [] [creatureType "Ninja"]]))
theorem okSakashimasStudentCopy : Ability.check [] sakashimasStudentCopy = [] := by decide
/-- Chameleon, Master of Disguise -/
def chameleonCopy : Ability :=
  .static (.entryRider thisCreature
    (.asCopyOf true (a (.and [creature, .hasPossessor .controller .you]))
      [.name "Chameleon, Master of Disguise"]))
theorem okChameleonCopy : Ability.check [] chameleonCopy = [] := by decide

def nefariousLichGain : Ability :=
  .static (.replacement (.lifeChanges .you .up) [] none (.draw .thatMuch (agent := .you))
      .repeatedly none)
theorem okNefariousLichGain : Ability.check [] nefariousLichGain = [] := by decide

def piousWarrior : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Pious Warrior", cost := some [generic 3, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Rebel", creatureType "Warrior"],
      text := [whenever (.isDealtDamage .combatOnly thisCreature) (gainLife .thatMuch (agent :=
          .you))],
      power := stat 2, toughness := stat 3 } }

def sanguineBond : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sanguine Bond", cost := some [generic 3, pip .black, pip .black],
      types := [.enchantment],
      text := [whenever (.lifeChanges .you .up) (loseLife .thatMuch (agent := (target .opponent)))]
          } }

def exquisiteBlood : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Exquisite Blood", cost := some [generic 4, pip .black], types := [.enchantment],
      text := [whenever (.lifeChanges anOpponent .down) (gainLife .thatMuch (agent := .you))] } }

/-- Agate-Blade Assassin -/
def agateBladeAssassin : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Agate-Blade Assassin", cost := some [generic 1, pip .black], types := [.creature],
      subtypes := [creatureType "Lizard", creatureType "Assassin"],
      text :=
        [ whenever (attacks thisCreature)
            (.sequence
              [loseLife (.lit 1) (agent := (.combatPlayer .defending)), gainLife (.lit 1) (agent :=
                  .you)]) ],
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
          whenever (attacks .you) (.draw (.lit 1) (agent := .you)) ],
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
    (loseLife .thatMuch (agent := (.combatPlayer .attacking)))
theorem okSoulsOfTheFaultlessDrain : Ability.check [] soulsOfTheFaultlessDrain = [] := by decide

def unstableShapeshifter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Unstable Shapeshifter", cost := some [generic 3, pip .blue], types := [.creature],
      subtypes := [creatureType "Shapeshifter"],
      text :=
        [ whenever (.enters (a (.and [creature, .otherThan .this])) none)
            (.establish (.copyChange thisCreature (that (.type .creature)) [.thisAbility])
              none) ],
      power := stat 0, toughness := stat 1 } }

def prismRing : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Prism Ring", cost := some [generic 1], types := [.artifact],
      text :=
        [ .static (entersChoosing thisArtifact .color),
          whenever (.casts .you (a (.and [spell, ofChosen .color])) none) (gainLife (.lit 1) (agent
              := .you)) ] } }

/-- Aisling Leprechaun -/
def aislingLeprechaun : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aisling Leprechaun", cost := some [pip .green], types := [.creature],
      subtypes := [creatureType "Faerie"],
      text :=
        [ triggeredOr (blocks thisCreature (some (a creature)))
            [becomesBlocked thisCreature (some (a creature))]
            (becomeColor (that (.type .creature)) (.some [.green]) none) ],
      power := stat 1, toughness := stat 1 } }

def avenShrine : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aven Shrine", cost := some [generic 1, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ whenever (.casts (a .anyPlayer) (a spell) none)
            (.sequence
              [ gainLife (.letter .x) (agent := they),
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
            (gainLife (.lit 4) (agent := .you)) ],
      power := stat 2, toughness := stat 2 } }

def kamiOfTerribleSecrets : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Kami of Terrible Secrets", cost := some [generic 3, pip .black], types := [.creature],
      subtypes := [creatureType "Spirit"],
      text :=
        [ triggeredIf (.enters thisCreature none)
            (.and [ exists_ (.and [artifact, .hasPossessor .controller .you]),
                    exists_ (.and [enchantment, .hasPossessor .controller .you]) ])
            (.sequence [.draw (.lit 1) (agent := .you), gainLife (.lit 1) (agent := .you)]) ],
      power := stat 3, toughness := stat 4 } }

def dreadSlaver : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dread Slaver", cost := some [generic 3, pip .black, pip .black],
      types := [.creature], subtypes := [creatureType "Zombie", creatureType "Horror"],
      text :=
        [ whenever
            (.dies (a (.and [creature, happenedToInvolving .damageTaken .thisTurn thisCreature])))
            (.sequence
              [ putOntoBattlefieldUnderYourControl it,
                become (that (.type .creature))
                  { characteristics := { colors := [.black], subtypes := [creatureType "Zombie"] } }
                  none ]) ],
      power := stat 3, toughness := stat 5 } }

/-- Steppe Lynx -/
def steppeLynx : Ability :=
  abilityWord "landfall"
    (whenever (.enters (a (.and [land, .hasPossessor .controller .you])) none)
      (get thisCreature (.up (.lit 2)) (.up (.lit 2)) (some untilEndOfTurn)))
theorem okSteppeLynx : Ability.check [] steppeLynx = [] := by decide
/-- Owlbear -/
def owlbear : Ability :=
  flavorWord "Keen Senses" (when (.enters thisCreature none) (.draw (.lit 1) (agent := .you)))
theorem okOwlbear : Ability.check [] owlbear = [] := by decide
/-- Netherese Puzzle-Ward -/
def netheresePuzzleWardIllumination : Ability :=
  whenever youRollHighestNatural (.draw (.lit 1) (agent := .you))
theorem okNetheresePuzzleWardIllumination :
    Ability.check [] netheresePuzzleWardIllumination = [] := by decide
/-- Farideh, Devil's Chosen -/
def faridehResultRead : Ability :=
  whenever (.rollsDice .you .many none .anyResult)
    (.sequence
      [ gain thisCreature (keyword "Flying") (some untilEndOfTurn),
        gain thisCreature (keyword "Menace") (some untilEndOfTurn),
        doIf (.anyResultIs .atLeast (.lit 10)) (.draw (.lit 1) (agent := .you)) ])
theorem okFaridehResultRead : Ability.check [] faridehResultRead = [] := by decide
/-- Jaws of Defeat -/
def jawsOfDefeat : Ability :=
  whenever (.enters (a creatureYouControl) none)
    (loseLife
      (.arith .differenceBetween (.statOf (.stat .power) (that (.type .creature)))
        (.statOf (.stat .toughness) (that (.type .creature)))) (agent := (target .opponent)))
theorem okJawsOfDefeat : Ability.check [] jawsOfDefeat = [] := by decide
/-- Defiling Daemogoth -/
def defilingDaemogothDrain : Instruction :=
  .sequence
    [loseLife (.letter .x) (agent := (each .opponent)), .define .x (eventSum .lifeGain .you
        .thisTurn)]
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
    (.duringPart .turn (some (each .opponent))) (.draw (.lit 1) (agent := .you))
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
    (exile (each (.inZone (libraryOf they))) (agent := some (that .player)))
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
    (costWasPaid (.byNthKeyword (.nth 1) "Kicker") none thisCreature) (gainLife (.lit 3) (agent :=
        .you))
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
            (loseLife (.lit 2) (agent := they)) ] } }

/-- Scheming Aspirant -/
def schemingAspirant : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Scheming Aspirant", cost := some [generic 1, pip .black], types := [.creature],
      subtypes := [creatureType "Phyrexian", creatureType "Advisor"],
      text :=
        [ whenever (.verbedEvent (some .you) (.action "Proliferate") none none)
            (.sequence [loseLife (.lit 2) (agent := (each .opponent)), gainLife (.lit 2) (agent :=
                .you)]) ],
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
  whenever (putIntoFrom yourCommander commandZone .anywhere) (.draw (.lit 1) (agent := .you))
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
  when (.enters thisLand (some (.zones [graveyard]))) (loseLife (.lit 2) (agent := .you))
theorem okOscorpIndustriesReturn : Ability.check [] oscorpIndustriesReturn = [] := by decide
def theLostAndTheDamnedEntryArm : GameEvent :=
  .enters (a (.and [land, .hasPossessor .controller .you])) (some (.anywhereBut [handOf .you]))
theorem okTheLostAndTheDamnedEntryArm :
    GameEvent.check [] theLostAndTheDamnedEntryArm = [] := by decide
/-- Forsaken Wastes -/
def forsakenWastesTargeted : Ability :=
  whenever (.becomesTarget thisEnchantment (a spell))
    (loseLife (.lit 5) (agent := (controllerOf (that .spell))))
theorem okForsakenWastesTargeted : Ability.check [] forsakenWastesTargeted = [] := by decide

/-- Fblthp, the Lost -/
def fblthp : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fblthp, the Lost", cost := some [generic 1, pip .blue],
      supertypes := [.legendary], types := [.creature], subtypes := [creatureType "Homunculus"],
      text :=
        [ when (.enters .this none)
            (.replace (.draw (.lit 1) (agent := .you))
              (.doIf (.or
                  [ .happened it (.mk .entry .triggering
                      (some (.fromZones (.zones [yourLibrary]) none))),
                    .matches it (.castFrom yourLibrary) ])
                (.draw (.lit 2) (agent := .you)) none)),
          when (.becomesTarget .this (a spell)) (shuffleInto .this (agent := .you)) ],
      power := stat 1, toughness := stat 1 } }

/-- Frost Walker -/
def frostWalker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Frost Walker", cost := some [generic 1, pip .blue], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text :=
        [ when (.becomesTarget thisCreature (a (.or [spell, .abilityHead .anyOnStack])))
            (sacrificeIt (agent := .you)) ],
      power := stat 4, toughness := stat 1 } }

/-- Loaming Shaman -/
def loamingShaman : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Loaming Shaman", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [creatureType "Centaur", creatureType "Shaman"],
      text :=
        [ when (.enters thisCreature none)
            (shuffleInto
              (.described (.target anyNumber) (.inZone (graveyardOf they))) (agent := (target
                  .anyPlayer))) ],
      power := stat 3, toughness := stat 2 } }

/-- Debris Beetle -/
def debrisBeetleTrigger : Ability :=
  when (.enters thisVehicle none)
    (.sequence [loseLife (.lit 3) (agent := (each .opponent)), gainLife (.lit 3) (agent := .you)])
theorem okDebrisBeetleTrigger : Ability.check [] debrisBeetleTrigger = [] := by decide
/-- Roads Go Ever, Ever On's chapters II and III -/
def roadsGoEverEverOnChapters : Ability :=
  when (.chapterMark [2, 3]) (move (a (.and [.isCard, .exiledWith thisSaga])) hand)
theorem okRoadsGoEverEverOnChapters : Ability.check [] roadsGoEverEverOnChapters = [] := by decide
/-- Wurmwall Sweeper -/
def wurmwallSweeperTrigger : Ability := when (.enters thisSpacecraft none) (surveil (.lit 2) (agent
    := .you))
theorem okWurmwallSweeperTrigger : Ability.check [] wurmwallSweeperTrigger = [] := by decide
/-- Case of the Crimson Pulse -/
def caseOfTheCrimsonPulseTrigger : Ability :=
  when (.enters thisCase none)
    (.sequence [discard (a (.inZone hand)) (agent := .you), .draw (.lit 2) (agent := .you)])
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
  .sequence
    [ offer
        (.sequence
          [ searchZonesOf .you (exactly 1)
              (.or [.named (.printed "Magnifying Glass"), .named (.printed "Thinking Cap")]),
            putOntoBattlefield foundCard ]) (agent := .you),
      .doIf (happenedAt (.verbedAct (.action "Search")) .you .thisWay yourLibrary) shuffle none ]
theorem okAgencyOutfitterSearch : Instruction.check [] agencyOutfitterSearch = [] := by decide

/-- Trouble in Pairs' -/
def troubleInPairsArms : Ability :=
  .triggered (.attacksWith anOpponent (some .you) (counted (atLeast 2) creature))
    [ .nthOccurrence (.nth 2) (some .turn) (.draws (a .opponent)),
      .nthOccurrence (.nth 2) (some .turn) (.casts (a .opponent) (a spell) none) ]
    none [] none none none (.draw (.lit 1) (agent := .you))
theorem okTroubleInPairsArms : Ability.check [] troubleInPairsArms = [] := by decide

/-- Bioplasm -/
def bioplasm : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bioplasm", cost := some [generic 3, pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Ooze"],
      text :=
        [ whenever (attacks thisCreature)
            (.sequence
              [ exile (topSlice (.lit 1)),
                .doIf bioplasmCardTest
                  (get thisCreature
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
        [ .spell none (.sequence
            [ replaceEvent (.enters (a (.and [creature, .not .wasCast])) none) (exile it)
                (some untilEndOfTurn),
              .draw (.lit 1) (agent := .you) ]) ] } }

/-- Gather Specimens -/
def gatherSpecimens : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gather Specimens", cost := some [generic 3, pip .blue, pip .blue, pip .blue],
      types := [.instant],
      text :=
        [ .spell none (.establish
            (.entryRider (a (.and [creature, .hasPossessor .controller anOpponent])) (.under .you))
            (some .thisTurn)) ] } }

/-- Don't Blink's replacement, without its written agent -/
def dontBlinkReplacement : Instruction :=
  .establish
    (.replacement (.enters (counted (atLeast 1) creature) (some (.zones [exileZone])))
      [.enters (counted (atLeast 1) (.and [creature, .castFrom exileZone])) none] none
      (shuffleInto them (agent := .you)) .repeatedly none)
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
            (get thisCreature (.up (.lit 2)) (.up (.lit 0)) (some untilEndOfTurn)) ],
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
            (get it (.up (.lit 2)) (.up (.lit 1)) (some untilEndOfTurn)) ],
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
    (mill (.lit 1) they (agent := (each .opponent)))
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
            (.draw (.lit 1) (agent := .you)) ],
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
    (.draw (.lit 1) (agent := .you))
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
        [ when (.enters thisEnchantment none) (.draw (.lit 1) (agent := .you)),
          .static (.allAbilityLoss (allOf land) (some .isManaAbility)) ] } }

/-- Promise of Bunrei -/
def promiseOfBunrei : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Promise of Bunrei", cost := some [generic 2, pip .white], types := [.enchantment],
      text :=
        [ when (.dies (a creatureYouControl))
            (.doIfDone (sacrifice thisEnchantment (agent := .you))
              (some (create (.lit 4) (creatureToken 1 1 [] [creatureType "Spirit"]))) none) ] } }

/-- Grave Peril -/
def gravePeril : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Grave Peril", cost := some [generic 1, pip .black], types := [.enchantment],
      text :=
        [ when (.enters (a (.and [creature, .not (.colorIs .black)])) none)
            (.doIfDone (sacrifice thisEnchantment (agent := .you)) (some (destroy (that (.type
                .creature))))
              none) ] } }

/-- Blood Reckoning -/
def bloodReckoning : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blood Reckoning", cost := some [generic 3, pip .black], types := [.enchantment],
      text :=
        [ whenever
            (attacksPlayer (a creature)
              (youOr (a (.and [.hasType .planeswalker, .hasPossessor .controller .you]))))
            (loseLife (.lit 1) (agent := (controllerOf (that (.type .creature))))) ] } }

/-- Jeskai Ascendancy's first trigger -/
def jeskaiAscendancyPump : Ability :=
  whenever (.casts .you (a (.and [spell, .not creature])) none)
    (.sequence
      [ get (bare creatureYouControl) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn),
        untap (those (.type .creature)) ])
theorem okJeskaiAscendancyPump : Ability.check [] jeskaiAscendancyPump = [] := by decide
def herosDemise : Instruction := destroy (target (.and [creature, .hasSupertype .legendary]))
theorem okHerosDemise : Instruction.check [] herosDemise = [] := by decide

def repayInKind : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Repay in Kind", cost := some [generic 5, pip .black, pip .black], types := [.sorcery],
      text :=
        [ .spell none
            (setLife (aggregate .min (.playerStat .lifeTotal) .anyPlayer) (agent := (each
                .anyPlayer))) ] } }

/-- Squelch -/
def squelch : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Squelch", cost := some [generic 1, pip .blue], types := [.instant],
      text :=
        [ .spell none (.sequence
            [.counterSpell (target (.abilityHead .anyActivated)), .draw (.lit 1) (agent := .you)]) ]
                } }

theorem bioplasmTwoCandidates : countOnes .object bioplasmAfterExile = 2 := by decide

end Semantics.Cards

import Semantics
import Semantics.Macros
import Semantics.Check.Card
import Semantics.Cards.Description

/-!
# Semantics.Cards.Deontic

Port of `idris/src/Experimental/Cards/Deontic.idr`: the printed cards of the Deontic family
(can't, must, may, tolls, and their windows) and the bench items beside them.
-/

open Semantics Semantics.Macros

namespace Semantics.Cards

def infiltrate : Instruction := forbidBeingBlocked (target creature) (some .thisTurn)
theorem okInfiltrate : Instruction.check [] infiltrate = [] := by decide
def changeOfHeart : Instruction := forbidAttack (target creature) (some .thisTurn)
theorem okChangeOfHeart : Instruction.check [] changeOfHeart = [] := by decide
def blindblast : Instruction :=
  .sequence
    [.dealDamage .this (.lit 1) (target creature), forbidBlock (that (.type .creature)) (some
        .thisTurn)]
theorem okBlindblast : Instruction.check [] blindblast = [] := by decide
def blindingFlare : Instruction :=
  forbidBlock (.described (.target anyNumber) creature) (some .thisTurn)
theorem okBlindingFlare : Instruction.check [] blindingFlare = [] := by decide
def cowardKiller : Instruction :=
  .sequence
    [ forbidBlock (target creature) (some .thisTurn),
      become (that (.type .creature)) { characteristics := { subtypes := [creatureType "Coward"] } }
        (some untilEndOfTurn) ]
theorem okCowardKiller : Instruction.check [] cowardKiller = [] := by decide

/-- Auriok Siege Sled -/
def auriokSiegeSledDenial : Ability :=
  activated (.mana [generic 1])
    (.establish (cantDoTo (.core .block) (target (.and [artifact, creature])) thisCreature)
      (some .thisTurn))
theorem okAuriokSiegeSledDenial : Ability.check [] auriokSiegeSledDenial = [] := by decide

/-- Blindblast -/
def blindblastWhole : Instruction :=
  .sequence
    [ .dealDamage .this (.lit 1) (target creature), forbidBlock (that (.type .creature)) (some
        .thisTurn),
      .draw (.lit 1) (agent := .you) ]
theorem okBlindblastWhole : Instruction.check [] blindblastWhole = [] := by decide

def sparkmagesGambit : Instruction :=
  .sequence
    [ .dealDamage .this (.lit 1) (.eachOf (.described (.target (upTo 2)) creature)),
      forbidBlock (those (.type .creature)) (some .thisTurn) ]
theorem okSparkmagesGambit : Instruction.check [] sparkmagesGambit = [] := by decide

/-- Glacial Chasm -/
def glacialChasmCant : Ability :=
  .static (deontic (allOf creatureYouControl) .forbid [.core .attack] .agent .noPatient)
theorem okGlacialChasmCant : Ability.check [] glacialChasmCant = [] := by decide

def desperateCastaways : Ability :=
  .static (onlyUnless (deontic thisCreature .forbid [.core .attack] .agent .noPatient)
    (exists_ (.and [artifact, .hasPossessor .controller .you])))
theorem okDesperateCastaways : Ability.check [] desperateCastaways = [] := by decide

def munghaWurm : Ability := .static (cantMoreThan .you (.action "Untap") 1 land)
theorem okMunghaWurm : Ability.check [] munghaWurm = [] := by decide
def dampingField : Ability :=
  .static (cantMoreThan (.playerGroup .allPlayers) (.action "Untap") 1 artifact)
theorem okDampingField : Ability.check [] dampingField = [] := by decide
def smoke : Ability := .static (cantMoreThan (.playerGroup .allPlayers) (.action "Untap") 1 creature)
theorem okSmoke : Ability.check [] smoke = [] := by decide
def winterOrb : Ability :=
  .static (onlyWhile (cantMoreThan (.playerGroup .allPlayers) (.action "Untap") 1 land)
    (.matches thisArtifact untapped))
theorem okWinterOrb : Ability.check [] winterOrb = [] := by decide
def staticOrb : Ability :=
  .static (onlyWhile (cantMoreThan (.playerGroup .allPlayers) (.action "Untap") 2 permanent)
    (.matches thisArtifact untapped))
theorem okStaticOrb : Ability.check [] staticOrb = [] := by decide

/-- Rule of Law -/
def ruleOfLaw : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rule of Law", cost := some [generic 2, pip .white], types := [.enchantment],
      text := [.static (cantMoreThan (.playerGroup .allPlayers) (.action "Cast") 1 spell)] } }

/-- Spirit of the Labyrinth -/
def spiritOfTheLabyrinth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Spirit of the Labyrinth", cost := some [generic 1, pip .white],
      types := [.enchantment, .creature], subtypes := [creatureType "Spirit"],
      text := [.static (cantMoreThan (.playerGroup .allPlayers) (.core .draw) 1 .isCard)],
      power := stat 3, toughness := stat 1 } }

def winterMoon : Ability :=
  .static (cantMoreThan (.playerGroup .allPlayers) (.action "Untap") 1
    (.and [land, .not (.hasSupertype .basic)]))
theorem okWinterMoon : Ability.check [] winterMoon = [] := by decide

def leitmotifComposer : Ability :=
  activated (.mana [generic 2, pip .blue])
    (.establish
      (deontic (allOf (.and [creature, .named (.printed "Leitmotif Composer")])) .forbid
        [.core .block] .patient .noPatient)
      (some .thisTurn))
theorem okLeitmotifComposer : Ability.check [] leitmotifComposer = [] := by decide

def berserkersOfBloodRidge : Ability :=
  .static (deontic thisCreature .require [.core .attack] .agent .noPatient)
theorem okBerserkersOfBloodRidge : Ability.check [] berserkersOfBloodRidge = [] := by decide

def trumpetingArmodon : Ability :=
  activated (.mana [generic 1, pip .green])
    (.establish
      (deontic (target creature) .require [.core .block] .agent (.counterpart thisCreature))
      (some .thisTurn))
theorem okTrumpetingArmodon : Ability.check [] trumpetingArmodon = [] := by decide

def loathsomeCatoblepas : Ability :=
  activated (.mana [generic 2, pip .green])
    (.establish (deontic thisCreature .require [.core .block] .patient .noPatient)
      (some .thisTurn))
theorem okLoathsomeCatoblepas : Ability.check [] loathsomeCatoblepas = [] := by decide

def hipparion : Ability :=
  .static (deontic thisCreature (.gatedBy (.mana [generic 1])) [.core .block] .agent
    (.counterpart (allOf (.and [creature, .compare [.stat .power] .atLeast (.lit 3)]))))
theorem okHipparion : Ability.check [] hipparion = [] := by decide

def frodoBaggins : Ability :=
  .static (onlyWhile (deontic thisCreature .require [.core .block] .patient .noPatient)
    (.matches thisCreature (.hasDesignation "Ring-bearer" (some .you))))
theorem okFrodoBaggins : Ability.check [] frodoBaggins = [] := by decide

def bloodshedFever : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bloodshed Fever", cost := some [pip .red], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          .static (deontic (.attachHost .enchanted (.type .creature)) .require [.core .attack]
            .agent .noPatient) ] } }

/-- Pacifism -/
def pacifism : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Pacifism", cost := some [generic 1, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          .static (deontic (.attachHost .enchanted (.type .creature)) .forbid
            [.core .attack, .core .block] .agent .noPatient) ] } }

/-- Everybody Lives! -/
def everybodyLivesGateLine : Instruction :=
  .establish
    (deontic (.playerGroup .allPlayers) .forbid [.core .loseGame, .core .winGame] .agent .noPatient)
    (some .thisTurn)
theorem okEverybodyLivesGateLine : Instruction.check [] everybodyLivesGateLine = [] := by decide

/-- Gaea's Revenge -/
def gaeasRevenge : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gaea's Revenge", cost := some [generic 5, pip .green, pip .green],
      types := [.creature], subtypes := [creatureType "Elemental"],
      text :=
        [ .static (objectCant (.action "Counter") .this),
          keyword "Haste",
          .static (cantBeTargetedBy thisCreature
            (allOf (.or [ .and [spell, .not (.colorIs .green)],
                          .abilityOf (allOf (.and [source, .not (.colorIs .green)])) ]))) ],
      power := stat 8, toughness := stat 5 } }

/-- Nowhere to Run -/
def nowhereToRunWardLine : StaticSpec :=
  .conjunction none
    [ canBeTargetedAsThough (allOf creatureYourOpponentsControl)
        (allOf (.or [spell, .abilityHead .anyOnStack]))
        (.not (.hasKeyword (.the "Hexproof"))),
      deontic
        (allOf (.and [.abilityHead (.keyword "Ward"), .abilityOf (those (.type .creature))]))
        .forbid [.core .trigger] .agent .noPatient ]
theorem okNowhereToRunWardLine : StaticSpec.check [] nowhereToRunWardLine = [] := by decide

/-- Mornsong Aria -/
def mornsongAriaLock : StaticSpec :=
  deontic (.playerGroup .allPlayers) .forbid [.core .draw, .core .gainLife] .agent .noPatient
theorem okMornsongAriaLock : StaticSpec.check [] mornsongAriaLock = [] := by decide

def brainwash : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Brainwash", cost := some [pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          .static (deontic (.attachHost .enchanted (.type .creature)) (.gatedBy (.mana [generic 3]))
            [.core .attack] .agent .noPatient) ] } }

def enkiraHostileScavenger : Ability :=
  .static (onlyWhile (deontic thisCreature .require [.core .block] .patient .noPatient)
    (.matches thisCreature (.isAttached (some .equipped))))
theorem okEnkiraHostileScavenger : Ability.check [] enkiraHostileScavenger = [] := by decide

def anglerTurtle : Ability :=
  .static (deontic (allOf creatureYourOpponentsControl) .require [.core .attack] .agent .noPatient)
theorem okAnglerTurtle : Ability.check [] anglerTurtle = [] := by decide

def ensnaringBridge : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ensnaring Bridge", cost := some [generic 3], types := [.artifact],
      text :=
        [ .static (deontic
            (allOf (.and [ creature,
                           .compare [.stat .power] .greater (countOf (.inZone (handOf .you))) ]))
            .forbid [.core .attack] .agent .noPatient) ] } }

def undercoverButler : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Undercover Butler", cost := some [generic 2, hybridPip .blue .black],
      types := [.creature], subtypes := [creatureType "Human", creatureType "Rogue"],
      text :=
        [ whenever
            (attacksPlayer thisCreature
              (the (.and [.anyPlayer, .superlative .max (.playerStat .lifeTotal) .anyPlayer])))
            (.establish (deontic it .forbid [.core .block] .patient .noPatient)
              (some .thisTurn)) ],
      power := stat 2, toughness := stat 3 } }

def aetherTunnel : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aether Tunnel", cost := some [generic 1, pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          .static (.conjunction none
            [ .modification (.attachHost .enchanted (.type .creature)) .power (.up (.lit 1)),
              .modification (itsOther (.attachHost .enchanted (.type .creature)) (.up (.lit 1)))
                .toughness (.up (.lit 0)),
              deontic it .forbid [.core .block] .patient .noPatient ]) ] } }

def excruciator : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Excruciator", cost := some [generic 6, pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Avatar"],
      text :=
        [ .static (.preventionBan .any (.described (.dealtBy thisCreature) .everywhere)
            .noPreventionOnly) ],
      power := stat 7, toughness := stat 7 } }

def flaringPain : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Flaring Pain", cost := some [generic 1, pip .red], types := [.instant],
      text :=
        [ .spell none
            (.establish (.preventionBan .any (.described .unattributed .everywhere)
                .noPreventionOnly)
              (some .thisTurn)),
          keywordCosting "Flashback" (.mana [pip .red]) ] } }

def gideonJura : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gideon Jura", cost := some [generic 3, pip .white, pip .white],
      supertypes := [.legendary], types := [.planeswalker], subtypes := [planeswalkerType "Gideon"],
      text :=
        [ activated (.loyaltySymbol (.up 2))
            (establishThroughout
              (deontic (allOf (.and [creature, .hasPossessor .controller (target .opponent)]))
                .require [.core .attack] .agent (.defendingPlayer thisPlaneswalker))
              (.duringNextTurnOf (that .player))),
          activated (.loyaltySymbol (.down 2)) (destroy (target (.and [creature, tapped]))),
          activated (.loyaltySymbol .zero)
            (.sequence
              [ .establish
                  (.qualityChange thisPlaneswalker .sets
                    (.bundle
                      { characteristics :=
                        { types := [.creature],
                          subtypes := [creatureType "Human", creatureType "Soldier"],
                          power := stat 6, toughness := stat 6 } }
                      (some .planeswalker)))
                  (some untilEndOfTurn),
                preventAll .any (.toRecipient it) (some .thisTurn) ]) ],
      loyalty := stat 6 } }

def pinpointAvalanche : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Pinpoint Avalanche", cost := some [generic 3, pip .red, pip .red],
      types := [.instant],
      text :=
        [ .spell none (.sequence
            [ .dealDamage .this (.lit 4) (target creature),
              .establish (.preventionBan .any .thatDamage .noPreventionOnly) none ]) ] } }

def whippoorwillImmunity : Instruction :=
  .sequence
    [ .establish (objectCant (.action "Regenerate") (target creature)) (some .thisTurn),
      .establish
        (.preventionBan .any
          (.described .unattributed (.toRecipient (that (.type .creature)))) .noRedirectEither)
        (some .thisTurn) ]
theorem okWhippoorwillImmunity : Instruction.check [] whippoorwillImmunity = [] := by decide

def callInAProfessional : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Call In a Professional", cost := some [generic 2, pip .red], types := [.instant],
      text :=
        [ .spell none (.sequence
            [ .establish (playerCant (.core .gainLife) (.playerGroup .allPlayers))
                (some .thisTurn),
              .establish
                (.preventionBan .any (.described .unattributed .everywhere) .noPreventionOnly)
                (some .thisTurn),
              .dealDamage .this (.lit 3) (target anyTarget) ]) ] } }

/-- Council of the Absolute -/
def councilOfTheAbsolute : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Council of the Absolute", cost := some [generic 2, pip .white, pip .blue],
      types := [.creature], subtypes := [creatureType "Human", creatureType "Advisor"],
      text :=
        [ .static (entersChoosingFrom thisCreature .cardName
            (.nameOfCard (.not (.or [creature, land])))),
          .static (cantDoTo (.action "Cast") (.playerGroup .yourOpponents)
            (allOf (.and [spell, .named .chosen]))),
          .static (.costShift (allOf (.and [spell, .named .chosen, castBy .you]))
            (.less (.lit 2) none)) ],
      power := stat 2, toughness := stat 4 } }

/-- Failure // Comply -/
def complyNameLock : Instruction :=
  .sequence
    [ choose (a (quality .cardName)),
      .establish
        (cantDoTo (.action "Cast") (.playerGroup .yourOpponents)
          (allOf (.and [spell, .named .chosen])))
        (some untilYourNextTurn) ]
theorem okComplyNameLock : Instruction.check [] complyNameLock = [] := by decide

/-- Gideon's Intervention -/
def gideonsIntervention : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gideon's Intervention", cost := some [generic 2, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ .static (entersChoosing thisEnchantment .cardName),
          .static (cantDoTo (.action "Cast") (.playerGroup .yourOpponents)
            (allOf (.and [spell, .named .chosen]))),
          .static (.damageRule .any (.dealtBy (allOf (.and [source, .named .chosen])))
            (.toRecipient (youAnd (allOf (.and [permanent, .hasPossessor .controller .you]))))
            (.prevent .all none) .repeatedly) ] } }

/-- Academic Probation -/
def academicProbationNameMode : Instruction :=
  .sequence
    [ choose (a (qualityFrom .cardName (.nameOfCard (.not land)))),
      .establish
        (cantDoTo (.action "Cast") (.playerGroup .yourOpponents)
          (allOf (.and [spell, .named .chosen])))
        (some untilYourNextTurn) ]
theorem okAcademicProbationNameMode : Instruction.check [] academicProbationNameMode = [] := by
  decide

def fatigue : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fatigue", cost := some [generic 1, pip .blue], types := [.sorcery],
      text := [.spell none (.skipPart .drawStep (.lit 1) (agent := (target .anyPlayer)))] } }

def meditate : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Meditate", cost := some [generic 2, pip .blue], types := [.instant],
      text := [.spell none (.sequence [.draw (.lit 4) (agent := .you), .skipPart .turn (.lit 1)
          (agent := .you)])] } }

/-- Blinding Angel {3}{W}{W} — Creature — Angel 2/4. "Flying. Whenever Blinding Angel deals
combat damage to a player, that player skips their next combat phase." -/
def blindingAngel : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blinding Angel", cost := some [generic 3, pip .white, pip .white],
      types := [.creature], subtypes := [creatureType "Angel"],
      text :=
        [ keyword "Flying",
          whenever
            (dealsCombatDamage thisCreature (a .anyPlayer))
            (.skipPart .combat (.lit 1) (agent := (that .player))) ],
      power := stat 2, toughness := stat 4 } }

def eonHub : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Eon Hub", cost := some [generic 5], types := [.artifact],
      text := [.static (.partSkip (.playerGroup .allPlayers) .upkeep)] } }

def stasis : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stasis", cost := some [generic 1, pip .blue], types := [.enchantment],
      text :=
        [ .static (.partSkip (.playerGroup .allPlayers) .untapStep),
          at_ (.beginningOf .the .upkeep (.byPlayer .you))
            (doUnless (sacrifice thisEnchantment (agent := .you)) (.mana [pip .blue]) (agent :=
                .you)) ] } }

def yawgmothsBargain : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Yawgmoth's Bargain", cost := some [generic 4, pip .black, pip .black],
      types := [.enchantment],
      text :=
        [ .static (.partSkip .you .drawStep),
          activated (payLife .you 1) (.draw (.lit 1) (agent := .you)) ] } }

def sandsOfTimeSkip : StaticSpec := .partSkip (each .anyPlayer) .untapStep
theorem okSandsOfTimeSkip : StaticSpec.check [] sandsOfTimeSkip = [] := by decide

/-- Wormfang Manta {5}{U}{U} — Creature — Nightmare Fish Beast 6/1. "Flying. When Wormfang
Manta enters, you skip your next turn. When Wormfang Manta leaves the battlefield, you take an
extra turn after this one." -/
def wormfangManta : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Wormfang Manta", cost := some [generic 5, pip .blue, pip .blue],
      types := [.creature],
      subtypes := [creatureType "Nightmare", creatureType "Fish", creatureType "Beast"],
      text :=
        [ keyword "Flying",
          when (.enters thisCreature none) (.skipPart .turn (.lit 1) (agent := .you)),
          when (leavesBattlefield thisCreature) (.addTurn (.lit 1) (agent := .you)) ],
      power := stat 6, toughness := stat 1 } }

def eaterOfDaysSkip : Instruction := .skipPart .turn (.lit 2) (agent := .you)
theorem okEaterOfDaysSkip : Instruction.check [] eaterOfDaysSkip = [] := by decide

/-- Empty City Ruse -/
def emptyCityRuse : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Empty City Ruse", cost := some [pip .white], types := [.sorcery],
      text :=
        [ .spell none
            (establishThroughout (.partSkip (target .opponent) .combat) (.duringNextTurnOf (that
                .player))) ] } }

/-- False Peace -/
def falsePeace : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "False Peace", cost := some [pip .white], types := [.sorcery],
      text :=
        [ .spell none
            (establishThroughout (.partSkip (target .anyPlayer) .combat) (.duringNextTurnOf (that
                .player))) ] } }

/-- Battlefront Krushok -/
def battlefrontKrushokEvasion : Ability :=
  .static (.deonticRule thisCreature .forbid [.core .block] .patient (some (.moreThan (.lit 1)))
    (.counterpart (allOf creature)) none .noRider)
theorem okBattlefrontKrushokEvasion : Ability.check [] battlefrontKrushokEvasion = [] := by decide

/-- Grand Abolisher -/
def grandAbolisher : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Grand Abolisher", cost := some [pip .white, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Cleric"],
      text :=
        [ .static (.partScope .turn (some .you)
            (.conjunction none
              [ cantDoTo (.action "Cast") (.playerGroup .yourOpponents) (allOf spell),
                cantDoTo (.action "Activate") (.playerGroup .yourOpponents)
                  (allOf (.and [ .abilityHead .anyActivated,
                                 .abilityOf (allOf (.or [artifact, creature, enchantment])) ])) ])) ],
      power := stat 2, toughness := stat 2 } }

/-- Festival -/
def festival : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Festival", cost := some [pip .white], types := [.instant],
      text :=
        [ .spell (some (.duringPart .upkeep (some anOpponent)))
            (forbidAttack (allOf creature) (some .thisTurn)) ] } }

def demotion : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Demotion", cost := some [pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          .static (.conjunction none
            [ deontic (.attachHost .enchanted (.type .creature)) .forbid [.core .block] .agent
                .noPatient,
              objectCant (.action "Activate")
                (allOf (.and [.abilityHead .anyActivated, .abilityOf it])) ]) ] } }

def terror : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Terror", cost := some [generic 1, pip .black], types := [.instant],
      text :=
        [ .spell none (.doAndForbid
            (destroy (target (.and [creature, .not artifact, .not (.colorIs .black)])))
            (.action "Regenerate") (itVerbed (.action "Destroy"))) ] } }

def snuffOut : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Snuff Out", cost := some [generic 3, pip .black], types := [.instant],
      text :=
        [ .static (onlyWhile (.altCost .this (some (payLife .you 4)))
            (exists_ (.and [land, .hasSubtype (landType "Swamp"), .hasPossessor .controller .you]))),
          .spell none (.doAndForbid
            (destroy (target (.and [creature, .not (.colorIs .black)])))
            (.action "Regenerate") (itVerbed (.action "Destroy"))) ] } }

def wrathOfGod : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Wrath of God", cost := some [generic 2, pip .white, pip .white],
      types := [.sorcery],
      text := [.spell none (.doAndForbid (destroy (allOf creature)) (.action "Regenerate") them)] }
          }

def damnation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Damnation", cost := some [generic 2, pip .black, pip .black], types := [.sorcery],
      text := [.spell none (.doAndForbid (destroy (allOf creature)) (.action "Regenerate") them)] }
          }

def glacialChasm : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Glacial Chasm", types := [.land],
      text :=
        [ cumulativeUpkeep (payLife .you 2),
          when (.enters thisLand none) (sacrifice (a land) (agent := .you)),
          .static (deontic (allOf creatureYouControl) .forbid [.core .attack] .agent .noPatient),
          .static (.damageRule .any .unattributed (.toRecipient .you) (.prevent .all none)
            .repeatedly) ] } }

/-- Peacekeeper -/
def peacekeeperCant : Ability :=
  .static (deontic (allOf creature) .forbid [.core .attack] .agent .noPatient)
theorem okPeacekeeperCant : Ability.check [] peacekeeperCant = [] := by decide

/-- Culling Mark -/
def cullingMark : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Culling Mark", cost := some [generic 2, pip .green], types := [.sorcery],
      text :=
        [ .spell none
            (.establish (deontic (target creature) .require [.core .block] .agent .noPatient)
              (some .thisTurn)) ] } }

/-- Blazing Archon -/
def blazingArchonCant : Ability :=
  .static (deontic (allOf creature) .forbid [.core .attack] .agent (.defendingPlayer .you))
theorem okBlazingArchonCant : Ability.check [] blazingArchonCant = [] := by decide

/-- Glaring Spotlight -/
def glaringSpotlight : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Glaring Spotlight", cost := some [generic 1], types := [.artifact],
      text :=
        [ .static (canBeTargetedAsThough
            (allOf (.and [ creature, .hasPossessor .controller (.playerGroup .yourOpponents),
                           .hasKeyword (.the "Hexproof") ]))
            (allOf (.and [.or [spell, .abilityHead .anyOnStack], .hasPossessor .controller .you]))
            (.not (.hasKeyword (.the "Hexproof")))),
          activated (.compound [.mana [generic 3], .perform (sacrifice thisArtifact (agent :=
              .you))])
            (.sequence
              [ gain (allOf creatureYouControl) (keyword "Hexproof") (some untilEndOfTurn),
                .establish
                  (deontic (allOf creatureYouControl) .forbid [.core .block] .patient .noPatient)
                  (some .thisTurn) ]) ] } }

/-- Canoptek Wraith -/
def canoptekWraith : Ability :=
  flavorWord "Wraith Form"
    (.static (.deonticRule thisCreature .forbid [.core .block] .patient none .noPatient none
        .noRider))
theorem okCanoptekWraith : Ability.check [] canoptekWraith = [] := by decide

/-- Gadrak, the Crown-Scourge -/
def gadrakCantAttack : Ability :=
  .static (onlyUnless (deontic thisCreature .forbid [.core .attack] .agent .noPatient)
    (.compareAmt (countOf (.and [artifact, .hasPossessor .controller .you])) .atLeast (.lit 4)))
theorem okGadrakCantAttack : Ability.check [] gadrakCantAttack = [] := by decide

/-- Berserker's Frenzy, the 1—14 striation -/
def berserkersFrenzyLowRoll : Instruction :=
  .sequence
    [ choose (counted anyNumber creature),
      .establish (deontic them .require [.core .block] .agent .noPatient) (some .thisTurn) ]
theorem okBerserkersFrenzyLowRoll : Instruction.check [] berserkersFrenzyLowRoll = [] := by decide

/-- Damn -/
def damnDestroyLine : Instruction :=
  .doAndForbid (destroy (target creature)) (.action "Regenerate")
    (theVerbed (.action "Destroy") (.type .creature) .thisWay .one)
theorem okDamnDestroyLine : Instruction.check [] damnDestroyLine = [] := by decide

def nekrataalWhole : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Nekrataal", cost := some [generic 2, pip .black, pip .black], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Assassin"],
      text :=
        [ keyword "FirstStrike",
          when (.enters thisCreature none)
            (.doAndForbid (destroy (target (.and [creature, .not artifact, .not (.colorIs
                .black)])))
              (.action "Regenerate") (that (.type .creature))) ],
      power := stat 2, toughness := stat 1 } }

/-- Concussive Bolt, both paragraphs -/
def concussiveBolt : Instruction :=
  .sequence
    [ .dealDamage .this (.lit 4) targetPlayerOrPlaneswalker,
      .doIf (.compareAmt (countOf (.and [artifact, .hasPossessor .controller .you])) .atLeast (.lit
          3))
        (.establish
          (deontic (allOf (.and [creature, .hasPossessor .controller splitOverPlaneswalker]))
            .forbid [.core .block] .agent .noPatient)
          (some .thisTurn))
        none ]
theorem okConcussiveBolt : Instruction.check [] concussiveBolt = [] := by decide

/-- Ulamog's Crusher -/
def ulamogsCrusher : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ulamog's Crusher", cost := some [generic 8], types := [.creature],
      subtypes := [creatureType "Eldrazi"],
      text :=
        [ keywordNumber "Annihilator" (.lit 2),
          .static (deontic thisCreature .require [.core .attack] .agent .noPatient) ],
      power := stat 8, toughness := stat 8 } }

/-- "attack you unless their controller pays {2} for each" -/
def attackToll : StaticSpec :=
  deontic (allOf creature)
    (.gatedBy (scaledMana .generic
      (times (.lit 2)
        (countOf (.and [creature, .hasPossessor .controller they, .inCombat .attackerOf (some .you)])))))
    [.core .attack] .agent (.defendingPlayer .you)

/-- Propaganda -/
def propaganda : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Propaganda", cost := some [generic 2, pip .blue], types := [.enchantment],
      text := [.static attackToll] } }

/-- Ghostly Prison -/
def ghostlyPrisonWhole : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ghostly Prison", cost := some [generic 2, pip .white], types := [.enchantment],
      text := [.static attackToll] } }

/-- Archangel of Tithes -/
def archangelOfTithesBlockToll : Ability :=
  .static (onlyWhile
    (deontic (allOf creature) (.gatedBy (scaledMana .generic (times (.lit 1) .groupSize)))
      [.core .block] .agent .noPatient)
    (.matches thisCreature attacking))
theorem okArchangelOfTithesBlockToll : Ability.check [] archangelOfTithesBlockToll = [] := by decide
/-- Archangel of Tithes -/
def archangelOfTithesAttackToll : Ability :=
  .static (onlyWhile
    (deontic (allOf creature) (.gatedBy (scaledMana .generic (times (.lit 1) .groupSize)))
      [.core .attack] .agent
      (.defendingPlayer
        (youOr (allOf (.and [.hasType .planeswalker, .hasPossessor .controller .you])))))
    (.matches thisCreature untapped))
theorem okArchangelOfTithesAttackToll : Ability.check [] archangelOfTithesAttackToll = [] := by
  decide

/-- Archon of Absolution -/
def archonOfAbsolution : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Archon of Absolution", cost := some [generic 3, pip .white], types := [.creature],
      subtypes := [creatureType "Archon"],
      text :=
        [ keyword "Flying",
          keywordQuality "Protection" (.colorIs .white),
          .static (deontic (allOf creature) (.gatedBy (scaledMana .generic (times (.lit 1) .groupSize)))
            [.core .attack] .agent
            (.defendingPlayer
              (youOr (allOf (.and [.hasType .planeswalker, .hasPossessor .controller .you]))))) ],
      power := stat 3, toughness := stat 2 } }

/-- Myr Prototype -/
def myrPrototype : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Myr Prototype", cost := some [generic 5], types := [.artifact, .creature],
      subtypes := [creatureType "Myr"],
      text :=
        [ at_ (.beginningOf .the .upkeep (.byPlayer .you))
            (.putCounters (.lit 1) (.printed plusOnePlusOne) thisCreature),
          .static (deontic thisCreature
            (.gatedBy (scaledMana .generic (times (.lit 1) (countersOn plusOnePlusOne it))))
            [.core .attack, .core .block] .agent .noPatient) ],
      power := stat 2, toughness := stat 2 } }

/-- Heat Wave -/
def heatWave : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Heat Wave", cost := some [generic 2, pip .red], types := [.enchantment],
      text :=
        [ cumulativeUpkeep (.mana [pip .red]),
          .static (deontic (allOf (.and [creature, .colorIs .blue])) .forbid [.core .block] .agent
            (.counterpart (allOf creatureYouControl))),
          .static (deontic (allOf (.and [creature, .not (.colorIs .blue)]))
            (.gatedBy (.perform (loseLife
              (times (.lit 1)
                (countOf (.and [creature, blocking, .hasPossessor .controller they]))) (agent :=
                    they))))
            [.core .block] .agent (.counterpart (allOf creatureYouControl))) ] } }

/-- Awesome Presence -/
def awesomePresence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Awesome Presence", cost := some [pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          .static (deontic (.attachHost .enchanted (.type .creature))
            (.gatedBy (scaledMana .generic (times (.lit 3)
              (countOf (.and [ creature, .hasPossessor .controller they,
                               .inCombat .blockerOf (some it) ])))))
            [.core .block] .patient .noPatient) ] } }

/-- Oppressive Rays -/
def oppressiveRays : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Oppressive Rays", cost := some [pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          .static (deontic (.attachHost .enchanted (.type .creature)) (.gatedBy (.mana [generic 3]))
            [.core .attack, .core .block] .agent .noPatient),
          .static (.costShift
            (allOf (.and [ .abilityHead .anyActivated,
                           .abilityOf (.attachHost .enchanted (.type .creature)) ]))
            (.more (.lit 3))) ] } }

/-- Distortion Strike -/
def distortionStrikeLine : Instruction :=
  .sequence
    [ get (target creature) (.up (.lit 1)) (.up (.lit 0)) (some untilEndOfTurn),
      .establish (deontic (that (.type .creature)) .forbid [.core .block] .patient .noPatient)
        (some .thisTurn) ]
theorem okDistortionStrikeLine : Instruction.check [] distortionStrikeLine = [] := by decide

/-- Retro-Mutation -/
def retroMutation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Retro-Mutation", cost := some [generic 2, pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keyword "Flash",
          keywordSubject "Enchant" creature,
          .static (.conjunction none
            [ .qualityChange (.attachHost .enchanted (.type .creature)) .sets
                (.bundle { characteristics := { subtypes := [creatureType "Turtle"] } } none),
              .modification it .power (.set (.lit 0)),
              .modification (itsOther it (.set (.lit 0))) .toughness (.set (.lit 1)),
              .deonticRule it .forbid [.core .attack] .agent none .noPatient none .noRider,
              .allAbilityLoss it none ]) ] } }

/-- Hotshot Mechanic -/
def hotshotMechanic : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hotshot Mechanic", cost := some [pip .white], types := [.artifact, .creature],
      subtypes := [creatureType "Fox", creatureType "Pilot"],
      text :=
        [ .static (.deonticRule thisCreature .permit [.ofAbility "Crew"] .agent none
            (.counterpart (allOf (.hasSubtype (artifactType "Vehicle"))))
            (some (.greater .power (.lit 2))) .noRider) ],
      power := stat 2, toughness := stat 1 } }

/-- Cloudspire Captain -/
def cloudspireCaptainCrewLine : StaticSpec :=
  .deonticRule thisCreature .permit [.ofAbility "Saddle", .ofAbility "Crew"] .agent none
    (.counterpartsAt
      [ ⟨.ofAbility "Saddle", allOf (.hasSubtype (creatureType "Mount"))⟩,
        ⟨.ofAbility "Crew", allOf (.hasSubtype (artifactType "Vehicle"))⟩ ])
    (some (.greater .power (.lit 2))) .noRider
theorem okCloudspireCaptainCrewLine : StaticSpec.check [] cloudspireCaptainCrewLine = [] := by
  decide

/-- Revoke Privileges -/
def revokePrivileges : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Revoke Privileges", cost := some [generic 2, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          .static (.deonticRule (.attachHost .enchanted (.type .creature)) .forbid
            [.core .attack, .core .block, .ofAbility "Crew"] .agent none
            (.counterpartsAt [⟨.ofAbility "Crew", allOf (.hasSubtype (artifactType "Vehicle"))⟩])
            none .noRider) ] } }

/-- Harried Spearguard -/
def harriedSpearguard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Harried Spearguard", cost := some [pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Soldier"],
      text :=
        [ keyword "Haste",
          when (.dies thisCreature)
            (create (.lit 1)
              { characteristics :=
                { colors := [.black], types := [.creature], subtypes := [creatureType "Rat"],
                  text :=
                    [ .static (.deonticRule (.asMarker .token .this) .forbid [.core .block] .agent
                        none
                        .noPatient none .noRider) ],
                  power := stat 1, toughness := stat 1 } }) ],
      power := stat 1, toughness := stat 1 } }

/-- Arrest -/
def arrest : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Arrest", cost := some [generic 2, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          .static (.conjunction none
            [ deontic (.attachHost .enchanted (.type .creature)) .forbid
                [.core .attack, .core .block] .agent .noPatient,
              deontic (allOf (.and [.abilityHead .anyActivated, .abilityOf it])) .forbid
                [.action "Activate"] .patient .noPatient ]) ] } }

/-- Conqueror's Flail -/
def conquerorsFlailProhibition : StaticSpec :=
  onlyWhile
    (.partScope .turn (some .you)
      (cantDoTo (.action "Cast") (.playerGroup .yourOpponents) (allOf spell)))
    (.matches .this (.attachedTo (a creature)))
theorem okConquerorsFlailProhibition : StaticSpec.check [] conquerorsFlailProhibition = [] := by
  decide

end Semantics.Cards

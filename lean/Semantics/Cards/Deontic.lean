import Semantics
import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Cards.Description

/-!
# Semantics.Cards.Deontic

Port of `idris/src/Experimental/Cards/Deontic.idr`: the printed cards of the Deontic family
(can't, must, may, tolls, and their windows) and the bench items beside them.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Cards

def infiltrate : Action := establishThroughout (cant any (block (target isCreature))) thisTurn
theorem okInfiltrate : Action.check (DRS.ofList []) infiltrate = [] := by decide
def changeOfHeart : Action := establishThroughout (cant (target isCreature) attack) thisTurn
theorem okChangeOfHeart : Action.check (DRS.ofList []) changeOfHeart = [] := by decide
def blindblast : Action :=
  sequentially
    [instruct this_ (dealDamage (.lit 1) (target isCreature) .noncombatOnly), establishThroughout
       (cant (that creature) block) thisTurn]
theorem okBlindblast : Action.check (DRS.ofList []) blindblast = [] := by decide
def blindingFlare : Action :=
  establishThroughout (cant (targets anyNumber isCreature) block) thisTurn
theorem okBlindingFlare : Action.check (DRS.ofList []) blindingFlare = [] := by decide
def cowardKiller : Action :=
  sequentially
    [ establishThroughout (cant (target isCreature) block) thisTurn,
      become (that creature) { characteristics := { subtypes := [coward] } }
        (some untilEndOfTurn) ]
theorem okCowardKiller : Action.check (DRS.ofList []) cowardKiller = [] := by decide

/-- Auriok Siege Sled -/
def auriokSiegeSledDenial : Ability :=
  activated (mana [generic 1])
    (establishThroughout (cant (target (and [isArtifact, isCreature])) (block thisCreature))
       thisTurn)
theorem okAuriokSiegeSledDenial : Ability.check (DRS.ofList []) auriokSiegeSledDenial = [] := by decide

/-- Blindblast -/
def blindblastWhole : Action :=
  sequentially
    [ instruct this_ (dealDamage (.lit 1) (target isCreature) .noncombatOnly), establishThroughout
        (cant (that creature) block) thisTurn,
      draw (.lit 1) ]
theorem okBlindblastWhole : Action.check (DRS.ofList []) blindblastWhole = [] := by decide

def sparkmagesGambit : Action :=
  sequentially
    [ instruct this_ (dealDamage (.lit 1) (eachOf (targets (upTo 2) isCreature)) .noncombatOnly),
      establishThroughout (cant (those creature) block) thisTurn ]
theorem okSparkmagesGambit : Action.check (DRS.ofList []) sparkmagesGambit = [] := by decide

/-- Glacial Chasm -/
def glacialChasmCant : Ability :=
  static (cant (allOf (and [isCreature, youControl])) attack)
theorem okGlacialChasmCant : Ability.check (DRS.ofList []) glacialChasmCant = [] := by decide

def desperateCastaways : Ability :=
  static (onlyUnless (cant thisCreature attack)
    (exists_ (and [isArtifact, youControl])))
theorem okDesperateCastaways : Ability.check (DRS.ofList []) desperateCastaways = [] := by decide

def munghaWurm : Ability :=
  static (cant you (untap (counted (moreThan 1) isLand)) (timing := some duringYourUntapStep))
theorem okMunghaWurm : Ability.check (DRS.ofList []) munghaWurm = [] := by decide
def dampingField : Ability :=
  static
      (cant (bare isPlayer) (untap (counted (moreThan 1) isArtifact))
         (timing := some duringTheirUntapSteps))
theorem okDampingField : Ability.check (DRS.ofList []) dampingField = [] := by decide
def smoke : Ability := static
    (cant (bare isPlayer) (untap (counted (moreThan 1) isCreature))
       (timing := some duringTheirUntapSteps))
theorem okSmoke : Ability.check (DRS.ofList []) smoke = [] := by decide
def winterOrb : Ability :=
  static
      (onlyWhile
         (cant (bare isPlayer) (untap (counted (moreThan 1) isLand))
            (timing := some duringTheirUntapSteps))
    (matches_ thisArtifact isUntapped))
theorem okWinterOrb : Ability.check (DRS.ofList []) winterOrb = [] := by decide
def staticOrb : Ability :=
  static
      (onlyWhile
         (cant (bare isPlayer) (untap (counted (moreThan 2) isPermanent))
            (timing := some duringTheirUntapSteps))
    (matches_ thisArtifact isUntapped))
theorem okStaticOrb : Ability.check (DRS.ofList []) staticOrb = [] := by decide

/-- Rule of Law -/
def ruleOfLaw : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rule of Law", cost := some [generic 2, pip .white], types := [.enchantment],
      text := [static (cant eachPlayer (cast (counted (moreThan 1) isSpell))
        (timing := some eachTurn))] } }

/-- Spirit of the Labyrinth -/
def spiritOfTheLabyrinth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Spirit of the Labyrinth", cost := some [generic 1, pip .white],
      types := [.enchantment, .creature], subtypes := [spirit],
      text := [static (cant eachPlayer (drawCount (moreThan 1)) (timing := some eachTurn))],
      power := stat 3, toughness := stat 1 } }

def winterMoon : Ability :=
  static
      (cant (bare isPlayer)
         (untap (counted (moreThan 1) (and [isLand, non (hasSupertype .basic)])))
      (timing := some duringTheirUntapSteps))
theorem okWinterMoon : Ability.check (DRS.ofList []) winterMoon = [] := by decide

def leitmotifComposer : Ability :=
  activated (mana [generic 2, pip .blue])
    (establishThroughout
       (cant any
          (block (allOf (and [isCreature, named (printedName "Leitmotif Composer")])))) thisTurn)
theorem okLeitmotifComposer : Ability.check (DRS.ofList []) leitmotifComposer = [] := by decide

def berserkersOfBloodRidge : Ability :=
  static (must thisCreature attack)
theorem okBerserkersOfBloodRidge : Ability.check (DRS.ofList []) berserkersOfBloodRidge = [] := by decide

def trumpetingArmodon : Ability :=
  activated (mana [generic 1, pip .green])
    (establishThroughout (must (target isCreature) (block thisCreature)) thisTurn)
theorem okTrumpetingArmodon : Ability.check (DRS.ofList []) trumpetingArmodon = [] := by decide

def loathsomeCatoblepas : Ability :=
  activated (mana [generic 2, pip .green])
    (establishThroughout (must (by_ (allOf isCreature)) (block thisCreature)) thisTurn)
theorem okLoathsomeCatoblepas : Ability.check (DRS.ofList []) loathsomeCatoblepas = [] := by decide

def hipparion : Ability :=
  static
      (gatedBy (mana [generic 1]) thisCreature
         (block (allOf (and [isCreature, hasPower .atLeast (.lit 3)]))))
theorem okHipparion : Ability.check (DRS.ofList []) hipparion = [] := by decide

def frodoBaggins : Ability :=
  static (onlyWhile (must (by_ (allOf isCreature)) (block thisCreature))
    (matches_ thisCreature (hasDesignation ringBearer (some you))))
theorem okFrodoBaggins : Ability.check (DRS.ofList []) frodoBaggins = [] := by decide

def bloodshedFever : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bloodshed Fever", cost := some [pip .red], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ enchant isCreature,
          static (must (enchanted creature) attack) ] } }

/-- Pacifism -/
def pacifism : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Pacifism", cost := some [generic 1, pip .white], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ enchant isCreature,
          static (conjunction
                    [cant (enchanted creature) attack, cant
                       (enchanted creature) block]) ] } }

/-- Everybody Lives! -/
def everybodyLivesGateLine : Action :=
  establishThroughout (conjunction
                         [cant (bare isPlayer) loseGame, cant (bare isPlayer)
                            winGame]) thisTurn
theorem okEverybodyLivesGateLine : Action.check (DRS.ofList []) everybodyLivesGateLine = [] := by decide

/-- Gaea's Revenge -/
def gaeasRevenge : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gaea's Revenge", cost := some [generic 5, pip .green, pip .green],
      types := [.creature], subtypes := [elemental],
      text :=
        [ static (cant any (counter thisSpell)),
          haste,
          static (cant (by_ (allOf (or [ and [isSpell, non (hasColor .green)],
                          abilityOf (allOf (and [isSource, non (hasColor .green)])) ])))
                              (target thisCreature)) ],
      power := stat 8, toughness := stat 5 } }

/-- Nowhere to Run -/
def nowhereToRunWardLine : StaticSpec :=
  conjunction [ can (by_ (allOf (or [isSpell, abilityHead .anyOnStack]))) (target (allOf (and [isCreature, opponentsControl]))) (asThough := some (holds (matches_ them (not (hasKeyword hexproof))))),
      cant (allOf (and [abilityHead (.keyword ward), abilityOf (those creature)]))
          trigger ]
theorem okNowhereToRunWardLine : StaticSpec.check (DRS.ofList []) nowhereToRunWardLine = [] := by decide

/-- Mornsong Aria -/
def mornsongAriaLock : StaticSpec :=
  conjunction [cant (bare isPlayer) (draw anyAmount), cant (bare isPlayer)
                 (gainLife anyAmount)]
theorem okMornsongAriaLock : StaticSpec.check (DRS.ofList []) mornsongAriaLock = [] := by decide

def brainwash : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Brainwash", cost := some [pip .white], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ enchant isCreature,
          static (gatedBy (mana [generic 3]) (enchanted creature) attack) ] } }

def enkiraHostileScavenger : Ability :=
  static (onlyWhile (must (by_ (allOf isCreature)) (block thisCreature))
    (matches_ thisCreature (isAttached (some .equipped))))
theorem okEnkiraHostileScavenger : Ability.check (DRS.ofList []) enkiraHostileScavenger = [] := by decide

def anglerTurtle : Ability :=
  static (must (allOf (and [isCreature, opponentsControl])) attack)
theorem okAnglerTurtle : Ability.check (DRS.ofList []) anglerTurtle = [] := by decide

def ensnaringBridge : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ensnaring Bridge", cost := some [generic 3], types := [.artifact],
      text :=
        [ static (cant (allOf (and [ isCreature,
                           hasPower .greater (countOf (cardIn yourHand)) ])) attack) ] } }

def undercoverButler : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Undercover Butler", cost := some [generic 2, hybridPip .blue .black],
      types := [.creature], subtypes := [human, rogue],
      text :=
        [ whenever
            (does thisCreature
               (attack
                  (some
                     (the (and [isPlayer, superlative .max (.playerStat .lifeTotal) isPlayer])))))
            (establishThroughout (cant any (block it)) thisTurn) ],
      power := stat 2, toughness := stat 3 } }

def aetherTunnel : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aether Tunnel", cost := some [generic 1, pip .blue], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ enchant isCreature,
          static (conjunction [ getsPt (enchanted creature) (up (.lit 1)) (up (.lit 0)),
              cant any (block it) ]) ] } }

def excruciator : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Excruciator", cost := some [generic 6, pip .red, pip .red], types := [.creature],
      subtypes := [avatar],
      text :=
        [ static (preventionBan .any (damageDealt (dealtBy thisCreature) everywhere)
            .noPreventionOnly) ],
      power := stat 7, toughness := stat 7 } }

def flaringPain : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Flaring Pain", cost := some [generic 1, pip .red], types := [.instant],
      text :=
        [ spell (establishThroughout (preventionBan .any (damageDealt unattributed everywhere)
                .noPreventionOnly) thisTurn),
          flashback (mana [pip .red]) ] } }

def gideonJura : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gideon Jura", cost := some [generic 3, pip .white, pip .white],
      supertypes := [.legendary], types := [.planeswalker], subtypes := [gideon],
      text :=
        [ activated (loyaltySymbol (.up 2))
            (establishThroughout
              (must (allOf (and [isCreature, controlledBy (target isOpponent)]))
                 (attack (some thisPlaneswalker)))
              (duringNextTurnOf (that player))),
          activated (loyaltySymbol (.down 2)) (destroy (target (and [isCreature, isTapped]))),
          activated (loyaltySymbol .zero)
            (sequentially
              [ establishThroughout (qualityChange thisPlaneswalker .sets
                    (bundle
                      { characteristics :=
                        { types := [.creature],
                          subtypes := [human, soldier],
                          power := stat 6, toughness := stat 6 } }
                      (some .planeswalker))) untilEndOfTurn,
                preventAll .any (toRecipient it) (some thisTurn) ]) ],
      loyalty := stat 6 } }

def pinpointAvalanche : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Pinpoint Avalanche", cost := some [generic 3, pip .red, pip .red],
      types := [.instant],
      text :=
        [ spell (sequentially
            [ instruct this_ (dealDamage (.lit 4) (target isCreature) .noncombatOnly),
              establish (preventionBan .any thatDamage .noPreventionOnly) ]) ] } }

def whippoorwillImmunity : Action :=
  sequentially
    [ establishThroughout (cant any (regeneration (target isCreature))) thisTurn,
      establishThroughout (preventionBan .any
          (damageDealt unattributed
             (toRecipient (that creature))) .noRedirectEither) thisTurn ]
theorem okWhippoorwillImmunity : Action.check (DRS.ofList []) whippoorwillImmunity = [] := by decide

def callInAProfessional : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Call In a Professional", cost := some [generic 2, pip .red], types := [.instant],
      text :=
        [ spell (sequentially
            [ establishThroughout (cant (bare isPlayer) (gainLife anyAmount)) thisTurn,
              establishThroughout (preventionBan .any (damageDealt unattributed everywhere)
                                     .noPreventionOnly) thisTurn,
              instruct this_ (dealDamage (.lit 3) (target isAnyTarget) .noncombatOnly) ]) ] } }

/-- Council of the Absolute -/
def councilOfTheAbsolute : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Council of the Absolute", cost := some [generic 2, pip .white, pip .blue],
      types := [.creature], subtypes := [human, advisor],
      text :=
        [ static (entersChoosingFrom thisCreature .cardName
            (nameOfCard (not (or [isCreature, isLand])))),
          static (cant yourOpponents (cast (allOf (and [isSpell, named chosenName])))),
          static (costShift (allOf (and [isSpell, named chosenName, castBy you]))
            (less (.lit 2))) ],
      power := stat 2, toughness := stat 4 } }

/-- Failure // Comply -/
def complyNameLock : Action :=
  sequentially
    [ choose (a (quality .cardName)),
      establishThroughout (cant yourOpponents
                             (cast (allOf (and [isSpell, named chosenName])))) untilYourNextTurn ]
theorem okComplyNameLock : Action.check (DRS.ofList []) complyNameLock = [] := by decide

/-- Gideon's Intervention -/
def gideonsIntervention : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gideon's Intervention", cost := some [generic 2, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ static (entersChoosing thisEnchantment .cardName),
          static (cant yourOpponents (cast (allOf (and [isSpell, named chosenName])))),
          static (damageRule .any (dealtBy (allOf (and [isSource, named chosenName])))
            (toRecipient (youAnd (allOf (and [isPermanent, youControl]))))
            (prevent allDamage) .repeatedly) ] } }

/-- Academic Probation -/
def academicProbationNameMode : Action :=
  sequentially
    [ choose (a (qualityFrom .cardName (nameOfCard (non isLand)))),
      establishThroughout (cant yourOpponents
                             (cast (allOf (and [isSpell, named chosenName])))) untilYourNextTurn ]
theorem okAcademicProbationNameMode : Action.check (DRS.ofList []) academicProbationNameMode = [] := by
  decide

def fatigue : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fatigue", cost := some [generic 1, pip .blue], types := [.sorcery],
      text := [spell (instruct (target isPlayer) (skipPart .drawStep (.lit 1)))] } }

def meditate : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Meditate", cost := some [generic 2, pip .blue], types := [.instant],
      text := [spell (sequentially [draw (.lit 4), skipPart .turn (.lit 1)])] } }

/-- Blinding Angel {3}{W}{W} — Creature — Angel 2/4. "Flying. Whenever Blinding Angel deals
combat damage to a player, that player skips their next combat phase." -/
def blindingAngel : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blinding Angel", cost := some [generic 3, pip .white, pip .white],
      types := [.creature], subtypes := [angel],
      text :=
        [ flying,
          whenever
            (does thisCreature (dealDamage (to := a isPlayer) (kind := .combatOnly)))
            (instruct (that player) (skipPart .combat (.lit 1))) ],
      power := stat 2, toughness := stat 4 } }

def eonHub : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Eon Hub", cost := some [generic 5], types := [.artifact],
      text := [static (partSkip (bare isPlayer) .upkeep)] } }

def stasis : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stasis", cost := some [generic 1, pip .blue], types := [.enchantment],
      text :=
        [ static (partSkip (bare isPlayer) .untapStep),
          at_ (beginningOfYour .upkeep)
            (doUnless (sacrifice thisEnchantment) (mana [pip .blue])) ] } }

def yawgmothsBargain : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Yawgmoth's Bargain", cost := some [generic 4, pip .black, pip .black],
      types := [.enchantment],
      text :=
        [ static (partSkip you .drawStep),
          activated (payLife 1) (draw (.lit 1)) ] } }

def sandsOfTimeSkip : StaticSpec := partSkip eachPlayer .untapStep
theorem okSandsOfTimeSkip : StaticSpec.check (DRS.ofList []) sandsOfTimeSkip = [] := by decide

/-- Wormfang Manta {5}{U}{U} — Creature — Nightmare Fish Beast 6/1. "Flying. When Wormfang
Manta enters, you skip your next turn. When Wormfang Manta leaves the battlefield, you take an
extra turn after this one." -/
def wormfangManta : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Wormfang Manta", cost := some [generic 5, pip .blue, pip .blue],
      types := [.creature],
      subtypes := [nightmare, fish, beast],
      text :=
        [ flying,
          when (does thisCreature enter) (skipPart .turn (.lit 1)),
          when (does thisCreature leave) (extraTurn (.lit 1)) ],
      power := stat 6, toughness := stat 1 } }

def eaterOfDaysSkip : Action := skipPart .turn (.lit 2)
theorem okEaterOfDaysSkip : Action.check (DRS.ofList []) eaterOfDaysSkip = [] := by decide

/-- Empty City Ruse -/
def emptyCityRuse : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Empty City Ruse", cost := some [pip .white], types := [.sorcery],
      text :=
        [ spell (establishThroughout (partSkip (target isOpponent) .combat) (duringNextTurnOf (that
                .player))) ] } }

/-- False Peace -/
def falsePeace : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "False Peace", cost := some [pip .white], types := [.sorcery],
      text :=
        [ spell (establishThroughout (partSkip (target isPlayer) .combat) (duringNextTurnOf (that
                .player))) ] } }

/-- Battlefront Krushok -/
def battlefrontKrushokEvasion : Ability :=
  static (cant (by_ (counted (moreThan 1) isCreature)) (block thisCreature))
theorem okBattlefrontKrushokEvasion : Ability.check (DRS.ofList []) battlefrontKrushokEvasion = [] := by decide

/-- Grand Abolisher -/
def grandAbolisher : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Grand Abolisher", cost := some [pip .white, pip .white], types := [.creature],
      subtypes := [human, cleric],
      text :=
        [ static (partScope .turn (some you)
            (conjunction [ cant yourOpponents (cast (allOf isSpell)),
                cant yourOpponents (activate (allOf (and [ abilityHead .anyActivated,
                                 abilityOf (allOf (or [isArtifact, isCreature, isEnchantment])) ]))) ])) ],
      power := stat 2, toughness := stat 2 } }

/-- Festival -/
def festival : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Festival", cost := some [pip .white], types := [.instant],
      text :=
        [ spell (establishThroughout (cant (allOf isCreature) attack) thisTurn)
            (timing := some (duringPart .upkeep (some anOpponent))) ] } }

def demotion : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Demotion", cost := some [pip .white], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ enchant isCreature,
          static (conjunction [ cant (enchanted creature) block,
              cant any (activate (allOf (and [abilityHead .anyActivated, abilityOf it]))) ]) ] } }

def terror : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Terror", cost := some [generic 1, pip .black], types := [.instant],
      text :=
        [ spell (destroyCantRegenerate
            (target (and [isCreature, non isArtifact, non (hasColor .black)]))) ] } }

def snuffOut : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Snuff Out", cost := some [generic 3, pip .black], types := [.instant],
      text :=
        [ static (onlyWhile (altCost this_ (some (payLife 4)))
            (exists_ (and [isLand, hasSubtype swamp, youControl]))),
          spell (destroyCantRegenerate (target (and [isCreature, non (hasColor .black)]))) ] } }

def wrathOfGod : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Wrath of God", cost := some [generic 2, pip .white, pip .white],
      types := [.sorcery],
      text := [spell (destroyCantRegenerate (allOf isCreature))] }
          }

def damnation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Damnation", cost := some [generic 2, pip .black, pip .black], types := [.sorcery],
      text := [spell (destroyCantRegenerate (allOf isCreature))] }
          }

def glacialChasm : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Glacial Chasm", types := [.land],
      text :=
        [ cumulativeUpkeep (payLife 2),
          when (does thisLand enter) (sacrifice (a isLand)),
          static (cant (allOf (and [isCreature, youControl])) attack),
          static (damageRule .any unattributed (toRecipient you) (prevent allDamage)
            .repeatedly) ] } }

/-- Peacekeeper -/
def peacekeeperCant : Ability :=
  static (cant (allOf isCreature) attack)
theorem okPeacekeeperCant : Ability.check (DRS.ofList []) peacekeeperCant = [] := by decide

/-- Culling Mark -/
def cullingMark : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Culling Mark", cost := some [generic 2, pip .green], types := [.sorcery],
      text :=
        [ spell (establishThroughout (must (target isCreature) block) thisTurn) ] } }

/-- Blazing Archon -/
def blazingArchonCant : Ability :=
  static (cant (allOf isCreature) (attack (some you)))
theorem okBlazingArchonCant : Ability.check (DRS.ofList []) blazingArchonCant = [] := by decide

/-- Glaring Spotlight -/
def glaringSpotlight : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Glaring Spotlight", cost := some [generic 1], types := [.artifact],
      text :=
        [ static
            (can (by_ (allOf (and [or [isSpell, abilityHead .anyOnStack], youControl])))
               (target (allOf (and [ isCreature, opponentsControl,
                           hasKeyword hexproof ]))) (asThough := some (holds (matches_ them (not (hasKeyword hexproof)))))),
          activated (compound [mana [generic 3], perform (sacrifice thisArtifact)])
            (sequentially
              [ gain (allOf (and [isCreature, youControl])) hexproof (some untilEndOfTurn),
                establishThroughout (cant any
                                       (block
                                          (allOf (and [isCreature, youControl])))) thisTurn ]) ] } }

/-- Canoptek Wraith -/
def canoptekWraith : Ability :=
  flavorWord "Wraith Form"
    (static (cant any (block thisCreature)))
theorem okCanoptekWraith : Ability.check (DRS.ofList []) canoptekWraith = [] := by decide

/-- Gadrak, the Crown-Scourge -/
def gadrakCantAttack : Ability :=
  static (onlyUnless (cant thisCreature attack)
    (compareAmt (countOf (and [isArtifact, youControl])) .atLeast (.lit 4)))
theorem okGadrakCantAttack : Ability.check (DRS.ofList []) gadrakCantAttack = [] := by decide

/-- Berserker's Frenzy, the 1—14 striation -/
def berserkersFrenzyLowRoll : Action :=
  sequentially
    [ choose (counted anyNumber isCreature),
      establishThroughout (must them block) thisTurn ]
theorem okBerserkersFrenzyLowRoll : Action.check (DRS.ofList []) berserkersFrenzyLowRoll = [] := by decide

/-- Damn -/
def damnDestroyLine : Action :=
  doAndForbid (destroy (target isCreature))
    (regeneration (theVerbed (.action "destroy") .one (some creature) .thisWay))
theorem okDamnDestroyLine : Action.check (DRS.ofList []) damnDestroyLine = [] := by decide

def nekrataalWhole : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Nekrataal", cost := some [generic 2, pip .black, pip .black], types := [.creature],
      subtypes := [human, assassin],
      text :=
        [ firstStrike,
          when (does thisCreature enter)
            (doAndForbid (destroy (target (and [isCreature, non isArtifact, not (hasColor
                .black)])))
              (regeneration (that creature))) ],
      power := stat 2, toughness := stat 1 } }

/-- Concussive Bolt, both paragraphs -/
def concussiveBolt : Action :=
  sequentially
    [ instruct this_ (dealDamage (.lit 4) targetPlayerOrPlaneswalker .noncombatOnly),
      doIf (compareAmt (countOf (and [isArtifact, youControl])) .atLeast (.lit
          3))
        (establishThroughout
           (cant (allOf (and [isCreature, controlledBy splitOverPlaneswalker]))
              block) thisTurn) ]
theorem okConcussiveBolt : Action.check (DRS.ofList []) concussiveBolt = [] := by decide

/-- Ulamog's Crusher -/
def ulamogsCrusher : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ulamog's Crusher", cost := some [generic 8], types := [.creature],
      subtypes := [eldrazi],
      text :=
        [ annihilator (.lit 2),
          static (must thisCreature attack) ],
      power := stat 8, toughness := stat 8 } }

/-- "attack you unless their controller pays {2} for each" -/
def attackToll : StaticSpec :=
  gatedBy (scaledGenericMana
      (forEach (.lit 2)
         (and [isCreature, controlledBy they, inCombat .attackerOf (some you)]))) (actor
                                                                                     (allOf
                                                                                        isCreature)) (attack (some you))

/-- Propaganda -/
def propaganda : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Propaganda", cost := some [generic 2, pip .blue], types := [.enchantment],
      text := [static attackToll] } }

/-- Ghostly Prison -/
def ghostlyPrisonWhole : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ghostly Prison", cost := some [generic 2, pip .white], types := [.enchantment],
      text := [static attackToll] } }

/-- Archangel of Tithes -/
def archangelOfTithesBlockToll : Ability :=
  static (onlyWhile
    (gatedBy (scaledGenericMana (times (.lit 1) groupSize)) (allOf isCreature) block)
    (matches_ thisCreature isAttacking))
theorem okArchangelOfTithesBlockToll : Ability.check (DRS.ofList []) archangelOfTithesBlockToll = [] := by decide
/-- Archangel of Tithes -/
def archangelOfTithesAttackToll : Ability :=
  static (onlyWhile
    (gatedBy (scaledGenericMana (times (.lit 1) groupSize)) (allOf isCreature)
       (attack (some (youOr (allOf (and [isPlaneswalker, youControl]))))))
    (matches_ thisCreature isUntapped))
theorem okArchangelOfTithesAttackToll : Ability.check (DRS.ofList []) archangelOfTithesAttackToll = [] := by
  decide

/-- Archon of Absolution -/
def archonOfAbsolution : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Archon of Absolution", cost := some [generic 3, pip .white], types := [.creature],
      subtypes := [archon],
      text :=
        [ flying,
          protection (hasColor .white),
          static (gatedBy (scaledGenericMana (times (.lit 1) groupSize)) (allOf isCreature)
                    (attack (some (youOr (allOf (and [isPlaneswalker, youControl])))))) ],
      power := stat 3, toughness := stat 2 } }

/-- Myr Prototype -/
def myrPrototype : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Myr Prototype", cost := some [generic 5], types := [.artifact, .creature],
      subtypes := [myr],
      text :=
        [ at_ (beginningOfYour .upkeep)
            (putCounters (.lit 1) (printedKind p1p1Counter) thisCreature .many),
          static
              (conjunction
                 [gatedBy (scaledGenericMana (times (.lit 1) (countersOn p1p1Counter it)))
                    thisCreature attack, gatedBy
              (scaledGenericMana (times (.lit 1) (countersOn p1p1Counter it))) thisCreature
              block]) ],
      power := stat 2, toughness := stat 2 } }

/-- Heat Wave -/
def heatWave : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Heat Wave", cost := some [generic 2, pip .red], types := [.enchantment],
      text :=
        [ cumulativeUpkeep (mana [pip .red]),
          static
              (cant (allOf (and [isCreature, hasColor .blue]))
                 (block (allOf (and [isCreature, youControl])))),
          static (gatedBy (perform (instruct they (loseLife
              (forEach (.lit 1) (and [isCreature, isBlocking, controlledBy they]))))) (actor
                                                                                         (allOf
                                                                                            (and
                                                                                               [isCreature, non (hasColor .blue)]))) (block (allOf (and [isCreature, youControl])))) ] } }

/-- Awesome Presence -/
def awesomePresence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Awesome Presence", cost := some [pip .blue], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ enchant isCreature,
          static (gatedBy (scaledGenericMana (forEach (.lit 3) (and [ isCreature, controlledBy they,
                               inCombat .blockerOf (some it) ]))) any (block
                                                                         (enchanted
                                                                            (.type
                                                                               .creature)))) ] } }

/-- Oppressive Rays -/
def oppressiveRays : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Oppressive Rays", cost := some [pip .white], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ enchant isCreature,
          static (conjunction
                    [gatedBy (mana [generic 3]) (enchanted creature) attack,
                     gatedBy (mana [generic 3]) (enchanted creature) block]),
          static (costShift
            (allOf (and [ abilityHead .anyActivated,
                           abilityOf (enchanted creature) ]))
            (more (.lit 3))) ] } }

/-- Distortion Strike -/
def distortionStrikeLine : Action :=
  sequentially
    [ get (target isCreature) (up (.lit 1)) (up (.lit 0)) (some untilEndOfTurn),
      establishThroughout (cant any (block (that creature))) thisTurn ]
theorem okDistortionStrikeLine : Action.check (DRS.ofList []) distortionStrikeLine = [] := by decide

/-- Retro-Mutation -/
def retroMutation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Retro-Mutation", cost := some [generic 2, pip .blue], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ flash,
          enchant isCreature,
          static (conjunction [ qualityChange (enchanted creature) .sets
                (bundle { characteristics := { subtypes := [turtle] } }),
              getsBase it (.lit 0) (.lit 1),
              cant it attack,
              allAbilityLoss it ]) ] } }

/-- Hotshot Mechanic -/
def hotshotMechanic : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hotshot Mechanic", cost := some [pip .white], types := [.artifact, .creature],
      subtypes := [fox, pilot],
      text :=
        [ static (can thisCreature (crew (allOf (hasSubtype
        vehicle))) (asThough := some (greaterBy .power (.lit 2)))) ],
      power := stat 2, toughness := stat 1 } }

/-- Cloudspire Captain -/
def cloudspireCaptainCrewLine : StaticSpec :=
  conjunction [ can thisCreature (saddle (allOf (hasSubtype
          mount))) (asThough := some (greaterBy .power (.lit 2))),
      can thisCreature (crew (allOf (hasSubtype
          vehicle))) (asThough := some (greaterBy .power (.lit 2))) ]
theorem okCloudspireCaptainCrewLine : StaticSpec.check (DRS.ofList []) cloudspireCaptainCrewLine = [] := by
  decide

/-- Revoke Privileges -/
def revokePrivileges : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Revoke Privileges", cost := some [generic 2, pip .white], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ enchant isCreature,
          static (conjunction [ cant (enchanted creature) attack,
        cant (enchanted creature) block,
        cant (enchanted creature) (crew (allOf (hasSubtype
            vehicle))) ]) ] } }

/-- Harried Spearguard -/
def harriedSpearguard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Harried Spearguard", cost := some [pip .red], types := [.creature],
      subtypes := [human, soldier],
      text :=
        [ haste,
          when (does thisCreature die)
            (createToken
              { characteristics :=
                { colors := [.black], types := [.creature], subtypes := [rat],
                  text :=
                    [ static (cant thisToken block) ],
                  power := stat 1, toughness := stat 1 } }) ],
      power := stat 1, toughness := stat 1 } }

/-- Arrest -/
def arrest : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Arrest", cost := some [generic 2, pip .white], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ enchant isCreature,
          static (conjunction
            [ cant (enchanted creature) attack,
              cant (enchanted creature) block,
              cant any (activate (allOf (and [abilityHead .anyActivated, abilityOf it]))) ]) ] } }

/-- Conqueror's Flail -/
def conquerorsFlailProhibition : StaticSpec :=
  onlyWhile
    (partScope .turn (some you)
      (cant yourOpponents (cast (allOf isSpell))))
    (matches_ this_ (attachedTo (a isCreature)))
theorem okConquerorsFlailProhibition : StaticSpec.check (DRS.ofList []) conquerorsFlailProhibition = [] := by
  decide

/-- Retrace's permission: the land card discarded is the cost of casting, not what is cast
[CR#702.81a,601.2h]. -/
def castFromGraveyardByDiscardingALand : StaticSpec :=
  can you
    (cast this_ (from_ := some yourGraveyard)
      (payment := payingInstead
        (compound [itsManaCost, perform (discard (a (and [isCard, isLand])))])))
theorem okCastFromGraveyardByDiscardingALand :
    StaticSpec.check (DRS.ofList []) castFromGraveyardByDiscardingALand = [] := by decide
end Semantics.Cards

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
    [instruct this_ (dealDamage 1 (target isCreature)), establishThroughout
       (cant (that creature) block) thisTurn]
theorem okBlindblast : Action.check (DRS.ofList []) blindblast = [] := by decide
def blindingFlare : Action :=
  establishThroughout (cant (targets anyNumber isCreature) block) thisTurn
theorem okBlindingFlare : Action.check (DRS.ofList []) blindingFlare = [] := by decide
def cowardKiller : Action :=
  sequentially
    [ establishThroughout (cant (target isCreature) block) thisTurn,
      become (that creature) { characteristics := { subtypes := [coward] } }
        untilEndOfTurn ]
theorem okCowardKiller : Action.check (DRS.ofList []) cowardKiller = [] := by decide

/-- Auriok Siege Sled -/
def auriokSiegeSledDenial : Ability :=
  activated [generic 1]
    (establishThroughout (cant (target (and [isArtifact, isCreature])) (block thisCreature))
       thisTurn)
theorem okAuriokSiegeSledDenial : Ability.check (DRS.ofList []) auriokSiegeSledDenial = [] := by decide

/-- Blindblast -/
def blindblastWhole : Action :=
  sequentially
    [ instruct this_ (dealDamage 1 (target isCreature)), establishThroughout
        (cant (that creature) block) thisTurn,
      draw 1 ]
theorem okBlindblastWhole : Action.check (DRS.ofList []) blindblastWhole = [] := by decide

def sparkmagesGambit : Action :=
  sequentially
    [ instruct this_ (dealDamage 1 (eachOf (targets (upTo 2) isCreature))),
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
  static (cant you (untap (counted (moreThan 1) isLand)) (timing := duringYourUntapStep))
theorem okMunghaWurm : Ability.check (DRS.ofList []) munghaWurm = [] := by decide
def dampingField : Ability :=
  static
      (cant (bare isPlayer) (untap (counted (moreThan 1) isArtifact))
         (timing := duringTheirUntapSteps))
theorem okDampingField : Ability.check (DRS.ofList []) dampingField = [] := by decide
def smoke : Ability := static
    (cant (bare isPlayer) (untap (counted (moreThan 1) isCreature))
       (timing := duringTheirUntapSteps))
theorem okSmoke : Ability.check (DRS.ofList []) smoke = [] := by decide
def winterOrb : Ability :=
  static
      (onlyWhile
         (cant (bare isPlayer) (untap (counted (moreThan 1) isLand))
            (timing := duringTheirUntapSteps))
    (is thisArtifact isUntapped))
theorem okWinterOrb : Ability.check (DRS.ofList []) winterOrb = [] := by decide
def staticOrb : Ability :=
  static
      (onlyWhile
         (cant (bare isPlayer) (untap (counted (moreThan 2) isPermanent))
            (timing := duringTheirUntapSteps))
    (is thisArtifact isUntapped))
theorem okStaticOrb : Ability.check (DRS.ofList []) staticOrb = [] := by decide

/-- Rule of Law -/
def ruleOfLaw : Spelled := spelled <| singleFacedCard
  { name := "Rule of Law", cost := [generic 2, pip .white], types := [.enchantment],
    text := [static (cant eachPlayer (cast (counted (moreThan 1) isSpell))
      (timing := eachTurn))] }

/-- Spirit of the Labyrinth -/
def spiritOfTheLabyrinth : Spelled := spelled <| singleFacedCard
  { name := "Spirit of the Labyrinth", cost := [generic 1, pip .white],
    types := [.enchantment, .creature], subtypes := [spirit],
    text := [static (cant eachPlayer (drawCount (moreThan 1)) (timing := eachTurn))],
    power := stat 3, toughness := stat 1 }

def winterMoon : Ability :=
  static
      (cant (bare isPlayer)
         (untap (counted (moreThan 1) (and [isLand, non (hasSupertype .basic)])))
      (timing := duringTheirUntapSteps))
theorem okWinterMoon : Ability.check (DRS.ofList []) winterMoon = [] := by decide

def leitmotifComposer : Ability :=
  activated [generic 2, pip .blue]
    (establishThroughout
       (cant any
          (block (allOf (and [isCreature, named "Leitmotif Composer"])))) thisTurn)
theorem okLeitmotifComposer : Ability.check (DRS.ofList []) leitmotifComposer = [] := by decide

def berserkersOfBloodRidge : Ability :=
  static (must thisCreature attack)
theorem okBerserkersOfBloodRidge : Ability.check (DRS.ofList []) berserkersOfBloodRidge = [] := by decide

def trumpetingArmodon : Ability :=
  activated [generic 1, pip .green]
    (establishThroughout (must (target isCreature) (block thisCreature)) thisTurn)
theorem okTrumpetingArmodon : Ability.check (DRS.ofList []) trumpetingArmodon = [] := by decide

def loathsomeCatoblepas : Ability :=
  activated [generic 2, pip .green]
    (establishThroughout (must any (block thisCreature)) thisTurn)
theorem okLoathsomeCatoblepas : Ability.check (DRS.ofList []) loathsomeCatoblepas = [] := by decide

def hipparion : Ability :=
  static
      (gatedBy [generic 1] thisCreature
         (block (allOf (and [isCreature, hasPower .atLeast 3]))))
theorem okHipparion : Ability.check (DRS.ofList []) hipparion = [] := by decide

def frodoBaggins : Ability :=
  static (onlyWhile (must any (block thisCreature))
    (is thisCreature (hasDesignation ringBearer you)))
theorem okFrodoBaggins : Ability.check (DRS.ofList []) frodoBaggins = [] := by decide

def bloodshedFever : Spelled := spelled <| singleFacedCard
  { name := "Bloodshed Fever", cost := [pip .red], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (must (enchanted creature) attack) ] }

/-- Pacifism -/
def pacifism : Spelled := spelled <| singleFacedCard
  { name := "Pacifism", cost := [generic 1, pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (conjunction
                  [cant (enchanted creature) attack, cant
                     (enchanted creature) block]) ] }

/-- Everybody Lives! -/
def everybodyLivesGateLine : Action :=
  establishThroughout (conjunction
                         [cant (bare isPlayer) loseGame, cant (bare isPlayer)
                            winGame]) thisTurn
theorem okEverybodyLivesGateLine : Action.check (DRS.ofList []) everybodyLivesGateLine = [] := by decide

/-- Gaea's Revenge -/
def gaeasRevenge : Spelled := spelled <| singleFacedCard
  { name := "Gaea's Revenge", cost := [generic 5, pip .green, pip .green],
    types := [.creature], subtypes := [elemental],
    text :=
      [ static (cant any (counter thisSpell)),
        haste,
        static (cant (by_ (allOf (or [ and [isSpell, non (hasColor .green)],
                        abilityOf (allOf (and [isSource, non (hasColor .green)])) ])))
                            (target thisCreature)) ],
    power := stat 8, toughness := stat 5 }

/-- Nowhere to Run -/
def nowhereToRunWardLine : StaticSpec :=
  conjunction [ can (by_ (allOf (or [isSpell, abilityHead .anyOnStack]))) (target (allOf (and [isCreature, opponentsControl]))) (asThough := holds (is them (not (hasKeyword hexproof)))),
      cant (allOf (and [abilityHead (.keyword ward), abilityOf (those creature)]))
          trigger ]
theorem okNowhereToRunWardLine : StaticSpec.check (DRS.ofList []) nowhereToRunWardLine = [] := by decide

/-- Mornsong Aria -/
def mornsongAriaLock : StaticSpec :=
  conjunction [cant (bare isPlayer) (draw anyAmount), cant (bare isPlayer)
                 (gainLife anyAmount)]
theorem okMornsongAriaLock : StaticSpec.check (DRS.ofList []) mornsongAriaLock = [] := by decide

def brainwash : Spelled := spelled <| singleFacedCard
  { name := "Brainwash", cost := [pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (gatedBy [generic 3] (enchanted creature) attack) ] }

def enkiraHostileScavenger : Ability :=
  static (onlyWhile (must any (block thisCreature))
    (is thisCreature (isAttached (some .equipped))))
theorem okEnkiraHostileScavenger : Ability.check (DRS.ofList []) enkiraHostileScavenger = [] := by decide

def anglerTurtle : Ability :=
  static (must (allOf (and [isCreature, opponentsControl])) attack)
theorem okAnglerTurtle : Ability.check (DRS.ofList []) anglerTurtle = [] := by decide

def ensnaringBridge : Spelled := spelled <| singleFacedCard
  { name := "Ensnaring Bridge", cost := [generic 3], types := [.artifact],
    text :=
      [ static (cant (allOf (and [ isCreature,
                         hasPower .greater (countOf (cardIn yourHand)) ])) attack) ] }

def undercoverButler : Spelled := spelled <| singleFacedCard
  { name := "Undercover Butler", cost := [generic 2, hybridPip .blue .black],
    types := [.creature], subtypes := [human, rogue],
    text :=
      [ whenever
          (does thisCreature
             (attack
                (some
                   (the (and [isPlayer, superlative .max (.playerStat .lifeTotal) isPlayer])))))
          (establishThroughout (cant any (block it)) thisTurn) ],
    power := stat 2, toughness := stat 3 }

def aetherTunnel : Spelled := spelled <| singleFacedCard
  { name := "Aether Tunnel", cost := [generic 1, pip .blue], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (conjunction [ getsPt (enchanted creature) (up 1) (up 0),
            cant any (block it) ]) ] }

def excruciator : Spelled := spelled <| singleFacedCard
  { name := "Excruciator", cost := [generic 6, pip .red, pip .red], types := [.creature],
    subtypes := [avatar],
    text :=
      [ static
          (cant any
            (preventDamage allDamage
              (damage := would (by_ thisCreature) (dealDamage anyAmount)))) ],
    power := stat 7, toughness := stat 7 }

def flaringPain : Spelled := spelled <| singleFacedCard
  { name := "Flaring Pain", cost := [generic 1, pip .red], types := [.instant],
    text :=
      [ spell (establishThroughout
            (cant any (preventDamage allDamage (damage := would any (dealDamage anyAmount))))
            thisTurn),
        flashback [pip .red] ] }

def gideonJura : Spelled := spelled <| singleFacedCard
  { name := "Gideon Jura", cost := [generic 3, pip .white, pip .white],
    supertypes := [.legendary], types := [.planeswalker], subtypes := [gideon],
    text :=
      [ activated (loyaltySymbol (up 2))
          (establishThroughout
            (must (allOf (and [isCreature, controlledBy (target isOpponent)]))
               (attack thisPlaneswalker))
            (duringNextTurnOf (that player))),
        activated (loyaltySymbol (down 2)) (destroy (target (and [isCreature, isTapped]))),
        activated (loyaltySymbol (up 0))
          (sequentially
            [ establishThroughout (qualityChange thisPlaneswalker .sets
                  (bundle
                    { characteristics :=
                      { types := [.creature],
                        subtypes := [human, soldier],
                        power := stat 6, toughness := stat 6 } }
                    (some .planeswalker))) untilEndOfTurn,
              preventAll it thisTurn ]) ],
    loyalty := stat 6 }

def pinpointAvalanche : Spelled := spelled <| singleFacedCard
  { name := "Pinpoint Avalanche", cost := [generic 3, pip .red, pip .red],
    types := [.instant],
    text :=
      [ spell (sequentially
          [ instruct this_ (dealDamage 4 (target isCreature)),
            establish (cant any (preventDamage allDamage)) (some modifiesPrecedingAction) ]) ] }

def whippoorwillImmunity : Action :=
  sequentially
    [ establishThroughout (cant any (regeneration (target isCreature))) thisTurn,
      establishThroughout
        (conjunction
          [ cant any
              (preventDamage allDamage
                (damage := would any (dealDamage anyAmount (that creature)))),
            cant any
              (redirectDamage allDamage (a (and [or [isPermanent, isPlayer], isOther]))
                (damage := would any (dealDamage anyAmount (that creature)))) ])
        thisTurn ]
theorem okWhippoorwillImmunity : Action.check (DRS.ofList []) whippoorwillImmunity = [] := by decide

def callInAProfessional : Spelled := spelled <| singleFacedCard
  { name := "Call In a Professional", cost := [generic 2, pip .red], types := [.instant],
    text :=
      [ spell (sequentially
          [ establishThroughout (cant (bare isPlayer) (gainLife anyAmount)) thisTurn,
            establishThroughout
              (cant any (preventDamage allDamage (damage := would any (dealDamage anyAmount))))
              thisTurn,
            instruct this_ (dealDamage 3 (target isAnyTarget)) ]) ] }

/-- Council of the Absolute -/
def councilOfTheAbsolute : Spelled := spelled <| singleFacedCard
  { name := "Council of the Absolute", cost := [generic 2, pip .white, pip .blue],
    types := [.creature], subtypes := [human, advisor],
    text :=
      [ static (entersChoosingFrom thisCreature .cardName
          (nameOfCard (not (or [isCreature, isLand])))),
        static (cant yourOpponents (cast (allOf (and [isSpell, named chosenName])))),
        static (costShift (allOf (and [isSpell, named chosenName, did you (cast which) thisObject]))
          (less 2)) ],
    power := stat 2, toughness := stat 4 }

/-- Failure // Comply -/
def complyNameLock : Action :=
  sequentially
    [ choose (a (quality .cardName)),
      establishThroughout (cant yourOpponents
                             (cast (allOf (and [isSpell, named chosenName])))) untilYourNextTurn ]
theorem okComplyNameLock : Action.check (DRS.ofList []) complyNameLock = [] := by decide

/-- Gideon's Intervention -/
def gideonsIntervention : Spelled := spelled <| singleFacedCard
  { name := "Gideon's Intervention", cost := [generic 2, pip .white, pip .white],
    types := [.enchantment],
    text :=
      [ static (entersChoosing thisEnchantment .cardName),
        static (cant yourOpponents (cast (allOf (and [isSpell, named chosenName])))),
        static
          (replace
            (would (by_ (allOf (and [isSource, named chosenName])))
              (dealDamage anyAmount (youAnd (allOf (and [isPermanent, youControl])))))
            (instead (preventDamage allDamage))) ] }

/-- Academic Probation -/
def academicProbationNameMode : Action :=
  sequentially
    [ choose (a (qualityFrom .cardName (nameOfCard (non isLand)))),
      establishThroughout (cant yourOpponents
                             (cast (allOf (and [isSpell, named chosenName])))) untilYourNextTurn ]
theorem okAcademicProbationNameMode : Action.check (DRS.ofList []) academicProbationNameMode = [] := by
  decide

def fatigue : Spelled := spelled <| singleFacedCard
  { name := "Fatigue", cost := [generic 1, pip .blue], types := [.sorcery],
    text := [spell (instruct (target isPlayer) (skipPart .drawStep 1))] }

def meditate : Spelled := spelled <| singleFacedCard
  { name := "Meditate", cost := [generic 2, pip .blue], types := [.instant],
    text := [spell (sequentially [draw 4, skipPart .turn 1])] }

/-- Blinding Angel {3}{W}{W} — Creature — Angel 2/4. "Flying. Whenever Blinding Angel deals
combat damage to a player, that player skips their next combat phase." -/
def blindingAngel : Spelled := spelled <| singleFacedCard
  { name := "Blinding Angel", cost := [generic 3, pip .white, pip .white],
    types := [.creature], subtypes := [angel],
    text :=
      [ flying,
        whenever
          (does thisCreature (dealDamage (to := a isPlayer) (kind := some .combat)))
          (instruct (that player) (skipPart .combat 1)) ],
    power := stat 2, toughness := stat 4 }

def eonHub : Spelled := spelled <| singleFacedCard
  { name := "Eon Hub", cost := [generic 5], types := [.artifact],
    text := [static (partSkip (bare isPlayer) .upkeep)] }

def stasis : Spelled := spelled <| singleFacedCard
  { name := "Stasis", cost := [generic 1, pip .blue], types := [.enchantment],
    text :=
      [ static (partSkip (bare isPlayer) .untapStep),
        at_ (beginningOfYour .upkeep)
          (doUnless (sacrifice thisEnchantment) [pip .blue]) ] }

def yawgmothsBargain : Spelled := spelled <| singleFacedCard
  { name := "Yawgmoth's Bargain", cost := [generic 4, pip .black, pip .black],
    types := [.enchantment],
    text :=
      [ static (partSkip you .drawStep),
        activated (payLife 1) (draw 1) ] }

def sandsOfTimeSkip : StaticSpec := partSkip eachPlayer .untapStep
theorem okSandsOfTimeSkip : StaticSpec.check (DRS.ofList []) sandsOfTimeSkip = [] := by decide

/-- Wormfang Manta {5}{U}{U} — Creature — Nightmare Fish Beast 6/1. "Flying. When Wormfang
Manta enters, you skip your next turn. When Wormfang Manta leaves the battlefield, you take an
extra turn after this one." -/
def wormfangManta : Spelled := spelled <| singleFacedCard
  { name := "Wormfang Manta", cost := [generic 5, pip .blue, pip .blue],
    types := [.creature],
    subtypes := [nightmare, fish, beast],
    text :=
      [ flying,
        when (does thisCreature enter) (skipPart .turn 1),
        when (does thisCreature leave) (extraTurn 1) ],
    power := stat 6, toughness := stat 1 }

def eaterOfDaysSkip : Action := skipPart .turn 2
theorem okEaterOfDaysSkip : Action.check (DRS.ofList []) eaterOfDaysSkip = [] := by decide

/-- Empty City Ruse -/
def emptyCityRuse : Spelled := spelled <| singleFacedCard
  { name := "Empty City Ruse", cost := [pip .white], types := [.sorcery],
    text :=
      [ spell (establishThroughout (partSkip (target isOpponent) .combat) (duringNextTurnOf (that
              .player))) ] }

/-- False Peace -/
def falsePeace : Spelled := spelled <| singleFacedCard
  { name := "False Peace", cost := [pip .white], types := [.sorcery],
    text :=
      [ spell (establishThroughout (partSkip (target isPlayer) .combat) (duringNextTurnOf (that
              .player))) ] }

/-- Battlefront Krushok -/
def battlefrontKrushokEvasion : Ability :=
  static (cant (by_ (counted (moreThan 1) isCreature)) (block thisCreature))
theorem okBattlefrontKrushokEvasion : Ability.check (DRS.ofList []) battlefrontKrushokEvasion = [] := by decide

/-- Grand Abolisher -/
def grandAbolisher : Spelled := spelled <| singleFacedCard
  { name := "Grand Abolisher", cost := [pip .white, pip .white], types := [.creature],
    subtypes := [human, cleric],
    text :=
      [ static (partScope .turn you
          (conjunction [ cant yourOpponents (cast (allOf isSpell)),
              cant yourOpponents (activate (allOf (and [ abilityHead .anyActivated,
                               abilityOf (allOf (or [isArtifact, isCreature, isEnchantment])) ]))) ])) ],
    power := stat 2, toughness := stat 2 }

/-- Festival -/
def festival : Spelled := spelled <| singleFacedCard
  { name := "Festival", cost := [pip .white], types := [.instant],
    text :=
      [ spell (establishThroughout (cant (allOf isCreature) attack) thisTurn)
          (timing := duringPart .upkeep (some anOpponent)) ] }

def demotion : Spelled := spelled <| singleFacedCard
  { name := "Demotion", cost := [pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (conjunction [ cant (enchanted creature) block,
            cant any (activate (allOf (and [abilityHead .anyActivated, abilityOf it]))) ]) ] }

def terror : Spelled := spelled <| singleFacedCard
  { name := "Terror", cost := [generic 1, pip .black], types := [.instant],
    text :=
      [ spell (destroyCantRegenerate
          (target (and [isCreature, non isArtifact, non (hasColor .black)]))) ] }

def snuffOut : Spelled := spelled <| singleFacedCard
  { name := "Snuff Out", cost := [generic 3, pip .black], types := [.instant],
    text :=
      [ static (onlyWhile (altCost this_ (payLife 4))
          (exists_ (and [isLand, swamp, youControl]))),
        spell (destroyCantRegenerate (target (and [isCreature, non (hasColor .black)]))) ] }

def wrathOfGod : Spelled := spelled <| singleFacedCard
  { name := "Wrath of God", cost := [generic 2, pip .white, pip .white],
    types := [.sorcery],
    text := [spell (destroyCantRegenerate (allOf isCreature))] }

def damnation : Spelled := spelled <| singleFacedCard
  { name := "Damnation", cost := [generic 2, pip .black, pip .black], types := [.sorcery],
    text := [spell (destroyCantRegenerate (allOf isCreature))] }

def glacialChasm : Spelled := spelled <| singleFacedCard
  { name := "Glacial Chasm", types := [.land],
    text :=
      [ cumulativeUpkeep (payLife 2),
        when (does thisLand enter) (sacrifice (a isLand)),
        static (cant (allOf (and [isCreature, youControl])) attack),
        static
          (replace (would any (dealDamage anyAmount you)) (instead (preventDamage allDamage))) ] }

/-- Peacekeeper -/
def peacekeeperCant : Ability :=
  static (cant (allOf isCreature) attack)
theorem okPeacekeeperCant : Ability.check (DRS.ofList []) peacekeeperCant = [] := by decide

/-- Culling Mark -/
def cullingMark : Spelled := spelled <| singleFacedCard
  { name := "Culling Mark", cost := [generic 2, pip .green], types := [.sorcery],
    text :=
      [ spell (establishThroughout (must (target isCreature) block) thisTurn) ] }

/-- Blazing Archon -/
def blazingArchonCant : Ability :=
  static (cant (allOf isCreature) (attack you))
theorem okBlazingArchonCant : Ability.check (DRS.ofList []) blazingArchonCant = [] := by decide

/-- Glaring Spotlight -/
def glaringSpotlight : Spelled := spelled <| singleFacedCard
  { name := "Glaring Spotlight", cost := [generic 1], types := [.artifact],
    text :=
      [ static
          (can (by_ (allOf (and [or [isSpell, abilityHead .anyOnStack], youControl])))
             (target (allOf (and [ isCreature, opponentsControl,
                         hasKeyword hexproof ]))) (asThough := holds (is them (not (hasKeyword hexproof))))),
        activated (compound [[generic 3], sacrifice thisArtifact])
          (sequentially
            [ gain (allOf (and [isCreature, youControl])) hexproof untilEndOfTurn,
              establishThroughout (cant any
                                     (block
                                        (allOf (and [isCreature, youControl])))) thisTurn ]) ] }

/-- Canoptek Wraith -/
def canoptekWraith : Ability :=
  flavorWord "Wraith Form"
    (static (cant any (block thisCreature)))
theorem okCanoptekWraith : Ability.check (DRS.ofList []) canoptekWraith = [] := by decide

/-- Gadrak, the Crown-Scourge -/
def gadrakCantAttack : Ability :=
  static (onlyUnless (cant thisCreature attack)
    (compareAmt (countOf (and [isArtifact, youControl])) .atLeast 4))
theorem okGadrakCantAttack : Ability.check (DRS.ofList []) gadrakCantAttack = [] := by decide

/-- Berserker's Frenzy, the 1—14 striation -/
def berserkersFrenzyLowRoll : Action :=
  sequentially
    [ choose (counted anyNumber isCreature),
      establishThroughout (must them block) thisTurn ]
theorem okBerserkersFrenzyLowRoll : Action.check (DRS.ofList []) berserkersFrenzyLowRoll = [] := by decide

/-- Damn -/
def damnDestroyLine : Action :=
  doUnderProhibition (destroy (target isCreature))
    (regeneration (a (and [isCreature, was (destroy which) thisWay])))
theorem okDamnDestroyLine : Action.check (DRS.ofList []) damnDestroyLine = [] := by decide

def nekrataalWhole : Spelled := spelled <| singleFacedCard
  { name := "Nekrataal", cost := [generic 2, pip .black, pip .black], types := [.creature],
    subtypes := [human, assassin],
    text :=
      [ firstStrike,
        when (does thisCreature enter)
          (doUnderProhibition (destroy (target (and [isCreature, non isArtifact, not (hasColor
              .black)])))
            (regeneration (that creature))) ],
    power := stat 2, toughness := stat 1 }

/-- Concussive Bolt, both paragraphs -/
def concussiveBolt : Action :=
  sequentially
    [ instruct this_ (dealDamage 4 targetPlayerOrPlaneswalker),
      doIf (compareAmt (countOf (and [isArtifact, youControl])) .atLeast 3)
        (establishThroughout
           (cant (allOf (and [isCreature, controlledBy splitOverPlaneswalker]))
              block) thisTurn) ]
theorem okConcussiveBolt : Action.check (DRS.ofList []) concussiveBolt = [] := by decide

/-- Ulamog's Crusher -/
def ulamogsCrusher : Spelled := spelled <| singleFacedCard
  { name := "Ulamog's Crusher", cost := [generic 8], types := [.creature],
    subtypes := [eldrazi],
    text :=
      [ annihilator 2,
        static (must thisCreature attack) ],
    power := stat 8, toughness := stat 8 }

/-- "attack you unless their controller pays {2} for each" -/
def attackToll : StaticSpec :=
  gatedBy (scaledGenericMana
      (forEach 2
         (and [isCreature, controlledBy they, inCombat .attackerOf you]))) (actor
                                                                                     (allOf
                                                                                        isCreature)) (attack you)

/-- Propaganda -/
def propaganda : Spelled := spelled <| singleFacedCard
  { name := "Propaganda", cost := [generic 2, pip .blue], types := [.enchantment],
    text := [static attackToll] }

/-- Ghostly Prison -/
def ghostlyPrisonWhole : Spelled := spelled <| singleFacedCard
  { name := "Ghostly Prison", cost := [generic 2, pip .white], types := [.enchantment],
    text := [static attackToll] }

/-- Archangel of Tithes -/
def archangelOfTithesBlockToll : Ability :=
  static (onlyWhile
    (gatedBy (scaledGenericMana (times 1 groupSize)) (allOf isCreature) block)
    (is thisCreature isAttacking))
theorem okArchangelOfTithesBlockToll : Ability.check (DRS.ofList []) archangelOfTithesBlockToll = [] := by decide
/-- Archangel of Tithes -/
def archangelOfTithesAttackToll : Ability :=
  static (onlyWhile
    (gatedBy (scaledGenericMana (times 1 groupSize)) (allOf isCreature)
       (attack (youOr (allOf (and [isPlaneswalker, youControl])))))
    (is thisCreature isUntapped))
theorem okArchangelOfTithesAttackToll : Ability.check (DRS.ofList []) archangelOfTithesAttackToll = [] := by
  decide

/-- Archon of Absolution -/
def archonOfAbsolution : Spelled := spelled <| singleFacedCard
  { name := "Archon of Absolution", cost := [generic 3, pip .white], types := [.creature],
    subtypes := [archon],
    text :=
      [ flying,
        protection (hasColor .white),
        static (gatedBy (scaledGenericMana (times 1 groupSize)) (allOf isCreature)
                  (attack (youOr (allOf (and [isPlaneswalker, youControl]))))) ],
    power := stat 3, toughness := stat 2 }

/-- Myr Prototype -/
def myrPrototype : Spelled := spelled <| singleFacedCard
  { name := "Myr Prototype", cost := [generic 5], types := [.artifact, .creature],
    subtypes := [myr],
    text :=
      [ at_ (beginningOfYour .upkeep)
          (putCounters 1 p1p1Counter thisCreature),
        static
            (conjunction
               [gatedBy (scaledGenericMana (times 1 (countersOn p1p1Counter it)))
                  thisCreature attack, gatedBy
            (scaledGenericMana (times 1 (countersOn p1p1Counter it))) thisCreature
            block]) ],
    power := stat 2, toughness := stat 2 }

/-- Heat Wave -/
def heatWave : Spelled := spelled <| singleFacedCard
  { name := "Heat Wave", cost := [generic 2, pip .red], types := [.enchantment],
    text :=
      [ cumulativeUpkeep [pip .red],
        static
            (cant (allOf (and [isCreature, hasColor .blue]))
               (block (allOf (and [isCreature, youControl])))),
        static (gatedBy (instruct they (loseLife
            (forEach 1 (and [isCreature, isBlocking, controlledBy they])))) (actor
                                                                                       (allOf
                                                                                          (and
                                                                                             [isCreature, non (hasColor .blue)]))) (block (allOf (and [isCreature, youControl])))) ] }

/-- Awesome Presence -/
def awesomePresence : Spelled := spelled <| singleFacedCard
  { name := "Awesome Presence", cost := [pip .blue], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (gatedBy (scaledGenericMana (forEach 3 (and [ isCreature, controlledBy they,
                             inCombat .blockerOf it ]))) any (block
                                                                       (enchanted
                                                                          (.type
                                                                             .creature)))) ] }

/-- Oppressive Rays -/
def oppressiveRays : Spelled := spelled <| singleFacedCard
  { name := "Oppressive Rays", cost := [pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (conjunction
                  [gatedBy [generic 3] (enchanted creature) attack,
                   gatedBy [generic 3] (enchanted creature) block]),
        static (costShift
          (allOf (and [ abilityHead .anyActivated,
                         abilityOf (enchanted creature) ]))
          (more 3)) ] }

/-- Distortion Strike -/
def distortionStrikeLine : Action :=
  sequentially
    [ get (target isCreature) (up 1) (up 0) untilEndOfTurn,
      establishThroughout (cant any (block (that creature))) thisTurn ]
theorem okDistortionStrikeLine : Action.check (DRS.ofList []) distortionStrikeLine = [] := by decide

/-- Retro-Mutation -/
def retroMutation : Spelled := spelled <| singleFacedCard
  { name := "Retro-Mutation", cost := [generic 2, pip .blue], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ flash,
        enchant isCreature,
        static (conjunction [ qualityChange (enchanted creature) .sets
              (bundle { characteristics := { subtypes := [turtle] } }),
            getsBase it 0 1,
            cant it attack,
            allAbilityLoss it ]) ] }

/-- Hotshot Mechanic -/
def hotshotMechanic : Spelled := spelled <| singleFacedCard
  { name := "Hotshot Mechanic", cost := [pip .white], types := [.artifact, .creature],
    subtypes := [fox, pilot],
    text :=
      [ static (can thisCreature (crew (allOf (hasSubtype
      vehicle))) (asThough := greaterBy .power 2)) ],
    power := stat 2, toughness := stat 1 }

/-- Cloudspire Captain -/
def cloudspireCaptainCrewLine : StaticSpec :=
  conjunction [ can thisCreature (saddle (allOf (hasSubtype
          mount))) (asThough := greaterBy .power 2),
      can thisCreature (crew (allOf (hasSubtype
          vehicle))) (asThough := greaterBy .power 2) ]
theorem okCloudspireCaptainCrewLine : StaticSpec.check (DRS.ofList []) cloudspireCaptainCrewLine = [] := by
  decide

/-- Revoke Privileges -/
def revokePrivileges : Spelled := spelled <| singleFacedCard
  { name := "Revoke Privileges", cost := [generic 2, pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (conjunction [ cant (enchanted creature) attack,
      cant (enchanted creature) block,
      cant (enchanted creature) (crew (allOf (hasSubtype
          vehicle))) ]) ] }

/-- Harried Spearguard -/
def harriedSpearguard : Spelled := spelled <| singleFacedCard
  { name := "Harried Spearguard", cost := [pip .red], types := [.creature],
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
    power := stat 1, toughness := stat 1 }

/-- Arrest -/
def arrest : Spelled := spelled <| singleFacedCard
  { name := "Arrest", cost := [generic 2, pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (conjunction
          [ cant (enchanted creature) attack,
            cant (enchanted creature) block,
            cant any (activate (allOf (and [abilityHead .anyActivated, abilityOf it]))) ]) ] }

/-- Conqueror's Flail -/
def conquerorsFlailProhibition : StaticSpec :=
  onlyWhile
    (partScope .turn you
      (cant yourOpponents (cast (allOf isSpell))))
    (is this_ (attachedTo (a isCreature)))
theorem okConquerorsFlailProhibition : StaticSpec.check (DRS.ofList []) conquerorsFlailProhibition = [] := by
  decide

/-- Retrace's permission: the land card discarded is an additional cost of casting, not what is
cast, added to the mana cost or an alternative cost [CR#702.81a,601.2f,601.2h]. -/
def castFromGraveyardByDiscardingALand : StaticSpec :=
  can you
    (cast this_ (from_ := yourGraveyard)
      (payment := plusAdditional (discard (a (and [isCard, isLand])))))
theorem okCastFromGraveyardByDiscardingALand :
    StaticSpec.check (DRS.ofList []) castFromGraveyardByDiscardingALand = [] := by decide

/-- "Auras already attached to those artifacts" -/
def aurasAlreadyAttachedToThoseArtifacts : NounPhrase :=
  allOf (and [aura, attachedTo (those artifact)])

/-- Guardian Beast's protection: "As long as this creature is untapped, noncreature artifacts you
control can't be enchanted, they have indestructible, and other players can't gain control of
them. This effect doesn't remove Auras already attached to those artifacts." The artifacts are the
subject the three clauses share, and "can't gain control of" forbids the control change,
`gainControlOf`. The exemption is the "can't be enchanted" clause's alone: that clause makes an
Aura already on such an artifact illegal [CR#303.4c,704.5m], and "already attached" reads the
host back, as *Benevolent Blessing*'s does. -/
def guardianBeastProtection : StaticSpec :=
  onlyWhile
    (conjunction
      [ retention (cant any (attachTo (allOf aura) them)) aurasAlreadyAttachedToThoseArtifacts,
        abilityGrant them indestructible,
        cant (bare (and [isPlayer, isOther])) (gainControlOf them) ]
      (bare (and [non isCreature, isArtifact, youControl])))
    (is thisCreature isUntapped)

/-- Guardian Beast -/
def guardianBeast : Spelled := spelled <| singleFacedCard
  { name := "Guardian Beast", cost := [generic 3, pip .black], types := [.creature],
    subtypes := [beast],
    text :=
      [ static guardianBeastProtection ],
    power := stat 2, toughness := stat 4 }
theorem okGuardianBeast : guardianBeast.card.check = [] := guardianBeast.ok

/-- The exemption reaches "can't be enchanted" and not "have indestructible" or "can't gain
control of" (owner ruling 2026-10-22, 3b): it wraps that clause in the card, and wrapped around
the whole conjunction it is refused, a conjunction being no one clause that removes Attachments
and having no one host for the Auras to be attached to. -/
theorem guardianBeastExemptsOnlyTheEnchantProhibition :
    (match guardianBeastProtection with
     | .conditional (.conjunction _ [.retention (.cant .any _ _) _, .characteristicChange _ _,
         .cant _ _ _]) _ _ => true
     | _ => false) = true ∧
    StaticSpec.check (DRS.ofList [])
      (onlyWhile
        (retention
          (conjunction
            [ cant any (attachTo (allOf aura) them),
              abilityGrant them indestructible,
              cant (bare (and [isPlayer, isOther])) (gainControlOf them) ]
            (bare (and [non isCreature, isArtifact, youControl])))
          aurasAlreadyAttachedToThoseArtifacts)
        (is thisCreature isUntapped)) = [.retentionClause, .retentionSubject] := by decide

end Semantics.Cards

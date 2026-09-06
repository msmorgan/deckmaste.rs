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

def infiltrate : Instruction := forbidBeingBlocked (target creature) (some Primitives.Duration.thisTurn)
theorem okInfiltrate : Instruction.check [] infiltrate = [] := by decide
def changeOfHeart : Instruction := forbidAttack (target creature) (some Primitives.Duration.thisTurn)
theorem okChangeOfHeart : Instruction.check [] changeOfHeart = [] := by decide
def blindblast : Instruction :=
  Primitives.Instruction.sequentially
    [Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 1) (target creature), forbidBlock (that (.type .creature)) (some
        Primitives.Duration.thisTurn)]
theorem okBlindblast : Instruction.check [] blindblast = [] := by decide
def blindingFlare : Instruction :=
  forbidBlock (Primitives.NounPhrase.described (Primitives.DetPhrase.target anyNumber) creature) (some Primitives.Duration.thisTurn)
theorem okBlindingFlare : Instruction.check [] blindingFlare = [] := by decide
def cowardKiller : Instruction :=
  Primitives.Instruction.sequentially
    [ forbidBlock (target creature) (some Primitives.Duration.thisTurn),
      become (that (.type .creature)) { characteristics := { subtypes := [creatureType "Coward"] } }
        (some untilEndOfTurn) ]
theorem okCowardKiller : Instruction.check [] cowardKiller = [] := by decide

/-- Auriok Siege Sled -/
def auriokSiegeSledDenial : Ability :=
  activated (Primitives.Cost.mana [generic 1])
    (Primitives.Instruction.establish (cantDoTo (.core .block) (target (Primitives.Predicate.and [artifact, creature])) thisCreature)
      (some Primitives.Duration.thisTurn))
theorem okAuriokSiegeSledDenial : Ability.check [] auriokSiegeSledDenial = [] := by decide

/-- Blindblast -/
def blindblastWhole : Instruction :=
  Primitives.Instruction.sequentially
    [ Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 1) (target creature), forbidBlock (that (.type .creature)) (some
        Primitives.Duration.thisTurn),
      Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you) ]
theorem okBlindblastWhole : Instruction.check [] blindblastWhole = [] := by decide

def sparkmagesGambit : Instruction :=
  Primitives.Instruction.sequentially
    [ Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 1) (Primitives.NounPhrase.eachOf (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 2)) creature)),
      forbidBlock (those (.type .creature)) (some Primitives.Duration.thisTurn) ]
theorem okSparkmagesGambit : Instruction.check [] sparkmagesGambit = [] := by decide

/-- Glacial Chasm -/
def glacialChasmCant : Ability :=
  Primitives.Ability.static (deontic (allOf creatureYouControl) Primitives.Compulsion.forbid [.core .attack] .agent Primitives.DeonticPatient.noPatient)
theorem okGlacialChasmCant : Ability.check [] glacialChasmCant = [] := by decide

def desperateCastaways : Ability :=
  Primitives.Ability.static (onlyUnless (deontic thisCreature Primitives.Compulsion.forbid [.core .attack] .agent Primitives.DeonticPatient.noPatient)
    (exists_ (Primitives.Predicate.and [artifact, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))
theorem okDesperateCastaways : Ability.check [] desperateCastaways = [] := by decide

def munghaWurm : Ability := Primitives.Ability.static (cantMoreThan Primitives.NounPhrase.you (.action "Untap") 1 land)
theorem okMunghaWurm : Ability.check [] munghaWurm = [] := by decide
def dampingField : Ability :=
  Primitives.Ability.static (cantMoreThan (Primitives.NounPhrase.playerGroup .allPlayers) (.action "Untap") 1 artifact)
theorem okDampingField : Ability.check [] dampingField = [] := by decide
def smoke : Ability := Primitives.Ability.static (cantMoreThan (Primitives.NounPhrase.playerGroup .allPlayers) (.action "Untap") 1 creature)
theorem okSmoke : Ability.check [] smoke = [] := by decide
def winterOrb : Ability :=
  Primitives.Ability.static (onlyWhile (cantMoreThan (Primitives.NounPhrase.playerGroup .allPlayers) (.action "Untap") 1 land)
    (Primitives.Condition.matches thisArtifact untapped))
theorem okWinterOrb : Ability.check [] winterOrb = [] := by decide
def staticOrb : Ability :=
  Primitives.Ability.static (onlyWhile (cantMoreThan (Primitives.NounPhrase.playerGroup .allPlayers) (.action "Untap") 2 permanent)
    (Primitives.Condition.matches thisArtifact untapped))
theorem okStaticOrb : Ability.check [] staticOrb = [] := by decide

/-- Rule of Law -/
def ruleOfLaw : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rule of Law", cost := some [generic 2, pip .white], types := [.enchantment],
      text := [Primitives.Ability.static (cantMoreThan (Primitives.NounPhrase.playerGroup .allPlayers) (.action "Cast") 1 spell)] } }

/-- Spirit of the Labyrinth -/
def spiritOfTheLabyrinth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Spirit of the Labyrinth", cost := some [generic 1, pip .white],
      types := [.enchantment, .creature], subtypes := [creatureType "Spirit"],
      text := [Primitives.Ability.static (cantMoreThan (Primitives.NounPhrase.playerGroup .allPlayers) (.core .draw) 1 Primitives.Predicate.isCard)],
      power := stat 3, toughness := stat 1 } }

def winterMoon : Ability :=
  Primitives.Ability.static (cantMoreThan (Primitives.NounPhrase.playerGroup .allPlayers) (.action "Untap") 1
    (Primitives.Predicate.and [land, Primitives.Predicate.not (Primitives.Predicate.hasSupertype .basic)]))
theorem okWinterMoon : Ability.check [] winterMoon = [] := by decide

def leitmotifComposer : Ability :=
  activated (Primitives.Cost.mana [generic 2, pip .blue])
    (Primitives.Instruction.establish
      (deontic (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.named (Primitives.NameSource.printed "Leitmotif Composer")])) Primitives.Compulsion.forbid
        [.core .block] .patient Primitives.DeonticPatient.noPatient)
      (some Primitives.Duration.thisTurn))
theorem okLeitmotifComposer : Ability.check [] leitmotifComposer = [] := by decide

def berserkersOfBloodRidge : Ability :=
  Primitives.Ability.static (deontic thisCreature Primitives.Compulsion.require [.core .attack] .agent Primitives.DeonticPatient.noPatient)
theorem okBerserkersOfBloodRidge : Ability.check [] berserkersOfBloodRidge = [] := by decide

def trumpetingArmodon : Ability :=
  activated (Primitives.Cost.mana [generic 1, pip .green])
    (Primitives.Instruction.establish
      (deontic (target creature) Primitives.Compulsion.require [.core .block] .agent (Primitives.DeonticPatient.counterpart thisCreature))
      (some Primitives.Duration.thisTurn))
theorem okTrumpetingArmodon : Ability.check [] trumpetingArmodon = [] := by decide

def loathsomeCatoblepas : Ability :=
  activated (Primitives.Cost.mana [generic 2, pip .green])
    (Primitives.Instruction.establish (deontic thisCreature Primitives.Compulsion.require [.core .block] .patient Primitives.DeonticPatient.noPatient)
      (some Primitives.Duration.thisTurn))
theorem okLoathsomeCatoblepas : Ability.check [] loathsomeCatoblepas = [] := by decide

def hipparion : Ability :=
  Primitives.Ability.static (deontic thisCreature (Primitives.Compulsion.gatedBy (Primitives.Cost.mana [generic 1])) [.core .block] .agent
    (Primitives.DeonticPatient.counterpart (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.compare [.stat .power] .atLeast (.lit 3)]))))
theorem okHipparion : Ability.check [] hipparion = [] := by decide

def frodoBaggins : Ability :=
  Primitives.Ability.static (onlyWhile (deontic thisCreature Primitives.Compulsion.require [.core .block] .patient Primitives.DeonticPatient.noPatient)
    (Primitives.Condition.matches thisCreature (Primitives.Predicate.hasDesignation "Ring-bearer" (some Primitives.NounPhrase.you))))
theorem okFrodoBaggins : Ability.check [] frodoBaggins = [] := by decide

def bloodshedFever : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bloodshed Fever", cost := some [pip .red], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          Primitives.Ability.static (deontic (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) Primitives.Compulsion.require [.core .attack]
            .agent Primitives.DeonticPatient.noPatient) ] } }

/-- Pacifism -/
def pacifism : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Pacifism", cost := some [generic 1, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          Primitives.Ability.static (deontic (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) Primitives.Compulsion.forbid
            [.core .attack, .core .block] .agent Primitives.DeonticPatient.noPatient) ] } }

/-- Everybody Lives! -/
def everybodyLivesGateLine : Instruction :=
  Primitives.Instruction.establish
    (deontic (Primitives.NounPhrase.playerGroup .allPlayers) Primitives.Compulsion.forbid [.core .loseGame, .core .winGame] .agent Primitives.DeonticPatient.noPatient)
    (some Primitives.Duration.thisTurn)
theorem okEverybodyLivesGateLine : Instruction.check [] everybodyLivesGateLine = [] := by decide

/-- Gaea's Revenge -/
def gaeasRevenge : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gaea's Revenge", cost := some [generic 5, pip .green, pip .green],
      types := [.creature], subtypes := [creatureType "Elemental"],
      text :=
        [ Primitives.Ability.static (objectCant (.action "Counter") Primitives.NounPhrase.this),
          keyword "Haste",
          Primitives.Ability.static (cantBeTargetedBy thisCreature
            (allOf (Primitives.Predicate.or [ Primitives.Predicate.and [spell, Primitives.Predicate.not (Primitives.Predicate.colorIs .green)],
                          Primitives.Predicate.abilityOf (allOf (Primitives.Predicate.and [source, Primitives.Predicate.not (Primitives.Predicate.colorIs .green)])) ]))) ],
      power := stat 8, toughness := stat 5 } }

/-- Nowhere to Run -/
def nowhereToRunWardLine : StaticSpec :=
  Primitives.StaticSpec.conjunction none
    [ canBeTargetedAsThough (allOf creatureYourOpponentsControl)
        (allOf (Primitives.Predicate.or [spell, Primitives.Predicate.abilityHead .anyOnStack]))
        (Primitives.Predicate.not (Primitives.Predicate.hasKeyword (.the "Hexproof"))),
      deontic
        (allOf (Primitives.Predicate.and [Primitives.Predicate.abilityHead (.keyword "Ward"), Primitives.Predicate.abilityOf (those (.type .creature))]))
        Primitives.Compulsion.forbid [.core .trigger] .agent Primitives.DeonticPatient.noPatient ]
theorem okNowhereToRunWardLine : StaticSpec.check [] nowhereToRunWardLine = [] := by decide

/-- Mornsong Aria -/
def mornsongAriaLock : StaticSpec :=
  deontic (Primitives.NounPhrase.playerGroup .allPlayers) Primitives.Compulsion.forbid [.core .draw, .core .gainLife] .agent Primitives.DeonticPatient.noPatient
theorem okMornsongAriaLock : StaticSpec.check [] mornsongAriaLock = [] := by decide

def brainwash : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Brainwash", cost := some [pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          Primitives.Ability.static (deontic (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (Primitives.Compulsion.gatedBy (Primitives.Cost.mana [generic 3]))
            [.core .attack] .agent Primitives.DeonticPatient.noPatient) ] } }

def enkiraHostileScavenger : Ability :=
  Primitives.Ability.static (onlyWhile (deontic thisCreature Primitives.Compulsion.require [.core .block] .patient Primitives.DeonticPatient.noPatient)
    (Primitives.Condition.matches thisCreature (Primitives.Predicate.isAttached (some .equipped))))
theorem okEnkiraHostileScavenger : Ability.check [] enkiraHostileScavenger = [] := by decide

def anglerTurtle : Ability :=
  Primitives.Ability.static (deontic (allOf creatureYourOpponentsControl) Primitives.Compulsion.require [.core .attack] .agent Primitives.DeonticPatient.noPatient)
theorem okAnglerTurtle : Ability.check [] anglerTurtle = [] := by decide

def ensnaringBridge : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ensnaring Bridge", cost := some [generic 3], types := [.artifact],
      text :=
        [ Primitives.Ability.static (deontic
            (allOf (Primitives.Predicate.and [ creature,
                           Primitives.Predicate.compare [.stat .power] .greater (countOf (Primitives.Predicate.inZone (handOf Primitives.NounPhrase.you))) ]))
            Primitives.Compulsion.forbid [.core .attack] .agent Primitives.DeonticPatient.noPatient) ] } }

def undercoverButler : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Undercover Butler", cost := some [generic 2, hybridPip .blue .black],
      types := [.creature], subtypes := [creatureType "Human", creatureType "Rogue"],
      text :=
        [ whenever
            (attacksPlayer thisCreature
              (the (Primitives.Predicate.and [Primitives.Predicate.anyPlayer, Primitives.Predicate.superlative .max (.playerStat .lifeTotal) Primitives.Predicate.anyPlayer])))
            (Primitives.Instruction.establish (deontic it Primitives.Compulsion.forbid [.core .block] .patient Primitives.DeonticPatient.noPatient)
              (some Primitives.Duration.thisTurn)) ],
      power := stat 2, toughness := stat 3 } }

def aetherTunnel : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aether Tunnel", cost := some [generic 1, pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ Primitives.StaticSpec.modification (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) .power (Primitives.Delta.up (.lit 1)),
              Primitives.StaticSpec.modification (itsOther (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (Primitives.Delta.up (.lit 1)))
                .toughness (Primitives.Delta.up (.lit 0)),
              deontic it Primitives.Compulsion.forbid [.core .block] .patient Primitives.DeonticPatient.noPatient ]) ] } }

def excruciator : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Excruciator", cost := some [generic 6, pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Avatar"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.preventionBan .any (Primitives.Unpreventable.described (Primitives.DamageAgent.dealtBy thisCreature) Primitives.DamageScope.everywhere)
            .noPreventionOnly) ],
      power := stat 7, toughness := stat 7 } }

def flaringPain : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Flaring Pain", cost := some [generic 1, pip .red], types := [.instant],
      text :=
        [ Primitives.Ability.spell none
            (Primitives.Instruction.establish (Primitives.StaticSpec.preventionBan .any (Primitives.Unpreventable.described Primitives.DamageAgent.unattributed Primitives.DamageScope.everywhere)
                .noPreventionOnly)
              (some Primitives.Duration.thisTurn)),
          keywordCosting "Flashback" (Primitives.Cost.mana [pip .red]) ] } }

def gideonJura : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gideon Jura", cost := some [generic 3, pip .white, pip .white],
      supertypes := [.legendary], types := [.planeswalker], subtypes := [planeswalkerType "Gideon"],
      text :=
        [ activated (Primitives.Cost.loyaltySymbol (.up 2))
            (establishThroughout
              (deontic (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller (target Primitives.Predicate.opponent)]))
                Primitives.Compulsion.require [.core .attack] .agent (Primitives.DeonticPatient.defendingPlayer thisPlaneswalker))
              (Primitives.Duration.duringNextTurnOf (that .player))),
          activated (Primitives.Cost.loyaltySymbol (.down 2)) (destroy (target (Primitives.Predicate.and [creature, tapped]))),
          activated (Primitives.Cost.loyaltySymbol .zero)
            (Primitives.Instruction.sequentially
              [ Primitives.Instruction.establish
                  (Primitives.StaticSpec.qualityChange thisPlaneswalker .sets
                    (Primitives.QualityPayload.bundle
                      { characteristics :=
                        { types := [.creature],
                          subtypes := [creatureType "Human", creatureType "Soldier"],
                          power := stat 6, toughness := stat 6 } }
                      (some .planeswalker)))
                  (some untilEndOfTurn),
                preventAll .any (Primitives.DamageScope.toRecipient it) (some Primitives.Duration.thisTurn) ]) ],
      loyalty := stat 6 } }

def pinpointAvalanche : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Pinpoint Avalanche", cost := some [generic 3, pip .red, pip .red],
      types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 4) (target creature),
              Primitives.Instruction.establish (Primitives.StaticSpec.preventionBan .any Primitives.Unpreventable.thatDamage .noPreventionOnly) none ]) ] } }

def whippoorwillImmunity : Instruction :=
  Primitives.Instruction.sequentially
    [ Primitives.Instruction.establish (objectCant (.action "Regenerate") (target creature)) (some Primitives.Duration.thisTurn),
      Primitives.Instruction.establish
        (Primitives.StaticSpec.preventionBan .any
          (Primitives.Unpreventable.described Primitives.DamageAgent.unattributed (Primitives.DamageScope.toRecipient (that (.type .creature)))) .noRedirectEither)
        (some Primitives.Duration.thisTurn) ]
theorem okWhippoorwillImmunity : Instruction.check [] whippoorwillImmunity = [] := by decide

def callInAProfessional : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Call In a Professional", cost := some [generic 2, pip .red], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ Primitives.Instruction.establish (playerCant (.core .gainLife) (Primitives.NounPhrase.playerGroup .allPlayers))
                (some Primitives.Duration.thisTurn),
              Primitives.Instruction.establish
                (Primitives.StaticSpec.preventionBan .any (Primitives.Unpreventable.described Primitives.DamageAgent.unattributed Primitives.DamageScope.everywhere) .noPreventionOnly)
                (some Primitives.Duration.thisTurn),
              Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 3) (target anyTarget) ]) ] } }

/-- Council of the Absolute -/
def councilOfTheAbsolute : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Council of the Absolute", cost := some [generic 2, pip .white, pip .blue],
      types := [.creature], subtypes := [creatureType "Human", creatureType "Advisor"],
      text :=
        [ Primitives.Ability.static (entersChoosingFrom thisCreature .cardName
            (Primitives.ChoiceDomain.nameOfCard (Primitives.Predicate.not (Primitives.Predicate.or [creature, land])))),
          Primitives.Ability.static (cantDoTo (.action "Cast") (Primitives.NounPhrase.playerGroup .yourOpponents)
            (allOf (Primitives.Predicate.and [spell, Primitives.Predicate.named Primitives.NameSource.chosen]))),
          Primitives.Ability.static (Primitives.StaticSpec.costShift (allOf (Primitives.Predicate.and [spell, Primitives.Predicate.named Primitives.NameSource.chosen, castBy Primitives.NounPhrase.you]))
            (Primitives.CostShift.less (.lit 2) none)) ],
      power := stat 2, toughness := stat 4 } }

/-- Failure // Comply -/
def complyNameLock : Instruction :=
  Primitives.Instruction.sequentially
    [ choose (a (quality .cardName)),
      Primitives.Instruction.establish
        (cantDoTo (.action "Cast") (Primitives.NounPhrase.playerGroup .yourOpponents)
          (allOf (Primitives.Predicate.and [spell, Primitives.Predicate.named Primitives.NameSource.chosen])))
        (some untilYourNextTurn) ]
theorem okComplyNameLock : Instruction.check [] complyNameLock = [] := by decide

/-- Gideon's Intervention -/
def gideonsIntervention : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gideon's Intervention", cost := some [generic 2, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ Primitives.Ability.static (entersChoosing thisEnchantment .cardName),
          Primitives.Ability.static (cantDoTo (.action "Cast") (Primitives.NounPhrase.playerGroup .yourOpponents)
            (allOf (Primitives.Predicate.and [spell, Primitives.Predicate.named Primitives.NameSource.chosen]))),
          Primitives.Ability.static (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (allOf (Primitives.Predicate.and [source, Primitives.Predicate.named Primitives.NameSource.chosen])))
            (Primitives.DamageScope.toRecipient (youAnd (allOf (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))))
            (Primitives.DamageOp.prevent Primitives.PreventCut.all none) .repeatedly) ] } }

/-- Academic Probation -/
def academicProbationNameMode : Instruction :=
  Primitives.Instruction.sequentially
    [ choose (a (qualityFrom .cardName (Primitives.ChoiceDomain.nameOfCard (Primitives.Predicate.not land)))),
      Primitives.Instruction.establish
        (cantDoTo (.action "Cast") (Primitives.NounPhrase.playerGroup .yourOpponents)
          (allOf (Primitives.Predicate.and [spell, Primitives.Predicate.named Primitives.NameSource.chosen])))
        (some untilYourNextTurn) ]
theorem okAcademicProbationNameMode : Instruction.check [] academicProbationNameMode = [] := by
  decide

def fatigue : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fatigue", cost := some [generic 1, pip .blue], types := [.sorcery],
      text := [Primitives.Ability.spell none (Primitives.Instruction.skipPart .drawStep (.lit 1) (agent := (target Primitives.Predicate.anyPlayer)))] } }

def meditate : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Meditate", cost := some [generic 2, pip .blue], types := [.instant],
      text := [Primitives.Ability.spell none (Primitives.Instruction.sequentially [Primitives.Instruction.draw (.lit 4) (agent := Primitives.NounPhrase.you), Primitives.Instruction.skipPart .turn (.lit 1)
          (agent := Primitives.NounPhrase.you)])] } }

/-- Blinding Angel {3}{W}{W} — Creature — Angel 2/4. "Flying. Whenever Blinding Angel deals
combat damage to a player, that player skips their next combat phase." -/
def blindingAngel : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blinding Angel", cost := some [generic 3, pip .white, pip .white],
      types := [.creature], subtypes := [creatureType "Angel"],
      text :=
        [ keyword "Flying",
          whenever
            (dealsCombatDamage thisCreature (a Primitives.Predicate.anyPlayer))
            (Primitives.Instruction.skipPart .combat (.lit 1) (agent := (that .player))) ],
      power := stat 2, toughness := stat 4 } }

def eonHub : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Eon Hub", cost := some [generic 5], types := [.artifact],
      text := [Primitives.Ability.static (Primitives.StaticSpec.partSkip (Primitives.NounPhrase.playerGroup .allPlayers) .upkeep)] } }

def stasis : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stasis", cost := some [generic 1, pip .blue], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.partSkip (Primitives.NounPhrase.playerGroup .allPlayers) .untapStep),
          at_ (Primitives.GameEvent.beginningOf .the .upkeep (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
            (doUnless (sacrifice thisEnchantment (agent := Primitives.NounPhrase.you)) (Primitives.Cost.mana [pip .blue]) (agent :=
                Primitives.NounPhrase.you)) ] } }

def yawgmothsBargain : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Yawgmoth's Bargain", cost := some [generic 4, pip .black, pip .black],
      types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.partSkip Primitives.NounPhrase.you .drawStep),
          activated (payLife Primitives.NounPhrase.you 1) (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)) ] } }

def sandsOfTimeSkip : StaticSpec := Primitives.StaticSpec.partSkip (each Primitives.Predicate.anyPlayer) .untapStep
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
          when (Primitives.GameEvent.enters thisCreature none) (Primitives.Instruction.skipPart .turn (.lit 1) (agent := Primitives.NounPhrase.you)),
          when (leavesBattlefield thisCreature) (Primitives.Instruction.addTurn (.lit 1) (agent := Primitives.NounPhrase.you)) ],
      power := stat 6, toughness := stat 1 } }

def eaterOfDaysSkip : Instruction := Primitives.Instruction.skipPart .turn (.lit 2) (agent := Primitives.NounPhrase.you)
theorem okEaterOfDaysSkip : Instruction.check [] eaterOfDaysSkip = [] := by decide

/-- Empty City Ruse -/
def emptyCityRuse : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Empty City Ruse", cost := some [pip .white], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none
            (establishThroughout (Primitives.StaticSpec.partSkip (target Primitives.Predicate.opponent) .combat) (Primitives.Duration.duringNextTurnOf (that
                .player))) ] } }

/-- False Peace -/
def falsePeace : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "False Peace", cost := some [pip .white], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none
            (establishThroughout (Primitives.StaticSpec.partSkip (target Primitives.Predicate.anyPlayer) .combat) (Primitives.Duration.duringNextTurnOf (that
                .player))) ] } }

/-- Battlefront Krushok -/
def battlefrontKrushokEvasion : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.deonticRule thisCreature Primitives.Compulsion.forbid [.core .block] .patient (some (Primitives.CountBound.moreThan (.lit 1)))
    (Primitives.DeonticPatient.counterpart (allOf creature)) none Primitives.DeonticRider.noRider)
theorem okBattlefrontKrushokEvasion : Ability.check [] battlefrontKrushokEvasion = [] := by decide

/-- Grand Abolisher -/
def grandAbolisher : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Grand Abolisher", cost := some [pip .white, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Cleric"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.partScope .turn (some Primitives.NounPhrase.you)
            (Primitives.StaticSpec.conjunction none
              [ cantDoTo (.action "Cast") (Primitives.NounPhrase.playerGroup .yourOpponents) (allOf spell),
                cantDoTo (.action "Activate") (Primitives.NounPhrase.playerGroup .yourOpponents)
                  (allOf (Primitives.Predicate.and [ Primitives.Predicate.abilityHead .anyActivated,
                                 Primitives.Predicate.abilityOf (allOf (Primitives.Predicate.or [artifact, creature, enchantment])) ])) ])) ],
      power := stat 2, toughness := stat 2 } }

/-- Festival -/
def festival : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Festival", cost := some [pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell (some (Primitives.Timing.duringPart .upkeep (some anOpponent)))
            (forbidAttack (allOf creature) (some Primitives.Duration.thisTurn)) ] } }

def demotion : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Demotion", cost := some [pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ deontic (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) Primitives.Compulsion.forbid [.core .block] .agent
                Primitives.DeonticPatient.noPatient,
              objectCant (.action "Activate")
                (allOf (Primitives.Predicate.and [Primitives.Predicate.abilityHead .anyActivated, Primitives.Predicate.abilityOf it])) ]) ] } }

def terror : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Terror", cost := some [generic 1, pip .black], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.doAndForbid
            (destroy (target (Primitives.Predicate.and [creature, Primitives.Predicate.not artifact, Primitives.Predicate.not (Primitives.Predicate.colorIs .black)])))
            (.action "Regenerate") (itVerbed (.action "Destroy"))) ] } }

def snuffOut : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Snuff Out", cost := some [generic 3, pip .black], types := [.instant],
      text :=
        [ Primitives.Ability.static (onlyWhile (Primitives.StaticSpec.altCost Primitives.NounPhrase.this (some (payLife Primitives.NounPhrase.you 4)))
            (exists_ (Primitives.Predicate.and [land, Primitives.Predicate.hasSubtype (landType "Swamp"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))),
          Primitives.Ability.spell none (Primitives.Instruction.doAndForbid
            (destroy (target (Primitives.Predicate.and [creature, Primitives.Predicate.not (Primitives.Predicate.colorIs .black)])))
            (.action "Regenerate") (itVerbed (.action "Destroy"))) ] } }

def wrathOfGod : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Wrath of God", cost := some [generic 2, pip .white, pip .white],
      types := [.sorcery],
      text := [Primitives.Ability.spell none (Primitives.Instruction.doAndForbid (destroy (allOf creature)) (.action "Regenerate") them)] }
          }

def damnation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Damnation", cost := some [generic 2, pip .black, pip .black], types := [.sorcery],
      text := [Primitives.Ability.spell none (Primitives.Instruction.doAndForbid (destroy (allOf creature)) (.action "Regenerate") them)] }
          }

def glacialChasm : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Glacial Chasm", types := [.land],
      text :=
        [ cumulativeUpkeep (payLife Primitives.NounPhrase.you 2),
          when (Primitives.GameEvent.enters thisLand none) (sacrifice (a land) (agent := Primitives.NounPhrase.you)),
          Primitives.Ability.static (deontic (allOf creatureYouControl) Primitives.Compulsion.forbid [.core .attack] .agent Primitives.DeonticPatient.noPatient),
          Primitives.Ability.static (Primitives.StaticSpec.damageRule .any Primitives.DamageAgent.unattributed (Primitives.DamageScope.toRecipient Primitives.NounPhrase.you) (Primitives.DamageOp.prevent Primitives.PreventCut.all none)
            .repeatedly) ] } }

/-- Peacekeeper -/
def peacekeeperCant : Ability :=
  Primitives.Ability.static (deontic (allOf creature) Primitives.Compulsion.forbid [.core .attack] .agent Primitives.DeonticPatient.noPatient)
theorem okPeacekeeperCant : Ability.check [] peacekeeperCant = [] := by decide

/-- Culling Mark -/
def cullingMark : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Culling Mark", cost := some [generic 2, pip .green], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none
            (Primitives.Instruction.establish (deontic (target creature) Primitives.Compulsion.require [.core .block] .agent Primitives.DeonticPatient.noPatient)
              (some Primitives.Duration.thisTurn)) ] } }

/-- Blazing Archon -/
def blazingArchonCant : Ability :=
  Primitives.Ability.static (deontic (allOf creature) Primitives.Compulsion.forbid [.core .attack] .agent (Primitives.DeonticPatient.defendingPlayer Primitives.NounPhrase.you))
theorem okBlazingArchonCant : Ability.check [] blazingArchonCant = [] := by decide

/-- Glaring Spotlight -/
def glaringSpotlight : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Glaring Spotlight", cost := some [generic 1], types := [.artifact],
      text :=
        [ Primitives.Ability.static (canBeTargetedAsThough
            (allOf (Primitives.Predicate.and [ creature, Primitives.Predicate.hasPossessor .controller (Primitives.NounPhrase.playerGroup .yourOpponents),
                           Primitives.Predicate.hasKeyword (.the "Hexproof") ]))
            (allOf (Primitives.Predicate.and [Primitives.Predicate.or [spell, Primitives.Predicate.abilityHead .anyOnStack], Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
            (Primitives.Predicate.not (Primitives.Predicate.hasKeyword (.the "Hexproof")))),
          activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 3], Primitives.Cost.perform (sacrifice thisArtifact (agent :=
              Primitives.NounPhrase.you))])
            (Primitives.Instruction.sequentially
              [ gain (allOf creatureYouControl) (keyword "Hexproof") (some untilEndOfTurn),
                Primitives.Instruction.establish
                  (deontic (allOf creatureYouControl) Primitives.Compulsion.forbid [.core .block] .patient Primitives.DeonticPatient.noPatient)
                  (some Primitives.Duration.thisTurn) ]) ] } }

/-- Canoptek Wraith -/
def canoptekWraith : Ability :=
  flavorWord "Wraith Form"
    (Primitives.Ability.static (Primitives.StaticSpec.deonticRule thisCreature Primitives.Compulsion.forbid [.core .block] .patient none Primitives.DeonticPatient.noPatient none
        Primitives.DeonticRider.noRider))
theorem okCanoptekWraith : Ability.check [] canoptekWraith = [] := by decide

/-- Gadrak, the Crown-Scourge -/
def gadrakCantAttack : Ability :=
  Primitives.Ability.static (onlyUnless (deontic thisCreature Primitives.Compulsion.forbid [.core .attack] .agent Primitives.DeonticPatient.noPatient)
    (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.and [artifact, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .atLeast (.lit 4)))
theorem okGadrakCantAttack : Ability.check [] gadrakCantAttack = [] := by decide

/-- Berserker's Frenzy, the 1—14 striation -/
def berserkersFrenzyLowRoll : Instruction :=
  Primitives.Instruction.sequentially
    [ choose (counted anyNumber creature),
      Primitives.Instruction.establish (deontic them Primitives.Compulsion.require [.core .block] .agent Primitives.DeonticPatient.noPatient) (some Primitives.Duration.thisTurn) ]
theorem okBerserkersFrenzyLowRoll : Instruction.check [] berserkersFrenzyLowRoll = [] := by decide

/-- Damn -/
def damnDestroyLine : Instruction :=
  Primitives.Instruction.doAndForbid (destroy (target creature)) (.action "Regenerate")
    (theVerbed (.action "Destroy") (.type .creature) .thisWay .one)
theorem okDamnDestroyLine : Instruction.check [] damnDestroyLine = [] := by decide

def nekrataalWhole : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Nekrataal", cost := some [generic 2, pip .black, pip .black], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Assassin"],
      text :=
        [ keyword "FirstStrike",
          when (Primitives.GameEvent.enters thisCreature none)
            (Primitives.Instruction.doAndForbid (destroy (target (Primitives.Predicate.and [creature, Primitives.Predicate.not artifact, Primitives.Predicate.not (Primitives.Predicate.colorIs
                .black)])))
              (.action "Regenerate") (that (.type .creature))) ],
      power := stat 2, toughness := stat 1 } }

/-- Concussive Bolt, both paragraphs -/
def concussiveBolt : Instruction :=
  Primitives.Instruction.sequentially
    [ Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 4) targetPlayerOrPlaneswalker,
      Primitives.Instruction.doIf (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.and [artifact, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .atLeast (.lit
          3))
        (Primitives.Instruction.establish
          (deontic (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller splitOverPlaneswalker]))
            Primitives.Compulsion.forbid [.core .block] .agent Primitives.DeonticPatient.noPatient)
          (some Primitives.Duration.thisTurn))
        none ]
theorem okConcussiveBolt : Instruction.check [] concussiveBolt = [] := by decide

/-- Ulamog's Crusher -/
def ulamogsCrusher : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ulamog's Crusher", cost := some [generic 8], types := [.creature],
      subtypes := [creatureType "Eldrazi"],
      text :=
        [ keywordNumber "Annihilator" (.lit 2),
          Primitives.Ability.static (deontic thisCreature Primitives.Compulsion.require [.core .attack] .agent Primitives.DeonticPatient.noPatient) ],
      power := stat 8, toughness := stat 8 } }

/-- "attack you unless their controller pays {2} for each" -/
def attackToll : StaticSpec :=
  deontic (allOf creature)
    (Primitives.Compulsion.gatedBy (scaledMana .generic
      (times (.lit 2)
        (countOf (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller they, Primitives.Predicate.inCombat .attackerOf (some Primitives.NounPhrase.you)])))))
    [.core .attack] .agent (Primitives.DeonticPatient.defendingPlayer Primitives.NounPhrase.you)

/-- Propaganda -/
def propaganda : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Propaganda", cost := some [generic 2, pip .blue], types := [.enchantment],
      text := [Primitives.Ability.static attackToll] } }

/-- Ghostly Prison -/
def ghostlyPrisonWhole : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ghostly Prison", cost := some [generic 2, pip .white], types := [.enchantment],
      text := [Primitives.Ability.static attackToll] } }

/-- Archangel of Tithes -/
def archangelOfTithesBlockToll : Ability :=
  Primitives.Ability.static (onlyWhile
    (deontic (allOf creature) (Primitives.Compulsion.gatedBy (scaledMana .generic (times (.lit 1) Primitives.Amount.groupSize)))
      [.core .block] .agent Primitives.DeonticPatient.noPatient)
    (Primitives.Condition.matches thisCreature attacking))
theorem okArchangelOfTithesBlockToll : Ability.check [] archangelOfTithesBlockToll = [] := by decide
/-- Archangel of Tithes -/
def archangelOfTithesAttackToll : Ability :=
  Primitives.Ability.static (onlyWhile
    (deontic (allOf creature) (Primitives.Compulsion.gatedBy (scaledMana .generic (times (.lit 1) Primitives.Amount.groupSize)))
      [.core .attack] .agent
      (Primitives.DeonticPatient.defendingPlayer
        (youOr (allOf (Primitives.Predicate.and [Primitives.Predicate.hasType .planeswalker, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))))
    (Primitives.Condition.matches thisCreature untapped))
theorem okArchangelOfTithesAttackToll : Ability.check [] archangelOfTithesAttackToll = [] := by
  decide

/-- Archon of Absolution -/
def archonOfAbsolution : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Archon of Absolution", cost := some [generic 3, pip .white], types := [.creature],
      subtypes := [creatureType "Archon"],
      text :=
        [ keyword "Flying",
          keywordQuality "Protection" (Primitives.Predicate.colorIs .white),
          Primitives.Ability.static (deontic (allOf creature) (Primitives.Compulsion.gatedBy (scaledMana .generic (times (.lit 1) Primitives.Amount.groupSize)))
            [.core .attack] .agent
            (Primitives.DeonticPatient.defendingPlayer
              (youOr (allOf (Primitives.Predicate.and [Primitives.Predicate.hasType .planeswalker, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))))) ],
      power := stat 3, toughness := stat 2 } }

/-- Myr Prototype -/
def myrPrototype : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Myr Prototype", cost := some [generic 5], types := [.artifact, .creature],
      subtypes := [creatureType "Myr"],
      text :=
        [ at_ (Primitives.GameEvent.beginningOf .the .upkeep (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
            (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne) thisCreature),
          Primitives.Ability.static (deontic thisCreature
            (Primitives.Compulsion.gatedBy (scaledMana .generic (times (.lit 1) (countersOn plusOnePlusOne it))))
            [.core .attack, .core .block] .agent Primitives.DeonticPatient.noPatient) ],
      power := stat 2, toughness := stat 2 } }

/-- Heat Wave -/
def heatWave : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Heat Wave", cost := some [generic 2, pip .red], types := [.enchantment],
      text :=
        [ cumulativeUpkeep (Primitives.Cost.mana [pip .red]),
          Primitives.Ability.static (deontic (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.colorIs .blue])) Primitives.Compulsion.forbid [.core .block] .agent
            (Primitives.DeonticPatient.counterpart (allOf creatureYouControl))),
          Primitives.Ability.static (deontic (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.not (Primitives.Predicate.colorIs .blue)]))
            (Primitives.Compulsion.gatedBy (Primitives.Cost.perform (loseLife
              (times (.lit 1)
                (countOf (Primitives.Predicate.and [creature, blocking, Primitives.Predicate.hasPossessor .controller they]))) (agent :=
                    they))))
            [.core .block] .agent (Primitives.DeonticPatient.counterpart (allOf creatureYouControl))) ] } }

/-- Awesome Presence -/
def awesomePresence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Awesome Presence", cost := some [pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          Primitives.Ability.static (deontic (Primitives.NounPhrase.attachHost .enchanted (.type .creature))
            (Primitives.Compulsion.gatedBy (scaledMana .generic (times (.lit 3)
              (countOf (Primitives.Predicate.and [ creature, Primitives.Predicate.hasPossessor .controller they,
                               Primitives.Predicate.inCombat .blockerOf (some it) ])))))
            [.core .block] .patient Primitives.DeonticPatient.noPatient) ] } }

/-- Oppressive Rays -/
def oppressiveRays : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Oppressive Rays", cost := some [pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          Primitives.Ability.static (deontic (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (Primitives.Compulsion.gatedBy (Primitives.Cost.mana [generic 3]))
            [.core .attack, .core .block] .agent Primitives.DeonticPatient.noPatient),
          Primitives.Ability.static (Primitives.StaticSpec.costShift
            (allOf (Primitives.Predicate.and [ Primitives.Predicate.abilityHead .anyActivated,
                           Primitives.Predicate.abilityOf (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) ]))
            (Primitives.CostShift.more (.lit 3))) ] } }

/-- Distortion Strike -/
def distortionStrikeLine : Instruction :=
  Primitives.Instruction.sequentially
    [ get (target creature) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 0)) (some untilEndOfTurn),
      Primitives.Instruction.establish (deontic (that (.type .creature)) Primitives.Compulsion.forbid [.core .block] .patient Primitives.DeonticPatient.noPatient)
        (some Primitives.Duration.thisTurn) ]
theorem okDistortionStrikeLine : Instruction.check [] distortionStrikeLine = [] := by decide

/-- Retro-Mutation -/
def retroMutation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Retro-Mutation", cost := some [generic 2, pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keyword "Flash",
          keywordSubject "Enchant" creature,
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ Primitives.StaticSpec.qualityChange (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) .sets
                (Primitives.QualityPayload.bundle { characteristics := { subtypes := [creatureType "Turtle"] } } none),
              Primitives.StaticSpec.modification it .power (Primitives.Delta.set (.lit 0)),
              Primitives.StaticSpec.modification (itsOther it (Primitives.Delta.set (.lit 0))) .toughness (Primitives.Delta.set (.lit 1)),
              Primitives.StaticSpec.deonticRule it Primitives.Compulsion.forbid [.core .attack] .agent none Primitives.DeonticPatient.noPatient none Primitives.DeonticRider.noRider,
              Primitives.StaticSpec.allAbilityLoss it none ]) ] } }

/-- Hotshot Mechanic -/
def hotshotMechanic : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hotshot Mechanic", cost := some [pip .white], types := [.artifact, .creature],
      subtypes := [creatureType "Fox", creatureType "Pilot"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.deonticRule thisCreature Primitives.Compulsion.permit [.ofAbility "Crew"] .agent none
            (Primitives.DeonticPatient.counterpart (allOf (Primitives.Predicate.hasSubtype (artifactType "Vehicle"))))
            (some (Primitives.AsThough.greater .power (.lit 2))) Primitives.DeonticRider.noRider) ],
      power := stat 2, toughness := stat 1 } }

/-- Cloudspire Captain -/
def cloudspireCaptainCrewLine : StaticSpec :=
  Primitives.StaticSpec.deonticRule thisCreature Primitives.Compulsion.permit [.ofAbility "Saddle", .ofAbility "Crew"] .agent none
    (Primitives.DeonticPatient.counterpartsAt
      [ ⟨.ofAbility "Saddle", allOf (Primitives.Predicate.hasSubtype (creatureType "Mount"))⟩,
        ⟨.ofAbility "Crew", allOf (Primitives.Predicate.hasSubtype (artifactType "Vehicle"))⟩ ])
    (some (Primitives.AsThough.greater .power (.lit 2))) Primitives.DeonticRider.noRider
theorem okCloudspireCaptainCrewLine : StaticSpec.check [] cloudspireCaptainCrewLine = [] := by
  decide

/-- Revoke Privileges -/
def revokePrivileges : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Revoke Privileges", cost := some [generic 2, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          Primitives.Ability.static (Primitives.StaticSpec.deonticRule (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) Primitives.Compulsion.forbid
            [.core .attack, .core .block, .ofAbility "Crew"] .agent none
            (Primitives.DeonticPatient.counterpartsAt [⟨.ofAbility "Crew", allOf (Primitives.Predicate.hasSubtype (artifactType "Vehicle"))⟩])
            none Primitives.DeonticRider.noRider) ] } }

/-- Harried Spearguard -/
def harriedSpearguard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Harried Spearguard", cost := some [pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Soldier"],
      text :=
        [ keyword "Haste",
          when (Primitives.GameEvent.dies thisCreature)
            (create (.lit 1)
              { characteristics :=
                { colors := [.black], types := [.creature], subtypes := [creatureType "Rat"],
                  text :=
                    [ Primitives.Ability.static (Primitives.StaticSpec.deonticRule (Primitives.NounPhrase.asMarker .token Primitives.NounPhrase.this) Primitives.Compulsion.forbid [.core .block] .agent
                        none
                        Primitives.DeonticPatient.noPatient none Primitives.DeonticRider.noRider) ],
                  power := stat 1, toughness := stat 1 } }) ],
      power := stat 1, toughness := stat 1 } }

/-- Arrest -/
def arrest : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Arrest", cost := some [generic 2, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ deontic (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) Primitives.Compulsion.forbid
                [.core .attack, .core .block] .agent Primitives.DeonticPatient.noPatient,
              deontic (allOf (Primitives.Predicate.and [Primitives.Predicate.abilityHead .anyActivated, Primitives.Predicate.abilityOf it])) Primitives.Compulsion.forbid
                [.action "Activate"] .patient Primitives.DeonticPatient.noPatient ]) ] } }

/-- Conqueror's Flail -/
def conquerorsFlailProhibition : StaticSpec :=
  onlyWhile
    (Primitives.StaticSpec.partScope .turn (some Primitives.NounPhrase.you)
      (cantDoTo (.action "Cast") (Primitives.NounPhrase.playerGroup .yourOpponents) (allOf spell)))
    (Primitives.Condition.matches Primitives.NounPhrase.this (Primitives.Predicate.attachedTo (a creature)))
theorem okConquerorsFlailProhibition : StaticSpec.check [] conquerorsFlailProhibition = [] := by
  decide

end Semantics.Cards

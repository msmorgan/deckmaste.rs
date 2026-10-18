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

def infiltrate : Action := forbidBeingBlocked (target creature) (some thisTurn)
theorem okInfiltrate : Action.check (DRS.ofList []) infiltrate = [] := by decide
def changeOfHeart : Action := forbidAttack (target creature) (some thisTurn)
theorem okChangeOfHeart : Action.check (DRS.ofList []) changeOfHeart = [] := by decide
def blindblast : Action :=
  Primitives.Action.sequentially
    [instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 1) (target creature) .noncombatOnly), forbidBlock (that (.type .creature)) (some
        thisTurn)]
theorem okBlindblast : Action.check (DRS.ofList []) blindblast = [] := by decide
def blindingFlare : Action :=
  forbidBlock (Primitives.NounPhrase.described (Primitives.DetPhrase.target anyNumber) creature) (some thisTurn)
theorem okBlindingFlare : Action.check (DRS.ofList []) blindingFlare = [] := by decide
def cowardKiller : Action :=
  Primitives.Action.sequentially
    [ forbidBlock (target creature) (some thisTurn),
      become (that (.type .creature)) { characteristics := { subtypes := [creatureType "Coward"] } }
        (some untilEndOfTurn) ]
theorem okCowardKiller : Action.check (DRS.ofList []) cowardKiller = [] := by decide

/-- Auriok Siege Sled -/
def auriokSiegeSledDenial : Ability :=
  activated (Primitives.Cost.mana [generic 1])
    (Primitives.Action.establish (cantDoTo (.core .block) (target (Primitives.Predicate.and [artifact, creature])) thisCreature)
      (some thisTurn))
theorem okAuriokSiegeSledDenial : Ability.check (DRS.ofList []) auriokSiegeSledDenial = [] := by decide

/-- Blindblast -/
def blindblastWhole : Action :=
  Primitives.Action.sequentially
    [ instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 1) (target creature) .noncombatOnly), forbidBlock (that (.type .creature)) (some
        thisTurn),
      Primitives.Action.draw (.lit 1) ]
theorem okBlindblastWhole : Action.check (DRS.ofList []) blindblastWhole = [] := by decide

def sparkmagesGambit : Action :=
  Primitives.Action.sequentially
    [ instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 1) (Primitives.NounPhrase.eachOf (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 2)) creature)) .noncombatOnly),
      forbidBlock (those (.type .creature)) (some thisTurn) ]
theorem okSparkmagesGambit : Action.check (DRS.ofList []) sparkmagesGambit = [] := by decide

/-- Glacial Chasm -/
def glacialChasmCant : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.cant (Primitives.Subject.actor (allOf creatureYouControl)) (Primitives.Action.attack none) {})
theorem okGlacialChasmCant : Ability.check (DRS.ofList []) glacialChasmCant = [] := by decide

def desperateCastaways : Ability :=
  Primitives.Ability.static (onlyUnless (Primitives.StaticSpec.cant (Primitives.Subject.actor thisCreature) (Primitives.Action.attack none) {})
    (exists_ (Primitives.Predicate.and [artifact, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))
theorem okDesperateCastaways : Ability.check (DRS.ofList []) desperateCastaways = [] := by decide

def munghaWurm : Ability := Primitives.Ability.static (cantMoreThan Primitives.NounPhrase.you (.action "untap") 1 land)
theorem okMunghaWurm : Ability.check (DRS.ofList []) munghaWurm = [] := by decide
def dampingField : Ability :=
  Primitives.Ability.static (cantMoreThan (allOf Primitives.Predicate.anyPlayer) (.action "untap") 1 artifact)
theorem okDampingField : Ability.check (DRS.ofList []) dampingField = [] := by decide
def smoke : Ability := Primitives.Ability.static (cantMoreThan (allOf Primitives.Predicate.anyPlayer) (.action "untap") 1 creature)
theorem okSmoke : Ability.check (DRS.ofList []) smoke = [] := by decide
def winterOrb : Ability :=
  Primitives.Ability.static (onlyWhile (cantMoreThan (allOf Primitives.Predicate.anyPlayer) (.action "untap") 1 land)
    (Primitives.Condition.matches thisArtifact untapped))
theorem okWinterOrb : Ability.check (DRS.ofList []) winterOrb = [] := by decide
def staticOrb : Ability :=
  Primitives.Ability.static (onlyWhile (cantMoreThan (allOf Primitives.Predicate.anyPlayer) (.action "untap") 2 permanent)
    (Primitives.Condition.matches thisArtifact untapped))
theorem okStaticOrb : Ability.check (DRS.ofList []) staticOrb = [] := by decide

/-- Rule of Law -/
def ruleOfLaw : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rule of Law", cost := some [generic 2, pip .white], types := [.enchantment],
      text := [Primitives.Ability.static (cantMoreThan (allOf Primitives.Predicate.anyPlayer) (.action "cast") 1 spell)] } }

/-- Spirit of the Labyrinth -/
def spiritOfTheLabyrinth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Spirit of the Labyrinth", cost := some [generic 1, pip .white],
      types := [.enchantment, .creature], subtypes := [creatureType "Spirit"],
      text := [Primitives.Ability.static (cantMoreThan (allOf Primitives.Predicate.anyPlayer) (.core .draw) 1 Primitives.Predicate.isCard)],
      power := stat 3, toughness := stat 1 } }

def winterMoon : Ability :=
  Primitives.Ability.static (cantMoreThan (allOf Primitives.Predicate.anyPlayer) (.action "untap") 1
    (Primitives.Predicate.and [land, Primitives.Predicate.not (Primitives.Predicate.hasSupertype .basic)]))
theorem okWinterMoon : Ability.check (DRS.ofList []) winterMoon = [] := by decide

def leitmotifComposer : Ability :=
  activated (Primitives.Cost.mana [generic 2, pip .blue])
    (Primitives.Action.establish
      (Primitives.StaticSpec.cant Primitives.Subject.any (Primitives.Action.block (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.named (Primitives.NameSource.printed "Leitmotif Composer")]))) {})
      (some thisTurn))
theorem okLeitmotifComposer : Ability.check (DRS.ofList []) leitmotifComposer = [] := by decide

def berserkersOfBloodRidge : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.must (Primitives.Subject.actor thisCreature) (Primitives.Action.attack none) {})
theorem okBerserkersOfBloodRidge : Ability.check (DRS.ofList []) berserkersOfBloodRidge = [] := by decide

def trumpetingArmodon : Ability :=
  activated (Primitives.Cost.mana [generic 1, pip .green])
    (Primitives.Action.establish
      (Primitives.StaticSpec.must (Primitives.Subject.actor (target creature)) (Primitives.Action.block thisCreature) {})
      (some thisTurn))
theorem okTrumpetingArmodon : Ability.check (DRS.ofList []) trumpetingArmodon = [] := by decide

def loathsomeCatoblepas : Ability :=
  activated (Primitives.Cost.mana [generic 2, pip .green])
    (Primitives.Action.establish (Primitives.StaticSpec.must (Primitives.Subject.by (allOf creature)) (Primitives.Action.block thisCreature) {})
      (some thisTurn))
theorem okLoathsomeCatoblepas : Ability.check (DRS.ofList []) loathsomeCatoblepas = [] := by decide

def hipparion : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.gatedBy (Primitives.Cost.mana [generic 1]) (Primitives.Subject.actor thisCreature) (Primitives.Action.block (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.compare [.stat .power] .atLeast (.lit 3)]))) {})
theorem okHipparion : Ability.check (DRS.ofList []) hipparion = [] := by decide

def frodoBaggins : Ability :=
  Primitives.Ability.static (onlyWhile (Primitives.StaticSpec.must (Primitives.Subject.by (allOf creature)) (Primitives.Action.block thisCreature) {})
    (Primitives.Condition.matches thisCreature (Primitives.Predicate.hasDesignation (.named "ringBearer") (some Primitives.NounPhrase.you))))
theorem okFrodoBaggins : Ability.check (DRS.ofList []) frodoBaggins = [] := by decide

def bloodshedFever : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bloodshed Fever", cost := some [pip .red], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") creature,
          Primitives.Ability.static (Primitives.StaticSpec.must (Primitives.Subject.actor (Primitives.NounPhrase.attachHost .enchanted (.type .creature))) (Primitives.Action.attack none) {}) ] } }

/-- Pacifism -/
def pacifism : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Pacifism", cost := some [generic 1, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") creature,
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none [Primitives.StaticSpec.cant (Primitives.Subject.actor (Primitives.NounPhrase.attachHost .enchanted (.type .creature))) (Primitives.Action.attack none) {}, Primitives.StaticSpec.cant (Primitives.Subject.actor (Primitives.NounPhrase.attachHost .enchanted (.type .creature))) (Primitives.Action.block Primitives.NounPhrase.any) {}]) ] } }

/-- Everybody Lives! -/
def everybodyLivesGateLine : Action :=
  Primitives.Action.establish
    (Primitives.StaticSpec.conjunction none [Primitives.StaticSpec.cant (Primitives.Subject.actor (allOf Primitives.Predicate.anyPlayer)) (Primitives.Action.conclude .loseGame) {}, Primitives.StaticSpec.cant (Primitives.Subject.actor (allOf Primitives.Predicate.anyPlayer)) (Primitives.Action.conclude .winGame) {}])
    (some thisTurn)
theorem okEverybodyLivesGateLine : Action.check (DRS.ofList []) everybodyLivesGateLine = [] := by decide

/-- Gaea's Revenge -/
def gaeasRevenge : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gaea's Revenge", cost := some [generic 5, pip .green, pip .green],
      types := [.creature], subtypes := [creatureType "Elemental"],
      text :=
        [ Primitives.Ability.static (objectCant (.action "counter") thisSpell),
          keyword (.named "haste"),
          Primitives.Ability.static (cantBeTargetedBy thisCreature
            (allOf (Primitives.Predicate.or [ Primitives.Predicate.and [spell, Primitives.Predicate.not (Primitives.Predicate.colorIs .green)],
                          Primitives.Predicate.abilityOf (allOf (Primitives.Predicate.and [source, Primitives.Predicate.not (Primitives.Predicate.colorIs .green)])) ]))) ],
      power := stat 8, toughness := stat 5 } }

/-- Nowhere to Run -/
def nowhereToRunWardLine : StaticSpec :=
  Primitives.StaticSpec.conjunction none
    [ canBeTargetedAsThough (allOf creatureYourOpponentsControl)
        (allOf (Primitives.Predicate.or [spell, Primitives.Predicate.abilityHead .anyOnStack]))
        (Primitives.Predicate.not (Primitives.Predicate.hasKeyword (.the (.named "hexproof")))),
      Primitives.StaticSpec.cant (Primitives.Subject.actor (allOf (Primitives.Predicate.and [Primitives.Predicate.abilityHead (.keyword (.named "ward")), Primitives.Predicate.abilityOf (those (.type .creature))]))) Primitives.Action.trigger {} ]
theorem okNowhereToRunWardLine : StaticSpec.check (DRS.ofList []) nowhereToRunWardLine = [] := by decide

/-- Mornsong Aria -/
def mornsongAriaLock : StaticSpec :=
  Primitives.StaticSpec.conjunction none [Primitives.StaticSpec.cant (Primitives.Subject.actor (allOf Primitives.Predicate.anyPlayer)) (Primitives.Action.draw Primitives.Amount.anyAmount) {}, Primitives.StaticSpec.cant (Primitives.Subject.actor (allOf Primitives.Predicate.anyPlayer)) (Primitives.Action.changeLife (Primitives.Delta.up Primitives.Amount.anyAmount)) {}]
theorem okMornsongAriaLock : StaticSpec.check (DRS.ofList []) mornsongAriaLock = [] := by decide

def brainwash : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Brainwash", cost := some [pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") creature,
          Primitives.Ability.static (Primitives.StaticSpec.gatedBy (Primitives.Cost.mana [generic 3]) (Primitives.Subject.actor (Primitives.NounPhrase.attachHost .enchanted (.type .creature))) (Primitives.Action.attack none) {}) ] } }

def enkiraHostileScavenger : Ability :=
  Primitives.Ability.static (onlyWhile (Primitives.StaticSpec.must (Primitives.Subject.by (allOf creature)) (Primitives.Action.block thisCreature) {})
    (Primitives.Condition.matches thisCreature (Primitives.Predicate.isAttached (some .equipped))))
theorem okEnkiraHostileScavenger : Ability.check (DRS.ofList []) enkiraHostileScavenger = [] := by decide

def anglerTurtle : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.must (Primitives.Subject.actor (allOf creatureYourOpponentsControl)) (Primitives.Action.attack none) {})
theorem okAnglerTurtle : Ability.check (DRS.ofList []) anglerTurtle = [] := by decide

def ensnaringBridge : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ensnaring Bridge", cost := some [generic 3], types := [.artifact],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.cant (Primitives.Subject.actor (allOf (Primitives.Predicate.and [ creature,
                           Primitives.Predicate.compare [.stat .power] .greater (countOf (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (handOf Primitives.NounPhrase.you)])) ]))) (Primitives.Action.attack none) {}) ] } }

def undercoverButler : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Undercover Butler", cost := some [generic 2, hybridPip .blue .black],
      types := [.creature], subtypes := [creatureType "Human", creatureType "Rogue"],
      text :=
        [ whenever
            (attacksPlayer thisCreature
              (the (Primitives.Predicate.and [Primitives.Predicate.anyPlayer, Primitives.Predicate.superlative .max (.playerStat .lifeTotal) Primitives.Predicate.anyPlayer])))
            (Primitives.Action.establish (Primitives.StaticSpec.cant Primitives.Subject.any (Primitives.Action.block it) {})
              (some thisTurn)) ],
      power := stat 2, toughness := stat 3 } }

def aetherTunnel : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aether Tunnel", cost := some [generic 1, pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") creature,
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ getsPt (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 0)),
              Primitives.StaticSpec.cant Primitives.Subject.any (Primitives.Action.block it) {} ]) ] } }

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
            (Primitives.Action.establish (Primitives.StaticSpec.preventionBan .any (Primitives.Unpreventable.described Primitives.DamageAgent.unattributed Primitives.DamageScope.everywhere)
                .noPreventionOnly)
              (some thisTurn)),
          keywordCosting (.named "flashback") (Primitives.Cost.mana [pip .red]) ] } }

def gideonJura : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gideon Jura", cost := some [generic 3, pip .white, pip .white],
      supertypes := [.legendary], types := [.planeswalker], subtypes := [planeswalkerType "Gideon"],
      text :=
        [ activated (Primitives.Cost.loyaltySymbol (.up 2))
            (establishThroughout
              (Primitives.StaticSpec.must (Primitives.Subject.actor (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller (target (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you))]))) (Primitives.Action.attack (some thisPlaneswalker)) {})
              (duringNextTurnOf (that .player))),
          activated (Primitives.Cost.loyaltySymbol (.down 2)) (destroy (target (Primitives.Predicate.and [creature, tapped]))),
          activated (Primitives.Cost.loyaltySymbol .zero)
            (Primitives.Action.sequentially
              [ Primitives.Action.establish
                  (Primitives.StaticSpec.qualityChange thisPlaneswalker .sets
                    (Primitives.QualityPayload.bundle
                      { characteristics :=
                        { types := [.creature],
                          subtypes := [creatureType "Human", creatureType "Soldier"],
                          power := stat 6, toughness := stat 6 } }
                      (some .planeswalker)))
                  (some untilEndOfTurn),
                preventAll .any (Primitives.DamageScope.toRecipient it) (some thisTurn) ]) ],
      loyalty := stat 6 } }

def pinpointAvalanche : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Pinpoint Avalanche", cost := some [generic 3, pip .red, pip .red],
      types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 4) (target creature) .noncombatOnly),
              Primitives.Action.establish (Primitives.StaticSpec.preventionBan .any Primitives.Unpreventable.thatDamage .noPreventionOnly) none ]) ] } }

def whippoorwillImmunity : Action :=
  Primitives.Action.sequentially
    [ Primitives.Action.establish (objectCant (.action "regenerate") (target creature)) (some thisTurn),
      Primitives.Action.establish
        (Primitives.StaticSpec.preventionBan .any
          (Primitives.Unpreventable.described Primitives.DamageAgent.unattributed (Primitives.DamageScope.toRecipient (that (.type .creature)))) .noRedirectEither)
        (some thisTurn) ]
theorem okWhippoorwillImmunity : Action.check (DRS.ofList []) whippoorwillImmunity = [] := by decide

def callInAProfessional : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Call In a Professional", cost := some [generic 2, pip .red], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ Primitives.Action.establish (playerCant (.core .gainLife) (allOf Primitives.Predicate.anyPlayer))
                (some thisTurn),
              Primitives.Action.establish
                (Primitives.StaticSpec.preventionBan .any (Primitives.Unpreventable.described Primitives.DamageAgent.unattributed Primitives.DamageScope.everywhere) .noPreventionOnly)
                (some thisTurn),
              instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 3) (target anyTarget) .noncombatOnly) ]) ] } }

/-- Council of the Absolute -/
def councilOfTheAbsolute : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Council of the Absolute", cost := some [generic 2, pip .white, pip .blue],
      types := [.creature], subtypes := [creatureType "Human", creatureType "Advisor"],
      text :=
        [ Primitives.Ability.static (entersChoosingFrom thisCreature .cardName
            (Primitives.ChoiceDomain.nameOfCard (Primitives.Predicate.not (Primitives.Predicate.or [creature, land])))),
          Primitives.Ability.static (cantDoTo (.action "cast") (allOf (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you))
            (allOf (Primitives.Predicate.and [spell, Primitives.Predicate.named Primitives.NameSource.chosen]))),
          Primitives.Ability.static (Primitives.StaticSpec.costShift (allOf (Primitives.Predicate.and [spell, Primitives.Predicate.named Primitives.NameSource.chosen, castBy Primitives.NounPhrase.you]))
            (Primitives.CostShift.less (.lit 2) none)) ],
      power := stat 2, toughness := stat 4 } }

/-- Failure // Comply -/
def complyNameLock : Action :=
  Primitives.Action.sequentially
    [ choose (a (quality .cardName)),
      Primitives.Action.establish
        (cantDoTo (.action "cast") (allOf (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you))
          (allOf (Primitives.Predicate.and [spell, Primitives.Predicate.named Primitives.NameSource.chosen])))
        (some untilYourNextTurn) ]
theorem okComplyNameLock : Action.check (DRS.ofList []) complyNameLock = [] := by decide

/-- Gideon's Intervention -/
def gideonsIntervention : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gideon's Intervention", cost := some [generic 2, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ Primitives.Ability.static (entersChoosing thisEnchantment .cardName),
          Primitives.Ability.static (cantDoTo (.action "cast") (allOf (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you))
            (allOf (Primitives.Predicate.and [spell, Primitives.Predicate.named Primitives.NameSource.chosen]))),
          Primitives.Ability.static (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (allOf (Primitives.Predicate.and [source, Primitives.Predicate.named Primitives.NameSource.chosen])))
            (Primitives.DamageScope.toRecipient (youAnd (allOf (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))))
            (Primitives.DamageOp.prevent Primitives.PreventCut.all none) .repeatedly) ] } }

/-- Academic Probation -/
def academicProbationNameMode : Action :=
  Primitives.Action.sequentially
    [ choose (a (qualityFrom .cardName (Primitives.ChoiceDomain.nameOfCard (Primitives.Predicate.not land)))),
      Primitives.Action.establish
        (cantDoTo (.action "cast") (allOf (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you))
          (allOf (Primitives.Predicate.and [spell, Primitives.Predicate.named Primitives.NameSource.chosen])))
        (some untilYourNextTurn) ]
theorem okAcademicProbationNameMode : Action.check (DRS.ofList []) academicProbationNameMode = [] := by
  decide

def fatigue : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fatigue", cost := some [generic 1, pip .blue], types := [.sorcery],
      text := [Primitives.Ability.spell none (instruct (target Primitives.Predicate.anyPlayer) (Primitives.Action.skipPart .drawStep (.lit 1)))] } }

def meditate : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Meditate", cost := some [generic 2, pip .blue], types := [.instant],
      text := [Primitives.Ability.spell none (Primitives.Action.sequentially [Primitives.Action.draw (.lit 4), Primitives.Action.skipPart .turn (.lit 1)])] } }

/-- Blinding Angel {3}{W}{W} — Creature — Angel 2/4. "Flying. Whenever Blinding Angel deals
combat damage to a player, that player skips their next combat phase." -/
def blindingAngel : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blinding Angel", cost := some [generic 3, pip .white, pip .white],
      types := [.creature], subtypes := [creatureType "Angel"],
      text :=
        [ keyword (.named "flying"),
          whenever
            (dealsCombatDamage thisCreature (a Primitives.Predicate.anyPlayer))
            (instruct (that .player) (Primitives.Action.skipPart .combat (.lit 1))) ],
      power := stat 2, toughness := stat 4 } }

def eonHub : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Eon Hub", cost := some [generic 5], types := [.artifact],
      text := [Primitives.Ability.static (Primitives.StaticSpec.partSkip (allOf Primitives.Predicate.anyPlayer) .upkeep)] } }

def stasis : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stasis", cost := some [generic 1, pip .blue], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.partSkip (allOf Primitives.Predicate.anyPlayer) .untapStep),
          at_ (Primitives.GameEvent.beginningOf .upkeep (some Primitives.NounPhrase.you) (Primitives.TurnRef.each none))
            (doUnless (sacrifice thisEnchantment) (Primitives.Cost.mana [pip .blue])) ] } }

def yawgmothsBargain : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Yawgmoth's Bargain", cost := some [generic 4, pip .black, pip .black],
      types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.partSkip Primitives.NounPhrase.you .drawStep),
          activated (payLife 1) (Primitives.Action.draw (.lit 1)) ] } }

def sandsOfTimeSkip : StaticSpec := Primitives.StaticSpec.partSkip (each Primitives.Predicate.anyPlayer) .untapStep
theorem okSandsOfTimeSkip : StaticSpec.check (DRS.ofList []) sandsOfTimeSkip = [] := by decide

/-- Wormfang Manta {5}{U}{U} — Creature — Nightmare Fish Beast 6/1. "Flying. When Wormfang
Manta enters, you skip your next turn. When Wormfang Manta leaves the battlefield, you take an
extra turn after this one." -/
def wormfangManta : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Wormfang Manta", cost := some [generic 5, pip .blue, pip .blue],
      types := [.creature],
      subtypes := [creatureType "Nightmare", creatureType "Fish", creatureType "Beast"],
      text :=
        [ keyword (.named "flying"),
          when (Primitives.GameEvent.enters thisCreature none) (Primitives.Action.skipPart .turn (.lit 1)),
          when (leavesBattlefield thisCreature) (extraTurn (.lit 1)) ],
      power := stat 6, toughness := stat 1 } }

def eaterOfDaysSkip : Action := Primitives.Action.skipPart .turn (.lit 2)
theorem okEaterOfDaysSkip : Action.check (DRS.ofList []) eaterOfDaysSkip = [] := by decide

/-- Empty City Ruse -/
def emptyCityRuse : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Empty City Ruse", cost := some [pip .white], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none
            (establishThroughout (Primitives.StaticSpec.partSkip (target (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)) .combat) (duringNextTurnOf (that
                .player))) ] } }

/-- False Peace -/
def falsePeace : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "False Peace", cost := some [pip .white], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none
            (establishThroughout (Primitives.StaticSpec.partSkip (target Primitives.Predicate.anyPlayer) .combat) (duringNextTurnOf (that
                .player))) ] } }

/-- Battlefront Krushok -/
def battlefrontKrushokEvasion : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.cant (Primitives.Subject.by (counted (Primitives.Quantity.moreThanOf (.lit 1)) creature)) (Primitives.Action.block thisCreature) {})
theorem okBattlefrontKrushokEvasion : Ability.check (DRS.ofList []) battlefrontKrushokEvasion = [] := by decide

/-- Grand Abolisher -/
def grandAbolisher : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Grand Abolisher", cost := some [pip .white, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Cleric"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.partScope .turn (some Primitives.NounPhrase.you)
            (Primitives.StaticSpec.conjunction none
              [ cantDoTo (.action "cast") (allOf (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)) (allOf spell),
                cantDoTo (.action "activate") (allOf (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you))
                  (allOf (Primitives.Predicate.and [ Primitives.Predicate.abilityHead .anyActivated,
                                 Primitives.Predicate.abilityOf (allOf (Primitives.Predicate.or [artifact, creature, enchantment])) ])) ])) ],
      power := stat 2, toughness := stat 2 } }

/-- Festival -/
def festival : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Festival", cost := some [pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell (some (duringPart .upkeep (some anOpponent)))
            (forbidAttack (allOf creature) (some thisTurn)) ] } }

def demotion : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Demotion", cost := some [pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") creature,
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ Primitives.StaticSpec.cant (Primitives.Subject.actor (Primitives.NounPhrase.attachHost .enchanted (.type .creature))) (Primitives.Action.block Primitives.NounPhrase.any) {},
              objectCant (.action "activate")
                (allOf (Primitives.Predicate.and [Primitives.Predicate.abilityHead .anyActivated, Primitives.Predicate.abilityOf it])) ]) ] } }

def terror : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Terror", cost := some [generic 1, pip .black], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.doAndForbid
            (destroy (target (Primitives.Predicate.and [creature, Primitives.Predicate.not artifact, Primitives.Predicate.not (Primitives.Predicate.colorIs .black)])))
            (.action "regenerate") (theVerbed (.action "destroy") .one)) ] } }

def snuffOut : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Snuff Out", cost := some [generic 3, pip .black], types := [.instant],
      text :=
        [ Primitives.Ability.static (onlyWhile (Primitives.StaticSpec.altCost Primitives.NounPhrase.this (some (payLife 4)))
            (exists_ (Primitives.Predicate.and [land, Primitives.Predicate.hasSubtype (landType "Swamp"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))),
          Primitives.Ability.spell none (Primitives.Action.doAndForbid
            (destroy (target (Primitives.Predicate.and [creature, Primitives.Predicate.not (Primitives.Predicate.colorIs .black)])))
            (.action "regenerate") (theVerbed (.action "destroy") .one)) ] } }

def wrathOfGod : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Wrath of God", cost := some [generic 2, pip .white, pip .white],
      types := [.sorcery],
      text := [Primitives.Ability.spell none (Primitives.Action.doAndForbid (destroy (allOf creature)) (.action "regenerate") them)] }
          }

def damnation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Damnation", cost := some [generic 2, pip .black, pip .black], types := [.sorcery],
      text := [Primitives.Ability.spell none (Primitives.Action.doAndForbid (destroy (allOf creature)) (.action "regenerate") them)] }
          }

def glacialChasm : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Glacial Chasm", types := [.land],
      text :=
        [ cumulativeUpkeep (payLife 2),
          when (Primitives.GameEvent.enters thisLand none) (sacrifice (a land)),
          Primitives.Ability.static (Primitives.StaticSpec.cant (Primitives.Subject.actor (allOf creatureYouControl)) (Primitives.Action.attack none) {}),
          Primitives.Ability.static (Primitives.StaticSpec.damageRule .any Primitives.DamageAgent.unattributed (Primitives.DamageScope.toRecipient Primitives.NounPhrase.you) (Primitives.DamageOp.prevent Primitives.PreventCut.all none)
            .repeatedly) ] } }

/-- Peacekeeper -/
def peacekeeperCant : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.cant (Primitives.Subject.actor (allOf creature)) (Primitives.Action.attack none) {})
theorem okPeacekeeperCant : Ability.check (DRS.ofList []) peacekeeperCant = [] := by decide

/-- Culling Mark -/
def cullingMark : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Culling Mark", cost := some [generic 2, pip .green], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none
            (Primitives.Action.establish (Primitives.StaticSpec.must (Primitives.Subject.actor (target creature)) (Primitives.Action.block Primitives.NounPhrase.any) {})
              (some thisTurn)) ] } }

/-- Blazing Archon -/
def blazingArchonCant : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.cant (Primitives.Subject.actor (allOf creature)) (Primitives.Action.attack (some Primitives.NounPhrase.you)) {})
theorem okBlazingArchonCant : Ability.check (DRS.ofList []) blazingArchonCant = [] := by decide

/-- Glaring Spotlight -/
def glaringSpotlight : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Glaring Spotlight", cost := some [generic 1], types := [.artifact],
      text :=
        [ Primitives.Ability.static (canBeTargetedAsThough
            (allOf (Primitives.Predicate.and [ creature, Primitives.Predicate.hasPossessor .controller (allOf (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)),
                           Primitives.Predicate.hasKeyword (.the (.named "hexproof")) ]))
            (allOf (Primitives.Predicate.and [Primitives.Predicate.or [spell, Primitives.Predicate.abilityHead .anyOnStack], Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
            (Primitives.Predicate.not (Primitives.Predicate.hasKeyword (.the (.named "hexproof"))))),
          activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 3], Primitives.Cost.perform (sacrifice thisArtifact)])
            (Primitives.Action.sequentially
              [ gain (allOf creatureYouControl) (keyword (.named "hexproof")) (some untilEndOfTurn),
                Primitives.Action.establish
                  (Primitives.StaticSpec.cant Primitives.Subject.any (Primitives.Action.block (allOf creatureYouControl)) {})
                  (some thisTurn) ]) ] } }

/-- Canoptek Wraith -/
def canoptekWraith : Ability :=
  flavorWord "Wraith Form"
    (Primitives.Ability.static (Primitives.StaticSpec.cant Primitives.Subject.any (Primitives.Action.block thisCreature) {}))
theorem okCanoptekWraith : Ability.check (DRS.ofList []) canoptekWraith = [] := by decide

/-- Gadrak, the Crown-Scourge -/
def gadrakCantAttack : Ability :=
  Primitives.Ability.static (onlyUnless (Primitives.StaticSpec.cant (Primitives.Subject.actor thisCreature) (Primitives.Action.attack none) {})
    (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.and [artifact, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .atLeast (.lit 4)))
theorem okGadrakCantAttack : Ability.check (DRS.ofList []) gadrakCantAttack = [] := by decide

/-- Berserker's Frenzy, the 1—14 striation -/
def berserkersFrenzyLowRoll : Action :=
  Primitives.Action.sequentially
    [ choose (counted anyNumber creature),
      Primitives.Action.establish (Primitives.StaticSpec.must (Primitives.Subject.actor them) (Primitives.Action.block Primitives.NounPhrase.any) {}) (some thisTurn) ]
theorem okBerserkersFrenzyLowRoll : Action.check (DRS.ofList []) berserkersFrenzyLowRoll = [] := by decide

/-- Damn -/
def damnDestroyLine : Action :=
  Primitives.Action.doAndForbid (destroy (target creature)) (.action "regenerate")
    (theVerbed (.action "destroy") .one (some (.type .creature)) .thisWay)
theorem okDamnDestroyLine : Action.check (DRS.ofList []) damnDestroyLine = [] := by decide

def nekrataalWhole : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Nekrataal", cost := some [generic 2, pip .black, pip .black], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Assassin"],
      text :=
        [ keyword (.named "firstStrike"),
          when (Primitives.GameEvent.enters thisCreature none)
            (Primitives.Action.doAndForbid (destroy (target (Primitives.Predicate.and [creature, Primitives.Predicate.not artifact, Primitives.Predicate.not (Primitives.Predicate.colorIs
                .black)])))
              (.action "regenerate") (that (.type .creature))) ],
      power := stat 2, toughness := stat 1 } }

/-- Concussive Bolt, both paragraphs -/
def concussiveBolt : Action :=
  Primitives.Action.sequentially
    [ instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 4) targetPlayerOrPlaneswalker .noncombatOnly),
      Primitives.Action.doIf (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.and [artifact, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .atLeast (.lit
          3))
        (Primitives.Action.establish
          (Primitives.StaticSpec.cant (Primitives.Subject.actor (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller splitOverPlaneswalker]))) (Primitives.Action.block Primitives.NounPhrase.any) {})
          (some thisTurn))
        none ]
theorem okConcussiveBolt : Action.check (DRS.ofList []) concussiveBolt = [] := by decide

/-- Ulamog's Crusher -/
def ulamogsCrusher : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ulamog's Crusher", cost := some [generic 8], types := [.creature],
      subtypes := [creatureType "Eldrazi"],
      text :=
        [ keywordNumber (.named "annihilator") (.lit 2),
          Primitives.Ability.static (Primitives.StaticSpec.must (Primitives.Subject.actor thisCreature) (Primitives.Action.attack none) {}) ],
      power := stat 8, toughness := stat 8 } }

/-- "attack you unless their controller pays {2} for each" -/
def attackToll : StaticSpec :=
  Primitives.StaticSpec.gatedBy (scaledGenericMana
      (times (.lit 2)
        (countOf (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller they, Primitives.Predicate.inCombat .attackerOf (some Primitives.NounPhrase.you)])))) (Primitives.Subject.actor (allOf creature)) (Primitives.Action.attack (some Primitives.NounPhrase.you)) {}

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
    (Primitives.StaticSpec.gatedBy (scaledGenericMana (times (.lit 1) Primitives.Amount.groupSize)) (Primitives.Subject.actor (allOf creature)) (Primitives.Action.block Primitives.NounPhrase.any) {})
    (Primitives.Condition.matches thisCreature attacking))
theorem okArchangelOfTithesBlockToll : Ability.check (DRS.ofList []) archangelOfTithesBlockToll = [] := by decide
/-- Archangel of Tithes -/
def archangelOfTithesAttackToll : Ability :=
  Primitives.Ability.static (onlyWhile
    (Primitives.StaticSpec.gatedBy (scaledGenericMana (times (.lit 1) Primitives.Amount.groupSize)) (Primitives.Subject.actor (allOf creature)) (Primitives.Action.attack (some (youOr (allOf (Primitives.Predicate.and [Primitives.Predicate.hasType .planeswalker, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))))) {})
    (Primitives.Condition.matches thisCreature untapped))
theorem okArchangelOfTithesAttackToll : Ability.check (DRS.ofList []) archangelOfTithesAttackToll = [] := by
  decide

/-- Archon of Absolution -/
def archonOfAbsolution : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Archon of Absolution", cost := some [generic 3, pip .white], types := [.creature],
      subtypes := [creatureType "Archon"],
      text :=
        [ keyword (.named "flying"),
          keywordQuality (.named "protection") (Primitives.Predicate.colorIs .white),
          Primitives.Ability.static (Primitives.StaticSpec.gatedBy (scaledGenericMana (times (.lit 1) Primitives.Amount.groupSize)) (Primitives.Subject.actor (allOf creature)) (Primitives.Action.attack (some (youOr (allOf (Primitives.Predicate.and [Primitives.Predicate.hasType .planeswalker, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))))) {}) ],
      power := stat 3, toughness := stat 2 } }

/-- Myr Prototype -/
def myrPrototype : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Myr Prototype", cost := some [generic 5], types := [.artifact, .creature],
      subtypes := [creatureType "Myr"],
      text :=
        [ at_ (Primitives.GameEvent.beginningOf .upkeep (some Primitives.NounPhrase.you) (Primitives.TurnRef.each none))
            (Primitives.Action.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter) thisCreature .many true),
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none [Primitives.StaticSpec.gatedBy (scaledGenericMana (times (.lit 1) (countersOn p1p1Counter it))) (Primitives.Subject.actor thisCreature) (Primitives.Action.attack none) {}, Primitives.StaticSpec.gatedBy (scaledGenericMana (times (.lit 1) (countersOn p1p1Counter it))) (Primitives.Subject.actor thisCreature) (Primitives.Action.block Primitives.NounPhrase.any) {}]) ],
      power := stat 2, toughness := stat 2 } }

/-- Heat Wave -/
def heatWave : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Heat Wave", cost := some [generic 2, pip .red], types := [.enchantment],
      text :=
        [ cumulativeUpkeep (Primitives.Cost.mana [pip .red]),
          Primitives.Ability.static (Primitives.StaticSpec.cant (Primitives.Subject.actor (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.colorIs .blue]))) (Primitives.Action.block (allOf creatureYouControl)) {}),
          Primitives.Ability.static (Primitives.StaticSpec.gatedBy (Primitives.Cost.perform (instruct they (loseLife
              (times (.lit 1)
                (countOf (Primitives.Predicate.and [creature, blocking, Primitives.Predicate.hasPossessor .controller they])))))) (Primitives.Subject.actor (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.not (Primitives.Predicate.colorIs .blue)]))) (Primitives.Action.block (allOf creatureYouControl)) {}) ] } }

/-- Awesome Presence -/
def awesomePresence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Awesome Presence", cost := some [pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") creature,
          Primitives.Ability.static (Primitives.StaticSpec.gatedBy (scaledGenericMana (times (.lit 3)
              (countOf (Primitives.Predicate.and [ creature, Primitives.Predicate.hasPossessor .controller they,
                               Primitives.Predicate.inCombat .blockerOf (some it) ])))) Primitives.Subject.any (Primitives.Action.block (Primitives.NounPhrase.attachHost .enchanted (.type .creature))) {}) ] } }

/-- Oppressive Rays -/
def oppressiveRays : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Oppressive Rays", cost := some [pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") creature,
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none [Primitives.StaticSpec.gatedBy (Primitives.Cost.mana [generic 3]) (Primitives.Subject.actor (Primitives.NounPhrase.attachHost .enchanted (.type .creature))) (Primitives.Action.attack none) {}, Primitives.StaticSpec.gatedBy (Primitives.Cost.mana [generic 3]) (Primitives.Subject.actor (Primitives.NounPhrase.attachHost .enchanted (.type .creature))) (Primitives.Action.block Primitives.NounPhrase.any) {}]),
          Primitives.Ability.static (Primitives.StaticSpec.costShift
            (allOf (Primitives.Predicate.and [ Primitives.Predicate.abilityHead .anyActivated,
                           Primitives.Predicate.abilityOf (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) ]))
            (Primitives.CostShift.more (.lit 3))) ] } }

/-- Distortion Strike -/
def distortionStrikeLine : Action :=
  Primitives.Action.sequentially
    [ get (target creature) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 0)) (some untilEndOfTurn),
      Primitives.Action.establish (Primitives.StaticSpec.cant Primitives.Subject.any (Primitives.Action.block (that (.type .creature))) {})
        (some thisTurn) ]
theorem okDistortionStrikeLine : Action.check (DRS.ofList []) distortionStrikeLine = [] := by decide

/-- Retro-Mutation -/
def retroMutation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Retro-Mutation", cost := some [generic 2, pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keyword (.named "flash"),
          keywordSubject (.named "enchant") creature,
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ Primitives.StaticSpec.qualityChange (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) .sets
                (Primitives.QualityPayload.bundle { characteristics := { subtypes := [creatureType "Turtle"] } } none),
              getsPt it (Primitives.Delta.set (.lit 0)) (Primitives.Delta.set (.lit 1)),
              Primitives.StaticSpec.cant (Primitives.Subject.actor it) (Primitives.Action.attack none) {},
              Primitives.StaticSpec.allAbilityLoss it none ]) ] } }

/-- Hotshot Mechanic -/
def hotshotMechanic : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hotshot Mechanic", cost := some [pip .white], types := [.artifact, .creature],
      subtypes := [creatureType "Fox", creatureType "Pilot"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.can (Primitives.Subject.actor thisCreature)
      (Primitives.Action.crew (allOf (Primitives.Predicate.hasSubtype
        (artifactType "Vehicle")))) { asThough := some (Primitives.AsThough.greater .power (.lit 2)) }) ],
      power := stat 2, toughness := stat 1 } }

/-- Cloudspire Captain -/
def cloudspireCaptainCrewLine : StaticSpec :=
  Primitives.StaticSpec.conjunction none
    [ Primitives.StaticSpec.can (Primitives.Subject.actor thisCreature)
        (Primitives.Action.saddle (allOf (Primitives.Predicate.hasSubtype
          (creatureType "Mount")))) { asThough := some (Primitives.AsThough.greater .power (.lit 2)) },
      Primitives.StaticSpec.can (Primitives.Subject.actor thisCreature)
        (Primitives.Action.crew (allOf (Primitives.Predicate.hasSubtype
          (artifactType "Vehicle")))) { asThough := some (Primitives.AsThough.greater .power (.lit 2)) } ]
theorem okCloudspireCaptainCrewLine : StaticSpec.check (DRS.ofList []) cloudspireCaptainCrewLine = [] := by
  decide

/-- Revoke Privileges -/
def revokePrivileges : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Revoke Privileges", cost := some [generic 2, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") creature,
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none
      [ Primitives.StaticSpec.cant (Primitives.Subject.actor (Primitives.NounPhrase.attachHost .enchanted (.type .creature)))
          (Primitives.Action.attack none) {},
        Primitives.StaticSpec.cant (Primitives.Subject.actor (Primitives.NounPhrase.attachHost .enchanted (.type .creature)))
          (Primitives.Action.block Primitives.NounPhrase.any) {},
        Primitives.StaticSpec.cant (Primitives.Subject.actor (Primitives.NounPhrase.attachHost .enchanted (.type .creature)))
          (Primitives.Action.crew (allOf (Primitives.Predicate.hasSubtype
            (artifactType "Vehicle")))) {} ]) ] } }

/-- Harried Spearguard -/
def harriedSpearguard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Harried Spearguard", cost := some [pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Soldier"],
      text :=
        [ keyword (.named "haste"),
          when (Primitives.GameEvent.dies thisCreature)
            (create (.lit 1)
              { characteristics :=
                { colors := [.black], types := [.creature], subtypes := [creatureType "Rat"],
                  text :=
                    [ Primitives.Ability.static (Primitives.StaticSpec.cant (Primitives.Subject.actor (Primitives.NounPhrase.asMarker .token Primitives.NounPhrase.this)) (Primitives.Action.block Primitives.NounPhrase.any) {}) ],
                  power := stat 1, toughness := stat 1 } }) ],
      power := stat 1, toughness := stat 1 } }

/-- Arrest -/
def arrest : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Arrest", cost := some [generic 2, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") creature,
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ Primitives.StaticSpec.cant (Primitives.Subject.actor (Primitives.NounPhrase.attachHost .enchanted (.type .creature))) (Primitives.Action.attack none) {}, Primitives.StaticSpec.cant (Primitives.Subject.actor (Primitives.NounPhrase.attachHost .enchanted (.type .creature))) (Primitives.Action.block Primitives.NounPhrase.any) {},
              Primitives.StaticSpec.cant Primitives.Subject.any
                (Primitives.Action.procedure .activate
                  (allOf (Primitives.Predicate.and [Primitives.Predicate.abilityHead .anyActivated,
                    Primitives.Predicate.abilityOf it])) Primitives.PlayPayment.itsOwnCost none none none)
                {} ]) ] } }

/-- Conqueror's Flail -/
def conquerorsFlailProhibition : StaticSpec :=
  onlyWhile
    (Primitives.StaticSpec.partScope .turn (some Primitives.NounPhrase.you)
      (cantDoTo (.action "cast") (allOf (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)) (allOf spell)))
    (Primitives.Condition.matches Primitives.NounPhrase.this (Primitives.Predicate.attachedTo (a creature)))
theorem okConquerorsFlailProhibition : StaticSpec.check (DRS.ofList []) conquerorsFlailProhibition = [] := by
  decide

end Semantics.Cards

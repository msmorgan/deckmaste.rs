import Semantics
import Semantics.Macros
import Semantics.Macros.Declared
import Semantics.Check.Card
import Semantics.Cards.Anaphora
import Semantics.Cards.Keyword

/-!
# Semantics.Cards.Counters

Port of `idris/src/Experimental/Cards/Counters.idr`: the printed cards of the Counters family
and the bench items beside them.
-/

open Semantics Semantics.Macros

namespace Semantics.Cards

/-- Crovax the Cursed -/
def crovaxTheCursed : Instruction :=
  Primitives.Instruction.offer (sacrifice (a creature))
    (some (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter) thisCreature .many true))
    (some (Primitives.Instruction.removeCounters (some (exactly 1)) (some (Primitives.CounterKindSource.printed p1p1Counter)) thisCreature .many true))
theorem okCrovaxTheCursed : Instruction.check (DRS.ofList []) crovaxTheCursed = [] := by decide
def additiveEvolution : Instruction :=
  Primitives.Instruction.sequentially
    [ create (.lit 1) (creatureToken 0 0 [.green, .blue] [creatureType "Fractal"]),
      Primitives.Instruction.putCounters (.lit 3) (Primitives.CounterKindSource.printed p1p1Counter) it .many true ]
theorem okAdditiveEvolution : Instruction.check (DRS.ofList []) additiveEvolution = [] := by decide
def battlegrowth : Instruction := Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter) (target creature) .many true
theorem okBattlegrowth : Instruction.check (DRS.ofList []) battlegrowth = [] := by decide
def chainbreaker : Instruction :=
  Primitives.Instruction.removeCounters (some (exactly 1)) (some (Primitives.CounterKindSource.printed m1m1Counter)) (target creature) .many true
theorem okChainbreaker : Instruction.check (DRS.ofList []) chainbreaker = [] := by decide
def kaitoBaneOfNightmares : Instruction :=
  Primitives.Instruction.sequentially
    [Primitives.Instruction.setStatus .tapped (target creature), Primitives.Instruction.putCounters (.lit 2) (Primitives.CounterKindSource.printed (.named "stunCounter")) it .many true]
theorem okKaitoBaneOfNightmares : Instruction.check (DRS.ofList []) kaitoBaneOfNightmares = [] := by decide
def jhoiraOfTheGhitu : Ability :=
  activated
    (Primitives.Cost.compound [Primitives.Cost.mana [generic 2], Primitives.Cost.perform (exile (a (Primitives.Predicate.and [Primitives.Predicate.not land, Primitives.Predicate.inZone (handOf Primitives.NounPhrase.you)])))])
    (Primitives.Instruction.putCounters (.lit 4) (Primitives.CounterKindSource.printed (.named "timeCounter"))
      (theVerbed (.action "exile") .one (some .card)) .many true)
theorem okJhoiraOfTheGhitu : Ability.check (DRS.ofList []) jhoiraOfTheGhitu = [] := by decide
def alaundoTheSeer : Instruction :=
  Primitives.Instruction.removeCounters (some (exactly 1)) (some (Primitives.CounterKindSource.printed (.named "timeCounter")))
    (each (Primitives.Predicate.and [Primitives.Predicate.hasPossessor .owner Primitives.NounPhrase.you, Primitives.Predicate.inZone exileZone])) .many true
theorem okAlaundoTheSeer : Instruction.check (DRS.ofList []) alaundoTheSeer = [] := by decide

def daydream : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Daydream", cost := some [pip .white], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ exile (target creatureYouControl),
              returnToBattlefieldWithCounters (that .card) (ownerOf (that .card)) (.lit 1)
                p1p1Counter ]),
          keywordCosting (.named "flashback") (Primitives.Cost.mana [generic 2, pip .white]) ] } }

def ashnodsTransmogrant : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.tapSymbol, Primitives.Cost.perform (sacrifice thisArtifact)])
    (Primitives.Instruction.sequentially
      [ Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter) (target (Primitives.Predicate.and [creature, Primitives.Predicate.not artifact])) .many true,
        become (that (.type .creature)) { characteristics := { types := [.artifact] } } none ])
theorem okAshnodsTransmogrant : Ability.check (DRS.ofList []) ashnodsTransmogrant = [] := by decide
def azulaAlwaysLies : Instruction :=
  chooseModes (oneThrough 2)
    [ get (target creature) (Primitives.Delta.down (.lit 1)) (Primitives.Delta.down (.lit 1)) (some untilEndOfTurn),
      Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter) (target creature) .many true ]
theorem okAzulaAlwaysLies : Instruction.check (DRS.ofList []) azulaAlwaysLies = [] := by decide
def bellowingAegisaur : Instruction :=
  Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter) (each (otherCreatureYouControl thisCreature)) .many true
theorem okBellowingAegisaur : Instruction.check (DRS.ofList []) bellowingAegisaur = [] := by decide
def carnifexDemon : Instruction :=
  Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed m1m1Counter) (each (otherCreature thisCreature)) .many true
theorem okCarnifexDemon : Instruction.check (DRS.ofList []) carnifexDemon = [] := by decide
def ajaniAdversaryOfTyrants : Instruction :=
  Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter) (Primitives.NounPhrase.eachOf (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 2)) creature)) .many true
theorem okAjaniAdversaryOfTyrants : Instruction.check (DRS.ofList []) ajaniAdversaryOfTyrants = [] := by decide
def naturesPanoply : Instruction :=
  Primitives.Instruction.sequentially
    [ choose (Primitives.NounPhrase.described (Primitives.DetPhrase.target anyNumber) creature),
      Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter) (Primitives.NounPhrase.eachOf them) .many true ]
theorem okNaturesPanoply : Instruction.check (DRS.ofList []) naturesPanoply = [] := by decide
def armamentCorps : Instruction :=
  distributeCounters (.lit 2) p1p1Counter
    (Primitives.NounPhrase.described (Primitives.DetPhrase.target (oneThrough 2)) creatureYouControl)
theorem okArmamentCorps : Instruction.check (DRS.ofList []) armamentCorps = [] := by decide
def savagebornHydra : Ability :=
  activatedOnlyDuring (Primitives.Cost.mana [generic 1, hybridPip .red .green])
    (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter) thisCreature .many true) asSorcery
theorem okSavagebornHydra : Ability.check (DRS.ofList []) savagebornHydra = [] := by decide
def woeleecher : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [pip .white], Primitives.Cost.tapSymbol])
    (Primitives.Instruction.doIfDone
      (Primitives.Instruction.removeCounters (some (exactly 1)) (some (Primitives.CounterKindSource.printed m1m1Counter)) (target creature) .many true)
      (some (gainLife (.lit 2))) none)
theorem okWoeleecher : Ability.check (DRS.ofList []) woeleecher = [] := by decide
def anointerOfValor : Ability :=
  whenever (attacks (a creature))
    (offerWhen (Primitives.Instruction.pay (Primitives.Cost.mana [generic 3]) .once .paid none none)
      (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter) (that (.type .creature)) .many true))
theorem okAnointerOfValor : Ability.check (DRS.ofList []) anointerOfValor = [] := by decide
def avianOddity : Instruction :=
  Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed flyingCounter) (target creatureYouControl) .many true
theorem okAvianOddity : Instruction.check (DRS.ofList []) avianOddity = [] := by decide
/-- Song of Eärendil -/
def songOfEarendil : Instruction :=
  Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed flyingCounter)
    (each (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, Primitives.Predicate.not (Primitives.Predicate.hasKeyword (.the (.named "flying")))])) .many true
theorem okSongOfEarendil : Instruction.check (DRS.ofList []) songOfEarendil = [] := by decide
def gideonsAvenger : Ability :=
  whenever (Primitives.GameEvent.does Primitives.Subject.any (Primitives.Instruction.setStatus .tapped (a (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller anOpponent]))))
    (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter) thisCreature .many true)
theorem okGideonsAvenger : Ability.check (DRS.ofList []) gideonsAvenger = [] := by decide
def prologueToPhyresis : Instruction :=
  Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "poison")) (each Primitives.Predicate.opponent) .many true
theorem okPrologueToPhyresis : Instruction.check (DRS.ofList []) prologueToPhyresis = [] := by decide
def screechingScorchbeast : Ability :=
  whenever (attacks thisCreature) (Primitives.Instruction.putCounters (.lit 2) (Primitives.CounterKindSource.printed (.named "radCounter")) (each Primitives.Predicate.anyPlayer) .many true)
theorem okScreechingScorchbeast : Ability.check (DRS.ofList []) screechingScorchbeast = [] := by decide
def merenOfClanNelToth : Ability :=
  whenever (Primitives.GameEvent.dies (a (otherCreatureYouControl thisCreature)))
    (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "experience")) Primitives.NounPhrase.you .many true)
theorem okMerenOfClanNelToth : Ability.check (DRS.ofList []) merenOfClanNelToth = [] := by decide
def kratosStoicFather : Ability :=
  at_ (Primitives.GameEvent.beginningOf .endStep (some Primitives.NounPhrase.you) (Primitives.TurnRef.each none))
    (Primitives.Instruction.putCounters (countersOn (.named "experience") Primitives.NounPhrase.you) (Primitives.CounterKindSource.printed p1p1Counter)
      (target creature) .many true)
theorem okKratosStoicFather : Ability.check (DRS.ofList []) kratosStoicFather = [] := by decide
def vashtaNerada : Ability :=
  triggeredIf (Primitives.GameEvent.beginningOf .endStep (some (each Primitives.Predicate.anyPlayer)) (Primitives.TurnRef.each none))
    (happened (Primitives.Instruction.move (a creature)
      (Primitives.EventSource.zones [battlefield]) (some graveyard) .departure none []) Primitives.Subject.any soFarThisTurn)
    (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter) thisCreature .many true)
theorem okVashtaNerada : Ability.check (DRS.ofList []) vashtaNerada = [] := by decide
def furiousSpinesplitter : Ability :=
  at_ (Primitives.GameEvent.beginningOf .endStep (some Primitives.NounPhrase.you) (Primitives.TurnRef.each none))
    (Primitives.Instruction.putCounters (forEach 1 (Primitives.Predicate.and
      [Primitives.Predicate.opponent, happenedTo Primitives.Subject.any (Primitives.Instruction.dealDamage Primitives.Amount.anyAmount (relative
      .player) .any) soFarThisTurn]))
      (Primitives.CounterKindSource.printed p1p1Counter) thisCreature .many true)
theorem okFuriousSpinesplitter : Ability.check (DRS.ofList []) furiousSpinesplitter = [] := by decide
def paladinOfAtonement : Ability :=
  triggeredIf (Primitives.GameEvent.beginningOf .upkeep (some (each Primitives.Predicate.anyPlayer)) (Primitives.TurnRef.each none))
    (happened (Primitives.Instruction.changeLife (Primitives.Delta.down Primitives.Amount.anyAmount)) (Primitives.Subject.actor Primitives.NounPhrase.you) lastTurn) (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed
      p1p1Counter) thisCreature .many true)
theorem okPaladinOfAtonement : Ability.check (DRS.ofList []) paladinOfAtonement = [] := by decide
def throneWarden : Ability :=
  triggeredIf (Primitives.GameEvent.beginningOf .endStep (some Primitives.NounPhrase.you) (Primitives.TurnRef.each none))
    (Primitives.Condition.matches Primitives.NounPhrase.you (Primitives.Predicate.hasDesignation (.named "monarch") none))
    (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter) thisCreature .many true)
theorem okThroneWarden : Ability.check (DRS.ofList []) throneWarden = [] := by decide
def bloodTyrant : Ability :=
  whenever (Primitives.GameEvent.does (Primitives.Subject.actor (a Primitives.Predicate.anyPlayer)) (Primitives.Instruction.conclude .loseGame)) (Primitives.Instruction.putCounters (.lit 5) (Primitives.CounterKindSource.printed p1p1Counter) thisCreature .many true)
theorem okBloodTyrant : Ability.check (DRS.ofList []) bloodTyrant = [] := by decide

def stagBeetle : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stag Beetle", cost := some [generic 3, pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Insect"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ entersWithCounters thisCreature (Primitives.Amount.letter .x) p1p1Counter,
              Primitives.StaticSpec.letterDefinition .x (countOf (otherCreature thisCreature)) ]) ],
      power := stat 0, toughness := stat 0 } }

def toweringTitan : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.conjunction none
    [ entersWithCounters thisCreature (Primitives.Amount.letter .x) p1p1Counter,
      Primitives.StaticSpec.letterDefinition .x
        (aggregate .sum (.stat .toughness) (otherCreatureYouControl thisCreature)) ])
theorem okToweringTitan : Ability.check (DRS.ofList []) toweringTitan = [] := by decide

def coretapper : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Coretapper", cost := some [generic 2], types := [.artifact, .creature],
      subtypes := [creatureType "Myr"],
      text :=
        [ activated Primitives.Cost.tapSymbol (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "chargeCounter")) (target artifact) .many true),
          activated (Primitives.Cost.perform (sacrifice thisCreature))
            (Primitives.Instruction.putCounters (.lit 2) (Primitives.CounterKindSource.printed (.named "chargeCounter")) (target artifact) .many true) ],
      power := stat 1, toughness := stat 1 } }

def divineIntervention : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Divine Intervention", cost := some [generic 6, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ Primitives.Ability.static (entersWithCounters thisEnchantment (.lit 2) (.named "interventionCounter")),
          at_ (Primitives.GameEvent.beginningOf .upkeep (some Primitives.NounPhrase.you) (Primitives.TurnRef.each none))
            (Primitives.Instruction.removeCounters (some (exactly 1)) (some (Primitives.CounterKindSource.printed (.named "interventionCounter")))
              thisEnchantment .many true),
          when (lastCounterRemovedBy (.named "interventionCounter") thisEnchantment Primitives.NounPhrase.you) Primitives.Instruction.drawGame ] } }

def celestialConvergence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Celestial Convergence", cost := some [generic 2, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ Primitives.Ability.static (entersWithCounters thisEnchantment (.lit 7) (.named "omenCounter")),
          at_ (Primitives.GameEvent.beginningOf .upkeep (some Primitives.NounPhrase.you) (Primitives.TurnRef.each none))
            (Primitives.Instruction.sequentially
              [ Primitives.Instruction.removeCounters (some (exactly 1)) (some (Primitives.CounterKindSource.printed (.named "omenCounter"))) thisEnchantment .many true,
                Primitives.Instruction.doIf (Primitives.Condition.compareAmt (countersOn (.named "omenCounter") thisEnchantment) .atMost (.lit 0))
                  (instruct (the (Primitives.Predicate.and [Primitives.Predicate.anyPlayer, Primitives.Predicate.superlative .max (.playerStat .lifeTotal) Primitives.Predicate.anyPlayer])) (Primitives.Instruction.conclude .winGame))
                  none,
                Primitives.Instruction.doIf (Primitives.Condition.compareAmt
                        (countOf (Primitives.Predicate.and [Primitives.Predicate.anyPlayer, Primitives.Predicate.superlative .max (.playerStat .lifeTotal) Primitives.Predicate.anyPlayer]))
                        .atLeast (.lit 2))
                  Primitives.Instruction.drawGame none ]) ] } }

def curseOfVengeance : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Curse of Vengeance", cost := some [pip .black], types := [.enchantment],
      subtypes := [enchantmentType "Aura", enchantmentType "Curse"],
      text :=
        [ keywordSubject (.named "enchant") Primitives.Predicate.anyPlayer,
          whenever (Primitives.GameEvent.does (Primitives.Subject.actor (Primitives.NounPhrase.attachHost .enchanted .player)) (Primitives.Instruction.procedure .cast ((a spell)) Primitives.PlayPayment.itsOwnCost none none none))
            (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "spiteCounter")) thisAura .many true),
          when (Primitives.GameEvent.does (Primitives.Subject.actor (Primitives.NounPhrase.attachHost .enchanted .player)) (Primitives.Instruction.conclude .loseGame))
            (Primitives.Instruction.sequentially
              [ gainLife (Primitives.Amount.letter .x), Primitives.Instruction.draw (Primitives.Amount.letter .x),
                Primitives.Instruction.define .x (countersOn (.named "spiteCounter") thisAura) ]) ] } }

def passagewaySeer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Passageway Seer", cost := some [generic 3, pip .black], types := [.creature],
      subtypes := [creatureType "Tiefling", creatureType "Warlock"],
      text :=
        [ keyword (.named "lifelink"),
          when (Primitives.GameEvent.enters thisCreature none) (Primitives.Instruction.gainDesignation Primitives.NounPhrase.you (.named "initiative") none),
          triggeredIf (Primitives.GameEvent.beginningOf .endStep (some Primitives.NounPhrase.you) (Primitives.TurnRef.each none))
            (Primitives.Condition.matches Primitives.NounPhrase.you (Primitives.Predicate.hasDesignation (.named "initiative") none))
            (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter) thisCreature .many true) ],
      power := stat 2, toughness := stat 2 } }

def chainsaw : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chainsaw", cost := some [generic 1, pip .red], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ when (Primitives.GameEvent.enters thisEquipment none)
            (instruct it (Primitives.Instruction.dealDamage (.lit 3) (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 1)) creature) .noncombatOnly)),
          whenever (Primitives.GameEvent.dies (counted (atLeast 1) creature))
            (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "revCounter")) thisEquipment .many true),
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ getsPt (Primitives.NounPhrase.attachHost .equipped (.type .creature)) (Primitives.Delta.up (Primitives.Amount.letter .x)) (Primitives.Delta.up (.lit 0)),
              Primitives.StaticSpec.letterDefinition .x (countersOn (.named "revCounter") thisEquipment) ]),
          keywordCosting (.named "equip") (Primitives.Cost.mana [generic 3]) ] } }

/-- Soul's Might -/
def soulsMight : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Soul's Might", cost := some [generic 4, pip .green], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ Primitives.Instruction.putCounters (Primitives.Amount.letter .x) (Primitives.CounterKindSource.printed p1p1Counter) (target creature) .many true,
              Primitives.Instruction.define .x (Primitives.Amount.statOf (.stat .power) (that (.type .creature))) ]) ] } }

def woodlandChampion : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Woodland Champion", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [creatureType "Elf", creatureType "Scout"],
      text :=
        [ whenever (Primitives.GameEvent.enters (counted (atLeast 1) (Primitives.Predicate.and [Primitives.Predicate.isToken, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) none)
            (Primitives.Instruction.putCounters Primitives.Amount.groupSize (Primitives.CounterKindSource.printed p1p1Counter) thisCreature .many true) ],
      power := stat 2, toughness := stat 2 } }

def voraciousBrood : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Voracious Brood", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [creatureType "Alien", creatureType "Insect"],
      text :=
        [ Primitives.Ability.static (entersWithCounters thisCreature
            (forEach 1 (Primitives.Predicate.and [creature, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)])) p1p1Counter),
          whenever (putIntoFrom (counted (atLeast 1) (Primitives.Predicate.and [Primitives.Predicate.isCard, creature])) (graveyardOf Primitives.NounPhrase.you) Primitives.EventSource.anywhere)
            (Primitives.Instruction.putCounters Primitives.Amount.groupSize (Primitives.CounterKindSource.printed p1p1Counter) thisCreature .many true) ],
      power := stat 1, toughness := stat 1 } }

def herdBaloth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Herd Baloth", cost := some [generic 3, pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Beast"],
      text :=
        [ whenever (counterEvent .put p1p1Counter .many thisCreature)
            (offer (create (.lit 1) (creatureToken 4 4 [.green] [creatureType "Beast"]))) ],
      power := stat 4, toughness := stat 4 } }

def flourishingDefenses : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Flourishing Defenses", cost := some [generic 4, pip .green], types := [.enchantment],
      text :=
        [ whenever (counterEvent .put m1m1Counter .one (a creature))
            (offer
              (create (.lit 1) (creatureToken 1 1 [.green] [creatureType "Elf", creatureType
                  "Warrior"]))) ] } }

def rakshasaVizier : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rakshasa Vizier", cost := some [generic 2, pip .black, pip .green, pip .blue],
      types := [.creature], subtypes := [creatureType "Demon"],
      text :=
        [ whenever
            (putIntoFrom (counted (atLeast 1) (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)])) exileZone
              (Primitives.EventSource.zones [graveyardOf Primitives.NounPhrase.you]))
            (Primitives.Instruction.putCounters Primitives.Amount.groupSize (Primitives.CounterKindSource.printed p1p1Counter) thisCreature .many true) ],
      power := stat 4, toughness := stat 4 } }

def foeLiage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Foe-liage", cost := some [generic 3, pip .green], types := [.creature],
      subtypes := [creatureType "Plant", creatureType "Mutant"],
      text :=
        [ triggeredOnlyDuring (Primitives.GameEvent.enters (a land) none) (duringPart .turn (some Primitives.NounPhrase.you))
            (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter) thisCreature .many true) ],
      power := stat 3, toughness := stat 3 } }

def hardenedScales : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hardened Scales", cost := some [pip .green], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.instead (Primitives.WouldClause.would Primitives.Subject.any (Primitives.Instruction.putCounters Primitives.Amount.anyAmount (Primitives.CounterKindSource.printed p1p1Counter) (a creatureYouControl) .many false)) (Primitives.Instruction.putCounters (plus Primitives.Amount.thatMuch (.lit 1)) (Primitives.CounterKindSource.printed p1p1Counter) it .many true) none .repeatedly none) ] } }

def branchingEvolution : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Branching Evolution", cost := some [generic 2, pip .green], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.instead (Primitives.WouldClause.would Primitives.Subject.any (Primitives.Instruction.putCounters Primitives.Amount.anyAmount (Primitives.CounterKindSource.printed p1p1Counter) (a creatureYouControl) .many false)) (Primitives.Instruction.putCounters (times (.lit 2) Primitives.Amount.thatMuch) (Primitives.CounterKindSource.printed p1p1Counter) (that (.type .creature)) .many true) none .repeatedly none) ] } }

/-- Corpsejack Menace -/
def corpsejackMenace : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.instead (Primitives.WouldClause.would Primitives.Subject.any (Primitives.Instruction.putCounters Primitives.Amount.anyAmount (Primitives.CounterKindSource.printed p1p1Counter) (a creatureYouControl) .many false)) (Primitives.Instruction.putCounters (times (.lit 2) Primitives.Amount.thatMuch) (Primitives.CounterKindSource.printed p1p1Counter) it .many true) none .repeatedly none)
theorem okCorpsejackMenace : Ability.check (DRS.ofList []) corpsejackMenace = [] := by decide

/-- Doubling Season -/
def doublingSeason : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Doubling Season", cost := some [generic 4, pip .green], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.instead (Primitives.WouldClause.would Primitives.Subject.any (Primitives.Instruction.createObject Primitives.Amount.anyAmount (Primitives.CreationSpec.token (Primitives.TokenSpec.copyOf (counted (atLeast 1) Primitives.Predicate.isToken) []) []) true (some Primitives.NounPhrase.you))) (Primitives.Instruction.create (times (.lit 2) Primitives.Amount.groupSize) Primitives.TokenSpec.asThose []) none .repeatedly none),
          Primitives.Ability.static (Primitives.StaticSpec.instead (Primitives.WouldClause.would Primitives.Subject.any (Primitives.Instruction.putCounters Primitives.Amount.anyAmount Primitives.CounterKindSource.anyKind (a (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .many true)) (Primitives.Instruction.putCounters (times (.lit 2) Primitives.Amount.thatMuch) Primitives.CounterKindSource.those (that .permanent) .many true) none .repeatedly none) ] } }

/-- Doc Samson, Super Psychiatrist -/
def docSamsonDistributive : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.instead (Primitives.WouldClause.would (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Instruction.putCounters Primitives.Amount.anyAmount Primitives.CounterKindSource.anyKind (a (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .many false)) (Primitives.Instruction.putCounters (plus Primitives.Amount.thatMuch (.lit 1)) Primitives.CounterKindSource.those (that .permanent) .many true) none .repeatedly none)
theorem okDocSamsonDistributive : Ability.check (DRS.ofList []) docSamsonDistributive = [] := by decide

/-- Winding Constrictor -/
def windingConstrictor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Winding Constrictor", cost := some [pip .black, pip .green], types := [.creature],
      subtypes := [creatureType "Snake"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.instead (Primitives.WouldClause.would Primitives.Subject.any (Primitives.Instruction.putCounters Primitives.Amount.anyAmount Primitives.CounterKindSource.anyKind (a (Primitives.Predicate.and [Primitives.Predicate.or [artifact, creature], Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .many false)) (Primitives.Instruction.putCounters (plus Primitives.Amount.thatMuch (.lit 1)) Primitives.CounterKindSource.those (that .permanent) .many true) none .repeatedly none),
          Primitives.Ability.static (Primitives.StaticSpec.instead (Primitives.WouldClause.would Primitives.Subject.any (Primitives.Instruction.putCounters Primitives.Amount.anyAmount Primitives.CounterKindSource.anyKind Primitives.NounPhrase.you .many false)) (Primitives.Instruction.putCounters (plus Primitives.Amount.thatMuch (.lit 1)) Primitives.CounterKindSource.those Primitives.NounPhrase.you .many true) none .repeatedly none) ],
      power := stat 2, toughness := stat 3 } }

/-- Aragorn, Company Leader -/
def aragornDistributive : Ability :=
  whenever (manyBareCountersPutBy Primitives.NounPhrase.you thisCreature)
    (Primitives.Instruction.putCounters (.lit 1) Primitives.CounterKindSource.those (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 1)) (otherCreature thisCreature)) .many true)
theorem okAragornDistributive : Ability.check (DRS.ofList []) aragornDistributive = [] := by decide
/-- Captain Marvel, Apex Avenger -/
def captainMarvelSameKinds : Ability :=
  triggeredIf (manyBareCountersPutBy Primitives.NounPhrase.you (a (otherCreature thisCreature)))
    (Primitives.Condition.not (Primitives.Condition.matches it (Primitives.Predicate.hasSubtype (creatureType "Kree"))))
    (offer (Primitives.Instruction.putCounters Primitives.Amount.thatMuch Primitives.CounterKindSource.those thisCreature .many true))
theorem okCaptainMarvelSameKinds : Ability.check (DRS.ofList []) captainMarvelSameKinds = [] := by decide
/-- Denry Klin, Editor in Chief -/
def denryKlinSameKinds : Ability :=
  triggeredIf (Primitives.GameEvent.enters (a (Primitives.Predicate.and [nontoken, creatureYouControl])) none)
    (Primitives.Condition.matches thisCreature (Primitives.Predicate.hasCounters none))
    (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.sameAs thisCreature) (that (.type .creature)) .many true)
theorem okDenryKlinSameKinds : Ability.check (DRS.ofList []) denryKlinSameKinds = [] := by decide

/-- Denry Klin, Editor in Chief -/
def denryKlin : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Denry Klin, Editor in Chief", cost := some [generic 2, pip .white, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Cat", creatureType "Advisor"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.entryRider thisCreature
            (Primitives.EntryRider.withCounters (.lit 1)
              (Primitives.CounterKindSource.chosen [p1p1Counter, .named "firstStrikeCounter", .named "vigilanceCounter"]) .fresh)),
          denryKlinSameKinds ],
      power := stat 2, toughness := stat 2 } }

/-- Me, the Immortal -/
def meTheImmortalCounterMenu : Ability :=
  at_ (Primitives.GameEvent.beginningOf .combat (some Primitives.NounPhrase.you) (Primitives.TurnRef.each none))
    (Primitives.Instruction.putCounters (.lit 1)
      (Primitives.CounterKindSource.chosen [p1p1Counter, .named "firstStrikeCounter", .named "vigilanceCounter", .named "menaceCounter"])
      thisCreature .many true)
theorem okMeTheImmortalCounterMenu : Ability.check (DRS.ofList []) meTheImmortalCounterMenu = [] := by decide

/-- Star Pupil -/
def starPupil : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Star Pupil", cost := some [pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ Primitives.Ability.static (entersWithCounters thisCreature (.lit 1) p1p1Counter),
          when (Primitives.GameEvent.dies thisCreature) (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.sameAs it) (target creatureYouControl) .many true) ],
      power := stat 0, toughness := stat 0 } }

/-- Tromell, Seymour's Butler -/
def tromell : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tromell, Seymour's Butler", cost := some [generic 2, pip .green],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Elf", creatureType "Advisor"],
      text :=
        [ Primitives.Ability.static (entersWithAdditionalCounters
            (each (Primitives.Predicate.and [creature, nontoken, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, Primitives.Predicate.otherThan thisCreature]))
            (.lit 1) p1p1Counter),
          activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 1], Primitives.Cost.tapSymbol])
            (Primitives.Instruction.sequentially
              [ Primitives.Instruction.repeatTimes (Primitives.Amount.letter .x) proliferate,
                Primitives.Instruction.define .x
                  (countOf (Primitives.Predicate.and [ nontoken, creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you,
                                   Primitives.Predicate.did (⟨Primitives.Subject.any, (Primitives.Instruction.move (relative .object) Primitives.EventSource.anywhere (some battlefield) .arrival none []), (Primitives.LookbackSpan.duration soFarThisTurn none)⟩) ])) ]) ],
      power := stat 2, toughness := stat 3 } }

/-- Runadi, Behemoth Caller -/
def runadiBehemothCaller : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.abilityGrant
    (allOf (Primitives.Predicate.and [ creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you,
                   Primitives.Predicate.compare [.counter p1p1Counter] .atLeast (.lit 3) ]))
    (keyword (.named "haste")))
theorem okRunadiBehemothCaller : Ability.check (DRS.ofList []) runadiBehemothCaller = [] := by decide
/-- Boon of Safety -/
def boonOfSafetyPut : Instruction :=
  Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "shieldCounter")) (target creature) .many true
theorem okBoonOfSafetyPut : Instruction.check (DRS.ofList []) boonOfSafetyPut = [] := by decide
/-- Vivien's Talent and Teferi's Talent -/
def talentLoyaltyPut : Instruction :=
  Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "loyaltyCounter")) (Primitives.NounPhrase.attachHost .enchanted (.type .planeswalker)) .many true
theorem okTalentLoyaltyPut : Instruction.check (DRS.ofList []) talentLoyaltyPut = [] := by decide
/-- Simic Fluxmage -/
def simicFluxmageMove : Instruction :=
  Primitives.Instruction.moveCounters (.lit 1) (some (Primitives.CounterKindSource.printed p1p1Counter)) thisCreature (target creature)
theorem okSimicFluxmageMove : Instruction.check (DRS.ofList []) simicFluxmageMove = [] := by decide
/-- Rikku, Resourceful Guardian -/
def rikkuStealMove : Instruction :=
  Primitives.Instruction.moveCounters (.lit 1) none (target (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller anOpponent]))
    (target creatureYouControl)
theorem okRikkuStealMove : Instruction.check (DRS.ofList []) rikkuStealMove = [] := by decide
/-- Littjara Mirrorlake -/
def littjaraMirrorlakeCopy : Instruction :=
  Primitives.Instruction.create (.lit 1)
    (Primitives.TokenSpec.copyOf (target (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
      [Primitives.CopyExcept.entersWithCounters (.lit 1) p1p1Counter .additional])
    []
theorem okLittjaraMirrorlakeCopy : Instruction.check (DRS.ofList []) littjaraMirrorlakeCopy = [] := by decide

def shalaiAndHallar : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shalai and Hallar", cost := some [generic 1, pip .red, pip .green, pip .white],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Angel", creatureType "Elf"],
      text :=
        [ keyword (.named "flying"), keyword (.named "vigilance"),
          whenever (counterEvent .put p1p1Counter .many (a creatureYouControl))
            (instruct thisCreature (Primitives.Instruction.dealDamage Primitives.Amount.thatMuch (target Primitives.Predicate.opponent) .noncombatOnly)) ],
      power := stat 3, toughness := stat 3 } }

def primalVigor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Primal Vigor", cost := some [generic 4, pip .green], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.instead (Primitives.WouldClause.would Primitives.Subject.any (Primitives.Instruction.createObject Primitives.Amount.anyAmount (Primitives.CreationSpec.token (Primitives.TokenSpec.copyOf (counted (atLeast 1) Primitives.Predicate.isToken) []) []) false none)) (Primitives.Instruction.create (times (.lit 2) Primitives.Amount.groupSize) Primitives.TokenSpec.asThose []) none .repeatedly none),
          Primitives.Ability.static (Primitives.StaticSpec.instead (Primitives.WouldClause.would Primitives.Subject.any (Primitives.Instruction.putCounters Primitives.Amount.anyAmount (Primitives.CounterKindSource.printed p1p1Counter) (a creature) .many false)) (Primitives.Instruction.putCounters (times (.lit 2) Primitives.Amount.thatMuch) (Primitives.CounterKindSource.printed p1p1Counter) (that (.type .creature)) .many true) none .repeatedly none) ] } }

/-- Patrolling Peacemaker -/
def patrollingPeacemaker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Patrolling Peacemaker", cost := some [generic 2, pip .white],
      types := [.artifact, .creature], subtypes := [creatureType "Robot", creatureType "Soldier"],
      text :=
        [ Primitives.Ability.static (entersWithCounters thisCreature (.lit 2) p1p1Counter),
          whenever (Primitives.GameEvent.does (Primitives.Subject.actor anOpponent) (Primitives.Instruction.commitCrime)) proliferate ],
      power := stat 0, toughness := stat 0 } }

/-- Galloping Lizrog -/
def gallopingLizrog : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Galloping Lizrog", cost := some [generic 3, pip .green, pip .blue],
      types := [.creature], subtypes := [creatureType "Frog", creatureType "Lizard"],
      text :=
        [ keyword (.named "trample"),
          when (Primitives.GameEvent.enters thisCreature none)
            (Primitives.Instruction.offer
              (Primitives.Instruction.removeCounters (some anyNumber) (some (Primitives.CounterKindSource.printed p1p1Counter))
                (among (allOf creatureYouControl)) .many true)
              (some (Primitives.Instruction.putCounters (times (.lit 2) removedThisWay) (Primitives.CounterKindSource.printed p1p1Counter) thisCreature .many true))
              none) ],
      power := stat 3, toughness := stat 3 } }

/-- Novijen Sages -/
def novijenSagesDraw : Ability :=
  activated
    (Primitives.Cost.compound
      [ Primitives.Cost.mana [generic 1],
        Primitives.Cost.perform (Primitives.Instruction.removeCounters (some (exactly 2)) (some (Primitives.CounterKindSource.printed p1p1Counter))
          (among (allOf creatureYouControl)) .many true) ])
    (Primitives.Instruction.draw (.lit 1))
theorem okNovijenSagesDraw : Ability.check (DRS.ofList []) novijenSagesDraw = [] := by decide
/-- Cyclone -/
def cycloneUpkeepPayment : Ability :=
  at_ (Primitives.GameEvent.beginningOf .upkeep (some Primitives.NounPhrase.you) (Primitives.TurnRef.each none))
    (Primitives.Instruction.sequentially
      [ Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "windCounter")) thisEnchantment .many true,
        doUnless (sacrifice thisEnchantment)
          (Primitives.Cost.scaled (Primitives.Cost.mana [pip .green])
            (times (.lit 1) (countersOn (.named "windCounter") thisEnchantment))) ])
theorem okCycloneUpkeepPayment : Ability.check (DRS.ofList []) cycloneUpkeepPayment = [] := by decide

def stormwildCapridor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stormwild Capridor", cost := some [generic 2, pip .white], types := [.creature],
      subtypes := [creatureType "Bird", creatureType "Goat"],
      text :=
        [ keyword (.named "flying"),
          Primitives.Ability.static (Primitives.StaticSpec.damageRule .noncombatOnly Primitives.DamageAgent.unattributed (Primitives.DamageScope.toRecipient thisCreature)
            (Primitives.DamageOp.prevent Primitives.PreventCut.all
              (some (Primitives.Instruction.putCounters preventedThisWay (Primitives.CounterKindSource.printed p1p1Counter) thisCreature .many true)))
            .repeatedly) ],
      power := stat 1, toughness := stat 3 } }

def testOfFaith : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Test of Faith", cost := some [generic 1, pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.establish
            (Primitives.StaticSpec.damageRule .any Primitives.DamageAgent.unattributed (Primitives.DamageScope.toRecipient (target creature))
              (Primitives.DamageOp.prevent (Primitives.PreventCut.shield (.lit 3))
                (some (Primitives.Instruction.putCounters preventedThisWay (Primitives.CounterKindSource.printed p1p1Counter) (that (.type .creature)) .many true)))
              .repeatedly)
            (some thisTurn)) ] } }

def temper : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Temper", cost := some [.variable, generic 1, pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.establish
            (Primitives.StaticSpec.damageRule .any Primitives.DamageAgent.unattributed (Primitives.DamageScope.toRecipient (target creature))
              (Primitives.DamageOp.prevent (Primitives.PreventCut.shield (Primitives.Amount.letter .x))
                (some (Primitives.Instruction.putCounters preventedThisWay (Primitives.CounterKindSource.printed p1p1Counter) (that (.type .creature)) .many true)))
              .repeatedly)
            (some thisTurn)) ] } }

def phytohydra : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Phytohydra", cost := some [generic 2, pip .green, pip .white, pip .white],
      types := [.creature], subtypes := [creatureType "Plant", creatureType "Hydra"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.instead (Primitives.WouldClause.would Primitives.Subject.any (Primitives.Instruction.dealDamage Primitives.Amount.anyAmount thisCreature .any)) (Primitives.Instruction.putCounters Primitives.Amount.thatMuch (Primitives.CounterKindSource.printed p1p1Counter) it .many true) none .repeatedly none) ],
      power := stat 1, toughness := stat 1 } }

def agelessEntity : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ageless Entity", cost := some [generic 3, pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text :=
        [ whenever (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Instruction.changeLife (Primitives.Delta.up Primitives.Amount.anyAmount))) (Primitives.Instruction.putCounters Primitives.Amount.thatMuch (Primitives.CounterKindSource.printed p1p1Counter) thisCreature .many true) ],
      power := stat 4, toughness := stat 4 } }

/-- Chromatic Armor -/
def chromaticArmor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chromatic Armor", cost := some [generic 1, pip .white, pip .blue],
      types := [.enchantment], subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") creature,
          Primitives.Ability.static (entersChoosing thisAura .color),
          Primitives.Ability.static (entersWithCounters thisAura (.lit 1) (.named "sleightCounter")),
          Primitives.Ability.static (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (allOf (Primitives.Predicate.and [source, ofTheLastChosen .color])))
            (Primitives.DamageScope.toRecipient (Primitives.NounPhrase.attachHost .enchanted (.type .creature))) (Primitives.DamageOp.prevent Primitives.PreventCut.all none) .repeatedly),
          activated (Primitives.Cost.mana [.variable])
            (Primitives.Instruction.sequentially
              [ Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "sleightCounter")) thisAura .many true,
                choose (a (quality .color)),
                Primitives.Instruction.define .x (countersOn (.named "sleightCounter") thisAura) ]) ] } }

def vault75MiddleSchool : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vault 75: Middle School", cost := some [generic 2, pip .white, pip .white],
      types := [.enchantment], subtypes := [enchantmentType "Saga"],
      text :=
        [ when (Primitives.GameEvent.chapterMark [1])
            (exile (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.compare [.stat .power] .atLeast (.lit 4)]))),
          when (Primitives.GameEvent.chapterMark [2, 3])
            (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter) (each creatureYouControl) .many true) ] } }

def keldonWarcaller : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Keldon Warcaller", cost := some [generic 1, pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Warrior"],
      text :=
        [ whenever (attacks thisCreature)
            (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "loreCounter"))
              (target (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (enchantmentType "Saga"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .many true) ],
      power := stat 2, toughness := stat 2 } }

def magistratesScepter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Magistrate's Scepter", cost := some [generic 3], types := [.artifact],
      text :=
        [ activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 4], Primitives.Cost.tapSymbol])
            (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "chargeCounter")) thisArtifact .many true),
          activated
            (Primitives.Cost.compound
              [ Primitives.Cost.tapSymbol,
                Primitives.Cost.perform (Primitives.Instruction.removeCounters (some (exactly 3)) (some (Primitives.CounterKindSource.printed (.named "chargeCounter")))
                  thisArtifact .many true) ])
            (extraTurn (.lit 1)) ] } }

def grumgully : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Grumgully, the Generous", cost := some [generic 1, pip .red, pip .green],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Goblin", creatureType "Shaman"],
      text :=
        [ Primitives.Ability.static (entersWithAdditionalCounters
            (each (Primitives.Predicate.and [ creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you,
                          Primitives.Predicate.not (Primitives.Predicate.hasSubtype (creatureType "Human")), Primitives.Predicate.otherThan thisCreature ]))
            (.lit 1) p1p1Counter) ],
      power := stat 3, toughness := stat 3 } }

def metallicMimic : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Metallic Mimic", cost := some [generic 2], types := [.artifact, .creature],
      subtypes := [creatureType "Shapeshifter"],
      text :=
        [ Primitives.Ability.static (entersChoosing thisCreature (.subtype .creature)),
          Primitives.Ability.static (Primitives.StaticSpec.qualityChange thisCreature .adds (Primitives.QualityPayload.chosenQuality (ofChosen (.subtype
              .creature)))),
          Primitives.Ability.static (entersWithAdditionalCounters
            (each (Primitives.Predicate.and [ creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, ofChosen (.subtype .creature),
                          Primitives.Predicate.otherThan thisCreature ]))
            (.lit 1) p1p1Counter) ],
      power := stat 2, toughness := stat 1 } }

def curatorBeastieLine : StaticSpec :=
  entersWithAdditionalCounters (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, colorless]))
    (.lit 2) p1p1Counter
theorem okCuratorBeastieLine : StaticSpec.check (DRS.ofList []) curatorBeastieLine = [] := by decide
def renataLine : StaticSpec :=
  entersWithAdditionalCounters
    (each (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, Primitives.Predicate.otherThan thisCreature])) (.lit 1)
    p1p1Counter
theorem okRenataLine : StaticSpec.check (DRS.ofList []) renataLine = [] := by decide
def entDraughtBasinAbility : Ability :=
  activatedOnlyDuring (Primitives.Cost.compound [Primitives.Cost.mana [.variable], Primitives.Cost.tapSymbol])
    (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter)
      (target (Primitives.Predicate.and [creature, Primitives.Predicate.compare [.stat .power] .eq (Primitives.Amount.letter .x)])) .many true)
    asSorcery
theorem okEntDraughtBasinAbility : Ability.check (DRS.ofList []) entDraughtBasinAbility = [] := by decide

def matopiGolem : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Matopi Golem", cost := some [generic 5], types := [.artifact, .creature],
      subtypes := [creatureType "Golem"],
      text :=
        [ activated (Primitives.Cost.mana [generic 1])
            (Primitives.Instruction.triggerThisWay (regenerate thisCreature) (regenerates thisCreature)
              (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed m1m1Counter) it .many true)) ],
      power := stat 3, toughness := stat 3 } }

def bramblewoodParagon : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bramblewood Paragon", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [creatureType "Elf", creatureType "Warrior"],
      text :=
        [ Primitives.Ability.static (entersWithAdditionalCounters
            (each (Primitives.Predicate.and [ creature, Primitives.Predicate.hasSubtype (creatureType "Warrior"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you,
                          Primitives.Predicate.otherThan thisCreature ]))
            (.lit 1) p1p1Counter),
          Primitives.Ability.static (Primitives.StaticSpec.abilityGrant
            (each (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, Primitives.Predicate.hasCounters (some p1p1Counter)]))
            (keyword (.named "trample"))) ],
      power := stat 2, toughness := stat 2 } }

def oonasBlackguard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Oona's Blackguard", cost := some [generic 1, pip .black], types := [.creature],
      subtypes := [creatureType "Faerie", creatureType "Rogue"],
      text :=
        [ keyword (.named "flying"),
          Primitives.Ability.static (entersWithAdditionalCounters
            (each (Primitives.Predicate.and [ creature, Primitives.Predicate.hasSubtype (creatureType "Rogue"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you,
                          Primitives.Predicate.otherThan thisCreature ]))
            (.lit 1) p1p1Counter),
          whenever
            (dealsCombatDamage
              (a (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, Primitives.Predicate.hasCounters (some p1p1Counter)]))
              (a Primitives.Predicate.anyPlayer))
            (discard (a (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.inZone (handOf (that .player))]))) ],
      power := stat 1, toughness := stat 1 } }

def crumblingAshes : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Crumbling Ashes", cost := some [generic 1, pip .black], types := [.enchantment],
      text :=
        [ at_ (Primitives.GameEvent.beginningOf .upkeep (some Primitives.NounPhrase.you) (Primitives.TurnRef.each none))
            (destroy (target (Primitives.Predicate.and [creature, Primitives.Predicate.hasCounters (some m1m1Counter)]))) ] } }

def hunterOfEyeblights : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hunter of Eyeblights", cost := some [generic 3, pip .black, pip .black],
      types := [.creature], subtypes := [creatureType "Elf", creatureType "Assassin"],
      text :=
        [ when (Primitives.GameEvent.enters thisCreature none)
            (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter) (target creatureYouDontControl) .many true),
          activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 2, pip .black], Primitives.Cost.tapSymbol])
            (destroy (target (Primitives.Predicate.and [creature, Primitives.Predicate.hasCounters none]))) ],
      power := stat 3, toughness := stat 3 } }

def pridemalkin : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Pridemalkin", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [creatureType "Cat"],
      text :=
        [ when (Primitives.GameEvent.enters thisCreature none)
            (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter) (target creatureYouControl) .many true),
          Primitives.Ability.static (Primitives.StaticSpec.abilityGrant
            (each (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, Primitives.Predicate.hasCounters (some p1p1Counter)]))
            (keyword (.named "trample"))) ],
      power := stat 2, toughness := stat 1 } }

def aboroth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aboroth", cost := some [generic 4, pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text :=
        [ cumulativeUpkeep (Primitives.Cost.perform (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed m1m1Counter) thisCreature .many true)) ],
      power := stat 9, toughness := stat 9 } }

def shelteringAncient : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sheltering Ancient", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [creatureType "Treefolk"],
      text :=
        [ keyword (.named "trample"),
          cumulativeUpkeep
            (Primitives.Cost.perform (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter)
              (a (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller anOpponent])) .many true)) ],
      power := stat 5, toughness := stat 5 } }

def coverOfWinterPut : Ability :=
  activated (Primitives.Cost.mana [.snow]) (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "ageCounter")) thisEnchantment .many true)
theorem okCoverOfWinterPut : Ability.check (DRS.ofList []) coverOfWinterPut = [] := by decide

def urborgScavengers : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Urborg Scavengers", cost := some [generic 2, pip .black], types := [.creature],
      subtypes := [creatureType "Spirit"],
      text :=
        [ triggeredOr (Primitives.GameEvent.enters thisCreature none) [attacks thisCreature]
            (Primitives.Instruction.sequentially
              [ exile (target (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.inZone graveyard])),
                Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter) thisCreature .many true ]),
          Primitives.Ability.alsoForKeywords
            (Primitives.Ability.static (onlyWhile (Primitives.StaticSpec.abilityGrant thisCreature (keyword (.named "flying")))
              (exists_ (Primitives.Predicate.and [Primitives.Predicate.exiledWith thisCreature, Primitives.Predicate.hasKeyword (.the (.named "flying"))]))))
            (["firstStrike", "doubleStrike", "deathtouch", "haste", "hexproof", "indestructible",
              "lifelink", "menace", "reach", "trample", "vigilance"].map (fun k => .the (.named k))) ],
      power := stat 2, toughness := stat 2 } }

def sengirVampire : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sengir Vampire", cost := some [generic 3, pip .black, pip .black], types := [.creature],
      subtypes := [creatureType "Vampire"],
      text :=
        [ keyword (.named "flying"),
          whenever (Primitives.GameEvent.dies (a (Primitives.Predicate.and [creature, happenedTo (Primitives.Subject.actor thisCreature) (Primitives.Instruction.dealDamage Primitives.Amount.anyAmount             (Primitives.NounPhrase.asMarker .permanent (relative .object)) .any) soFarThisTurn])))
            (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter) thisCreature .many true) ],
      power := stat 4, toughness := stat 4 } }

def mildManneredLibrarian : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mild-Mannered Librarian", cost := some [pip .green], types := [.creature],
      subtypes := [creatureType "Human"],
      text :=
        [ activatedOnlyOnce (Primitives.Cost.mana [generic 3, pip .green])
            (Primitives.Instruction.sequentially
              [ Primitives.Instruction.establish
                  (Primitives.StaticSpec.qualityChange thisCreature .sets
                    (Primitives.QualityPayload.bundle { characteristics := { subtypes := [creatureType "Werewolf"] } } none))
                  none,
                Primitives.Instruction.putCounters (.lit 2) (Primitives.CounterKindSource.printed p1p1Counter) it .many true,
                Primitives.Instruction.draw (.lit 1) ])
            oncePerGame ],
      power := stat 1, toughness := stat 1 } }

def infernalVessel : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Infernal Vessel", cost := some [generic 2, pip .black], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Cleric"],
      text :=
        [ triggeredIf (Primitives.GameEvent.dies thisCreature) (Primitives.Condition.not (Primitives.Condition.matches it (Primitives.Predicate.hasSubtype (creatureType "Demon"))))
            (Primitives.Instruction.sequentially
              [ returnToBattlefieldWithCounters it (ownerOf it) (.lit 2) p1p1Counter,
                become it { characteristics := { subtypes := [creatureType "Demon"] } } none ]) ],
      power := stat 2, toughness := stat 1 } }

/-- Vivien's Talent -/
def viviensTalentTrigger : Ability :=
  whenever (Primitives.GameEvent.enters (a (Primitives.Predicate.and [nontoken, creatureYouControl])) none)
    (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "loyaltyCounter")) (Primitives.NounPhrase.attachHost .enchanted (.type .planeswalker)) .many true)
theorem okViviensTalentTrigger : Ability.check (DRS.ofList []) viviensTalentTrigger = [] := by decide
/-- Bioessence Hydra -/
def bioessenceHydraTrigger : Ability :=
  whenever
    (counterEvent .put (.named "loyaltyCounter") .many
      (allOf (Primitives.Predicate.and [Primitives.Predicate.hasType .planeswalker, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))
    (Primitives.Instruction.putCounters Primitives.Amount.thatMuch (Primitives.CounterKindSource.printed p1p1Counter) thisCreature .many true)
theorem okBioessenceHydraTrigger : Ability.check (DRS.ofList []) bioessenceHydraTrigger = [] := by decide
/-- Vraska, Betrayal's Sting -/
def vraskaBetrayalsStingUltimate : Instruction :=
  Primitives.Instruction.doIf (Primitives.Condition.compareAmt (countersOn (.named "poison") (target Primitives.Predicate.anyPlayer)) .less (.lit 9))
    (Primitives.Instruction.putCounters Primitives.Amount.theDifference (Primitives.CounterKindSource.printed (.named "poison")) they .many true) none
theorem okVraskaBetrayalsStingUltimate :
    Instruction.check (DRS.ofList []) vraskaBetrayalsStingUltimate = [] := by decide
/-- Vexing Puzzlebox -/
def vexingPuzzleboxCounters : Ability :=
  whenever (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Instruction.rollDice Primitives.Amount.anyAmount .anyDie .many Primitives.RollWatch.anyResult))
    (Primitives.Instruction.putCounters (Primitives.Amount.theOutcome .rollResult) (Primitives.CounterKindSource.printed (.named "chargeCounter")) thisArtifact .many true)
theorem okVexingPuzzleboxCounters : Ability.check (DRS.ofList []) vexingPuzzleboxCounters = [] := by decide
def spaceFamilyGoblinsonRoll : Ability :=
  whenever (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Instruction.rollDice Primitives.Amount.anyAmount .anyDie .one Primitives.RollWatch.anyResult))
    (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter) thisCreature .many true)
theorem okSpaceFamilyGoblinsonRoll : Ability.check (DRS.ofList []) spaceFamilyGoblinsonRoll = [] := by decide
/-- Atomwheel Acrobats -/
def atomwheelAcrobatsRoll : Ability :=
  whenever (youRollResultIn (Primitives.Quantity.range (some 1) (some 2)))
    (Primitives.Instruction.putCounters Primitives.Amount.thatMuch (Primitives.CounterKindSource.printed p1p1Counter) thisCreature .many true)
theorem okAtomwheelAcrobatsRoll : Ability.check (DRS.ofList []) atomwheelAcrobatsRoll = [] := by decide
/-- Resolute Veggiesaur -/
def resoluteVeggiesaurThirdDie : Ability :=
  whenever (Primitives.GameEvent.nthOccurrence (.nth 3) (some .turn) (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Instruction.rollDice Primitives.Amount.anyAmount .anyDie .one Primitives.RollWatch.anyResult)))
    (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter) thisCreature .many true)
theorem okResoluteVeggiesaurThirdDie : Ability.check (DRS.ofList []) resoluteVeggiesaurThirdDie = [] := by decide
/-- Run the Play (Striding Shotcaller's other half) -/
def runThePlayCounters : Instruction :=
  Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter)
    (Primitives.NounPhrase.eachOf (Primitives.NounPhrase.described (Primitives.DetPhrase.target (Primitives.Quantity.upToOf (Primitives.Amount.letter .x))) creature)) .many true
theorem okRunThePlayCounters : Instruction.check (DRS.ofList [letterB .x]) runThePlayCounters = [] := by decide
/-- Font of Agonies -/
def fontOfAgoniesTrigger : Ability :=
  whenever (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Instruction.pay (Primitives.Cost.perform (Primitives.Instruction.changeLife (Primitives.Delta.down Primitives.Amount.anyAmount))) .once .paid none none)) (Primitives.Instruction.putCounters Primitives.Amount.thatMuch (Primitives.CounterKindSource.printed (.named "bloodCounter")) thisEnchantment .many true)
theorem okFontOfAgoniesTrigger : Ability.check (DRS.ofList []) fontOfAgoniesTrigger = [] := by decide
/-- Heart of Kiran -/
def heartOfKiranCrewAltCost : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.altCost (allOf (Primitives.Predicate.and [Primitives.Predicate.abilityHead (.keyword (.named "crew")), Primitives.Predicate.abilityOf Primitives.NounPhrase.this]))
    (some (Primitives.Cost.perform (Primitives.Instruction.removeCounters (some (exactly 1)) (some (Primitives.CounterKindSource.printed (.named "loyaltyCounter")))
      (a (Primitives.Predicate.and [Primitives.Predicate.hasType .planeswalker, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .many true))))
theorem okHeartOfKiranCrewAltCost : Ability.check (DRS.ofList []) heartOfKiranCrewAltCost = [] := by decide
/-- Tainted Adversary -/
def taintedAdversaryOffer : Ability :=
  when (Primitives.GameEvent.enters thisCreature none)
    (Primitives.Instruction.triggerReflexively (offer (Primitives.Instruction.pay (Primitives.Cost.mana [generic 2, pip .black]) .anyNumberOfTimes .paid none none))
      (Primitives.Instruction.putCounters Primitives.Amount.thatMuch (Primitives.CounterKindSource.printed p1p1Counter) thisCreature .many true))
theorem okTaintedAdversaryOffer : Ability.check (DRS.ofList []) taintedAdversaryOffer = [] := by decide
/-- Korvold, Gleeful Glutton -/
def korvoldCombatTrigger : Ability :=
  whenever (dealsCombatDamage thisCreature (a Primitives.Predicate.anyPlayer))
    (Primitives.Instruction.sequentially
      [ Primitives.Instruction.putCounters (Primitives.Amount.letter .x) (Primitives.CounterKindSource.printed p1p1Counter) thisCreature .many true,
        Primitives.Instruction.draw (Primitives.Amount.letter .x),
        Primitives.Instruction.define .x (Primitives.Amount.distinctCount .permanentType (allOf (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)]))) ])
theorem okKorvoldCombatTrigger : Ability.check (DRS.ofList []) korvoldCombatTrigger = [] := by decide
/-- Mirelurk Queen -/
def mirelurkQueenTrigger : Ability :=
  triggeredOnlyOnce
    (Primitives.GameEvent.does Primitives.Subject.any (Semantics.Instruction.ownScope (some (.action "mill"))
      (Semantics.Instruction.enact (.action "mill")
        (Primitives.Instruction.move (Semantics.NounPhrase.inCaller ((counted (atLeast 1) (Primitives.Predicate.and [Primitives.Predicate.not land,
        Primitives.Predicate.inZone (Primitives.ZoneExpr.zone .library .bare)]))))
          (Primitives.EventSource.zones [Primitives.ZoneExpr.zone .library .bare])
          (some graveyard) .arrival none []) none none)))
    oncePerTurn
    (Primitives.Instruction.sequentially [Primitives.Instruction.draw (.lit 1), Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter)
        thisCreature .many true])
theorem okMirelurkQueenTrigger : Ability.check (DRS.ofList []) mirelurkQueenTrigger = [] := by decide

/-- Whirling Dervish -/
def whirlingDervish : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Whirling Dervish", cost := some [pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Monk"],
      text :=
        [ keywordQuality (.named "protection") (Primitives.Predicate.colorIs .black),
          triggeredIf (Primitives.GameEvent.beginningOf .endStep (some (each Primitives.Predicate.anyPlayer)) (Primitives.TurnRef.each none))
            (happened (Primitives.Instruction.dealDamage Primitives.Amount.anyAmount anOpponent .any) (Primitives.Subject.actor thisCreature) soFarThisTurn)
            (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter) it .many true) ],
      power := stat 1, toughness := stat 1 } }

/-- Ichor Shade -/
def ichorShade : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ichor Shade", cost := some [generic 2, pip .black], types := [.creature],
      subtypes := [creatureType "Phyrexian", creatureType "Shade"],
      text :=
        [ triggeredIf (Primitives.GameEvent.beginningOf .endStep (some Primitives.NounPhrase.you) (Primitives.TurnRef.each none))
            (Primitives.Condition.did (⟨Primitives.Subject.any, (Primitives.Instruction.move (a (Primitives.Predicate.or [artifact, creature])) (Primitives.EventSource.zones [battlefield]) (some graveyard) .departure none []), (Primitives.LookbackSpan.duration soFarThisTurn none)⟩))
            (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter) thisCreature .many true) ],
      power := stat 2, toughness := stat 3 } }

/-- Asmira, Holy Avenger -/
def asmiraHolyAvenger : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Asmira, Holy Avenger", cost := some [generic 2, pip .green, pip .white],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Cleric"],
      text :=
        [ keyword (.named "flying"),
          at_ (Primitives.GameEvent.beginningOf .endStep (some (each Primitives.Predicate.anyPlayer)) (Primitives.TurnRef.each none))
            (Primitives.Instruction.putCounters
              (Primitives.Amount.tally (⟨Primitives.Subject.any, (Primitives.Instruction.move (a creature) (Primitives.EventSource.zones
                  [battlefield]) (some (graveyardOf Primitives.NounPhrase.you)) .departure none []), (Primitives.LookbackSpan.duration soFarThisTurn none)⟩) .events)
              (Primitives.CounterKindSource.printed p1p1Counter) thisCreature .many true) ],
      power := stat 2, toughness := stat 3 } }

/-- Thought Sponge -/
def thoughtSponge : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Thought Sponge", cost := some [generic 3, pip .blue], types := [.creature],
      subtypes := [creatureType "Sponge"],
      text :=
        [ keyword (.named "flash"),
          Primitives.Ability.static (entersWithCounters thisCreature greatestCardsAnOpponentDrew p1p1Counter),
          when (Primitives.GameEvent.dies thisCreature) (Primitives.Instruction.draw (Primitives.Amount.statOf (.stat .power) thisCreature)) ],
      power := stat 1, toughness := stat 1 } }

/-- Skeleton Ship -/
def skeletonShip : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Skeleton Ship", cost := some [generic 3, pip .blue, pip .black],
      supertypes := [.legendary], types := [.creature], subtypes := [creatureType "Skeleton"],
      text :=
        [ when
            (Primitives.GameEvent.stateHolds
              (Primitives.Condition.not (exists_ (Primitives.Predicate.and [land, Primitives.Predicate.hasSubtype (landType "Island"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))))
            (sacrifice thisCreature),
          activated Primitives.Cost.tapSymbol (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed m1m1Counter) (target creature) .many true) ],
      power := stat 0, toughness := stat 3 } }

/-- Contractual Safeguard -/
def contractualSafeguardPass : Instruction :=
  Primitives.Instruction.sequentially
    [ choose (a (Primitives.Predicate.counterKindOn (a creatureYouControl))),
      Primitives.Instruction.putCounters (.lit 1) Primitives.CounterKindSource.bound (each (otherCreatureYouControl it)) .many true ]
theorem okContractualSafeguardPass : Instruction.check (DRS.ofList []) contractualSafeguardPass = [] := by decide

/-- Feral Contest -/
def feralContest : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Feral Contest", cost := some [generic 3, pip .green], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter) (target creatureYouControl) .many true,
              requireBlockIt (target (Primitives.Predicate.and [creature, Primitives.Predicate.other])) (some thisTurn) ]) ] } }

private semantic_template companyRecipient : NounPhrase :=
  .firstMention "thranduilsCompany.recipient" (target creatureYouControl)
private semantic_template companyRecipientAgain : NounPhrase :=
  .laterMention "thranduilsCompany.recipient" (target creatureYouControl)

/-- Thranduil's Company -/
def thranduilsCompany : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Thranduil's Company", cost := some [generic 2, pip .green, pip .blue],
      types := [.creature], subtypes := [creatureType "Elf", creatureType "Soldier"],
      text :=
        [ Primitives.Ability.static (onlyWhile (mayPlayAdditionalLands Primitives.NounPhrase.you (exactly 1))
            (exists_ (Primitives.Predicate.and [ otherCreature thisCreature, Primitives.Predicate.hasSubtype (creatureType "Elf"),
                             Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you ]))),
          abilityWord "landfall"
            (whenever (Primitives.GameEvent.enters (a (Primitives.Predicate.and [land, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) none)
              (Primitives.Instruction.sequentially
                [ Primitives.Instruction.putCounters (.lit 2) (Primitives.CounterKindSource.printed p1p1Counter)
                    companyRecipient .many true,
                  gain
                    companyRecipientAgain
                    (keyword (.named "vigilance")) (some untilEndOfTurn) ])) ],
      power := stat 3, toughness := stat 4 } }

private semantic_template stunningShotRecipient : NounPhrase :=
  .firstMention "stunningShot.tappedCreature"
    (.described (.target (upTo 1)) (.and [creature, .hasPossessor .controller anOpponent]))
private semantic_template stunningShotRecipientAgain : NounPhrase :=
  .laterMention "stunningShot.tappedCreature"
    (.described (.target (upTo 1)) (.and [creature, .hasPossessor .controller anOpponent]))

/-- Stunning Shot -/
def stunningShot : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stunning Shot", cost := some [generic 1, pip .white], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ Primitives.Instruction.putCounters (.lit 2) (Primitives.CounterKindSource.printed p1p1Counter)
                (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 1)) creatureYouControl) .many true,
              tap stunningShotRecipient,
              Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "stunCounter"))
                stunningShotRecipientAgain .many true ]) ] } }

/-- Stonebinder's Familiar -/
def stonebindersFamiliar : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stonebinder's Familiar", cost := some [pip .white], types := [.creature],
      subtypes := [creatureType "Spirit", creatureType "Dog"],
      text :=
        [ Primitives.Ability.triggered (Primitives.GameEvent.putInto (counted (atLeast 1) Primitives.Predicate.isCard) exileZone none) [] none []
            (some (duringPart .turn (some Primitives.NounPhrase.you))) (some oncePerTurn) none
            (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter) thisCreature .many true) ],
      power := stat 1, toughness := stat 1 } }

/-- Nazgûl -/
def nazgulRingTrigger : Ability :=
  whenever (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Declared.theRingTemptsYou))
    (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter)
      (each (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (creatureType "Wraith"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .many true)
theorem okNazgulRingTrigger : Ability.check (DRS.ofList []) nazgulRingTrigger = [] := by decide

/-- Captain Marvel, Apex Avenger -/
def captainMarvelApexAvenger : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Captain Marvel, Apex Avenger", cost := some [generic 5, pip .red, pip .white],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Kree", creatureType "Hero"],
      text :=
        [ keyword (.named "flying"), keyword (.named "doubleStrike"), keyword (.named "indestructible"),
          triggeredIf
            (Primitives.GameEvent.does (Primitives.Subject.by Primitives.NounPhrase.you) (Primitives.Instruction.putCounters Primitives.Amount.anyAmount Primitives.CounterKindSource.anyKind (a (Primitives.Predicate.and [creature, Primitives.Predicate.otherThan Primitives.NounPhrase.this])) .many false))
            (Primitives.Condition.matches it (Primitives.Predicate.not (Primitives.Predicate.hasSubtype (creatureType "Kree"))))
            (offer (Primitives.Instruction.putCounters Primitives.Amount.thatMuch Primitives.CounterKindSource.those Primitives.NounPhrase.this .many true)) ],
      power := stat 4, toughness := stat 4 } }

/-- Blood Spatter Analysis -/
def bloodSpatterAnalysis : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blood Spatter Analysis", cost := some [pip .black, pip .red], types := [.enchantment],
      text :=
        [ when (Primitives.GameEvent.enters thisEnchantment none)
            (instruct thisEnchantment (Primitives.Instruction.dealDamage (.lit 3) (target (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller anOpponent])) .noncombatOnly)),
          whenever (Primitives.GameEvent.dies (counted (atLeast 1) creature))
            (Primitives.Instruction.sequentially
              [ mill (.lit 1) Primitives.NounPhrase.you,
                Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "bloodstainCounter")) thisEnchantment .many true,
                Primitives.Instruction.triggerReflexively
                  (Primitives.Instruction.doOnlyIf (sacrifice thisEnchantment)
                    (Primitives.Condition.compareAmt (countersOn (.named "bloodstainCounter") thisEnchantment) .atLeast (.lit 5))
                    none)
                  (move (target (Primitives.Predicate.and [creature, Primitives.Predicate.isCard, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)])) hand) ]) ] } }

/-- Bewitching Leechcraft -/
def bewitchingLeechcraft : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bewitching Leechcraft", cost := some [generic 1, pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") creature,
          when (Primitives.GameEvent.enters thisAura none) (tap (Primitives.NounPhrase.attachHost .enchanted (.type .creature))),
          Primitives.Ability.static (Primitives.StaticSpec.abilityGrant (Primitives.NounPhrase.attachHost .enchanted (.type .creature))
            (Primitives.Ability.static (Primitives.StaticSpec.instead (Primitives.WouldClause.would Primitives.Subject.any (Primitives.Instruction.setStatus .untapped thisCreature)) (Primitives.Instruction.doIfDone
                (Primitives.Instruction.removeCounters (some (exactly 1)) (some (Primitives.CounterKindSource.printed p1p1Counter)) thisCreature .many true)
                (some (untap thisCreature)) none) (some (duringPart .untapStep (some Primitives.NounPhrase.you))) .repeatedly none))) ] } }

def agentOfTheShadowThievesGrantedTrigger : Ability :=
  Primitives.Ability.triggered (Primitives.GameEvent.does (Primitives.Subject.actor thisCreature) (Primitives.Instruction.attack (some (a Primitives.Predicate.anyPlayer)))) [] none [] none none
    (some (Primitives.Condition.not (exists_ (Primitives.Predicate.and [ Primitives.Predicate.opponent,
                                 Primitives.Predicate.compare [.playerStat .lifeTotal] .greater
                                   (lifeTotalOf (that .player)) ]))))
    (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter) thisCreature .many true)
theorem okAgentOfTheShadowThievesGrantedTrigger :
    Ability.check (DRS.ofList []) agentOfTheShadowThievesGrantedTrigger = [] := by decide

/-- Bloodline Pretender -/
def bloodlinePretender : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bloodline Pretender", cost := some [generic 3], types := [.artifact, .creature],
      subtypes := [creatureType "Shapeshifter"],
      text :=
        [ keyword (.named "changeling"),
          Primitives.Ability.static (entersChoosing thisCreature (.subtype .creature)),
          whenever
            (Primitives.GameEvent.enters (a (Primitives.Predicate.and [ creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, Primitives.Predicate.otherThan thisCreature,
                                ofChosen (.subtype .creature) ])) none)
            (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter) thisCreature .many true) ],
      power := stat 2, toughness := stat 2 } }

/-- Chamber Sentry -/
def chamberSentryDamage : Ability :=
  activated
    (Primitives.Cost.compound
      [ Primitives.Cost.mana [.variable], Primitives.Cost.tapSymbol,
        Primitives.Cost.perform (Primitives.Instruction.removeCounters (some (Primitives.Quantity.exactlyOf (Primitives.Amount.letter .x))) (some (Primitives.CounterKindSource.printed p1p1Counter))
          thisCreature .many true) ])
    (instruct Primitives.NounPhrase.this (Primitives.Instruction.dealDamage (Primitives.Amount.letter .x) (target anyTarget) .noncombatOnly))
theorem okChamberSentryDamage : Ability.check (DRS.ofList []) chamberSentryDamage = [] := by decide

/-- Menacing Ogre -/
def menacingOgre : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Menacing Ogre", cost := some [generic 3, pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Ogre"],
      text :=
        [ keyword (.named "trample"), keyword (.named "haste"),
          when (Primitives.GameEvent.enters thisCreature none)
            (Primitives.Instruction.sequentially
              [ instruct (each Primitives.Predicate.anyPlayer) (choose (disclosure := .secretly) (a (quality .number))),
                Primitives.Instruction.revealChoices .numbers,
                instruct (each (Primitives.Predicate.and [Primitives.Predicate.anyPlayer, Primitives.Predicate.choseExtreme .max])) (loseLife Primitives.Amount.thatMuch),
                doIf (Primitives.Condition.matches Primitives.NounPhrase.you (Primitives.Predicate.choseExtreme .max))
                  (Primitives.Instruction.putCounters (.lit 2) (Primitives.CounterKindSource.printed p1p1Counter) thisCreature .many true) ]) ],
      power := stat 3, toughness := stat 3 } }

/-- Charnel Troll -/
def charnelTroll : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Charnel Troll", cost := some [generic 1, pip .black, pip .green], types := [.creature],
      subtypes := [creatureType "Troll"],
      text :=
        [ keyword (.named "trample"),
          at_ (Primitives.GameEvent.beginningOf .upkeep (some Primitives.NounPhrase.you) (Primitives.TurnRef.each none))
            (Primitives.Instruction.doIfDone (exile (a (Primitives.Predicate.and [creature, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)])))
              (some (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter) thisCreature .many true))
              (some (sacrifice thisCreature))),
          activated
            (Primitives.Cost.compound
              [ Primitives.Cost.mana [pip .black, pip .green],
                Primitives.Cost.perform (discard (a (Primitives.Predicate.and [creature, Primitives.Predicate.inZone yourHand]))) ])
            (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter) thisCreature .many true) ],
      power := stat 4, toughness := stat 4 } }

/-- Mistbreath Elder -/
def mistbreathElder : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mistbreath Elder", cost := some [pip .green], types := [.creature],
      subtypes := [creatureType "Frog", creatureType "Warrior"],
      text :=
        [ at_ (Primitives.GameEvent.beginningOf .upkeep (some Primitives.NounPhrase.you) (Primitives.TurnRef.each none))
            (Primitives.Instruction.doIfDone (returnTo (a (otherCreatureYouControl thisCreature)) hand [])
              (some (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter) thisCreature .many true))
              (some (offer (returnTo thisCreature hand [])))) ],
      power := stat 2, toughness := stat 2 } }

/-- Dark Depths -/
def darkDepths : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dark Depths", supertypes := [.legendary, .snow], types := [.land],
      text :=
        [ Primitives.Ability.static (entersWithCounters thisLand (.lit 10) (.named "iceCounter")),
          activated (Primitives.Cost.mana [generic 3])
            (removeCounters (exactly 1) (some (Primitives.CounterKindSource.printed (.named "iceCounter"))) thisLand),
          when (Primitives.GameEvent.stateHolds (Primitives.Condition.not (Primitives.Condition.matches thisLand (Primitives.Predicate.hasCounters (some (.named "iceCounter"))))))
            (Primitives.Instruction.doIfDone (sacrifice thisLand) (some (create (.lit 1) maritLage)) none)
                ] } }

/-- Thallid -/
def thallid : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Thallid", cost := some [pip .green], types := [.creature],
      subtypes := [creatureType "Fungus"],
      text :=
        [ at_ (Primitives.GameEvent.beginningOf .upkeep (some Primitives.NounPhrase.you) (Primitives.TurnRef.each none))
            (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "sporeCounter")) thisCreature .many true),
          activated
            (Primitives.Cost.perform (Primitives.Instruction.removeCounters (some (exactly 3)) (some (Primitives.CounterKindSource.printed (.named "sporeCounter"))) thisCreature .many true))
            (create (.lit 1) (creatureToken 1 1 [.green] [creatureType "Saproling"])) ],
      power := stat 1, toughness := stat 1 } }

/-- Aether Chaser -/
def aetherChaserEnergy : Ability :=
  when (Primitives.GameEvent.enters thisCreature none) (Primitives.Instruction.putCounters (.lit 2) (Primitives.CounterKindSource.printed (.named "energy")) Primitives.NounPhrase.you .many true)
theorem okAetherChaserEnergy : Ability.check (DRS.ofList []) aetherChaserEnergy = [] := by decide
/-- Archfiend of the Dross -/
def archfiendOfTheDrossEntersOiled : StaticSpec :=
  entersWithCounters thisCreature (.lit 4) (.named "oilCounter")
theorem okArchfiendOfTheDrossEntersOiled :
    StaticSpec.check (DRS.ofList []) archfiendOfTheDrossEntersOiled = [] := by decide
/-- Archfiend of the Dross -/
def archfiendOfTheDrossOilUpkeep : Ability :=
  at_ (Primitives.GameEvent.beginningOf .upkeep (some Primitives.NounPhrase.you) (Primitives.TurnRef.each none))
    (Primitives.Instruction.removeCounters (some (exactly 1)) (some (Primitives.CounterKindSource.printed (.named "oilCounter"))) thisCreature .many true)
theorem okArchfiendOfTheDrossOilUpkeep : Ability.check (DRS.ofList []) archfiendOfTheDrossOilUpkeep = [] := by
  decide
def neurokTransmuter : Instruction :=
  become (target creature) { characteristics := { types := [.artifact] } } (some untilEndOfTurn)
theorem okNeurokTransmuter : Instruction.check (DRS.ofList []) neurokTransmuter = [] := by decide
def syphonMind : Instruction :=
  Primitives.Instruction.sequentially
    [ Primitives.Instruction.sequentially [instruct (each otherPlayer) (choose (a (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.inZone hand]))), discard them],
      Primitives.Instruction.doForEach (theVerbed (.action "discard") .many (some .card) .thisWay) (Primitives.Instruction.draw (.lit 1)) ]
theorem okSyphonMind : Instruction.check (DRS.ofList []) syphonMind = [] := by decide
def peek : Instruction := Primitives.Instruction.sequentially [lookAtHandOf (target Primitives.Predicate.anyPlayer), Primitives.Instruction.draw (.lit 1)]
theorem okPeek : Instruction.check (DRS.ofList []) peek = [] := by decide
/-- Bumi, King of Three Trials -/
def bumiScryMode : Instruction := instruct (target Primitives.Predicate.anyPlayer) (scry (.lit 3))
theorem okBumiScryMode : Instruction.check (DRS.ofList []) bumiScryMode = [] := by decide
/-- Final Act -/
def finalActCounterMode : Instruction := loseAllCounters none (agent := (each Primitives.Predicate.opponent))
theorem okFinalActCounterMode : Instruction.check (DRS.ofList []) finalActCounterMode = [] := by decide
def leeches : Instruction :=
  loseAllCounters (some (Primitives.CounterKindSource.printed (.named "poison"))) (agent := (target Primitives.Predicate.anyPlayer))
theorem okLeeches : Instruction.check (DRS.ofList []) leeches = [] := by decide

/-- Woeleecher -/
def woeleecherWhole : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Woeleecher", cost := some [generic 5, pip .white], types := [.creature],
      subtypes := [creatureType "Elemental"], text := [woeleecher],
      power := stat 3, toughness := stat 5 } }

end Semantics.Cards

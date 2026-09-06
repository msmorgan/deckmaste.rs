import Semantics
import Semantics.Macros
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
  Primitives.Instruction.offer (sacrifice (a creature) (agent := Primitives.NounPhrase.you))
    (some (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne) thisCreature))
    (some (Primitives.Instruction.removeCounters (some (exactly 1)) (some (Primitives.CounterKindSource.printed plusOnePlusOne)) thisCreature)) (agent
        := Primitives.NounPhrase.you)
theorem okCrovaxTheCursed : Instruction.check [] crovaxTheCursed = [] := by decide
def additiveEvolution : Instruction :=
  Primitives.Instruction.sequentially
    [ create (.lit 1) (creatureToken 0 0 [.green, .blue] [creatureType "Fractal"]),
      Primitives.Instruction.putCounters (.lit 3) (Primitives.CounterKindSource.printed plusOnePlusOne) it ]
theorem okAdditiveEvolution : Instruction.check [] additiveEvolution = [] := by decide
def battlegrowth : Instruction := Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne) (target creature)
theorem okBattlegrowth : Instruction.check [] battlegrowth = [] := by decide
def chainbreaker : Instruction :=
  Primitives.Instruction.removeCounters (some (exactly 1)) (some (Primitives.CounterKindSource.printed minusOneMinusOne)) (target creature)
theorem okChainbreaker : Instruction.check [] chainbreaker = [] := by decide
def kaitoBaneOfNightmares : Instruction :=
  Primitives.Instruction.sequentially
    [Primitives.Instruction.setStatus .tapped (target creature), Primitives.Instruction.putCounters (.lit 2) (Primitives.CounterKindSource.printed (.named "Stun")) it]
theorem okKaitoBaneOfNightmares : Instruction.check [] kaitoBaneOfNightmares = [] := by decide
def jhoiraOfTheGhitu : Ability :=
  activated
    (Primitives.Cost.compound [Primitives.Cost.mana [generic 2], Primitives.Cost.perform (exile (a (Primitives.Predicate.and [Primitives.Predicate.not land, Primitives.Predicate.inZone (handOf Primitives.NounPhrase.you)])))])
    (Primitives.Instruction.putCounters (.lit 4) (Primitives.CounterKindSource.printed (.named "Time"))
      (theVerbed (.action "Exile") .card .attributive .one))
theorem okJhoiraOfTheGhitu : Ability.check [] jhoiraOfTheGhitu = [] := by decide
def alaundoTheSeer : Instruction :=
  Primitives.Instruction.removeCounters (some (exactly 1)) (some (Primitives.CounterKindSource.printed (.named "Time")))
    (each (Primitives.Predicate.and [Primitives.Predicate.hasPossessor .owner Primitives.NounPhrase.you, Primitives.Predicate.inZone exileZone]))
theorem okAlaundoTheSeer : Instruction.check [] alaundoTheSeer = [] := by decide

def daydream : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Daydream", cost := some [pip .white], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ exile (target creatureYouControl),
              returnToBattlefieldWithCounters (that .card) (ownerOf (that .card)) (.lit 1)
                plusOnePlusOne ]),
          keywordCosting "Flashback" (Primitives.Cost.mana [generic 2, pip .white]) ] } }

def ashnodsTransmogrant : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.tapSymbol, Primitives.Cost.perform (sacrifice thisArtifact (agent := Primitives.NounPhrase.you))])
    (Primitives.Instruction.sequentially
      [ Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne) (target (Primitives.Predicate.and [creature, Primitives.Predicate.not artifact])),
        become (that (.type .creature)) { characteristics := { types := [.artifact] } } none ])
theorem okAshnodsTransmogrant : Ability.check [] ashnodsTransmogrant = [] := by decide
def azulaAlwaysLies : Instruction :=
  chooseModes (oneThrough 2)
    [ get (target creature) (Primitives.Delta.down (.lit 1)) (Primitives.Delta.down (.lit 1)) (some untilEndOfTurn),
      Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne) (target creature) ]
theorem okAzulaAlwaysLies : Instruction.check [] azulaAlwaysLies = [] := by decide
def bellowingAegisaur : Instruction :=
  Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne) (each (otherCreatureYouControl thisCreature))
theorem okBellowingAegisaur : Instruction.check [] bellowingAegisaur = [] := by decide
def carnifexDemon : Instruction :=
  Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed minusOneMinusOne) (each (otherCreature thisCreature))
theorem okCarnifexDemon : Instruction.check [] carnifexDemon = [] := by decide
def ajaniAdversaryOfTyrants : Instruction :=
  Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne) (Primitives.NounPhrase.eachOf (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 2)) creature))
theorem okAjaniAdversaryOfTyrants : Instruction.check [] ajaniAdversaryOfTyrants = [] := by decide
def naturesPanoply : Instruction :=
  Primitives.Instruction.sequentially
    [ choose (Primitives.NounPhrase.described (Primitives.DetPhrase.target anyNumber) creature),
      Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne) (Primitives.NounPhrase.eachOf them) ]
theorem okNaturesPanoply : Instruction.check [] naturesPanoply = [] := by decide
def armamentCorps : Instruction :=
  distributeCounters (.lit 2) plusOnePlusOne
    (Primitives.NounPhrase.described (Primitives.DetPhrase.target (oneThrough 2)) creatureYouControl)
theorem okArmamentCorps : Instruction.check [] armamentCorps = [] := by decide
def savagebornHydra : Ability :=
  activatedOnlyDuring (Primitives.Cost.mana [generic 1, hybridPip .red .green])
    (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne) thisCreature) Primitives.Timing.asSorcery
theorem okSavagebornHydra : Ability.check [] savagebornHydra = [] := by decide
def woeleecher : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [pip .white], Primitives.Cost.tapSymbol])
    (Primitives.Instruction.doIfDone
      (Primitives.Instruction.removeCounters (some (exactly 1)) (some (Primitives.CounterKindSource.printed minusOneMinusOne)) (target creature))
      (some (gainLife (.lit 2) (agent := Primitives.NounPhrase.you))) none)
theorem okWoeleecher : Ability.check [] woeleecher = [] := by decide
def anointerOfValor : Ability :=
  whenever (attacks (a creature))
    (offerWhen (Primitives.Instruction.pay (Primitives.Cost.mana [generic 3]) .once (agent := Primitives.NounPhrase.you))
      (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne) (that (.type .creature))) (agent := Primitives.NounPhrase.you))
theorem okAnointerOfValor : Ability.check [] anointerOfValor = [] := by decide
def avianOddity : Instruction :=
  Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed flyingCounter) (target creatureYouControl)
theorem okAvianOddity : Instruction.check [] avianOddity = [] := by decide
/-- Song of Eärendil -/
def songOfEarendil : Instruction :=
  Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed flyingCounter)
    (each (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, Primitives.Predicate.not (Primitives.Predicate.hasKeyword (.the "Flying"))]))
theorem okSongOfEarendil : Instruction.check [] songOfEarendil = [] := by decide
def gideonsAvenger : Ability :=
  whenever (Primitives.GameEvent.statusEvent (a (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller anOpponent])) .tapped)
    (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne) thisCreature)
theorem okGideonsAvenger : Ability.check [] gideonsAvenger = [] := by decide
def prologueToPhyresis : Instruction :=
  Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "Poison")) (each Primitives.Predicate.opponent)
theorem okPrologueToPhyresis : Instruction.check [] prologueToPhyresis = [] := by decide
def screechingScorchbeast : Ability :=
  whenever (attacks thisCreature) (Primitives.Instruction.putCounters (.lit 2) (Primitives.CounterKindSource.printed (.named "Rad")) (each Primitives.Predicate.anyPlayer))
theorem okScreechingScorchbeast : Ability.check [] screechingScorchbeast = [] := by decide
def merenOfClanNelToth : Ability :=
  whenever (Primitives.GameEvent.dies (a (otherCreatureYouControl thisCreature)))
    (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "Experience")) Primitives.NounPhrase.you)
theorem okMerenOfClanNelToth : Ability.check [] merenOfClanNelToth = [] := by decide
def kratosStoicFather : Ability :=
  at_ (Primitives.GameEvent.beginningOf .the .endStep (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
    (Primitives.Instruction.putCounters (countersOn (.named "Experience") Primitives.NounPhrase.you) (Primitives.CounterKindSource.printed plusOnePlusOne)
      (target creature))
theorem okKratosStoicFather : Ability.check [] kratosStoicFather = [] := by decide
def vashtaNerada : Ability :=
  triggeredIf (Primitives.GameEvent.beginningOf .the .endStep (Primitives.HeaderPossessor.byPlayer (each Primitives.Predicate.anyPlayer)))
    (happened (Primitives.GameEvent.dies (Primitives.NounPhrase.asMarker .permanent (relative
      .object))) (a creature) .thisTurn)
    (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne) thisCreature)
theorem okVashtaNerada : Ability.check [] vashtaNerada = [] := by decide
def furiousSpinesplitter : Ability :=
  at_ (Primitives.GameEvent.beginningOf .the .endStep (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
    (Primitives.Instruction.putCounters (forEach 1 (Primitives.Predicate.and
      [Primitives.Predicate.opponent, happenedTo (Primitives.GameEvent.isDealtDamage .any (relative
      .player)) .thisTurn]))
      (Primitives.CounterKindSource.printed plusOnePlusOne) thisCreature)
theorem okFuriousSpinesplitter : Ability.check [] furiousSpinesplitter = [] := by decide
def paladinOfAtonement : Ability :=
  triggeredIf (Primitives.GameEvent.beginningOf .the .upkeep (Primitives.HeaderPossessor.byPlayer (each Primitives.Predicate.anyPlayer)))
    (happened (Primitives.GameEvent.lifeChanges (relative .player) .down) Primitives.NounPhrase.you
      .lastTurn) (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed
      plusOnePlusOne) thisCreature)
theorem okPaladinOfAtonement : Ability.check [] paladinOfAtonement = [] := by decide
def throneWarden : Ability :=
  triggeredIf (Primitives.GameEvent.beginningOf .the .endStep (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
    (Primitives.Condition.matches Primitives.NounPhrase.you (Primitives.Predicate.hasDesignation "the monarch" none))
    (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne) thisCreature)
theorem okThroneWarden : Ability.check [] throneWarden = [] := by decide
def bloodTyrant : Ability :=
  whenever (Primitives.GameEvent.losesGame (a Primitives.Predicate.anyPlayer)) (Primitives.Instruction.putCounters (.lit 5) (Primitives.CounterKindSource.printed plusOnePlusOne) thisCreature)
theorem okBloodTyrant : Ability.check [] bloodTyrant = [] := by decide

def stagBeetle : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stag Beetle", cost := some [generic 3, pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Insect"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ entersWithCounters thisCreature (Primitives.Amount.letter .x) plusOnePlusOne,
              Primitives.StaticSpec.letterDefinition .x (countOf (otherCreature thisCreature)) ]) ],
      power := stat 0, toughness := stat 0 } }

def toweringTitan : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.conjunction none
    [ entersWithCounters thisCreature (Primitives.Amount.letter .x) plusOnePlusOne,
      Primitives.StaticSpec.letterDefinition .x
        (aggregate .sum (.stat .toughness) (otherCreatureYouControl thisCreature)) ])
theorem okToweringTitan : Ability.check [] toweringTitan = [] := by decide

def coretapper : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Coretapper", cost := some [generic 2], types := [.artifact, .creature],
      subtypes := [creatureType "Myr"],
      text :=
        [ activated Primitives.Cost.tapSymbol (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "Charge")) (target artifact)),
          activated (Primitives.Cost.perform (sacrifice thisCreature (agent := Primitives.NounPhrase.you)))
            (Primitives.Instruction.putCounters (.lit 2) (Primitives.CounterKindSource.printed (.named "Charge")) (target artifact)) ],
      power := stat 1, toughness := stat 1 } }

def divineIntervention : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Divine Intervention", cost := some [generic 6, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ Primitives.Ability.static (entersWithCounters thisEnchantment (.lit 2) (.named "Intervention")),
          at_ (Primitives.GameEvent.beginningOf .the .upkeep (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
            (Primitives.Instruction.removeCounters (some (exactly 1)) (some (Primitives.CounterKindSource.printed (.named "Intervention")))
              thisEnchantment),
          when (lastCounterRemovedBy (.named "Intervention") thisEnchantment Primitives.NounPhrase.you) Primitives.Instruction.drawGame ] } }

def celestialConvergence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Celestial Convergence", cost := some [generic 2, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ Primitives.Ability.static (entersWithCounters thisEnchantment (.lit 7) (.named "Omen")),
          at_ (Primitives.GameEvent.beginningOf .the .upkeep (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
            (Primitives.Instruction.sequentially
              [ Primitives.Instruction.removeCounters (some (exactly 1)) (some (Primitives.CounterKindSource.printed (.named "Omen"))) thisEnchantment,
                Primitives.Instruction.doIf (Primitives.Condition.compareAmt (countersOn (.named "Omen") thisEnchantment) .atMost (.lit 0))
                  (Primitives.Instruction.conclude .winGame (agent := (the (Primitives.Predicate.and [Primitives.Predicate.anyPlayer, Primitives.Predicate.superlative .max
                      (.playerStat .lifeTotal) Primitives.Predicate.anyPlayer]))))
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
        [ keywordSubject "Enchant" Primitives.Predicate.anyPlayer,
          whenever (Primitives.GameEvent.casts (Primitives.NounPhrase.attachHost .enchanted .player)
            (some (a spell)) none)
            (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "Spite")) thisAura),
          when (Primitives.GameEvent.losesGame (Primitives.NounPhrase.attachHost .enchanted .player))
            (Primitives.Instruction.sequentially
              [ gainLife (Primitives.Amount.letter .x) (agent := Primitives.NounPhrase.you), Primitives.Instruction.draw (Primitives.Amount.letter .x) (agent := Primitives.NounPhrase.you),
                Primitives.Instruction.define .x (countersOn (.named "Spite") thisAura) ]) ] } }

def passagewaySeer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Passageway Seer", cost := some [generic 3, pip .black], types := [.creature],
      subtypes := [creatureType "Tiefling", creatureType "Warlock"],
      text :=
        [ keyword "Lifelink",
          when (Primitives.GameEvent.enters thisCreature none) (Primitives.Instruction.gainDesignation Primitives.NounPhrase.you "the initiative" .instructed
              none),
          triggeredIf (Primitives.GameEvent.beginningOf .the .endStep (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
            (Primitives.Condition.matches Primitives.NounPhrase.you (Primitives.Predicate.hasDesignation "the initiative" none))
            (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne) thisCreature) ],
      power := stat 2, toughness := stat 2 } }

def chainsaw : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chainsaw", cost := some [generic 1, pip .red], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ when (Primitives.GameEvent.enters thisEquipment none)
            (Primitives.Instruction.dealDamage it (.lit 3) (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 1)) creature)),
          whenever (Primitives.GameEvent.dies (counted (atLeast 1) creature))
            (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "Rev")) thisEquipment),
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ Primitives.StaticSpec.modification (Primitives.NounPhrase.attachHost .equipped (.type .creature)) .power (Primitives.Delta.up (Primitives.Amount.letter .x)),
              Primitives.StaticSpec.modification (Primitives.NounPhrase.attachHost .equipped (.type .creature)) .toughness (Primitives.Delta.up (.lit 0)),
              Primitives.StaticSpec.letterDefinition .x (countersOn (.named "Rev") thisEquipment) ]),
          keywordCosting "Equip" (Primitives.Cost.mana [generic 3]) ] } }

/-- Soul's Might -/
def soulsMight : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Soul's Might", cost := some [generic 4, pip .green], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ Primitives.Instruction.putCounters (Primitives.Amount.letter .x) (Primitives.CounterKindSource.printed plusOnePlusOne) (target creature),
              Primitives.Instruction.define .x (Primitives.Amount.statOf (.stat .power) (that (.type .creature))) ]) ] } }

def woodlandChampion : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Woodland Champion", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [creatureType "Elf", creatureType "Scout"],
      text :=
        [ whenever (Primitives.GameEvent.enters (counted (atLeast 1) (Primitives.Predicate.and [Primitives.Predicate.isToken, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) none)
            (Primitives.Instruction.putCounters Primitives.Amount.groupSize (Primitives.CounterKindSource.printed plusOnePlusOne) thisCreature) ],
      power := stat 2, toughness := stat 2 } }

def voraciousBrood : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Voracious Brood", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [creatureType "Alien", creatureType "Insect"],
      text :=
        [ Primitives.Ability.static (entersWithCounters thisCreature
            (forEach 1 (Primitives.Predicate.and [creature, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)])) plusOnePlusOne),
          whenever (putIntoFrom (counted (atLeast 1) creature) (graveyardOf Primitives.NounPhrase.you) Primitives.EventSource.anywhere)
            (Primitives.Instruction.putCounters Primitives.Amount.groupSize (Primitives.CounterKindSource.printed plusOnePlusOne) thisCreature) ],
      power := stat 1, toughness := stat 1 } }

def herdBaloth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Herd Baloth", cost := some [generic 3, pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Beast"],
      text :=
        [ whenever (counterEvent .put plusOnePlusOne .many thisCreature)
            (offer (create (.lit 1) (creatureToken 4 4 [.green] [creatureType "Beast"])) (agent :=
                Primitives.NounPhrase.you)) ],
      power := stat 4, toughness := stat 4 } }

def flourishingDefenses : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Flourishing Defenses", cost := some [generic 4, pip .green], types := [.enchantment],
      text :=
        [ whenever (counterEvent .put minusOneMinusOne .one (a creature))
            (offer
              (create (.lit 1) (creatureToken 1 1 [.green] [creatureType "Elf", creatureType
                  "Warrior"])) (agent := Primitives.NounPhrase.you)) ] } }

def rakshasaVizier : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rakshasa Vizier", cost := some [generic 2, pip .black, pip .green, pip .blue],
      types := [.creature], subtypes := [creatureType "Demon"],
      text :=
        [ whenever
            (putIntoFrom (counted (atLeast 1) (Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you))) exileZone
              (Primitives.EventSource.zones [graveyardOf Primitives.NounPhrase.you]))
            (Primitives.Instruction.putCounters Primitives.Amount.groupSize (Primitives.CounterKindSource.printed plusOnePlusOne) thisCreature) ],
      power := stat 4, toughness := stat 4 } }

def foeLiage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Foe-liage", cost := some [generic 3, pip .green], types := [.creature],
      subtypes := [creatureType "Plant", creatureType "Mutant"],
      text :=
        [ triggeredOnlyDuring (Primitives.GameEvent.enters (a land) none) (Primitives.Timing.duringPart .turn (some Primitives.NounPhrase.you))
            (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne) thisCreature) ],
      power := stat 3, toughness := stat 3 } }

def hardenedScales : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hardened Scales", cost := some [pip .green], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.replacement (counterEvent .put plusOnePlusOne .many (a creatureYouControl)) []
            none
            (Primitives.Instruction.putCounters (plus Primitives.Amount.thatMuch (.lit 1)) (Primitives.CounterKindSource.printed plusOnePlusOne) it) .repeatedly none) ] } }

def branchingEvolution : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Branching Evolution", cost := some [generic 2, pip .green], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.replacement (counterEvent .put plusOnePlusOne .many (a creatureYouControl)) []
            none
            (Primitives.Instruction.putCounters (times (.lit 2) Primitives.Amount.thatMuch) (Primitives.CounterKindSource.printed plusOnePlusOne) (that (.type .creature)))
            .repeatedly none) ] } }

/-- Corpsejack Menace -/
def corpsejackMenace : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.replacement (counterEvent .put plusOnePlusOne .many (a creatureYouControl)) [] none
    (Primitives.Instruction.putCounters (times (.lit 2) Primitives.Amount.thatMuch) (Primitives.CounterKindSource.printed plusOnePlusOne) it) .repeatedly none)
theorem okCorpsejackMenace : Ability.check [] corpsejackMenace = [] := by decide

/-- Doubling Season -/
def doublingSeason : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Doubling Season", cost := some [generic 4, pip .green], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.replacement (tokensCreatedByEffectUnder (counted (atLeast 1) Primitives.Predicate.isToken) Primitives.NounPhrase.you) []
            none
            (Primitives.Instruction.create (times (.lit 2) Primitives.Amount.groupSize) Primitives.TokenSpec.asThose [] (agent := Primitives.NounPhrase.you)) .repeatedly none),
          Primitives.Ability.static (Primitives.StaticSpec.replacement
            (manyCountersPutByEffect (a (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))) [] none
            (Primitives.Instruction.putCounters (times (.lit 2) Primitives.Amount.thatMuch) Primitives.CounterKindSource.those (that .permanent)) .repeatedly none) ] } }

/-- Doc Samson, Super Psychiatrist -/
def docSamsonDistributive : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.replacement
    (manyBareCountersPutBy Primitives.NounPhrase.you (a (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))) [] none
    (Primitives.Instruction.putCounters (plus Primitives.Amount.thatMuch (.lit 1)) Primitives.CounterKindSource.those (that .permanent)) .repeatedly none)
theorem okDocSamsonDistributive : Ability.check [] docSamsonDistributive = [] := by decide

/-- Winding Constrictor -/
def windingConstrictor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Winding Constrictor", cost := some [pip .black, pip .green], types := [.creature],
      subtypes := [creatureType "Snake"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.replacement
            (bareCounterEvent .put .many
              (a (Primitives.Predicate.and [Primitives.Predicate.or [artifact, creature], Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))) [] none
            (Primitives.Instruction.putCounters (plus Primitives.Amount.thatMuch (.lit 1)) Primitives.CounterKindSource.those (that .permanent)) .repeatedly none),
          Primitives.Ability.static (Primitives.StaticSpec.replacement (bareCounterEvent .put .many Primitives.NounPhrase.you) [] none
            (Primitives.Instruction.putCounters (plus Primitives.Amount.thatMuch (.lit 1)) Primitives.CounterKindSource.those Primitives.NounPhrase.you) .repeatedly none) ],
      power := stat 2, toughness := stat 3 } }

/-- Aragorn, Company Leader -/
def aragornDistributive : Ability :=
  whenever (manyBareCountersPutBy Primitives.NounPhrase.you thisCreature)
    (Primitives.Instruction.putCounters (.lit 1) Primitives.CounterKindSource.those (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 1)) (otherCreature thisCreature)))
theorem okAragornDistributive : Ability.check [] aragornDistributive = [] := by decide
/-- Captain Marvel, Apex Avenger -/
def captainMarvelSameKinds : Ability :=
  triggeredIf (manyBareCountersPutBy Primitives.NounPhrase.you (a (otherCreature thisCreature)))
    (Primitives.Condition.not (Primitives.Condition.matches it (Primitives.Predicate.hasSubtype (creatureType "Kree"))))
    (offer (Primitives.Instruction.putCounters Primitives.Amount.thatMuch Primitives.CounterKindSource.those thisCreature) (agent := Primitives.NounPhrase.you))
theorem okCaptainMarvelSameKinds : Ability.check [] captainMarvelSameKinds = [] := by decide
/-- Denry Klin, Editor in Chief -/
def denryKlinSameKinds : Ability :=
  triggeredIf (Primitives.GameEvent.enters (a (Primitives.Predicate.and [nontoken, creatureYouControl])) none)
    (Primitives.Condition.matches thisCreature (Primitives.Predicate.hasCounters none))
    (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.sameAs thisCreature) (that (.type .creature)))
theorem okDenryKlinSameKinds : Ability.check [] denryKlinSameKinds = [] := by decide

/-- Denry Klin, Editor in Chief -/
def denryKlin : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Denry Klin, Editor in Chief", cost := some [generic 2, pip .white, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Cat", creatureType "Advisor"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.entryRider thisCreature
            (Primitives.TokenRider.withCounters (.lit 1)
              (Primitives.CounterKindSource.chosen [plusOnePlusOne, .keyword "FirstStrike", .keyword "Vigilance"]) .fresh)),
          denryKlinSameKinds ],
      power := stat 2, toughness := stat 2 } }

/-- Me, the Immortal -/
def meTheImmortalCounterMenu : Ability :=
  at_ (Primitives.GameEvent.beginningOf .the .combat (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
    (Primitives.Instruction.putCounters (.lit 1)
      (Primitives.CounterKindSource.chosen [plusOnePlusOne, .keyword "FirstStrike", .keyword "Vigilance", .keyword "Menace"])
      thisCreature)
theorem okMeTheImmortalCounterMenu : Ability.check [] meTheImmortalCounterMenu = [] := by decide

/-- Star Pupil -/
def starPupil : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Star Pupil", cost := some [pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ Primitives.Ability.static (entersWithCounters thisCreature (.lit 1) plusOnePlusOne),
          when (Primitives.GameEvent.dies thisCreature) (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.sameAs it) (target creatureYouControl)) ],
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
            (.lit 1) plusOnePlusOne),
          activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 1], Primitives.Cost.tapSymbol])
            (Primitives.Instruction.sequentially
              [ Primitives.Instruction.repeatTimes (Primitives.Amount.letter .x) proliferate,
                Primitives.Instruction.define .x
                  (countOf (Primitives.Predicate.and [ nontoken, creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you,
                                   Primitives.Predicate.happenedTo (Primitives.LookbackClause.mk
                                     (Primitives.GameEvent.enters (relative .object) none)
                                     .thisTurn) ])) ]) ],
      power := stat 2, toughness := stat 3 } }

/-- Runadi, Behemoth Caller -/
def runadiBehemothCaller : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.abilityGrant
    (allOf (Primitives.Predicate.and [ creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you,
                   Primitives.Predicate.compare [.counter plusOnePlusOne] .atLeast (.lit 3) ]))
    (keyword "Haste"))
theorem okRunadiBehemothCaller : Ability.check [] runadiBehemothCaller = [] := by decide
/-- Boon of Safety -/
def boonOfSafetyPut : Instruction :=
  Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "Shield")) (target creature)
theorem okBoonOfSafetyPut : Instruction.check [] boonOfSafetyPut = [] := by decide
/-- Vivien's Talent and Teferi's Talent -/
def talentLoyaltyPut : Instruction :=
  Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "Loyalty")) (Primitives.NounPhrase.attachHost .enchanted (.type .planeswalker))
theorem okTalentLoyaltyPut : Instruction.check [] talentLoyaltyPut = [] := by decide
/-- Simic Fluxmage -/
def simicFluxmageMove : Instruction :=
  Primitives.Instruction.moveCounters (.lit 1) (some (Primitives.CounterKindSource.printed plusOnePlusOne)) thisCreature (target creature)
theorem okSimicFluxmageMove : Instruction.check [] simicFluxmageMove = [] := by decide
/-- Rikku, Resourceful Guardian -/
def rikkuStealMove : Instruction :=
  Primitives.Instruction.moveCounters (.lit 1) none (target (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller anOpponent]))
    (target creatureYouControl)
theorem okRikkuStealMove : Instruction.check [] rikkuStealMove = [] := by decide
/-- Littjara Mirrorlake -/
def littjaraMirrorlakeCopy : Instruction :=
  Primitives.Instruction.create (.lit 1)
    (Primitives.TokenSpec.copyOf (target (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
      [Primitives.CopyExcept.entersWithCounters (.lit 1) plusOnePlusOne .additional])
    [] (agent := Primitives.NounPhrase.you)
theorem okLittjaraMirrorlakeCopy : Instruction.check [] littjaraMirrorlakeCopy = [] := by decide

def shalaiAndHallar : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shalai and Hallar", cost := some [generic 1, pip .red, pip .green, pip .white],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Angel", creatureType "Elf"],
      text :=
        [ keyword "Flying", keyword "Vigilance",
          whenever (counterEvent .put plusOnePlusOne .many (a creatureYouControl))
            (Primitives.Instruction.dealDamage thisCreature Primitives.Amount.thatMuch (target Primitives.Predicate.opponent)) ],
      power := stat 3, toughness := stat 3 } }

def primalVigor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Primal Vigor", cost := some [generic 4, pip .green], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.replacement (tokensCreated (counted (atLeast 1) Primitives.Predicate.isToken)) [] none
            (Primitives.Instruction.create (times (.lit 2) Primitives.Amount.groupSize) Primitives.TokenSpec.asThose [] (agent := Primitives.NounPhrase.you)) .repeatedly none),
          Primitives.Ability.static (Primitives.StaticSpec.replacement (counterEvent .put plusOnePlusOne .many (a creature)) [] none
            (Primitives.Instruction.putCounters (times (.lit 2) Primitives.Amount.thatMuch) (Primitives.CounterKindSource.printed plusOnePlusOne) (that (.type .creature)))
            .repeatedly none) ] } }

/-- Patrolling Peacemaker -/
def patrollingPeacemaker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Patrolling Peacemaker", cost := some [generic 2, pip .white],
      types := [.artifact, .creature], subtypes := [creatureType "Robot", creatureType "Soldier"],
      text :=
        [ Primitives.Ability.static (entersWithCounters thisCreature (.lit 2) plusOnePlusOne),
          whenever (Primitives.GameEvent.commitsCrime anOpponent) proliferate ],
      power := stat 0, toughness := stat 0 } }

/-- Galloping Lizrog -/
def gallopingLizrog : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Galloping Lizrog", cost := some [generic 3, pip .green, pip .blue],
      types := [.creature], subtypes := [creatureType "Frog", creatureType "Lizard"],
      text :=
        [ keyword "Trample",
          when (Primitives.GameEvent.enters thisCreature none)
            (Primitives.Instruction.offer
              (Primitives.Instruction.removeCounters (some anyNumber) (some (Primitives.CounterKindSource.printed plusOnePlusOne))
                (among (allOf creatureYouControl)))
              (some (Primitives.Instruction.putCounters (times (.lit 2) removedThisWay) (Primitives.CounterKindSource.printed plusOnePlusOne) thisCreature))
              none (agent := Primitives.NounPhrase.you)) ],
      power := stat 3, toughness := stat 3 } }

/-- Novijen Sages -/
def novijenSagesDraw : Ability :=
  activated
    (Primitives.Cost.compound
      [ Primitives.Cost.mana [generic 1],
        Primitives.Cost.perform (Primitives.Instruction.removeCounters (some (exactly 2)) (some (Primitives.CounterKindSource.printed plusOnePlusOne))
          (among (allOf creatureYouControl))) ])
    (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you))
theorem okNovijenSagesDraw : Ability.check [] novijenSagesDraw = [] := by decide
/-- Cyclone -/
def cycloneUpkeepPayment : Ability :=
  at_ (Primitives.GameEvent.beginningOf .the .upkeep (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
    (Primitives.Instruction.sequentially
      [ Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "Wind")) thisEnchantment,
        doUnless (sacrifice thisEnchantment (agent := Primitives.NounPhrase.you))
          (scaledMana (.run [pip .green])
            (times (.lit 1) (countersOn (.named "Wind") thisEnchantment))) (agent := Primitives.NounPhrase.you) ])
theorem okCycloneUpkeepPayment : Ability.check [] cycloneUpkeepPayment = [] := by decide

def stormwildCapridor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stormwild Capridor", cost := some [generic 2, pip .white], types := [.creature],
      subtypes := [creatureType "Bird", creatureType "Goat"],
      text :=
        [ keyword "Flying",
          Primitives.Ability.static (Primitives.StaticSpec.damageRule .noncombatOnly Primitives.DamageAgent.unattributed (Primitives.DamageScope.toRecipient thisCreature)
            (Primitives.DamageOp.prevent Primitives.PreventCut.all
              (some (Primitives.Instruction.putCounters preventedThisWay (Primitives.CounterKindSource.printed plusOnePlusOne) thisCreature)))
            .repeatedly) ],
      power := stat 1, toughness := stat 3 } }

def testOfFaith : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Test of Faith", cost := some [generic 1, pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.establish
            (Primitives.StaticSpec.damageRule .any Primitives.DamageAgent.unattributed (Primitives.DamageScope.toRecipient (target creature))
              (Primitives.DamageOp.prevent (Primitives.PreventCut.shield (.lit 3))
                (some (Primitives.Instruction.putCounters preventedThisWay (Primitives.CounterKindSource.printed plusOnePlusOne) (that (.type .creature)))))
              .repeatedly)
            (some Primitives.Duration.thisTurn)) ] } }

def temper : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Temper", cost := some [.variable, generic 1, pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.establish
            (Primitives.StaticSpec.damageRule .any Primitives.DamageAgent.unattributed (Primitives.DamageScope.toRecipient (target creature))
              (Primitives.DamageOp.prevent (Primitives.PreventCut.shield (Primitives.Amount.letter .x))
                (some (Primitives.Instruction.putCounters preventedThisWay (Primitives.CounterKindSource.printed plusOnePlusOne) (that (.type .creature)))))
              .repeatedly)
            (some Primitives.Duration.thisTurn)) ] } }

def phytohydra : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Phytohydra", cost := some [generic 2, pip .green, pip .white, pip .white],
      types := [.creature], subtypes := [creatureType "Plant", creatureType "Hydra"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.replacement (Primitives.GameEvent.isDealtDamage .any thisCreature) [] none
            (Primitives.Instruction.putCounters Primitives.Amount.thatMuch (Primitives.CounterKindSource.printed plusOnePlusOne) it) .repeatedly none) ],
      power := stat 1, toughness := stat 1 } }

def agelessEntity : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ageless Entity", cost := some [generic 3, pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text :=
        [ whenever (Primitives.GameEvent.lifeChanges Primitives.NounPhrase.you .up) (Primitives.Instruction.putCounters Primitives.Amount.thatMuch (Primitives.CounterKindSource.printed plusOnePlusOne) thisCreature) ],
      power := stat 4, toughness := stat 4 } }

/-- Chromatic Armor -/
def chromaticArmor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chromatic Armor", cost := some [generic 1, pip .white, pip .blue],
      types := [.enchantment], subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          Primitives.Ability.static (entersChoosing thisAura .color),
          Primitives.Ability.static (entersWithCounters thisAura (.lit 1) (.named "Sleight")),
          Primitives.Ability.static (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (allOf (Primitives.Predicate.and [source, ofTheLastChosen .color])))
            (Primitives.DamageScope.toRecipient (Primitives.NounPhrase.attachHost .enchanted (.type .creature))) (Primitives.DamageOp.prevent Primitives.PreventCut.all none) .repeatedly),
          activated (Primitives.Cost.mana [.variable])
            (Primitives.Instruction.sequentially
              [ Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "Sleight")) thisAura,
                choose (a (quality .color)),
                Primitives.Instruction.define .x (countersOn (.named "Sleight") thisAura) ]) ] } }

def vault75MiddleSchool : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vault 75: Middle School", cost := some [generic 2, pip .white, pip .white],
      types := [.enchantment], subtypes := [enchantmentType "Saga"],
      text :=
        [ when (Primitives.GameEvent.chapterMark [1])
            (exile (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.compare [.stat .power] .atLeast (.lit 4)]))),
          when (Primitives.GameEvent.chapterMark [2, 3])
            (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne) (each creatureYouControl)) ] } }

def keldonWarcaller : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Keldon Warcaller", cost := some [generic 1, pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Warrior"],
      text :=
        [ whenever (attacks thisCreature)
            (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "Lore"))
              (target (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (enchantmentType "Saga"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))) ],
      power := stat 2, toughness := stat 2 } }

def magistratesScepter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Magistrate's Scepter", cost := some [generic 3], types := [.artifact],
      text :=
        [ activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 4], Primitives.Cost.tapSymbol])
            (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "Charge")) thisArtifact),
          activated
            (Primitives.Cost.compound
              [ Primitives.Cost.tapSymbol,
                Primitives.Cost.perform (Primitives.Instruction.removeCounters (some (exactly 3)) (some (Primitives.CounterKindSource.printed (.named "Charge")))
                  thisArtifact) ])
            (Primitives.Instruction.addTurn (.lit 1) (agent := Primitives.NounPhrase.you)) ] } }

def grumgully : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Grumgully, the Generous", cost := some [generic 1, pip .red, pip .green],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Goblin", creatureType "Shaman"],
      text :=
        [ Primitives.Ability.static (entersWithAdditionalCounters
            (each (Primitives.Predicate.and [ creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you,
                          Primitives.Predicate.not (Primitives.Predicate.hasSubtype (creatureType "Human")), Primitives.Predicate.otherThan thisCreature ]))
            (.lit 1) plusOnePlusOne) ],
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
            (.lit 1) plusOnePlusOne) ],
      power := stat 2, toughness := stat 1 } }

def curatorBeastieLine : StaticSpec :=
  entersWithAdditionalCounters (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, colorless]))
    (.lit 2) plusOnePlusOne
theorem okCuratorBeastieLine : StaticSpec.check [] curatorBeastieLine = [] := by decide
def renataLine : StaticSpec :=
  entersWithAdditionalCounters
    (each (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, Primitives.Predicate.otherThan thisCreature])) (.lit 1)
    plusOnePlusOne
theorem okRenataLine : StaticSpec.check [] renataLine = [] := by decide
def entDraughtBasinAbility : Ability :=
  activatedOnlyDuring (Primitives.Cost.compound [Primitives.Cost.mana [.variable], Primitives.Cost.tapSymbol])
    (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne)
      (target (Primitives.Predicate.and [creature, Primitives.Predicate.compare [.stat .power] .eq (Primitives.Amount.letter .x)])))
    Primitives.Timing.asSorcery
theorem okEntDraughtBasinAbility : Ability.check [] entDraughtBasinAbility = [] := by decide

def matopiGolem : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Matopi Golem", cost := some [generic 5], types := [.artifact, .creature],
      subtypes := [creatureType "Golem"],
      text :=
        [ activated (Primitives.Cost.mana [generic 1])
            (Primitives.Instruction.triggerThisWay (Primitives.Instruction.regenerate thisCreature) (regenerates thisCreature)
              (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed minusOneMinusOne) it)) ],
      power := stat 3, toughness := stat 3 } }

def bramblewoodParagon : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bramblewood Paragon", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [creatureType "Elf", creatureType "Warrior"],
      text :=
        [ Primitives.Ability.static (entersWithAdditionalCounters
            (each (Primitives.Predicate.and [ creature, Primitives.Predicate.hasSubtype (creatureType "Warrior"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you,
                          Primitives.Predicate.otherThan thisCreature ]))
            (.lit 1) plusOnePlusOne),
          Primitives.Ability.static (Primitives.StaticSpec.abilityGrant
            (each (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, Primitives.Predicate.hasCounters (some plusOnePlusOne)]))
            (keyword "Trample")) ],
      power := stat 2, toughness := stat 2 } }

def oonasBlackguard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Oona's Blackguard", cost := some [generic 1, pip .black], types := [.creature],
      subtypes := [creatureType "Faerie", creatureType "Rogue"],
      text :=
        [ keyword "Flying",
          Primitives.Ability.static (entersWithAdditionalCounters
            (each (Primitives.Predicate.and [ creature, Primitives.Predicate.hasSubtype (creatureType "Rogue"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you,
                          Primitives.Predicate.otherThan thisCreature ]))
            (.lit 1) plusOnePlusOne),
          whenever
            (dealsCombatDamage
              (a (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, Primitives.Predicate.hasCounters (some plusOnePlusOne)]))
              (a Primitives.Predicate.anyPlayer))
            (discard (a (Primitives.Predicate.inZone hand)) (agent := (that .player))) ],
      power := stat 1, toughness := stat 1 } }

def crumblingAshes : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Crumbling Ashes", cost := some [generic 1, pip .black], types := [.enchantment],
      text :=
        [ at_ (Primitives.GameEvent.beginningOf .the .upkeep (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
            (destroy (target (Primitives.Predicate.and [creature, Primitives.Predicate.hasCounters (some minusOneMinusOne)]))) ] } }

def hunterOfEyeblights : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hunter of Eyeblights", cost := some [generic 3, pip .black, pip .black],
      types := [.creature], subtypes := [creatureType "Elf", creatureType "Assassin"],
      text :=
        [ when (Primitives.GameEvent.enters thisCreature none)
            (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne) (target creatureYouDontControl)),
          activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 2, pip .black], Primitives.Cost.tapSymbol])
            (destroy (target (Primitives.Predicate.and [creature, Primitives.Predicate.hasCounters none]))) ],
      power := stat 3, toughness := stat 3 } }

def pridemalkin : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Pridemalkin", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [creatureType "Cat"],
      text :=
        [ when (Primitives.GameEvent.enters thisCreature none)
            (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne) (target creatureYouControl)),
          Primitives.Ability.static (Primitives.StaticSpec.abilityGrant
            (each (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, Primitives.Predicate.hasCounters (some plusOnePlusOne)]))
            (keyword "Trample")) ],
      power := stat 2, toughness := stat 1 } }

def aboroth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aboroth", cost := some [generic 4, pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text :=
        [ cumulativeUpkeep (Primitives.Cost.perform (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed minusOneMinusOne) thisCreature)) ],
      power := stat 9, toughness := stat 9 } }

def shelteringAncient : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sheltering Ancient", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [creatureType "Treefolk"],
      text :=
        [ keyword "Trample",
          cumulativeUpkeep
            (Primitives.Cost.perform (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne)
              (a (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller anOpponent])))) ],
      power := stat 5, toughness := stat 5 } }

def coverOfWinterPut : Ability :=
  activated (Primitives.Cost.mana [.snow]) (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "Age")) thisEnchantment)
theorem okCoverOfWinterPut : Ability.check [] coverOfWinterPut = [] := by decide

def urborgScavengers : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Urborg Scavengers", cost := some [generic 2, pip .black], types := [.creature],
      subtypes := [creatureType "Spirit"],
      text :=
        [ triggeredOr (Primitives.GameEvent.enters thisCreature none) [attacks thisCreature]
            (Primitives.Instruction.sequentially
              [ exile (target (Primitives.Predicate.inZone graveyard)),
                Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne) thisCreature ]),
          Primitives.Ability.alsoForKeywords
            (Primitives.Ability.static (onlyWhile (Primitives.StaticSpec.abilityGrant thisCreature (keyword "Flying"))
              (exists_ (Primitives.Predicate.and [Primitives.Predicate.exiledWith thisCreature, Primitives.Predicate.hasKeyword (.the "Flying")]))))
            (["FirstStrike", "DoubleStrike", "Deathtouch", "Haste", "Hexproof", "Indestructible",
              "Lifelink", "Menace", "Reach", "Trample", "Vigilance"].map .the) ],
      power := stat 2, toughness := stat 2 } }

def sengirVampire : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sengir Vampire", cost := some [generic 3, pip .black, pip .black], types := [.creature],
      subtypes := [creatureType "Vampire"],
      text :=
        [ keyword "Flying",
          whenever (Primitives.GameEvent.dies (a (Primitives.Predicate.and [creature, happenedTo
            (Primitives.GameEvent.dealsDamage .any thisCreature (some
            (Primitives.NounPhrase.asMarker .permanent (relative .object)))) .thisTurn])))
            (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne) thisCreature) ],
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
                Primitives.Instruction.putCounters (.lit 2) (Primitives.CounterKindSource.printed plusOnePlusOne) it,
                Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you) ])
            Primitives.UsageLimit.oncePerGame ],
      power := stat 1, toughness := stat 1 } }

def infernalVessel : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Infernal Vessel", cost := some [generic 2, pip .black], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Cleric"],
      text :=
        [ triggeredIf (Primitives.GameEvent.dies thisCreature) (itIsntA (Primitives.Predicate.hasSubtype (creatureType "Demon")))
            (Primitives.Instruction.sequentially
              [ returnToBattlefieldWithCounters it (ownerOf it) (.lit 2) plusOnePlusOne,
                become it { characteristics := { subtypes := [creatureType "Demon"] } } none ]) ],
      power := stat 2, toughness := stat 1 } }

/-- Vivien's Talent -/
def viviensTalentTrigger : Ability :=
  whenever (Primitives.GameEvent.enters (a (Primitives.Predicate.and [nontoken, creatureYouControl])) none)
    (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "Loyalty")) (Primitives.NounPhrase.attachHost .enchanted (.type .planeswalker)))
theorem okViviensTalentTrigger : Ability.check [] viviensTalentTrigger = [] := by decide
/-- Bioessence Hydra -/
def bioessenceHydraTrigger : Ability :=
  whenever
    (counterEvent .put (.named "Loyalty") .many
      (allOf (Primitives.Predicate.and [Primitives.Predicate.hasType .planeswalker, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))
    (Primitives.Instruction.putCounters Primitives.Amount.thatMuch (Primitives.CounterKindSource.printed plusOnePlusOne) thisCreature)
theorem okBioessenceHydraTrigger : Ability.check [] bioessenceHydraTrigger = [] := by decide
/-- Vraska, Betrayal's Sting -/
def vraskaBetrayalsStingUltimate : Instruction :=
  Primitives.Instruction.doIf (Primitives.Condition.compareAmt (countersOn (.named "Poison") (target Primitives.Predicate.anyPlayer)) .less (.lit 9))
    (Primitives.Instruction.putCounters Primitives.Amount.theDifference (Primitives.CounterKindSource.printed (.named "Poison")) they) none
theorem okVraskaBetrayalsStingUltimate :
    Instruction.check [] vraskaBetrayalsStingUltimate = [] := by decide
/-- Vexing Puzzlebox -/
def vexingPuzzleboxCounters : Ability :=
  whenever (Primitives.GameEvent.rollsDice Primitives.NounPhrase.you .many none Primitives.RollWatch.anyResult)
    (Primitives.Instruction.putCounters (Primitives.Amount.theOutcome .rollResult) (Primitives.CounterKindSource.printed (.named "Charge")) thisArtifact)
theorem okVexingPuzzleboxCounters : Ability.check [] vexingPuzzleboxCounters = [] := by decide
def spaceFamilyGoblinsonRoll : Ability :=
  whenever (Primitives.GameEvent.rollsDice Primitives.NounPhrase.you .one none Primitives.RollWatch.anyResult)
    (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne) thisCreature)
theorem okSpaceFamilyGoblinsonRoll : Ability.check [] spaceFamilyGoblinsonRoll = [] := by decide
/-- Atomwheel Acrobats -/
def atomwheelAcrobatsRoll : Ability :=
  whenever (youRollResultIn (Primitives.Quantity.range (some 1) (some 2)))
    (Primitives.Instruction.putCounters Primitives.Amount.thatMuch (Primitives.CounterKindSource.printed plusOnePlusOne) thisCreature)
theorem okAtomwheelAcrobatsRoll : Ability.check [] atomwheelAcrobatsRoll = [] := by decide
/-- Resolute Veggiesaur -/
def resoluteVeggiesaurThirdDie : Ability :=
  whenever (Primitives.GameEvent.nthOccurrence (.nth 3) (some .turn) (Primitives.GameEvent.rollsDice Primitives.NounPhrase.you .one none Primitives.RollWatch.anyResult))
    (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne) thisCreature)
theorem okResoluteVeggiesaurThirdDie : Ability.check [] resoluteVeggiesaurThirdDie = [] := by decide
/-- Run the Play (Striding Shotcaller's other half) -/
def runThePlayCounters : Instruction :=
  Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne)
    (Primitives.NounPhrase.eachOf (Primitives.NounPhrase.described (Primitives.DetPhrase.target (Primitives.Quantity.upToOf (Primitives.Amount.letter .x))) creature))
theorem okRunThePlayCounters : Instruction.check [letterB .x] runThePlayCounters = [] := by decide
/-- Font of Agonies -/
def fontOfAgoniesTrigger : Ability :=
  whenever (Primitives.GameEvent.paysLife Primitives.NounPhrase.you) (Primitives.Instruction.putCounters Primitives.Amount.thatMuch (Primitives.CounterKindSource.printed (.named "Blood")) thisEnchantment)
theorem okFontOfAgoniesTrigger : Ability.check [] fontOfAgoniesTrigger = [] := by decide
/-- Heart of Kiran -/
def heartOfKiranCrewAltCost : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.altCost (allOf (Primitives.Predicate.and [Primitives.Predicate.abilityHead (.keyword "Crew"), Primitives.Predicate.abilityOf Primitives.NounPhrase.this]))
    (some (Primitives.Cost.perform (Primitives.Instruction.removeCounters (some (exactly 1)) (some (Primitives.CounterKindSource.printed (.named "Loyalty")))
      (a (Primitives.Predicate.and [Primitives.Predicate.hasType .planeswalker, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))))))
theorem okHeartOfKiranCrewAltCost : Ability.check [] heartOfKiranCrewAltCost = [] := by decide
/-- Tainted Adversary -/
def taintedAdversaryOffer : Ability :=
  when (Primitives.GameEvent.enters thisCreature none)
    (Primitives.Instruction.triggerReflexively (offer (Primitives.Instruction.pay (Primitives.Cost.mana [generic 2, pip .black]) .anyNumberOfTimes (agent :=
        Primitives.NounPhrase.you)) (agent := Primitives.NounPhrase.you))
      (Primitives.Instruction.putCounters Primitives.Amount.thatMuch (Primitives.CounterKindSource.printed plusOnePlusOne) thisCreature))
theorem okTaintedAdversaryOffer : Ability.check [] taintedAdversaryOffer = [] := by decide
/-- Korvold, Gleeful Glutton -/
def korvoldCombatTrigger : Ability :=
  whenever (dealsCombatDamage thisCreature (a Primitives.Predicate.anyPlayer))
    (Primitives.Instruction.sequentially
      [ Primitives.Instruction.putCounters (Primitives.Amount.letter .x) (Primitives.CounterKindSource.printed plusOnePlusOne) thisCreature,
        Primitives.Instruction.draw (Primitives.Amount.letter .x) (agent := Primitives.NounPhrase.you),
        Primitives.Instruction.define .x (Primitives.Amount.distinctCount .permanentType (allOf (Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)))) ])
theorem okKorvoldCombatTrigger : Ability.check [] korvoldCombatTrigger = [] := by decide
/-- Mirelurk Queen -/
def mirelurkQueenTrigger : Ability :=
  triggeredOnlyOnce
    (Primitives.GameEvent.verbedEvent none (.action "Mill")
      (some (counted (atLeast 1) (Primitives.Predicate.and [Primitives.Predicate.not land,
        Primitives.Predicate.inZone library]))) none none)
    Primitives.UsageLimit.oncePerTurn
    (Primitives.Instruction.sequentially [Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you), Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne)
        thisCreature])
theorem okMirelurkQueenTrigger : Ability.check [] mirelurkQueenTrigger = [] := by decide

/-- Whirling Dervish -/
def whirlingDervish : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Whirling Dervish", cost := some [pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Monk"],
      text :=
        [ keywordQuality "Protection" (Primitives.Predicate.colorIs .black),
          triggeredIf (Primitives.GameEvent.beginningOf .the .endStep (Primitives.HeaderPossessor.byPlayer (each Primitives.Predicate.anyPlayer)))
            (happened (Primitives.GameEvent.dealsDamage .any (relative .object) (some anOpponent))
              thisCreature .thisTurn)
            (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne) it) ],
      power := stat 1, toughness := stat 1 } }

/-- Ichor Shade -/
def ichorShade : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ichor Shade", cost := some [generic 2, pip .black], types := [.creature],
      subtypes := [creatureType "Phyrexian", creatureType "Shade"],
      text :=
        [ triggeredIf (Primitives.GameEvent.beginningOf .the .endStep (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
            (Primitives.Condition.happened (a (Primitives.Predicate.or [artifact, creature]))
              (Primitives.LookbackClause.mk (Primitives.GameEvent.putInto (relative .object)
                graveyard (some (Primitives.EventSource.zones [battlefield]))) .thisTurn))
            (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne) thisCreature) ],
      power := stat 2, toughness := stat 3 } }

/-- Asmira, Holy Avenger -/
def asmiraHolyAvenger : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Asmira, Holy Avenger", cost := some [generic 2, pip .green, pip .white],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Cleric"],
      text :=
        [ keyword "Flying",
          at_ (Primitives.GameEvent.beginningOf .the .endStep (Primitives.HeaderPossessor.byPlayer (each Primitives.Predicate.anyPlayer)))
            (Primitives.Instruction.putCounters
              (Primitives.Amount.eventTally .count (a creature)
                (Primitives.LookbackClause.mk (Primitives.GameEvent.putInto (relative .object)
                  (graveyardOf Primitives.NounPhrase.you) (some (Primitives.EventSource.zones
                  [battlefield]))) .thisTurn))
              (Primitives.CounterKindSource.printed plusOnePlusOne) thisCreature) ],
      power := stat 2, toughness := stat 3 } }

/-- Thought Sponge -/
def thoughtSponge : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Thought Sponge", cost := some [generic 3, pip .blue], types := [.creature],
      subtypes := [creatureType "Sponge"],
      text :=
        [ keyword "Flash",
          Primitives.Ability.static (entersWithCounters thisCreature greatestCardsAnOpponentDrew plusOnePlusOne),
          when (Primitives.GameEvent.dies thisCreature) (Primitives.Instruction.draw (Primitives.Amount.statOf (.stat .power) thisCreature) (agent := Primitives.NounPhrase.you)) ],
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
            (sacrifice thisCreature (agent := Primitives.NounPhrase.you)),
          activated Primitives.Cost.tapSymbol (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed minusOneMinusOne) (target creature)) ],
      power := stat 0, toughness := stat 3 } }

/-- Contractual Safeguard -/
def contractualSafeguardPass : Instruction :=
  Primitives.Instruction.sequentially
    [ choose (a (Primitives.Predicate.counterKindOn (a creatureYouControl))),
      Primitives.Instruction.putCounters (.lit 1) Primitives.CounterKindSource.bound (each (otherCreatureYouControl it)) ]
theorem okContractualSafeguardPass : Instruction.check [] contractualSafeguardPass = [] := by decide

/-- Feral Contest -/
def feralContest : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Feral Contest", cost := some [generic 3, pip .green], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne) (target creatureYouControl),
              requireBlockIt (target (Primitives.Predicate.and [creature, Primitives.Predicate.other])) (some Primitives.Duration.thisTurn) ]) ] } }

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
                [ Primitives.Instruction.putCounters (.lit 2) (Primitives.CounterKindSource.printed plusOnePlusOne) (target creatureYouControl),
                  gain
                    (itPrior (Primitives.Instruction.putCounters (.lit 2) (Primitives.CounterKindSource.printed plusOnePlusOne) (target creatureYouControl)))
                    (keyword "Vigilance") (some untilEndOfTurn) ])) ],
      power := stat 3, toughness := stat 4 } }

/-- Stunning Shot -/
def stunningShot : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stunning Shot", cost := some [generic 1, pip .white], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ Primitives.Instruction.putCounters (.lit 2) (Primitives.CounterKindSource.printed plusOnePlusOne)
                (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 1)) creatureYouControl),
              tap (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 1)) (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller anOpponent])),
              Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "Stun"))
                (itPrior
                  (tap (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 1))
                    (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller anOpponent])))) ]) ] } }

/-- Stonebinder's Familiar -/
def stonebindersFamiliar : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stonebinder's Familiar", cost := some [pip .white], types := [.creature],
      subtypes := [creatureType "Spirit", creatureType "Dog"],
      text :=
        [ Primitives.Ability.triggered (Primitives.GameEvent.putInto (counted (atLeast 1) Primitives.Predicate.isCard) exileZone none) [] none []
            (some (Primitives.Timing.duringPart .turn (some Primitives.NounPhrase.you))) (some Primitives.UsageLimit.oncePerTurn) none
            (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne) thisCreature) ],
      power := stat 1, toughness := stat 1 } }

/-- Nazgûl -/
def nazgulRingTrigger : Ability :=
  whenever (Primitives.GameEvent.verbedEvent (some Primitives.NounPhrase.you) (.action
    "The Ring Tempts You") none none none)
    (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne)
      (each (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (creatureType "Wraith"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))
theorem okNazgulRingTrigger : Ability.check [] nazgulRingTrigger = [] := by decide

/-- Captain Marvel, Apex Avenger -/
def captainMarvelApexAvenger : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Captain Marvel, Apex Avenger", cost := some [generic 5, pip .red, pip .white],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Kree", creatureType "Hero"],
      text :=
        [ keyword "Flying", keyword "DoubleStrike", keyword "Indestructible",
          triggeredIf
            (Primitives.GameEvent.counterEvent .put none (a (Primitives.Predicate.and [creature, Primitives.Predicate.otherThan Primitives.NounPhrase.this])) .many (some Primitives.NounPhrase.you) false)
            (Primitives.Condition.matches it (Primitives.Predicate.not (Primitives.Predicate.hasSubtype (creatureType "Kree"))))
            (offer (Primitives.Instruction.putCounters Primitives.Amount.thatMuch Primitives.CounterKindSource.those Primitives.NounPhrase.this) (agent := Primitives.NounPhrase.you)) ],
      power := stat 4, toughness := stat 4 } }

/-- Blood Spatter Analysis -/
def bloodSpatterAnalysis : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blood Spatter Analysis", cost := some [pip .black, pip .red], types := [.enchantment],
      text :=
        [ when (Primitives.GameEvent.enters thisEnchantment none)
            (Primitives.Instruction.dealDamage thisEnchantment (.lit 3)
              (target (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller anOpponent]))),
          whenever (Primitives.GameEvent.dies (counted (atLeast 1) creature))
            (Primitives.Instruction.sequentially
              [ mill (.lit 1) Primitives.NounPhrase.you (agent := Primitives.NounPhrase.you),
                Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "Bloodstain")) thisEnchantment,
                Primitives.Instruction.triggerReflexively
                  (Primitives.Instruction.doOnlyIf (sacrifice thisEnchantment (agent := Primitives.NounPhrase.you))
                    (Primitives.Condition.compareAmt (countersOn (.named "Bloodstain") thisEnchantment) .atLeast (.lit 5))
                    none)
                  (move (target (Primitives.Predicate.and [creature, Primitives.Predicate.isCard, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)])) hand) ]) ] } }

/-- Bewitching Leechcraft -/
def bewitchingLeechcraft : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bewitching Leechcraft", cost := some [generic 1, pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          when (Primitives.GameEvent.enters thisAura none) (tap (Primitives.NounPhrase.attachHost .enchanted (.type .creature))),
          Primitives.Ability.static (Primitives.StaticSpec.abilityGrant (Primitives.NounPhrase.attachHost .enchanted (.type .creature))
            (Primitives.Ability.static (Primitives.StaticSpec.replacement (Primitives.GameEvent.statusEvent thisCreature .untapped) []
              (some (Primitives.Timing.duringPart .untapStep (some Primitives.NounPhrase.you)))
              (Primitives.Instruction.doIfDone
                (Primitives.Instruction.removeCounters (some (exactly 1)) (some (Primitives.CounterKindSource.printed plusOnePlusOne)) thisCreature)
                (some (untap thisCreature)) none)
              .repeatedly none))) ] } }

def agentOfTheShadowThievesGrantedTrigger : Ability :=
  Primitives.Ability.triggered (Primitives.GameEvent.combat .attackerOf thisCreature (some (a Primitives.Predicate.anyPlayer))) [] none [] none none
    (some (Primitives.Condition.not (exists_ (Primitives.Predicate.and [ Primitives.Predicate.opponent,
                                 Primitives.Predicate.compare [.playerStat .lifeTotal] .greater
                                   (lifeTotalOf (that .player)) ]))))
    (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne) thisCreature)
theorem okAgentOfTheShadowThievesGrantedTrigger :
    Ability.check [] agentOfTheShadowThievesGrantedTrigger = [] := by decide

/-- Bloodline Pretender -/
def bloodlinePretender : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bloodline Pretender", cost := some [generic 3], types := [.artifact, .creature],
      subtypes := [creatureType "Shapeshifter"],
      text :=
        [ keyword "Changeling",
          Primitives.Ability.static (entersChoosing thisCreature (.subtype .creature)),
          whenever
            (Primitives.GameEvent.enters (a (Primitives.Predicate.and [ creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, Primitives.Predicate.otherThan thisCreature,
                                ofChosen (.subtype .creature) ])) none)
            (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne) thisCreature) ],
      power := stat 2, toughness := stat 2 } }

/-- Chamber Sentry -/
def chamberSentryDamage : Ability :=
  activated
    (Primitives.Cost.compound
      [ Primitives.Cost.mana [.variable], Primitives.Cost.tapSymbol,
        Primitives.Cost.perform (Primitives.Instruction.removeCounters (some (Primitives.Quantity.exactlyOf (Primitives.Amount.letter .x))) (some (Primitives.CounterKindSource.printed plusOnePlusOne))
          thisCreature) ])
    (Primitives.Instruction.dealDamage Primitives.NounPhrase.this (Primitives.Amount.letter .x) (target anyTarget))
theorem okChamberSentryDamage : Ability.check [] chamberSentryDamage = [] := by decide

/-- Menacing Ogre -/
def menacingOgre : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Menacing Ogre", cost := some [generic 3, pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Ogre"],
      text :=
        [ keyword "Trample", keyword "Haste",
          when (Primitives.GameEvent.enters thisCreature none)
            (Primitives.Instruction.sequentially
              [ choose (disclosure := .secretly) (a (quality .number)) (agent := some (each
                  Primitives.Predicate.anyPlayer)),
                Primitives.Instruction.revealChoices .numbers,
                loseLife Primitives.Amount.thatMuch (agent := (each (Primitives.Predicate.and [Primitives.Predicate.anyPlayer, Primitives.Predicate.choseExtreme .max]))),
                doIf (Primitives.Condition.matches Primitives.NounPhrase.you (Primitives.Predicate.choseExtreme .max))
                  (Primitives.Instruction.putCounters (.lit 2) (Primitives.CounterKindSource.printed plusOnePlusOne) thisCreature) ]) ],
      power := stat 3, toughness := stat 3 } }

/-- Charnel Troll -/
def charnelTroll : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Charnel Troll", cost := some [generic 1, pip .black, pip .green], types := [.creature],
      subtypes := [creatureType "Troll"],
      text :=
        [ keyword "Trample",
          at_ (Primitives.GameEvent.beginningOf .the .upkeep (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
            (Primitives.Instruction.doIfDone (exile (a (Primitives.Predicate.and [creature, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)])))
              (some (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne) thisCreature))
              (some (sacrifice thisCreature (agent := Primitives.NounPhrase.you)))),
          activated
            (Primitives.Cost.compound
              [ Primitives.Cost.mana [pip .black, pip .green],
                Primitives.Cost.perform (discard (a (Primitives.Predicate.and [creature, Primitives.Predicate.inZone hand])) (agent := Primitives.NounPhrase.you)) ])
            (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne) thisCreature) ],
      power := stat 4, toughness := stat 4 } }

/-- Mistbreath Elder -/
def mistbreathElder : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mistbreath Elder", cost := some [pip .green], types := [.creature],
      subtypes := [creatureType "Frog", creatureType "Warrior"],
      text :=
        [ at_ (Primitives.GameEvent.beginningOf .the .upkeep (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
            (Primitives.Instruction.doIfDone (returnTo (a (otherCreatureYouControl thisCreature)) hand [])
              (some (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne) thisCreature))
              (some (offer (returnTo thisCreature hand []) (agent := Primitives.NounPhrase.you)))) ],
      power := stat 2, toughness := stat 2 } }

/-- Dark Depths -/
def darkDepths : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dark Depths", supertypes := [.legendary, .snow], types := [.land],
      text :=
        [ Primitives.Ability.static (entersWithCounters thisLand (.lit 10) (.named "Ice")),
          activated (Primitives.Cost.mana [generic 3])
            (removeCounters (exactly 1) (some (Primitives.CounterKindSource.printed (.named "Ice"))) thisLand),
          when (Primitives.GameEvent.stateHolds (Primitives.Condition.not (Primitives.Condition.matches thisLand (Primitives.Predicate.hasCounters (some (.named "Ice"))))))
            (Primitives.Instruction.doIfDone (sacrifice thisLand (agent := Primitives.NounPhrase.you)) (some (create (.lit 1) maritLage)) none)
                ] } }

/-- Thallid -/
def thallid : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Thallid", cost := some [pip .green], types := [.creature],
      subtypes := [creatureType "Fungus"],
      text :=
        [ at_ (Primitives.GameEvent.beginningOf .the .upkeep (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
            (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "Spore")) thisCreature),
          activated
            (Primitives.Cost.perform (Primitives.Instruction.removeCounters (some (exactly 3)) (some (Primitives.CounterKindSource.printed (.named "Spore"))) thisCreature))
            (create (.lit 1) (creatureToken 1 1 [.green] [creatureType "Saproling"])) ],
      power := stat 1, toughness := stat 1 } }

/-- Aether Chaser -/
def aetherChaserEnergy : Ability :=
  when (Primitives.GameEvent.enters thisCreature none) (Primitives.Instruction.putCounters (.lit 2) (Primitives.CounterKindSource.printed (.named "Energy")) Primitives.NounPhrase.you)
theorem okAetherChaserEnergy : Ability.check [] aetherChaserEnergy = [] := by decide
/-- Archfiend of the Dross -/
def archfiendOfTheDrossEntersOiled : StaticSpec :=
  entersWithCounters thisCreature (.lit 4) (.named "Oil")
theorem okArchfiendOfTheDrossEntersOiled :
    StaticSpec.check [] archfiendOfTheDrossEntersOiled = [] := by decide
/-- Archfiend of the Dross -/
def archfiendOfTheDrossOilUpkeep : Ability :=
  at_ (Primitives.GameEvent.beginningOf .the .upkeep (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
    (Primitives.Instruction.removeCounters (some (exactly 1)) (some (Primitives.CounterKindSource.printed (.named "Oil"))) thisCreature)
theorem okArchfiendOfTheDrossOilUpkeep : Ability.check [] archfiendOfTheDrossOilUpkeep = [] := by
  decide
def neurokTransmuter : Instruction :=
  become (target creature) { characteristics := { types := [.artifact] } } (some untilEndOfTurn)
theorem okNeurokTransmuter : Instruction.check [] neurokTransmuter = [] := by decide
def syphonMind : Instruction :=
  Primitives.Instruction.sequentially
    [ discard (a (Primitives.Predicate.inZone hand)) (agent := (each otherPlayer)),
      Primitives.Instruction.doForEach (theVerbed (.action "Discard") .card .thisWay .many) (Primitives.Instruction.draw (.lit 1) (agent :=
          Primitives.NounPhrase.you)) ]
theorem okSyphonMind : Instruction.check [] syphonMind = [] := by decide
def peek : Instruction := Primitives.Instruction.sequentially [lookAtHandOf (target Primitives.Predicate.anyPlayer), Primitives.Instruction.draw (.lit 1) (agent :=
    Primitives.NounPhrase.you)]
theorem okPeek : Instruction.check [] peek = [] := by decide
/-- Bumi, King of Three Trials -/
def bumiScryMode : Instruction := scry (.lit 3) (agent := (target Primitives.Predicate.anyPlayer))
theorem okBumiScryMode : Instruction.check [] bumiScryMode = [] := by decide
/-- Final Act -/
def finalActCounterMode : Instruction := loseAllCounters none (agent := (each Primitives.Predicate.opponent))
theorem okFinalActCounterMode : Instruction.check [] finalActCounterMode = [] := by decide
def leeches : Instruction :=
  loseAllCounters (some (Primitives.CounterKindSource.printed (.named "Poison"))) (agent := (target Primitives.Predicate.anyPlayer))
theorem okLeeches : Instruction.check [] leeches = [] := by decide

/-- Woeleecher -/
def woeleecherWhole : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Woeleecher", cost := some [generic 5, pip .white], types := [.creature],
      subtypes := [creatureType "Elemental"], text := [woeleecher],
      power := stat 3, toughness := stat 5 } }

end Semantics.Cards

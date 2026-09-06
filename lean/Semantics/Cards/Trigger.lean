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
  delayWithin (Primitives.GameEvent.dies (target creature)) Primitives.Duration.thisTurn (move (that .card) battlefield)
theorem okGracefulReprieve : Instruction.check [] gracefulReprieve = [] := by decide

def cloudkinSeer : Ability := when (Primitives.GameEvent.enters thisCreature none) (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you))
theorem okCloudkinSeer : Ability.check [] cloudkinSeer = [] := by decide
def promiseOfTomorrow : Ability := whenever (Primitives.GameEvent.dies (a creatureYouControl)) (exile it)
theorem okPromiseOfTomorrow : Ability.check [] promiseOfTomorrow = [] := by decide
def libraryLarcenist : Ability := whenever (attacks thisCreature) (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you))
theorem okLibraryLarcenist : Ability.check [] libraryLarcenist = [] := by decide
def jhessianThief : Ability :=
  whenever (dealsCombatDamage thisCreature (a Primitives.Predicate.anyPlayer)) (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you))
theorem okJhessianThief : Ability.check [] jhessianThief = [] := by decide
def scholarOfStars : Ability :=
  triggeredIf (Primitives.GameEvent.enters thisCreature none)
    (exists_ (Primitives.Predicate.and [artifact, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you))
theorem okScholarOfStars : Ability.check [] scholarOfStars = [] := by decide

def miserysShadow : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.replacement (Primitives.GameEvent.dies (a (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller (a Primitives.Predicate.opponent)]))) []
    none (exile it) .repeatedly none)
theorem okMiserysShadow : Ability.check [] miserysShadow = [] := by decide

def beastWhisperer : Ability :=
  whenever (Primitives.GameEvent.casts Primitives.NounPhrase.you (some (a (Primitives.Predicate.and
    [creature, spell]))) none) (Primitives.Instruction.draw (.lit 1) (agent :=
    Primitives.NounPhrase.you))
theorem okBeastWhisperer : Ability.check [] beastWhisperer = [] := by decide
def mesmericOrb : Ability :=
  whenever (Primitives.GameEvent.statusEvent (a permanent) .untapped)
    (mill (.lit 1) they (agent := (controllerOf (that .permanent))))
theorem okMesmericOrb : Ability.check [] mesmericOrb = [] := by decide
def secretPlans : Ability :=
  whenever (Primitives.GameEvent.statusEvent (a (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .faceUp)
    (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you))
theorem okSecretPlans : Ability.check [] secretPlans = [] := by decide

/-- Teferi's Imp -/
def teferisImpPhasesOut : Ability :=
  whenever (Primitives.GameEvent.statusEvent thisCreature .phasedOut) (discard (a (Primitives.Predicate.inZone hand)) (agent := Primitives.NounPhrase.you))
theorem okTeferisImpPhasesOut : Ability.check [] teferisImpPhasesOut = [] := by decide
/-- Teferi's Imp -/
def teferisImpPhasesIn : Ability :=
  whenever (Primitives.GameEvent.statusEvent thisCreature .phasedIn) (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you))
theorem okTeferisImpPhasesIn : Ability.check [] teferisImpPhasesIn = [] := by decide

/-- Oubliette -/
def oubliette : Ability :=
  when (Primitives.GameEvent.enters thisEnchantment none)
    (phaseOutUntil (target creature) (leavesBattlefield thisEnchantment))
theorem okOubliette : Ability.check [] oubliette = [] := by decide

def shimmeringEfreet : Ability :=
  whenever (Primitives.GameEvent.statusEvent thisCreature .phasedIn) (Primitives.Instruction.setStatus .phasedOut (target creature))
theorem okShimmeringEfreet : Ability.check [] shimmeringEfreet = [] := by decide
def hollowhengeSpirit : Ability :=
  when (Primitives.GameEvent.enters thisCreature none)
    (Primitives.Instruction.removeFromCombat (target (Primitives.Predicate.and [creature, Primitives.Predicate.or [attacking, blocking]])))
theorem okHollowhengeSpirit : Ability.check [] hollowhengeSpirit = [] := by decide
def netcasterSpider : Ability :=
  whenever (blocks thisCreature (some (a (Primitives.Predicate.and [creature, Primitives.Predicate.hasKeyword (.the "Flying")]))))
    (get thisCreature (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 0)) (some untilEndOfTurn))
theorem okNetcasterSpider : Ability.check [] netcasterSpider = [] := by decide
def viashinoWeaponsmith : Ability :=
  whenever (becomesBlocked thisCreature (some (a creature)))
    (get thisCreature (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 2)) (some untilEndOfTurn))
theorem okViashinoWeaponsmith : Ability.check [] viashinoWeaponsmith = [] := by decide
def somberwaldAlpha : Ability :=
  whenever (becomesBlocked (a creatureYouControl) none)
    (get it (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1)) (some untilEndOfTurn))
theorem okSomberwaldAlpha : Ability.check [] somberwaldAlpha = [] := by decide
def vertigoSpawn : Ability :=
  whenever (blocks thisCreature (some (a creature)))
    (Primitives.Instruction.sequentially
      [ Primitives.Instruction.setStatus .tapped (that (.type .creature)),
        Primitives.Instruction.skipUntap (that (.type .creature)) (.lit 1) ])
theorem okVertigoSpawn : Ability.check [] vertigoSpawn = [] := by decide
def orneryDilophosaur : Ability :=
  triggeredIf (attacks thisCreature)
    (exists_ (Primitives.Predicate.and [ creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you,
                     Primitives.Predicate.compare [.stat .power] .atLeast (.lit 4) ]))
    (get thisCreature (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 2)) (some untilEndOfTurn))
theorem okOrneryDilophosaur : Ability.check [] orneryDilophosaur = [] := by decide
def incisorGlider : Ability :=
  triggeredIf (attacks thisCreature)
    (Primitives.Condition.compareAmt (countersOn (.named "Poison") (a Primitives.Predicate.opponent)) .atLeast (.lit 3))
    (get (allOf creatureYouControl) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1)) (some untilEndOfTurn))
theorem okIncisorGlider : Ability.check [] incisorGlider = [] := by decide
def stormFleetSpy : Ability :=
  triggeredIf (Primitives.GameEvent.enters thisCreature none) (happened (Primitives.GameEvent.combat
    .attackerOf (relative .player) none) Primitives.NounPhrase.you .thisTurn)
    (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you))
theorem okStormFleetSpy : Ability.check [] stormFleetSpy = [] := by decide
def loanShark : Ability :=
  triggeredIf (Primitives.GameEvent.enters thisCreature none)
    (Primitives.Condition.compareAmt (eventCount (Primitives.GameEvent.casts (relative .player) none
      none) Primitives.NounPhrase.you .thisTurn) .atLeast (.lit 2)) (Primitives.Instruction.draw
      (.lit 1) (agent :=
        Primitives.NounPhrase.you))
theorem okLoanShark : Ability.check [] loanShark = [] := by decide

def forceOfDespair : Instruction :=
  destroy (allOf (Primitives.Predicate.and [creature, happenedTo (Primitives.GameEvent.enters
    (relative .object) none) .thisTurn]))
theorem okForceOfDespair : Instruction.check [] forceOfDespair = [] := by decide
def cradleToGrave : Instruction :=
  destroy (target (Primitives.Predicate.and [creature, Primitives.Predicate.not
    (Primitives.Predicate.colorIs .black), happenedTo (Primitives.GameEvent.enters (relative
    .object) none) .thisTurn]))
theorem okCradleToGrave : Instruction.check [] cradleToGrave = [] := by decide

def aragornKingOfGondor : Ability :=
  when (Primitives.GameEvent.enters thisCreature none) (Primitives.Instruction.gainDesignation Primitives.NounPhrase.you "the monarch" .instructed none)
theorem okAragornKingOfGondor : Ability.check [] aragornKingOfGondor = [] := by decide
def firmamentSage : Ability := whenever (Primitives.GameEvent.gameBecomes "night") (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you))
theorem okFirmamentSage : Ability.check [] firmamentSage = [] := by decide
def deeprootWarrior : Ability :=
  whenever (becomesBlocked thisCreature none)
    (get it (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1)) (some untilEndOfTurn))
theorem okDeeprootWarrior : Ability.check [] deeprootWarrior = [] := by decide
def borderlandMarauder : Ability :=
  whenever (attacks thisCreature) (get it (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 0)) (some untilEndOfTurn))
theorem okBorderlandMarauder : Ability.check [] borderlandMarauder = [] := by decide
def lichsMasteryLoss : Ability :=
  when (leavesBattlefield thisEnchantment) (Primitives.Instruction.conclude .loseGame (agent := Primitives.NounPhrase.you))
theorem okLichsMasteryLoss : Ability.check [] lichsMasteryLoss = [] := by decide
def phageTheUntouchable : Ability :=
  whenever (dealsCombatDamage thisCreature (a Primitives.Predicate.anyPlayer)) (Primitives.Instruction.conclude .loseGame (agent := (that
      .player)))
theorem okPhageTheUntouchable : Ability.check [] phageTheUntouchable = [] := by decide
def elderscaleWurm : Ability :=
  triggeredIf (Primitives.GameEvent.enters thisCreature none) (Primitives.Condition.compareAmt (lifeTotalOf Primitives.NounPhrase.you) .less (.lit 7))
    (setLife (.lit 7) (agent := Primitives.NounPhrase.you))
theorem okElderscaleWurm : Ability.check [] elderscaleWurm = [] := by decide
/-- Krang, Master Mind -/
def krang : Ability :=
  triggeredIf (Primitives.GameEvent.enters thisCreature none)
    (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.inZone (handOf Primitives.NounPhrase.you))) .less (.lit 4)) (Primitives.Instruction.draw Primitives.Amount.theDifference (agent :=
        Primitives.NounPhrase.you))
theorem okKrang : Ability.check [] krang = [] := by decide

def carrionGrub : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Carrion Grub", cost := some [generic 3, pip .black], types := [.creature],
      subtypes := [creatureType "Insect"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ Primitives.StaticSpec.modification thisCreature .power (Primitives.Delta.up (Primitives.Amount.letter .x)),
              Primitives.StaticSpec.modification thisCreature .toughness (Primitives.Delta.up (.lit 0)),
              Primitives.StaticSpec.letterDefinition .x
                (aggregate .max (.stat .power) (Primitives.Predicate.and [creature, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)])) ]),
          when (Primitives.GameEvent.enters thisCreature none) (mill (.lit 4) Primitives.NounPhrase.you (agent := Primitives.NounPhrase.you)) ],
      power := stat 0, toughness := stat 5 } }

/-- Once Upon a Time -/
def onceUponATimeFirstCast : Predicate := nthCastBy (.nth 1) Primitives.NounPhrase.you (.within .thisGame)
theorem okOnceUponATimeFirstCast : Predicate.check .object [] onceUponATimeFirstCast = [] := by
  decide

def youngPyromancer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Young Pyromancer", cost := some [generic 1, pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Shaman"],
      text :=
        [ whenever (Primitives.GameEvent.casts Primitives.NounPhrase.you (some (a
          (Primitives.Predicate.and [instantOrSorcery, spell]))) none)
            (Primitives.Instruction.create (.lit 1)
              (Primitives.TokenSpec.written (creatureToken 1 1 [.red] [creatureType "Elemental"])) [] (agent := Primitives.NounPhrase.you))
                  ],
      power := stat 2, toughness := stat 1 } }

def archivistOfGondor : Ability :=
  triggeredIf (dealsCombatDamage yourCommander (a Primitives.Predicate.anyPlayer)) (thereIsNo "the monarch")
    (Primitives.Instruction.gainDesignation Primitives.NounPhrase.you "the monarch" .instructed none)
theorem okArchivistOfGondor : Ability.check [] archivistOfGondor = [] := by decide

def hissingMiasma : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hissing Miasma", cost := some [generic 1, pip .black, pip .black],
      types := [.enchantment],
      text :=
        [ whenever (attacksPlayer (a creature) Primitives.NounPhrase.you) (loseLife (.lit 1) (agent := (controllerOf
            it))) ] } }

def orimsPrayer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Orim's Prayer", cost := some [generic 1, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ whenever (attacksPlayer (counted (atLeast 1) creature) Primitives.NounPhrase.you)
            (gainLife (forEach 1 (Primitives.Predicate.and [attacking, creature])) (agent := Primitives.NounPhrase.you)) ] } }

def lesserGargadon : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lesser Gargadon", cost := some [generic 2, pip .red, pip .red],
      types := [.creature], subtypes := [creatureType "Beast"],
      text :=
        [ triggeredOr (attacks thisCreature) [blocks thisCreature none]
            (sacrifice (a land) (agent := Primitives.NounPhrase.you)) ],
      power := stat 6, toughness := stat 4 } }

/-- Chub Toad -/
def chubToad : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chub Toad", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [creatureType "Frog"],
      text :=
        [ triggeredOr (blocks thisCreature none) [becomesBlocked thisCreature none]
            (get it (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 2)) (some untilEndOfTurn)) ],
      power := stat 1, toughness := stat 1 } }

/-- Giggling Skitterspike -/
def gigglingSkitterspikeTriggers : List GameEvent :=
  [blocks thisCreature none, Primitives.GameEvent.becomesTarget thisCreature (a spell)]
theorem okGigglingSkitterspikeTriggers :
    GameEvent.checkAll [] gigglingSkitterspikeTriggers = [] := by decide

/-- Naban, Dean of Iteration -/
def nabanDeanOfIteration : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Naban, Dean of Iteration", cost := some [generic 1, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.additionalTriggers
            (Primitives.GameEvent.causes
              (Primitives.Causing.event
                (Primitives.GameEvent.enters (a (Primitives.Predicate.and [ Primitives.Predicate.hasSubtype (creatureType "Wizard"),
                                    Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you ])) none))
              (Primitives.GameEvent.triggers
                (a (Primitives.Predicate.and [ Primitives.Predicate.abilityHead .anyTriggered,
                           Primitives.Predicate.abilityOf (a (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) ]))))
            (exactly 1)) ],
      power := stat 2, toughness := stat 1 } }

/-- Bloom Hulk -/
def bloomHulk : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bloom Hulk", cost := some [generic 3, pip .green], types := [.creature],
      subtypes := [creatureType "Plant", creatureType "Elemental"],
      text := [when (Primitives.GameEvent.enters thisCreature none) proliferate],
      power := stat 4, toughness := stat 4 } }

/-- Stalwart Successor -/
def stalwartSuccessorHeader : GameEvent := bareCounterEvent .put .many (a creatureYouControl)
theorem okStalwartSuccessorHeader : GameEvent.check [] stalwartSuccessorHeader = [] := by decide

/-- Hollowmurk Siege -/
def hollowmurkSiegeSultai : Ability :=
  triggeredOnlyOnce (bareCounterEvent .put .one (a creatureYouControl)) Primitives.UsageLimit.oncePerTurn
    (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you))
theorem okHollowmurkSiegeSultai : Ability.check [] hollowmurkSiegeSultai = [] := by decide

/-- Clone -/
def clone : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Clone", cost := some [generic 3, pip .blue], types := [.creature],
      subtypes := [creatureType "Shapeshifter"],
      text := [Primitives.Ability.static (Primitives.StaticSpec.entryRider thisCreature (Primitives.TokenRider.asCopyOf true (a creature) []))],
      power := stat 0, toughness := stat 0 } }

/-- Quicksilver Gargantuan -/
def quicksilverGargantuan : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Quicksilver Gargantuan", cost := some [generic 5, pip .blue, pip .blue],
      types := [.creature], subtypes := [creatureType "Shapeshifter"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.entryRider thisCreature
            (Primitives.TokenRider.asCopyOf true (a creature) [Primitives.CopyExcept.pt (.lit 7) (.lit 7)])) ],
      power := stat 7, toughness := stat 7 } }

/-- Sculpting Steel -/
def sculptingSteel : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sculpting Steel", cost := some [generic 3], types := [.artifact],
      text := [Primitives.Ability.static (Primitives.StaticSpec.entryRider thisArtifact (Primitives.TokenRider.asCopyOf true (a artifact) []))] } }

/-- Sakashima's Student -/
def sakashimasStudentCopy : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.entryRider thisCreature
    (Primitives.TokenRider.asCopyOf true (a creature) [Primitives.CopyExcept.types [] [creatureType "Ninja"]]))
theorem okSakashimasStudentCopy : Ability.check [] sakashimasStudentCopy = [] := by decide
/-- Chameleon, Master of Disguise -/
def chameleonCopy : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.entryRider thisCreature
    (Primitives.TokenRider.asCopyOf true (a (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
      [Primitives.CopyExcept.name "Chameleon, Master of Disguise"]))
theorem okChameleonCopy : Ability.check [] chameleonCopy = [] := by decide

def nefariousLichGain : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.replacement (Primitives.GameEvent.lifeChanges Primitives.NounPhrase.you .up) [] none (Primitives.Instruction.draw Primitives.Amount.thatMuch (agent := Primitives.NounPhrase.you))
      .repeatedly none)
theorem okNefariousLichGain : Ability.check [] nefariousLichGain = [] := by decide

def piousWarrior : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Pious Warrior", cost := some [generic 3, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Rebel", creatureType "Warrior"],
      text := [whenever (Primitives.GameEvent.isDealtDamage .combatOnly thisCreature) (gainLife Primitives.Amount.thatMuch (agent :=
          Primitives.NounPhrase.you))],
      power := stat 2, toughness := stat 3 } }

def sanguineBond : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sanguine Bond", cost := some [generic 3, pip .black, pip .black],
      types := [.enchantment],
      text := [whenever (Primitives.GameEvent.lifeChanges Primitives.NounPhrase.you .up) (loseLife Primitives.Amount.thatMuch (agent := (target Primitives.Predicate.opponent)))]
          } }

def exquisiteBlood : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Exquisite Blood", cost := some [generic 4, pip .black], types := [.enchantment],
      text := [whenever (Primitives.GameEvent.lifeChanges anOpponent .down) (gainLife Primitives.Amount.thatMuch (agent := Primitives.NounPhrase.you))] } }

/-- Agate-Blade Assassin -/
def agateBladeAssassin : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Agate-Blade Assassin", cost := some [generic 1, pip .black], types := [.creature],
      subtypes := [creatureType "Lizard", creatureType "Assassin"],
      text :=
        [ whenever (attacks thisCreature)
            (Primitives.Instruction.sequentially
              [loseLife (.lit 1) (agent := (Primitives.NounPhrase.combatPlayer .defending)), gainLife (.lit 1) (agent :=
                  Primitives.NounPhrase.you)]) ],
      power := stat 1, toughness := stat 3 } }

/-- Bard, Heir of Girion -/
def bardHeirOfGirion : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bard, Heir of Girion", cost := some [generic 2, pip .white, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Archer"],
      text :=
        [ keyword "Reach", keyword "Vigilance",
          Primitives.Ability.static (getsPt (allOf (otherCreatureYouControl thisCreature)) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1))),
          whenever (attacks Primitives.NounPhrase.you) (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)) ],
      power := stat 4, toughness := stat 4 } }

/-- Fiend Binder -/
def fiendBinder : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fiend Binder", cost := some [generic 3, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Soldier"],
      text :=
        [ whenever (attacks thisCreature)
            (Primitives.Instruction.setStatus .tapped
              (target (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller (Primitives.NounPhrase.combatPlayer .defending)]))) ],
      power := stat 3, toughness := stat 2 } }

/-- Souls of the Faultless -/
def soulsOfTheFaultlessDrain : Ability :=
  whenever (Primitives.GameEvent.isDealtDamage .combatOnly thisCreature)
    (loseLife Primitives.Amount.thatMuch (agent := (Primitives.NounPhrase.combatPlayer .attacking)))
theorem okSoulsOfTheFaultlessDrain : Ability.check [] soulsOfTheFaultlessDrain = [] := by decide

def unstableShapeshifter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Unstable Shapeshifter", cost := some [generic 3, pip .blue], types := [.creature],
      subtypes := [creatureType "Shapeshifter"],
      text :=
        [ whenever (Primitives.GameEvent.enters (a (Primitives.Predicate.and [creature, Primitives.Predicate.otherThan Primitives.NounPhrase.this])) none)
            (Primitives.Instruction.establish (Primitives.StaticSpec.copyChange thisCreature (that (.type .creature)) [Primitives.CopyExcept.thisAbility])
              none) ],
      power := stat 0, toughness := stat 1 } }

def prismRing : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Prism Ring", cost := some [generic 1], types := [.artifact],
      text :=
        [ Primitives.Ability.static (entersChoosing thisArtifact .color),
          whenever (Primitives.GameEvent.casts Primitives.NounPhrase.you (some (a
            (Primitives.Predicate.and [spell, ofChosen .color]))) none) (gainLife (.lit 1) (agent
              := Primitives.NounPhrase.you)) ] } }

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
        [ whenever (Primitives.GameEvent.casts (a Primitives.Predicate.anyPlayer) (some (a spell))
          none)
            (Primitives.Instruction.sequentially
              [ gainLife (Primitives.Amount.letter .x) (agent := they),
                Primitives.Instruction.define .x (countOf (Primitives.Predicate.and [Primitives.Predicate.inZone graveyard, Primitives.Predicate.named (Primitives.NameSource.sameAs (that .spell))])) ]) ] } }

def chromeReplicator : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chrome Replicator", cost := some [generic 5], types := [.artifact, .creature],
      subtypes := [creatureType "Construct"],
      text :=
        [ triggeredIf (Primitives.GameEvent.enters thisCreature none)
            (Primitives.Condition.exists_ (withTheSameName
              (counted (atLeast 2)
                (Primitives.Predicate.and [permanent, Primitives.Predicate.not land, nontoken, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))))
            (create (.lit 1)
              { characteristics :=
                { types := [.artifact, .creature], subtypes := [creatureType "Construct"],
                  power := stat 4, toughness := stat 4 } }) ],
      power := stat 4, toughness := stat 4 } }

def admiralsOrder : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Admiral's Order", cost := some [generic 1, pip .blue, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.static (onlyWhile (Primitives.StaticSpec.altCost Primitives.NounPhrase.this (some (Primitives.Cost.mana [pip .blue])))
            (happened (Primitives.GameEvent.combat .attackerOf (relative .player) none)
              Primitives.NounPhrase.you .thisTurn)),
          Primitives.Ability.spell none (Primitives.Instruction.counterSpell (target spell)) ] } }

def fyndhornDruid : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fyndhorn Druid", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [creatureType "Elf", creatureType "Druid"],
      text :=
        [ triggeredIf (Primitives.GameEvent.dies thisCreature) (happened
          (Primitives.GameEvent.combat .blockedBy (Primitives.NounPhrase.asMarker .permanent
          (relative .object)) none) it .thisTurn)
            (gainLife (.lit 4) (agent := Primitives.NounPhrase.you)) ],
      power := stat 2, toughness := stat 2 } }

def kamiOfTerribleSecrets : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Kami of Terrible Secrets", cost := some [generic 3, pip .black], types := [.creature],
      subtypes := [creatureType "Spirit"],
      text :=
        [ triggeredIf (Primitives.GameEvent.enters thisCreature none)
            (Primitives.Condition.and [ exists_ (Primitives.Predicate.and [artifact, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]),
                    exists_ (Primitives.Predicate.and [enchantment, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]) ])
            (Primitives.Instruction.sequentially [Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you), gainLife (.lit 1) (agent := Primitives.NounPhrase.you)]) ],
      power := stat 3, toughness := stat 4 } }

def dreadSlaver : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dread Slaver", cost := some [generic 3, pip .black, pip .black],
      types := [.creature], subtypes := [creatureType "Zombie", creatureType "Horror"],
      text :=
        [ whenever
            (Primitives.GameEvent.dies (a (Primitives.Predicate.and [creature, happenedTo
              (Primitives.GameEvent.dealsDamage .any thisCreature (some
              (Primitives.NounPhrase.asMarker .permanent (relative .object)))) .thisTurn])))
            (Primitives.Instruction.sequentially
              [ putOntoBattlefieldUnderYourControl it,
                become (that (.type .creature))
                  { characteristics := { colors := [.black], subtypes := [creatureType "Zombie"] } }
                  none ]) ],
      power := stat 3, toughness := stat 5 } }

/-- Steppe Lynx -/
def steppeLynx : Ability :=
  abilityWord "landfall"
    (whenever (Primitives.GameEvent.enters (a (Primitives.Predicate.and [land, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) none)
      (get thisCreature (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 2)) (some untilEndOfTurn)))
theorem okSteppeLynx : Ability.check [] steppeLynx = [] := by decide
/-- Owlbear -/
def owlbear : Ability :=
  flavorWord "Keen Senses" (when (Primitives.GameEvent.enters thisCreature none) (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)))
theorem okOwlbear : Ability.check [] owlbear = [] := by decide
/-- Netherese Puzzle-Ward -/
def netheresePuzzleWardIllumination : Ability :=
  whenever youRollHighestNatural (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you))
theorem okNetheresePuzzleWardIllumination :
    Ability.check [] netheresePuzzleWardIllumination = [] := by decide
/-- Farideh, Devil's Chosen -/
def faridehResultRead : Ability :=
  whenever (Primitives.GameEvent.rollsDice Primitives.NounPhrase.you .many none Primitives.RollWatch.anyResult)
    (Primitives.Instruction.sequentially
      [ gain thisCreature (keyword "Flying") (some untilEndOfTurn),
        gain thisCreature (keyword "Menace") (some untilEndOfTurn),
        doIf (Primitives.Condition.anyResultIs .atLeast (.lit 10)) (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)) ])
theorem okFaridehResultRead : Ability.check [] faridehResultRead = [] := by decide
/-- Jaws of Defeat -/
def jawsOfDefeat : Ability :=
  whenever (Primitives.GameEvent.enters (a creatureYouControl) none)
    (loseLife
      (Primitives.Amount.arith .differenceBetween (Primitives.Amount.statOf (.stat .power) (that (.type .creature)))
        (Primitives.Amount.statOf (.stat .toughness) (that (.type .creature)))) (agent := (target Primitives.Predicate.opponent)))
theorem okJawsOfDefeat : Ability.check [] jawsOfDefeat = [] := by decide
/-- Defiling Daemogoth -/
def defilingDaemogothDrain : Instruction :=
  Primitives.Instruction.sequentially
    [loseLife (Primitives.Amount.letter .x) (agent := (each Primitives.Predicate.opponent)),
      Primitives.Instruction.define .x (eventSum (Primitives.GameEvent.lifeChanges (relative
      .player) .up) Primitives.NounPhrase.you .thisTurn)]
theorem okDefilingDaemogothDrain : Instruction.check [] defilingDaemogothDrain = [] := by decide
def skullsporeNexusTrigger : Ability :=
  whenever (Primitives.GameEvent.dies (counted (atLeast 1) (Primitives.Predicate.and [nontoken, creatureYouControl])))
    (create (.lit 1)
      (creatureTokenOf (Primitives.Amount.aggregate .sum (.stat .power) (those .card))
        (Primitives.Amount.aggregate .sum (.stat .power) (those .card))
        [.green] [creatureType "Fungus", creatureType "Dinosaur"]))
theorem okSkullsporeNexusTrigger : Ability.check [] skullsporeNexusTrigger = [] := by decide
/-- Wavebreak Hippocamp -/
def wavebreakHippocamp : Ability :=
  triggeredOnlyDuring (Primitives.GameEvent.nthOccurrence (.nth 1) none (Primitives.GameEvent.casts
    Primitives.NounPhrase.you (some (a spell)) none))
    (Primitives.Timing.duringPart .turn (some (each Primitives.Predicate.opponent))) (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you))
theorem okWavebreakHippocamp : Ability.check [] wavebreakHippocamp = [] := by decide
/-- Midnight Clock -/
def midnightClockHeader : GameEvent :=
  Primitives.GameEvent.nthOccurrence (.nth 12) none (counterEvent .put (.named "Hour") .one thisArtifact)
theorem okMidnightClockHeader : GameEvent.check [] midnightClockHeader = [] := by decide
/-- Political Triumph -/
def politicalTriumphHeader : GameEvent :=
  Primitives.GameEvent.nthOccurrence (.nth 4) none (counterEvent .put (.named "Plan") .one thisEnchantment)
theorem okPoliticalTriumphHeader : GameEvent.check [] politicalTriumphHeader = [] := by decide
/-- Thought Lash -/
def thoughtLashTrigger : Ability :=
  when (Primitives.GameEvent.paysCost (some (a Primitives.Predicate.anyPlayer)) .unpaid thisEnchantment "CumulativeUpkeep")
    (exile (each (Primitives.Predicate.inZone (libraryOf they))) (agent := some (that .player)))
theorem okThoughtLashTrigger : Ability.check [] thoughtLashTrigger = [] := by decide
/-- Heart of Bogardan -/
def heartOfBogardanHeader : GameEvent :=
  Primitives.GameEvent.paysCost (some (a Primitives.Predicate.anyPlayer)) .unpaid thisEnchantment "CumulativeUpkeep"
theorem okHeartOfBogardanHeader : GameEvent.check [] heartOfBogardanHeader = [] := by decide
/-- Balduvian Fallen -/
def balduvianFallenHeader : GameEvent := Primitives.GameEvent.paysCost none .paid thisCreature "CumulativeUpkeep"
theorem okBalduvianFallenHeader : GameEvent.check [] balduvianFallenHeader = [] := by decide
/-- Stormscape Battlemage -/
def stormscapeBattlemageFirstKicker : Ability :=
  triggeredIf (Primitives.GameEvent.enters thisCreature none)
    (costWasPaid (.byNthKeyword (.nth 1) "Kicker") none thisCreature) (gainLife (.lit 3) (agent :=
        Primitives.NounPhrase.you))
theorem okStormscapeBattlemageFirstKicker :
    Ability.check [] stormscapeBattlemageFirstKicker = [] := by decide
/-- All-Seeing Arbiter -/
def allSeeingArbiterHeader : GameEvent :=
  Primitives.GameEvent.verbedEvent (some Primitives.NounPhrase.you) (.action "Discard") (some (a
    (Primitives.Predicate.inZone hand))) none none
theorem okAllSeeingArbiterHeader : GameEvent.check [] allSeeingArbiterHeader = [] := by decide

/-- Liliana's Caress -/
def lilianasCaress : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Liliana's Caress", cost := some [generic 1, pip .black], types := [.enchantment],
      text :=
        [ whenever (Primitives.GameEvent.verbedEvent (some anOpponent) (.action "Discard") (some (a
          (Primitives.Predicate.inZone hand))) none none)
            (loseLife (.lit 2) (agent := they)) ] } }

/-- Scheming Aspirant -/
def schemingAspirant : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Scheming Aspirant", cost := some [generic 1, pip .black], types := [.creature],
      subtypes := [creatureType "Phyrexian", creatureType "Advisor"],
      text :=
        [ whenever (Primitives.GameEvent.verbedEvent (some Primitives.NounPhrase.you) (.action
          "Proliferate") none none none)
            (Primitives.Instruction.sequentially [loseLife (.lit 2) (agent := (each Primitives.Predicate.opponent)), gainLife (.lit 2) (agent :=
                Primitives.NounPhrase.you)]) ],
      power := stat 1, toughness := stat 3 } }

/-- Reciprocate -/
def reciprocate : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Reciprocate", cost := some [pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (exile
            (target (Primitives.Predicate.and [creature, happenedTo
              (Primitives.GameEvent.dealsDamage .any (relative .object) (some
              Primitives.NounPhrase.you)) .thisTurn]))) ] } }

/-- Tahngarth, First Mate -/
def tahngarthHeader : GameEvent := Primitives.GameEvent.attacksWith anOpponent none (counted (atLeast 1) creature)
theorem okTahngarthHeader : GameEvent.check [] tahngarthHeader = [] := by decide
/-- Myth Unbound -/
def mythUnboundTrigger : Ability :=
  whenever (putIntoFrom yourCommander commandZone Primitives.EventSource.anywhere) (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you))
theorem okMythUnboundTrigger : Ability.check [] mythUnboundTrigger = [] := by decide

/-- Commander's Insignia -/
def commandersInsignia : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Commander's Insignia", cost := some [generic 2, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ Primitives.Ability.static (getsPt (allOf creatureYouControl)
            (Primitives.Delta.up (eventCount (Primitives.GameEvent.casts (relative .player) (some
              (Primitives.NounPhrase.asMarker .spell yourCommander)) (some
              (Primitives.EventSource.zones [commandZone]))) Primitives.NounPhrase.you .thisGame))
            (Primitives.Delta.up (eventCount (Primitives.GameEvent.casts (relative .player) (some
              (Primitives.NounPhrase.asMarker .spell yourCommander)) (some
              (Primitives.EventSource.zones [commandZone]))) Primitives.NounPhrase.you .thisGame)))
              ] } }

/-- Faith's Reward -/
def faithsReward : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Faith's Reward", cost := some [generic 3, pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (move
            (allOf (Primitives.Predicate.and [ permanentCard, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you),
                           Primitives.Predicate.happenedTo (Primitives.LookbackClause.mk
                             (Primitives.GameEvent.putInto (relative .object) (graveyardOf
                             Primitives.NounPhrase.you) (some (Primitives.EventSource.zones
                             [battlefield]))) .thisTurn) ]))
            battlefield) ] } }

/-- Oscorp Industries -/
def oscorpIndustriesReturn : Ability :=
  when (Primitives.GameEvent.enters thisLand (some (Primitives.EventSource.zones [graveyard]))) (loseLife (.lit 2) (agent := Primitives.NounPhrase.you))
theorem okOscorpIndustriesReturn : Ability.check [] oscorpIndustriesReturn = [] := by decide
def theLostAndTheDamnedEntryArm : GameEvent :=
  Primitives.GameEvent.enters (a (Primitives.Predicate.and [land, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) (some (Primitives.EventSource.anywhereBut [handOf Primitives.NounPhrase.you]))
theorem okTheLostAndTheDamnedEntryArm :
    GameEvent.check [] theLostAndTheDamnedEntryArm = [] := by decide
/-- Forsaken Wastes -/
def forsakenWastesTargeted : Ability :=
  whenever (Primitives.GameEvent.becomesTarget thisEnchantment (a spell))
    (loseLife (.lit 5) (agent := (controllerOf (that .spell))))
theorem okForsakenWastesTargeted : Ability.check [] forsakenWastesTargeted = [] := by decide

/-- Fblthp, the Lost -/
def fblthp : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fblthp, the Lost", cost := some [generic 1, pip .blue],
      supertypes := [.legendary], types := [.creature], subtypes := [creatureType "Homunculus"],
      text :=
        [ when (Primitives.GameEvent.enters Primitives.NounPhrase.this none)
            (Primitives.Instruction.replace (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you))
              (Primitives.Instruction.doIf (Primitives.Condition.or
                  [ Primitives.Condition.happened it (Primitives.LookbackClause.mk
                    (Primitives.GameEvent.enters (relative .object) (some
                    (Primitives.EventSource.zones [yourLibrary]))) .triggering),
                    Primitives.Condition.matches it (Primitives.Predicate.castFrom yourLibrary) ])
                (Primitives.Instruction.draw (.lit 2) (agent := Primitives.NounPhrase.you)) none)),
          when (Primitives.GameEvent.becomesTarget Primitives.NounPhrase.this (a spell)) (shuffleInto Primitives.NounPhrase.this (agent := Primitives.NounPhrase.you)) ],
      power := stat 1, toughness := stat 1 } }

/-- Frost Walker -/
def frostWalker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Frost Walker", cost := some [generic 1, pip .blue], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text :=
        [ when (Primitives.GameEvent.becomesTarget thisCreature (a (Primitives.Predicate.or [spell, Primitives.Predicate.abilityHead .anyOnStack])))
            (sacrificeIt (agent := Primitives.NounPhrase.you)) ],
      power := stat 4, toughness := stat 1 } }

/-- Loaming Shaman -/
def loamingShaman : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Loaming Shaman", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [creatureType "Centaur", creatureType "Shaman"],
      text :=
        [ when (Primitives.GameEvent.enters thisCreature none)
            (shuffleInto
              (Primitives.NounPhrase.described (Primitives.DetPhrase.target anyNumber) (Primitives.Predicate.inZone (graveyardOf they))) (agent := (target
                  Primitives.Predicate.anyPlayer))) ],
      power := stat 3, toughness := stat 2 } }

/-- Debris Beetle -/
def debrisBeetleTrigger : Ability :=
  when (Primitives.GameEvent.enters thisVehicle none)
    (Primitives.Instruction.sequentially [loseLife (.lit 3) (agent := (each Primitives.Predicate.opponent)), gainLife (.lit 3) (agent := Primitives.NounPhrase.you)])
theorem okDebrisBeetleTrigger : Ability.check [] debrisBeetleTrigger = [] := by decide
/-- Roads Go Ever, Ever On's chapters II and III -/
def roadsGoEverEverOnChapters : Ability :=
  when (Primitives.GameEvent.chapterMark [2, 3]) (move (a (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.exiledWith thisSaga])) hand)
theorem okRoadsGoEverEverOnChapters : Ability.check [] roadsGoEverEverOnChapters = [] := by decide
/-- Wurmwall Sweeper -/
def wurmwallSweeperTrigger : Ability := when (Primitives.GameEvent.enters thisSpacecraft none) (surveil (.lit 2) (agent
    := Primitives.NounPhrase.you))
theorem okWurmwallSweeperTrigger : Ability.check [] wurmwallSweeperTrigger = [] := by decide
/-- Case of the Crimson Pulse -/
def caseOfTheCrimsonPulseTrigger : Ability :=
  when (Primitives.GameEvent.enters thisCase none)
    (Primitives.Instruction.sequentially [discard (a (Primitives.Predicate.inZone hand)) (agent := Primitives.NounPhrase.you), Primitives.Instruction.draw (.lit 2) (agent := Primitives.NounPhrase.you)])
theorem okCaseOfTheCrimsonPulseTrigger : Ability.check [] caseOfTheCrimsonPulseTrigger = [] := by
  decide

/-- Ferocious Pup -/
def ferociousPup : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ferocious Pup", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [creatureType "Wolf"],
      text :=
        [ when (Primitives.GameEvent.enters thisCreature none)
            (create (.lit 1) (creatureToken 2 2 [.green] [creatureType "Wolf"])) ],
      power := stat 0, toughness := stat 1 } }

/-- Agency Outfitter's search -/
def agencyOutfitterSearch : Instruction :=
  Primitives.Instruction.sequentially
    [ offer
        (Primitives.Instruction.sequentially
          [ searchZonesOf Primitives.NounPhrase.you (exactly 1)
              (Primitives.Predicate.or [Primitives.Predicate.named (Primitives.NameSource.printed "Magnifying Glass"), Primitives.Predicate.named (Primitives.NameSource.printed "Thinking Cap")]),
            putOntoBattlefield foundCard ]) (agent := Primitives.NounPhrase.you),
      Primitives.Instruction.doIf (happened (Primitives.GameEvent.verbedEvent (some (relative
        .player)) (.action "Search") none none (some yourLibrary)) Primitives.NounPhrase.you
        .thisWay) shuffle none ]
theorem okAgencyOutfitterSearch : Instruction.check [] agencyOutfitterSearch = [] := by decide

/-- Trouble in Pairs' -/
def troubleInPairsArms : Ability :=
  Primitives.Ability.triggered (Primitives.GameEvent.attacksWith anOpponent (some Primitives.NounPhrase.you) (counted (atLeast 2) creature))
    [ Primitives.GameEvent.nthOccurrence (.nth 2) (some .turn) (Primitives.GameEvent.draws (a Primitives.Predicate.opponent)),
      Primitives.GameEvent.nthOccurrence (.nth 2) (some .turn) (Primitives.GameEvent.casts (a
        Primitives.Predicate.opponent) (some (a spell)) none) ]
    none [] none none none (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you))
theorem okTroubleInPairsArms : Ability.check [] troubleInPairsArms = [] := by decide

/-- Bioplasm -/
def bioplasm : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bioplasm", cost := some [generic 3, pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Ooze"],
      text :=
        [ whenever (attacks thisCreature)
            (Primitives.Instruction.sequentially
              [ exile (topSlice (.lit 1)),
                Primitives.Instruction.doIf bioplasmCardTest
                  (get thisCreature
                    (Primitives.Delta.up (Primitives.Amount.statOf (.stat .power)
                      (theVerbed (.action "Exile") (.ofType .card .creature) .attributive .one)))
                    (Primitives.Delta.up (Primitives.Amount.statOf (.stat .toughness) (itVerbed (.action "Exile"))))
                    (some Primitives.Duration.thisTurn))
                  none ]) ],
      power := stat 4, toughness := stat 4 } }

/-- Hostile Investigator -/
def hostileInvestigatorHeader : GameEvent :=
  Primitives.GameEvent.verbedEvent (some (counted (atLeast 1) Primitives.Predicate.anyPlayer)) (.action "Discard")
    (some (counted (atLeast 1) Primitives.Predicate.isCard)) none none
theorem okHostileInvestigatorHeader : GameEvent.check [] hostileInvestigatorHeader = [] := by decide

/-- Hallowed Moonlight -/
def hallowedMoonlight : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hallowed Moonlight", cost := some [generic 1, pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ replaceEvent (Primitives.GameEvent.enters (a (Primitives.Predicate.and [creature, Primitives.Predicate.not Primitives.Predicate.wasCast])) none) (exile it)
                (some untilEndOfTurn),
              Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you) ]) ] } }

/-- Gather Specimens -/
def gatherSpecimens : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gather Specimens", cost := some [generic 3, pip .blue, pip .blue, pip .blue],
      types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.establish
            (Primitives.StaticSpec.entryRider (a (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller anOpponent])) (Primitives.TokenRider.under Primitives.NounPhrase.you))
            (some Primitives.Duration.thisTurn)) ] } }

/-- Don't Blink's replacement, without its written agent -/
def dontBlinkReplacement : Instruction :=
  Primitives.Instruction.establish
    (Primitives.StaticSpec.replacement (Primitives.GameEvent.enters (counted (atLeast 1) creature) (some (Primitives.EventSource.zones [exileZone])))
      [Primitives.GameEvent.enters (counted (atLeast 1) (Primitives.Predicate.and [creature, Primitives.Predicate.castFrom exileZone])) none] none
      (shuffleInto them (agent := Primitives.NounPhrase.you)) .repeatedly none)
    (some untilEndOfTurn)
theorem okDontBlinkReplacement : Instruction.check [] dontBlinkReplacement = [] := by decide

/-- Seasoned Warrenguard -/
def seasonedWarrenguard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Seasoned Warrenguard", cost := some [pip .white], types := [.creature],
      subtypes := [creatureType "Rabbit", creatureType "Warrior"],
      text :=
        [ triggeredWhile (attacks thisCreature)
            (Primitives.Concurrent.whileTrue (exists_ (Primitives.Predicate.and [Primitives.Predicate.isToken, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))
            (get thisCreature (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 0)) (some untilEndOfTurn)) ],
      power := stat 1, toughness := stat 2 } }

/-- Brazen Blademaster -/
def brazenBlademaster : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Brazen Blademaster", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [creatureType "Orc", creatureType "Pirate"],
      text :=
        [ triggeredWhile (attacks thisCreature)
            (Primitives.Concurrent.whileTrue
              (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.and [artifact, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .atLeast
                (.lit 2)))
            (get it (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 1)) (some untilEndOfTurn)) ],
      power := stat 2, toughness := stat 3 } }

/-- Autarch Mammoth -/
def autarchMammothLine : Ability :=
  triggeredJoined (Primitives.GameEvent.enters thisCreature none)
    [ joinedHeadWhile (attacks thisCreature)
        (Primitives.Concurrent.whileTrue (Primitives.Condition.matches thisCreature (Primitives.Predicate.hasDesignation "saddled" none))) ]
    (create (.lit 1) (creatureToken 3 3 [.green] [creatureType "Elephant"]))
theorem okAutarchMammothLine : Ability.check [] autarchMammothLine = [] := by decide
/-- Altar of the Brood -/
def altarOfTheBrood : Ability :=
  whenever (Primitives.GameEvent.enters (a (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, Primitives.Predicate.otherThan Primitives.NounPhrase.this])) none)
    (mill (.lit 1) they (agent := (each Primitives.Predicate.opponent)))
theorem okAltarOfTheBrood : Ability.check [] altarOfTheBrood = [] := by decide
/-- Foul Emissary -/
def foulEmissaryLine : Ability :=
  triggeredWhile (Primitives.GameEvent.verbedEvent (some Primitives.NounPhrase.you) (.action
    "Sacrifice") (some thisCreature) none none)
    (Primitives.Concurrent.whileDoing (Primitives.GameEvent.casts Primitives.NounPhrase.you (some (a
      (Primitives.Predicate.and [spell, Primitives.Predicate.hasKeyword (.the "Emerge")]))) none))
    (create (.lit 1) (creatureToken 3 2 [] [creatureType "Eldrazi", creatureType "Horror"]))
theorem okFoulEmissaryLine : Ability.check [] foulEmissaryLine = [] := by decide

/-- Bramble Elemental -/
def brambleElemental : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bramble Elemental", cost := some [generic 3, pip .green, pip .green],
      types := [.creature], subtypes := [creatureType "Elemental"],
      text :=
        [ whenever (Primitives.GameEvent.attachment .attached (a (Primitives.Predicate.hasSubtype (enchantmentType "Aura"))) thisCreature)
            (create (.lit 2) (creatureToken 1 1 [.green] [creatureType "Saproling"])) ],
      power := stat 4, toughness := stat 4 } }

/-- Stone Haven Outfitter -/
def stoneHavenOutfitter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stone Haven Outfitter", cost := some [generic 1, pip .white], types := [.creature],
      subtypes := [creatureType "Kor", creatureType "Artificer", creatureType "Ally"],
      text :=
        [ Primitives.Ability.static (getsPt (allOf (Primitives.Predicate.and [creatureYouControl, Primitives.Predicate.isAttached (some .equipped)]))
            (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1))),
          whenever (Primitives.GameEvent.dies (a (Primitives.Predicate.and [creatureYouControl, Primitives.Predicate.isAttached (some .equipped)])))
            (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)) ],
      power := stat 2, toughness := stat 2 } }

/-- Cloud, Ex-SOLDIER -/
def cloudExSoldierAttach : Ability :=
  when (Primitives.GameEvent.enters thisCreature none)
    (attachToIt
      (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 1))
        (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (artifactType "Equipment"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))
theorem okCloudExSoldierAttach : Ability.check [] cloudExSoldierAttach = [] := by decide
/-- Akiri, Fearless Voyager -/
def akiriEquippedAttackers : Ability :=
  whenever
    (Primitives.GameEvent.attacksWith Primitives.NounPhrase.you (some (a Primitives.Predicate.anyPlayer))
      (counted (atLeast 1) (Primitives.Predicate.and [creatureYouControl, Primitives.Predicate.isAttached (some .equipped)])))
    (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you))
theorem okAkiriEquippedAttackers : Ability.check [] akiriEquippedAttackers = [] := by decide
/-- Vexing Bauble -/
def vexingBaubleTrigger : Ability :=
  triggeredIf (Primitives.GameEvent.casts (a Primitives.Predicate.anyPlayer) (some (a spell)) none)
    (noManaSpentToCast it)
    (Primitives.Instruction.counterSpell (that .spell))
theorem okVexingBaubleTrigger : Ability.check [] vexingBaubleTrigger = [] := by decide

/-- Void Mirror -/
def voidMirror : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Void Mirror", cost := some [generic 2], types := [.artifact],
      text :=
        [ triggeredIf (Primitives.GameEvent.casts (a Primitives.Predicate.anyPlayer) (some (a
          spell)) none) (noColoredManaSpentToCast it)
            (Primitives.Instruction.counterSpell (that .spell)) ] } }

/-- Blood Sun -/
def bloodSun : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blood Sun", cost := some [generic 2, pip .red], types := [.enchantment],
      text :=
        [ when (Primitives.GameEvent.enters thisEnchantment none) (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)),
          Primitives.Ability.static (Primitives.StaticSpec.allAbilityLoss (allOf land) (some Primitives.Predicate.isManaAbility)) ] } }

/-- Promise of Bunrei -/
def promiseOfBunrei : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Promise of Bunrei", cost := some [generic 2, pip .white], types := [.enchantment],
      text :=
        [ when (Primitives.GameEvent.dies (a creatureYouControl))
            (Primitives.Instruction.doIfDone (sacrifice thisEnchantment (agent := Primitives.NounPhrase.you))
              (some (create (.lit 4) (creatureToken 1 1 [] [creatureType "Spirit"]))) none) ] } }

/-- Grave Peril -/
def gravePeril : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Grave Peril", cost := some [generic 1, pip .black], types := [.enchantment],
      text :=
        [ when (Primitives.GameEvent.enters (a (Primitives.Predicate.and [creature, Primitives.Predicate.not (Primitives.Predicate.colorIs .black)])) none)
            (Primitives.Instruction.doIfDone (sacrifice thisEnchantment (agent := Primitives.NounPhrase.you)) (some (destroy (that (.type
                .creature))))
              none) ] } }

/-- Blood Reckoning -/
def bloodReckoning : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blood Reckoning", cost := some [generic 3, pip .black], types := [.enchantment],
      text :=
        [ whenever
            (attacksPlayer (a creature)
              (youOr (a (Primitives.Predicate.and [Primitives.Predicate.hasType .planeswalker, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))))
            (loseLife (.lit 1) (agent := (controllerOf (that (.type .creature))))) ] } }

/-- Jeskai Ascendancy's first trigger -/
def jeskaiAscendancyPump : Ability :=
  whenever (Primitives.GameEvent.casts Primitives.NounPhrase.you (some (a (Primitives.Predicate.and
    [spell, Primitives.Predicate.not creature]))) none)
    (Primitives.Instruction.sequentially
      [ get (bare creatureYouControl) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1)) (some untilEndOfTurn),
        untap (those (.type .creature)) ])
theorem okJeskaiAscendancyPump : Ability.check [] jeskaiAscendancyPump = [] := by decide
def herosDemise : Instruction := destroy (target (Primitives.Predicate.and [creature, Primitives.Predicate.hasSupertype .legendary]))
theorem okHerosDemise : Instruction.check [] herosDemise = [] := by decide

def repayInKind : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Repay in Kind", cost := some [generic 5, pip .black, pip .black], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none
            (setLife (aggregate .min (.playerStat .lifeTotal) Primitives.Predicate.anyPlayer) (agent := (each
                Primitives.Predicate.anyPlayer))) ] } }

/-- Squelch -/
def squelch : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Squelch", cost := some [generic 1, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [Primitives.Instruction.counterSpell (target (Primitives.Predicate.abilityHead .anyActivated)), Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)]) ]
                } }

theorem bioplasmTwoCandidates : countOnes .object bioplasmAfterExile = 2 := by decide

end Semantics.Cards

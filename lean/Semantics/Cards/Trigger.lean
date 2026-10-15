import Semantics
import Semantics.Macros
import Semantics.Macros.Declared
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
  delayWithin (Primitives.GameEvent.dies (target creature)) thisTurn (move (that .card) battlefield)
theorem okGracefulReprieve : Instruction.check (DRS.ofList []) gracefulReprieve = [] := by decide

def cloudkinSeer : Ability := when (Primitives.GameEvent.enters thisCreature none) (Primitives.Instruction.draw (.lit 1))
theorem okCloudkinSeer : Ability.check (DRS.ofList []) cloudkinSeer = [] := by decide
def promiseOfTomorrow : Ability := whenever (Primitives.GameEvent.dies (a creatureYouControl)) (exile it)
theorem okPromiseOfTomorrow : Ability.check (DRS.ofList []) promiseOfTomorrow = [] := by decide
def libraryLarcenist : Ability := whenever (attacks thisCreature) (Primitives.Instruction.draw (.lit 1))
theorem okLibraryLarcenist : Ability.check (DRS.ofList []) libraryLarcenist = [] := by decide
def jhessianThief : Ability :=
  whenever (dealsCombatDamage thisCreature (a Primitives.Predicate.anyPlayer)) (Primitives.Instruction.draw (.lit 1))
theorem okJhessianThief : Ability.check (DRS.ofList []) jhessianThief = [] := by decide
def scholarOfStars : Ability :=
  triggeredIf (Primitives.GameEvent.enters thisCreature none)
    (exists_ (Primitives.Predicate.and [artifact, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) (Primitives.Instruction.draw (.lit 1))
theorem okScholarOfStars : Ability.check (DRS.ofList []) scholarOfStars = [] := by decide

def miserysShadow : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.instead (Primitives.WouldClause.would Primitives.Subject.any (Primitives.Instruction.move (a (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller (a Primitives.Predicate.opponent)])) (Primitives.EventSource.zones [battlefield]) (some graveyard) .departure none [])) (exile it) none .repeatedly none)
theorem okMiserysShadow : Ability.check (DRS.ofList []) miserysShadow = [] := by decide

def beastWhisperer : Ability :=
  whenever (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Instruction.procedure .cast ((a (Primitives.Predicate.and
    [creature, spell]))) Primitives.PlayPayment.itsOwnCost none none none)) (Primitives.Instruction.draw (.lit 1))
theorem okBeastWhisperer : Ability.check (DRS.ofList []) beastWhisperer = [] := by decide
def mesmericOrb : Ability :=
  whenever (Primitives.GameEvent.does Primitives.Subject.any (Primitives.Instruction.setStatus .untapped (a permanent)))
    (instruct (controllerOf (that .permanent)) (mill (.lit 1) they))
theorem okMesmericOrb : Ability.check (DRS.ofList []) mesmericOrb = [] := by decide
def secretPlans : Ability :=
  whenever (Primitives.GameEvent.does Primitives.Subject.any (Primitives.Instruction.setStatus .faceUp (a (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))))
    (Primitives.Instruction.draw (.lit 1))
theorem okSecretPlans : Ability.check (DRS.ofList []) secretPlans = [] := by decide

/-- Teferi's Imp -/
def teferisImpPhasesOut : Ability :=
  whenever (Primitives.GameEvent.does Primitives.Subject.any (Primitives.Instruction.setStatus .phasedOut thisCreature)) (discard (a (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.inZone yourHand])))
theorem okTeferisImpPhasesOut : Ability.check (DRS.ofList []) teferisImpPhasesOut = [] := by decide
/-- Teferi's Imp -/
def teferisImpPhasesIn : Ability :=
  whenever (Primitives.GameEvent.does Primitives.Subject.any (Primitives.Instruction.setStatus .phasedIn thisCreature)) (Primitives.Instruction.draw (.lit 1))
theorem okTeferisImpPhasesIn : Ability.check (DRS.ofList []) teferisImpPhasesIn = [] := by decide

/-- Oubliette -/
def oubliette : Ability :=
  when (Primitives.GameEvent.enters thisEnchantment none)
    (phaseOutUntil (target creature) (leavesBattlefield thisEnchantment))
theorem okOubliette : Ability.check (DRS.ofList []) oubliette = [] := by decide

def shimmeringEfreet : Ability :=
  whenever (Primitives.GameEvent.does Primitives.Subject.any (Primitives.Instruction.setStatus .phasedIn thisCreature)) (Primitives.Instruction.setStatus .phasedOut (target creature))
theorem okShimmeringEfreet : Ability.check (DRS.ofList []) shimmeringEfreet = [] := by decide
def hollowhengeSpirit : Ability :=
  when (Primitives.GameEvent.enters thisCreature none)
    (Primitives.Instruction.removeFromCombat (target (Primitives.Predicate.and [creature, Primitives.Predicate.or [attacking, blocking]])))
theorem okHollowhengeSpirit : Ability.check (DRS.ofList []) hollowhengeSpirit = [] := by decide
def netcasterSpider : Ability :=
  whenever (blocks thisCreature (some (a (Primitives.Predicate.and [creature, Primitives.Predicate.hasKeyword (.the (.named "flying"))]))))
    (get thisCreature (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 0)) (some untilEndOfTurn))
theorem okNetcasterSpider : Ability.check (DRS.ofList []) netcasterSpider = [] := by decide
def viashinoWeaponsmith : Ability :=
  whenever (becomesBlocked thisCreature (some (a creature)))
    (get thisCreature (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 2)) (some untilEndOfTurn))
theorem okViashinoWeaponsmith : Ability.check (DRS.ofList []) viashinoWeaponsmith = [] := by decide
def somberwaldAlpha : Ability :=
  whenever (becomesBlocked (a creatureYouControl) none)
    (get it (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1)) (some untilEndOfTurn))
theorem okSomberwaldAlpha : Ability.check (DRS.ofList []) somberwaldAlpha = [] := by decide
def vertigoSpawn : Ability :=
  whenever (blocks thisCreature (some (a creature)))
    (Primitives.Instruction.sequentially
      [ Primitives.Instruction.setStatus .tapped (that (.type .creature)),
        Primitives.Instruction.skipUntap (that (.type .creature)) (.lit 1) ])
theorem okVertigoSpawn : Ability.check (DRS.ofList []) vertigoSpawn = [] := by decide
def orneryDilophosaur : Ability :=
  triggeredIf (attacks thisCreature)
    (exists_ (Primitives.Predicate.and [ creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you,
                     Primitives.Predicate.compare [.stat .power] .atLeast (.lit 4) ]))
    (get thisCreature (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 2)) (some untilEndOfTurn))
theorem okOrneryDilophosaur : Ability.check (DRS.ofList []) orneryDilophosaur = [] := by decide
def incisorGlider : Ability :=
  triggeredIf (attacks thisCreature)
    (Primitives.Condition.compareAmt (countersOn (.named "poison") (a Primitives.Predicate.opponent)) .atLeast (.lit 3))
    (get (allOf creatureYouControl) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1)) (some untilEndOfTurn))
theorem okIncisorGlider : Ability.check (DRS.ofList []) incisorGlider = [] := by decide
def stormFleetSpy : Ability :=
  triggeredIf (Primitives.GameEvent.enters thisCreature none) (happened (Primitives.Instruction.attack none) (Primitives.Subject.actor Primitives.NounPhrase.you) soFarThisTurn)
    (Primitives.Instruction.draw (.lit 1))
theorem okStormFleetSpy : Ability.check (DRS.ofList []) stormFleetSpy = [] := by decide
def loanShark : Ability :=
  triggeredIf (Primitives.GameEvent.enters thisCreature none)
    (Primitives.Condition.compareAmt (eventCount (Primitives.Instruction.procedure .cast (Primitives.NounPhrase.any) Primitives.PlayPayment.itsOwnCost none none none) (Primitives.Subject.actor Primitives.NounPhrase.you) soFarThisTurn) .atLeast (.lit 2)) (Primitives.Instruction.draw
      (.lit 1))
theorem okLoanShark : Ability.check (DRS.ofList []) loanShark = [] := by decide

def forceOfDespair : Instruction :=
  destroy (allOf (Primitives.Predicate.and [creature, happenedTo Primitives.Subject.any (Primitives.Instruction.move (relative .object) Primitives.EventSource.anywhere (some battlefield) .arrival none []) soFarThisTurn]))
theorem okForceOfDespair : Instruction.check (DRS.ofList []) forceOfDespair = [] := by decide
def cradleToGrave : Instruction :=
  destroy (target (Primitives.Predicate.and [creature, Primitives.Predicate.not
    (Primitives.Predicate.colorIs .black), happenedTo Primitives.Subject.any (Primitives.Instruction.move (relative
    .object) Primitives.EventSource.anywhere (some battlefield) .arrival none []) soFarThisTurn]))
theorem okCradleToGrave : Instruction.check (DRS.ofList []) cradleToGrave = [] := by decide

def aragornKingOfGondor : Ability :=
  when (Primitives.GameEvent.enters thisCreature none) (Primitives.Instruction.gainDesignation Primitives.NounPhrase.you (.named "monarch") none)
theorem okAragornKingOfGondor : Ability.check (DRS.ofList []) aragornKingOfGondor = [] := by decide
def firmamentSage : Ability := whenever (Primitives.GameEvent.gameBecomes (.named "night")) (Primitives.Instruction.draw (.lit 1))
theorem okFirmamentSage : Ability.check (DRS.ofList []) firmamentSage = [] := by decide
def deeprootWarrior : Ability :=
  whenever (becomesBlocked thisCreature none)
    (get it (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1)) (some untilEndOfTurn))
theorem okDeeprootWarrior : Ability.check (DRS.ofList []) deeprootWarrior = [] := by decide
def borderlandMarauder : Ability :=
  whenever (attacks thisCreature) (get it (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 0)) (some untilEndOfTurn))
theorem okBorderlandMarauder : Ability.check (DRS.ofList []) borderlandMarauder = [] := by decide
def lichsMasteryLoss : Ability :=
  when (leavesBattlefield thisEnchantment) (Primitives.Instruction.conclude .loseGame)
theorem okLichsMasteryLoss : Ability.check (DRS.ofList []) lichsMasteryLoss = [] := by decide
def phageTheUntouchable : Ability :=
  whenever (dealsCombatDamage thisCreature (a Primitives.Predicate.anyPlayer)) (instruct (that .player) (Primitives.Instruction.conclude .loseGame))
theorem okPhageTheUntouchable : Ability.check (DRS.ofList []) phageTheUntouchable = [] := by decide
def elderscaleWurm : Ability :=
  triggeredIf (Primitives.GameEvent.enters thisCreature none) (Primitives.Condition.compareAmt (lifeTotalOf Primitives.NounPhrase.you) .less (.lit 7))
    (setLife (.lit 7))
theorem okElderscaleWurm : Ability.check (DRS.ofList []) elderscaleWurm = [] := by decide
/-- Krang, Master Mind -/
def krang : Ability :=
  triggeredIf (Primitives.GameEvent.enters thisCreature none)
    (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.inZone (handOf Primitives.NounPhrase.you)])) .less (.lit 4)) (Primitives.Instruction.draw Primitives.Amount.theDifference)
theorem okKrang : Ability.check (DRS.ofList []) krang = [] := by decide

def carrionGrub : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Carrion Grub", cost := some [generic 3, pip .black], types := [.creature],
      subtypes := [creatureType "Insect"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ getsPt thisCreature (Primitives.Delta.up (Primitives.Amount.letter .x)) (Primitives.Delta.up (.lit 0)),
              Primitives.StaticSpec.letterDefinition .x
                (aggregate .max (.stat .power) (Primitives.Predicate.and [creature, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)])) ]),
          when (Primitives.GameEvent.enters thisCreature none) (mill (.lit 4) Primitives.NounPhrase.you) ],
      power := stat 0, toughness := stat 5 } }

/-- Once Upon a Time -/
def onceUponATimeFirstCast : Predicate := nthCastBy (.nth 1) Primitives.NounPhrase.you .game
theorem okOnceUponATimeFirstCast : Predicate.checkAt .object (DRS.ofList []) onceUponATimeFirstCast = [] := by
  decide

def youngPyromancer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Young Pyromancer", cost := some [generic 1, pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Shaman"],
      text :=
        [ whenever (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Instruction.procedure .cast ((a
          (Primitives.Predicate.and [instantOrSorcery, spell]))) Primitives.PlayPayment.itsOwnCost none none none))
            (Primitives.Instruction.create (.lit 1)
              (Primitives.TokenSpec.written (creatureToken 1 1 [.red] [creatureType "Elemental"])) [])
                  ],
      power := stat 2, toughness := stat 1 } }

def archivistOfGondor : Ability :=
  triggeredIf (dealsCombatDamage yourCommander (a Primitives.Predicate.anyPlayer)) (thereIsNo (.named "monarch"))
    (Primitives.Instruction.gainDesignation Primitives.NounPhrase.you (.named "monarch") none)
theorem okArchivistOfGondor : Ability.check (DRS.ofList []) archivistOfGondor = [] := by decide

def hissingMiasma : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hissing Miasma", cost := some [generic 1, pip .black, pip .black],
      types := [.enchantment],
      text :=
        [ whenever (attacksPlayer (a creature) Primitives.NounPhrase.you) (instruct (controllerOf it) (loseLife (.lit 1))) ] } }

def orimsPrayer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Orim's Prayer", cost := some [generic 1, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ whenever (attacksPlayer (counted (atLeast 1) creature) Primitives.NounPhrase.you)
            (gainLife (forEach 1 (Primitives.Predicate.and [attacking, creature]))) ] } }

def lesserGargadon : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lesser Gargadon", cost := some [generic 2, pip .red, pip .red],
      types := [.creature], subtypes := [creatureType "Beast"],
      text :=
        [ triggeredOr (attacks thisCreature) [blocks thisCreature none]
            (sacrifice (a land)) ],
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
  [blocks thisCreature none, Primitives.GameEvent.does (Primitives.Subject.actor (a spell)) (Primitives.Instruction.target thisCreature)]
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
              (Primitives.GameEvent.triggers (a (Primitives.Predicate.and [ Primitives.Predicate.abilityHead .anyTriggered,
                           Primitives.Predicate.abilityOf (a (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) ])) none none))
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
theorem okStalwartSuccessorHeader : GameEvent.check (DRS.ofList []) stalwartSuccessorHeader = [] := by decide

/-- Hollowmurk Siege -/
def hollowmurkSiegeSultai : Ability :=
  triggeredOnlyOnce (bareCounterEvent .put .one (a creatureYouControl)) oncePerTurn
    (Primitives.Instruction.draw (.lit 1))
theorem okHollowmurkSiegeSultai : Ability.check (DRS.ofList []) hollowmurkSiegeSultai = [] := by decide

/-- Clone -/
def clone : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Clone", cost := some [generic 3, pip .blue], types := [.creature],
      subtypes := [creatureType "Shapeshifter"],
      text := [Primitives.Ability.static (Primitives.StaticSpec.entryRider thisCreature (Primitives.EntryRider.asCopyOf true (a creature) []))],
      power := stat 0, toughness := stat 0 } }

/-- Quicksilver Gargantuan -/
def quicksilverGargantuan : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Quicksilver Gargantuan", cost := some [generic 5, pip .blue, pip .blue],
      types := [.creature], subtypes := [creatureType "Shapeshifter"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.entryRider thisCreature
            (Primitives.EntryRider.asCopyOf true (a creature) [Primitives.CopyExcept.pt (.lit 7) (.lit 7)])) ],
      power := stat 7, toughness := stat 7 } }

/-- Sculpting Steel -/
def sculptingSteel : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sculpting Steel", cost := some [generic 3], types := [.artifact],
      text := [Primitives.Ability.static (Primitives.StaticSpec.entryRider thisArtifact (Primitives.EntryRider.asCopyOf true (a artifact) []))] } }

/-- Sakashima's Student -/
def sakashimasStudentCopy : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.entryRider thisCreature
    (Primitives.EntryRider.asCopyOf true (a creature) [Primitives.CopyExcept.types [] [creatureType "Ninja"]]))
theorem okSakashimasStudentCopy : Ability.check (DRS.ofList []) sakashimasStudentCopy = [] := by decide
/-- Chameleon, Master of Disguise -/
def chameleonCopy : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.entryRider thisCreature
    (Primitives.EntryRider.asCopyOf true (a (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
      [Primitives.CopyExcept.name "Chameleon, Master of Disguise"]))
theorem okChameleonCopy : Ability.check (DRS.ofList []) chameleonCopy = [] := by decide

def nefariousLichGain : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.instead (Primitives.WouldClause.would (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Instruction.changeLife (Primitives.Delta.up Primitives.Amount.anyAmount))) (Primitives.Instruction.draw Primitives.Amount.thatMuch) none .repeatedly none)
theorem okNefariousLichGain : Ability.check (DRS.ofList []) nefariousLichGain = [] := by decide

def piousWarrior : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Pious Warrior", cost := some [generic 3, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Rebel", creatureType "Warrior"],
      text := [whenever (Primitives.GameEvent.isDealtDamage .combatOnly thisCreature) (gainLife Primitives.Amount.thatMuch)],
      power := stat 2, toughness := stat 3 } }

def sanguineBond : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sanguine Bond", cost := some [generic 3, pip .black, pip .black],
      types := [.enchantment],
      text := [whenever (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Instruction.changeLife (Primitives.Delta.up Primitives.Amount.anyAmount))) (instruct (target Primitives.Predicate.opponent) (loseLife Primitives.Amount.thatMuch))]
          } }

def exquisiteBlood : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Exquisite Blood", cost := some [generic 4, pip .black], types := [.enchantment],
      text := [whenever (Primitives.GameEvent.does (Primitives.Subject.actor anOpponent) (Primitives.Instruction.changeLife (Primitives.Delta.down Primitives.Amount.anyAmount))) (gainLife Primitives.Amount.thatMuch)] } }

/-- Agate-Blade Assassin -/
def agateBladeAssassin : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Agate-Blade Assassin", cost := some [generic 1, pip .black], types := [.creature],
      subtypes := [creatureType "Lizard", creatureType "Assassin"],
      text :=
        [ whenever (attacks thisCreature)
            (Primitives.Instruction.sequentially
              [instruct (Primitives.NounPhrase.combatPlayer .defending) (loseLife (.lit 1)), gainLife (.lit 1)]) ],
      power := stat 1, toughness := stat 3 } }

/-- Bard, Heir of Girion -/
def bardHeirOfGirion : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bard, Heir of Girion", cost := some [generic 2, pip .white, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Archer"],
      text :=
        [ keyword (.named "reach"), keyword (.named "vigilance"),
          Primitives.Ability.static (getsPt (allOf (otherCreatureYouControl thisCreature)) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1))),
          whenever (attacks Primitives.NounPhrase.you) (Primitives.Instruction.draw (.lit 1)) ],
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
    (instruct (Primitives.NounPhrase.combatPlayer .attacking) (loseLife Primitives.Amount.thatMuch))
theorem okSoulsOfTheFaultlessDrain : Ability.check (DRS.ofList []) soulsOfTheFaultlessDrain = [] := by decide

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
          whenever (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Instruction.procedure .cast ((a
            (Primitives.Predicate.and [spell, ofChosen .color]))) Primitives.PlayPayment.itsOwnCost none none none)) (gainLife (.lit 1)) ] } }

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
        [ whenever (Primitives.GameEvent.does (Primitives.Subject.actor (a Primitives.Predicate.anyPlayer)) (Primitives.Instruction.procedure .cast ((a spell)) Primitives.PlayPayment.itsOwnCost none none none))
            (Primitives.Instruction.sequentially
              [ instruct they (gainLife (Primitives.Amount.letter .x)),
                Primitives.Instruction.define .x (countOf (Primitives.Predicate.and [Primitives.Predicate.inZone graveyard, (Primitives.Predicate.shares .cardName .share (that .spell))])) ]) ] } }

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
            (happened (Primitives.Instruction.attack none) (Primitives.Subject.actor Primitives.NounPhrase.you) soFarThisTurn)),
          Primitives.Ability.spell none (Primitives.Instruction.counterSpell (target spell)) ] } }

def fyndhornDruid : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fyndhorn Druid", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [creatureType "Elf", creatureType "Druid"],
      text :=
        [ triggeredIf (Primitives.GameEvent.dies thisCreature) (happened (Primitives.Instruction.block (Primitives.NounPhrase.asMarker .permanent
          (relative .object))) (Primitives.Subject.actor it) soFarThisTurn)
            (gainLife (.lit 4)) ],
      power := stat 2, toughness := stat 2 } }

def kamiOfTerribleSecrets : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Kami of Terrible Secrets", cost := some [generic 3, pip .black], types := [.creature],
      subtypes := [creatureType "Spirit"],
      text :=
        [ triggeredIf (Primitives.GameEvent.enters thisCreature none)
            (Primitives.Condition.and [ exists_ (Primitives.Predicate.and [artifact, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]),
                    exists_ (Primitives.Predicate.and [enchantment, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]) ])
            (Primitives.Instruction.sequentially [Primitives.Instruction.draw (.lit 1), gainLife (.lit 1)]) ],
      power := stat 3, toughness := stat 4 } }

def dreadSlaver : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dread Slaver", cost := some [generic 3, pip .black, pip .black],
      types := [.creature], subtypes := [creatureType "Zombie", creatureType "Horror"],
      text :=
        [ whenever
            (Primitives.GameEvent.dies (a (Primitives.Predicate.and [creature, happenedTo (Primitives.Subject.actor thisCreature) (Primitives.Instruction.dealDamage Primitives.Amount.anyAmount               (Primitives.NounPhrase.asMarker .permanent (relative .object)) .any) soFarThisTurn])))
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
theorem okSteppeLynx : Ability.check (DRS.ofList []) steppeLynx = [] := by decide
/-- Owlbear -/
def owlbear : Ability :=
  flavorWord "Keen Senses" (when (Primitives.GameEvent.enters thisCreature none) (Primitives.Instruction.draw (.lit 1)))
theorem okOwlbear : Ability.check (DRS.ofList []) owlbear = [] := by decide
/-- Netherese Puzzle-Ward -/
def netheresePuzzleWardIllumination : Ability :=
  whenever youRollHighestNatural (Primitives.Instruction.draw (.lit 1))
theorem okNetheresePuzzleWardIllumination :
    Ability.check (DRS.ofList []) netheresePuzzleWardIllumination = [] := by decide
/-- Farideh, Devil's Chosen -/
def faridehResultRead : Ability :=
  whenever (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Instruction.rollDice Primitives.Amount.anyAmount .anyDie .many Primitives.RollWatch.anyResult))
    (Primitives.Instruction.sequentially
      [ gain thisCreature (keyword (.named "flying")) (some untilEndOfTurn),
        gain thisCreature (keyword (.named "menace")) (some untilEndOfTurn),
        doIf (Primitives.Condition.anyResultIs .atLeast (.lit 10)) (Primitives.Instruction.draw (.lit 1)) ])
theorem okFaridehResultRead : Ability.check (DRS.ofList []) faridehResultRead = [] := by decide
/-- Jaws of Defeat -/
def jawsOfDefeat : Ability :=
  whenever (Primitives.GameEvent.enters (a creatureYouControl) none)
    (instruct (target Primitives.Predicate.opponent) (loseLife
      (Primitives.Amount.arith .differenceBetween (Primitives.Amount.statOf (.stat .power) (that (.type .creature)))
        (Primitives.Amount.statOf (.stat .toughness) (that (.type .creature))))))
theorem okJawsOfDefeat : Ability.check (DRS.ofList []) jawsOfDefeat = [] := by decide
/-- Defiling Daemogoth -/
def defilingDaemogothDrain : Instruction :=
  Primitives.Instruction.sequentially
    [instruct (each Primitives.Predicate.opponent) (loseLife (Primitives.Amount.letter .x)),
      Primitives.Instruction.define .x (eventSum (Primitives.Instruction.changeLife (Primitives.Delta.up Primitives.Amount.anyAmount)) (Primitives.Subject.actor Primitives.NounPhrase.you) soFarThisTurn)]
theorem okDefilingDaemogothDrain : Instruction.check (DRS.ofList []) defilingDaemogothDrain = [] := by decide
def skullsporeNexusTrigger : Ability :=
  whenever (Primitives.GameEvent.dies (counted (atLeast 1) (Primitives.Predicate.and [nontoken, creatureYouControl])))
    (create (.lit 1)
      (creatureTokenOf (Primitives.Amount.aggregate .sum (.stat .power) (those .card))
        (Primitives.Amount.aggregate .sum (.stat .power) (those .card))
        [.green] [creatureType "Fungus", creatureType "Dinosaur"]))
theorem okSkullsporeNexusTrigger : Ability.check (DRS.ofList []) skullsporeNexusTrigger = [] := by decide
/-- Wavebreak Hippocamp -/
def wavebreakHippocamp : Ability :=
  triggeredOnlyDuring (Primitives.GameEvent.nthOccurrence (.nth 1) none (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Instruction.procedure .cast ((a spell)) Primitives.PlayPayment.itsOwnCost none none none)))
    (duringPart .turn (some (each Primitives.Predicate.opponent))) (Primitives.Instruction.draw (.lit 1))
theorem okWavebreakHippocamp : Ability.check (DRS.ofList []) wavebreakHippocamp = [] := by decide
/-- Midnight Clock -/
def midnightClockHeader : GameEvent :=
  Primitives.GameEvent.nthOccurrence (.nth 12) none (counterEvent .put (.named "hourCounter") .one thisArtifact)
theorem okMidnightClockHeader : GameEvent.check (DRS.ofList []) midnightClockHeader = [] := by decide
/-- Political Triumph -/
def politicalTriumphHeader : GameEvent :=
  Primitives.GameEvent.nthOccurrence (.nth 4) none (counterEvent .put (.named "planCounter") .one thisEnchantment)
theorem okPoliticalTriumphHeader : GameEvent.check (DRS.ofList []) politicalTriumphHeader = [] := by decide
/-- Thought Lash -/
def thoughtLashTrigger : Ability :=
  when (Primitives.GameEvent.does (Primitives.Subject.actor (a Primitives.Predicate.anyPlayer)) (Primitives.Instruction.pay Primitives.Cost.anyCost .once .unpaid (some thisEnchantment) (some (.named "cumulativeUpkeep"))))
    (instruct (that .player) (exile (each (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.inZone (Primitives.ZoneExpr.zone .library (Primitives.ZoneScope.possessedBy they))]))))
theorem okThoughtLashTrigger : Ability.check (DRS.ofList []) thoughtLashTrigger = [] := by decide
/-- Heart of Bogardan -/
def heartOfBogardanHeader : GameEvent :=
  Primitives.GameEvent.does (Primitives.Subject.actor (a Primitives.Predicate.anyPlayer)) (Primitives.Instruction.pay Primitives.Cost.anyCost .once .unpaid (some thisEnchantment) (some (.named "cumulativeUpkeep")))
theorem okHeartOfBogardanHeader : GameEvent.check (DRS.ofList []) heartOfBogardanHeader = [] := by decide
/-- Balduvian Fallen -/
def balduvianFallenHeader : GameEvent := Primitives.GameEvent.does Primitives.Subject.any (Primitives.Instruction.pay Primitives.Cost.anyCost .once .paid (some thisCreature) (some (.named "cumulativeUpkeep")))
theorem okBalduvianFallenHeader : GameEvent.check (DRS.ofList []) balduvianFallenHeader = [] := by decide
/-- Stormscape Battlemage -/
def stormscapeBattlemageFirstKicker : Ability :=
  triggeredIf (Primitives.GameEvent.enters thisCreature none)
    (costWasPaid (.byNthKeyword (.nth 1) (.named "kicker")) none thisCreature) (gainLife (.lit 3))
theorem okStormscapeBattlemageFirstKicker :
    Ability.check (DRS.ofList []) stormscapeBattlemageFirstKicker = [] := by decide
/-- All-Seeing Arbiter -/
def allSeeingArbiterHeader : GameEvent :=
  Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (discard ((a
    (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.inZone hand]))))
theorem okAllSeeingArbiterHeader : GameEvent.check (DRS.ofList []) allSeeingArbiterHeader = [] := by decide

/-- Liliana's Caress -/
def lilianasCaress : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Liliana's Caress", cost := some [generic 1, pip .black], types := [.enchantment],
      text :=
        [ whenever (Primitives.GameEvent.does (Primitives.Subject.actor anOpponent) (discard ((a
          (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.inZone hand])))))
            (instruct they (loseLife (.lit 2))) ] } }

/-- Scheming Aspirant -/
def schemingAspirant : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Scheming Aspirant", cost := some [generic 1, pip .black], types := [.creature],
      subtypes := [creatureType "Phyrexian", creatureType "Advisor"],
      text :=
        [ whenever (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Declared.proliferate))
            (Primitives.Instruction.sequentially [instruct (each Primitives.Predicate.opponent) (loseLife (.lit 2)), gainLife (.lit 2)]) ],
      power := stat 1, toughness := stat 3 } }

/-- Reciprocate -/
def reciprocate : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Reciprocate", cost := some [pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (exile
            (target (Primitives.Predicate.and [creature, happenedTo (Primitives.Subject.actor (relative .object)) (Primitives.Instruction.dealDamage Primitives.Amount.anyAmount               Primitives.NounPhrase.you .any) soFarThisTurn]))) ] } }

/-- Tahngarth, First Mate -/
def tahngarthHeader : GameEvent := Primitives.GameEvent.does (Primitives.Subject.actor anOpponent) (Primitives.Instruction.attackWith (counted (atLeast 1) creature) none)
theorem okTahngarthHeader : GameEvent.check (DRS.ofList []) tahngarthHeader = [] := by decide
/-- Myth Unbound -/
def mythUnboundTrigger : Ability :=
  whenever (putIntoFrom yourCommander commandZone Primitives.EventSource.anywhere) (Primitives.Instruction.draw (.lit 1))
theorem okMythUnboundTrigger : Ability.check (DRS.ofList []) mythUnboundTrigger = [] := by decide

/-- Commander's Insignia -/
def commandersInsignia : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Commander's Insignia", cost := some [generic 2, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ Primitives.Ability.static (getsPt (allOf creatureYouControl)
            (Primitives.Delta.up (eventCount (Primitives.Instruction.procedure .cast (((some
              (Primitives.NounPhrase.asMarker .spell yourCommander))).getD .any) Primitives.PlayPayment.itsOwnCost none (some
              (Primitives.EventSource.zones [commandZone])) none) (Primitives.Subject.actor Primitives.NounPhrase.you) thisGame))
            (Primitives.Delta.up (eventCount (Primitives.Instruction.procedure .cast (((some
              (Primitives.NounPhrase.asMarker .spell yourCommander))).getD .any) Primitives.PlayPayment.itsOwnCost none (some
              (Primitives.EventSource.zones [commandZone])) none) (Primitives.Subject.actor Primitives.NounPhrase.you) thisGame)))
              ] } }

/-- Faith's Reward -/
def faithsReward : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Faith's Reward", cost := some [generic 3, pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (move
            (allOf (Primitives.Predicate.and [ permanentCard, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you),
                           Primitives.Predicate.did (⟨Primitives.Subject.any, (Primitives.Instruction.move (relative .object) (Primitives.EventSource.zones
                             [battlefield]) (some (graveyardOf
                             Primitives.NounPhrase.you)) .departure none []), (Primitives.LookbackSpan.duration soFarThisTurn none)⟩) ]))
            battlefield) ] } }

/-- Oscorp Industries -/
def oscorpIndustriesReturn : Ability :=
  when (Primitives.GameEvent.enters thisLand (some (Primitives.EventSource.zones [graveyard]))) (loseLife (.lit 2))
theorem okOscorpIndustriesReturn : Ability.check (DRS.ofList []) oscorpIndustriesReturn = [] := by decide
def theLostAndTheDamnedEntryArm : GameEvent :=
  Primitives.GameEvent.enters (a (Primitives.Predicate.and [land, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) (some (Primitives.EventSource.anywhereBut [handOf Primitives.NounPhrase.you]))
theorem okTheLostAndTheDamnedEntryArm :
    GameEvent.check (DRS.ofList []) theLostAndTheDamnedEntryArm = [] := by decide
/-- Forsaken Wastes -/
def forsakenWastesTargeted : Ability :=
  whenever (Primitives.GameEvent.does (Primitives.Subject.actor (a spell)) (Primitives.Instruction.target thisEnchantment))
    (instruct (controllerOf (that .spell)) (loseLife (.lit 5)))
theorem okForsakenWastesTargeted : Ability.check (DRS.ofList []) forsakenWastesTargeted = [] := by decide

/-- Fblthp, the Lost -/
def fblthp : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fblthp, the Lost", cost := some [generic 1, pip .blue],
      supertypes := [.legendary], types := [.creature], subtypes := [creatureType "Homunculus"],
      text :=
        [ when (Primitives.GameEvent.enters Primitives.NounPhrase.this none)
            (Primitives.Instruction.replace (Primitives.Instruction.draw (.lit 1))
              (Primitives.Instruction.doIf (Primitives.Condition.or
                  [ Primitives.Condition.did (⟨Primitives.Subject.any, (Primitives.Instruction.move it                     (Primitives.EventSource.zones [yourLibrary]) (some battlefield) .arrival none []), Primitives.LookbackSpan.triggering⟩),
                    Primitives.Condition.matches it (Primitives.Predicate.castFrom yourLibrary) ])
                (Primitives.Instruction.draw (.lit 2)) none)),
          when (Primitives.GameEvent.does (Primitives.Subject.actor (a spell)) (Primitives.Instruction.target Primitives.NounPhrase.this)) (shuffleInto Primitives.NounPhrase.this) ],
      power := stat 1, toughness := stat 1 } }

/-- Frost Walker -/
def frostWalker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Frost Walker", cost := some [generic 1, pip .blue], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text :=
        [ when (Primitives.GameEvent.does (Primitives.Subject.actor (a (Primitives.Predicate.or [spell, Primitives.Predicate.abilityHead .anyOnStack]))) (Primitives.Instruction.target thisCreature))
            (sacrifice itPermanent) ],
      power := stat 4, toughness := stat 1 } }

/-- Loaming Shaman -/
def loamingShaman : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Loaming Shaman", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [creatureType "Centaur", creatureType "Shaman"],
      text :=
        [ when (Primitives.GameEvent.enters thisCreature none)
            (instruct (target Primitives.Predicate.anyPlayer) (shuffleInto
              (Primitives.NounPhrase.described (Primitives.DetPhrase.target anyNumber) (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.inZone (graveyardOf they)])))) ],
      power := stat 3, toughness := stat 2 } }

/-- Debris Beetle -/
def debrisBeetleTrigger : Ability :=
  when (Primitives.GameEvent.enters thisVehicle none)
    (Primitives.Instruction.sequentially [instruct (each Primitives.Predicate.opponent) (loseLife (.lit 3)), gainLife (.lit 3)])
theorem okDebrisBeetleTrigger : Ability.check (DRS.ofList []) debrisBeetleTrigger = [] := by decide
/-- Roads Go Ever, Ever On's chapters II and III -/
def roadsGoEverEverOnChapters : Ability :=
  when (Primitives.GameEvent.chapterMark [2, 3]) (move (a (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.exiledWith thisSaga])) hand)
theorem okRoadsGoEverEverOnChapters : Ability.check (DRS.ofList []) roadsGoEverEverOnChapters = [] := by decide
/-- Wurmwall Sweeper -/
def wurmwallSweeperTrigger : Ability := when (Primitives.GameEvent.enters thisSpacecraft none) (surveil (.lit 2))
theorem okWurmwallSweeperTrigger : Ability.check (DRS.ofList []) wurmwallSweeperTrigger = [] := by decide
/-- Case of the Crimson Pulse -/
def caseOfTheCrimsonPulseTrigger : Ability :=
  when (Primitives.GameEvent.enters thisCase none)
    (Primitives.Instruction.sequentially [discard (a (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.inZone yourHand])), Primitives.Instruction.draw (.lit 2)])
theorem okCaseOfTheCrimsonPulseTrigger : Ability.check (DRS.ofList []) caseOfTheCrimsonPulseTrigger = [] := by
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
            putOntoBattlefield (theVerbed (.action "search") .one) ]),
      Primitives.Instruction.doIf (happenedThisWay (Primitives.Instruction.search (Primitives.SearchScope.oneZone Primitives.ZoneExpr.anyZone) anyNumber (Primitives.Predicate.and [])) (Primitives.Subject.actor Primitives.NounPhrase.you)) shuffle none ]
theorem okAgencyOutfitterSearch : Instruction.check (DRS.ofList []) agencyOutfitterSearch = [] := by decide

/-- Trouble in Pairs' -/
def troubleInPairsArms : Ability :=
  Primitives.Ability.triggered (Primitives.GameEvent.does (Primitives.Subject.actor anOpponent) (Primitives.Instruction.attackWith (counted (atLeast 2) creature) (some Primitives.NounPhrase.you)))
    [ Primitives.GameEvent.nthOccurrence (.nth 2) (some .turn) (Primitives.GameEvent.does (Primitives.Subject.actor (a Primitives.Predicate.opponent)) (Primitives.Instruction.draw Primitives.Amount.anyAmount)),
      Primitives.GameEvent.nthOccurrence (.nth 2) (some .turn) (Primitives.GameEvent.does (Primitives.Subject.actor (a
        Primitives.Predicate.opponent)) (Primitives.Instruction.procedure .cast ((a spell)) Primitives.PlayPayment.itsOwnCost none none none)) ]
    none [] none none none (Primitives.Instruction.draw (.lit 1))
theorem okTroubleInPairsArms : Ability.check (DRS.ofList []) troubleInPairsArms = [] := by decide

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
                      (theVerbed (.action "exile") .one (some (.ofType .card .creature)))))
                    (Primitives.Delta.up (Primitives.Amount.statOf (.stat .toughness) (theVerbed (.action "exile") .one)))
                    (some thisTurn))
                  none ]) ],
      power := stat 4, toughness := stat 4 } }

/-- Hostile Investigator -/
def hostileInvestigatorHeader : GameEvent :=
  Primitives.GameEvent.does (Primitives.Subject.actor (counted (atLeast 1) Primitives.Predicate.anyPlayer)) (discard ((counted (atLeast 1) Primitives.Predicate.isCard)))
theorem okHostileInvestigatorHeader : GameEvent.check (DRS.ofList []) hostileInvestigatorHeader = [] := by decide

/-- Hallowed Moonlight -/
def hallowedMoonlight : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hallowed Moonlight", cost := some [generic 1, pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ replaceEvent (Primitives.WouldClause.would Primitives.Subject.any (Primitives.Instruction.move (a (Primitives.Predicate.and [creature, Primitives.Predicate.not (Primitives.Predicate.castBy Primitives.NounPhrase.any none)])) Primitives.EventSource.anywhere (some battlefield) .arrival none [])) (exile it)
                (some untilEndOfTurn),
              Primitives.Instruction.draw (.lit 1) ]) ] } }

/-- Gather Specimens -/
def gatherSpecimens : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gather Specimens", cost := some [generic 3, pip .blue, pip .blue, pip .blue],
      types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.establish
            (Primitives.StaticSpec.entryRider (a (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller anOpponent])) (Primitives.EntryRider.under Primitives.NounPhrase.you))
            (some thisTurn)) ] } }

/-- Don't Blink's replacement, without its written agent -/
def dontBlinkReplacement : Instruction :=
  Primitives.Instruction.establish
    (Primitives.StaticSpec.instead (Primitives.WouldClause.wouldAny [(Primitives.WouldClause.would Primitives.Subject.any (Primitives.Instruction.move (counted (atLeast 1) creature) (Primitives.EventSource.zones [exileZone]) (some battlefield) .arrival none [])), (Primitives.WouldClause.would Primitives.Subject.any (Primitives.Instruction.move (counted (atLeast 1) (Primitives.Predicate.and [creature, Primitives.Predicate.castFrom exileZone])) Primitives.EventSource.anywhere (some battlefield) .arrival none []))]) (shuffleInto them) none .repeatedly none)
    (some untilEndOfTurn)
theorem okDontBlinkReplacement : Instruction.check (DRS.ofList []) dontBlinkReplacement = [] := by decide

/-- Seasoned Warrenguard -/
def seasonedWarrenguard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Seasoned Warrenguard", cost := some [pip .white], types := [.creature],
      subtypes := [creatureType "Rabbit", creatureType "Warrior"],
      text :=
        [ triggeredWhile (attacks thisCreature)
            (whileTrue (exists_ (Primitives.Predicate.and [Primitives.Predicate.isToken, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))
            (get thisCreature (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 0)) (some untilEndOfTurn)) ],
      power := stat 1, toughness := stat 2 } }

/-- Brazen Blademaster -/
def brazenBlademaster : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Brazen Blademaster", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [creatureType "Orc", creatureType "Pirate"],
      text :=
        [ triggeredWhile (attacks thisCreature)
            (whileTrue
              (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.and [artifact, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .atLeast
                (.lit 2)))
            (get it (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 1)) (some untilEndOfTurn)) ],
      power := stat 2, toughness := stat 3 } }

/-- Autarch Mammoth -/
def autarchMammothLine : Ability :=
  triggeredJoined (Primitives.GameEvent.enters thisCreature none)
    [ joinedHeadWhile (attacks thisCreature)
        (whileTrue (Primitives.Condition.matches thisCreature (Primitives.Predicate.hasDesignation (.named "saddled") none))) ]
    (create (.lit 1) (creatureToken 3 3 [.green] [creatureType "Elephant"]))
theorem okAutarchMammothLine : Ability.check (DRS.ofList []) autarchMammothLine = [] := by decide
/-- Altar of the Brood -/
def altarOfTheBrood : Ability :=
  whenever (Primitives.GameEvent.enters (a (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, Primitives.Predicate.otherThan Primitives.NounPhrase.this])) none)
    (instruct (each Primitives.Predicate.opponent) (mill (.lit 1) they))
theorem okAltarOfTheBrood : Ability.check (DRS.ofList []) altarOfTheBrood = [] := by decide
/-- Foul Emissary -/
def foulEmissaryLine : Ability :=
  triggeredWhile (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (sacrifice (thisCreature)))
    (whileDoing (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Instruction.procedure .cast ((a
      (Primitives.Predicate.and [spell, Primitives.Predicate.hasKeyword (.the (.named "emerge"))]))) Primitives.PlayPayment.itsOwnCost none none none)))
    (create (.lit 1) (creatureToken 3 2 [] [creatureType "Eldrazi", creatureType "Horror"]))
theorem okFoulEmissaryLine : Ability.check (DRS.ofList []) foulEmissaryLine = [] := by decide

/-- Bramble Elemental -/
def brambleElemental : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bramble Elemental", cost := some [generic 3, pip .green, pip .green],
      types := [.creature], subtypes := [creatureType "Elemental"],
      text :=
        [ whenever (Primitives.GameEvent.does Primitives.Subject.any (Primitives.Instruction.attachment .attached (a (Primitives.Predicate.hasSubtype (enchantmentType "Aura"))) (some thisCreature)))
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
            (Primitives.Instruction.draw (.lit 1)) ],
      power := stat 2, toughness := stat 2 } }

/-- Cloud, Ex-SOLDIER -/
def cloudExSoldierAttach : Ability :=
  when (Primitives.GameEvent.enters thisCreature none)
    (attachToIt
      (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 1))
        (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (artifactType "Equipment"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))
theorem okCloudExSoldierAttach : Ability.check (DRS.ofList []) cloudExSoldierAttach = [] := by decide
/-- Akiri, Fearless Voyager -/
def akiriEquippedAttackers : Ability :=
  whenever
    (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Instruction.attackWith (counted (atLeast 1) (Primitives.Predicate.and [creatureYouControl, Primitives.Predicate.isAttached (some .equipped)])) (some (a Primitives.Predicate.anyPlayer))))
    (Primitives.Instruction.draw (.lit 1))
theorem okAkiriEquippedAttackers : Ability.check (DRS.ofList []) akiriEquippedAttackers = [] := by decide
/-- Vexing Bauble -/
def vexingBaubleTrigger : Ability :=
  triggeredIf (Primitives.GameEvent.does (Primitives.Subject.actor (a Primitives.Predicate.anyPlayer)) (Primitives.Instruction.procedure .cast ((a spell)) Primitives.PlayPayment.itsOwnCost none none none))
    (noManaSpentToCast it)
    (Primitives.Instruction.counterSpell (that .spell))
theorem okVexingBaubleTrigger : Ability.check (DRS.ofList []) vexingBaubleTrigger = [] := by decide

/-- Void Mirror -/
def voidMirror : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Void Mirror", cost := some [generic 2], types := [.artifact],
      text :=
        [ triggeredIf (Primitives.GameEvent.does (Primitives.Subject.actor (a Primitives.Predicate.anyPlayer)) (Primitives.Instruction.procedure .cast ((a
          spell)) Primitives.PlayPayment.itsOwnCost none none none)) (noColoredManaSpentToCast it)
            (Primitives.Instruction.counterSpell (that .spell)) ] } }

/-- Blood Sun -/
def bloodSun : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blood Sun", cost := some [generic 2, pip .red], types := [.enchantment],
      text :=
        [ when (Primitives.GameEvent.enters thisEnchantment none) (Primitives.Instruction.draw (.lit 1)),
          Primitives.Ability.static (Primitives.StaticSpec.allAbilityLoss (allOf land) (some Primitives.Predicate.isManaAbility)) ] } }

/-- Promise of Bunrei -/
def promiseOfBunrei : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Promise of Bunrei", cost := some [generic 2, pip .white], types := [.enchantment],
      text :=
        [ when (Primitives.GameEvent.dies (a creatureYouControl))
            (Primitives.Instruction.doIfDone (sacrifice thisEnchantment)
              (some (create (.lit 4) (creatureToken 1 1 [] [creatureType "Spirit"]))) none) ] } }

/-- Grave Peril -/
def gravePeril : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Grave Peril", cost := some [generic 1, pip .black], types := [.enchantment],
      text :=
        [ when (Primitives.GameEvent.enters (a (Primitives.Predicate.and [creature, Primitives.Predicate.not (Primitives.Predicate.colorIs .black)])) none)
            (Primitives.Instruction.doIfDone (sacrifice thisEnchantment) (some (destroy (that (.type
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
            (instruct (controllerOf (that (.type .creature))) (loseLife (.lit 1))) ] } }

/-- Jeskai Ascendancy's first trigger -/
def jeskaiAscendancyPump : Ability :=
  whenever (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Instruction.procedure .cast ((a (Primitives.Predicate.and
    [spell, Primitives.Predicate.not creature]))) Primitives.PlayPayment.itsOwnCost none none none))
    (Primitives.Instruction.sequentially
      [ get (bare creatureYouControl) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1)) (some untilEndOfTurn),
        untap (those (.type .creature)) ])
theorem okJeskaiAscendancyPump : Ability.check (DRS.ofList []) jeskaiAscendancyPump = [] := by decide
def herosDemise : Instruction := destroy (target (Primitives.Predicate.and [creature, Primitives.Predicate.hasSupertype .legendary]))
theorem okHerosDemise : Instruction.check (DRS.ofList []) herosDemise = [] := by decide

def repayInKind : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Repay in Kind", cost := some [generic 5, pip .black, pip .black], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none
            (instruct (each Primitives.Predicate.anyPlayer) (setLife (aggregate .min (.playerStat .lifeTotal) Primitives.Predicate.anyPlayer))) ] } }

/-- Squelch -/
def squelch : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Squelch", cost := some [generic 1, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [Primitives.Instruction.counterSpell (target (Primitives.Predicate.abilityHead .anyActivated)), Primitives.Instruction.draw (.lit 1)]) ]
                } }

theorem bioplasmTwoCandidates : countOnes .object bioplasmAfterExile = 2 := by decide

/-- "From anywhere" observes Dread after its arrival in the graveyard. -/
def dread : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dread", cost := some [generic 3, pip .black, pip .black, pip .black],
      types := [.creature], subtypes := [creatureType "Elemental", creatureType "Incarnation"],
      text :=
        [ keyword (.named "fear"),
          whenever (Primitives.GameEvent.dealsDamage .any (a creature)
            (some Primitives.NounPhrase.you)) (destroy it),
          when (putIntoFrom Primitives.NounPhrase.this graveyard Primitives.EventSource.anywhere)
            (instruct (Primitives.NounPhrase.possessorOf .owner it) (shuffleInto it)) ],
      power := stat 6, toughness := stat 6 } }

private semantic_template creatureDamagedByAttachedHostThisTurn (attachment : AttachWord) : NounPhrase :=
  a (.and [creature,
    .did ⟨.actor (.attachHost attachment (.type .creature)),
      .dealDamage .anyAmount .which .any, .duration soFarThisTurn none⟩])

/-- Vampiric Embrace. -/
def vampiricEmbrace : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vampiric Embrace", cost := some [generic 2, pip .black, pip .black],
      types := [.enchantment], subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") creature,
          Primitives.Ability.static (getsPt
            (Primitives.NounPhrase.attachHost .enchanted (.type .creature))
            (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 2))),
          Primitives.Ability.static (Primitives.StaticSpec.abilityGrant
            (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (keyword (.named "flying"))),
          whenever (Primitives.GameEvent.dies (creatureDamagedByAttachedHostThisTurn .enchanted))
            (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter)
              (that (.type .creature)) .many true) ] } }

/-- Scythe of the Wretched. -/
def scytheOfTheWretched : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Scythe of the Wretched", cost := some [generic 2], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ Primitives.Ability.static (getsPt
            (Primitives.NounPhrase.attachHost .equipped (.type .creature))
            (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 2))),
          whenever (Primitives.GameEvent.dies (creatureDamagedByAttachedHostThisTurn .equipped))
            (Primitives.Instruction.sequentially
              [ returnTo (that .card) battlefield
                  [Primitives.MoveRider.entry (Primitives.EntryRider.under Primitives.NounPhrase.you)],
                Primitives.Instruction.attachTo thisEquipment (that (.type .creature)) ]),
          keywordCosting (.named "equip") (Primitives.Cost.mana [generic 4]) ] } }

end Semantics.Cards

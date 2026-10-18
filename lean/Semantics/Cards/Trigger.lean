import Semantics
import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Cards.Anaphora

/-!
# Semantics.Cards.Trigger

Port of `idris/src/Experimental/Cards/Trigger.idr`: the printed cards of the Trigger family
(headers, intervening-ifs, joined and alternative headers, delayed and replacement shapes) and
the bench items beside them.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Cards

def gracefulReprieve : Action :=
  delayWithin (does (actor (target isCreature)) die) thisTurn (putInto (that .card) battlefield)
theorem okGracefulReprieve : Action.check (DRS.ofList []) gracefulReprieve = [] := by decide

def cloudkinSeer : Ability := when (does (actor thisCreature) enter) (Primitives.Action.draw (.lit 1))
theorem okCloudkinSeer : Ability.check (DRS.ofList []) cloudkinSeer = [] := by decide
def promiseOfTomorrow : Ability := whenever (does (actor (a (and [isCreature, youControl]))) die) (exile it)
theorem okPromiseOfTomorrow : Ability.check (DRS.ofList []) promiseOfTomorrow = [] := by decide
def libraryLarcenist : Ability := whenever (does (actor thisCreature) attack) (Primitives.Action.draw (.lit 1))
theorem okLibraryLarcenist : Ability.check (DRS.ofList []) libraryLarcenist = [] := by decide
def jhessianThief : Ability :=
  whenever (does (actor thisCreature) (dealDamage (to := a Primitives.Predicate.anyPlayer) (kind := .combatOnly))) (Primitives.Action.draw (.lit 1))
theorem okJhessianThief : Ability.check (DRS.ofList []) jhessianThief = [] := by decide
def scholarOfStars : Ability :=
  triggeredIf (does (actor thisCreature) enter)
    (exists_ (Primitives.Predicate.and [isArtifact, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) (Primitives.Action.draw (.lit 1))
theorem okScholarOfStars : Ability.check (DRS.ofList []) scholarOfStars = [] := by decide

def miserysShadow : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.instead (Primitives.WouldClause.would Primitives.Subject.any (Primitives.Action.move (a (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasPossessor .controller (a (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you))])) (battlefield) (some graveyard) .departure none [])) (exile it) none .repeatedly none)
theorem okMiserysShadow : Ability.check (DRS.ofList []) miserysShadow = [] := by decide

def beastWhisperer : Ability :=
  whenever (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.procedure .cast ((a (Primitives.Predicate.and
    [isCreature, isSpell]))) Primitives.PlayPayment.itsOwnCost none none none)) (Primitives.Action.draw (.lit 1))
theorem okBeastWhisperer : Ability.check (DRS.ofList []) beastWhisperer = [] := by decide
def mesmericOrb : Ability :=
  whenever (Primitives.GameEvent.does Primitives.Subject.any (Primitives.Action.setStatus .untapped (a isPermanent)))
    (instruct (controllerOf (that .permanent)) (mill (.lit 1) they))
theorem okMesmericOrb : Ability.check (DRS.ofList []) mesmericOrb = [] := by decide
def secretPlans : Ability :=
  whenever (Primitives.GameEvent.does Primitives.Subject.any (Primitives.Action.setStatus .faceUp (a (Primitives.Predicate.and [isPermanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))))
    (Primitives.Action.draw (.lit 1))
theorem okSecretPlans : Ability.check (DRS.ofList []) secretPlans = [] := by decide

/-- Teferi's Imp -/
def teferisImpPhasesOut : Ability :=
  whenever (Primitives.GameEvent.does Primitives.Subject.any (Primitives.Action.setStatus .phasedOut thisCreature)) (discard (a (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ yourHand])))
theorem okTeferisImpPhasesOut : Ability.check (DRS.ofList []) teferisImpPhasesOut = [] := by decide
/-- Teferi's Imp -/
def teferisImpPhasesIn : Ability :=
  whenever (Primitives.GameEvent.does Primitives.Subject.any (Primitives.Action.setStatus .phasedIn thisCreature)) (Primitives.Action.draw (.lit 1))
theorem okTeferisImpPhasesIn : Ability.check (DRS.ofList []) teferisImpPhasesIn = [] := by decide

/-- Oubliette -/
def oubliette : Ability :=
  when (does (actor thisEnchantment) enter)
    (phaseOutUntil (target isCreature) (does (actor thisEnchantment) leave))
theorem okOubliette : Ability.check (DRS.ofList []) oubliette = [] := by decide

def shimmeringEfreet : Ability :=
  whenever (Primitives.GameEvent.does Primitives.Subject.any (Primitives.Action.setStatus .phasedIn thisCreature)) (Primitives.Action.setStatus .phasedOut (target isCreature))
theorem okShimmeringEfreet : Ability.check (DRS.ofList []) shimmeringEfreet = [] := by decide
def hollowhengeSpirit : Ability :=
  when (does (actor thisCreature) enter)
    (Primitives.Action.removeFromCombat (target (Primitives.Predicate.and [isCreature, Primitives.Predicate.or [isAttacking, isBlocking]])))
theorem okHollowhengeSpirit : Ability.check (DRS.ofList []) hollowhengeSpirit = [] := by decide
def netcasterSpider : Ability :=
  whenever (does (actor thisCreature) (block (a (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasKeyword (.the (.named "flying"))]))))
    (get thisCreature (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 0)) (some untilEndOfTurn))
theorem okNetcasterSpider : Ability.check (DRS.ofList []) netcasterSpider = [] := by decide
def viashinoWeaponsmith : Ability :=
  whenever (does (by_ (a isCreature)) (block thisCreature))
    (get thisCreature (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 2)) (some untilEndOfTurn))
theorem okViashinoWeaponsmith : Ability.check (DRS.ofList []) viashinoWeaponsmith = [] := by decide
def somberwaldAlpha : Ability :=
  whenever (does Primitives.Subject.any (block (a (and [isCreature, youControl]))))
    (get it (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1)) (some untilEndOfTurn))
theorem okSomberwaldAlpha : Ability.check (DRS.ofList []) somberwaldAlpha = [] := by decide
def vertigoSpawn : Ability :=
  whenever (does (actor thisCreature) (block (a isCreature)))
    (Primitives.Action.sequentially
      [ Primitives.Action.setStatus .tapped (that (.type .creature)),
        Primitives.Action.skipUntap (that (.type .creature)) (.lit 1) ])
theorem okVertigoSpawn : Ability.check (DRS.ofList []) vertigoSpawn = [] := by decide
def orneryDilophosaur : Ability :=
  triggeredIf (does (actor thisCreature) attack)
    (exists_ (Primitives.Predicate.and [ isCreature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you,
                     Primitives.Predicate.compare [.stat .power] .atLeast (.lit 4) ]))
    (get thisCreature (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 2)) (some untilEndOfTurn))
theorem okOrneryDilophosaur : Ability.check (DRS.ofList []) orneryDilophosaur = [] := by decide
def incisorGlider : Ability :=
  triggeredIf (does (actor thisCreature) attack)
    (Primitives.Condition.compareAmt (countersOn (.named "poison") (a (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you))) .atLeast (.lit 3))
    (get (allOf (and [isCreature, youControl])) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1)) (some untilEndOfTurn))
theorem okIncisorGlider : Ability.check (DRS.ofList []) incisorGlider = [] := by decide
def stormFleetSpy : Ability :=
  triggeredIf (does (actor thisCreature) enter) (happened (Primitives.Action.attack none) (Primitives.Subject.actor Primitives.NounPhrase.you) soFarThisTurn)
    (Primitives.Action.draw (.lit 1))
theorem okStormFleetSpy : Ability.check (DRS.ofList []) stormFleetSpy = [] := by decide
def loanShark : Ability :=
  triggeredIf (does (actor thisCreature) enter)
    (Primitives.Condition.compareAmt (eventCount (Primitives.Action.procedure .cast (Primitives.NounPhrase.any) Primitives.PlayPayment.itsOwnCost none none none) (Primitives.Subject.actor Primitives.NounPhrase.you) soFarThisTurn) .atLeast (.lit 2)) (Primitives.Action.draw
      (.lit 1))
theorem okLoanShark : Ability.check (DRS.ofList []) loanShark = [] := by decide

def forceOfDespair : Action :=
  destroy (allOf (Primitives.Predicate.and [isCreature, happenedTo Primitives.Subject.any (Primitives.Action.move (relative .object) Primitives.Location.anyLocation (some battlefield) .arrival none []) soFarThisTurn]))
theorem okForceOfDespair : Action.check (DRS.ofList []) forceOfDespair = [] := by decide
def cradleToGrave : Action :=
  destroy (target (Primitives.Predicate.and [isCreature, Primitives.Predicate.not
    (Primitives.Predicate.colorIs .black), happenedTo Primitives.Subject.any (Primitives.Action.move (relative
    .object) Primitives.Location.anyLocation (some battlefield) .arrival none []) soFarThisTurn]))
theorem okCradleToGrave : Action.check (DRS.ofList []) cradleToGrave = [] := by decide

def aragornKingOfGondor : Ability :=
  when (does (actor thisCreature) enter) (Primitives.Action.gainDesignation Primitives.NounPhrase.you (.named "monarch") none)
theorem okAragornKingOfGondor : Ability.check (DRS.ofList []) aragornKingOfGondor = [] := by decide
def firmamentSage : Ability := whenever (Primitives.GameEvent.gameBecomes (.named "night")) (Primitives.Action.draw (.lit 1))
theorem okFirmamentSage : Ability.check (DRS.ofList []) firmamentSage = [] := by decide
def deeprootWarrior : Ability :=
  whenever (does Primitives.Subject.any (block thisCreature))
    (get it (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1)) (some untilEndOfTurn))
theorem okDeeprootWarrior : Ability.check (DRS.ofList []) deeprootWarrior = [] := by decide
def borderlandMarauder : Ability :=
  whenever (does (actor thisCreature) attack) (get it (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 0)) (some untilEndOfTurn))
theorem okBorderlandMarauder : Ability.check (DRS.ofList []) borderlandMarauder = [] := by decide
def lichsMasteryLoss : Ability :=
  when (does (actor thisEnchantment) leave) (Primitives.Action.conclude .loseGame)
theorem okLichsMasteryLoss : Ability.check (DRS.ofList []) lichsMasteryLoss = [] := by decide
def phageTheUntouchable : Ability :=
  whenever (does (actor thisCreature) (dealDamage (to := a Primitives.Predicate.anyPlayer) (kind := .combatOnly))) (instruct (that .player) (Primitives.Action.conclude .loseGame))
theorem okPhageTheUntouchable : Ability.check (DRS.ofList []) phageTheUntouchable = [] := by decide
def elderscaleWurm : Ability :=
  triggeredIf (does (actor thisCreature) enter) (Primitives.Condition.compareAmt (lifeTotalOf Primitives.NounPhrase.you) .less (.lit 7))
    (setLife (.lit 7))
theorem okElderscaleWurm : Ability.check (DRS.ofList []) elderscaleWurm = [] := by decide
/-- Krang, Master Mind -/
def krang : Ability :=
  triggeredIf (does (actor thisCreature) enter)
    (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (handOf Primitives.NounPhrase.you)])) .less (.lit 4)) (Primitives.Action.draw Primitives.Amount.theDifference)
theorem okKrang : Ability.check (DRS.ofList []) krang = [] := by decide

def carrionGrub : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Carrion Grub", cost := some [generic 3, pip .black], types := [.creature],
      subtypes := [creatureType "Insect"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ getsPt thisCreature (Primitives.Delta.up (Primitives.Amount.letter .x)) (Primitives.Delta.up (.lit 0)),
              Primitives.StaticSpec.letterDefinition .x
                (aggregate .max (.stat .power) (Primitives.Predicate.and [isCreature, Primitives.Predicate.in_ (graveyardOf Primitives.NounPhrase.you)])) ]),
          when (does (actor thisCreature) enter) (mill (.lit 4) Primitives.NounPhrase.you) ],
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
        [ whenever (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.procedure .cast ((a
          (Primitives.Predicate.and [isInstantOrSorcery, isSpell]))) Primitives.PlayPayment.itsOwnCost none none none))
            (create (.lit 1)
              (Primitives.TokenSpec.written (creatureToken 1 1 [.red] [creatureType "Elemental"])) [])
                  ],
      power := stat 2, toughness := stat 1 } }

def archivistOfGondor : Ability :=
  triggeredIf (does (actor yourCommander) (dealDamage (to := a Primitives.Predicate.anyPlayer) (kind := .combatOnly))) (thereIsNo (.named "monarch"))
    (Primitives.Action.gainDesignation Primitives.NounPhrase.you (.named "monarch") none)
theorem okArchivistOfGondor : Ability.check (DRS.ofList []) archivistOfGondor = [] := by decide

def hissingMiasma : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hissing Miasma", cost := some [generic 1, pip .black, pip .black],
      types := [.enchantment],
      text :=
        [ whenever (does (actor (a isCreature)) (attack (some Primitives.NounPhrase.you))) (instruct (controllerOf it) (loseLife (.lit 1))) ] } }

def orimsPrayer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Orim's Prayer", cost := some [generic 1, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ whenever (does (actor (counted (atLeast 1) isCreature)) (attack (some Primitives.NounPhrase.you)))
            (gainLife (forEach (.lit 1) (Primitives.Predicate.and [isAttacking, isCreature]))) ] } }

def lesserGargadon : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lesser Gargadon", cost := some [generic 2, pip .red, pip .red],
      types := [.creature], subtypes := [creatureType "Beast"],
      text :=
        [ triggeredOr (does (actor thisCreature) attack) [does (actor thisCreature) block]
            (sacrifice (a isLand)) ],
      power := stat 6, toughness := stat 4 } }

/-- Chub Toad -/
def chubToad : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chub Toad", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [creatureType "Frog"],
      text :=
        [ triggeredOr (does (actor thisCreature) block) [does Primitives.Subject.any (block thisCreature)]
            (get it (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 2)) (some untilEndOfTurn)) ],
      power := stat 1, toughness := stat 1 } }

/-- Giggling Skitterspike -/
def gigglingSkitterspikeTriggers : List GameEvent :=
  [does (actor thisCreature) block, Primitives.GameEvent.does (Primitives.Subject.actor (a isSpell)) (Primitives.Action.target thisCreature)]
theorem okGigglingSkitterspikeTriggers :
    GameEvent.checkAll (DRS.ofList []) gigglingSkitterspikeTriggers = [] := by decide

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
                (does (actor (a (Primitives.Predicate.and [ Primitives.Predicate.hasSubtype (creatureType "Wizard"),
                                    Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you ]))) enter))
              (Primitives.GameEvent.triggers (a (Primitives.Predicate.and [ Primitives.Predicate.abilityHead .anyTriggered,
                           Primitives.Predicate.abilityOf (a (Primitives.Predicate.and [isPermanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) ])) none none))
            (exactly 1)) ],
      power := stat 2, toughness := stat 1 } }

/-- Bloom Hulk -/
def bloomHulk : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bloom Hulk", cost := some [generic 3, pip .green], types := [.creature],
      subtypes := [creatureType "Plant", creatureType "Elemental"],
      text := [when (does (actor thisCreature) enter) proliferate],
      power := stat 4, toughness := stat 4 } }

/-- Stalwart Successor -/
def stalwartSuccessorHeader : GameEvent := bareCounterEvent .put .many (a (and [isCreature, youControl]))
theorem okStalwartSuccessorHeader : GameEvent.check (DRS.ofList []) stalwartSuccessorHeader = [] := by decide

/-- Hollowmurk Siege -/
def hollowmurkSiegeSultai : Ability :=
  triggeredOnlyOnce (bareCounterEvent .put .one (a (and [isCreature, youControl]))) oncePerTurn
    (Primitives.Action.draw (.lit 1))
theorem okHollowmurkSiegeSultai : Ability.check (DRS.ofList []) hollowmurkSiegeSultai = [] := by decide

/-- Clone -/
def clone : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Clone", cost := some [generic 3, pip .blue], types := [.creature],
      subtypes := [creatureType "Shapeshifter"],
      text := [Primitives.Ability.static (Primitives.StaticSpec.entryRider thisCreature (Primitives.MoveRider.asCopyOf true (a isCreature) []))],
      power := stat 0, toughness := stat 0 } }

/-- Quicksilver Gargantuan -/
def quicksilverGargantuan : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Quicksilver Gargantuan", cost := some [generic 5, pip .blue, pip .blue],
      types := [.creature], subtypes := [creatureType "Shapeshifter"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.entryRider thisCreature
            (Primitives.MoveRider.asCopyOf true (a isCreature) [Primitives.CopyExcept.pt (.lit 7) (.lit 7)])) ],
      power := stat 7, toughness := stat 7 } }

/-- Sculpting Steel -/
def sculptingSteel : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sculpting Steel", cost := some [generic 3], types := [.artifact],
      text := [Primitives.Ability.static (Primitives.StaticSpec.entryRider thisArtifact (Primitives.MoveRider.asCopyOf true (a isArtifact) []))] } }

/-- Sakashima's Student -/
def sakashimasStudentCopy : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.entryRider thisCreature
    (Primitives.MoveRider.asCopyOf true (a isCreature) [Primitives.CopyExcept.types [] [creatureType "Ninja"]]))
theorem okSakashimasStudentCopy : Ability.check (DRS.ofList []) sakashimasStudentCopy = [] := by decide
/-- Chameleon, Master of Disguise -/
def chameleonCopy : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.entryRider thisCreature
    (Primitives.MoveRider.asCopyOf true (a (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
      [Primitives.CopyExcept.name "Chameleon, Master of Disguise"]))
theorem okChameleonCopy : Ability.check (DRS.ofList []) chameleonCopy = [] := by decide

def nefariousLichGain : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.instead (Primitives.WouldClause.would (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.changeLife (Primitives.Delta.up Primitives.Amount.anyAmount))) (Primitives.Action.draw Primitives.Amount.thatMuch) none .repeatedly none)
theorem okNefariousLichGain : Ability.check (DRS.ofList []) nefariousLichGain = [] := by decide

def piousWarrior : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Pious Warrior", cost := some [generic 3, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Rebel", creatureType "Warrior"],
      text := [whenever (does Primitives.Subject.any (dealDamage (to := thisCreature) (kind := .combatOnly))) (gainLife Primitives.Amount.thatMuch)],
      power := stat 2, toughness := stat 3 } }

def sanguineBond : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sanguine Bond", cost := some [generic 3, pip .black, pip .black],
      types := [.enchantment],
      text := [whenever (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.changeLife (Primitives.Delta.up Primitives.Amount.anyAmount))) (instruct (target (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)) (loseLife Primitives.Amount.thatMuch))]
          } }

def exquisiteBlood : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Exquisite Blood", cost := some [generic 4, pip .black], types := [.enchantment],
      text := [whenever (Primitives.GameEvent.does (Primitives.Subject.actor anOpponent) (Primitives.Action.changeLife (Primitives.Delta.down Primitives.Amount.anyAmount))) (gainLife Primitives.Amount.thatMuch)] } }

/-- Agate-Blade Assassin -/
def agateBladeAssassin : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Agate-Blade Assassin", cost := some [generic 1, pip .black], types := [.creature],
      subtypes := [creatureType "Lizard", creatureType "Assassin"],
      text :=
        [ whenever (does (actor thisCreature) attack)
            (Primitives.Action.sequentially
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
          Primitives.Ability.static (getsPt (allOf ((and [isCreature, youControl, otherThan thisCreature]))) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1))),
          whenever (does (actor you) attack) (Primitives.Action.draw (.lit 1)) ],
      power := stat 4, toughness := stat 4 } }

/-- Fiend Binder -/
def fiendBinder : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fiend Binder", cost := some [generic 3, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Soldier"],
      text :=
        [ whenever (does (actor thisCreature) attack)
            (Primitives.Action.setStatus .tapped
              (target (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasPossessor .controller (Primitives.NounPhrase.combatPlayer .defending)]))) ],
      power := stat 3, toughness := stat 2 } }

/-- Souls of the Faultless -/
def soulsOfTheFaultlessDrain : Ability :=
  whenever (does Primitives.Subject.any (dealDamage (to := thisCreature) (kind := .combatOnly)))
    (instruct (Primitives.NounPhrase.combatPlayer .attacking) (loseLife Primitives.Amount.thatMuch))
theorem okSoulsOfTheFaultlessDrain : Ability.check (DRS.ofList []) soulsOfTheFaultlessDrain = [] := by decide

def unstableShapeshifter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Unstable Shapeshifter", cost := some [generic 3, pip .blue], types := [.creature],
      subtypes := [creatureType "Shapeshifter"],
      text :=
        [ whenever (does (actor (a (Primitives.Predicate.and [isCreature, Primitives.Predicate.otherThan Primitives.NounPhrase.this]))) enter)
            (Primitives.Action.establish (Primitives.StaticSpec.copyChange thisCreature (that (.type .creature)) [Primitives.CopyExcept.thisAbility])
              none) ],
      power := stat 0, toughness := stat 1 } }

def prismRing : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Prism Ring", cost := some [generic 1], types := [.artifact],
      text :=
        [ Primitives.Ability.static (entersChoosing thisArtifact .color),
          whenever (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.procedure .cast ((a
            (Primitives.Predicate.and [isSpell, ofChosen .color]))) Primitives.PlayPayment.itsOwnCost none none none)) (gainLife (.lit 1)) ] } }

/-- Aisling Leprechaun -/
def aislingLeprechaun : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aisling Leprechaun", cost := some [pip .green], types := [.creature],
      subtypes := [creatureType "Faerie"],
      text :=
        [ triggeredOr (does (actor thisCreature) (block (a isCreature)))
            [does (by_ (a isCreature)) (block thisCreature)]
            (becomeColor (that (.type .creature)) (.some [.green]) none) ],
      power := stat 1, toughness := stat 1 } }

def avenShrine : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aven Shrine", cost := some [generic 1, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ whenever (Primitives.GameEvent.does (Primitives.Subject.actor (a Primitives.Predicate.anyPlayer)) (Primitives.Action.procedure .cast ((a isSpell)) Primitives.PlayPayment.itsOwnCost none none none))
            (Primitives.Action.sequentially
              [ instruct they (gainLife (Primitives.Amount.letter .x)),
                Primitives.Action.define .x (countOf (Primitives.Predicate.and [Primitives.Predicate.in_ graveyard, (Primitives.Predicate.shares .cardName .share (that .spell))])) ]) ] } }

def chromeReplicator : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chrome Replicator", cost := some [generic 5], types := [.artifact, .creature],
      subtypes := [creatureType "Construct"],
      text :=
        [ triggeredIf (does (actor thisCreature) enter)
            (Primitives.Condition.exists_ (withTheSameName
              (counted (atLeast 2)
                (Primitives.Predicate.and [isPermanent, Primitives.Predicate.not isLand, isNontoken, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))))
            (createToken
              { characteristics :=
                { types := [.artifact, .creature], subtypes := [creatureType "Construct"],
                  power := stat 4, toughness := stat 4 } }) ],
      power := stat 4, toughness := stat 4 } }

def admiralsOrder : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Admiral's Order", cost := some [generic 1, pip .blue, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.static (onlyWhile (Primitives.StaticSpec.altCost Primitives.NounPhrase.this (some (Primitives.Cost.mana [pip .blue])))
            (happened (Primitives.Action.attack none) (Primitives.Subject.actor Primitives.NounPhrase.you) soFarThisTurn)),
          Primitives.Ability.spell none (counter (target isSpell)) ] } }

def fyndhornDruid : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fyndhorn Druid", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [creatureType "Elf", creatureType "Druid"],
      text :=
        [ triggeredIf (does (actor thisCreature) die) (happened (Primitives.Action.block it) Primitives.Subject.any soFarThisTurn)
            (gainLife (.lit 4)) ],
      power := stat 2, toughness := stat 2 } }

def kamiOfTerribleSecrets : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Kami of Terrible Secrets", cost := some [generic 3, pip .black], types := [.creature],
      subtypes := [creatureType "Spirit"],
      text :=
        [ triggeredIf (does (actor thisCreature) enter)
            (Primitives.Condition.and [ exists_ (Primitives.Predicate.and [isArtifact, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]),
                    exists_ (Primitives.Predicate.and [isEnchantment, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]) ])
            (Primitives.Action.sequentially [Primitives.Action.draw (.lit 1), gainLife (.lit 1)]) ],
      power := stat 3, toughness := stat 4 } }

def dreadSlaver : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dread Slaver", cost := some [generic 3, pip .black, pip .black],
      types := [.creature], subtypes := [creatureType "Zombie", creatureType "Horror"],
      text :=
        [ whenever
            (does (actor (a (Primitives.Predicate.and [isCreature, happenedTo (Primitives.Subject.actor thisCreature) (Primitives.Action.dealDamage Primitives.Amount.anyAmount               (Primitives.NounPhrase.asMarker .permanent (relative .object)) .any) soFarThisTurn]))) die)
            (Primitives.Action.sequentially
              [ putOntoBattlefieldUnderYourControl it,
                become (that (.type .creature))
                  { characteristics := { colors := [.black], subtypes := [creatureType "Zombie"] } }
                  none ]) ],
      power := stat 3, toughness := stat 5 } }

/-- Steppe Lynx -/
def steppeLynx : Ability :=
  abilityWord "landfall"
    (whenever (does (actor (a (Primitives.Predicate.and [isLand, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))) enter)
      (get thisCreature (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 2)) (some untilEndOfTurn)))
theorem okSteppeLynx : Ability.check (DRS.ofList []) steppeLynx = [] := by decide
/-- Owlbear -/
def owlbear : Ability :=
  flavorWord "Keen Senses" (when (does (actor thisCreature) enter) (Primitives.Action.draw (.lit 1)))
theorem okOwlbear : Ability.check (DRS.ofList []) owlbear = [] := by decide
/-- Netherese Puzzle-Ward -/
def netheresePuzzleWardIllumination : Ability :=
  whenever (does (actor you) (rollDice (watch := Primitives.RollWatch.highestNatural))) (Primitives.Action.draw (.lit 1))
theorem okNetheresePuzzleWardIllumination :
    Ability.check (DRS.ofList []) netheresePuzzleWardIllumination = [] := by decide
/-- Farideh, Devil's Chosen -/
def faridehResultRead : Ability :=
  whenever (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.rollDice Primitives.Amount.anyAmount .anyDie .many Primitives.RollWatch.anyResult))
    (Primitives.Action.sequentially
      [ gain thisCreature (keyword (.named "flying")) (some untilEndOfTurn),
        gain thisCreature (keyword (.named "menace")) (some untilEndOfTurn),
        doIf (Primitives.Condition.anyResultIs .atLeast (.lit 10)) (Primitives.Action.draw (.lit 1)) ])
theorem okFaridehResultRead : Ability.check (DRS.ofList []) faridehResultRead = [] := by decide
/-- Jaws of Defeat -/
def jawsOfDefeat : Ability :=
  whenever (does (actor (a (and [isCreature, youControl]))) enter)
    (instruct (target (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)) (loseLife
      (Primitives.Amount.arith .differenceBetween (Primitives.Amount.statOf (.stat .power) (that (.type .creature)))
        (Primitives.Amount.statOf (.stat .toughness) (that (.type .creature))))))
theorem okJawsOfDefeat : Ability.check (DRS.ofList []) jawsOfDefeat = [] := by decide
/-- Defiling Daemogoth -/
def defilingDaemogothDrain : Action :=
  Primitives.Action.sequentially
    [instruct (each (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)) (loseLife (Primitives.Amount.letter .x)),
      Primitives.Action.define .x (eventSum (Primitives.Action.changeLife (Primitives.Delta.up Primitives.Amount.anyAmount)) (Primitives.Subject.actor Primitives.NounPhrase.you) soFarThisTurn)]
theorem okDefilingDaemogothDrain : Action.check (DRS.ofList []) defilingDaemogothDrain = [] := by decide
def skullsporeNexusTrigger : Ability :=
  whenever (does (actor (counted (atLeast 1) (Primitives.Predicate.and [isNontoken, and [isCreature, youControl]]))) die)
    (createToken (creatureTokenOf (Primitives.Amount.aggregate .sum (.stat .power) (those .card))
        (Primitives.Amount.aggregate .sum (.stat .power) (those .card))
        [.green] [creatureType "Fungus", creatureType "Dinosaur"]))
theorem okSkullsporeNexusTrigger : Ability.check (DRS.ofList []) skullsporeNexusTrigger = [] := by decide
/-- Wavebreak Hippocamp -/
def wavebreakHippocamp : Ability :=
  triggeredOnlyDuring (Primitives.GameEvent.nthOccurrence (.nth 1) none (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.procedure .cast ((a isSpell)) Primitives.PlayPayment.itsOwnCost none none none)))
    (duringPart .turn (some (each (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)))) (Primitives.Action.draw (.lit 1))
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
  when (Primitives.GameEvent.does (Primitives.Subject.actor (a Primitives.Predicate.anyPlayer)) (Primitives.Action.pay Primitives.Cost.anyCost .once .unpaid (some thisEnchantment) (some (.named "cumulativeUpkeep"))))
    (instruct (that .player) (exile (each (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (Primitives.Location.zone .library (some they))]))))
theorem okThoughtLashTrigger : Ability.check (DRS.ofList []) thoughtLashTrigger = [] := by decide
/-- Heart of Bogardan -/
def heartOfBogardanHeader : GameEvent :=
  Primitives.GameEvent.does (Primitives.Subject.actor (a Primitives.Predicate.anyPlayer)) (Primitives.Action.pay Primitives.Cost.anyCost .once .unpaid (some thisEnchantment) (some (.named "cumulativeUpkeep")))
theorem okHeartOfBogardanHeader : GameEvent.check (DRS.ofList []) heartOfBogardanHeader = [] := by decide
/-- Balduvian Fallen -/
def balduvianFallenHeader : GameEvent := Primitives.GameEvent.does Primitives.Subject.any (Primitives.Action.pay Primitives.Cost.anyCost .once .paid (some thisCreature) (some (.named "cumulativeUpkeep")))
theorem okBalduvianFallenHeader : GameEvent.check (DRS.ofList []) balduvianFallenHeader = [] := by decide
/-- Stormscape Battlemage -/
def stormscapeBattlemageFirstKicker : Ability :=
  triggeredIf (does (actor thisCreature) enter)
    (costWasPaid (.byNthKeyword (.nth 1) (.named "kicker")) none thisCreature) (gainLife (.lit 3))
theorem okStormscapeBattlemageFirstKicker :
    Ability.check (DRS.ofList []) stormscapeBattlemageFirstKicker = [] := by decide
/-- All-Seeing Arbiter -/
def allSeeingArbiterHeader : GameEvent :=
  Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (discard ((a
    (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ hand]))))
theorem okAllSeeingArbiterHeader : GameEvent.check (DRS.ofList []) allSeeingArbiterHeader = [] := by decide

/-- Liliana's Caress -/
def lilianasCaress : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Liliana's Caress", cost := some [generic 1, pip .black], types := [.enchantment],
      text :=
        [ whenever (Primitives.GameEvent.does (Primitives.Subject.actor anOpponent) (discard ((a
          (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ hand])))))
            (instruct they (loseLife (.lit 2))) ] } }

/-- Scheming Aspirant -/
def schemingAspirant : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Scheming Aspirant", cost := some [generic 1, pip .black], types := [.creature],
      subtypes := [creatureType "Phyrexian", creatureType "Advisor"],
      text :=
        [ whenever (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (proliferate))
            (Primitives.Action.sequentially [instruct (each (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)) (loseLife (.lit 2)), gainLife (.lit 2)]) ],
      power := stat 1, toughness := stat 3 } }

/-- Reciprocate -/
def reciprocate : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Reciprocate", cost := some [pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (exile
            (target (Primitives.Predicate.and [isCreature, happenedTo (Primitives.Subject.actor (relative .object)) (Primitives.Action.dealDamage Primitives.Amount.anyAmount               Primitives.NounPhrase.you .any) soFarThisTurn]))) ] } }

/-- Tahngarth, First Mate -/
def tahngarthHeader : GameEvent := Primitives.GameEvent.does (Primitives.Subject.actor anOpponent) (Primitives.Action.attackWith (counted (atLeast 1) isCreature) none)
theorem okTahngarthHeader : GameEvent.check (DRS.ofList []) tahngarthHeader = [] := by decide
/-- Myth Unbound -/
def mythUnboundTrigger : Ability :=
  whenever (does Primitives.Subject.any (putInto yourCommander commandZone (from_ := some Primitives.Location.anyLocation))) (Primitives.Action.draw (.lit 1))
theorem okMythUnboundTrigger : Ability.check (DRS.ofList []) mythUnboundTrigger = [] := by decide

/-- Commander's Insignia -/
def commandersInsignia : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Commander's Insignia", cost := some [generic 2, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ Primitives.Ability.static (getsPt (allOf (and [isCreature, youControl]))
            (Primitives.Delta.up (eventCount (Primitives.Action.procedure .cast (((Primitives.NounPhrase.asMarker .spell yourCommander))) Primitives.PlayPayment.itsOwnCost none (some
              (commandZone)) none) (Primitives.Subject.actor Primitives.NounPhrase.you) thisGame))
            (Primitives.Delta.up (eventCount (Primitives.Action.procedure .cast (((Primitives.NounPhrase.asMarker .spell yourCommander))) Primitives.PlayPayment.itsOwnCost none (some
              (commandZone)) none) (Primitives.Subject.actor Primitives.NounPhrase.you) thisGame)))
              ] } }

/-- Faith's Reward -/
def faithsReward : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Faith's Reward", cost := some [generic 3, pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (putInto
            (allOf (Primitives.Predicate.and [ isPermanentCard, Primitives.Predicate.in_ (graveyardOf Primitives.NounPhrase.you),
                           Primitives.Predicate.did (⟨Primitives.Subject.any, (Primitives.Action.move (relative .object) battlefield (some (graveyardOf
                             Primitives.NounPhrase.you)) .departure none []), (Primitives.LookbackSpan.duration soFarThisTurn none)⟩) ]))
            battlefield) ] } }

/-- Oscorp Industries -/
def oscorpIndustriesReturn : Ability :=
  when (does (actor thisLand) (enter (from_ := some (graveyard)))) (loseLife (.lit 2))
theorem okOscorpIndustriesReturn : Ability.check (DRS.ofList []) oscorpIndustriesReturn = [] := by decide
def theLostAndTheDamnedEntryArm : GameEvent :=
  does (actor (a (Primitives.Predicate.and [isLand, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))) (enter (from_ := some (Primitives.Location.not (handOf Primitives.NounPhrase.you))))
theorem okTheLostAndTheDamnedEntryArm :
    GameEvent.check (DRS.ofList []) theLostAndTheDamnedEntryArm = [] := by decide
/-- Forsaken Wastes -/
def forsakenWastesTargeted : Ability :=
  whenever (Primitives.GameEvent.does (Primitives.Subject.actor (a isSpell)) (Primitives.Action.target thisEnchantment))
    (instruct (controllerOf (that .spell)) (loseLife (.lit 5)))
theorem okForsakenWastesTargeted : Ability.check (DRS.ofList []) forsakenWastesTargeted = [] := by decide

/-- Fblthp, the Lost -/
def fblthp : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fblthp, the Lost", cost := some [generic 1, pip .blue],
      supertypes := [.legendary], types := [.creature], subtypes := [creatureType "Homunculus"],
      text :=
        [ when (does (actor «this») enter)
            (Primitives.Action.replace (Primitives.Action.draw (.lit 1))
              (Primitives.Action.doIf (Primitives.Condition.or
                  [ Primitives.Condition.did (⟨Primitives.Subject.any, (Primitives.Action.move it                     (yourLibrary) (some battlefield) .arrival none []), Primitives.LookbackSpan.triggering⟩),
                    Primitives.Condition.matches it (Primitives.Predicate.castFrom yourLibrary) ])
                (Primitives.Action.draw (.lit 2)) none)),
          when (Primitives.GameEvent.does (Primitives.Subject.actor (a isSpell)) (Primitives.Action.target Primitives.NounPhrase.this)) (shuffleInto Primitives.NounPhrase.this) ],
      power := stat 1, toughness := stat 1 } }

/-- Frost Walker -/
def frostWalker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Frost Walker", cost := some [generic 1, pip .blue], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text :=
        [ when (Primitives.GameEvent.does (Primitives.Subject.actor (a (Primitives.Predicate.or [isSpell, Primitives.Predicate.abilityHead .anyOnStack]))) (Primitives.Action.target thisCreature))
            (sacrifice itPermanent) ],
      power := stat 4, toughness := stat 1 } }

/-- Loaming Shaman -/
def loamingShaman : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Loaming Shaman", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [creatureType "Centaur", creatureType "Shaman"],
      text :=
        [ when (does (actor thisCreature) enter)
            (instruct (target Primitives.Predicate.anyPlayer) (shuffleInto
              (Primitives.NounPhrase.described (Primitives.DetPhrase.target anyNumber) (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (graveyardOf they)])))) ],
      power := stat 3, toughness := stat 2 } }

/-- Debris Beetle -/
def debrisBeetleTrigger : Ability :=
  when (does (actor thisVehicle) enter)
    (Primitives.Action.sequentially [instruct (each (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)) (loseLife (.lit 3)), gainLife (.lit 3)])
theorem okDebrisBeetleTrigger : Ability.check (DRS.ofList []) debrisBeetleTrigger = [] := by decide
/-- Roads Go Ever, Ever On's chapters II and III -/
def roadsGoEverEverOnChapters : Ability :=
  when (Primitives.GameEvent.chapterMark [2, 3]) (putInto (a (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (Primitives.Location.exiledWith thisSaga)])) hand)
theorem okRoadsGoEverEverOnChapters : Ability.check (DRS.ofList []) roadsGoEverEverOnChapters = [] := by decide
/-- Wurmwall Sweeper -/
def wurmwallSweeperTrigger : Ability := when (does (actor thisSpacecraft) enter) (surveil (.lit 2))
theorem okWurmwallSweeperTrigger : Ability.check (DRS.ofList []) wurmwallSweeperTrigger = [] := by decide
/-- Case of the Crimson Pulse -/
def caseOfTheCrimsonPulseTrigger : Ability :=
  when (does (actor thisCase) enter)
    (Primitives.Action.sequentially [discard (a (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ yourHand])), Primitives.Action.draw (.lit 2)])
theorem okCaseOfTheCrimsonPulseTrigger : Ability.check (DRS.ofList []) caseOfTheCrimsonPulseTrigger = [] := by
  decide

/-- Ferocious Pup -/
def ferociousPup : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ferocious Pup", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [creatureType "Wolf"],
      text :=
        [ when (does (actor thisCreature) enter)
            (createToken (creatureToken 2 2 [.green] [creatureType "Wolf"])) ],
      power := stat 0, toughness := stat 1 } }

/-- Agency Outfitter's search -/
def agencyOutfitterSearch : Action :=
  Primitives.Action.sequentially
    [ may
        (Primitives.Action.sequentially
          [ searchZonesOf Primitives.NounPhrase.you (exactly 1)
              (Primitives.Predicate.or [Primitives.Predicate.named (Primitives.NameSource.printed "Magnifying Glass"), Primitives.Predicate.named (Primitives.NameSource.printed "Thinking Cap")]),
            putOntoBattlefield (theVerbed (.action "search") .one) ]),
      Primitives.Action.doIf (happenedThisWay (Primitives.Action.search (Primitives.Location.anyLocation) anyNumber (Primitives.Predicate.and [])) (Primitives.Subject.actor Primitives.NounPhrase.you)) shuffle none ]
theorem okAgencyOutfitterSearch : Action.check (DRS.ofList []) agencyOutfitterSearch = [] := by decide

/-- Trouble in Pairs' -/
def troubleInPairsArms : Ability :=
  Primitives.Ability.triggered (Primitives.GameEvent.does (Primitives.Subject.actor anOpponent) (Primitives.Action.attackWith (counted (atLeast 2) isCreature) (some Primitives.NounPhrase.you)))
    [ Primitives.GameEvent.nthOccurrence (.nth 2) (some .turn) (Primitives.GameEvent.does (Primitives.Subject.actor (a (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you))) (Primitives.Action.draw Primitives.Amount.anyAmount)),
      Primitives.GameEvent.nthOccurrence (.nth 2) (some .turn) (Primitives.GameEvent.does (Primitives.Subject.actor (a
        (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you))) (Primitives.Action.procedure .cast ((a isSpell)) Primitives.PlayPayment.itsOwnCost none none none)) ]
    none [] none none none (Primitives.Action.draw (.lit 1))
theorem okTroubleInPairsArms : Ability.check (DRS.ofList []) troubleInPairsArms = [] := by decide

/-- Bioplasm -/
def bioplasm : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bioplasm", cost := some [generic 3, pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Ooze"],
      text :=
        [ whenever (does (actor thisCreature) attack)
            (Primitives.Action.sequentially
              [ exile (topSlice (.lit 1)),
                Primitives.Action.doIf bioplasmCardTest
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
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ replaceEvent (Primitives.WouldClause.would Primitives.Subject.any (Primitives.Action.move (a (Primitives.Predicate.and [isCreature, Primitives.Predicate.not (Primitives.Predicate.castBy Primitives.NounPhrase.any none)])) Primitives.Location.anyLocation (some battlefield) .arrival none [])) (exile it)
                (some untilEndOfTurn),
              Primitives.Action.draw (.lit 1) ]) ] } }

/-- Gather Specimens -/
def gatherSpecimens : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gather Specimens", cost := some [generic 3, pip .blue, pip .blue, pip .blue],
      types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.establish
            (Primitives.StaticSpec.entryRider (a (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasPossessor .controller anOpponent])) (Primitives.MoveRider.under Primitives.NounPhrase.you))
            (some thisTurn)) ] } }

/-- Don't Blink's replacement, without its written agent -/
def dontBlinkReplacement : Action :=
  Primitives.Action.establish
    (Primitives.StaticSpec.instead (Primitives.WouldClause.wouldAny [(Primitives.WouldClause.would Primitives.Subject.any (Primitives.Action.move (counted (atLeast 1) isCreature) (exileZone) (some battlefield) .arrival none [])), (Primitives.WouldClause.would Primitives.Subject.any (Primitives.Action.move (counted (atLeast 1) (Primitives.Predicate.and [isCreature, Primitives.Predicate.castFrom exileZone])) Primitives.Location.anyLocation (some battlefield) .arrival none []))]) (shuffleInto them) none .repeatedly none)
    (some untilEndOfTurn)
theorem okDontBlinkReplacement : Action.check (DRS.ofList []) dontBlinkReplacement = [] := by decide

/-- Seasoned Warrenguard -/
def seasonedWarrenguard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Seasoned Warrenguard", cost := some [pip .white], types := [.creature],
      subtypes := [creatureType "Rabbit", creatureType "Warrior"],
      text :=
        [ triggeredWhile (does (actor thisCreature) attack)
            (whileTrue (exists_ (Primitives.Predicate.and [Primitives.Predicate.isToken, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))
            (get thisCreature (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 0)) (some untilEndOfTurn)) ],
      power := stat 1, toughness := stat 2 } }

/-- Brazen Blademaster -/
def brazenBlademaster : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Brazen Blademaster", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [creatureType "Orc", creatureType "Pirate"],
      text :=
        [ triggeredWhile (does (actor thisCreature) attack)
            (whileTrue
              (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.and [isArtifact, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .atLeast
                (.lit 2)))
            (get it (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 1)) (some untilEndOfTurn)) ],
      power := stat 2, toughness := stat 3 } }

/-- Autarch Mammoth -/
def autarchMammothLine : Ability :=
  triggeredJoined (does (actor thisCreature) enter)
    [ joinedHeadWhile (does (actor thisCreature) attack)
        (whileTrue (Primitives.Condition.matches thisCreature (Primitives.Predicate.hasDesignation (.named "saddled") none))) ]
    (createToken (creatureToken 3 3 [.green] [creatureType "Elephant"]))
theorem okAutarchMammothLine : Ability.check (DRS.ofList []) autarchMammothLine = [] := by decide
/-- Altar of the Brood -/
def altarOfTheBrood : Ability :=
  whenever (does (actor (a (Primitives.Predicate.and [isPermanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, Primitives.Predicate.otherThan Primitives.NounPhrase.this]))) enter)
    (instruct (each (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)) (mill (.lit 1) they))
theorem okAltarOfTheBrood : Ability.check (DRS.ofList []) altarOfTheBrood = [] := by decide
/-- Foul Emissary -/
def foulEmissaryLine : Ability :=
  triggeredWhile (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (sacrifice (thisCreature)))
    (whileDoing (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.procedure .cast ((a
      (Primitives.Predicate.and [isSpell, Primitives.Predicate.hasKeyword (.the (.named "emerge"))]))) Primitives.PlayPayment.itsOwnCost none none none)))
    (createToken (creatureToken 3 2 [] [creatureType "Eldrazi", creatureType "Horror"]))
theorem okFoulEmissaryLine : Ability.check (DRS.ofList []) foulEmissaryLine = [] := by decide

/-- Bramble Elemental -/
def brambleElemental : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bramble Elemental", cost := some [generic 3, pip .green, pip .green],
      types := [.creature], subtypes := [creatureType "Elemental"],
      text :=
        [ whenever (Primitives.GameEvent.does Primitives.Subject.any (Primitives.Action.attachment .attached (a (Primitives.Predicate.hasSubtype (enchantmentType "Aura"))) (some thisCreature)))
            (createToken (creatureToken 1 1 [.green] [creatureType "Saproling"]) (count := (.lit 2))) ],
      power := stat 4, toughness := stat 4 } }

/-- Stone Haven Outfitter -/
def stoneHavenOutfitter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stone Haven Outfitter", cost := some [generic 1, pip .white], types := [.creature],
      subtypes := [creatureType "Kor", creatureType "Artificer", creatureType "Ally"],
      text :=
        [ Primitives.Ability.static (getsPt (allOf (Primitives.Predicate.and [and [isCreature, youControl], Primitives.Predicate.isAttached (some .equipped)]))
            (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1))),
          whenever (does (actor (a (Primitives.Predicate.and [and [isCreature, youControl], Primitives.Predicate.isAttached (some .equipped)]))) die)
            (Primitives.Action.draw (.lit 1)) ],
      power := stat 2, toughness := stat 2 } }

/-- Cloud, Ex-SOLDIER -/
def cloudExSoldierAttach : Ability :=
  when (does (actor thisCreature) enter)
    (attachToIt
      (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 1))
        (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (artifactType "Equipment"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))
theorem okCloudExSoldierAttach : Ability.check (DRS.ofList []) cloudExSoldierAttach = [] := by decide
/-- Akiri, Fearless Voyager -/
def akiriEquippedAttackers : Ability :=
  whenever
    (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.attackWith (counted (atLeast 1) (Primitives.Predicate.and [and [isCreature, youControl], Primitives.Predicate.isAttached (some .equipped)])) (some (a Primitives.Predicate.anyPlayer))))
    (Primitives.Action.draw (.lit 1))
theorem okAkiriEquippedAttackers : Ability.check (DRS.ofList []) akiriEquippedAttackers = [] := by decide
/-- Vexing Bauble -/
def vexingBaubleTrigger : Ability :=
  triggeredIf (Primitives.GameEvent.does (Primitives.Subject.actor (a Primitives.Predicate.anyPlayer)) (Primitives.Action.procedure .cast ((a isSpell)) Primitives.PlayPayment.itsOwnCost none none none))
    (noManaSpentToCast it)
    (counter (that .spell))
theorem okVexingBaubleTrigger : Ability.check (DRS.ofList []) vexingBaubleTrigger = [] := by decide

/-- Void Mirror -/
def voidMirror : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Void Mirror", cost := some [generic 2], types := [.artifact],
      text :=
        [ triggeredIf (Primitives.GameEvent.does (Primitives.Subject.actor (a Primitives.Predicate.anyPlayer)) (Primitives.Action.procedure .cast ((a
          isSpell)) Primitives.PlayPayment.itsOwnCost none none none)) (noColoredManaSpentToCast it)
            (counter (that .spell)) ] } }

/-- Blood Sun -/
def bloodSun : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blood Sun", cost := some [generic 2, pip .red], types := [.enchantment],
      text :=
        [ when (does (actor thisEnchantment) enter) (Primitives.Action.draw (.lit 1)),
          Primitives.Ability.static (Primitives.StaticSpec.allAbilityLoss (allOf isLand) (some Primitives.Predicate.isManaAbility)) ] } }

/-- Promise of Bunrei -/
def promiseOfBunrei : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Promise of Bunrei", cost := some [generic 2, pip .white], types := [.enchantment],
      text :=
        [ when (does (actor (a (and [isCreature, youControl]))) die)
            (Primitives.Action.doIfDone (sacrifice thisEnchantment)
              (some (createToken (creatureToken 1 1 [] [creatureType "Spirit"]) (count := (.lit 4)))) none) ] } }

/-- Grave Peril -/
def gravePeril : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Grave Peril", cost := some [generic 1, pip .black], types := [.enchantment],
      text :=
        [ when (does (actor (a (Primitives.Predicate.and [isCreature, Primitives.Predicate.not (Primitives.Predicate.colorIs .black)]))) enter)
            (Primitives.Action.doIfDone (sacrifice thisEnchantment) (some (destroy (that (.type
                .creature))))
              none) ] } }

/-- Blood Reckoning -/
def bloodReckoning : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blood Reckoning", cost := some [generic 3, pip .black], types := [.enchantment],
      text :=
        [ whenever
            (does (actor (a isCreature)) (attack (some (youOr (a (Primitives.Predicate.and [Primitives.Predicate.hasType .planeswalker, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))))))
            (instruct (controllerOf (that (.type .creature))) (loseLife (.lit 1))) ] } }

/-- Jeskai Ascendancy's first trigger -/
def jeskaiAscendancyPump : Ability :=
  whenever (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.procedure .cast ((a (Primitives.Predicate.and
    [isSpell, Primitives.Predicate.not isCreature]))) Primitives.PlayPayment.itsOwnCost none none none))
    (Primitives.Action.sequentially
      [ get (bare (and [isCreature, youControl])) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1)) (some untilEndOfTurn),
        untap (those (.type .creature)) ])
theorem okJeskaiAscendancyPump : Ability.check (DRS.ofList []) jeskaiAscendancyPump = [] := by decide
def herosDemise : Action := destroy (target (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasSupertype .legendary]))
theorem okHerosDemise : Action.check (DRS.ofList []) herosDemise = [] := by decide

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
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [counter (target (Primitives.Predicate.abilityHead .anyActivated)), Primitives.Action.draw (.lit 1)]) ]
                } }

theorem bioplasmTwoCandidates : countOnes .object bioplasmAfterExile = 2 := by decide

/-- "From anywhere" observes Dread after its arrival in the graveyard. -/
def dread : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dread", cost := some [generic 3, pip .black, pip .black, pip .black],
      types := [.creature], subtypes := [creatureType "Elemental", creatureType "Incarnation"],
      text :=
        [ keyword (.named "fear"),
          whenever (does (actor (a isCreature)) (dealDamage (to := Primitives.NounPhrase.you))) (destroy it),
          when (does Primitives.Subject.any (putInto Primitives.NounPhrase.this graveyard (from_ := some Primitives.Location.anyLocation)))
            (instruct (Primitives.NounPhrase.possessorOf .owner it) (shuffleInto it)) ],
      power := stat 6, toughness := stat 6 } }

private semantic_template creatureDamagedByAttachedHostThisTurn (attachment : AttachWord) : NounPhrase :=
  a (.and [isCreature,
    .did ⟨.actor (.attachHost attachment (.type .creature)),
      .dealDamage .anyAmount .which .any, .duration soFarThisTurn none⟩])

/-- Vampiric Embrace. -/
def vampiricEmbrace : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vampiric Embrace", cost := some [generic 2, pip .black, pip .black],
      types := [.enchantment], subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") isCreature,
          Primitives.Ability.static (getsPt
            (Primitives.NounPhrase.attachHost .enchanted (.type .creature))
            (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 2))),
          Primitives.Ability.static (Primitives.StaticSpec.abilityGrant
            (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (keyword (.named "flying"))),
          whenever (does (actor (creatureDamagedByAttachedHostThisTurn .enchanted)) die)
            (Primitives.Action.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter)
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
          whenever (does (actor (creatureDamagedByAttachedHostThisTurn .equipped)) die)
            (Primitives.Action.sequentially
              [ returnTo (that .card) battlefield
                  [(Primitives.MoveRider.under Primitives.NounPhrase.you)],
                attachTo thisEquipment (that (.type .creature)) ]),
          keywordCosting (.named "equip") (Primitives.Cost.mana [generic 4]) ] } }

end Semantics.Cards

import Semantics.Words

/-!
# Semantics.Events

The event vocabulary a trigger watches and a lookback asks about. Port of
`idris/src/Experimental/Events.idr`, syntax only; the event names and facts the checker
classifies with live in `Check/Events`.
-/

namespace Semantics

/-- The event a lookback names ("that died this turn"); an act event names its verb. -/
inductive EventName where
  | death | departure | damageTaken | cardDrawn | entry | attackDeclaration | blockDeclaration
  | combatDamage | partBeginning | spellCast | statusChange | turnedFaceUp | phasingChange
  | blockedDeclaration | attachment | unattachment | lastCounterRemoval | lifeGain | lifeLoss
  | gameDesignation | placement | counterPlacement | counterRemoval | gameLoss | tokenCreation
  | chapterArrival | abilityActivation | statValueChange | flipWin | flipLoss
  | coinFlip | diceRoll | costPayment | costNonpayment | lifePayment | becomesTarget
  | damageDealing
  | verbedAct (verb : VerbLabel)
  | stateMatch | abilityTrigger | crimeCommission | tappedForMana
  deriving DecidableEq, Repr

/-- The verbs a deontic rule ranges over ("can't attack or block"). -/
abbrev Deeds := List VerbLabel

inductive CounterMove where
  | put | removed
  deriving DecidableEq, Repr

inductive CounterBatch where
  | one | many | last
  deriving DecidableEq, Repr

inductive DiceBatch where
  | one | many
  deriving DecidableEq, Repr

inductive PaymentOutcome where
  | paid | unpaid
  deriving DecidableEq, Repr

inductive LifeMove where
  | up | down
  deriving DecidableEq, Repr

inductive TallyOp where
  | count | sum
  deriving DecidableEq, Repr

inductive ReplUse where
  | repeatedly | nextTimeOnly
  deriving DecidableEq, Repr

/-- The word a triggered ability opens with. -/
inductive TriggerWord where
  | when | whenever | at_
  /-- The dice template's word ("After you roll a die, …"); the body reads the event's outcome,
  which the header context already carries. -/
  | after
  deriving DecidableEq, Repr

inductive DamageKind where
  | any | combatOnly | noncombatOnly
  deriving DecidableEq, Repr

inductive CondMarking where
  | asLongAs | unless_ | ifSo
  deriving DecidableEq, Repr

inductive PlayLimit where
  | onceEachYourTurn | onceEachTurn
  deriving DecidableEq, Repr

/-- When a play permission applies. -/
inductive PlayTiming where
  | whileSearchingLibrary | duringEachOfYourTurns
  deriving DecidableEq, Repr

/-- How "a …" chooses. -/
inductive ChoiceMode where
  | unmarked
  /-- "of their choice": the chooser is read in a window of the stack, so a clause's own
  subject shadows any player named before it. -/
  | theirChoice (window : Window)
  | atRandom
  | yourChoice
  deriving DecidableEq, Repr

end Semantics

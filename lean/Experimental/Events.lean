import Experimental.Words

/-!
# Experimental.Events

The event vocabulary: what a trigger watches and a lookback asks about. Port of
`idris/src/Experimental/Events.idr`, syntax only.

Not ported (checker machinery): `EventFacts` and its table, the `LookbackSubject` and
`LookbackComplement` witnesses, `Possessable`.
-/

namespace Mtg

inductive EventName where
  | death | departure | damageTaken | cardDrawn | entry | attackDeclaration | blockDeclaration
  | combatDamage | partBeginning | spellCast | statusChange | turnedFaceUp | phasingChange
  | blockedDeclaration | attachment | unattachment | lastCounterRemoval | lifeGain | lifeLoss
  | timeShift | placement | counterPlacement | counterRemoval | gameLoss | tokenCreation
  | chapterArrival | abilityActivation | statValueChange | regeneration | flipWin | flipLoss
  | coinFlip | diceRoll | costPayment | costNonpayment | lifePayment | becomesTarget
  | damageDealing
  | verbedAct (verb : VerbLabel)
  | stateMatch | abilityTrigger | crimeCommission | tappedForMana
  deriving DecidableEq, Repr

/-- The verbs a deontic rule ranges over ("can't attack or block"). -/
abbrev Deeds := List VerbLabel

inductive CounterMove where
  | put | taken
  deriving DecidableEq, Repr

inductive CounterBatch where
  | one | many | last
  deriving DecidableEq, Repr

inductive DiceBatch where
  | one | many
  deriving DecidableEq, Repr

inductive RolledDie where
  | any
  | sided (n : Nat)
  | planar
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

/-- The word a triggered ability opens with. `at` is a Lean keyword, hence `atTime`. -/
inductive TriggerWord where
  | when | whenever | atTime
  deriving DecidableEq, Repr

inductive DamageKind where
  | any | combatOnly | noncombatOnly
  deriving DecidableEq, Repr

inductive StaticKind where
  | ptDelta | keywordGrant | deedRestriction | typeAddition | controlGrant | replacement
  | prevention | conditional | entryRider | costModification | manaPersistence | ptDefinition
  | basePtSet | ptSwitch | typeSet | typeLoss | colorSet | abilityLoss | coordination
  | copyEffect | visibilityRider | outcomeImmunity | triggerMultiplier | turnSkip
  | letterDefinition
  deriving DecidableEq, Repr

inductive CondMarking where
  | asLongAs | unless | ifSo
  deriving DecidableEq, Repr

inductive PlayLimit where
  | onceEachYourTurn | onceEachTurn
  deriving DecidableEq, Repr

inductive PlayWindow where
  | whileSearchingLibrary | duringEachOfYourTurns
  deriving DecidableEq, Repr

/-- How "a …" chooses. -/
inductive ChoiceMode where
  | unmarked
  /-- "of their choice": the chooser is read in a window of the stack, so a clause's own
  subject shadows any player named before it. -/
  | theirChoice (w : Window)
  | atRandom
  | yourChoice
  deriving DecidableEq, Repr

end Mtg

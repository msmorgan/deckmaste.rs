import Semantics.Words

/-!
# Semantics.Events

Shared event parameters and the markings and modes clauses carry. Event patterns themselves
live in the recursive phrase model; their facts are checked in `Check/Events`.
-/

namespace Semantics

inductive CounterBatch where
  | one | many
  /-- Removal that leaves no counters of this kind. Emptiness is part of the trigger event,
  evaluated at trigger time [CR#310.12b,702.62a], not an intervening if rechecked on resolution
  [CR#603.4]. Suspend's separate "if it's exiled" is the intervening if [CR#702.62a]. -/
  | emptying
  deriving DecidableEq, Repr

inductive PaymentOutcome where
  | paid | unpaid
  deriving DecidableEq, Repr

/-- Whether a lookback tally counts event occurrences ("the number of times…") or sums the
quantity acted on ("the number of cards…", "the amount of life…"). -/
inductive TallyMode where
  | events
  | quantity
  deriving DecidableEq, Repr

inductive ReplUse where
  | repeatedly | nextTimeOnly
  deriving DecidableEq, Repr

inductive DamageKind where
  | any | combatOnly | noncombatOnly
  deriving DecidableEq, Repr

instance : AnyValue DamageKind where
  any := .any

/-- How a damage result combines with the other results of damage [CR#120.3d]. -/
inductive ResultMode where
  | intrinsic
  | instead (of_ : OutcomeSort)
  | inAddition
  deriving DecidableEq, Repr

/-- How a static ability's condition holds it in force: "as long as" it holds, or "if" it does.
"Unless C" is "if not C" (`onlyUnless`); whether a negated condition is said with "unless" is
read from the condition, not stored. -/
inductive CondMarking where
  | asLongAs | ifSo
  deriving DecidableEq, Repr

/-- How "a …" chooses. -/
inductive ChoiceMode where
  | unmarked
  | theirChoice
  | atRandom
  | yourChoice
  deriving DecidableEq, Repr

end Semantics

attribute [semantic_expression] Semantics.ResultMode

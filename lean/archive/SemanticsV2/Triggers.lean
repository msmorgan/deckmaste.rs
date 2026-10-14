import Semantics.Phrase

/-!
# Semantics.Triggers

Usage limits and trigger headers. `Timing` and `Duration` live in the phrase block, because a
lookback reads a span and a timing and both are built from events and conditions.
-/

namespace Semantics

inductive CountBound where
  | moreThan (amount : Amount)
  | additional (quantity : Quantity)
  deriving Repr, BEq

/-- How often something may be done: no more than `times` per `per` [CR#602.5b]. An event's
ordinal ("the second spell each turn") is not a usage limit. -/
inductive UsageLimit where
  | mk (times : CountBound) (per : Period)
  deriving Repr, BEq

/-- A second trigger header joined to the first with "or". -/
structure JoinedHeader where
  event : GameEvent
  alternatives : List GameEvent
  while_ : Option Timing
  timing : Option Timing
  deriving Repr, BEq

end Semantics

classify_semantic_syntax

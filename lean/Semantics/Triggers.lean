import Semantics.Phrase

/-!
# Semantics.Triggers

Game events, durations, and trigger headers. Port of `idris/src/Experimental/Triggers.idr`,
syntax only, reshaped: the combat events are one constructor over `CombatRelation`, the
attachment events one over `AttachMove`, a regeneration is the act event it is, and the
day/night shift is the game gaining a designation.
-/

namespace Semantics

/-- When an ability may be used or a trigger fires: "any time you could cast a sorcery",
"during your turn", "before attackers are declared". -/
inductive Timing where
  | asSorcery
  | asInstant
  | duringPart (part : TurnPart) (whose : Option NounPhrase)
  | beforePart (part : TurnPart) (whose : Option NounPhrase)
  deriving Repr, BEq

inductive Duration where
  | thisTurn
  | restOfGame
  | until_ (end_ : DurationEnd)
  | forAsLongAs (condition : Condition)
  | untilEvent (event : GameEvent)
  | duringNextTurnOf (player : NounPhrase)
  deriving Repr, BEq

inductive UsageLimit where
  | oncePerTurn | oncePerGame | actionOncePerTurn
  deriving DecidableEq, Repr

/-- "while …": a condition or an event underway. -/
inductive Concurrent where
  | whileTrue (condition : Condition)
  | whileDoing (event : GameEvent)
  deriving Repr, BEq

/-- A second trigger header joined to the first with "or". -/
structure JoinedHeader where
  event : GameEvent
  alternatives : List GameEvent
  while_ : Option Concurrent
  timing : Option Timing
  deriving Repr, BEq

end Semantics

attribute [semantic_expression] Semantics.Duration Semantics.Timing Semantics.UsageLimit

classify_semantic_syntax

import Semantics.Check.EventContext

/-!
# Semantics.Check.PhraseRules

Minimal core event checking (`GameEvent.check`, `DidClause.check`, `WouldClause.check`).
-/

namespace Semantics

def DidClause.check (drs : DRS) (c : DidClause) : List Refusal :=
  (stepDidClause drs c).2

def WouldClause.check (drs : DRS) (c : WouldClause) : List Refusal :=
  (stepWouldClause drs c).2

def GameEvent.check (drs : DRS) (ev : GameEvent) : List Refusal :=
  (stepGameEvent drs ev).2

end Semantics

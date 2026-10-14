import Semantics.Triggers
import Semantics.Check.Phrase

/-!
# Semantics.Check.EventContext

Event-derived `DRS` extension helpers (`docs/decisions/semantics-v3.md` §3.5).
-/

namespace Semantics

def DidClause.extend (drs : DRS) (c : DidClause) : DRS :=
  (stepDidClause drs c).1

def WouldClause.extend (drs : DRS) (c : WouldClause) : DRS :=
  (stepWouldClause drs c).1

def GameEvent.extend (drs : DRS) (ev : GameEvent) : DRS :=
  (stepGameEvent drs ev).1

end Semantics

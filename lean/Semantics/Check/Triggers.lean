import Semantics.Check.PhraseRules

/-!
# Semantics.Check.Triggers

Trigger list checking helpers.
-/

namespace Semantics

def GameEvent.checkAll (drs : DRS) (evs : List GameEvent) : List Refusal :=
  (stepGameEventList 64 drs evs).2

end Semantics

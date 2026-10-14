import Semantics.Abilities
import Semantics.Check.Triggers

/-!
# Semantics.Check.Abilities

Minimal core instruction, cost, static-spec, and ability stepping re-exports.
-/

namespace Semantics

def Cost.check (drs : DRS) (c : Cost) : List Refusal :=
  (stepCost drs c).2

end Semantics

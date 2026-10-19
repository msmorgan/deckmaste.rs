import Semantics.Abilities
import Semantics.Check.Phrase

/-!
# Semantics.Check.Abilities

Minimal core action, cost, static-spec, and ability stepping re-exports.
-/

namespace Semantics

def Cost.check (drs : DRS) (c : Cost) : List Refusal :=
  (stepCost 64 drs c).2

end Semantics

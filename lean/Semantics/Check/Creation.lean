import Semantics.Abilities
import Semantics.Check.Words

namespace Semantics

private def createdTokenTypes (sourceTypes : List CardType) (spec : TokenSpec)
    (fuel : Nat := 64) : List CardType :=
  match fuel, spec with
  | 0, _ => []
  | n+1, .inCaller body => createdTokenTypes sourceTypes body n
  | _+1, .written bundle => normalizeTypes bundle.characteristics.types
  | _+1, .doubleFaced front _ => normalizeTypes front.characteristics.types
  | _+1, .asThose | _+1, .copyOf _ _ => normalizeTypes sourceTypes
termination_by structural fuel

/-- The token or emblem produced by a creation instruction. For copied tokens, `sourceTypes`
comes from the selected source, rather than another referent in the surrounding context. -/
def CreationSpec.createdReferent (spec : CreationSpec) (countPlur performerPlur : Plurality)
    (sourceTypes : List CardType) : Referent :=
  let (types, zone, origin) := match spec with
    | .token ts _ => (createdTokenTypes sourceTypes ts, Zone.battlefield, some Origin.token)
    | .emblem _ _ => ([], Zone.command, none)
  { det := .a
    createdResult := true
    plur := outputPlur performerPlur countPlur
    payload := .object types (some zone) none origin none none }

end Semantics

import Semantics.Abilities
import Semantics.Check.Words

namespace Semantics

private def createdTokenTypes (sourceTypes : List CardType) : TokenSpec → List CardType
  | .written bundle => normalizeTypes bundle.characteristics.types
  | .doubleFaced front _ => normalizeTypes front.characteristics.types
  | .asThose | .copyOf _ _ => normalizeTypes sourceTypes

/-- The token or emblem produced by a creation instruction. For copied tokens, `sourceTypes`
comes from the selected source, rather than another referent in the surrounding context. -/
def CreationSpec.createdReferent (spec : CreationSpec) (countPlur performerPlur : Plurality)
    (sourceTypes : List CardType) : Referent :=
  let (types, zone, origin) := match spec with
    | .token ts _ => (createdTokenTypes sourceTypes ts, Zone.battlefield, some Origin.token)
    | .emblem _ _ => ([], Zone.command, none)
  { det := .a
    plur := outputPlur performerPlur countPlur
    payload := .object types (some zone) none origin none none }

end Semantics

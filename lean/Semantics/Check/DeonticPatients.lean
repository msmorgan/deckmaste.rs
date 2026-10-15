import Semantics.Check.CardFacts
import Semantics.Check.Words

namespace Semantics

def deonticPatientOk (before after : DRS) (action : Instruction)
    (patientRole : GramRole) : Bool :=
  match action.scopeBody with
  | .block _ =>
    let actors := (actorShape? before).map (·.refIds) |>.getD []
    let mentions := if after.currentClause == before.currentClause then
        after.pendingRoles.take (after.pendingRoles.length - before.pendingRoles.length)
      else after.pendingRoles
    !(mentions.any fun (id, role) => role == patientRole && actors.contains id)
  | _ => true

end Semantics

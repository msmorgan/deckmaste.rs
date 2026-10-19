import Semantics.Check.CardFacts
import Semantics.Check.Words
import Semantics.Check.Events

namespace Semantics

def deonticPatientOk (before after : DRS) (action : Action)
    (patientRole : GramRole) : Bool :=
  match action.scopeBody with
  | .block _ =>
    let actors := (actorShape? before).map (·.refIds) |>.getD []
    let mentions := if after.currentClause == before.currentClause then
        after.pendingRoles.take (after.pendingRoles.length - before.pendingRoles.length)
      else after.pendingRoles
    !(mentions.any fun (id, role) => role == patientRole && actors.contains id)
  | _ => true

def Action.canonicalPatient (action : Action) (fuel : Nat := 32) : Option NounPhrase :=
  match fuel, action with
  | n + 1, .inCaller body => (body.canonicalPatient n).map NounPhrase.inCaller
  | n + 1, .bracketed body => body.canonicalPatient n
  | _ + 1, .move subject .. | _ + 1, .setStatus _ subject
  | _ + 1, .turnOver subject | _ + 1, .clearDamage subject
  | _ + 1, .target subject | _ + 1, .block subject => some subject
  | _, _ => none
termination_by structural fuel

def publishProhibitionSources (before after : DRS) : DRS :=
  let sources := after.referents.filter (fun ref => ref.id ≥ before.nextId &&
    !ref.isActorFrame && !ref.isDifference &&
    (Kind.lte ref.kind (.join .object .player)) &&
    (ref.det == .target || ref.det == .the || ref.attachmentHost))
  let ids := sources.map (·.id)
  { before with
    referents := sources ++ before.referents
    nextId := max before.nextId after.nextId
    nextSelectionScope := max before.nextSelectionScope after.nextSelectionScope
    pendingRoles := after.pendingRoles.filter (fun mention => ids.contains mention.1) ++ before.pendingRoles }

end Semantics

import Semantics.Check.CardFacts
import Semantics.Check.Words
import Semantics.Check.Events

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

def Instruction.deonticDeed (action : Instruction) (fuel : Nat := 32) : Option Deed :=
  match fuel, action with
  | n + 1, .inCaller body | n + 1, .bracketed body | n + 1, .ownScope none body => body.deonticDeed n
  | _ + 1, .ownScope (some deed) _ | _ + 1, .enact deed _ _ _ => some deed
  | _ + 1, .procedure .cast .. => some (.action "cast")
  | _ + 1, .procedure .activate .. => some (.action "activate")
  | _ + 1, .procedure .play .. => some (.action "play")
  | _ + 1, .attack .. => some (.core .attack)
  | _ + 1, .block .. => some (.core .block)
  | _ + 1, .conclude .loseGame => some (.core .loseGame)
  | _ + 1, .conclude .winGame => some (.core .winGame)
  | _ + 1, .setStatus .tapped _ => some (.action "tap")
  | _ + 1, .setStatus .untapped _ => some (.action "untap")
  | _ + 1, .target _ => some (.core .target)
  | _, _ => none

def deonticReferentFits (deed : Deed) (isAgent : Bool) (ref : Referent)
    (checkZone : Bool := true) : Bool :=
  let ability := match ref.payload with | .ability .. => true | _ => false
  if deed == .core .attack && !isAgent && ref.kind == .player then true
  else
    let role := deedRoleOf deed isAgent
    let typeFits := role.types.isEmpty || deedAltOk deed isAgent ref.ty
    deedKindOk deed isAgent ref.kind && deedClassOk deed isAgent ability && typeFits &&
      zoneFits (if checkZone then ref.zone else none) (deedZoneOf deed isAgent)

def Instruction.canonicalPatient (action : Instruction) (fuel : Nat := 32) : Option NounPhrase :=
  match fuel, action with
  | n + 1, .inCaller body => (body.canonicalPatient n).map NounPhrase.inCaller
  | n + 1, .bracketed body => body.canonicalPatient n
  | _ + 1, .move subject .. | _ + 1, .setStatus _ subject
  | _ + 1, .turnOver subject | _ + 1, .clearDamage subject
  | _ + 1, .target subject | _ + 1, .block subject => some subject
  | _, _ => none
termination_by structural fuel

def Instruction.checksEnactPatient (action : Instruction) (fuel : Nat := 32) : Bool :=
  match fuel, action with
  | n + 1, .inCaller body | n + 1, .bracketed body | n + 1, .ownScope _ body =>
    body.checksEnactPatient n
  | _ + 1, .enact deed body _ _ => knownAct deed && body.canonicalPatient.isSome
  | _, _ => false
termination_by structural fuel

def deonticDeedFits (before after : DRS) (action : Instruction)
    (patientRole : GramRole) : Bool :=
  match action.deonticDeed with
  | none => true
  | some deed =>
    let actorIds := ((actorShape? before).map (·.refIds)).getD []
    let actorOk := match actorShape? before with
      | some shape =>
        deedKindOk deed true shape.kind &&
          actorIds.all (fun id => (before.findById? id).elim true (fun ref => deonticReferentFits deed true ref))
      | none => match action.scopeBody with
        | .conclude .. => false
        | _ => true
    let mentions := if after.currentClause == before.currentClause then
        after.pendingRoles.take (after.pendingRoles.length - before.pendingRoles.length)
      else after.pendingRoles
    let patientIds := (mentions.filterMap fun (id, role) =>
      if role == patientRole && !actorIds.contains id then some id else none).eraseDups
    let checkPatientZone := match action.scopeBody with
      | .procedure .. => false
      | _ => true
    actorOk && (action.checksEnactPatient || patientIds.all (fun id =>
      (after.findById? id).elim true (fun ref => deonticReferentFits deed false ref checkPatientZone)))

def publishProhibitionSources (before after : DRS) : DRS :=
  let sources := after.referents.filter (fun ref => ref.id ≥ before.nextId &&
    !ref.isActorFrame && !ref.isGap &&
    (Kind.lte ref.kind (.join .object .player)) &&
    (ref.det == .target || ref.det == .the || ref.attachmentHost))
  let ids := sources.map (·.id)
  { before with
    referents := sources ++ before.referents
    nextId := max before.nextId after.nextId
    nextSelectionScope := max before.nextSelectionScope after.nextSelectionScope
    pendingRoles := after.pendingRoles.filter (fun mention => ids.contains mention.1) ++ before.pendingRoles }

end Semantics

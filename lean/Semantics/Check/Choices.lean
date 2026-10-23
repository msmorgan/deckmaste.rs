import Semantics.Check.Words

namespace Semantics

/-- Embedded ability text retains selected values without retaining ordinary discourse. -/
def choiceContext (drs : DRS) : DRS :=
  { referents := drs.visibleReferents.filter fun ref =>
      match ref.payload with
      | .quality _ | .player true => true
      | _ => false
    nextId := drs.nextId
    currentClause := drs.currentClause }

/-- The context of text an ability puts on or grants to an object, or gives an emblem: chosen
values carry in, as in `choiceContext`, and so do the objects the ability has acted on (stamped
by a deed or a move), read back by a Participial Description only (`Referent.linked`), never by a
pronoun. Abilities on a token, an emblem or a permanent and on the source of the ability that
created it or put it onto the battlefield can be linked [CR#607.1d,607.2a], and an ability granted
to the object the granting ability is printed on counts as printed on it [CR#607.1a]. A grant onto
another object ("that creature gains '… copy the exiled card'", Massimo, the Magician) is read as
Oracle writes it. Inside a printed triggered or activated ability (`DRS.grantorLinked`), the
objects that ability acts on are read as it publishes them, those its later steps act on
included: the granted ability is linked to the whole ability that grants it, not to the steps
before the grant [CR#607.1,607.1a]. -/
def linkedTextContext (drs : DRS) : DRS :=
  let current := drs.visibleReferents.filterMap fun ref =>
    match ref.payload with
    | .quality _ | .player true => some ref
    | .object _ (some _) (some _) _ _ =>
      let acted := match drs.grantorLinked with
        | none => true
        | some grantor =>
          -- The granting ability's own objects are read as it publishes them, below; of those
          -- its trigger event named, only one it acted on.
          ref.id < grantor.since &&
            (ref.linked || (grantor.before.find? (·.id == ref.id)).all
              (·.payload.prov != ref.payload.prov))
      if ref.isActorFrame || !acted then none
      else if ref.linked then some ref
      else some { ref with linked := true, linkSite := drs.nextId, linkArms := [] }
    | _ => none
  -- The granting ability's own objects take fresh ids, at a site of their own.
  let site := drs.nextId
  let published := drs.grantorLinked.elim [] (·.published)
  let grantor := published.zipIdx.map fun (ref, index) =>
    { ref with id := site + 1 + index, linked := true, linkSite := site + 1 }
  { choiceContext drs with
    sourceless := false
    referents := grantor.reverse ++ current
    nextId := if published.isEmpty then drs.nextId else site + 1 + published.length }

private theorem chosenContextKeepsIdentityAndDropsOrdinaryMentions :
    let source := DRS.ofList
      [qualityB .color, ChoiceSort.binding .player, { det := .a, plur := .one, payload := .object [] none none none none },
       { det := .a, plur := .one, payload := .player false }]
    (choiceContext source).referents.map (·.id) = [3, 2] ∧
      (choiceContext source).nextId = source.nextId ∧
      (choiceContext (enterOwnScope source)).referents = [] := by decide

end Semantics

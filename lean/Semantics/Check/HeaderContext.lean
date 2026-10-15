import Semantics.Check.Words

namespace Semantics

def headerReferentCorresponds (left right : Referent) : Bool :=
  { left with id := 0, clause := 0, pronounResolved := false, idents := [] } ==
    { right with id := 0, clause := 0, pronounResolved := false, idents := [] }

def headerDescriptionCorresponds (left right : Referent) : Bool :=
  headerReferentCorresponds left { right with role := left.role }

def headerFreshReferents (before after : DRS) : List Referent :=
  after.visibleReferents.filter fun ref => !(before.referents.any (·.id == ref.id))

def commonHeaderContext (before : DRS) (branches : List DRS) : DRS :=
  match branches with
  | [] => before
  | [primary] => primary
  | primary :: alternatives =>
    let fresh := headerFreshReferents before primary
    let common := fresh.filterMap fun ref =>
      let uniqueHere := (fresh.filter (headerReferentCorresponds ref)).length == 1
      let correspondences := alternatives.map fun branch =>
        let branchFresh := headerFreshReferents before branch
        let sameRole := branchFresh.filter (headerReferentCorresponds ref)
        if !sameRole.isEmpty then sameRole
        else if (fresh.filter (headerDescriptionCorresponds ref)).length == 1 then
          branchFresh.filter (headerDescriptionCorresponds ref)
        else []
      if uniqueHere && correspondences.all (fun rs => rs.length == 1) then
        some { ref with idents := ref.idents.filter fun ident =>
          correspondences.all (fun rs => rs.all (fun r => r.idents.contains ident)) }
      else none
    let inherited := before.referents.filterMap fun original =>
      match primary.findById? original.id with
      | some refined =>
        if alternatives.all (fun branch => (branch.findById? original.id).isSome) then
          some (if alternatives.all (fun branch => (branch.findById? original.id).any
            (headerReferentCorresponds refined)) then refined else original)
        else none
      | none => none
    let retained := common ++ inherited
    let ids := retained.map (·.id)
    { primary with
      referents := retained
      nextId := branches.foldl (fun n branch => max n branch.nextId) before.nextId
      pendingRoles := primary.pendingRoles.filter (fun mention => ids.contains mention.1)
      resolvedIds := primary.resolvedIds.filter ids.contains
      performer := before.performer }

end Semantics

import Semantics.Check.Words

namespace Semantics

def Payload.targetGroup : Payload → Payload
  | .object types zone stamp origin _ =>
    .object types zone stamp origin none
  | .join left right => .join left.targetGroup right.targetGroup
  | payload => payload

def publishLoopTargetResults (closed : DRS) (body : DRS)
    (outerIds : List ReferentId) : DRS :=
  (body.referents.filter (fun ref => ref.announcedTargetResult &&
    !outerIds.contains ref.id)).reverse.foldl (fun state ref =>
      state.push {
        det := .target, plur := .many, payload := ref.payload.targetGroup,
        announcedTargetResult := true
      } .obj true) closed


def publishLoopCreatedResults (closed : DRS) (body : DRS)
    (outerIds : List ReferentId) : DRS :=
  (body.referents.filter (fun ref => ref.createdResult &&
    !outerIds.contains ref.id)).reverse.foldl (fun state ref =>
      state.push {
        det := .the, plur := .many, payload := ref.payload.targetGroup,
        createdResult := true
      } .obj true) closed

/-- The outcomes a kind loop's body published, each sort once and as a group, never as one
antecedent: "double the number of each kind of counter on it" puts counters, whatever the kinds,
but "that much" after a loop has no single total to read. -/
def publishLoopOutcomes (closed : DRS) (body : DRS) (outerIds : List ReferentId) : DRS :=
  let sorts := body.referents.foldl (fun sorts ref =>
    match ref.payload with
    | .outcome s => if outerIds.contains ref.id || sorts.contains s then sorts else sorts ++ [s]
    | _ => sorts) []
  sorts.foldl (fun state s => state.push { det := .the, plur := .many, payload := .outcome s }
    .other false) closed

end Semantics

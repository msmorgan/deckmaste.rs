import Semantics.Check.Words

namespace Semantics

def Payload.targetGroup : Payload → Payload
  | .object types zone stamp origin _ possessor =>
    .object types zone stamp origin none possessor
  | .join left right => .join left.targetGroup right.targetGroup
  | payload => payload

def publishLoopTargetResults (closed : DRS) (body : DRS)
    (outerIds : List ReferentId) : DRS :=
  (body.referents.filter (fun ref => ref.announcedTargetResult &&
    !outerIds.contains ref.id)).reverse.foldl (fun state ref =>
      state.push {
        det := .target, plur := .many, payload := ref.payload.targetGroup,
        counterMemoryAvailable := ref.counterMemoryAvailable,
        announcedTargetResult := true
      } .obj true) closed

end Semantics

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

private theorem chosenContextKeepsIdentityAndDropsOrdinaryMentions :
    let source := DRS.ofList
      [qualityB .color, ChoiceSort.binding .player, { det := .a, plur := .one, payload := .object [] none none none none none },
       { det := .a, plur := .one, payload := .player false }]
    (choiceContext source).referents.map (·.id) = [3, 2] ∧
      (choiceContext source).nextId = source.nextId ∧
      (choiceContext (enterOwnScope source)).referents = [] := by decide

end Semantics

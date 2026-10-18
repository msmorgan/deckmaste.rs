import Semantics.Definitions

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.ActionForms

theorem exileOmittedAgent :
    exile thisPermanent =
      Action.ownScope (some (.action "exile"))
        (.enact (.action "exile")
          (.move (.inCaller (.firstMention "exile.subject" thisPermanent))
            .anyLocation (some exileZone) .arrival none []) none none) := by rfl

theorem exileNamedAgent :
    instruct .you (exile thisPermanent) =
      Action.instruct .you
        (Action.ownScope (some (.action "exile"))
        (.enact (.action "exile")
          (.move (.inCaller (.firstMention "exile.subject" thisPermanent))
            .anyLocation (some exileZone) .arrival none []) none none)) := by rfl

theorem chooseOmittedAgent :
    choose (a isCreature) = Action.choose none (a isCreature) .openly none := by rfl

theorem chooseNamedAgent :
    instruct .you (choose (a isCreature)) =
      Action.instruct .you (Action.choose none (a isCreature) .openly none) := by rfl

theorem chooseSecretlyNamedAgent :
    instruct .you (choose (a isCreature) (disclosure := .secretly)) =
      Action.instruct .you (Action.choose none (a isCreature) .secretly none) := by rfl

theorem millNamedAgent :
    mill (.lit 3) .you =
      Action.ownScope (some (.action "mill"))
        (.enact (.action "mill")
          (.move (.slice .top (.inCaller (.lit 3))
            (libraryOf (.inCaller (.firstMention "mill.whose" .you))))
            (.zone .library none) (some graveyard) .arrival none []) none none) := by rfl

end Semantics.Proofs.ActionForms

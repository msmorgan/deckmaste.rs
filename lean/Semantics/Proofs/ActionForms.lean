import Semantics.Macros

open Semantics Semantics.Macros

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
    choose (a creature) = Action.choose none (a creature) .openly none := by rfl

theorem chooseNamedAgent :
    instruct .you (choose (a creature)) =
      Action.instruct .you (Action.choose none (a creature) .openly none) := by rfl

theorem chooseSecretlyNamedAgent :
    instruct .you (choose (a creature) (disclosure := .secretly)) =
      Action.instruct .you (Action.choose none (a creature) .secretly none) := by rfl

theorem millNamedAgent :
    mill (.lit 3) .you =
      Action.ownScope (some (.action "mill"))
        (.enact (.action "mill")
          (.move (.slice .top (.inCaller (.lit 3))
            (library (.inCaller (.firstMention "mill.whose" .you))))
            (.zone .library none) (some graveyard) .arrival none []) none none) := by rfl

end Semantics.Proofs.ActionForms

import Semantics.Macros

open Semantics Semantics.Macros

namespace Semantics.Proofs.InstructionForms

theorem exileOmittedAgent :
    exile thisPermanent =
      Instruction.ownScope (some (.action "exile"))
        (.enact (.action "exile")
          (.move (.inCaller (.firstMention "exile.subject" thisPermanent))
            .anywhere (some exileZone) .arrival none []) none none) := by rfl

theorem exileNamedAgent :
    instruct .you (exile thisPermanent) =
      Instruction.instruct .you
        (Instruction.ownScope (some (.action "exile"))
        (.enact (.action "exile")
          (.move (.inCaller (.firstMention "exile.subject" thisPermanent))
            .anywhere (some exileZone) .arrival none []) none none)) := by rfl

theorem chooseOmittedAgent :
    choose (a creature) = Instruction.choose none (a creature) .openly none := by rfl

theorem chooseNamedAgent :
    instruct .you (choose (a creature)) =
      Instruction.instruct .you (Instruction.choose none (a creature) .openly none) := by rfl

theorem chooseSecretlyNamedAgent :
    instruct .you (choose (a creature) (disclosure := .secretly)) =
      Instruction.instruct .you (Instruction.choose none (a creature) .secretly none) := by rfl

theorem millNamedAgent :
    mill (.lit 3) .you =
      Instruction.ownScope (some (.action "mill"))
        (.enact (.action "mill")
          (.move (.slice .top (.inCaller (.lit 3))
            (library (.inCaller (.firstMention "mill.whose" .you))))
            (.zones [.zone .library .bare]) (some graveyard) .arrival none []) none none) := by rfl

end Semantics.Proofs.InstructionForms

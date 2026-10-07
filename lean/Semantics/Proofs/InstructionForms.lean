import Semantics.Macros

open Semantics Semantics.Macros

namespace Semantics.Proofs.InstructionForms

theorem exileOmittedAgent :
    exile thisPermanent =
      Instruction.enact (.action "Exile") (.move thisPermanent .wherever exileZone []) := by rfl

theorem exileNamedAgent :
    act .you (exile thisPermanent) =
      Instruction.act .you
        (Instruction.enact (.action "Exile") (.move thisPermanent .wherever exileZone [])) := by rfl

theorem chooseOmittedAgent :
    choose (a creature) = Instruction.choose none (a creature) .openly none := by rfl

theorem chooseNamedAgent :
    act .you (choose (a creature)) =
      Instruction.act .you (Instruction.choose none (a creature) .openly none) := by rfl

theorem chooseSecretlyNamedAgent :
    act .you (choose (a creature) (disclosure := .secretly)) =
      Instruction.act .you (Instruction.choose none (a creature) .secretly none) := by rfl

theorem millNamedAgent :
    mill (.lit 3) .you =
      Instruction.enact (.action "Mill")
        (.move (.librarySlice .top (.lit 3) .you) (.zone .library .bare) graveyard []) := by rfl

end Semantics.Proofs.InstructionForms

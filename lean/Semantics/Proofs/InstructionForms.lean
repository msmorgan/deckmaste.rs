import Semantics.Macros

open Semantics Semantics.Macros

namespace Semantics.Proofs.InstructionForms

theorem exileOmittedAgent :
    exile thisPermanent =
      Instruction.enact (.action "Exile") (.move thisPermanent exileZone []) none := by rfl

theorem exileNamedAgent :
    exile thisPermanent (agent := NounPhrase.you) =
      Instruction.enact (.action "Exile") (.move thisPermanent exileZone []) (some .you) := by rfl

theorem chooseOmittedAgent :
    choose (a creature) = Instruction.choose none (a creature) .openly none none := by rfl

theorem chooseNamedAgent :
    choose (a creature) (agent := NounPhrase.you) =
      Instruction.choose none (a creature) .openly none (some .you) := by rfl

theorem chooseSecretlyNamedAgent :
    choose (a creature) (disclosure := .secretly) (agent := NounPhrase.you) =
      Instruction.choose none (a creature) .secretly none (some .you) := by rfl

theorem millNamedAgent :
    mill (.lit 3) .you (agent := .you) =
      Instruction.enact (.action "Mill")
        (.move (.librarySlice .top (.lit 3) .you) graveyard []) (some .you) := by rfl

end Semantics.Proofs.InstructionForms

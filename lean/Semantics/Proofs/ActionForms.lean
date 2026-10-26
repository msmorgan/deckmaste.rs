import Semantics.Definitions

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.ActionForms

/-- A player mills from their own library [CR#701.17a]: the library is the performer's, so a
handoff to a player checks and a handoff to an object, which has no library, is refused. -/
theorem millReadsThePerformersLibrary :
    Action.check (DRS.ofList []) (instruct .you (mill 3)) = [] ∧
      Action.check (DRS.ofList []) (instruct (target isCreature) (mill 3)) =
        [.kindMismatch .player .object] := by
  decide

end Semantics.Proofs.ActionForms

import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Proofs.FaceDown

Pins for the one face-down spec and for special actions (`docs/decisions/semantics-v2.md` §7,
ruling 2026-10-08, "Face-down is one spec, and a special action is a cost and an action").
A card exiled face down carries `exiledFaceDown` [CR#406.3a]; a face-down spell or permanent its
listed characteristics [CR#708.2]; a move's riders are of the kind its destination allows
(owner ruling 2026-10-08, Q28–Q29).
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.FaceDown

/-- "Exile target creature face down." -/
theorem okExileFaceDown : Action.check (DRS.ofList []) (exileFaceDown (target isCreature)) = [] := by decide

/-- "Turn target creature face down. It's a 2/2 creature." -/
theorem okTurnFaceDown :
    Action.check (DRS.ofList []) (.turnFaceDown (target isCreature) faceDownCreature) = [] := by decide

/-- Only an object is turned face down [CR#708.2]. -/
theorem badTurnAPlayerFaceDown :
    Action.check (DRS.ofList []) (.turnFaceDown (target (.isOpponentOf .you)) faceDownCreature) =
      [.kindMismatch .object .player] := by decide

/-- A special action whose action reads an anaphor with no antecedent is refused. -/
theorem badSpecialActionUnboundAnaphor :
    StaticSpec.check (DRS.ofList []) (.specialAction none (.mana [generic 1]) (.setStatus .faceUp it)) =
      [.anaphor .bare .one 0] := by decide

/-- Morph {1}{U}: cast face down for {3}; turning it face up for {1}{U} is a special action,
and its values revert as it turns [CR#702.37e,708.8]. -/
theorem okMorph : Ability.check (DRS.ofList []) (morph (.mana [generic 1, pip .blue])) = [] := by
  decide

/-- Megamorph {1}{U} [CR#702.37b]. -/
theorem okMegamorph :
    Ability.check (DRS.ofList []) (megamorph (.mana [generic 1, pip .blue])) = [] := by decide

/-- Disguise {1}{U} [CR#702.168a]. -/
theorem okDisguise :
    Ability.check (DRS.ofList []) (disguise (.mana [generic 1, pip .blue])) = [] := by decide

/-- Foretell {1}{U}: exile face down during your turn for {2}, cast on a later turn
[CR#702.143a]. -/
theorem okForetell :
    Ability.check (DRS.ofList []) (foretell (.mana [generic 1, pip .blue])) = [] := by decide

/-- The foretold card's cast permission begins as the turn it was foretold ends, so it is not
cast that turn [CR#702.143a]. -/
theorem foretoldCastOnALaterTurn :
    onALaterTurn = .between (some (.endOf .turn none .current)) none := rfl

/-- Plot {1}{U} [CR#702.170a,702.170d]. -/
theorem okPlot : Ability.check (DRS.ofList []) (plot (.mana [generic 1, pip .blue])) = [] := by
  decide

/-- Hideaway 4 [CR#702.75a]. -/
theorem okHideaway : Ability.check (DRS.ofList []) (hideaway (.lit 4)) = [] := by decide

end Semantics.Proofs.FaceDown

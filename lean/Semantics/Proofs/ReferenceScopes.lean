import Semantics.Macros

open Semantics Semantics.Macros

namespace Semantics.Proofs.ReferenceScopes

private def creatureBinding : Referent :=
  { det := .target, plur := .one,
    payload := .object [.creature] (some .battlefield) none none (some 1) none }
private def artifactBinding : Referent :=
  { det := .target, plur := .one,
    payload := .object [.artifact] (some .battlefield) none none (some 1) none }
private def playerBinding : Referent :=
  { det := .the, plur := .one, payload := .player false }
private def outer : DRS := DRS.ofList [artifactBinding, letterB .x]
private def creatureX : NounPhrase :=
  target (.and [creature, .compare [.stat .power] .eq (.letter .x)])
private def artifactX : NounPhrase :=
  target (.and [artifact, .compare [.stat .manaValue] .eq (.letter .x)])
private def powerAfter (drs : DRS) : DRS :=
  (stepInstruction 64 .postEvent .obj drs
    (.establish (.modification (target creature) .power (.up (.letter .x))) none)).1

-- P16: a true tie in grammatical role and clause remains a refusal.
theorem ordinaryItRemainsAmbiguous :
    NounPhrase.checkAt (some .object) (DRS.ofList [creatureBinding, artifactBinding]) it =
      [.anaphor .bare .one 2] := by decide

-- P17: the newer clause wins without an explicit positional window.
theorem formerDepthReachesOuterArtifact :
    NounPhrase.checkAt (some .object) (powerAfter outer) it = [] := by decide

theorem ownStatWithoutOuter :
    NounPhrase.checkAt (some .object) (powerAfter (DRS.ofList [])) it = [] := by decide

theorem ownStatWithOuter :
    NounPhrase.checkAt (some .object) (powerAfter outer) it = [] := by decide

theorem ownStatKeepsCreature :
    (firstReach .bare .one (powerAfter outer)).map (·.ty) = some [.creature] := by decide

theorem attachWithoutOuterLetter :
    Instruction.check (DRS.ofList [creatureBinding]) (attachToIt artifactX) = [] := by decide

theorem attachWithOuterLetter :
    Instruction.check (DRS.ofList [creatureBinding, playerBinding, letterB .x])
      (attachToIt artifactX) = [] := by decide

-- P15: the new attachment's clause-mate exclusion preserves the outer tie.
theorem attachKeepsOuterAmbiguity :
    Instruction.check (DRS.ofList [creatureBinding, creatureBinding, letterB .x])
      (attachToIt artifactX) = [.anaphor .bare .one 2] := by decide

theorem blockWithoutOuterLetter :
    Instruction.check (DRS.ofList [creatureBinding])
      (requireBlockIt creatureX (some untilEndOfTurn)) = [] := by decide

theorem blockWithOuterLetter :
    Instruction.check (DRS.ofList [creatureBinding, playerBinding, letterB .x])
      (requireBlockIt creatureX (some untilEndOfTurn)) = [] := by decide

theorem damageOwnPowerWithoutOuter :
    Instruction.check (DRS.ofList []) (dealDamageOwnPower creatureX (target creature)) = [] := by decide

theorem damageOwnPowerWithOuter :
    Instruction.check outer (dealDamageOwnPower creatureX (target creature)) = [] := by decide

end Semantics.Proofs.ReferenceScopes

import Semantics.Macros
import Semantics.Check.Card

open Semantics Semantics.Macros

namespace Semantics.Proofs.DrtWitnesses

private def kalitasBody : Instruction := .sequentially [
  destroy (target creature),
  create (.lit 1)
    { characteristics :=
        { colors := [.black], types := [.creature], subtypes := [creatureType "Vampire"] } },
  .establish (.ptModification it (.set (powerOf (that (.type .creature))))
    (.set (toughnessOf (that (.type .creature))))) none]


theorem kalitasReadback : Instruction.check (DRS.ofList []) kalitasBody = [] := by decide


theorem kalitasSelectsTokenThenDestroyedCreature :
    let result := (stepInstruction 64 .postEvent .obj (DRS.ofList []) kalitasBody).1
    result.resolvedIds = [1, 2] ∧
      (result.findById? 2).map (·.pronounResolved) = some true ∧
      (result.findById? 1).map (·.pronounResolved) = some false := by decide

private def destroyedCreature : DRS :=
  (stepInstruction 64 .postEvent .obj (DRS.ofList []) (destroy (target creature))).1

theorem destroyThenReadItsPower :
    Instruction.check (DRS.ofList [])
      (.sequentially [destroy (target creature), gainLife (powerOf it)]) = [] := by decide

theorem destroyedCreatureHistoricalPower :
    Amount.check destroyedCreature (powerOf (that (.type .creature))) = [] := by decide

theorem historicalReadDoesNotChangeCurrentZone :
    ((stepAmount 64 destroyedCreature (powerOf (that (.type .creature)))).1.referents.map
      (fun r => r.zone)) = [some .graveyard] := by decide

theorem historicalReadDoesNotLeakIntoFollowingAction :
    Instruction.check destroyedCreature
      (.sequentially [gainLife (powerOf (that (.type .creature))),
        .putCounters (.lit 1) (.printed p1p1Counter) (that (.type .creature)) .one true]) =
      [.anaphor (.word (.type .creature)) .one 0] := by decide

end Semantics.Proofs.DrtWitnesses

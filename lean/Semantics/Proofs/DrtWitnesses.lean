import Semantics.Macros
import Semantics.Check.Card

open Semantics Semantics.Macros

namespace Semantics.Proofs.DrtWitnesses

private def kalitasBody : Action := .sequentially [
  destroy (target creature),
  .doIf (happenedThisWay
      (.move (that (.type .creature)) (.zones [battlefield])
        (some (graveyardOf .any)) .departure none []) .any)
    (.sequentially [
      create (.lit 1)
        { characteristics :=
            { colors := [.black], types := [.creature], subtypes := [creatureType "Vampire"] } },
      .establish (.ptModification it (.set (powerOf (that (.type .creature))))
        (.set (toughnessOf (that (.type .creature))))) none]) none]


theorem kalitasReadback : Action.check (DRS.ofList []) kalitasBody = [] := by decide


theorem kalitasSelectsTokenThenDestroyedCreature :
    let result := (stepAction 64 .postEvent .obj (DRS.ofList []) kalitasBody).1
    result.resolvedIds.contains 1 = true ∧ result.resolvedIds.contains 2 = true ∧
      (result.findById? 2).map (·.pronounResolved) = some true ∧
      (result.findById? 2).map (·.payload.orig) = some (some .token) ∧
      (result.findById? 1).map (·.pronounResolved) = some false ∧
      (result.findById? 1).map (·.zone) = some (some .graveyard) := by decide

private def destroyedCreature : DRS :=
  (stepAction 64 .postEvent .obj (DRS.ofList []) (destroy (target creature))).1

theorem destroyThenReadItsPower :
    Action.check (DRS.ofList [])
      (.sequentially [destroy (target creature), gainLife (powerOf it)]) = [] := by decide

theorem destroyedCreatureHistoricalPower :
    Amount.check destroyedCreature (powerOf (that (.type .creature))) = [] := by decide

theorem historicalReadDoesNotChangeCurrentZone :
    ((stepAmount 64 destroyedCreature (powerOf (that (.type .creature)))).1.referents.map
      (fun r => r.zone)) = [some .graveyard] := by decide

theorem historicalReadDoesNotLeakIntoFollowingAction :
    Action.check destroyedCreature
      (.sequentially [gainLife (powerOf (that (.type .creature))),
        .putCounters (.lit 1) (.printed p1p1Counter) (that (.type .creature)) .one true]) =
      [.anaphor (.word (.type .creature)) .one 0] := by decide

private def brokenVisageBody : Action := .sequentially [
  destroy (target (.and [creature, .not artifact, attacking])),
  create (.lit 1)
    { characteristics :=
        { colors := [.black], types := [.creature], subtypes := [creatureType "Spirit"] } },
  .establish (.ptModification it (.set (powerOf (that (.type .creature))))
    (.set (toughnessOf (that (.type .creature))))) none]

theorem brokenVisageReadback :
    Action.check (DRS.ofList []) brokenVisageBody = [] := by decide

theorem brokenVisageSelectsTokenThenDestroyedCreature :
    let result := (stepAction 64 .postEvent .obj (DRS.ofList []) brokenVisageBody).1
    result.resolvedIds = [1, 2] ∧
      (result.findById? 2).map (·.pronounResolved) = some true ∧
      (result.findById? 1).map (·.zone) = some (some .graveyard) := by decide

end Semantics.Proofs.DrtWitnesses

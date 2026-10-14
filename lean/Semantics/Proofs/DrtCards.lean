import Semantics.Macros.Declared

open Semantics Semantics.Macros

namespace Semantics.Proofs.DrtCards

private def feralContest : Instruction := .sequentially [
  .putCounters (.lit 1) (.printed p1p1Counter) (target creatureYouControl) .one true,
  .establish
    (.must (.actor (target (.and [creature, .other]))) (.block it) { rider := .noRider })
    (some thisTurn)]

private def monstrousStep : Instruction := .sequentially [
  get (target creature) (.up (.lit 7)) (.up (.lit 7)) (some untilEndOfTurn),
  .establish
    (.must (.actor (.described (.target (upTo 1)) (.and [creature, .other])))
      (.block it) { rider := .noRider }) (some thisTurn)]

private def cloudAttach : Ability :=
  when (.does .any (putOntoBattlefield thisCreature))
    (Primitives.Instruction.attachTo
      (.described (.target (upTo 1))
        (.and [.hasSubtype (artifactType "Equipment"), .hasPossessor .controller .you])) it)

theorem feralContestUsesOrdinaryPronoun :
    Instruction.check (DRS.ofList []) feralContest = [] := by decide

theorem monstrousStepUsesOrdinaryPronoun :
    Instruction.check (DRS.ofList []) monstrousStep = [] := by decide

theorem cloudUsesOrdinaryPronoun :
    Ability.check (DRS.ofList []) cloudAttach = [] := by decide


theorem feralContestBlocksFirstCreature :
    (stepInstruction 64 .postEvent .obj (DRS.ofList []) feralContest).1.resolvedIds = [1] := by
  decide

theorem monstrousStepBlocksFirstCreature :
    (stepInstruction 64 .postEvent .obj (DRS.ofList []) monstrousStep).1.resolvedIds = [0] := by
  decide

theorem airbendSingleTargetUsesPerCardLoop :
    Instruction.check (DRS.ofList []) (Declared.airbend (target creature)) = [] := by decide

theorem airbendMultipleTargetsUsePerCardLoop :
    Instruction.check (DRS.ofList [])
      (Declared.airbend (.described (.target (upTo 2)) creature)) = [] := by decide

end Semantics.Proofs.DrtCards

import Semantics.Check
import Semantics.Check.NumberSlots
import Semantics.Definitions

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.CheckerFuel

set_option maxRecDepth 8192
set_option maxHeartbeats 1600000

private def wrap {α : Type} (ctor : α → α) : Nat → α → α
  | 0, value => value
  | n + 1, value => ctor (wrap ctor n value)

private def empty : DRS := DRS.ofList []

/-- The readers still classify the last readable depth correctly. The next wrapper is refused
by the checker, before an exhausted reader can supply its default to an accepted term. -/
theorem nounKindBudget :
    (wrap NounPhrase.inCaller 63 .you).kind? = some .player ∧
      (stepNounPhrase 64 empty .other false (wrap NounPhrase.inCaller 64 .you)).2.2 =
        [.recursionLimit] := by decide

theorem predicateKindBudget :
    (wrap Predicate.inCaller 63 .isPlayer).kind? = some .player ∧
      (stepPredicate 64 empty (wrap Predicate.inCaller 64 .isPlayer)).2 =
        [.recursionLimit] := by decide

theorem nounPluralityBudget :
    (wrap NounPhrase.inCaller 63 (.described .all .isPlayer)).plur = .many ∧
      (stepNounPhrase 64 empty .other false
        (wrap NounPhrase.inCaller 64 (.described .all .isPlayer))).2.2 =
          [.recursionLimit] := by decide

theorem amountPluralityBudget :
    (wrap Amount.inCaller 63 (.lit 1)).plur = .one ∧
      (stepAmount 64 empty (wrap Amount.inCaller 64 (.lit 1))).2 =
        [.recursionLimit] := by decide

theorem quantityPluralityBudget :
    (wrap Quantity.additional 63 .theLast).plur = .one ∧
      (stepQuantityWith (stepOps 64) empty (wrap Quantity.additional 64 .theLast)).2 =
        [.recursionLimit] := by decide

theorem eventFactsBudget :
    (wrap GameEvent.inCaller 63 (.does (.actor .you) (.draw (.exactlyOf 1)))).facts.hasMagnitude
      = true ∧
      (stepGameEvent 64 empty
        (wrap GameEvent.inCaller 64 (.does (.actor .you) (.draw (.exactlyOf 1))))).2.contains
          .recursionLimit = true := by decide

theorem actionMagnitudeBudget :
    (wrap (Action.ownScope none) 63 (.draw (.exactlyOf 1))).eventHasMagnitude = true ∧
      (stepAction 64 .postEvent .obj empty
        (wrap (Action.ownScope none) 64 (.draw (.exactlyOf 1)))).2.contains
          .recursionLimit = true := by decide

/-- A quantity, timing or location deeper than the former helper limit of eight is accepted.
The same helpers refuse at the checker's remaining depth, including a smaller supplied budget. -/
theorem helperDepthUsesCheckerBudget :
    (stepQuantityWith (stepOps 64) empty (wrap Quantity.additional 12 .theLast)).2 = [] ∧
      (stepTimingWith (stepOps 64) empty (wrap Timing.not 12 .anyTiming)).2 = [] ∧
      (stepLocationWith (stepOps 64) empty (wrap Location.not 12 .anyLocation)).2 = [] ∧
      (stepTimingWith (stepOps 64) empty (wrap Timing.not 64 .anyTiming)).2 =
        [.recursionLimit] ∧
      (stepLocationWith (stepOps 64) empty (wrap Location.not 64 .anyLocation)).2 =
        [.recursionLimit] ∧
      (stepQuantityWith (stepOps 4) empty (wrap Quantity.additional 4 .theLast)).2 =
        [.recursionLimit] := by decide

/-- Moving from a helper back into an AST stepper does not replenish the depth. -/
theorem quantityAndAmountShareDepth :
    (stepQuantityWith (stepOps 64) empty
      (wrap Quantity.additional 32 (.exactlyOf (wrap Amount.inCaller 32 1)))).2 =
        [.recursionLimit] := by decide

private def countedRider : MoveRider := .withCounters 1 (.printed p1p1Counter) .fresh

theorem riderNumberSlotsBudget :
    (wrap MoveRider.inCaller 63 countedRider).numberSlots = [(1, .clamped)] ∧
      (stepMoveRiderWith (stepOps 64) empty
        (wrap MoveRider.inCaller 64 countedRider)).2.contains .recursionLimit = true := ⟨rfl, by decide⟩

theorem replacementNumberSlotsBudget :
    (wrap Replacement.inCaller 63 (.withRiders [countedRider])).numberSlots =
      [(1, .clamped)] ∧
      (stepReplacementWith (stepOps 64) empty
        (wrap Replacement.inCaller 64 (.withRiders [countedRider])) true).2.contains
          .recursionLimit = true := ⟨rfl, by decide⟩

theorem costNumberSlotsBudget :
    (wrap Cost.inCaller 63 (.scaled .tapSymbol 1)).numberSlots = [(1, .clamped)] ∧
      (stepCost 64 empty (wrap Cost.inCaller 64 (.scaled .tapSymbol 1))).2.contains
        .recursionLimit = true := ⟨rfl, by decide⟩

theorem staticNumberSlotsBudget :
    (wrap StaticSpec.inCaller 63 (.letterDefinition .x 1)).numberSlots = [(1, .signed)] ∧
      (stepStaticSpec 64 empty
        (wrap StaticSpec.inCaller 64 (.letterDefinition .x 1))).2.contains
          .recursionLimit = true := ⟨rfl, by decide⟩

theorem actionNumberSlotsBudget :
    (wrap Action.inCaller 63 (.draw (.exactlyOf 1))).numberSlots = [(1, .clamped)] ∧
      (stepAction 64 .postEvent .obj empty
        (wrap Action.inCaller 64 (.draw (.exactlyOf 1)))).2.contains
          .recursionLimit = true := ⟨rfl, by decide⟩

-- Game events have no own number slots; their actions declare theirs. Still pin the event
-- reader's traversal boundary and the refusal before a deeper event can be accepted.
theorem eventNumberSlotsBudget :
    (wrap GameEvent.inCaller 63 .now).numberSlots = [] ∧
      (stepGameEvent 64 empty (wrap GameEvent.inCaller 64 .now)).2 =
        [.recursionLimit] := by decide

private def costFixtures : List Cost := [
  .tapSymbol, .untapSymbol, .itsManaCost, .mana [generic 1], .loyaltySymbol (.up 1),
  .scaled .tapSymbol (.arith .times 1 2), .inCaller (.scaled .tapSymbol (.arith .times 1 2)),
  .perform (.ownScope none (.draw (.exactlyOf 1))),
  .perform (.ownScope none (.inCaller (.ownScope none (.draw (.exactlyOf 1))))),
  .perform (.doForEach (.described .all .isPlayer)
    (.ownScope none (.draw (.exactlyOf 1)))),
  .compound [.tapSymbol, .inCaller (.perform (.draw (.exactlyOf 1)))],
  .or [.tapSymbol, .untapSymbol],
  .withSubstitute (.mana [generic 1]) { action := .ownScope none (.draw (.exactlyOf 1)), pays := [.generic] }
]

private def costContexts : List DRS :=
  let base := empty.push (kindBinding .player) .subj true
  let inner := enterOwnScope base
  let rementioned := { inner with pendingRoles := [(0, .obj)] }
  [base, inner, enterOwnScope rementioned]

/-- Costs return with the scope stack balanced at each tested entry depth, even when traversal
is refused. The fixtures cover every cost constructor, nested Definition/caller scopes, loops,
alternatives and compound costs. -/
theorem costsKeepScopeStackBalanced :
    [0, 1, 4, 64].all (fun fuel ↦ costContexts.all (fun drs ↦
      (costFixtures ++ [Cost.anyCost, Cost.compound [], Cost.or []]).all (fun cost ↦
        (stepCost fuel drs cost).1.scopeStack.length == drs.scopeStack.length))) = true := by decide

theorem balancedCostsStillCheck :
    costContexts.all (fun drs ↦ costFixtures.all (fun cost ↦
      (stepCost 64 drs cost).2.isEmpty)) = true := by decide

/-- Each cost-bearing payment restores every frame's caller mentions in order. It retains the
paid context's other frame fields, including the arguments the payment unmasked. -/
theorem paymentsRestoreCallerPendingFrameForFrame :
    costContexts.all (fun drs ↦ costFixtures.all (fun cost ↦
      [PlayPayment.payingInstead cost, .plusAdditional cost,
        .faceDown { characteristics := none } cost, .transformed cost].all (fun payment ↦
          let paid := stepCost 64 drs cost
          let result := stepPlayPaymentWith (stepOps 64) drs payment
          result.1.scopeStack.length == drs.scopeStack.length &&
          result.1.scopeStack.map (·.callerPending) == drs.scopeStack.map (·.callerPending) &&
          result.1.scopeStack.map (·.callerReferentIds) ==
            paid.1.scopeStack.map (·.callerReferentIds) &&
          result.2 == paid.2))) = true := by decide

theorem paymentKeepsCallerArgumentMutations :
    let before := enterOwnScope empty
    let result := stepPlayPaymentWith (stepOps 64) before
      (.payingInstead (.inCaller (.perform (tap (target isCreature)))))
    result.2 = [] ∧
      result.1.scopeStack.map (·.callerPending) = before.scopeStack.map (·.callerPending) ∧
      result.1.scopeStack.any (fun frame ↦ !frame.callerReferentIds.isEmpty) = true := by decide

end Semantics.Proofs.CheckerFuel

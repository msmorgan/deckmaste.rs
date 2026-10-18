import Semantics.Macros.Declared

namespace Semantics.Macros.Checks

open Semantics

/-- error: An Action Definition must establish its own Reference Scope -/
#guard_msgs in
semantic_definition missingReferenceScope : Action := .draw (.lit 1)

open Lean Elab Command in
run_cmd do
  let env ← getEnv
  unless Authoring.definitionAttribute.hasTag env ``Declared.airbend &&
      Authoring.definitionAttribute.hasTag env ``Declared.ripple &&
      Authoring.templateAttribute.hasTag env ``dealDamageOwnPower do
    throwError "Definition and Template declarations must retain their explicit categories"

theorem declaredAirbendOne :
    (Declared.airbend (target creature)).check (DRS.ofList []) = [] := by decide

theorem declaredAirbendMany :
    (Declared.airbend (.described (.target (upTo 2)) creature)).check (DRS.ofList []) = [] := by
  decide

theorem declaredConnive :
    (instruct (target creature) Declared.connive).check (DRS.ofList []) = [] := by decide

theorem declaredRipple : (Declared.ripple (.lit 4)).check = [] := by decide

private def callerContext : DRS :=
  nextClause (stepAction 64 .postEvent .obj (DRS.ofList []) (tap (target creature))).1

theorem keywordCostReadsCaller :
    (Declared.cycling (.perform (tap it))).check callerContext = [] := by decide

theorem keywordBodyCannotReadCaller :
    (Ability.keyword (.named "flying") []
      [.static (.cant .any (.block it) { rider := .noRider })]).check callerContext =
        [.anaphor .bare .one 0] := by decide

theorem callerPronoun :
    (Action.sequentially [tap (target creature), untap it]).check (DRS.ofList []) = [] := by
  decide

theorem definitionCannotReadUnpassedCaller :
    (Action.sequentially [tap (target creature),
      .ownScope (some (.action "tap")) (.setStatus .tapped it)]).check (DRS.ofList []) =
        [.anaphor .bare .one 0] := by decide

theorem templateKeepsSingleTarget :
    ((stepAction 64 .postEvent .obj (DRS.ofList [])
      (dealDamageOwnPower (target creature) (target creature))).1.referents.filter
        (fun r ↦ r.det == .target)).length = 2 := by decide

theorem parameterIdentitySurvivesTap :
    (Action.sequentially [tap (target creature),
      .setStatus .untapped (.laterMention "tap.subject" .any)]).check (DRS.ofList []) = [] := by
  decide

theorem movedParameterCannotBeReused :
    (Action.sequentially [exile (target creature),
      .setStatus .untapped (.laterMention "exile.subject" .any)]).check (DRS.ofList []) =
        [.bindingIdentity "exile.subject"] := by decide

theorem atomicDefinitionLaw :
    (tap (target creature)).checkDefinition (DRS.ofList []) = [] := by decide

theorem revealUpdatesActionStamp :
    (Action.sequentially [tap (target creature), revealCards it,
      tap (theVerbed (.action "reveal") .one)]).check (DRS.ofList []) = [] := by decide

theorem exhaustedTraversalRefuses :
    (stepAction 0 .postEvent .obj (DRS.ofList []) (tap (target creature))).2 =
      [.recursionLimit] := by decide

theorem exhaustedWrapperRefuses :
    (stepLocationWith (stepOps 64) (DRS.ofList []) hand 0).2 =
      [.recursionLimit] := by decide

end Semantics.Macros.Checks

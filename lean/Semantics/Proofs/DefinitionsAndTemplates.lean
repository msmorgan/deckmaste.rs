import Semantics.Check
import Semantics.Check.Card
import Semantics.Definitions

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.DefinitionsAndTemplates

/-- error: An Action Definition must establish its own Reference Scope -/
#guard_msgs in
semantic_definition missingReferenceScope : Action := .draw (.exactlyOf (.lit 1))

open Lean Elab Command in
run_cmd do
  let env ← getEnv
  unless Authoring.definitionAttribute.hasTag env ``Semantics.Definitions.airbend &&
      Authoring.definitionAttribute.hasTag env ``Semantics.Definitions.ripple &&
      Authoring.templateAttribute.hasTag env ``Semantics.Templates.dealDamageOwnPower do
    throwError "Definition and Template declarations must retain their explicit categories"

theorem okAirbendOne :
    (airbend (target isCreature)).check (DRS.ofList []) = [] := by decide

theorem okAirbendUpToTwo :
    (airbend (.described (.target (upTo 2)) isCreature)).check (DRS.ofList []) = [] := by decide

theorem okConniveOne :
    (connive (.lit 1)).check (DRS.ofList []) = [] := by decide

theorem okConniveTarget :
    (instruct (target isCreature) (connive (.lit 2))).check (DRS.ofList []) = [] := by decide

theorem okRippleFour :
    (ripple (.lit 4)).check = [] := by decide

private def callerContext : DRS :=
  nextClause (stepAction 64 .postEvent .obj (DRS.ofList []) (tap (target isCreature))).1

theorem keywordCostReadsCaller :
    (cycling (.perform (tap it))).check callerContext = [] := by decide

theorem keywordBodyCannotReadCaller :
    (Ability.keyword (.named "flying") []
      [.static (.cant .any (.block it) {})]).check callerContext =
        [.anaphor .bare .one 0] := by decide

theorem callerPronoun :
    (Action.sequentially [tap (target isCreature), untap it]).check (DRS.ofList []) = [] := by
  decide

theorem definitionCannotReadUnpassedCaller :
    (Action.sequentially [tap (target isCreature),
      .ownScope (some (.action "tap")) (.setStatus .tapped it)]).check (DRS.ofList []) =
        [.anaphor .bare .one 0] := by decide

theorem templateKeepsSingleTarget :
    ((stepAction 64 .postEvent .obj (DRS.ofList [])
      (dealDamageOwnPower (target isCreature) (target isCreature))).1.referents.filter
        (fun r ↦ r.det == .target)).length = 2 := by decide

theorem parameterIdentitySurvivesTap :
    (Action.sequentially [tap (target isCreature),
      .setStatus .untapped (.laterMention "tap.subject" .any)]).check (DRS.ofList []) = [] := by
  decide

theorem movedParameterCannotBeReused :
    (Action.sequentially [exile (target isCreature),
      .setStatus .untapped (.laterMention "exile.subject" .any)]).check (DRS.ofList []) =
        [.bindingIdentity "exile.subject"] := by decide

theorem atomicDefinitionLaw :
    (tap (target isCreature)).checkDefinition (DRS.ofList []) = [] := by decide

theorem revealUpdatesActionStamp :
    (Action.sequentially [tap (target isCreature), reveal it,
      tap (the (was (reveal which)))]).check (DRS.ofList []) = [] := by decide

theorem exhaustedTraversalRefuses :
    (stepAction 0 .postEvent .obj (DRS.ofList []) (tap (target isCreature))).2 =
      [.recursionLimit] := by decide

theorem exhaustedWrapperRefuses :
    (stepLocationWith (stepOps 64) (DRS.ofList []) hand 0).2 =
      [.recursionLimit] := by decide

end Semantics.Proofs.DefinitionsAndTemplates

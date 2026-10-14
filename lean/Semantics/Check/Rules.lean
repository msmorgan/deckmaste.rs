import Semantics.Rules
import Semantics.Check.Card

/-!
# Semantics.Check.Rules

Minimal core rules-as-data and registry-definition checking.
-/

namespace Semantics

def SbaRule.check (r : SbaRule) : List Refusal :=
  let (drs1, e1) := stepPredicate (DRS.ofList []) r.scope
  let (drs2, e2) := stepCondition drs1 r.when
  let (_, e3) := stepInstruction .postEvent .obj (nextClause drs2) r.then_
  e1 ++ e2 ++ e3

def ConferralRule.check (r : ConferralRule) : List Refusal :=
  let (drs1, e1) := stepPredicate (DRS.ofList []) r.scope
  let e2 := checkAbility drs1 r.confer
  e1 ++ e2

def Conferral.check : Conferral → List Refusal
  | .ability ab => checkAbility (DRS.ofList []) ab
  | .property spec => (stepStaticSpec (DRS.ofList []) spec).2
  | .stateBased when then_ =>
    let (drs1, e1) := stepCondition (DRS.ofList []) when
    let (_, e2) := stepInstruction .postEvent .obj (nextClause drs1) then_
    e1 ++ e2
  | .turnBased _ then_ =>
    (stepInstruction .postEvent .obj (DRS.ofList []) then_).2

def Definition.check : Definition → List Refusal
  | .counter _ k confers =>
    let hErr := if k == .object || k == .player then [] else [.definitionHolder k]
    hErr ++ confers.flatMap Conferral.check
  | .subtype _ rules => rules.flatMap Conferral.check
  | .designation label scope _ zone type half =>
    let nErr := if label.name.isEmpty then [.definitionNamed] else []
    let sErr := if scope != .heldBy .object && (zone.isSome || type.isSome || half.isSome) then
      [.definitionScoped label]
    else []
    nErr ++ sErr
  | .cardType _ _ confers => confers.flatMap Conferral.check
  | .relation label _ _ =>
    if label.name.isEmpty then [.definitionNamed] else []

def PredefinedToken.check (t : PredefinedToken) : List Refusal :=
  let nErr := if t.name.isEmpty then [.tokenNamed] else []
  nErr ++ t.token.characteristics.check (DRS.ofList [])

def RulesTables.check (t : RulesTables) : List Refusal :=
  t.sba.flatMap SbaRule.check ++
  t.conferral.flatMap ConferralRule.check ++
  t.damageResult.flatMap (fun dr => (stepStaticSpec (DRS.ofList []) (.damageResult dr)).2)

end Semantics

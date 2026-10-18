import Semantics.Rules
import Semantics.Check.Card

/-!
# Semantics.Check.Rules

Scope and registry-definition checking for rules-as-data.
-/

namespace Semantics

def Action.checkDefinition (drs : DRS) (body : Action) : List Refusal :=
  match body with
  | .ownScope (some deed) (.enact inner action becomes locus) =>
    if deed == inner && action.eventHeadOk then
      (Action.enact inner action becomes locus).check drs
    else body.check drs
  | _ => body.check drs

def SbaRule.check (r : SbaRule) : List Refusal :=
  let (drs1, e1) := stepPredicate 64 (DRS.ofList []) r.scope
  let (drs2, e2) := stepCondition 64 drs1 r.when
  let (_, e3) := stepAction 64 .postEvent .obj (nextClause drs2) r.then_
  e1 ++ refuse (Kind.lte (r.scope.kindOr .object) (.join .object .player))
    (.kindLte (r.scope.kindOr .object) (.join .object .player)) ++ e2 ++ e3

def ConferralRule.check (r : ConferralRule) : List Refusal :=
  let (drs1, e1) := stepPredicate 64 (DRS.ofList []) r.scope
  let e2 := checkAbility 64 drs1 r.confer
  e1 ++ refuse (Kind.lte (r.scope.kindOr .object) (.join .object .player))
    (.kindLte (r.scope.kindOr .object) (.join .object .player)) ++ e2

def Conferral.check : Conferral → List Refusal
  | .ability ab => checkAbility 64 (DRS.ofList []) ab
  | .property spec => (stepStaticSpec 64 (DRS.ofList []) spec).2
  | .stateBased when then_ =>
    let (drs1, e1) := stepCondition 64 (DRS.ofList []) when
    let (_, e2) := stepAction 64 .postEvent .obj (nextClause drs1) then_
    e1 ++ e2
  | .turnBased _ then_ =>
    (stepAction 64 .postEvent .obj (DRS.ofList []) then_).2

def Definition.check : Definition → List Refusal
  | .counter kind k confers =>
    let hErr := if k == .object || k == .player then [] else [.definitionHolder k]
    hErr ++ refuse (!kind.name.isEmpty) .definitionNamed ++ confers.flatMap Conferral.check
  | .subtype sub rules =>
    refuse (!sub.label.isEmpty) .definitionNamed ++ rules.flatMap Conferral.check
  | .designation label scope _ zone type half =>
    let nErr := if label.name.isEmpty then [.definitionNamed] else []
    let sErr := if scope != .heldBy .object && (zone.isSome || type.isSome || half.isSome) then
      [.definitionScoped label]
    else []
    nErr ++ sErr
  | .cardType _ _ confers => confers.flatMap Conferral.check
  | .relation label left right =>
    refuse (!label.name.isEmpty) .definitionNamed ++
      refuse (left == .object || left == .player) (.definitionHolder left) ++
      refuse (right == .object || right == .player) (.definitionHolder right)

def PredefinedToken.check (t : PredefinedToken) : List Refusal :=
  let nErr := if t.name.isEmpty then [.tokenNamed] else []
  nErr ++ t.token.characteristics.check (DRS.ofList [])

def RulesTables.check (t : RulesTables) : List Refusal :=
  t.sba.flatMap SbaRule.check ++
  t.conferral.flatMap ConferralRule.check ++
  t.damageResult.flatMap (fun dr => (stepStaticSpec 64 (DRS.ofList []) (.damageResult dr)).2)

def distinctPredefinedTokens : List PredefinedToken → Bool
  | [] => true
  | token :: rest => !(rest.map (·.name)).elem token.name && distinctPredefinedTokens rest

end Semantics

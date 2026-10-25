import Semantics.Rules
import Semantics.Check.Card

/-!
# Semantics.Check.Rules

Scope and registry-definition checking for rules-as-data.
-/

namespace Semantics

def SbaRule.check (r : SbaRule) : List Refusal :=
  let (drs1, e1) := stepPredicate 64 (DRS.ofList []) r.scope
  let (drs2, e2) := stepCondition 64 drs1 r.when
  let (_, e3) := stepAction 64 .postEvent .obj (nextClause drs2) r.then_
  e1 ++ refuse (Kind.lte (r.scope.kindOr .object) (.join .object .player))
    (.kindLte (r.scope.kindOr .object) (.join .object .player)) ++ e2 ++ e3

/-- A conferral read in `drs`: the empty context for a registry definition's, its scope's for a
`ConferralRule`'s. -/
def Conferral.checkIn (drs : DRS) : Conferral → List Refusal
  | .ability ab => checkAbility 64 drs ab
  | .property spec => (stepStaticSpec 64 drs spec).2
  | .stateBased when then_ =>
    let (drs1, e1) := stepCondition 64 drs when
    let (_, e2) := stepAction 64 .postEvent .obj (nextClause drs1) then_
    e1 ++ e2
  | .turnBased _ then_ =>
    (stepAction 64 .postEvent .obj drs then_).2

def Conferral.check (c : Conferral) : List Refusal := c.checkIn (DRS.ofList [])

/-- The context a registry definition's conferrals are read in: empty, and with no Source when
the definition's conferrals have none (`Definition.controllingDesignation?`) [CR#725.2,726.2]. -/
def Definition.conferralContext (d : Definition) : DRS :=
  { DRS.ofList [] with sourceless := d.controllingDesignation?.isSome }

def ConferralRule.check (r : ConferralRule) : List Refusal :=
  let (drs1, e1) := stepPredicate 64 (DRS.ofList []) r.scope
  let e2 := r.confer.checkIn drs1
  e1 ++ refuse (Kind.lte (r.scope.kindOr .object) (.join .object .player))
    (.kindLte (r.scope.kindOr .object) (.join .object .player)) ++ e2

def Definition.check : Definition → List Refusal
  | .counter kind confers =>
    let k := kind.holder
    let hErr := if k == .object || k == .player then [] else [.definitionHolder k]
    hErr ++ refuse (!kind.name.isEmpty) .definitionNamed ++ confers.flatMap Conferral.check
  | .subtype sub rules =>
    refuse (!sub.label.isEmpty) .definitionNamed ++ rules.flatMap Conferral.check
  | d@(.designation label confers) =>
    refuse (!label.name.isEmpty) .definitionNamed ++ confers.flatMap (·.checkIn d.conferralContext)
  | .cardType type confers =>
    refuse (!type.name.isEmpty) .definitionNamed ++ confers.flatMap Conferral.check
  | .relation label => refuse (!label.name.isEmpty) .definitionNamed

def PredefinedToken.check (t : PredefinedToken) : List Refusal :=
  let nErr := if t.name.isEmpty then [.tokenNamed] else []
  nErr ++ t.token.characteristics.check (DRS.ofList [])

def RulesTables.check (t : RulesTables) : List Refusal :=
  t.sba.flatMap SbaRule.check ++
  t.conferral.flatMap ConferralRule.check

def distinctPredefinedTokens : List PredefinedToken → Bool
  | [] => true
  | token :: rest => !(rest.map (·.name)).elem token.name && distinctPredefinedTokens rest

end Semantics

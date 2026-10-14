import Semantics.Card
import Semantics.Check.AbilityRules

/-!
# Semantics.Check.Card

Minimal core card and face checking.
-/

namespace Semantics

def Characteristics.check (drs : DRS) (c : Characteristics) : List Refusal :=
  let e1 := match c.power with | some p => p.check drs | none => []
  let e2 := match c.toughness with | some t => t.check drs | none => []
  let e3 := match c.loyalty with | some l => l.check drs | none => []
  let e4 := match c.defense with | some d => d.check drs | none => []
  let e5 := Ability.checkAll drs c.text
  e1 ++ e2 ++ e3 ++ e4 ++ e5

def CardFace.check (f : CardFace) : List Refusal :=
  f.characteristics.check (DRS.ofList [])

def Card.check : Card → List Refusal
  | .singleFaced f => f.check
  | .transforming a b | .modalDfc a b | .split a b | .adventurer a b | .flip a b =>
    a.check ++ b.check
  | .sharedLineSplit s l r =>
    s.check ++ Ability.checkAll (DRS.ofList []) l.text ++ Ability.checkAll (DRS.ofList []) r.text
  | .leveler f bands =>
    f.check ++ (bands.flatMap fun b => Ability.checkAll (DRS.ofList []) b.text)
  | .prototype f _ => f.check

end Semantics

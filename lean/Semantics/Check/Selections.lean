import Semantics.Abilities
import Semantics.Check.Words

/-!
# Semantics.Check.Selections

Count and payload metadata for ordered groups and selections from them.
-/

namespace Semantics

def Amount.exact (amount : Amount) (fuel : Nat := 64) : Option Nat :=
  match fuel, amount with
  | n + 1, .inCaller body => body.exact n
  | _ + 1, .lit (.ofNat n) => some n
  | _, _ => none
termination_by structural fuel

def Quantity.exact (quantity : Quantity) (fuel : Nat := 64) : Option Nat :=
  match fuel, quantity with
  | n + 1, .inCaller body => body.exact n
  | _ + 1, .range (some low) (some high) => if low == high then some low else none
  | n + 1, .exactlyOf amount => amount.exact n
  | _, _ => none
termination_by structural fuel

def SliceCount.exact : SliceCount → Option Nat
  | .counted quantity => quantity.exact
  | .whole | .totalling .. => none

def selectionPayload (types : List CardType) (group : Option Referent)
    (count : SliceCount) : Payload :=
  match (group.map (fun (ref : Referent) => ref.payload) : Option Payload) with
  | some (.player chosen) => .player chosen
  | _ =>
    .object (mergeTypes (group.map Referent.ty |>.getD []) types)
      (group.bind Referent.zone) none none count.exact

def slicePayload (zone : Option Zone) (amount : Amount) : Payload :=
  .object [] zone none none amount.exact

def runPayloads (zone : Option Zone) (stopTypes : List CardType) : Payload × Payload :=
  (.object [] zone none none none,
    .object stopTypes zone none none (some 1))

end Semantics

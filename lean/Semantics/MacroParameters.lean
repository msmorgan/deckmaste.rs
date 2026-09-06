import Semantics.Check

/-! Construction support for typed semantic macro parameters. These helpers are internal;
the declaration command is the authoring interface. -/

namespace Semantics

class MacroCapture (α : Type) where
  input : α → CaptureInput
  read : Nat → Nat → α → α

instance : MacroCapture NounPhrase where
  input := CaptureInput.subject
  read scope index source := match source with
    | .pro _ _ (.parameter _ _) => source
    | _ => .pro (.parameter ⟨source.kindOr .object, source.isAbility, source.isYou,
        source.selfDefinedOk, source.ascribable, twoPartiesOk source, source.opponentOnly,
        source.bareThis⟩) source.plur (.parameter scope index)

instance : MacroCapture Amount where
  input := CaptureInput.value
  read scope index source := match source with
    | .parameter _ _ _ => source
    | _ => .parameter scope index source.shape

class MacroExpansion (α : Type) where
  bind : Nat → List CaptureInput → α → α
  caller : Nat → α → α

instance : MacroExpansion Instruction where
  bind := Instruction.withBindings
  caller := Instruction.inCaller

instance : MacroExpansion Cost where
  bind := Cost.withBindings
  caller := Cost.inCaller

instance : MacroExpansion StaticSpec where
  bind := StaticSpec.withBindings
  caller := StaticSpec.inCaller

instance : MacroExpansion Amount where
  bind := Amount.withBindings
  caller := Amount.inCaller

instance : MacroExpansion Predicate where
  bind := Predicate.withBindings
  caller := Predicate.inCaller

instance : MacroExpansion Quantity where
  bind := Quantity.withBindings
  caller := Quantity.inCaller

instance : MacroExpansion ZoneExpr where
  bind := ZoneExpr.withBindings
  caller := ZoneExpr.inCaller

instance : MacroExpansion Condition where
  bind := Condition.withBindings
  caller := Condition.inCaller

instance : MacroExpansion NounPhrase where
  bind := NounPhrase.withBindings
  caller := NounPhrase.inCaller

instance : MacroExpansion GameEvent where
  bind := GameEvent.withBindings
  caller := GameEvent.inCaller

class MacroSplice (α : Type) where
  close : Nat → α → α

instance (priority := 100) {α : Type} [MacroExpansion α] : MacroSplice α where
  close := MacroExpansion.caller

instance {α β : Type} [MacroSplice β] : MacroSplice (α → β) where
  close scope body := fun argument => MacroSplice.close scope (body argument)

end Semantics

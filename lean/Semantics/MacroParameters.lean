import Semantics.Abilities

/-!
# Semantics.MacroParameters

Caller-scope wrapping (`inCaller`) for `Definition` arguments in Semantics v3
(`docs/decisions/semantics-v3.md` §1.4, §2). `withBindings`, `CaptureInput`, and
`Amount.parameter` are retired; parameter identity uses `firstMention` and `laterMention`
(`BindingId`).
-/

namespace Semantics

class MacroExpansion (α : Type) where
  caller : α → α

instance : MacroExpansion Action where
  caller := Action.inCaller

instance : MacroExpansion Cost where
  caller := Cost.inCaller

instance : MacroExpansion StaticSpec where
  caller := StaticSpec.inCaller

instance : MacroExpansion Amount where
  caller := Amount.inCaller

instance : MacroExpansion Predicate where
  caller := Predicate.inCaller

instance : MacroExpansion Quantity where
  caller := Quantity.inCaller

instance : MacroExpansion ZoneExpr where
  caller := ZoneExpr.inCaller

instance : MacroExpansion Condition where
  caller := Condition.inCaller

instance : MacroExpansion NounPhrase where
  caller := NounPhrase.inCaller

instance : MacroExpansion GameEvent where
  caller := GameEvent.inCaller

class MacroSplice (α : Type) where
  close : α → α

instance (priority := 100) {α : Type} [MacroExpansion α] : MacroSplice α where
  close := MacroExpansion.caller

instance {α β : Type} [MacroSplice β] : MacroSplice (α → β) where
  close body := fun argument => MacroSplice.close (body argument)

end Semantics

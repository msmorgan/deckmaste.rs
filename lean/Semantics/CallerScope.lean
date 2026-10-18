import Semantics.Abilities

/-!
# Semantics.CallerScope

Caller-scope wrapping (`inCaller`) for `Definition` arguments in Semantics v3
(`docs/decisions/semantics-v3.md` §1.4, §2). `withBindings`, `CaptureInput`, and
`Amount.parameter` are retired; parameter identity uses `firstMention` and `laterMention`
(`BindingId`).
-/

namespace Semantics

class CallerScope (α : Type) where
  caller : α → α

instance : CallerScope Action where
  caller := Action.inCaller

instance : CallerScope Cost where
  caller := Cost.inCaller

instance : CallerScope StaticSpec where
  caller := StaticSpec.inCaller

instance : CallerScope Amount where
  caller := Amount.inCaller

instance : CallerScope Predicate where
  caller := Predicate.inCaller

instance : CallerScope Quantity where
  caller := Quantity.inCaller

instance : CallerScope Location where
  caller := Location.inCaller

instance : CallerScope Condition where
  caller := Condition.inCaller

instance : CallerScope NounPhrase where
  caller := NounPhrase.inCaller

instance : CallerScope GameEvent where
  caller := GameEvent.inCaller

class CallerSplice (α : Type) where
  close : α → α

instance (priority := 100) {α : Type} [CallerScope α] : CallerSplice α where
  close := CallerScope.caller

instance {α β : Type} [CallerSplice β] : CallerSplice (α → β) where
  close body := fun argument => CallerSplice.close (body argument)

end Semantics

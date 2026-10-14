import Semantics.Check.PhraseRules

namespace Semantics

def GameEvent.checkAll (bs : Bindings) (evs : List GameEvent) : List Refusal :=
  evs.flatMap (GameEvent.check bs)

/-- A usage limit's count is an amount; its period names no referent. -/
def UsageLimit.check (bs : Bindings) : UsageLimit → List Refusal
  | .mk (.moreThan a) _ => Amount.check bs a
  | .mk (.additional q) _ => Quantity.check bs q

def OptUsageLimit.check (bs : Bindings) : Option UsageLimit → List Refusal
  | none => []
  | some l => UsageLimit.check bs l

def JoinedHeader.check (bs : Bindings) (j : JoinedHeader) : List Refusal :=
  GameEvent.check bs j.event ++ j.alternatives.flatMap (GameEvent.check bs) ++
    OptTiming.check (headerCtx bs j.alternatives j.event) j.while_ ++
    OptTiming.check bs j.timing

end Semantics

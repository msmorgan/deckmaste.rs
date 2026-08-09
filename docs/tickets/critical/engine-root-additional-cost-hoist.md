---
needs: [engine-bound-references]
---
**Engine: honor core's root `AdditionalCost` contract for spells and activated
abilities.** `OneShotEffect::AdditionalCost` currently executes only when the
effect resolves. Core specifies that a root node is hoisted to announcement and
paid before the spell/ability is put on the stack; only a nested node is a
resolution-time cost.

Hoist the root for both cast and activation paths, surface any cost choices at
the payment point, and carry the paid-object role into the later body through
the named binding channel. Choice-bearing costs such as
`With(ChooseOne(...), Do(Sacrifice(... That ...)))` must record the chosen
object; `cost_paid_object` must not skip it merely because `Do` is nested under
`With`. A failed or declined mandatory payment prevents announcement rather
than letting the body resolve.

Foundations witness: Ayli, Eternal Pilgrim's first activated ability. Test that
the creature is chosen and sacrificed during activation, opponents can respond
only after payment, and resolution gains life equal to that creature's
last-known toughness without a null `EventObject` panic.

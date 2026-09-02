---
needs: [core-regions-discourse]
---
**Stage 3 of [Core is explicit regions](../../decisions/core-explicit-regions.md):
costs as an announcement block.** Standard constraints apply.

The stage's capture and linked-memory halves are split out to
`core-regions-captures-and-memory`, which lands on this one — see the landing
record below.

## Scope

- Costs: `Cost` becomes a block of cost instructions with dests, run at
  announcement in the ability's own activation [CR#601.2b]; a paid product
  (the sacrificed creature, the discarded card, the exiled card) is a def
  the body reads. `Cost::ChooseAndPay { binder, body }` and the payment
  drain's frame-bound `With`/`Each` are replaced by the activation. The
  root `AdditionalCost` is hoisted to announcement for both the cast and
  activation paths, cost choices surface at the payment point, and a failed
  or declined mandatory payment prevents announcement. Verify against the
  CR at pickup whether a nested resolution-time `AdditionalCost` is
  rules-real; if it is not, the variant is deleted rather than kept as a
  `May`-with-cost twin.

## Absorbed tickets (deleted 2026-09-02; their witnesses are gates here)

- `engine-root-additional-cost-hoist`: Ayli, Eternal Pilgrim's first
  activated ability. The creature is chosen and sacrificed during
  activation, opponents respond only after payment, and resolution gains
  life equal to that creature's last-known toughness.
- `engine-bound-references`: Fling reads the sacrificed creature as a paid
  product, never through the trigger's event-object slot; a choice-bearing
  cost binds its product.

## Gates

Canon re-lowers green with the engine suites and the Idris re-emit gate.
Fixtures: the two witnesses above.

## Landing record

**The CR question the ticket delegated: a nested resolution-time
`AdditionalCost` is NOT rules-real, so the variant is deleted.**
[CR#118.8] defines an additional cost as one "that its controller must pay at
the same time they pay the spell's mana cost or the ability's activation
cost", and [CR#118.8a] scopes them to costs "applied to a spell as it's being
cast or to an ability as it's being activated", announced per [CR#601.2b].
There is no rule admitting an additional cost at any other moment. A payment
made WHILE a spell or ability resolves is [CR#118.12]'s "[do something]. If
[a player] does, [effect]" — the action there "is a cost, paid when the spell
or ability resolves" — which core already spells as `May`. So the node was
deleted outright rather than kept as a `May`-with-cost twin, and lowering
hoists the semantic `AdditionalCost` at an ability root onto that ability's
cost. A nested one is now a lowering error citing the two rules.

What landed:

- Core: `CostComponent` is an instruction set — `Act { dest, action }`,
  `Choose`, `Search`, `Let` — replacing `ChooseAndPay`; `CostBinder` and
  `Instr::AdditionalCost` are deleted; `SpellAbility` gains `cost` and
  `Mode::cost` is a plain `Cost`; `validate_announced` runs the cost block's
  definitions ahead of the body. `Reference::Bound`/`Linked` are deleted
  (their store is never built, so an unresolved name is a lowering error).
- Lowering: `peel_announcement` hoists a root `AdditionalCost`;
  `lower_cost_block` reuses `lower_binder`, so a payment binder becomes the
  decision instruction the paying verbs read; the cost's bindings are scoped
  to the block, and its last paid product becomes the region's event-object
  channel so "the sacrificed creature" resolves to the payment's register.
- Engine: `CostSummary::steps` keeps the block in announcement order and the
  payment window runs it against the announce activation; the IOU protocol
  gained per-instruction obligations and offers no obligation that reads a
  register an earlier obligation has not yet written. `StatOf`/`CounterCount`
  read last-known information once the object has left the zone the effect
  expected it in ([CR#608.2h]) even though [CR#400.7j] still lets the effect
  find it.

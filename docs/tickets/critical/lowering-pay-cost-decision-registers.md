---
needs: []
---
**A decision-bearing cost inside a BODY's `Pay` declares no register in the
enclosing region, so the card is refused at load.** Found by
`core-regions-witness-fixtures`; it is the blocker on Painful Quandary.
Standard constraints apply.

## The gap

`Unless(effect, who, unless)` expands to `May { who, effect: Pay(cost),
if_not: effect }` — the [CR#118.12a] punisher. When `cost` carries a discard
(`DiscardCards(1)`, whose body is the `Composite(Discard)` wrapping a `Choose`
decision), lowering emits the read but never declares that decision's
definition in the region it lands in, and `deckmaste_core::validate` refuses
the card:

```
region read RefId(8) is not dominated by one of its 8 definitions
```

The 8 definitions are the trigger region's parameters alone — the cost block
contributed none. It is not about triggers: the same `Unless` inside a `Spell`
ability fails the same way (`RefId(3)` of 3). Nor about costs generally: a
mana cost and a `Do(Sacrifice(…, This))` cost inside the same `Pay` both load,
and `DiscardCards(1)` in an ability's `cost:` FIELD loads. Only a
decision-bearing cost reached through `Action::Pay` is undeclared.

## Acceptance

`crates/deckmaste_engine/tests/region_witnesses.rs`'s
`painful_quandary_punishes_the_caster_it_triggered_on` loses its `#[ignore]`
and passes unchanged — both branches: refusing the cost loses the caster 5
life, paying it discards from the caster's hand instead.

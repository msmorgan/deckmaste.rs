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

## Landing record

- Construction count: decision-bearing `Action::Pay` regions validated 0 → 1;
  no construction inputs, generated artifacts, or action variants changed.
- Coverage and lock state: this ticket restores the Painful Quandary witness,
  moving `region_witnesses` from 13 passing / 6 ignored to 14 passing / 5
  ignored. After refresh incorporated another restoration from the default
  line, the merged suite is 15 passing / 4 ignored. `cr-citations.lock` and
  other lock files are unchanged.
- Assurance: restored 1
  (`painful_quandary_punishes_the_caster_it_triggered_on`); re-spelled 1
  (the witness's decision driver now declines the resolution-effect
  `Payment` prompt instead of the retired `YesNo` prompt, while its outcome
  assertions are unchanged); ignored blockers added 0; added 1
  (`pay_cost_decisions_extend_the_enclosing_definition_sequence`), plus cast
  event-object/controller assertions in an existing trigger test; removed 0.
- Positive artifacts: `cargo test -q -p deckmaste_core -p deckmaste_engine`
  passes (51 core unit tests; 738/739 engine unit tests with one pre-existing
  ignore; all integration suites green, including `region_witnesses` at
  15/19 with four pre-existing ignores); clippy passes for both crates and all
  targets with warnings denied; formatting is clean. Citation audit reads all
  three changed code sites against their inline rules references; citation
  checks report 0 non-compliant strings and 17,988 current citations with 0
  stale.
- Deviations and additions: once load validation passed, the unchanged card
  assertions exposed two runtime prerequisites: `SpellCast` now reports the
  spell controller as event actor, and a resolution-payment fork binds
  semantic `You` to the payer while preserving the original consequence
  frame. Both are required for Painful Quandary to punish and discard from
  the caster rather than the trigger controller.
- STOPs: none.

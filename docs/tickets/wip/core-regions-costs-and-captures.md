---
needs: [core-regions-discourse]
---
**Stage 3 of [Core is explicit regions](../../decisions/core-explicit-regions.md):
costs as an announcement block, captures across region boundaries, and
linked memory as declared cells.** Standard constraints apply.

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
- Captures: `Delayed`, `Reflexive`, and carried bodies (`Composite`, a
  floating replacement) declare `captures: [RefId]`; the created region's
  params carry capture provenance and are snapshotted at creation
  [CR#603.7,603.12]. `created_trigger_context`'s hand-written `this` plus
  defending-player capture is replaced; a created body starts from its own
  event roles plus its declared captures and nothing else.
- Linked memory [CR#607]: a card declares its memory cells; `Remember {
  cell, value }` writes one; a reading ability takes the cell as a param
  with linked provenance. `Reference::Bound(Ident)` and `Linked(Ident)` are
  deleted; the named-role store they waited for is never built.
- `Count::TimesPaid` and `Condition::PaidCost` read the activation's paid
  record, not a stack scan by source id, so an ETB "if it was kicked"
  recheck after the entry left the stack still reads true (the
  `engine-alt-costs` seam noted in the absorbed ticket).

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
Fixtures: the two witnesses above; an exile-then-return-at-end-step card
whose delayed trigger captures the exiled object; a `Composite` body with
a capture list; a linked pair (exile with one ability, act on the exiled
card with another) through a declared cell; a kicked permanent's ETB
recheck after the spell has left the stack.

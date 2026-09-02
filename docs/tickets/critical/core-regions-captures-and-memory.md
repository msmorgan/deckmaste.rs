---
needs: [core-regions-costs-and-captures]
---
**The capture and linked-memory halves of stage 3 of
[Core is explicit regions](../../decisions/core-explicit-regions.md).** Split
off `core-regions-costs-and-captures` on 2026-09-02 when the cost half landed
alone; the ADR's Staging section still names one stage-3 ticket, and its laws
7 and 8 are the acceptance contract here. Standard constraints apply.

## Scope

- Captures: `Delayed`, `Reflexive`, and carried bodies (`Composite`, a
  floating replacement) declare `captures: [RefId]`; the created region's
  params carry capture provenance and are snapshotted at creation
  [CR#603.7,603.12]. `created_trigger_context`'s hand-written `this` plus
  defending-player capture is replaced; a created body starts from its own
  event roles plus its declared captures and nothing else.
- Linked memory [CR#607]: a card declares its memory cells; `Remember {
  cell, value }` writes one; a reading ability takes the cell as a param
  with linked provenance. `Provenance::Linked` already exists and is
  supplied only through `enter_region_with`'s explicit channel; nothing
  writes it yet.
- `Count::TimesPaid` and `Condition::PaidCost` read the activation's paid
  record, not a stack scan by source id, so an ETB "if it was kicked"
  recheck after the entry left the stack still reads true (the
  `engine-alt-costs` seam noted in the absorbed ticket). The record lives
  with the linked-memory work because [CR#702.33e] makes "if it was kicked"
  a LINKED ability read: it must survive the one zone change from stack to
  battlefield, which the stack entry does not.

## The live defect

Captures are not merely unbuilt; they silently produce nothing. A created
region's params already carry capture provenance, but a delayed trigger's
`CreatedTrigger` has no activation to read them from, so every capture
resolves to an unavailable value at firing time. A card whose delayed
trigger reads a captured object therefore fizzles that read rather than
failing loudly, and `created_trigger_context` still hand-captures only the
source and the defending player. Whatever shape the capture list takes, a
capture that cannot be supplied must be a load-time or lowering error, never
an unavailable value at firing.

## Context from the cost half

`CostComponent::Act { dest, .. }` already captures a paid product's register,
and `activation_product` chases a moved instruction product to its new
incarnation ([CR#400.7j]) while `StatOf`/`CounterCount` fall back to the
snapshot for information reads ([CR#608.2h]). A capture list crossing a
region boundary needs the same split decided once and stated in the ADR: a
snapshot for information, a chased live id for actions.

## Gates

Canon re-lowers green with the engine suites and the Idris re-emit gate.
Fixtures: an exile-then-return-at-end-step card whose delayed trigger
captures the exiled object; a `Composite` body with a capture list; a linked
pair (exile with one ability, act on the exiled card with another) through a
declared cell; a kicked permanent's ETB recheck after the spell has left the
stack.

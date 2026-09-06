---
needs: []
---
## Deferred status — 2026-09-06

Parked as a v1 witness/lowering obligation, not a request to add new v1
syntax. Lean already has `Cost.either`, so the v2 workbench can represent a
cost alternative; that alone does not establish Perforating Artist's payment
behavior, payer scope, or lowering. Revisit against the successor lowering
path, preserving the witness and acceptance below. The original v1 diagnosis
is retained for context; it is not a claim that Lean lacks cost alternatives.

## Original v1 gap

**"Unless that player sacrifices … OR discards a card" has no spelling: a
semantic `Cost` is a conjunction.** Found by
`core-regions-witness-fixtures`; it is the blocker on Perforating Artist.
Standard constraints apply.

## The gap

`Cost` is `Arc<[CostComponent]>` — every component must be paid — and the
`Unless` macro takes exactly one `Cost`. A punisher whose escape is a CHOICE
between two costs ("each opponent loses 3 life unless that player sacrifices a
nonland permanent of their choice or discards a card") therefore cannot be
written at all. Nesting two `May(Pay(…))`s would ask two sequential yes/no
questions instead of offering one choice among three outcomes, so it is not a
faithful substitute.

A design decision is needed before an implementation: whether the disjunction
lives on `Cost` (a component that is itself a choice among costs), on the
`Unless`/`May` shape, or as a modal cost rider. Whichever it is, it must keep
[CR#118.12a]'s reading — the escape is a cost the affected player MAY pay —
and the payer must stay the loop element, not the ability's controller.

## Related

Either disjunct alone would still hit
`lowering-pay-cost-decision-registers`: both are decision-bearing costs inside
a body's `Pay`.

## Acceptance

`crates/deckmaste_engine/tests/region_witnesses.rs`'s
`perforating_artist_taxes_each_opponent_who_will_not_pay` loses its `#[ignore]`
and passes with the punisher spelled, alongside the Raid intervening-if and
the per-opponent loop already in that fixture.

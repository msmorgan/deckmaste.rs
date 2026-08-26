---
needs: [workbench-named-memory-channels]
---
# Make a paid cost readable later: the cost tag and its four consumers

Kicker, multikicker, buyback, escalate-as-a-condition, "if its madness cost was
paid" — every card whose later clause asks *whether, or how many times, an
earlier optional cost was paid*. The alternative-cost row itself is
[workbench-cost-and-payment-residues](workbench-cost-and-payment-residues.md)'s;
this ticket is only the readback channel that ticket does not own.

## Context

Source: [the v1/v2 comparison](../../memory/scratch/experimental-vs-semantics-comparison.md),
axis 19 (2026-08-24) — graded **MISSING** and "Not recorded anywhere as scope".
Scope split verified against the cost ticket: it owns the five populations the
alternative-cost static refuses, the free-cast family, the keyword cost catalog,
the cost-statement pieces, cumulative upkeep, the cost gate and the scaled
payment. It does **not** mention `CostTag`, `PaidCost`, `TimesPaid`,
`WasPaidWith`, `WasCastWith` or kicker anywhere — that is this ticket.

## From the v1 comparison (2026-08-24)

> Crate: `CostTag(Ident)` (`cost.rs:190`) with four consumers —
> `OptionalCost { components, tag, repeatable }` (`cost.rs:253`),
> `Condition::PaidCost(CostTag)` and `CastWith(CostTag)` (`condition.rs:52`),
> `Count::TimesPaid(CostTag)` (`count.rs:167`),
> `StatePredicate::WasPaidWith`/`WasCastWith` (`filter.rs:88`), plus
> `DeonticAction::Cast { cost: Option<AlternativeCost>, tag: Option<CostTag> }`
> (`deontic.rs:231`) and `AlternativeCost { Free, Components }`
> (`deontic.rs:27`). `StaticEffect::CostOption(OptionalCost)`
> (`continuous.rs:282`) is how a card declares one.
>
> Workbench: grep for `CostTag`, `WasPaidWith`, `TimesPaid`, `PaidCost`,
> `Kicker` across `Experimental.idr` and `Experimental/*.idr` returns **zero
> hits**. The only alternative-cost surface is `StaticEffect.AltCost : (c :
> Maybe (Cost bs)) -> {auto 0 ap : AltPayment c}` (`Experimental.idr:2668`) — a
> bare "you may pay this instead", untagged, so nothing downstream can read
> whether it was paid. Kicker, multikicker, buyback, escalate-as-a-condition,
> "if its madness cost was paid" — none have a v2 spelling.
> `ModalCostRider { Entwine(Cost), Escalate(Cost) }` (`ability.rs:158`) likewise
> has no counterpart.

## Why this needs the memory ruling first

`CostTag(Ident)` is a ninth `Ident`-keyed channel beside the eight in
[workbench-named-memory-channels](workbench-named-memory-channels.md), and it
runs into the same binder-contract clause 1 for the same reason: the tag is a
name minted by a cost and read by a clause that is not its child. Designing a
keyed cost tag before that ruling either pre-empts it or contradicts it. If the
ruling goes the source-anchored way, this channel is anchored to the *cost
component* that declared it and the tag is spelling; if it goes the carve-out
way, the tag is an ordinary keyed mint. Either way the shape falls out of the
ruling — hence the `needs:`.

Note that the printed English usually names the tag with the keyword itself
("if this spell was kicked", "for each time it was kicked"), which is the same
argument the memory ticket's option A rests on. Bring the measurement to that
ruling rather than re-litigating it here.

## What the round owes beyond the channel

- Whether the tag's declaration site is a static (`CostOption`'s shape) or a
  rider on the cost, and whether repeatability is a flag on the declaration or a
  property of the read (the crate puts `repeatable` on the declaration and the
  count on the read).
- Whether "cast with" and "paid with" are one relation or two — the crate has
  both a condition pair and a predicate pair.
- `ModalCostRider`'s entwine/escalate: whether a modal cost rider is this
  family or the modal clause's, decided once.
- Measure each surface before minting: an untagged optional cost with no later
  reader needs nothing from this round, and `AltCost` already writes it.

## Consumption boundary

The Idris workbench only: `idris/src/Experimental.idr` (`AltCost`,
`AltPayment`, `Cost` and its components, the condition frame, the `Amount`
reads, the description frame's predicates, the modal clause's riders),
`idris/src/Experimental/Words.idr` (the tag's vocabulary and the keyword catalog
row it usually shares), `idris/src/Experimental/Events.idr` if the read is a
lookback, the pin modules `idris/src/Experimental/Proofs*.idr`, evidence bench
`idris/src/Experimental/Cards.idr`. No Rust crate. The alternative-cost row, the
free cast and the keyword cost columns stay with
[workbench-cost-and-payment-residues](workbench-cost-and-payment-residues.md);
this round must not widen them.

## Acceptance

- The tag's shape follows the memory-channel ruling rather than restating the
  question; no second keying convention is invented here.
- A kicked spell reads its own payment back and at least one kicker card and one
  multikicker card bench, or the shortfall is named card by card.
- "Cast with" and "paid with" are settled as one relation or two, with the
  reason recorded where the read is defined.
- The declaration site is decided once and the repeatable/count split is stated
  where both halves are defined.
- The cost ticket's landed rows are unchanged; this round adds a readback and
  does not re-open the alternative-cost static.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

- **Routed from workbench-counter-distributive-residues (close, 2026-08-26):** "If life was paid" (Nahiri, the Unforgiving) reads back a CAST-TIME payment; no condition row reaches the payment channel. It is a paid-readback, so it lands here.

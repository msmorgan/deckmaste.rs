---
needs: []
---
**Announce-time modal mode choice: per-mode targets/costs and the
entwine/escalate cost riders.** `resolve/effect.rs`'s `OneShotEffect::Modal`
arm resolves modes at RESOLUTION time — it can pick target/cost-free modes
today, but two shapes are genuine announce-time features it can't reach and
currently fizzle rather than crash:

1. **Per-mode targets/costs** ([CR#601.2c,700.2c,700.2h]): a mode's targets
   are announced only for modes actually chosen, and a mode's own cost is
   part of the total locked at [CR#601.2f] — both decisions have to happen
   during casting, before the resolution-time `Modal` node ever runs. A mode
   carrying either a top-level `Targeted` wrapper or a `cost` fizzles (no
   effect) instead of choosing modes.
2. **Entwine/escalate riders** ([CR#702.42a,702.120a,601.2b]): `ChooseSpec.
   rider` is announced with the mode choice and its extra cost locked into
   the total at [CR#601.2f] — again a casting-time decision, not something a
   resolution-time modal node can retroactively charge. A `ChooseSpec` with
   `rider: Some(_)` fizzles the same way.

**Witness:** canon's `Collective Resistance` ("Escalate {G} … Choose one or
more — • Destroy target artifact. • Destroy target enchantment. • Target
creature gains hexproof and indestructible until end of turn.") hits BOTH
seams at once — every mode is `Targeted`, and `choose.rider` is
`Escalate([Mana([Green])])`. Before this ticket's fizzles landed, resolving
it panicked on a `todo!()` (a live, grammar-covered, rendering canon card);
`resolve::effect::tests::modal_with_per_mode_targets_or_rider_fizzles_without_panic`
is the regression guard.

**What building this needs:** modal mode choice has to move to the
[CR#601.2a..601.2i] announce/cast pipeline (alongside target announcement and
cost payment), so a chosen mode's targets can be announced ([CR#601.2c]) and
its per-mode cost / the rider's extra cost can be folded into the total
locked at [CR#601.2f] — before the spell/ability is even put on the stack.
That is a real feature build across casting, not a resolution-time patch:
the `PendingDecision::ChooseModes` shape, its `ChoiceContinuation::Modal`
continuation, and the `resolve/effect.rs` `Modal` arm all currently assume
mode choice happens when the effect RUNS, and would need to move to the
announce path (mirroring how targets already work) instead.

Also unbuilt, discovered alongside these two: a **copy of a modal spell
inherits the SAME chosen modes** ([CR#700.2g]) — `StackEntry` carries
`targets`/`x`/`paid_costs` from announce time but has no chosen-modes field
to copy, because mode choice isn't an announce-time decision yet either. Once
this ticket lands mode choice on the announce path, that copy-inheritance
gap should close as a natural side effect (see the seam comment on
`Copied::apply` in `step/stack.rs`).

Related: [[engine-alt-costs]] (announce-time alternative-cost selection,
same casting-pipeline family), [[core-action-riders-cost-modes]] (done — the
`ChooseSpec`/`ModalCostRider` grammar shapes this ticket's engine work rides
on), [[core-pay-player-action]] (the action-role-reshape umbrella this
seam was discovered under).

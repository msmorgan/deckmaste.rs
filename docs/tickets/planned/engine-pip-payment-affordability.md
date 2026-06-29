---
needs: [core-pip-payment]
---
**Engine: make the castability / affordability gate `PayPips`-aware so
convoke / delve / improvise can enable otherwise-unaffordable casts.** Follow-up
to `core-pip-payment`.

`core-pip-payment` landed `StaticEffect::PayPips` and the per-pip
alternative-payment hook in `pay_cost` (the cost-assembly window [CR#601.2g]):
when a spell is actually paid, eligible pips are covered by tapping a permanent /
exiling a graveyard card *rather than* paying that mana [CR#702.51a]. But the
legality gate (`can_cast` / cost-affordability in
`crates/deckmaste_engine/src/{legal.rs,cast.rs}`) is still `PayPips`-unaware: it
requires the FULL mana to be affordable before allowing the cast, so
convoke/delve/improvise cannot yet make an otherwise-uncastable spell castable —
the very point of the mechanics. (The `core-pip-payment` tests float full mana so
the cast is legal, then observe the hook shrinking the real `PayMana`.)

Scope: teach the affordability/castability check to count pips coverable by a
spell's `PayPips` statics (given currently-eligible untapped permanents /
graveyard cards) as payable, so a spell is legal to cast when mana plus available
pip-payment resources together cover the locked-in cost [CR#601.2h]. Must stay
consistent with the existing payment hook (same eligibility filters; each resource
counted once) and must NOT alter the printed cost or mana value (still payment,
not reduction).

Verdict: **improvement / completeness** (makes the landed mechanic end-to-end
playable). Effort: **S–M** — mirror the `pay_cost` pip walk in the legality
gate's affordability computation.

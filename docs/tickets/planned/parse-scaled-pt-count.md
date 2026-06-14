---
needs: []
---
The dynamic-count P/T pump grammar (gen-dynamic-count, extended by
parse-trigger-event-breadth) scales only by a UNIT base: "+1/+0 for each
<filter>" emits `AddPower(CountOf(...))`, but "+2/+0 for each <filter>" declines
because the `Count` enum has no product form (`2 × count`). Goblin Piledriver
("Whenever ~ attacks, it gets +2/+0 until end of turn for each other attacking
Goblin") is the bench-card exemplar left as `Unparsed` by
parse-trigger-event-breadth — its trigger (`ThisAttacks`) and the attacking
qualifier already parse; only the +2 base blocks it.

This needs a CORE grammar decision (out of scope for the parse-only tickets):
either a `Count` product variant (mirroring `CostChange::Scaled { change, times
}`, which already scales cost components by a `Count`), or a `Modification`
scaling wrapper. Once core gains the form, extend
`modify::parse_pt_changes_scaled` to accept a non-unit base and emit the product.
Verify with a `cargo xtask generate` delta: Goblin Piledriver graduates with the
scaled self-pump. Part of demo-goblins-elves (bench cards).

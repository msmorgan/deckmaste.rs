---
needs: []
---
## Deferred status — 2026-09-06

Parked as a successor-lowering obligation. The original gap concerns v1
execution regions; the Lean surface model does not establish how a produced
per-player group crosses those regions. This is unresolved, not answered by
v2 grammar or ordinary linked-ability memory. Preserve the Deadly Brew witness
and its required behavior below, and revisit when the lowering representation
is concrete rather than extending v1 syntax in this pass.

## Original v1 gap

**"…this way" cannot name a group an ENCLOSED region produced.** Found by
`core-regions-witness-fixtures`; it is the blocker on Deadly Brew. Standard
constraints apply.

## The gap

"Each player sacrifices a creature or planeswalker of their choice. If you
sacrificed a permanent this way, you may return another permanent card from
your graveyard to your hand." The first sentence's choice happens inside the
`Each`-over-players body — a region of its own — and a register does not
escape its region ([Core is explicit regions](../../decisions/core-explicit-regions.md)
law 1). `Noting` binds only the newest antecedent of the region it wraps, so it
cannot reach in either.

The one channel that crosses a region boundary is linked memory ([CR#607.1],
law 8), and it is the wrong shape twice over: it links two abilities printed on
one OBJECT, not two clauses of one resolution, and the group wanted here is
per-player rather than per-card.

Restructuring the card to "you sacrifice, then each opponent sacrifices" is not
a substitute — [CR#608.2e] has the choices made in APNAP order and the action
processed simultaneously, which the rewrite would break.

## Acceptance

`crates/deckmaste_engine/tests/region_witnesses.rs`'s
`deadly_brew_lets_the_caster_rebuy_after_the_table_sacrifices` loses its
`#[ignore]` and passes: each player sacrifices one of their own, and the
caster — having sacrificed one this way — returns a DIFFERENT permanent card
from their graveyard to their hand.

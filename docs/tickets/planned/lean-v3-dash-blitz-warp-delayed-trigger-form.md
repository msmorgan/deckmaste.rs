---
needs: []
---
# Dash, blitz and warp: the delayed-trigger ability without an invented event

Standard constraints apply. Lean first (`lean/Semantics/`). Status: proposed, after
[lean-v3-flash-conditional-sacrifice](../wip/lean-v3-flash-conditional-sacrifice.md).

Dash, blitz and warp each represent separate static abilities [CR#702.109a,702.152a,702.185a];
"If this spell's dash cost was paid, return the permanent this spell becomes …" is its own
ability, which is why it names the cost. So it is not a rider on the alternative cost (that would
merge abilities the CR keeps separate [CR#113.2c]).

What is wrong today (`Definitions/Keywords.lean`, `blitz`, `dash`, `warp`): that ability is written
as `.asEvent (does .any (.move thisPermanent stack battlefield .arrival [])) …` over
`thisPermanent`, an invented event. The CR text says "the permanent this spell becomes", and
[CR#608.3g] gives the timing: a static ability that functions on the stack and creates a delayed
triggered ability creates it as that permanent is put onto the battlefield.

Proposal: a form for a separate static ability that functions on the stack and creates its
delayed trigger per [CR#608.3g], reading `becomes thisSpell` (and `keywordCostPaid … thisSpell`),
with no invented event; afterwards `StaticSpec.asEvent` is only for printed "As <event>, <do>".
Keep the existing pins' claims; rewrite any that pinned the `asEvent` tree, saying why.

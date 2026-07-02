---
needs: []
---
Apply `CostModifier` statics in the cost calculation pipeline [CR#601.2f]:
affinity, reducers, and taxers. Unlocks several keyword macros that reduce or tax
casting costs.

NOTE — scope carve-out: convoke / delve / improvise are **not** cost modifiers.
They are per-pip *alternative payment* applied after the total cost locks in, not
reductions to it ([CR#702.51b]). They belong to `core-pip-payment` (planned/), not
this ticket.

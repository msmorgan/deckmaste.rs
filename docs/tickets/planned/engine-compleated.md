---
needs: [engine-planeswalkers]
---
Compleated (Phyrexian loyalty): a planeswalker with the compleated static ability enters
with fewer loyalty counters if the player who cast it paid life for Phyrexian mana symbols
in its cost — minus two loyalty counters per such symbol. Needs the planeswalker
starting-loyalty replacement (from engine-planeswalkers) plus tracking, at cast, whether
each Phyrexian mana symbol was paid with life; models a modification of the starting-loyalty
replacement, structurally analogous to the existing Phyrexian-mana cost-option machinery on
the counter axis.

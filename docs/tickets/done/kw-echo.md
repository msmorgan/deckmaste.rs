---
needs: []
---
Echo [CR#702.30] keyword macro. 50 cards currently parse to `Keyword(Echo([cost]))`
but DON'T graduate: the keyword parser emits Echo, yet no `Echo` macro backs it,
so the graduate report lists Echo as the #1 blocked-on-macro (50 cards). This is a
half-implemented keyword — parsed, not macro-backed.

Author the Echo keyword macro (Keyword-kind, mirror `plugins/builtin/macros/keyword/`
Cycling/Affinity for the shape) under `plugins/builtin/macros/keyword/Echo.ron` so
`Keyword(Echo([cost]))` expands and the cards graduate.

Echo [cost] [CR#702.30,702.30b] = a triggered ability: "At the beginning of your
upkeep, if this permanent came under your control since the beginning of your most
recent upkeep, sacrifice it unless you pay [cost]." Pieces: upkeep trigger
(AtBeginningOf your upkeep), intervening-if "came under control since last upkeep",
`Unless(Sacrifice(This), pay [cost])`. Param: the echo cost (cost/mana param, like
Cycling's `[Cost(Param(0))]`).

Verify the "came under control since last upkeep" condition is representable with
existing primitives; if it needs an unbuilt engine primitive, flag it — graduation
(parse-only) can still succeed if the macro expands to valid RON even where engine
evaluation is a seam.

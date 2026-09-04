---
needs: []
---
**Index the `Instructions` telescope by a segment stack so a body's delta is
data, not length arithmetic.** Residue of `workbench-loop-delta` (2026-09-04),
which found the ticket's `(delta, rest)` split undefinable: the tail of
`Sequentially`/`Simultaneously` is typed over `instrIntro e` with delta and
rest already concatenated, and re-splitting a later in-place mutator needs
`(X ++ d) ++ r = X ++ (d ++ r)` over abstract `d`, `r`. That round instead
promoted the assumption to a checked obligation (`Effect.KeepsOuter`): a loop
body that mutates the enclosing stack is refused. `instrDelta` and
`distributedDelta` still compute by length prefix.

Ruling needed: is the segment-indexed telescope (each instruction's context as
a stack of segments, its delta the top segment) worth a grammar-wide round?
It removes every `take`/`length` delta and lets a loop over a mutating body
typecheck soundly. Size: L. Done when: no length arithmetic recovers a delta
anywhere in `Effect.idr`; `badLoopedZoneMoveRead`'s subject typechecks as a
positive witness where the CR allows it; build at its module count.
Standard constraints apply.

Ruling 2026-09-04: parked. Trigger to reopen: a printed, vintage-legal card
refused by `Effect.KeepsOuter`/`EnactKeepsOuter` where the CR allows the
sentence. Until then the guard's refusals have all been genuine.

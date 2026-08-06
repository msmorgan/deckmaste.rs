---
needs: []
---
**`Count::ThatMany`/`ThatMuch` reads a state-global, last-write-wins register
instead of a frame-scoped binding.** The magnitude anaphor has no channel of its
own, so its resolver reads whichever amount landed most recently — the value twin
of the `newest_move_product` recency heuristic, but a distinct register and reader.

`resolve/count.rs:284-290` reads `GameState.that_much` (`state.rs:401`), a mutable
scalar re-fixed by every amount-bearing apply: `step.rs:339` (life/damage/counter
amounts), `:930` (per-card draw), `:1483` (batch size), `:1486` (coin flips),
`:1489` (dice). It is not frame state at all.

Consequence: a card fixing two magnitudes ("you gain 2 life … deal X damage …
prevent that much") reads whichever apply landed last, not the semantic antecedent;
an `Each` over players followed by a "that much" read sees only the final element's
amount.

## Fix

Bind the magnitude on the continuation frame like its siblings `allotment` /
`crossed`: the amount-fixing instruction sets `anaphora.that_much` on the frame it
continues into, so nesting scopes it for free and no global register is read. The
trigger-seeded path (`TriggerBindings.that_much`) is already correctly channeled —
only the intra-resolution register is global. See
[Effect atom independence](../../decisions/effect-atom-independence.md).

Verify: engine tests; a two-magnitude fixture reads the semantic antecedent, not
the last apply; an `Each`-over-players + "that much" reads the per-element amount.

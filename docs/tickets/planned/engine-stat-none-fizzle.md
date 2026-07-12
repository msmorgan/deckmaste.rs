---
needs: []
---
A stat read on an object without that printed stat can panic the engine — e.g.
a `Creature`-typed authored card with `toughness: None` hits the
`StatOf(Toughness)` `.expect` in the toughness-0 SBA sweep.

Ruling (2026-07-12): this violates the engine-robustness principle (authoring
mistakes never crash the engine — they fizzle). Make missing-stat reads total:
a `StatOf` on an absent printed stat should fizzle the consuming
condition/effect (or read a defined default — decide in design, consistent with
how unresolvable references no-op), never panic.

While there, audit other `.expect`/`unwrap` sites on printed characteristics
reachable from authored RON (loyalty, power, mana cost) for the same class of
crash.

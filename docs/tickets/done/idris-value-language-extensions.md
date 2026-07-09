---
needs: []
design: true
---
**Idris grammar: value-language extensions — randomness, repetition, dig-until, derived
properties, and the niche-arithmetic tail.** Net-new value/effect primitives in
`idris/src/Core.idr`; several are niche, hence `maybe/`. From the 2026-06-26 grammar census.

1. **Integer randomness.** Coin flip, die roll, planar-die — as *branchable values*, not
   object selection. `Random : Quantity -> Predicate -> Selection` only picks objects. Add
   `FlipCoin : OneShotEffect -> OneShotEffect -> OneShotEffect` and `RollDie : Nat -> Count`
   (result bound like `ThatMuch`) plus a numeric-range branch for d20 tables.
2. **Count-driven repetition.** `RepeatN : Count -> OneShotEffect -> OneShotEffect` (Storm,
   Replicate, Squad). `Each` iterates a *selection*, not an integer range.
3. **Dig-until.** `RevealUntil : Predicate b AnObject -> {from} -> Bindable` binding the
   first match and the passed-over prefix separately (Cascade, Discover, Ripple, Hideaway).
   `TopOfLibrary` is a fixed count.
4. **Derived per-object properties + aggregators.** Reuse the (now-freed) `Property` name as
   a derived per-object value — pip-count, color-count, mana value — plus `SumOver`/
   `CountDistinct` aggregators over a `Selection`. One design that (a) decomposes the bespoke
   `Devotion` primitive, (b) covers distinct-value counting (Coven, Delirium), (c) covers
   cast-time mana introspection (Converge, Sunburst, Adamant — "if {W} was spent").
5. **Proliferate.** A dedicated action that forall's over the *dynamic present-counter set*
   on a permanent/player — which the closed `CounterKind` + `Each`/`PutCounters` can't
   compose. (The closed `CounterKind` enum itself is deliberate; do not open it.)
6. **Niche arithmetic / reads tail (split a line out as card pressure warrants):**
   divide-by-N (≠2), parity (odd/even), exponentiation (Exponential Growth); `TargetCountOf`
   as a value (Strive); type-line entry count (Embiggen); top-of-graveyard ordering/adjacency
   (Death Spark, Volrath's Shapeshifter); `WasMilled` provenance; aggregate over *counters*
   (Bioessence Hydra) + cross-player min/max (Balance, Arbiter of Knollridge); atomic
   life-or-control exchange/swap (Axis of Mortality, Avarice Totem); `ThatProduced` mana
   anaphor (Vorinclex) + runtime imprinted-color production (Chrome Mox).

*Serializes with the other `idris-*` grammar tickets — they all rewrite
`idris/src/Core.idr`, so only one can be in flight at a time. `needs:` is empty because
the blocking is file-level, not logical precedence.*

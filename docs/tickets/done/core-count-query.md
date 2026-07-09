---
needs: []
design: true
---
History counting landed earlier as `Count::EventCount`/`EventSum` (matching an
`EventFilter` over the history log within a `Lookback`) — the `Count::Query(QueryKey)`
seam that once held `CardsDrawn`/`LandsPlayed`/`StormCount` is gone.

This ticket added the aggregate fold and devotion: a `Countable` domain
(`Objects | ManaSymbols`) that `CountOf`/`CountDistinct` now range over,
`Count::Aggregate(AggregateOp, Projection)` folding a per-element `Count` over
`Reference::It` (`SumOf`/`MinOf`/`MaxOf`/`AverageOf`), and `SymbolPred` grown into
the mana-symbol matcher. `Selection::Pick` shares the same `Projection` (retiring
`Extremum` for the extremal `AggregateOp`s). Devotion decomposes to
`Aggregate(SumOf, Project(<permanents you control>, CountOf(ManaSymbols(It,
CountsAs(...)))))` ([CR#700.5]); it renders "your devotion to <colors>" and
typechecks against the Idris gate.

Remaining (unforced, additive): the `Countable::{Players, Events, ManaSpent}`
sources and sunburst/converge (`CountDistinct Colors ManaSpent`) — deferred until
a card needs them.

---
needs: []
---
**Open `CounterKind` to a `Named label` constructor with a facts row, and
delete the closed enum's index tables.** Cleanroom review 2026-09-03, F1. The
counter kind is an open label the corpus keeps extending; the workbench froze
it into an enum.

`Words.CounterKind` (`Words.idr:3304–3330`) has 25 variants plus
`KeywordCounter k` (gated by `KeywordCounterEligible`, `Words.idr:2535`).
Counter kinds with no variant, motivating the opening: energy, oil, quest,
finality, fade, spore, depletion, level, verse, ki, divinity, soul, page,
fuse, flood, bounty, tide, growth, doom, study, strike, dream, brick, plague,
slime, fungus, egg, wish, stash, scream. Thallid (VINTAGE): "At the beginning
of your upkeep, put a spore counter on this creature." Probe:

```
p = PutCounters (Lit 1) (PrintedKind Spore) Macros.thisCreature
Error: Undefined name Spore.          (the twin with Charge typechecks)
```

`Energy`, `Fade`, `Oil`, `Defense` refuse the same way (Aether Chaser,
Blastoderm, Archfiend of the Dross, Portent Tracker). The closed enum
contradicts the open-label ruling that `ActFacts`/`KeywordLabel` already
follow, `subtypes-are-labels`, and the builtin-v2 ADR's counter-kind
declarations.

## Fix

`CounterKind = BoostCounter d d | KeywordCounter k | Named (label : String)`,
with a `counterFacts` row table for the kinds that carry rules data —
`counterScope` (`Words.idr:3343–3345`: poison, rad, experience and energy are
player-held) becomes a column. `counterIx`, `Eq CounterKind` and
`counterScope` (`Words.idr:3333–3392`) are replaced by the row table.

Fold the `CounterKindSource.PrintedKind` consumers in the same pass so
`RemoveCountersAmong` / `MoveCounters` / `LosesCounters` (`Effect.idr:1214–
1232`) stop taking `Maybe CounterKind` while `PutCounters` / `RemoveCounters`
take `Maybe (CounterKindSource bs)`; `namedKind` (`Effect.idr:107`) exists
only to bridge the two and goes with it.

Bench migration: 118 `PrintedKind` sites, 85 of them through
`Macros.plusOnePlusOne` / `minusOneMinusOne` / `flyingCounter`, so ~33 sites
change spelling. Chromatic Armor's two sleight-counter lines
(`Cards.idr:5531`) are blocked on this ticket and should land with it.

Size: M.

Done when: the build is 23/23 with 0 errors and 0 warnings; `CounterKind` is
three constructors and the label form is open; `counterIx`, `Eq CounterKind`
and the standalone `counterScope` table are gone, replaced by one
`counterFacts` row per rules-bearing kind; Thallid's spore counter and at
least one each of energy / oil / fade are typechecking bench witnesses;
`namedKind` is gone and the counter-kind slot has one type across
`PutCounters`/`RemoveCounters`/`MoveCounters`/`RemoveCountersAmong`; every
existing counter pin still refutes for its named reason. Standard constraints
apply.

---
needs: []
---
**Core: `TriggerMultiplier` static (Panharmonicon / Yarok / trigger doublers).**
From the 2026-06-28 idris↔rust model audit.

Today there is no data form for "matching triggered abilities trigger an
additional time" (`StaticEffect`, `crates/deckmaste_core/src/continuous.rs`).
Panharmonicon, Yarok, and the trigger half of Doubling Season have no
representation.

Idris `StaticEffect.TriggerMultiplier (cause : EventQuery) (extra : Count)
{affected}` (`idris/src/Core.idr`): the affected triggers fire `extra` additional
times. It is **not** a copy — each instance chooses its own modes/targets — and
multipliers **add** rather than compound (two Panharmonicons → 3×, not 4×).
`affected` filters the affected ability's source permanent (default "you
control"; override for opponent/any doublers).

Adoption: add a `TriggerMultiplier` static (a cause event-query, an extra count,
and an affected-source filter).

Verdict: **improvement** (a recurring real card family with a clean additive
spec). Effort: **M–L** (engine-heavy: the trigger-firing path must consult it).
Related: NONE.

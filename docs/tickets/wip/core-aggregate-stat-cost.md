---
needs: []
---
**Core: `TapTotal` — an aggregate-stat cost (Crew, and the convoke/devotion-scaling
cost family).** From the 2026-06-28 idris↔rust model audit.

Today `CostComponent` (`crates/deckmaste_core/src/cost.rs`) has no "tap a chosen
subset whose summed [stat] satisfies [cmp] [N]" form, so Crew ("tap any number
of creatures with total power N or greater") has no cost shape.

Idris `Cost.TapTotal (c : Characteristic) Cmp (Count) (of_ : Predicate)`
(`idris/src/Core.idr`) is one shape for Crew and the convoke/devotion-scaling
cost family the engine authors flagged it should subsume. The `of_` is an open
filter, so it stays plugin-safe.

Adoption: add a `TapTotal` cost component (stat axis + comparator + count +
filter).

Verdict: **improvement** (Crew has no representation today). Effort: **M**.
Related: NONE.

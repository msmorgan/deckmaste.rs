---
needs: []
---
**Core: value-language parity — basic `Count` arithmetic, `CountDistinct`, and
extremal-element `Pick` selection.** From the 2026-06-28 idris↔rust model audit.

Today `Count` (`crates/deckmaste_core/src/count.rs`) has `Min` and a few readers
but no general arithmetic, no distinct-union count, and there is no way to select
the *extremal element* of a set.

Idris (`idris/src/Core.idr`) provides:
- **`Count` arithmetic:** `Plus / Minus / Times / Half RoundMode / Max` ("half
  its power rounded up", "twice X", "X plus N").
- **`CountDistinct Characteristic Countable`** — the size of the distinct union
  of a characteristic across a set (Domain = distinct land subtypes; Coven;
  Sunburst over colors of mana spent; Tarmogoyf's distinct card types in
  graveyards).
- **`Selection.Pick op Projection`** — the extremal *element* (the creature with
  the greatest/least power), gated to the extremal ops; the element-twin of the
  aggregate fold. (`Only` = the unique element of a filter is the degenerate
  case.)

Adoption: add the arithmetic constructors, `CountDistinct`, and an extremal
`Pick` selection.

Verdict: **improvement** (basic arithmetic and distinct-counting are real,
widespread gaps). Effort: **M**.

Related — **overlaps/extends `core-count-query` (maybe/)**, which owns the
aggregate-fold + devotion-style sums (an `Aggregate`/`Projection` over object and
mana-symbol sources). Recommend landing this and that together: the aggregate
fold and `Pick` share the same `Projection` machinery, and the mana-symbol source
(devotion/converge/sunburst) belongs with the fold. `idris-value-language-extensions`
(maybe/) covers only the *niche* arithmetic tail (divide-by-N, parity,
exponentiation) on the Idris side — basic arithmetic + distinct-count are not in
it.

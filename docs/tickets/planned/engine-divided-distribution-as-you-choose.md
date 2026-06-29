---
needs: []
---
**Engine: divided distribution (`DivideAmong` / `split_evenly`) is rules-illegal
and not player-chosen.** Found in the 2026-06-29 code review.

`split_evenly` (`crates/deckmaste_engine/src/resolve.rs:453`) splits the total
evenly: when chosen targets > total, some recipients get a **0** share (divided
damage/counters must put at least one on each recipient); when the group resolves
to 0 elements the whole total is silently dropped. The division is also fixed
(even), not the "divided as you choose" player decision the rules require.

Fix: bound the target choice so each gets ≥1 (divide cardinality and amount
together), and surface the as-you-choose split as a payment/announcement-time
decision (the documented seam). Add tests for total < targets and a chosen
(uneven) split. Pairs with `engine-divide-among-player-panic` (same `DivideAmong`
path) and the Arc Lightning data fix.

Severity: **medium** (rules-incorrect distribution). Effort: **M**.

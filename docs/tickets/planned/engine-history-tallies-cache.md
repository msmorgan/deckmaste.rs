---
needs: [engine-history-tallies]
---
Optimization: reconstruct O(1) cached counters for hot history queries
(lands/draws/storm) on top of the `engine-history-tallies` log, which stays the
source of truth. Deferred from that slice (the old `Tally` struct was deleted
rather than keep dual bookkeeping).

## Deferred — optimization, comes after naive completion (2026-06-14)
Project rule: all optimizations come after every engine function is implemented
naively. This is a perf cache, so it waits until the engine is functionally
complete — don't claim it before then, regardless of its dep being done.

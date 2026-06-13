---
needs: [engine-history-tallies]
---
Optimization: reconstruct O(1) cached counters for hot history queries
(lands/draws/storm) on top of the `engine-history-tallies` log, which stays the
source of truth. Deferred from that slice (the old `Tally` struct was deleted
rather than keep dual bookkeeping).

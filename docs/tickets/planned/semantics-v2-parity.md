---
needs: [lowering-v2]
---
**The cutover finish line (updated 2026-10-14 per
[`docs/decisions/semantics-v3.md`](../../decisions/semantics-v3.md)).** A
harness that lowers each canon card through both v1 and v3
(`deckmaste_semantics_v3` / `deckmaste_lowering_v3`) and compares the core
terms. v1 is a starting point, not an oracle: not everything in canon is known
to lower correctly, so each disagreement is adjudicated by hand and the verdict
recorded. Once the per-card ratchet is clean, the TUI whole-game slow tests run
on v3 and must match. Standard constraints apply.

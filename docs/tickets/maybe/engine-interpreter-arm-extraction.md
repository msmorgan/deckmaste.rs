---
needs: [ci-cite-gate]
---
**[design] Consider extracting the largest coherent arms from the engine
effect interpreter without changing dispatch or behavior.**
`resolve/effect.rs` contains a roughly 1,000-line `run_effect` match; the
`Each`, `May`, and `With` arms appear large enough to hide their individual
contracts while remaining natural semantic units.

This reopens a prior refactor boundary. `refactor-oversized-fns` deliberately
kept `run_effect` as a flat dispatcher, and `split-resolve-tests` records the
effect logic as reasonably sized. Before implementation, confirm that the
proposed helpers improve local reasoning rather than merely satisfying a line
metric. The central match and one-arm-per-variant dispatch must remain flat;
traits, a second dispatch table, or subsystem scattering are out of scope.

If approved, move only the three arm bodies into named same-module helpers,
preserving control-flow/return behavior exactly and carrying their citations
and invariant comments with them. Gates: engine test-count parity, focused
tests for each extracted arm, workspace clippy/fmt, citation checking, and a
comment/citation multiset audit. No English-crate refactor is in scope.

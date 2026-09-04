---
needs: []
---
# Cap english_v2 corpus workers from the environment

`cargo xtask english_v2 {coverage,ambiguity,roundtrip,parse}` default
`--workers` to `available_parallelism()` (24 on the dev host,
`crates/xtask/src/english_v2.rs`, `default_corpus_workers`). With several
agents running gates concurrently that is ~150 parser threads plus 24-job
cargo builds per agent, and the host stalls.

Add an environment override read by `default_corpus_workers`
(`DECKMASTE_XTASK_WORKERS`, positive integer; invalid values are an error
naming the variable, never a silent fallback). An explicit `--workers` still
wins. The `PERFORMANCE english-v2` telemetry line already prints the worker
count it ran with; keep that so a capped run is visible at review. The
quiet-host ceiling stays defined at full parallelism: when the effective
worker count is below `available_parallelism()`, the ceiling comparison
prints `criterion=capped_workers` instead of `quiet_host` and never emits the
regression warning.

Consumption boundary: `crates/xtask` only. Tests: the override applied, the
flag winning over it, the invalid-value error, the capped criterion label.
Standard constraints apply.

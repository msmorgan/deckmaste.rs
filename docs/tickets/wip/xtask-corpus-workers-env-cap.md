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

## Landing record

Measured on change `unnoksps` with 16,771 covered lock identities. Before and
after are the same grammar and corpus tree: 16,771 selected and covered units,
0 selected-uncovered units, 15,870 parse failures, 0 unresolved ties, and 0
internal, round-trip, ownership, traversal, leaf-traversal, gap, overlap,
synthetic, or provenance-plan failures. The selection census remains 11,515
unique and 5,256 specificity-resolved selections; exception-resolved selections
and exception uses remain 0. Construction declarations remain 397. No identity
is newly covered, so there is no new selected analysis to list.

The coverage lock remains byte-unchanged: 16,771 identities, 49,421 lines, and
SHA-256 `2cf7f9b716e13b27d626c60638d1266ca6cdc083bbcca87cb1435b4b00cd65ca`.
The coverage check reported 26 permitted and 5 forbidden licensing checkers.

Positive gates: `cargo fmt --all`; `cargo clippy -p xtask --all-targets -- -D
warnings`; and `cargo test -p xtask` (434 passed, 0 failed, 1 ignored; then 12,
1, 1, and 0-test targets all passed). `DECKMASTE_XTASK_WORKERS=8 cargo xtask
english_v2 coverage --check` passed with zero newly covered identities and zero
drops.

Performance advisory: the refreshed 8-worker coverage check ran for
55.696310714 s against the 16.26 s quiet-host ceiling, at 77,208 ns/B over
16,771 accepted units / 1,523,802 accepted
bytes; host load was 36.87/57.94/64.80. Concurrent-process count is unavailable in this sandbox;
the reviewer stamps contention. This sandbox's `available_parallelism()` was 8,
so that exact run correctly printed `criterion=quiet_host` and its quiet-host
warning rather than `criterion=capped_workers`. The capped-label unit test
injects a 4-worker host for a 2-worker run and proves the required advisory-only
`criterion=capped_workers` path never emits a regression warning.

Assurance: restored 0; re-spelled 0; ignored 0; added 4; removed 0. Deviations
and additions: none. Glossary gaps: none. STOP: none. Decision wanted: none.

### Review corrections

Measured on change `unnoksps`, lock byte-unchanged: 16,771 identities, 49,421
lines, SHA-256
`2cf7f9b716e13b27d626c60638d1266ca6cdc083bbcca87cb1435b4b00cd65ca`. Coverage,
census and construction numbers are unchanged from the record above.

- MEDIUM — flag-over-environment precedence lived in `CorpusArgs::workers()`,
  while `resolved_corpus_workers` carried an `explicit_workers` parameter no
  caller ever set, so the precedence test covered a path production never took.
  `CorpusArgs::workers()` now delegates to `resolved_corpus_workers` with its
  own `--workers` value, and `default_corpus_workers` takes the environment
  value it parses; the precedence test now exercises the production path.
- MEDIUM — the capped-label test asserted only the capped branch, so a constant
  `"capped_workers"` satisfied the suite. Added the full-parallelism
  counterpart asserting `criterion=quiet_host` with the regression warning, and
  asserted the effective worker count (`workers=2`) on the capped line.
- MEDIUM — the invalid-value test covered `0` alone; it now covers `0`, `-1`,
  `abc` and the empty string, each required to name the variable.
- MEDIUM — `ceiling_seconds=` and `criterion=` on the `PERFORMANCE` line, and
  the `write_corpus_performance_to` injected-host seam, are additions beyond
  the ticket's letter that the record listed as "none"; they are kept (a capped
  run prints no warning, so the criterion would otherwise be invisible) and
  disclosed here.
- LOW — the `--workers` help text now names `DECKMASTE_XTASK_WORKERS`.

Assurance after review: restored 0; re-spelled 0; ignored 0; added 5;
removed 0.

Environment note: `available_parallelism()` reads 24 at xtask start-up but 8 by
the time the telemetry line is written (measured with a temporary probe, since
reverted), so an 8-worker run on this host is not capped and still warns; the
capped path was verified end-to-end at 4 workers.

Reviewer gate artifacts: `cargo fmt --all`; `cargo clippy -p xtask
--all-targets -- -D warnings` clean; `cargo test -p xtask` 435 passed, 0
failed, 1 ignored (then 12, 1, 1 and 0-test targets); `taskset -c 0-23 env
DECKMASTE_XTASK_WORKERS=4 cargo xtask english_v2 coverage --check` passed with
`covered_units=16771`, `selected_uncovered_units=0`,
`licensing_checker_permitted=26`, `licensing_checker_forbidden=5`, and
`PERFORMANCE english-v2 gate=coverage workers=4 elapsed_seconds=213.090533078
accepted_cpu_micros_per_byte=119.048 ceiling_seconds=16.260
criterion=capped_workers` with no regression warning; `cargo xtask english_v2
ambiguity --require-resolved --workers 8` reported `unresolved_ties=0` and
`internal_failures=0`.

Reviewer performance advisory: the 4-worker capped coverage check ran
213.090533078 s against the 16.26 s quiet-host ceiling at 119,048 ns/B over
16,771 accepted units / 1,523,802 accepted bytes, host load
30.28/42.21/50.92. Contention at landing: 3-4 concurrent codex executors and
two other reviewers on the host.

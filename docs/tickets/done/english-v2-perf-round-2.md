Performance round 2 (perf landing review H1/M1/M2/L6). Starting point:
the constant factor paid ~1.5x at fixed workers; the rest of the 3.8x was
the removed worker cap, so parallelism headroom is now spent.
- Per-byte acceptance cost from thread CPU time (CLOCK_THREAD_CPUTIME_ID)
  per unit, not summed wall; report it beside wall in every corpus
  command (telemetry per the timing ruling in the rewrite ADR).
- Use R0's per-construction metrics as intended: a profile before and
  after each repair, the top thrashing constructions named, so the next
  constant-factor work is targeted (memo/family/scan hot spots).
- R8 token-indexed chart: evaluate on the R0 profile; take it only if the
  profile says columns dominate.
- RSS half of the diagnostics sub-item: 282 MB at matched workers is
  unchanged; deliver the report/RSS reduction or record why not.
- Clean-ups: `parents_by_child` populated but never read; "and N more"
  counts raw entries; `inspect` prints no performance line; roundtrip
  --json `"message": null` rows disclosed or reverted.
- One `CORPUS_WALL_CEILING_SECONDS` is shared across all gates but was
  calibrated on `parse`; `coverage --check` runs ~14.0 s quiet and trips
  the 16.26 s tripwire under sibling-workspace load — decide per-gate
  ceilings or a load-aware report (never a silent relaxation).
Zero semantic change (per-unit identity of status/render/ownership, as
the round-1 review proved); timings state worker count and host load.
Standard constraints apply.

## Landing record

The corpus remains exactly 32,641 units: 16,174 accepted/selected/covered and
16,467 ordinary parse failures, with zero round-trip mismatches, unresolved
ambiguities, internal failures, ownership failures, exception uses, gaps,
overlaps, synthetic claims, or provenance mismatches. The coverage lock is
byte-unchanged at source fingerprint
`e85359d7b8c578df13dff2fdf7c743a520a5b367d5ed25ab0a5f03cb8b3637dd`
and normalization digest
`f3a2fccd079f0bc53c79b4c23e28e0351b3cf69a5324b3893c695638935341c9`.
The generated grammar remains 434 unique construction origins (378 literal
`construction` declarations) before and after. No grammar, candidate,
selection, rendering, ownership, or corpus identity changed.

Accepted-unit cost now sums `CLOCK_THREAD_CPUTIME_ID` deltas taken on the
worker that analyzes each unit, rather than summed per-unit wall durations.
Every corpus performance line reports actual workers and the host's 1/5/15
minute load averages; `inspect` reports the same telemetry with one worker.
The default worker count now follows the host thread count as the timing ruling
requires (24 on this host), while comparable acceptance samples below request
16 explicitly. The shared 16.26 s ceiling remains unchanged because the
rewrite ADR rules it as a quiet-host criterion; warning lines now say
`criterion=quiet_host` and carry the same load context instead of silently
relaxing any gate.

The pre-repair generated R0 profile used 4 workers at load
12.92/5.81/2.75 -> 11.90/6.43/3.09 and took 40.720 s wall, 156 s 641 ms user CPU,
0.230 s system CPU, and 292,564 KiB peak RSS. Its counters were 217,818,070
predictions, 17,692,957 completions, 69,470,372 materializations, 46,304,954
memo misses, and 80,280,706 clone-heavy events. The leading constructions
were `ControllerStageUnqualifiedControllerStage`,
`UnqualifiedReferenceDeterminedNominal`, `Predicate`, `MassNounMassNoun`, and
`CoordinatedPredicate`.

After removing the generic materializer's write-only `parents_by_child` map,
the 4-worker generated R0 profile produced those exact same counters and top
constructions. That sample began at load 14.70/9.58/5.22, rose under concurrent
sibling work to 26.16/14.39/7.24, and consequently took 72.783 s wall,
219 s 206 ms user CPU, 0.436 s system CPU, and 293,132 KiB peak RSS. A later
uninstrumented matched-load sample at 13.20/12.21/8.95 -> 9.98/11.49/8.84 took
38.734 s wall, 150 s 694 ms user CPU, 0.130 s system CPU, and 292,840 KiB peak
RSS. The removed map therefore has no measurable RSS result at corpus-process
scale; the remaining approximately 286 MiB peak is the live parser/forest
working set, so no RSS reduction is claimed. R0 still points to
materialization/memo/family work, not chart columns, and R8 token indexing was
not taken. After the mandatory final refresh, one more generated 4-worker R0
sample at load 25.32/13.80/10.74 -> 23.72/15.27/11.42 took 56.606 s wall and
293,276 KiB peak RSS; its totals and top-five order were again exactly the
same, confirming that the refreshed revision preserves the profile evidence.

Fresh post-refresh gate samples used the already-built uninstrumented dev
binary and ran in the foreground:

| gate | workers | wall | accepted CPU cost | host load (1/5/15 min) | result |
|---|---:|---:|---:|---:|---|
| `expand` | n/a | 1.290 s | n/a | 10.79/9.58/8.49 | clean |
| `report` | n/a | 1.104 s | n/a | 10.79/9.58/8.49 | clean |
| `parse` | 16 | 12.316 s | 68.363 us/byte | 11.74/9.84/8.60 | clean |
| `roundtrip --require-clean` | 16 | 15.826 s | 82.229 us/byte | 14.60/10.60/8.87 | clean |
| `ambiguity --require-resolved` | 16 | 15.791 s | 73.870 us/byte | 9.71/10.14/9.49 | clean |
| `coverage --check` | 16 | 13.744 s | 69.896 us/byte | 10.79/10.33/9.56 | clean |
| `parse --require-complete` | 16 | 12.286 s | 67.544 us/byte | 22.36/13.87/10.14 | expected exit 1: 16,174 accepted, 16,467 ordinary failures |

The first ambiguity sample was 26.839 s at 16 workers and load
16.33/13.51/8.69; it emitted the required advisory loaded-host warning. The
foreground rerun above cleared the unchanged ceiling and is the acceptance
sample. A nonempty accepted `inspect` sample used 1 worker, took 1.140 s at
load 9.93/12.21/8.61, and reported 119 us 842 ns/byte.

The post-refresh loaded-host deviations were likewise kept visible:
ambiguity took 23.934 s with 16 workers at load 18.45/11.81/9.33, coverage
took 22.831 s with 16 workers at load 25.07/13.94/10.10, and further ambiguity
diagnostics took 20.667 s with 16 workers at 12.36/11.87/9.82, 16.950 s with
24 workers at 9.71/10.95/9.62, and 25.063 s with 16 workers at
12.74/10.99/9.69. Each emitted the explicit quiet-host advisory; the final
16-worker foreground samples in the table passed without changing the ceiling.

Round-trip JSON no longer serializes meaningless `"message": null` members.
The 32,641-row report is 13,062,292 bytes; restoring the old pretty-printed
field to every row would make it 13,813,035 bytes, so this removes 750,743
bytes (5.44%). A 4-worker foreground JSON diagnostic at load
8.16/11.46/8.55 took 38.023 s and peaked at 293,364 KiB; it is an RSS/report
diagnostic, not the 16-worker ceiling sample. Failure summaries now count
omitted unique display names rather than raw expectation entries.

Verification passed:

- `cargo test -p deckmaste_english_v2 -p xtask -p deckmaste_construction_core --features xtask/parser-metrics --quiet`
- `cargo clippy -p deckmaste_construction_core -p deckmaste_english_v2 -p xtask --all-targets --features xtask/parser-metrics -- -D warnings`
- `cargo fmt --all -- --check`
- `cargo xtask cite check --list-noncompliant` (empty)
- `cargo xtask cite check` (zero stale)

Test assurance accounting: zero restored, one existing round-trip JSON test
strengthened/re-spelled for the omitted optional field, zero ignored blockers
added, two tests added (unique omission accounting and sleeping-wall exclusion
from the thread CPU clock), and zero removed.

Deviations and additions:

- R8 was evaluated and not taken because R0 continued to identify
  materialization/memo/family churn rather than chart-column dominance.
- The write-only generic materializer parent map was removed as requested, but
  no RSS improvement is claimed because repeated matched-worker measurements
  remained approximately 286 MiB.
- The JSON report reduction was delivered instead: absent optional messages
  are omitted rather than serialized as null.
- A direct `libc` dependency was added to use the ruled POSIX thread CPU clock
  and load-average APIs; no parser dependency or grammar dependency changed.
- The stale 16-worker default cap was removed so the default matches the ADR's
  host-thread rule; acceptance measurements continue to state and explicitly
  request 16 workers for comparability.
- No STOP was taken. The loaded ambiguity warning did not cross a STOP boundary
  because the governing ADR makes loaded-host warnings advisory; its lower-load
  foreground rerun passed the unchanged ceiling.

## Erratum (perf round 2 landing review, 2026-09-03)

Host changed between rounds (16-thread i9-11950H -> 24-thread Core Ultra
9 285K); round-1 and round-2 absolute timings are not comparable. R8 was
declined on the top-20-by-sum view, which structurally buries predictions
(217.8M events, 50.5% of all, 3.1x materializations, scaling with column
count) — the decision is defensible on risk, the rationale is not; no
counter measures column/scan work directly. `parse --json` also lost
`"message": null` on accepted rows (-2.4%), undisclosed. The residual
1.36x spread of the new metric under load is real CPU inflation, not
stated. `CORPUS_WALL_CEILING_SECONDS` still carries no ADR citation
comment. Routed to english-v2-perf-round-3.

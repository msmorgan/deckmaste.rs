# English parser performance audit

Run the single-input audit before a corpus command. It reports deterministic
parser work for a representative short input and the supported `Ballroom
Brawlers` face: unique chart items, maximum chart-column width, and packed
forest counters. A large single-input increase is an algorithmic regression;
lowering corpus workers or imposing a process-memory limit is containment, not
an accepted fix.

Build before measuring. The measured process is the already-built binary, not
`cargo`, so compiler CPU, I/O, wall time, and RSS cannot contaminate the result.

```sh
cargo build -p xtask
PERF_BIN=target/debug/cargo-xtask
/usr/bin/time -f 'wall_seconds\t%e\npeak_rss_kib\t%M\ncpu_percent\t%P\nmajor_page_faults\t%F\nfilesystem_inputs\t%I\nfilesystem_outputs\t%O' "$PERF_BIN" english performance --json > /tmp/english-performance-current.json
```

Read the `/usr/bin/time` record on stderr with the JSON audit. Reject a result
whose CPU or I/O signature shows compiling or another unrelated workload leaked
into the measurement. `elapsed_millis` in the JSON is diagnostic; parent/current
review gates deterministic work counters, not a cross-machine time budget.

To compare a change with its already-measured parent, retain the parent's JSON
outside the repository and run the current binary with it. The allowance is a
reviewed relative change, not a timeless machine budget.

```sh
PERF_BIN=target/debug/cargo-xtask
/usr/bin/time -f 'wall_seconds\t%e\npeak_rss_kib\t%M\ncpu_percent\t%P\nmajor_page_faults\t%F\nfilesystem_inputs\t%I\nfilesystem_outputs\t%O' "$PERF_BIN" english performance --json --check --baseline /tmp/english-performance-parent.json --max-work-growth-percent 10 > /tmp/english-performance-current.json
```

The clean case is a current audit copied to `/tmp/english-performance-parent.json`;
the command reports `performance check passed`. The checked regression fixture
in `crates/xtask/src/english/performance.rs` raises `chart_unique_items` from
100 to 111 with a 10% allowance and asserts that the gate rejects it.

Only after both single inputs have been audited, measure corpus parallelism.
The `--workers` value is explicit and is included in each report; the current
cap of four workers is secondary containment of the outer multiplier.

```sh
PERF_BIN=target/debug/cargo-xtask
/usr/bin/time -f 'wall_seconds\t%e\npeak_rss_kib\t%M\ncpu_percent\t%P\nmajor_page_faults\t%F\nfilesystem_inputs\t%I\nfilesystem_outputs\t%O' "$PERF_BIN" english roundtrip --workers 4 --json > /tmp/english-roundtrip-workers-4.json
/usr/bin/time -f 'wall_seconds\t%e\npeak_rss_kib\t%M\ncpu_percent\t%P\nmajor_page_faults\t%F\nfilesystem_inputs\t%I\nfilesystem_outputs\t%O' "$PERF_BIN" english recovery --workers 4 --json > /tmp/english-recovery-workers-4.json
```

The single-input files and their timing records are per-face costs. The two
corpus files and their separate timing records are parallelism measurements;
do not divide one by the other or use a worker limit as evidence that a
single-parse regression is fixed.

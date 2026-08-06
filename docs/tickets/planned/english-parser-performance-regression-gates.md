**Make English parser and corpus-tool performance regressions visible before
resource limits conceal them.** During `english-coordination-derived`, one
supported face that previously parsed in about 0.39 s / 38,412 KiB RSS grew to
multi-gigabyte behavior; the first response bounded Rayon corpus concurrency,
mistaking the outer multiplier for the single-parse regression. A chained build
and timing invocation also produced a contaminated 1.19 GiB “one card” result.

Add a reproducible, build-excluding benchmark/audit for representative short
input plus the known stress face Ballroom Brawlers, and for corpus `roundtrip`
and `recovery` at explicit worker counts. Record wall time, peak RSS, and
deterministic parser-work counters (at least unique chart items and maximum
column width; add packed-forest counts if needed). Compare a change against its
parent rather than relying on timeless absolute numbers.

The diagnostic order is a gate: measure one input before corpus concurrency;
an order-of-magnitude single-input increase is an algorithmic regression until
proved otherwise. A worker limit or process memory ceiling is only secondary
containment and cannot be the sole accepted fix. Keep build time and compiler
memory outside the measured command, report the exact command and worker count,
and reject results whose CPU/I/O signature shows the build leaked into them.

Incident reference (2026-08-06, debug `cargo-xtask`, same local snapshot): the
pre-derived Ballroom run was 38,412 KiB / 0.39 s; the corrected generated parser
was 91,120 KiB / 0.47 s; a four-worker 32,344-face round-trip was 422,828 KiB /
39.90 s. These are calibration evidence, not permanent cross-machine budgets.

Gate: the harness has a checked regression fixture that fails when deterministic
single-input work grows beyond its reviewed allowance; its report separates
per-face cost from corpus parallelism; and documentation demonstrates an
intentional failing regression plus the clean parent/current comparison.

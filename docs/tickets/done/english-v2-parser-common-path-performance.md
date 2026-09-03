---
needs: [english-v2-plan-09-effect-and-predicate-grammar]
---
Optimize the English-v2 full-corpus analysis path without narrowing grammar,
discarding candidates, adding semantic ranking, or omitting corpus units. The
v1-versus-v2 investigation this ticket used to schedule has been performed; its
findings are recorded below and what remains is engineering against them.

The Plan 09 boundary samples this ticket carries:

~~~text
ELAPSED expand 3.49
ELAPSED report 2.09
ELAPSED parse 84.68
ELAPSED roundtrip 78.69
ELAPSED ambiguity 82.29
ELAPSED coverage 85.75
ELAPSED require-complete 79.92 EXIT 1
~~~

## Diagnosis: the Earley chart's constant factor, and nothing downstream of it

A flat CPU profile over a full parse run (dev, 56K samples; release profiles
agree) attributes 73.6% of the run to memory churn and ordered-map lookup:
37.2% libc malloc/free/realloc, 13.4% clone (`Vec<Child>`,
`Vec<SpannedLexical>`, `Vec<RulePosition>`, `BuildValue`, `Leaf`,
`MaterializedCandidate`), 10.7% BTreeMap, 7.8% drop glue, 4.5% RawVec growth.
Real grammar work — engine, materialize, scan, construction guards — is 21.3%.
Render, ownership, selection, diagnostics and the coverage lock are 0.00% of
the profile. Cost is linear in text length (about 1.0 ms + 0.061 ms per byte
per unit): there is no complexity blowup to chase, only a large constant.

Normalized against v1 on the same machine, corpus and day, v2 costs 2.7x per
successfully-handled unit and 5.3x per byte of successfully-handled text —
while not rendering, and while v1's figure includes its structural recovery.
Raw wall clock (65.8 s v2 versus 58.1 s v1) flatters v2 because it credits it
for the 80% of the corpus it declines.

## The wall-clock ceiling is the wrong acceptance criterion (pending user ruling)

Acceptance, not failure, is the expensive regime, and it is the one that grows.
The 20% of units that parse burn 47% of the CPU while carrying 10% of the text:
232 µs per byte against 30 µs per byte for a failing unit, because a failing
chart dies early and builds no forest.

So today's 66–86 s is a floor, not a ceiling. At the current per-byte
acceptance cost a fully covering grammar over the same 5.16 MB projects to
about 1,197 s CPU — roughly 75 s wall even at perfect 16-way parallelism, with
the parallelism headroom already spent. A repair that just clears 16.26 s today
breaches again by Plan 11, and accepted units are also the short ones (mean
82 bytes against 177 for failures), so 232 µs/byte is optimistic for the
longer text Plans 10+ will admit.

The ceiling should therefore move from wall time per separately launched
command to per-byte acceptance cost, targeted an order of magnitude below
today's figure. **That changes a ruled acceptance criterion and needs a user
ruling before this ticket closes.** Until the ruling is made, acceptance stays
the 16.26-second ceiling and this ticket additionally records per-byte
acceptance cost at every measurement.

## R0 — per-construction metrics in the generated grammar, feature-gated (user-pinned shape; do this first)

Before any repair lands, the declaration compiler emits aggregated metrics
into the generated grammar functions behind a cargo feature (off by default,
zero-cost when disabled): per-construction counters for predictions,
completions, materializations, memo misses, and clone-heavy paths, with
optional attributed time. Corpus commands compiled with the feature print a
ranked top-N thrashing-constructions table beside the totals. Emission lives
in the compiler so the instrumentation is generated alongside each
construction — no hand-maintained mirror, and it grows with the grammar
automatically. This is how R1–R9's impact hypotheses get falsified per
construction rather than per profile, and how future frontier rounds see
which construction is thrashing from now on.

Fold the runtime-observability requirement back in here: the plan-gate
timing harness was excised with the Plan 09 fossils, so corpus commands
currently print elapsed time with no regression warning. Every corpus
command prints elapsed wall time and per-byte acceptance cost
unconditionally; the loud named regression warning against a recorded
baseline returns with whatever criterion the pending ceiling ruling sets.

## Ranked repairs

Every missing optimization already has a working implementation in v1 in this
repository; none of them touches the grammar. Impact figures are hypotheses to
falsify by measurement, in this order.

| # | change | site | expected | v1 precedent |
|---|---|---|---|---|
| R1 | Hoist the materialization memo out of `completion_has_checked_build` — one state per parse, invalidated per node on family extension; stop cloning values on memo hit/insert | `materialize.rs:793-834`, `:242`, `:275` | 2–5x; targets the acceptance regime | none — this is a defect v1 never had |
| R2 | Intern forest nodes through a `HashMap<NodeKey, NodeId>` instead of a linear `nodes.iter().position` per completion | `engine.rs:669-682` | 1.5–3x; flattens the grammar-growth curve | `forest.intern_node` |
| R3 | Index completer waiters by `(origin, category, suppress_right_boundary, origin_state)`; iterate by reference instead of cloning key and families | `engine.rs:497-511` | 1.5–2.5x | hashed waiter index |
| R4 | Precompute an LHS→rules prediction index, and the same for `completed_by_start` filtering | `engine.rs:534-556`, `:558-566` | 1.3–2x today, more per plan | `rules_for` |
| R5 | Binarize the SPPF: one intermediate node per item instead of copying `Family.children`; share prefixes via `Rc` | `engine.rs:568`, `:761`, `:893-913` | 1.5–2.5x | `Rc`-shared prefix features |
| R6 | Replace BTreeMap/BTreeSet with hashbrown in the chart and the memo | `engine.rs:890`, `materialize.rs:146-153` | 1.1–1.2x | v1 already depends on hashbrown |
| R7 | Memoize lexical scans by `(terminal, offset, position, suppress)` | `parser/scan.rs:659-676` | 1.05–1.15x | `self.scans` |
| R8 | Token-index the chart on word boundaries, keeping byte spans on leaves for byte-exact render | `engine.rs:445-450`, `scan.rs:129`, `:156` | 1.2–1.5x (6.06 bytes/token, so 6x fewer columns) | token-indexed chart |
| R9 | Bound the post-forest-growth re-enqueue to items that can depend on the extended node | `engine.rs:475`, `:915-928` | multiplier on all of the above | — |
| R10 | Raise the hardcoded `MAX_CORPUS_UNIT_JOBS = 4` and expose it as `--workers`; schedule longest-text-first to avoid a rayon tail | `corpus.rs:16`, `:170-212` | 1.7–1.9x **measured** on a 16-thread machine — not 4x; the workload is memory-bound | v1 exposes `--workers` |

R1–R4 are contained engine changes with working precedents; if they deliver the
low end of their range they land parse near 9 s. R5 and R8 are the structural
reserves — hold them until measurement says R1–R4 were not enough. R6 needs a
check that no iteration-order determinism is relied on. R10 is same-day and
zero-risk.

The dev build-profile item is **done**: `sha2`, `deckmaste_data`,
`deckmaste_catalogs`, `syn`, `proc-macro2` and `quote` now carry dev
`opt-level = 2` overrides. Note a corrected attribution while doing it — the
SHA-256-with-un-inlined-SIMD profile that motivated the first three is the
*corpus-loading* fixed overhead, and `expand`/`report` do not load the corpus.
Their cost is `syn` re-parsing the generated grammar, and optimizing the
proc-macro crates is what roughly halves both, retiring the two
relative-slowdown warnings as the build-profile artifact they always were.
Running the corpus gates from a release or dedicated profile remains available
(`parse` 75.8→65.8) but slows gate turnaround, so it is a fallback rather than
a first move.

## Sub-item: failure diagnostics are a second, separate regression

v1 emitted structured, ranked, bounded rejections — typed reasons carrying
production, lhs, origin, position, dot, slot and prefix features, kept as a
top-k ordered furthest-progress-first and capped. v2's corpus-facing failure
message is the entire live rule-position set of the last non-empty chart
column: unranked, unbounded, and not de-duplicated by display name. Measured
over the 26,083 failing units: 55.6 MB of messages, median 179 expected items
and 3,339 bytes per failure, worst case 194 items with `declaration verb`
repeated 12 times and `declaration noun` 8 times. `parse --json` writes a
69.6 MB report, and the audit report retaining `text` and `rendered` strings
for all 32,641 rows puts RSS at 280 MB.

v2 already owns the machinery for better (`parser/diagnostic.rs`, 1,882 lines,
plus the xtask diagnostic module) — the census path simply does not use it.
Route the census through it, emit a ranked top-k, and skip building the
expectation set at all when the caller only wants a count. This is a usability
regression that slows every future frontier round; it is worth ~1–3% of CPU and
all of the report and RSS bloat, but it is scheduled here for the diagnostics,
not the speed.

## Acceptance

Byte-exact rendering, total ownership, zero ties and exceptions, the complete
32,641-unit outcome partition, and the ordinary add-only coverage ratchet are
unchanged. Every separately launched command satisfies the 16.26-second ceiling
(or the per-byte criterion, once ruled), and each measurement records per-byte
acceptance cost beside wall time. Failure messages are ranked and bounded.
Record fresh v1/v2 baselines, profiles, the repairs actually taken, and
post-repair samples.

## Landing record

The ruled 16.26-second wall ceiling remains the acceptance criterion; no new
criterion ruling was made. Measurements below are ordinary dev-profile runs on
the same shared 16-thread host. The fresh v1 comparison still reports its
existing four-worker cap, so it is contextual rather than a like-for-like
parallelism comparison.

| sample | before | after |
|---|---:|---:|
| v2 `parse` wall | 56.690 s | 15.059 s |
| accepted cost | about 232 us/byte in the ticket diagnosis | 87.016 us/byte |
| parse report size | 23,936,694 bytes | 3,490,375 bytes |
| fresh v1 `roundtrip --require-clean` wall | 36.537 s | n/a |

Fresh final corpus-gate samples, each explicitly requested with 16 workers:

| gate | wall | accepted cost | result |
|---|---:|---:|---|
| `parse --require-complete` | 15.059 s | 87.016 us/byte | expected exit 1: 16,174 accepted and 16,467 ordinary failures |
| `roundtrip --require-clean` | 14.591 s | 83.371 us/byte | clean |
| `ambiguity --require-resolved` | 14.879 s | 74.246 us/byte | resolved |
| `coverage --check` | 13.281 s | 70.244 us/byte | clean |

`expand` took 1.564 s and `report` took 1.137 s. The complete 32,641-unit
partition after the final refresh is 16,174 accepted and 16,467 ordinary parse
failures, with zero roundtrip mismatches, zero unresolved ties, zero exceptions
or exception uses, zero ownership failures, and zero internal failures.
Coverage selected and covered all 16,174 accepted units, with zero gaps,
overlaps, synthetic spans, or provenance mismatches; the add-only lock did not
change. The refresh brought 208 additional accepted units from default; the
feature itself did not change grammar, candidate selection, or outcomes.

R0 landed as generated, feature-gated counters for prediction, completion,
materialization, memo misses, and clone-heavy activity. The final full-corpus
instrumented sample recorded 217,800,452 predictions, 17,684,186 completions,
69,436,102 materializations, 46,280,504 memo misses, and 80,240,134 clone-heavy
events. Its leading constructions were
`ControllerStageUnqualifiedControllerStage`,
`UnqualifiedReferenceDeterminedNominal`, `Predicate`, `MassNounMassNoun`, and
`CoordinatedPredicate`. Instrumentation is deliberately off by default and its
atomic observation overhead is excluded from the normal-command ceiling.

The retained repairs are persistent per-parse checked-materialization state
with a completion-only value cache; indexed node interning, waiters, LHS rules,
and completed starts; shared-prefix partial families with an inline singleton
family representation; family-reachable requeueing; lexical-scan memoization;
requested worker parallelism with longest-text-first scheduling; and streaming
or omitted audit strings where the caller does not consume them. Corpus-facing
parse failures now rank literal, terminal, then nonterminal expectations,
deduplicate display names, show at most eight names, and report the omitted
count while the parser retains its complete private diagnostic.

Experiments replacing the chart with a hash map, adding a completion reverse
dependency index, and using a custom hot-path hasher regressed measurements and
were removed. A direct `hashbrown` dependency also violated the repository's
dependency invariant and was removed. R8 token indexing was not needed after
the ceiling cleared. No grammar, candidate set, semantic ranking, corpus
selection, or direct dependency set changed.

Verification passed with the parser-metrics feature enabled:

- `cargo test -p deckmaste_english_v2 -p xtask -p deckmaste_construction_core --features xtask/parser-metrics --quiet`
- `cargo clippy -p deckmaste_construction_core -p deckmaste_english_v2 -p xtask --all-targets --features xtask/parser-metrics -- -D warnings`
- `cargo fmt --all -- --check`

Test assurance accounting: zero tests restored, one existing concurrency test
re-spelled to assert the requested worker count rather than the removed
four-worker cap, zero ignored blockers added, one new bounded/ranked diagnostic
test added, and zero tests removed. Existing CLI fixtures were updated only for
the new `--workers` interface.

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
81.8 B against 177.2 B for failures), so 232 µs/byte is optimistic for the
longer text Plans 10+ will admit.

The ceiling should therefore move from wall time per separately launched
command to per-byte acceptance cost, targeted an order of magnitude below
today's figure. **That changes a ruled acceptance criterion and needs a user
ruling before this ticket closes.** Until the ruling is made, acceptance stays
the 16.26-second ceiling and this ticket additionally records per-byte
acceptance cost at every measurement.

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

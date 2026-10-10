---
needs: []
---
# Cut the grammar crate's compile time: emit each rule as its own function instead of one 69,000-line `Grammar::default`

Priority: critical for cadence. No grammar, lexicon or Reading change.

## Why

Every rebuild after a grammar edit takes about 18–25 minutes, and each ticket
rebuilds several times (probe binary, tests, clippy, again after a refresh).
Measured 2026-10-10 on trunk `psumvttx` (contended machine, so wall times are
upper bounds):

- `cargo build -p deckmaste_english_v3` after a `declarations.rs` change:
  about 25 minutes wall; cargo timings give frontend 44 s, codegen 1,479 s.
- One rustc thread does all of it: LLVM `opt` then ThinLTO on a single
  codegen unit, about 21 CPU-minutes. Every other unit is done in under 30 s.
- The expanded crate is 161k lines; `impl Default for Grammar { fn default() }`
  alone is 69,560 lines (9 MB): one expression building `rules: vec![Rule{…}
  × 723]` and `productions: vec![… × 723]` with 14,586 nested `vec![]`
  literals. It is emitted by `grammar()` in
  `crates/deckmaste_construction_v3_core/src/emit.rs` (about line 201).
- `opt-level = 1` does not help (20 minutes); `opt-level = 0` gives 31 s but
  makes the census about 2.7× slower.
- A prototype that emits each rule and each production as its own
  `#[inline(never)]` function, with `default()` calling them, built the crate
  in 96–163 s at `opt-level = 2`, `cargo build -p xtask` in about 3 minutes,
  with a byte-identical bounded census (300 faces, 14,405 Readings). The
  function runs once behind a `OnceLock`.

## Goal

1. The emitter produces one small function per rule and per production (or
   an equivalent split) so that no generated function is large enough to
   dominate codegen; `Grammar::default` only assembles them. Same `Grammar`
   value, same Readings.
2. Measure and, if cheap, cut the next bottleneck: the integration-test
   binary (`crates/deckmaste_english_v3/tests/main.rs`, 31.5k lines, built at
   the package's `opt-level = 2`) takes about 6 minutes on its own.

## Analysis

Orchestrator rulings 2026-10-10:

- E1. Start from the prototype patch if it is present at
  `/tmp/claude-1000/-home-msmorgan-Projects-deckmaste-rs/6a836d28-7861-4fbf-bda1-fd89fab69fd9/scratchpad/emit-split-rules.patch`
  (apply it by hand; it was not run against the test suites). The slice
  grammar's `Grammar::default` gets the same treatment if the same emitter
  produces it.
- E2. Equivalence is the acceptance test, not a review opinion: a bounded
  census (2,000 random faces chosen by the stock `diff --manifest … --random
  2000 --seed 20261010`, plus the 300-face controls) before and after must be
  identical per face in count AND digest (`cargo xtask english-v3 diff`:
  0 lost, 0 gained, 0 decreases, 0 increases, 0 digest-only).
- E3. Keep `opt-level = 2` for the grammar crates; do not trade census speed
  for build speed.
- E4. For Goal 2, try in this order and keep the first that works without
  recompiling the library twice: a profile override that lowers optimisation
  for the test target only; splitting `tests/main.rs` into a few test
  binaries so they compile in parallel; leaving it. Record build and run time
  for each attempt. Do not weaken or delete tests.
- E5. Record before/after wall times for the four commands tickets actually
  run after a real `declarations.rs` edit (add and remove a feature
  declaration, not a bare `touch`): `cargo build -p xtask`, `cargo test -p
  deckmaste_english_v3 --no-run`, `cargo test -p deckmaste_english_v3`,
  `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`; note machine
  load next to each.

## Method

Standard constraints apply. Tests: full `cargo test -p` and clippy for
`deckmaste_construction_v3_core`, `deckmaste_english_v3` and `xtask`
(`cargo test -p xtask english_v3`). No census `--all`. No grammar or lexicon
edits.

## Landing record

Six items as specified in `docs/tickets/README.md`; item 1 is the E2
equivalence diff, item 6 the E5 timing table.

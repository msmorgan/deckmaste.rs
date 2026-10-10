---
needs: []
---
# Build the English v3 integration tests in under two minutes: `cargo test -p deckmaste_english_v3 --no-run` takes 7

Priority: high. Build time, not grammar: no Construction, lexicon or census
change.

## Why

`english-v3-emit-rules-as-functions` (`zrrnvqto`) cut `cargo build -p xtask`
after a `declarations.rs` edit from 22:12 to 2:03. The test build did not
follow: its landing record (contended machine) has

| Command, after a real declaration edit | Wall |
| --- | --- |
| `cargo build -p xtask` | 2:02.85 |
| `cargo test -p deckmaste_english_v3 --no-run` | 6:56.85 |
| `cargo test -p deckmaste_english_v3` (run only) | 0:21.08 |
| `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` | 0:36.93 |

Almost all of the 6:57 is one unit: the single integration-test binary
`crates/deckmaste_english_v3/tests/main.rs` (116 suite files under
`tests/suite/`, about 34k lines, 743 `#[test]` at `nvotwtqp`), compiled at the
package's `[profile.dev.package.deckmaste_english_v3] opt-level = 2` (root
`Cargo.toml`). Every grammar ticket pays it at least twice (final suite, and
again after a refresh). Goal 2 of that ticket was split off by orchestrator
ruling; this is it.

## Goal

After a real `declarations.rs` edit, building the English v3 tests takes under
about 2 minutes wall beyond the library build, with every test still present
and passing, the library still at `opt-level = 2`, and the test run not
materially slower than today's 21 s.

## Analysis

No CGEL citation applies. Facts:

- A Cargo profile override is per package, not per target. Lowering only the
  test target of `deckmaste_english_v3` is not expressible; any per-profile
  trick that changes the package's opt level builds the library twice (once
  for xtask at 2, once for tests), which is the cost just removed.
- `[lib] test = false` is already set; `src/` has no unit tests.
- `tests/common/mod.rs` builds the workspace Lexicon and compiles the Grammar
  once per test binary (`common::lexicon()`, `common::ENVIRONMENT`).
- Dev-dependencies of the test target: `deckmaste_lexical_source`, `clap`.
- CI runs `cargo test --workspace` and `cargo clippy --workspace
  --all-targets`; neither names the package.

Orchestrator rulings 2026-10-10:

- TP1. Try in this order and keep the first that meets the Goal.
  (a) Move `tests/` to a sibling workspace package (working name
  `deckmaste_english_v3_tests`, `publish = false`, no `[profile.dev.package]`
  entry) that depends on `deckmaste_english_v3`: the test code compiles at the
  workspace default against the shared `opt-level = 2` library, built once.
  (b) Keep the package and split `tests/main.rs` into several test binaries
  that compile in parallel. (c) Both.
- TP2. Never lower the optimisation of `deckmaste_english_v3`,
  `deckmaste_construction_v3` or any lexical crate, and never give the tests
  a second copy of the library (E3 of `english-v3-emit-rules-as-functions`).
  `cargo build -p xtask` after the change must reuse the same library
  artefact the tests use: show it with `cargo build --timings` or the
  fingerprint output, not by argument.
- TP3. No test is weakened, deleted, merged or `#[ignore]`d. The count of
  `#[test]` functions and of tests run is identical before and after, by name
  (`cargo test … -- --list` diffed); a move is a move.
- TP4. Run time is part of the result. Under (a) the test crate's own code
  and its monomorphised generics run unoptimised; under (b) each binary
  loads the Lexicon and compiles the Grammar again. Measure the full run for
  each option tried. Accept up to about 60 s for the full run; beyond that,
  take the next option or STOP with the numbers. A per-package `opt-level`
  for the new test package is allowed as a tuning knob (it does not touch the
  library).
- TP5. The command every brief names must keep meaning something. After the
  change `cargo test -p deckmaste_english_v3` must not exit 0 having run no
  tests while the real suite lives elsewhere, unless the same landing
  (i) records the new commands in CLAUDE.md "Tests for a landing" (one line:
  the package that holds the English v3 tests, for both `cargo test -p` and
  `cargo clippy -p … --tests`), in `docs/tickets/README.md` (Landing record,
  item 5) and in the tests' own README, including the single-module form
  (`--test main new_module::` today); and (ii) leaves the old package with
  no test target at all, so that the old command reports nothing to run
  rather than a green suite. Done tickets are history and are not edited.
  Open tickets in `planned/` are covered by the CLAUDE.md line; tickets in
  `wip/` belong to other workspaces and are not touched.
- TP6. Under (b), the split is by existing suite files (no file is cut), the
  number of binaries is small (3–6), `use crate::common;` keeps working in
  every suite file, and `cargo test -p deckmaste_english_v3` still runs all
  of them, so TP5 is satisfied without a rename.
- TP7. CI needs no new step under either option (workspace commands); say so
  in the landing record after checking `.github/workflows/ci.yml`.

## Witnesses

Not grammar: the witnesses are timings. Record before and after, on the claim
parent and on the result, after the same real `declarations.rs` edit (add and
remove a feature declaration, not a bare `touch`), with `uptime` load next to
each:

- `cargo build -p xtask`
- the test build (`cargo test -p <package> --no-run`)
- the test run alone
- `cargo clippy -p <package> --tests -- -D warnings`
- the incremental case that matters most to a ticket: a one-line edit to one
  suite file, then the test build.

Controls that must not change: the `-- --list` test names (TP3); a bounded
census of 300 random covered faces, identical per face in count and digest
(`cargo xtask english-v3 diff`: 0 lost, 0 gained, 0 decreases, 0 increases,
0 digest-only), which shows the library was not touched.

## Method

Standard constraints apply. Deltas:

1. No grammar, lexicon or plugin edit; no census beyond the 300-face control
   (`cargo xtask english-v3 diff --manifest … --random 300 --seed 20261008`,
   `--workers 6 --samples-per-face 0`).
2. Measure the baseline first, then each option tried, in a table in the
   landing record (item 6 carries the timings instead of a census wall
   time). Keep the logs under the workspace's ignored `target/english-v3/`.
3. Full `cargo test` and `cargo clippy --tests -- -D warnings` for
   `deckmaste_english_v3` and the new package (if any), and
   `cargo clippy --workspace --all-targets -- -D warnings` once, since a new
   workspace member changes what CI lints.
4. `jj` moves keep history: move the files (`tests/suite/*`, `tests/common`,
   `tests/main.rs`, the README), do not recreate them.
5. Assurance counts in item 5: restored 0 / re-spelled 0 / ignored 0 /
   added 0 / removed 0 is the expected result.

## Out of scope

- The library's own build time (done in `english-v3-emit-rules-as-functions`).
- xtask's tests and the lexical crates' tests.
- Making the test run itself faster (parallel Lexicon loading, caching).
- Any change to which tests exist.

## Landing record

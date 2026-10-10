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
- TP8 (orchestrator amendment, 2026-10-10). Reverse TP1: try (b) first,
  using 4–6 binaries. Keep it if the test build after a real declaration
  edit takes under about 2.5 minutes on a lightly loaded machine and the full
  run stays under about 60 s; otherwise try (a) with TP5 in full. Under (b),
  document binary names only in the tests' README. After refresh, preserve
  every trunk suite registration in exactly one binary and compare test names
  again. Each binary root states where new suite modules go.

The build-time task briefing supersedes the former 300-face census control:
no census applies to this ticket; timings replace grammatical witnesses.

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

Controls that must not change: the `-- --list` test names and every test
assertion (TP3). No census is required by the build-time task briefing.

## Method

Standard constraints apply. Deltas:

1. No landed grammar, lexicon or plugin edit; no census under the build-time
   task briefing. The temporary declaration edit is measurement only.
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

1. **Census delta**: not applicable under the build-time briefing; no census
   was run. Selector, face count and census diff are not applicable. All
   figures below were measured on `puryqpqq`; the baseline uses the test
   layout of claim parent `mpolznkq`, refreshed onto `uoqyuxtnowpo`.
2. **Witnesses**: option (b) meets TP8: the declaration-edit test build is
   75.25 s and the full run is 29.18 s. The single-suite rebuild is 29.37 s.
   The full package command and single-module selection remain available;
   the documented `suite_6 bounded_quantity_mass_noun::` selection lists
   its eight tests. No sibling-package option was needed.
3. **Spot checks**: no newly covered faces apply. All 124 parent suite
   modules are registered exactly once, with cross-module helper imports
   kept together. The before/after `cargo test -p deckmaste_english_v3 --
   --list` name diff is empty: 790 names before and after. Suite files and
   their assertions are unchanged in the final diff; all 785 literal
   `#[test]` attributes remain.
4. **Deviations and STOPs**: TP8 reverses TP1 and is recorded above. The
   inherited arity failure caused a STOP; trunk fix `uoqyuxtnowpo` resolved
   it before the baseline. Four test dev-dependencies align existing
   dependency features with xtask to satisfy TP2; no optimization profile
   changed. Both measurement phases toggle the same unused feature
   declaration (baseline adds it, split removes it); both first build
   xtask. Both phases also change the same assertion's diagnostic message
   for the one-line suite measurement. All temporary edits are restored.
   Each binary allows unused shared-fixture helpers with an explicit reason.
   Cargo's split timing report has six concurrently compiling test units
   and no library compilation; all use
   `libdeckmaste_english_v3-361a25159ac980c9.rlib`, also used by xtask.
   The final xtask build reports the library as `Fresh`. No Construction,
   frame or test was added or deleted; no glossary gap arose.
5. **Tests**: all commands below succeeded. Both full package runs passed
   all 790 tests, with zero failures or ignored tests. Both package clippy
   runs and the workspace clippy run are clean. Assurance: restored 0 /
   re-spelled 0 / ignored-with-blocker 0 / added 0 / removed 0; no removals
   need justification. Checked `.github/workflows/ci.yml`: its workspace
   commands discover all six binaries, so no new CI step is needed.
   Citation checks report zero noncompliant strings and zero stale citations;
   no new rules were cited.

   ```sh
   cargo test -p deckmaste_english_v3 -- --list
   cargo test -p deckmaste_english_v3 --no-run --timings
   cargo test -p deckmaste_english_v3 --no-run --timings -v
   cargo test -p deckmaste_english_v3 --no-run
   cargo test -p deckmaste_english_v3
   cargo test -p deckmaste_english_v3 --test suite_6 bounded_quantity_mass_noun:: -- --list
   cargo clippy -p deckmaste_english_v3 --tests -- -D warnings
   cargo clippy --workspace --all-targets -- -D warnings
   ```
6. **Timing**: controlled wall times on `puryqpqq`; uptime loads are the
   one/five/fifteen-minute readings at each command's start. Initial cache
   warming is excluded. The declaration-edit builds use `--timings`, and
   xtask plus the split test build also use `-v`. Evidence is under
   `target/english-v3/english-v3-integration-test-package/`.

   | Command/case | Baseline wall | Baseline load | Split wall | Split load |
   | --- | ---: | --- | ---: | --- |
   | `cargo build -p xtask --timings -v` | 121.90 s | 15.17 / 11.98 / 8.22 | 101.19 s | 3.24 / 4.08 / 5.99 |
   | `cargo test -p deckmaste_english_v3 --no-run` after declaration edit and xtask build | 409.58 s | 12.47 / 12.59 / 8.93 | 75.25 s | 5.80 / 5.09 / 6.18 |
   | `cargo test -p deckmaste_english_v3` (run only) | 12.92 s | 2.58 / 3.97 / 6.08 | 29.18 s | 4.43 / 5.61 / 6.29 |
   | `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` | 25.91 s | 4.58 / 4.36 / 6.18 | 22.90 s | 4.16 / 5.45 / 6.21 |
   | `cargo test -p deckmaste_english_v3 --no-run` after one-line suite edit | 297.52 s | 2.79 / 6.40 / 7.42 | 29.37 s | 7.81 / 6.32 / 6.55 |
   | `cargo clippy --workspace --all-targets -- -D warnings` | — | — | 42.83 s | 3.87 / 5.26 / 6.12 |
   | `cargo build -p xtask --timings -v` after final tests (reuse proof) | — | — | 0.14 s | 4.39 / 5.41 / 6.18 |

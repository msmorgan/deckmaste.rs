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

1. **Census delta** (`tsuuozyn`, before the emitter edit and after the final
   emitter edit; claim parent `qquyuyns`): E2 compares the stock-diff selection
   of 2,000 covered faces (seed 20261010) plus 300 controls (seed 20261008),
   deduplicated to 2,276 identities. Covered 2,276 → 2,276; Readings 117,740 →
   117,740. Lost / gained / decreases / increases / digest-only / source
   changes: all 0; 0 issues. Lost faces: none. Evidence:
   `target/english-v3/english-v3-emit-rules-as-functions/{manifest,before,after,diff}.json`.

   After the mandatory refresh, `zmxzpyny` rechecks the identical 2,276-face
   manifest: covered 2,276 → 2,276; Readings 117,740 → 60,035; lost 0, gained
   0, decreases 929, increases 11, digest-only 567, unchanged 769; 0 source
   changes and 0 issues. These incoming Reading retirements, T5 carrier gains,
   and constructor re-spells are attributed to trunk `ukytykwr` under T1–T5
   ([trailing-Adjunct landing](../done/english-v3-trailing-adjunct-height.md#landing-record)).
   Lost faces remain none. Evidence: `refreshed.json` and `refreshed-diff.json`
   in the same evidence directory. The emitter's E2 acceptance result above
   remains the exact-zero comparison made before refresh.
2. **Witnesses** (`tsuuozyn`): E2 replaces parse witnesses for this build-time
   ticket. All 2,276 face counts and Reading digests are identical; the existing
   English v3 suite also exercises the main and slice grammars.
3. **Spot checks** (`tsuuozyn`): no newly covered faces; E2 requires zero
   gains, so ten newly covered faces are inapplicable.
4. **Deviations and STOPs** (`tsuuozyn`): the E1 prototype hit the existing
   `too_many_lines` lint (165/150). Its helper-generation expressions were moved
   into private `grammar_functions`, preserving the generated functions and
   calls; no lint allowance. No Construction, frame, lexicon route or test
   added or deleted; no glossary gap or STOP. By orchestrator ruling 2026-10-10,
   Goal 2 (the integration-test binary, responsible for the remaining 6:57
   `test --no-run` time) is split into its own follow-up ticket and is NOT done
   here. Goal 1 was accepted by orchestrator review on 2026-10-10. Grammar
   optimization remains 2.
   The closed implementation `tsuuozyn` was refreshed successfully onto
   incoming grammar/core changes `ukytykwr` (trailing Adjunct height) and
   `kxwsyulp` (keyword dash costs), including the inherited exact-count
   re-spells in `youxkuvq`. No conflict repair or feature code edit was needed.
5. **Tests** (`tsuuozyn`): `cargo test -p deckmaste_english_v3` — 731 passed;
   `cargo test -p deckmaste_construction_v3_core` — 34 passed;
   `cargo test -p xtask english_v3` — 56 passed. All three
   `cargo clippy -p <crate> --tests -- -D warnings` commands passed, with
   `<crate>` respectively `deckmaste_english_v3`,
   `deckmaste_construction_v3_core`, and `xtask`. The E5 `cargo test -p
   deckmaste_english_v3 --no-run` and subsequent full 731-test run also passed.
   Test changes: restored 0 / re-spelled 0 / ignored 0 / added 0 / removed 0;
   no removals to justify. Citation check: 0 noncompliant, 0 stale; diff audit:
   0 citation sites; no new rules to bless.
   Required post-refresh full suites on `zmxzpyny`:
   `cargo test -p deckmaste_construction_v3_core` — 36 passed;
   `cargo test -p deckmaste_english_v3` — 743 passed; no failed or ignored tests.
   Review-requested clippy checks on the current refreshed head (`mswmnwtr`)
   all passed once:
   `cargo clippy -p deckmaste_construction_v3_core --tests -- -D warnings`;
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`;
   `cargo clippy -p xtask --tests -- -D warnings`. Evidence:
   `review-{core,english,xtask}-clippy.log` in the evidence directory. Test
   change counts above describe this ticket; incoming test additions and
   re-spells belong to their trunk tickets. Post-refresh citation checks also
   report 0 noncompliant and 0 stale citations; the record diff adds no citations.
6. **Timing** (`tsuuozyn`): bounded E2 census, 6 workers, samples-per-face 0,
   under `/usr/bin/time -v`: before 38.36 s (uptime load 13.48/19.33/20.39),
   after 35.24 s (10.84/14.25/13.41). E5 before uses the claim-parent build;
   after holds a temporary unused feature declaration through all four commands,
   then removes it. The final source retains no declaration change. Loads below
   are uptime's 1/5/15-minute averages measured immediately before each command;
   these are contended-machine measurements. The lint-extraction verification
   reused identical generated code; its warm rebuild is not the E5 after result.

   | Command | Before wall; load | After real declaration edit wall; load |
   | --- | --- | --- |
   | `cargo build -p xtask` | 22:12.43; 14.65/16.33/19.17 | 2:02.85; 5.04/9.34/11.55 |
   | `cargo test -p deckmaste_english_v3 --no-run` | 20:42.48; 13.48/18.80/20.18 | 6:56.85; 11.31/10.79/11.84 |
   | `cargo test -p deckmaste_english_v3` | 0:21.53; 10.82/9.83/11.93 | 0:21.08; 9.48/10.70/11.45 |
   | `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` | 0:34.09; 11.33/10.04/11.96 | 0:36.93; 9.95/10.76/11.46 |

   Timing logs and uptime captures are under
   `target/english-v3/english-v3-emit-rules-as-functions/` (`before-*`, `edit-*`,
   and `after-census.*`).

   Post-refresh bounded census (`zmxzpyny`): 22.85 s wall, 6 workers,
   samples-per-face 0, under `/usr/bin/time -v`; uptime load 8.74/12.45/12.43.
   Evidence: `refresh-census.{time,load,log}`. No census `--all` or Lean check
   was run. Orchestrator review accepted Goal 1 and authorized integration on
   2026-10-10.

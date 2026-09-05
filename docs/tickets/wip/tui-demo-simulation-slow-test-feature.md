---
needs: []
---
# Move the TUI demo auto-play simulation behind a `slow-tests` feature

`game::tests::demo_auto_plays_to_completion`
(`crates/deckmaste_tui/src/game.rs:330`) runs 130–236 s on the dev host
(measured 2026-09-05; high variance) — about 45% of the whole
`cargo test --workspace` wall time. No other test in any crate exceeds 20 s.
Every english_v2 landing whose gate scope is the workspace suite pays for
auto-playing a demo game.

Pinned shape: a `slow-tests` cargo feature on `deckmaste_tui` (default off)
gating this test with `#[cfg(feature = "slow-tests")]` — not `#[ignore]`
(assurance: `#[ignore]` needs a named blocker; this is a schedule, not a
blocker) — plus one documented invocation that runs it
(`cargo test -p deckmaste_tui --features slow-tests`), wired into CI's full
job so the test still runs on every trunk change, and a one-line note in
CLAUDE.md's gate-scope section that `cargo test --workspace` excludes
`slow-tests` and CI runs them. If the test's variance comes from a
nondeterministic seed, say so in the landing record (do not fix it here).

Fences: deleting or weakening the test; `#[ignore]`; any change to what the
test asserts. Consumption boundary: `crates/deckmaste_tui`, CI config,
CLAUDE.md. Standard constraints apply.

## Landing record

Measured on refreshed change `zxwtnrqroovp` (refresh was a no-op) with
`CARGO_BUILD_JOBS=8` on the shared host.

- Added default-off `deckmaste_tui/slow-tests`; only
  `game::tests::demo_auto_plays_to_completion` is conditional on it. The test
  and its assertion are retained. CI's full job runs the feature-enabled TUI
  suite, while the normal workspace suite leaves the feature disabled.
- Gates: `cargo fmt --all`; both requested TUI clippy invocations with
  `-D warnings`; YAML parsing with PyYAML; and `cargo test --workspace` all
  passed. The default test listing does not contain the gated test.
- Timing: pre-change `cargo test --workspace` was 486 s; refreshed
  feature-default `cargo test --workspace` was 332 s (154 s less under shared
  host load). Feature-enabled TUI runs took 256.43 s and 234.18 s for the
  76-test unit target (22.25 s variance); both ran and passed the demo
  simulation. Its test seed is fixed, so this is scheduling/load variance, not
  a nondeterministic seed.
- Corpus figures: coverage lock, construction count, selection census, newly
  covered identities, and performance advisory are unchanged (no grammar or
  xtask change); no english_v2 corpus gates were run.
- Assurance counts: restored 0; re-spelled 0; ignored 0; added 0; removed 0.
  The existing test is feature-gated, not ignored, deleted, or weakened.
- Deviations and additions: none. STOP: none. Glossary gap: none.

### Review corrections

Re-gated on the refreshed tree: change `zxwtnrqroovp`, working-copy content
`e4bd665c` after `kata refresh` pulled in `english-v2-tail-restrictive-focus-adverb`
(the implementer's no-op refresh predates that landing, so its numbers were
measured on an earlier tree; the re-gate below supersedes them). No corpus gate
applies — the diff touches no grammar, catalog, or xtask path, so the coverage
lock, construction count and selection census are untouched and none was run.

- Finding (MEDIUM, record accuracy — fixed here, remainder routed): the ticket's
  premise that no other test exceeds 20 s is false, and the record did not say
  where the remaining wall time goes. Feature OFF, the `deckmaste_tui` lib target
  is 203.71 s of a 310 s `cargo test --workspace` — about two thirds of the run.
  Feature ON the same target is 287.80 s, so the gated demo test is worth ~84 s
  of workspace wall time, not the ~45% the ticket assumed. Two more whole-game
  simulations stay in the default set and each trips libtest's 60-second notice:
  `driver::tests::auto_play_produces_only_legal_decisions` and
  `interact::tests::interactive_path_produces_only_legal_decisions`. Extending
  the gate to them is beyond this ticket's pinned shape (one named test), so it
  is routed to `docs/tickets/planned/tui-remaining-slow-simulation-tests.md`
  rather than done here.
- Gate artifacts (this review, foreground, `CARGO_BUILD_JOBS=8`): `cargo fmt
  --all` clean; `cargo clippy -p deckmaste_tui --all-targets -- -D warnings` and
  the same with `--features slow-tests` both clean; `cargo test --workspace`
  128 targets, 6178 passed, 0 failed, 6 ignored, 310 s wall; `cargo test -p
  deckmaste_tui --features slow-tests` 76 passed, 0 failed, 287.80 s, with
  `demo_auto_plays_to_completion ... ok`; `--list` shows 75 tests without the
  feature and the gated test absent, 76 and present with it; `.github/workflows/
  ci.yml` parses (PyYAML via `uvx`) and the new step is step 16 of 19 in the
  `clippy-test` job, between `test` and the noncanon WC99 gate.
- Performance advisory: measured under real contention — 2 concurrent executors
  plus this review on the shared 24-core host. Workspace-suite wall 310 s
  (implementer measured 332 s pre-refresh; 486 s before the change). The
  slow-tests suite was executed 3 times in total across the landing (implementer
  2, review 1); no per-byte coverage telemetry line applies, as no corpus
  command ran.
- Assurance counts re-checked against the diff: restored 0; re-spelled 0;
  ignored 0; added 0; removed 0 — the diff adds one `#[cfg]` line and changes no
  test body. Confirmed the seed claim: `game::build_game` calls
  `build_game_with_seed(SEED)` with the fixed suite seed, so the 22 s spread
  between runs is host load, not a nondeterministic seed.
- Deviations and additions (review): one planned ticket minted,
  `tui-remaining-slow-simulation-tests`. STOP: none. Glossary gap: none.

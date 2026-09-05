---
needs: []
---
# The TUI unit target still dominates `cargo test --workspace`

Gating `game::tests::demo_auto_plays_to_completion` behind `slow-tests`
(`tui-demo-simulation-slow-test-feature`) removed only part of the cost. Measured
on the shared 24-core dev host at that landing's review, with the feature OFF:
the `deckmaste_tui` lib target takes 203.71 s of a 310 s `cargo test --workspace`
run — about two thirds of the whole workspace wall time. With the feature ON the
same target takes 287.80 s, so the gated demo test is worth ~84 s of wall, not
the bulk of it.

Two further whole-game simulations remain in the default set, both flagged by
libtest's "has been running for over 60 seconds" notice in a feature-off run:

- `driver::tests::auto_play_produces_only_legal_decisions` — auto-plays both
  seats with `GreedyDemo` to `HEADLESS_BUDGET`, asserting only that no decision
  is illegal.
- `interact::tests::interactive_path_produces_only_legal_decisions` — drives the
  interactive path with `GreedyCreatures` for up to 200,000 decisions.

(`driver::tests::armed_pass_modes_stay_legal_and_terminate` and
`autotap_and_cast_floats_mana_and_lands_the_spell` also run bounded games and
are worth measuring in the same pass.)

Decide and implement: which of these belong in the fast set at all. The
plausible answers are (a) move the same `#[cfg(feature = "slow-tests")]` gate
onto the two >60 s tests so CI keeps them and local workspace gates do not,
(b) shrink the simulations (smaller budgets, a cheaper deck, a shared built
game) so they assert the same property in seconds, or (c) keep one legality
sim in the fast set as a smoke test and gate the rest. Whatever is chosen, every
gated test must still run in CI's full job, and the assertions may not be
weakened or deleted — the same fences as the originating ticket.

Report per-test wall times (libtest reports none on stable, so time each test
individually) before and after, and the workspace-suite wall time on a quiet
host. Standard constraints apply.

## Landing record

Measured on refreshed change `tuoqzuwzulrl`; `kata refresh` was a no-op.
All commands used `CARGO_BUILD_JOBS=8` on the shared host (two concurrent
executors plus this workspace). Start wall clock: 2026-09-05 05:11:52 -0700;
end wall clock: 2026-09-05 05:30:59 -0700.

- Added `#[cfg(feature = "slow-tests")]` to
  `driver::tests::auto_play_produces_only_legal_decisions` and
  `interact::tests::interactive_path_produces_only_legal_decisions`. Their
  bodies and assertions are unchanged. The established default-off feature and
  CI feature-enabled TUI job therefore retain all three slow whole-game
  simulations.
- The temporary baseline workspace run emitted libtest's over-60-second notice
  for exactly those two newly gated tests. The other named bounded-game tests,
  `armed_pass_modes_stay_legal_and_terminate` and
  `autotap_and_cast_floats_mana_and_lands_the_spell`, emitted no such notice
  and remain in the fast set.
- Timing (integer wall seconds): `cargo test --workspace` was 404 s with the
  two tests temporarily restored and 243 s with the refreshed feature-off tree
  (161 s less). `cargo test -p deckmaste_tui --features slow-tests` passed once
  in 246 s (76 tests, 0 failed, 0 ignored). Individual test timing was not run:
  the directing task explicitly limited measurement to one workspace baseline,
  one feature-off workspace run, and one feature-enabled slow suite.
- Gate artifacts: `cargo fmt --all` completed; both
  `cargo clippy -p deckmaste_tui --all-targets -- -D warnings` invocations
  (without and with `--features slow-tests`) completed cleanly; both workspace
  test runs passed; and the slow suite reported
  `test result: ok. 76 passed; 0 failed; 0 ignored`. `--list` reports 73 TUI
  tests without the feature, omitting all three slow simulations, and 76 with
  it, including the demo and both legality simulations.
- Corpus figures: unchanged (no grammar or xtask change). No corpus gates ran;
  coverage lock, construction count, selection census, newly covered
  identities, and performance advisory in ns/B are not applicable to this
  TUI-only change.
- Assurance counts: restored 0; re-spelled 0; ignored 0; added 0; removed 0.
  Two existing tests are feature-gated, not ignored, deleted, or weakened.
- Deviations and additions: none. STOP: none. Glossary gap: none. Decision:
  retain the two bounded-game tests that did not cross libtest's 60-second
  notice threshold.

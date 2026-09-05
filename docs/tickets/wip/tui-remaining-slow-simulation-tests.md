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

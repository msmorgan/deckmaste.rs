---
needs: []
---
Two CI coverage holes remain in the noncanon strategy suite:

1. **The noncanon keep-green gate never runs in CI.** The WC99 suite
   (`crates/deckmaste_noncanon/tests/wc99.rs`) is behind the non-default
   `noncanon_tests` feature, which ci.yml never passes. The crate-level
   `#![cfg(feature = "noncanon_tests")]` compiles the whole file out, so the
   subset gate and the only assertion exercising RON-driven opponent
   targeting (`spell_damage_to_players > 0`) are dead in CI; only the
   single-seed smoke tests in `src/game.rs` run. Add
   `--features noncanon_tests` (or a feature-matrix entry) to the CI test
   step; keep the `#[ignore]`d 50-game full matchup local.

2. **The among-filter seat anchoring has no pinning test.** The
   strategy-evaluator fix (among-filter resolves `You`/`Opponent` from the
   acting seat, `strategy.rs`) is correct for all callers but landed with
   zero tests using a non-`None` `among`. Reverting it breaks nothing
   ungated because the only behavioral assertion lives in the CI-dead suite
   above. Add a strategy unit test with a non-`None` `among` that pins the
   seat anchoring (`Ref(This)` inside `among` is the seat's proxy, not the
   candidate).

Acceptance: the focused unit test runs in the default crate suite, the WC99
subset runs in CI with a fixed seed, and the intentionally expensive 50-game
matchup remains opt-in.

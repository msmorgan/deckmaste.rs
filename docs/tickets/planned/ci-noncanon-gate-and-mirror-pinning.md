---
needs: []
---
Two CI coverage holes in the noncanon strategy suite. The data-mirror pinning
and missing generated-fixture work was split into
`ci-derived-data-fixtures`.

1. **The noncanon keep-green gate never runs in CI.** The WC99 suite
   (crates/deckmaste_noncanon/tests/wc99.rs) is behind the non-default
   `noncanon_tests` feature, which ci.yml never passes — the crate-level
   `#![cfg(feature = "noncanon_tests")]` compiles the whole file out, so the
   subset gate and the only assertion exercising RON-driven opponent
   targeting (`spell_damage_to_players > 0`) are dead in CI; only the
   single-seed smoke tests in src/game.rs run. Add
   `--features noncanon_tests` (or a feature-matrix entry) to the CI test
   step; keep the `#[ignore]`d 50-game full matchup local.

2. **The among-filter seat anchoring has no pinning test.** The
   strategy-evaluator fix (among-filter resolves `You`/`Opponent` from the
   acting seat, `strategy.rs`) is correct for all callers but landed with
   zero tests using a non-`None` `among` — reverting it breaks nothing
   ungated, because the only behavioral assert lives in the CI-dead
   feature-gated suite above. Add a strategy unit test with a non-`None`
   `among` that pins the seat anchoring (and `Ref(This)`-in-among = the
   seat's proxy, not the candidate).

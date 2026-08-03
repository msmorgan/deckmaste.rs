---
needs: []
---
Three CI coverage holes plus a hermeticity hazard around the data mirror:

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

3. **Mirror drift.** The CI data-mirror checkout is unpinned (no `ref:`)
   against a repo documented as self-refreshed weekly, and CI generates the
   corpus with `--minimal` while local corpora are full-stub — two sources
   of trunk flipping red/green with no repo change (an ungraduated card
   panics `CardSource::card` in the ungated smoke test). Pin the mirror
   checkout to a commit (with a documented bump procedure), and either
   document the `--minimal`-vs-full divergence as accepted or close it.

4. **62 corpus-parsing tests are `ignored` in CI, across two fixtures.**
   Both were *unguarded* and panicked the runner on every push until the
   build.rs gates landed (`deckmaste_frames`, then `xtask`); `cargo test`
   fail-stops at the first failing binary, so the second fixture only
   surfaced once the first was gated. Reproduce either by staging a `data/`
   holding **only** `catalogs`, `rules`, and `mtgjson/AtomicCards.json` —
   exactly what ci.yml stages — then `cargo test --workspace
   --no-fail-fast`.

   - **`gen_catalogs`** — `data/gen/catalogs`, written by `cargo xtask
     catalogs` from `data/rules/cr.txt`. 37 frames unify/render + 4 frames
     pilot + 13 xtask tests. The mirror carries no CR text snapshot
     (`fetch_data --minimal` takes `cr/keywords`, not `link/cr`), and
     committing one is **barred** — rule text is never committed, only
     derived artifacts.
   - **`derived_cards`** — `data/derived/cards.jsonl`, the 35k-row oracle
     snapshot (`deckmaste_plugin::fidelity::ORACLE_SNAPSHOT`). 8 xtask
     tests, plus the already-skipping `deckmaste_plugin` fidelity gate.

   **`cards.jsonl` is the recoverable one, and should be recovered.** It is
   a flat per-face projection of mtgjson AtomicCards carrying that dump's
   own field names (`manaCost`, `manaValue`, `colorIdentity`,
   `supertypes`) — produced once by an ad-hoc jq transform and then read
   throughout the tree by accretion; nothing in the repo writes it. CI
   already decompresses `AtomicCards.json`, so an xtask subcommand
   reproducing that projection would un-gate all 8 tests, retire an
   undocumented hand-made dependency, and commit nothing. Do this one.

   The catalogs are the harder half — the twelve outputs are bare name
   lists (~7 KB, the same class of derived artifact the mirror already
   ships as `keywords.json` and the Scryfall dumps), so either:

   - **Runtime fetch.** ci.yml pulls `link/cr` into the gitignored `data/`
     and runs `cargo xtask catalogs`; nothing is committed. One step, but
     it puts api.academyruins.com on trunk's critical path — the same
     red/green-with-no-repo-change hazard as item 3.
   - **Mirror-side generation.** The mirror's refresh workflow fetches
     cr.txt into its own gitignored tree and commits only the twelve
     derived lists. Hermetic, and CI just stages them like the rest — but
     it needs the deckmaste toolchain in a repo that is currently a fish
     fetch script.

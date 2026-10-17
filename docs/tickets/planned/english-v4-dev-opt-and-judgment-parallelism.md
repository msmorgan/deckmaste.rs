---
needs: []
---
# Optimize `english-v4` dev profile and parallelize judgment evaluation

Speed up `cargo xtask english-v4` census runs and `cargo test -p
deckmaste_english_v4 --test judgments`:

1. **Add `[profile.dev.package]` `opt-level = 2` overrides in root `Cargo.toml`:**
   Root `Cargo.toml` configures `opt-level = 2` for `deckmaste_english`,
   `deckmaste_english_v3`, `deckmaste_lexical`, and `deckmaste_lexical_source`,
   but omits the `v4` stack, leaving the chart parser, packed forest, and
   round-trip verifier running at `opt-level = 0` under `cargo test` and
   `cargo xtask`. Add `opt-level = 2` overrides for:
   - `deckmaste_syntax`
   - `deckmaste_english_v4`
   - `deckmaste_lexical_source_v4`
2. **Share the loaded lexical environment and bound grammar and parallelize
   `check_judgments` in `crates/deckmaste_english_v4/tests/judgments.rs`:**
   - Cache `LexicalEnvironment::load()` (and the bound `CompiledGrammar`) in a
     `std::sync::OnceLock` inside `tests/judgments.rs` instead of reloading all
     30k+ card names into the trie and re-binding the grammar in four separate
     `#[test]` functions (`judgments_at_or_below_the_completed_stratum_hold`,
     `catalog_determinative_judgments_hold`,
     `every_word_a_judgment_names_is_declared`, and
     `stratum_zero_spines_hold_over_seed_payloads`).
   - Parallelize `check_judgments` across `std::thread::available_parallelism()`
     (matching `deckmaste_english_v4::census::run`) while preserving deterministic
     failure ordering and `enforced` / `pending` counts.

## Gate

- `cargo test -p deckmaste_english_v4` and `cargo clippy -p deckmaste_english_v4 --tests -- -D warnings` pass.
- `cargo xtask english-v4 strata --check` passes with identical census counts.

Standard constraints apply.

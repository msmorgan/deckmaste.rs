---
needs: []
---
A runnable, narrated example that resolves one hard interaction step by step,
printing the relevant `[CR#…]` as it goes — the engine's "see it work" piece for
the README. Add it under `crates/deckmaste_engine/examples/` (a `cargo run
--example …`, alongside `full_game_1k.rs`); pick a layers or replacement-effect
minefield, build the state in-Rust so it needs no data download, and print each
step with its rule reference.

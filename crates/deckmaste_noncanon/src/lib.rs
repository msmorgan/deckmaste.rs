//! Noncanon matchup harness: real decks played through the engine's decision
//! API by honest archetype pilots with tactical lookahead.
//!
//! "Noncanon" = real card encodings copied from generator output and
//! hand-graduated here, outside the blessed `canon` set. The matchup tests
//! under `tests/` are gated by the `noncanon_tests` feature:
//!
//! ```sh
//! cargo test -p deckmaste_noncanon --features noncanon_tests
//! ```

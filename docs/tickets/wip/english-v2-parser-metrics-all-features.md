Finish the parser-metrics all-features fix (cfg-warning landing review
M1). `cargo test -p deckmaste_construction --all-features` still fails
1 of 28 trybuild cases: `structural_checked_constructor_accessor_
collision.rs` also `include!`s `parser/engine.rs` (line 27) and lacks the
feature-gated no-op `mod metrics` sink that `tests/compiled_consumer.rs`
gained — 11x `E0433: cannot find metrics in super`. Add the same gated
sink there (cfg'd out by default, so no `.stderr` snapshot can move) and
make the landing record's all-features evidence a command that actually
runs trybuild (`cargo test -p deckmaste_construction --all-features`),
not `cargo check`. Zero grammar/runtime change; standard constraints
apply.

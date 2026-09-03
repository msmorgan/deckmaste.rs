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

## Landing record (2026-09-03)

Measured on change `suwsxmwk` with 16,237 covered lock identities.

- Failure before/after: before, `cargo test -p deckmaste_construction
  --all-features` failed 1 of 28 trybuild cases because
  `structural_checked_constructor_accessor_collision.rs` emitted 11
  `E0433: cannot find metrics in super` diagnostics in addition to its intended
  collision diagnostic. After adding the feature-gated no-op sink, all 28
  cases pass and the existing `.stderr` snapshot is byte-unchanged.
- Coverage and construction count: 16,237 -> 16,237 selected and covered
  units; 16,404 parse failures; 378 -> 378 construction declarations. Every
  selected-uncovered, unresolved-tie, internal-failure, exception-use,
  round-trip, ownership, gap, overlap, synthetic-claim, and
  provenance-plan-mismatch counter remains zero.
- Selection census: 10,731 unique and 5,506 specificity-resolved selections,
  unchanged by this fixture-only edit.
- Coverage lock: current and byte-unchanged at 48,887 lines, SHA-256
  `482785cac240a055b484152861421caf19799e10fdc6667ce239db0df78d66a8`.
- Assurance: restored 0; re-spelled 0; ignored with blockers 0; added 0;
  removed 0. The existing negative fixture and its exact collision diagnostic
  remain the oracle.
- Positive artifacts: `cargo test -p deckmaste_construction --all-features`
  (including all 28 trybuild cases and 38 compiled-consumer tests), `cargo test
  -p deckmaste_construction`, `cargo clippy -p deckmaste_construction
  --all-targets --all-features -- -D warnings`, `cargo xtask english_v2
  coverage --check`, `cargo xtask english_v2 ambiguity --require-resolved`,
  and `jj fix` are green. The ambiguity gate emitted only its existing
  common-path wall-clock warning and completed successfully.
- Deviations and additions: the sink is placed after the diagnostic-triggering
  construction so adding its source does not move the line pinned by the
  default-feature `.stderr` snapshot. No grammar, runtime, construction,
  corpus, lock, or test-count change. No scratch or probe tree was created.
- STOPs: none.

Silence the `parser-metrics` cfg warning that reds the workspace suite.
Since the perf rounds, `cargo test --workspace` fails: four of 28
`deckmaste_construction` trybuild expected-stderr snapshots receive an
unexpected-cfg warning for the `parser-metrics` feature (the feature is
declared in xtask but referenced by `cfg(feature = "parser-metrics")` in
generated/compiler code compiled in crates that do not declare it). Fix
the cause, not the snapshots: declare the feature where the cfg is
evaluated (or use `--check-cfg` names / a `cfg_attr` gate in the emitting
crate) so no warning is emitted in any crate; trybuild snapshots then
pass unchanged. `cargo test --workspace` must be green; zero grammar or
runtime change. Landing record states the exact warning text before and
its absence after. Standard constraints apply.

## Landing record (2026-09-03)

- Warning before/after: before, four of the 28
  `deckmaste_construction` trybuild cases received ``warning: unexpected
  `cfg` condition value: `parser-metrics` `` ahead of their expected
  diagnostics; after declaring `parser-metrics` in the destination package,
  all 28 snapshots pass unchanged and that warning is absent from both the
  focused and full-workspace test output.
- Coverage and construction count: selected and covered units remain
  16,237 -> 16,237; parse failures remain 16,404; every selected-uncovered,
  unresolved, internal, round-trip, ownership, gap, overlap, synthetic, and
  provenance counter remains zero. Constructions remain 378 -> 378.
- Coverage lock: byte-unchanged at 48,887 lines, SHA-256
  `482785cac240a055b484152861421caf19799e10fdc6667ce239db0df78d66a8`.
- Assurance: restored 0; re-spelled 0; ignored with blockers 0; added 0;
  removed 0. The diagnostic snapshots themselves are unchanged.
- Positive artifacts: `cargo test --workspace` is green, including the
  formerly failing 28-case construction trybuild suite; `cargo clippy
  --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`,
  `cargo check -p deckmaste_construction --all-features --all-targets`, and
  `cargo xtask english_v2 coverage --check` are green.
- Deviations and additions: the construction package's shared parser fixture
  gains a feature-gated no-op metrics sink. This keeps the newly declared
  feature valid under an all-features test build without adding production
  instrumentation or changing parser behavior. No scratch or probe tree was
  created.
- STOPs: none.

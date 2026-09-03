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

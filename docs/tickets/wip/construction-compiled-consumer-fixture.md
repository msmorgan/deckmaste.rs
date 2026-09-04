Fix the workspace-suite red in `deckmaste_construction`'s compiled-consumer
fixture. `cargo test --workspace` fails at
crates/deckmaste_construction/tests/compiled_consumer.rs:406 —
`declaration_noun_features` is missing — reproduced by the adjunct-class
executor before refresh, after refresh, and after integration, so it
predates that landing (likely the locative re-issue's class-level noun
licences added a generated item the fixture does not provide). First
confirm on the parent of `onpzrwsz` which landing introduced it; then fix
the fixture (or the emitter, if the generated item is missing a provider
in the consumer contract) so `cargo test --workspace` is green — no
snapshot loosening, no `#[ignore]`. Zero grammar/runtime change; landing
record states the failing test output before and the green run after,
with the measured-tree stamp. Standard constraints apply.

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

## Landing record

Measured on change `ypmzsstm` with 16,555 covered lock identities.

- Coverage: 16,555 -> 16,555 selected and covered units; parse failures
  16,086 -> 16,086; lock diff **+0/-0 rows**. The schema-4 lock retained
  source fingerprint
  `e85359d7b8c578df13dff2fdf7c743a520a5b367d5ed25ab0a5f03cb8b3637dd`
  and SHA-256
  `074a2a5ed71e5b131a9d3ade9727cab974215df98b9f913cbd6ac1590a4993c0`.
- Selection census: unique 11,063 -> 11,063; specificity-resolved 5,492 ->
  5,492; exception-resolved 0 -> 0; unresolved ties 0 -> 0; internal
  failures 0 -> 0; exception uses 0 -> 0.
- Construction declarations: 394 -> 394. Literal/lexicon collisions: 59 ->
  59.
- Regression provenance: annotation on the parent of `onpzrwsz` attributes
  the generated `declaration_noun_features` call to change `nnmwmukz`
  (`english-v2: enforce noun attachment licences`). The pre-fix
  `cargo test --workspace` failed with `E0599` at
  `crates/deckmaste_construction/tests/compiled_consumer.rs:406`, reporting
  that `ParserEnvironment` had no `declaration_noun_features` method.
- Gates: the focused compiled-consumer suite passed all 38 tests;
  `cargo xtask english_v2 coverage --check` and
  `cargo xtask english_v2 ambiguity --require-resolved` exited 0 with no
  lock drift or unresolved ties. The refreshed final
  `cargo test --workspace` rerun
  exited 0, including 417 passing xtask library tests and the complete
  workspace doc-test set. `cargo fmt --all -- --check` and strict workspace
  all-target clippy also exited 0; fmt emitted only the stable-toolchain
  warnings for nightly-only repository settings.
- Assurance census: restored 0; re-spelled 0; ignored 0; added 0; removed 0.
  No snapshot was loosened and no grammar or runtime code changed.
- STOPs: none.

### Deviations and additions

- Updated the xtask coverage authenticator's exact collision count from 60
  to 59. The adjunct-class landing deliberately removed one collision and
  recorded the 60 -> 59 change, but its full workspace run stopped first at
  the compiled-consumer error and therefore did not expose this stale
  expectation. The isolated authenticator passes with the recorded value.

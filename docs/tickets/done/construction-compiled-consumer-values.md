Make the compiled-consumer noun-feature mock value-bearing
(compiled-consumer landing review M1). The fixture's new
`declaration_noun_features` mock is value-inert: returning constant
`Unlicensed`/`NonRelational`, or flipping `number_invariant`, leaves all
38 tests green — only `Some -> None` is caught. Add one licensing /
non-licensing declaration pair and one number-invariant pair whose
generated behaviour the tests assert at value level, so the emitter's
host contract is authenticated, not just linked. Zero grammar/runtime
change; gate on `cargo test --workspace` per CLAUDE.md. Standard
constraints apply.

## Landing record

Measured on change `rmlrslurskqx` with 16,702 covered lock identities.

- Coverage: 16,702 -> 16,702 selected and covered units; parse failures
  15,939 -> 15,939; selected-uncovered, unresolved ties, internal failures,
  round-trip mismatches, and ownership failures remain 0. The schema-4 lock
  retained source fingerprint
  `e85359d7b8c578df13dff2fdf7c743a520a5b367d5ed25ab0a5f03cb8b3637dd`
  and SHA-256
  `837688617caf0b0ff0c3e6551fcf2d58772e463f71040c1c42e6d51f2cb26446`.
- Selection census: unique 11,138 -> 11,138; specificity-resolved 5,564 ->
  5,564; exception-resolved 0 -> 0; unresolved ties 0 -> 0; internal failures
  0 -> 0; exception uses 0 -> 0.
- Construction declarations: 398 -> 398. Literal/lexicon collisions: 59 ->
  59.
- Value authentication: the synthetic `Relic` declaration is unlicensed,
  non-relational, and number-invariant; `Elf` remains object-attachment
  licensed, qualified-relational, and number-varying. The generated
  `DeclarationNoun` accessors are compared against all six expected values.
  Temporarily replacing the callback result with constant
  `Unlicensed`/`NonRelational` made the focused test fail on `Elf`'s licence;
  temporarily inverting `number_invariant` made it fail on `Relic`'s value.
  Both mutations were reverted before the green gates.
- Gates: the focused compiled-consumer suite passed all 38 tests;
  `cargo test --workspace` passed, including all 28 construction trybuild
  fixtures, 417 xtask library tests, and the complete doc-test set;
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo xtask english_v2 coverage --check`, and
  `cargo xtask english_v2 ambiguity --require-resolved` exited 0. `jj fix`
  reported no remaining formatting changes.
- Assurance census: restored 0; re-spelled 0; ignored 0; added 0; removed 0.
  Existing test coverage was strengthened in place; no test was deleted,
  weakened, or ignored.
- STOPs: none.

### Deviations and additions

- Added only fixture-side noun-class override data and a same-surface plural
  for the synthetic invariant noun, as requested. No production grammar,
  runtime behavior, construction declaration, or file outside the
  `deckmaste_construction` compiled-consumer fixture and this ticket changed.

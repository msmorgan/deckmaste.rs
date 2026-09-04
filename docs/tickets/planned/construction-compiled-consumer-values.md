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

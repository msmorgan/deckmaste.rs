---
needs: [english-ast-wrapper-dedup]
---
**Convert the debug-string assertion idiom in
`crates/deckmaste_english/tests/public_api.rs` (plus the in-crate uses in
`grammar/clause.rs` and `grammar/ability.rs` test modules) to typed
accessors or `matches!`-style structural assertions.** The integration
suite asserts by `format!("{:#?}")` substring, including
whitespace-stripped signatures like
`compact(&ast).contains("Subordinate(Until,Finite(")`. These break on any
`Debug` rename, and loose prefixes (`.contains("Coordinated(")` matches
several `Coordinated*` types) can pass on the wrong structure. The crate's
unit tests already use the sound idiom.

Gated transitively on [[english-ast-grouping]] through
[[english-ast-wrapper-dedup]]: both reshape the AST and would break debug
strings regardless — convert once, after the reshuffle settles. The recovery
campaign keeps adding tests in the string idiom until then; accepted (cheap to
write, converted mechanically here). Gate: exact test-count parity, and each
converted assertion must still fail on the structure it guards (spot-verify by
temporarily disabling one guarded mechanism). Standard constraints apply.

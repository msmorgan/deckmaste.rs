Compiler-side "final constituent" primitive (quoted-terminator review F2).
`sentence_ends_in_quoted_block()` is ~200 lines of hand-written walker (16
functions over ~50 AST arms) mirroring the grammar's rightmost spine —
sound today (`_ => false` fails closed, coordinations take `.last()`) but
silently incomplete: a future construction in a final position gets no
quote termination and nothing detects it (the lock fails on units lost,
never on units never gained). Emit the primitive from the declaration
compiler: for every construction, the generated code knows which role is
its final constituent from the form; expose a derived predicate
("rightmost leaf is <category>") that constructions can `checked by`
without hand-mirroring the spine. Replace the walker with it; pin the
soundness property (a negative that reaches the walker — two `is_err()`
lines close F3). Zero grammar change; `expand` differs only by the
emitted primitive. Standard constraints apply.

## Landing record

Measured on change `stkxzopqyyltwqwuuquzxuwxkklnntym` with 16,702 covered
lock identities. The measured lock has 49,352 lines and SHA-256
`837688617caf0b0ff0c3e6551fcf2d58772e463f71040c1c42e6d51f2cb26446`.

| gate | refreshed parent | measured tree |
| --- | ---: | ---: |
| selected and covered units | 16,702 | 16,702 |
| ordinary parse failures | 15,939 | 15,939 |
| unique selections | 11,138 | 11,132 |
| specificity-resolved selections | 5,564 | 5,570 |
| unresolved ties | 0 | 0 |
| construction declarations | 398 | 398 |
| coverage-lock identities | 16,702 | 16,702 |

- Coverage and lock state: the refreshed parent and measured tree have the
  same 16,702 identities, line count, and lock hash; no lock rows or
  construction declarations changed and no retirement manifest was created
  or used. Selected-uncovered units, internal failures, exception resolutions,
  exception uses, round-trip mismatches, ownership failures, gaps, overlaps,
  synthetic claims, and provenance-plan mismatches are all zero. Literal /
  lexicon collisions remain 59.
- Selection census: six formerly unique units are now specificity-resolved:
  Pestbrood Sloth, Pest Summoning, Ratcatcher Trainee // Pest Problem, Synapse
  Necromage, Wildfire Awakener, and Wolf's Quarry. The declaration-derived
  rightmost spine admits a quote-terminated token-creation analysis the
  handwritten walker omitted; specificity selects the same locked identity
  and exact rendering in every case. This is the ticket's intended closure of
  the walker's silent-incompleteness defect, not a declaration-grammar change.
- Expansion audit: a direct diff between the refreshed-parent and measured
  `english_v2 expand` outputs contains only the generated `RightmostLeaf` and
  `RightmostLeafCategory` traits, `rightmost_leaf_is` predicate and their
  declaration-derived implementations, plus the two checked-callback
  substitutions that consume the primitive.
- Positive artifacts after refresh: `cargo fmt --all -- --check`; `cargo
  clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace`;
  the complete `deckmaste_construction_core`, `deckmaste_construction`, and
  `deckmaste_english_v2` suites; `cargo xtask english_v2 ambiguity
  --require-resolved --json`; `cargo xtask english_v2 coverage --check`; and
  `cargo xtask cite check` all exited zero. The corpus gates emitted only their
  non-failing common-path performance warning while the shared host was above
  the quiet-host criterion.
- Assurance census: restored 0; re-spelled 0; ignored with blockers 0; added
  2 compiler test functions and 1 negative witness assertion; removed 0. The
  emitter tests authenticate automatic discovery for a new construction and
  rightmost recursion through sums, products, optionals, and sequences. The
  existing quote-boundary test now contains the required two `is_err()`
  assertions for an ordinary periodless final sentence and the same failure
  after a preceding complete sentence.
- Deviations and additions: no construction or test was added or deleted
  beyond the ticket. The emitted namespace reserves the two generated trait
  names and predicate name, and the emission-plan counts increase only for the
  primitive's items. The English follow-up is limited to replacing the two
  checked callbacks and deleting the 16-function handwritten walker.
- STOPs: none. The required refresh merged the concurrent `constructions.rs`
  work cleanly without a recorded conflict, so no Harmony repair was needed.

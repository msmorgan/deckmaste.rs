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

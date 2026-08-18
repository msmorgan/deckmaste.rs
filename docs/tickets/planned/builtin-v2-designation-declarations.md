---
needs: [builtin-v2-spelling-stub-design]
---
Port the builtin designation registry rows to committed `builtin_v2`
declarations and give each declaration its semantic `spelling` plus any
English `grammar` facts required by the ratified
[builtin-v2 spelling/grammar ADR](../../decisions/builtin-v2-macro-spelling-and-grammar.md).
Preserve scope and conferrals on the declaration so a plugin can introduce a
designation and use it in same-plugin English without a parser-code edit.

The CR has no central designation catalog. Inventory evidence must therefore
come from the actual builtin declaration manifest and load tests, not an
invented closed word list. These are committed source records, never generated
stubs. Standard constraints apply.

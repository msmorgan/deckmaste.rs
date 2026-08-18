---
needs: [builtin-v2-spelling-stub-design]
---
Port the builtin counter-kind registry rows to committed `builtin_v2`
declarations and give each declaration its semantic `spelling` plus any
English `grammar` facts required by the ratified
[builtin-v2 spelling/grammar ADR](../../decisions/builtin-v2-macro-spelling-and-grammar.md).
Preserve scope and conferrals on the declaration so a plugin can introduce a
new counter kind and use it in same-plugin English without a parser-code edit.

`counter-kind-phrases.txt` is only the CR-defined multi-token keyword-counter
span inventory [CR#122.1b], not a complete counter-kind catalog. Do not use it
as a closed-world declaration list or reject productive single-token counter
kinds for being absent from it. These are committed source records, never
generated stubs. Standard constraints apply.

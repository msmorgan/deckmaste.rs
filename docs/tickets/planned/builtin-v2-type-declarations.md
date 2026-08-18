---
needs: [builtin-v2-spelling-stub-design]
---
Port the builtin card-type registry rows to committed `builtin_v2`
declarations and give each declaration its semantic `spelling` plus any
English `grammar` facts required by the ratified
[builtin-v2 spelling/grammar ADR](../../decisions/builtin-v2-macro-spelling-and-grammar.md).
The declaration remains the single source for type identity and conferrals.

Do not claim a `card-types.txt` bijection: that CR catalog includes types the
current semantic type-line axis does not represent, while `TypeDef` references
are open plugin data. Record the modeled boundary explicitly and keep novel
plugin-declared type names out of hardcoded parser tables. These are committed
source records, never generated stubs. Standard constraints apply.

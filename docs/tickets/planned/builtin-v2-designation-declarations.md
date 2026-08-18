---
needs: [builtin-v2-grammar-consumer]
---
Port the builtin designation registry rows to committed `builtin_v2`
declarations and give each declaration its semantic `spelling` plus any
English `grammar` facts required by the ratified
[builtin-v2 spelling/grammar ADR](../../decisions/builtin-v2-macro-spelling-and-grammar.md).
Preserve scope and conferrals on the declaration so a plugin can introduce a
designation and use it in same-plugin English without a parser-code edit.

Each declaration carries one semantic designation identity, its object/player/
game scope, existing definition and conferrals, its exact identity surface,
and only the grammar facts needed to recognize that surface. Governing phrases
such as `becomes`, `is`, `has`, `takes the initiative`, and `gets` belong to
ordinary or semantic construction frames; `grammar` must not turn the
designation row into a one-off predicate DSL. Follow style guide §14, "Named
and batch terms."

The CR has no central designation catalog, and not every rules term, status,
emblem, count, or marker is a designation. Inventory evidence therefore comes
from the actual builtin declarations and focused behavior tests, not an
invented closed word list or capitalization heuristic. Re-author current
builtin rows in the v2 schema rather than loading v1 files through an adapter.

These are committed source records, never generated stubs. Acceptance covers
at least player-, object-, and game-scoped declarations, a multiword or
article-bearing identity, a declaration with conferred behavior, and a
synthetic same-plugin designation absent from official data. The semantic
validator still rejects a declaration used with the wrong carrier scope, and
every builtin record loads through the v2 declaration reader. Standard
constraints apply.

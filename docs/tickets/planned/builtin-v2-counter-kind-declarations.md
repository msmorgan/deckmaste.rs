---
needs: [builtin-v2-grammar-consumer]
---
Port the builtin counter-kind registry rows to committed `builtin_v2`
declarations and give each declaration its semantic `spelling` plus any
English `grammar` facts required by the ratified
[builtin-v2 spelling/grammar ADR](../../decisions/builtin-v2-macro-spelling-and-grammar.md).
Preserve scope and conferrals on the declaration so a plugin can introduce a
new counter kind and use it in same-plugin English without a parser-code edit.

Each declaration carries one semantic counter identity, its object/player
scope, existing conferrals, the kind phrase that precedes the carrier word
`counter`, and only the grammar facts needed to recognize that phrase. The
declaration does not include the carrier word itself and does not encode the
`put`, `remove`, `get`, or `has` constructions; those are core grammar. Follow
style guide §12, "Counters."

Keep the structured families separate from lexical counter phrases. `+1/+1`,
`-1/-1`, and related power/toughness forms are codec atoms, not noun lexemes;
symbol-bearing forms likewise stay with their codec. Keyword counters use the
approved lowercase keyword phrase and preserve their conferred ability on the
counter declaration [CR#122.1b]. Ordinary single-token and plugin-defined
counter kinds remain open identities.

`counter-kind-phrases.txt` is only the CR-defined multi-token keyword-counter
span inventory [CR#122.1b], not a complete counter-kind catalog. Do not use it
as a closed-world declaration list or reject productive single-token counter
kinds for being absent from it. Re-author the current builtin semantic rows in
the v2 schema; do not load v1 files through an adapter. These are committed
source records, never generated stubs. Acceptance covers object- and
player-scoped rows, a conferring keyword counter, a multiword phrase, a
structured P/T codec form, and a synthetic same-plugin single-token kind absent
from every catalog, and loads every builtin record through the v2 declaration
reader. Standard constraints apply.

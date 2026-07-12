---
needs: []
---
Design question (2026-07-12, exploratory — may well close as won't-fix):
should `TypeDef.permanent: bool` become a structured `TypeKind`? Sketch:

    kind: Spell | Permanent{can_attack, can_block, ...}

Today `permanent` is a bare bool read by `is_permanent_spell` (resolve.rs) to
fork battlefield-entry vs resolve; combat capability is NOT a field — it's
conferred data (`Creature.ron`'s `TypeDef.confers` grants
`May(Attack)`/`May(Block)`, default-deny).

Tension to resolve in the design conversation before any restructure: struct
fields like `can_attack`/`can_block` would sit next to — or compete with — the
conferred `May(...)` grants that the combatant work just established as THE
capability mechanism. If `TypeKind` only replaces the spell/permanent fork
(without absorbing capabilities back into fields), it's a smaller, cleaner
question: is a two-variant kind worth more than a bool?

Requires a design pause with the user; do not implement from this ticket as
written.

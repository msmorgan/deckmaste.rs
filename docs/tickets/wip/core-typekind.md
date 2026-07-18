---
needs: []
---
WON'T-FIX (2026-07-18, decided with the user): keep `TypeDef.permanent: bool`.

Rationale:

- `.permanent` has exactly **one** production reader — `is_permanent_spell`
  (`crates/deckmaste_engine/src/resolve/mod.rs`), `.any(|t| t.permanent)` —
  driving a single boolean fork: enter the battlefield vs. resolve-then-graveyard.
  There is no third resolution mode. A bool is the honest, minimal representation
  for that fork.
- The structured `TypeKind`'s only real payoff — capabilities as struct fields
  (`Permanent{can_attack, can_block, …}`) — is exactly what the conferred
  `May(…)` grants replaced ([[composite-types-capability-gating]],
  [[conferrals-sourced-from-ron]]). Reintroducing them as fields creates a second
  source of truth for combat capability competing with `TypeDef.confers` (folded
  in `layer.rs fold_conferred_abilities`).
- Stripped of the capability fields, a two-variant `Spell | Permanent` enum is
  just nominal typing over a bool with no extensibility actually coming, at the
  cost of migrating ~40 fixtures + the `Type::permanent()` const-fn mirror +
  serde/RON authoring. YAGNI — not worth it.

Original framing preserved below for the design history.

---

Design question (2026-07-12, exploratory — may well close as won't-fix):
should `TypeDef.permanent: bool` become a structured `TypeKind`? Sketch:

    kind: Spell | Permanent{can_attack, can_block, ...}

Today `permanent` is a bare bool read by `is_permanent_spell` (`resolve/mod.rs`) to
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

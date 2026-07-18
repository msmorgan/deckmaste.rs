---
needs: []
---
DEFERRED to `maybe/` (2026-07-18, with the user): keep `TypeDef.permanent: bool`
for now — but leave the door open. A structured `TypeKind` might still prove
useful later, so this stays a live speculative item rather than a hard won't-fix.

Why not now:

- `.permanent` has exactly **one** production reader — `is_permanent_spell`
  (`crates/deckmaste_engine/src/resolve/mod.rs`), `.any(|t| t.permanent)` —
  driving a single boolean fork: enter the battlefield vs. resolve-then-graveyard.
  There is no third resolution mode today. A bool is the honest, minimal
  representation for that fork.
- The structured `TypeKind`'s only real payoff — capabilities as struct fields
  (`Permanent{can_attack, can_block, …}`) — is exactly what the conferred
  `May(…)` grants replaced ([[composite-types-capability-gating]],
  [[conferrals-sourced-from-ron]]). Reintroducing them as fields creates a second
  source of truth for combat capability competing with `TypeDef.confers` (folded
  in `layer.rs fold_conferred_abilities`).
- Stripped of the capability fields, a two-variant `Spell | Permanent` enum is
  just nominal typing over a bool with no extensibility actually coming, at the
  cost of migrating ~40 fixtures + the `Type::permanent()` const-fn mirror +
  serde/RON authoring. YAGNI at present.

Revisit if either emerges:

- a card-resolution mode beyond `{enter battlefield, resolve-to-graveyard}`
  appears (a genuine third variant the bool can't express), or
- `permanent`-branching spreads from the single `is_permanent_spell` site to
  several readers, where nominal `TypeKind` typing would start to pay for itself.

Guardrail if reopened: keep it to the spell/permanent fork. Do **not** absorb
capabilities back into fields — combat/casting capability stays conferred data
in `TypeDef.confers`, per the rulings above.

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

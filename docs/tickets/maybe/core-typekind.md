---
needs: [type-def-permanent-type-flag]
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
  (`Permanent{combatant, …}`) — belongs instead in conferred registry data
  ([Types grant capabilities](../../decisions/types-grant-capabilities.md),
  [Conferrals come from registries](../../decisions/conferrals-come-from-registries.md)).
  Reintroducing the Combatant role as a struct field creates a second source of
  truth competing with `TypeDef.confers` (folded in `layer.rs
  fold_conferred_abilities`). The planned `engine-combatant-role` correction
  replaces the current `May(Attack)` proxy with that explicit bundled role; it
  does not move the role into `TypeKind`.
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
capabilities back into fields — the Combatant role and other capabilities stay
conferred data in `TypeDef.confers`, per the rulings above.

This item now follows the `type-def-permanent-type-flag` rename: the flag it
would restructure becomes the CR-faithful `TypeDef.permanent_type` first. See
the glossary's [`Permanent Type`, `Spell`, and `Permanent`
definitions](../../contexts/game-model/CONTEXT.md) — Permanent Type is a property of a Card
Type, while Spell and Permanent are nonexclusive *current roles* of an Object.
So any future enum must not be named or shaped as `Spell | Permanent`; that
spelling asserts an exclusivity the rules do not have.

Original framing preserved below for the design history.

---

Design question (2026-07-12, exploratory — may well close as won't-fix):
should `TypeDef.permanent: bool` become a structured `TypeKind`? Sketch:

    kind: Spell | Permanent{can_attack, can_block, ...}

Today `permanent` is a bare bool read by `is_permanent_spell` (`resolve/mod.rs`) to
fork battlefield-entry vs resolve; combat capability is NOT a field — it's
conferred data (`Creature.ron`'s `TypeDef.confers` grants
`May(Attack)`/`May(Block)`, default-deny).

Tension resolved by the later glossary discussion: `Combatant` is a bundled
creature-like role, not `May(Attack) || May(Block)`, and belongs in conferred
registry data rather than this enum. If `TypeKind` only replaces the
spell/permanent fork (without absorbing that role back into fields), it's a
smaller, cleaner question: is a two-variant kind worth more than a bool?

Requires a design pause with the user; do not implement from this ticket as
written.

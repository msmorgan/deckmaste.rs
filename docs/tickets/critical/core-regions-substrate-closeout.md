---
needs: []
---
**Close out stage 1 of [Core is explicit regions](../../decisions/core-explicit-regions.md):
the landing (`core: add explicit region substrate`, 2026-09-02) delivered the
type vocabulary, the validator, the shadow-table and fallback deletions, and
the activation table, but left four ADR laws unmet and one regression.**
`core-regions-discourse` builds on this and must not start on the compat
register layout. Standard constraints apply.

## Scope

1. **Modal resolution regression.** `decide/pending/cast.rs` schedules only
   `mode.effect.body.first()`, and `Block::from(Sequentially(parts))` in
   `core/region.rs` yields multi-element blocks, so a mode spelled as a
   "then" sequence loses every step after its first. Schedule the whole
   block; pin it with a two-step mode test.
2. **Law 4: delete the compat spellings.** `core/reference.rs` reintroduces
   `This`, `You`, `Opponent`, `EventObject`, `EventPatient`, `EventActor`,
   `DefendingPlayer`, and `Target(n)` as consts and a constructor over a
   hardcoded register layout (source 0, controller 1, event roles 2..5,
   targets from 6, `Opponent` as a `u32::MAX - 1` sentinel with its own
   runtime arm); `Selection::Targets` likewise. Roughly 1,500 call sites
   still spell them. Replace every read with `Reg` over a param whose
   provenance lowering assigned, drop the fixed layout and the sentinel,
   and make `OpponentOf` an expression. Rust fixtures spell params
   explicitly or go through semantics and `lower`.
3. **Law 11: collapse `Frame`.** `stack.rs` still carries `source`,
   `controller`, `this`, `defending_player`, and an unchanged `Anaphora`;
   the activation reads values back out of `frame.anaphora`, and
   `resolve/effect.rs` clones the frame by value into child work items 28
   times. The activation record owns the params and defs; `Frame` is the
   activation id, payment state, and the residual discourse record (`it`,
   `that`, `chosen`, `allotment`) that stage 2 removes.
4. **Delete the relocated fallback.** `activation.rs` still answers an
   `EventObject` read from the patient slot. One channel per provenance.
5. **Witness tests and restored assertions.** Add tests for the witness
   cards listed in `docs/tickets/done/core-regions-substrate.md`; restore
   value comparison in `plugin/tests/corpus_identity.rs` (now discriminant
   only), the Tribal Flames cross-check in `canon.rs`, and cycling's
   macro-derived comparison in `keywords.rs`, re-spelled against regions
   rather than dropped.

Law 5 (the decision-in-expression carve-out) is not here; it lands with
`core-regions-discourse`, where decisions become instructions.

## Gates

Engine, core, lowering, and plugin suites green with `plugins/wizards`
regenerated first; a grep proves no compat const or `Target(` constructor
survives in `deckmaste_core`; `Frame` has no `anaphora.targets`, `source`,
`controller`, `this`, or `defending_player` field.

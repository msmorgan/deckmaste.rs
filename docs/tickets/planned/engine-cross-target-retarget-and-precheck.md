---
needs: []
---
Two paths still judge a target slot that reads an EARLIER slot's announced
register without that register's value. Neither is reachable by any card in
`plugins/canon` or `plugins/wizards` (0 cross-referencing target specs in
either), so nothing is broken today; both are the residue
`engine-cross-target-announce-enumeration` named at landing. Standard
constraints apply.

## The gap

- **Castability precheck.** `GameState::announcement_effect_satisfiable` runs
  `legal_targets` with `ActivationId::NONE`, so a cross-referencing slot is
  judged on the null-packed set: a positive cross-reference reads as
  unsatisfiable, a negated one as satisfiable over everything. It cannot use
  `slot_candidates`'s union widening, which needs a register file to write the
  trial into. A spell can therefore be offered whose announcement has no legal
  completion, and the player reaches a prompt they cannot answer.
- **Retarget.** `Retarget::resolve` ([CR#707.10c,115.7d]) validates against the
  per-slot sets enumerated with the entry's CURRENT targets in the register
  file. Changing an earlier slot invalidates a later slot's set, and
  `cross_target_choice_legal` is not called there. [CR#115.7d]'s "must not cause
  any unchanged targets to become illegal" is the rule to build against, and it
  interacts with the union rule that a slot's current target is always keepable.

## Acceptance

A cross-referencing spell with no legal announcement is not offered as
castable, and a retarget that changes an earlier slot re-derives the later
slots it feeds. Fixtures spelled in the `region_witnesses.rs` style, since no
corpus card reaches either path.

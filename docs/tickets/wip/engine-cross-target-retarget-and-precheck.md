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

## Landing record

### Rules and implementation

[CR#601.2c] requires an appropriate object or player for every required
target, so castability now asks whether a COMPLETE telescoping announcement
exists. The ordinary independent-slot path keeps the existing compact
`announce_satisfiable` gate. Only a target list that reads an earlier announced
slot materializes a temporary activation and searches slot subsets in
declaration order, writing each trial prefix before deriving the next slot.
The activation family and its temporarily consumed ids are reclaimed, so a
read-only legal-action probe does not perturb later execution.

For choosing new targets, [CR#115.7e] evaluates only the final set, while
[CR#115.7d] requires every changed target to be legal and forbids a changed
target from causing an unchanged target to become illegal. Retarget menus now
union candidates reachable from fresh earlier choices with candidates under
the current prefix and the current targets themselves. Submission re-derives
each later slot against the proposed prefix. A retained target that was already
illegal remains keepable; one that was legal under the old prefix but illegal
under the proposed prefix is rejected.

### Numbers and assurance

- Corpus reachability remains unchanged: the inherited census is 0
  cross-referencing target specs in `plugins/canon` (0 of 372) and
  `plugins/wizards` (0 of 30,688); neither plugin changed.
- Construction count and coverage locks: unchanged; no construction,
  `english-v2-coverage.lock`, or citation-lock entry changed.
- `region_witnesses`: 17 passed / 2 ignored before; 20 passed / 2 ignored
  after.
- Tests restored: 0; re-spelled: 0; ignored with blockers: 2 unchanged; added:
  3; removed: 0.
- Added witnesses:
  `cross_target_spell_without_a_complete_announcement_is_not_castable`,
  `cross_target_retarget_rederives_later_slots_from_the_proposed_prefix`, and
  `cross_target_retarget_keeps_a_later_target_that_was_already_illegal`.

### Deviations and additions

- Added the third retarget witness beyond the two acceptance paths to pin
  [CR#115.7d]'s explicit exception for an unchanged target that was already
  illegal. This guards the current-target union rule while the cross-prefix
  validator is active.
- The exact satisfiability helper also replaces the set-only check for an
  in-flight modal selection. This is the same precheck contract after a real
  announce activation exists and prevents an impossible cross-target mode from
  being selected even when another mode made the spell initially offerable.
- No plugin/card grammar, core/lowering type, generated wizard, or public API
  changed.

### Gates

- `cargo fmt --all -- --check` — clean.
- `cargo test -p deckmaste_engine` — all suites green: lib 747 passed / 1
  ignored; `region_witnesses` 20 passed / 2 ignored; full-game Monte Carlo 1
  ignored; all other engine integration tests passed.
- `cargo test -p deckmaste_core -p deckmaste_lowering -p deckmaste_plugin` —
  all suites and doc tests green; core 53 passed, lowering unit 738 passed,
  plugin unit 103 passed, 0 failures.
- `cargo clippy -p deckmaste_engine --all-targets -- -D warnings` — clean.
- `cargo xtask cite check --list-noncompliant` — 0 non-compliant.
- `cargo xtask cite check` — 17,702 citations, 0 stale.
- `jj --no-pager diff --git | cargo xtask cite audit --diff` — 12 changed
  citation sites audited against [CR#601.2c,115.7d,115.7e,707.10c].
- `cargo xtask idris-check plugins/canon --differential` — 68 agreed sound, 0
  agreed unsound, 12 skipped, 0 disagreements.

### STOPs

None.

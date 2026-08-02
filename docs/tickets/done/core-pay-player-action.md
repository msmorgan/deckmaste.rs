---
needs: []
---
**Action role reshape: merge `PlayerAction` into `Action` with explicit,
CR-derived role slots; delete `By` and its embed-default sugar; add the
slotless `Pay(Cost)` verb; collapse `MayPay`/`MustPay` into `May { who,
effect: Pay(cost), if_did, if_not }`; consolidate the life family into
`ChangeLife(patient, LifeOp)`.** Design settled 2026-08-01 — the full record
(laws, per-variant table, decider principle, bindings and payment-path
contracts, event-layer migration, seams, rejected alternatives) is
`docs/superpowers/specs/2026-08-01-action-role-reshape-design.md`. This
ticket grew from "make Pay a player action" into the umbrella for that spec;
the original collapse argument survives as spec §5 — [CR#118.12a] itself
defines "[do something] unless [a player does something else]" as
may-do-plus-if-not, so `MustPay` and `MayPay` were one node wearing two
shapes, and the branch semantics ("chose to pay, regardless of what events
actually occurred") are [CR#118.12]'s.

## Scope (one feature; spec §§4–13)

- **Core grammar**: the merged role-honest enum (spec §4 table); `May{who}`;
  `Pay`; `ChangeLife`/`LifeOp`; `ChooseValue` (né `ChooseAndNote`);
  `Modal{who}` + payload rows; `Retarget` with the [CR#115.7a..115.7d] mode
  discriminant; `CopySpell{controller, spec, retarget}`;
  `Shuffle(Selection)` + `Selection::LibraryOf`; `InChosenOrder`.
- **Engine**: re-arm ~190 match sites; the payment-path rework (choice-time
  binding capture, `Agency::CostPayment` on all cost-paid verbs, payment
  batch id); event-layer role migration (life patient relabel + cause
  fields, sacrifice cause-stamp fix, filter cause slots,
  `Shuffled`/`Revealed` exposure, `Untapped` cause, `EventActor` alias
  deprecation).
- **Idris + emitter**: merged constructors; strip `{default You actor}`
  args (subsumes the player half of `core-remove-default-args`); `Pay` +
  its `intro`/`costCaps` arm; delete the `By`→per-constructor adapter.
- **Cards/canon**: ~24–31 canon lines; eligibility table widens to the whole
  `ChangeLife` family (gain-life costs are printed — Invigorate — and
  [CR#119.7] legislates them; retire the GainLife-cost-rejected tests);
  `MayPay`/`Unless` become render macros; wizards regen.

## Gates

Standard constraints apply. Specific: workspace suites green;
`cargo xtask idris-check plugins/canon` no regressions; `cargo xtask
fidelity` clean; no frames entry body contains `By(` (the round-2 sugar
bridge deletes; the `ChangeLife` entries keep STRUCTURAL bodies selecting
their `LifeOp` constructor — spec §12); `TargetedDealDamage`'s `body:` is
untouched (permanent per the round-2 ruling); `cargo xtask cite check`
clean.

## Sequencing

Land BEFORE any PlayerAction-heavy macro-frames round-3 coverage work
(round-2 ledger carry-forward #6; the core-friction list's single entry is
this reshape).

Related: `core-remove-default-args` (player-agent half subsumed here;
`This`/`[Library]` halves remain its scope), `engine-alt-costs`,
`core-alt-costs`, `core-may-pay-must-pay` (done/).

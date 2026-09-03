---
needs: []
---
**A target slot whose filter reads an EARLIER slot's announced register is
enumerated before that register exists.** Found by
`core-regions-witness-fixtures`; it is the blocker on two of its nineteen
witnesses. Standard constraints apply.

## The gap

`GameState::legal_targets_for_specs` maps each `TargetSpec` to
`legal_targets(spec, carrier, activation)` INDEPENDENTLY, and the announce
activation's `AnnouncedTarget(k)` registers are written only later, by
`ChooseTargets::resolve`. A filter that reads `Reg(AnnouncedTarget(0))`
therefore evaluates against an unavailable product at enumeration time, and
null packing decides the slot the wrong way in both directions:

- a POSITIVE cross-reference yields an EMPTY candidate set — Fiery
  Annihilation's "up to one target Equipment attached to that creature" can
  never be chosen at all;
- a NEGATED one yields EVERY candidate — Run Away Together's "two target
  creatures controlled by different players" accepts two creatures under one
  controller, which is not a legal announcement ([CR#601.2c]).

The ADR's law that targets are "a telescope over earlier params" is what the
enumeration must honour: slot `k`'s candidates are relative to the choices
already made for slots `< k`.

## Acceptance

`crates/deckmaste_engine/tests/region_witnesses.rs`'s
`fiery_annihilation_exiles_only_equipment_on_the_damaged_creature` and
`run_away_together_refuses_two_targets_under_one_controller` lose their
`#[ignore]` and pass unchanged. Whatever shape the fix takes — announcing
slots in order, or re-deriving later slots as earlier ones are picked — the
`ChooseTargets` prompt must still be answerable in one submission, since
`ChooseTargets::resolve` validates the whole set.

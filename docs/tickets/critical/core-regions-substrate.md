---
needs: [core-demacro]
---
**Stage 1 of [Core is explicit regions](../../decisions/core-explicit-regions.md):
the region substrate, the announce and event-role channels as params, and
the engine activation record.** Standard constraints apply; the ADR's laws
are the acceptance contract.

## Scope

- Core types: `DefId`, `RefId`, `Kind`, `Provenance`, `Param`, `Region`,
  `Block`, `Instr`, `Reference::Reg`, `Reference::OpponentOf`. Existing
  `OneShotEffect` variants become `Instr` arms unchanged in this stage
  except the ones this stage retires; the discourse-channel shapes (`With`,
  `Each`, `Distribute`, `Binder`, `It`, `That`, `They`, `Them`) survive
  until `core-regions-discourse`.
- `SpellAbility`, `ActivatedAbility`, `TriggeredAbility`, `Mode` gain
  `targets: [TargetSpec]` (a telescope over earlier params, as
  `TargetSpec::Distinct` already is); `OneShotEffect::Targeted` is
  deleted. Announced X is a param.
- Deleted from core: `This`, `You`, `Opponent`, `DefendingPlayer`,
  `EventObject`, `EventPatient`, `EventActor`, `Target(n)`,
  `Selection::Targets(n)`. Each is a param read by `Reg` with the matching
  provenance.
- `deckmaste_core::validate`: def sequence, dominance, kind, range, region
  closure. Unbound reads become load errors here.
- Lowering: builds regions and params for the ability kinds, modal modes,
  and delayed/reflexive bodies; maps a root semantic `Targeted` onto the
  ability's `targets` and a mode's `Targeted` onto the mode's; lowers the
  exophoric and event-role anaphors to param reads. The It-to-lone-target
  fallback has no lowering image and the engine arm is deleted.
- Engine: a per-resolution activation table on `GameState`; work items
  carry an activation id; `Frame` keeps only the id, payment state, and the
  residual discourse record (`it`, `that`, `chosen`, `allotment`) that
  stage 2 removes. `eval_reference` reads `Reg` from the activation and
  returns one product (live id, optional LKI) dispatched on provenance; the
  two shadow tables (`resolve/count.rs` `lki_counters`, `condition.rs`
  `bound_snapshot`) are deleted, and `EventObject`'s sibling-slot fallback
  goes with them. Every matcher (`target::matches_with`, the snapshot
  matcher, chooser enumeration, `SelectAll`, target announce/recheck) takes
  the activation, so a candidate-relative read of a param is ordinary.
- Null packing: `eval_reference_set` and every one-shot verb see an
  unavailable product as an empty set; nothing emits an event over a null
  or departed current-only id.

## Absorbed tickets (deleted 2026-09-02; their witnesses are gates here)

- `engine-candidate-frame-bindings`: per-player choices, target-relative
  filters, trigger-relative targets, whole-zone selections. Witnesses:
  Angel of Finality; Arbiter of Woe; Blasphemous Edict; Bloodtithe
  Collector; Burglar Rat; Deadly Brew; Duress; Liliana, Dreadhorde General;
  Painful Quandary; Perforating Artist; Pilfer; River's Rebuke; Tribute to
  Hunger; Tinybones, Bauble Burglar; Fiery Annihilation; Run Away Together;
  Steel Hellkite; Trygon Predator.
- `engine-snapshot-frame-references`: snapshot matching keeps candidate and
  event bindings. Witness: Predator Ooze's damaged-this-turn history
  predicate, three paths, all plain booleans.
- `engine-reference-resolution-snapshot-channel`: one reference product,
  shadow tables gone; a gone-object `CounterCount(This)` reads the
  snapshot; a patient-only frame resolves the event object.
- `engine-it-target-fallback-removal`; `validate-unbound-anaphor-lint`
  (a bare anaphor with no antecedent is a load error, now by construction).
- `engine-unbound-ref-oneshot-fizzle`, `engine-unbound-eventobject-panics`:
  `PutCounters`/`RemoveCounters`/`DealDamage` over an unavailable operand
  fizzle with no event, never reach the live-id assert.

## Gates

Canon re-lowers green: engine suites and the Idris re-emit gate. Byte
stability of core RON is not expected; behavior equivalence is the bar.
Fixtures: a spell with two same-sort targets read as two params; a trigger
reading its event roles; a modal spell whose modes declare their own
targets; a delayed trigger with no targets of its own inside a targeted
spell; the witness cards above.

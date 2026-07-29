---
needs: [english-ast-grouping]
---
**Deduplicate repeated AST wrapper scaffolding.** Split from
`english-ast-grouping` when its nominal-coordination work acquired parser and
attachment scope; these two representation-only changes remain independently
designed.

- **Shared `TriggerHeader` struct** — `{ introducer: TriggerWord, event:
  TriggerEvent, intervening_condition: Option<DependentClause> }` is written
  out three times: `TriggeredAbility`, `ModalFrame::Triggered`, and
  `ChoiceTrigger` (whose doc admits it mirrors the frame). Extract once; the
  three sites hold the struct.
- **`Attachment<T>` scaffolding** — `ClauseAttachment { position, comma,
  kind }` and `DependentAttachment { position, comma, clause }` share their
  scaffolding; the narrower gerund type is deliberate precision (gerunds admit
  only dependent clauses), so unify as a generic `Attachment<T> { position,
  comma, payload: T }` rather than collapsing the payload types.

Representation-only throughout: recovery census byte-identical, round-trip
stays at zero. Standard constraints apply.

## Completion

- Added `TriggerHeader` for the three sites that still share the exact
  single-event shape: `ModalFrame::Triggered`, `TriggeredSentence`, and
  `ChoiceInstruction::trigger_prefix`. The ticket premise had drifted since it
  was written: top-level `TriggeredAbility` now owns a richer
  `TriggerConditionList`, so it remains deliberately separate rather than
  losing coordinated trigger conditions to the wrapper abstraction.
- Replaced the duplicated clause/dependent attachment records with
  `Attachment<T> { position, comma, payload }`; `ClauseAttachment` and
  `DependentAttachment` remain public aliases with their distinct payload
  types.
- Updated parser lowering, rendering, recovery traversal, unit fixtures, and
  the three public API debug signatures affected by the field rename. No
  grammar rule, selection cost, or renderer surface changed.
- The supported corpus recovery/opacity JSON is byte-for-byte identical to the
  parent (3,459 structural spans / 63,737 source tokens; 1,413 licensed opacity
  occurrences / 2,096 source tokens). All 31,685 supported faces round-trip
  with zero mismatches or render errors.

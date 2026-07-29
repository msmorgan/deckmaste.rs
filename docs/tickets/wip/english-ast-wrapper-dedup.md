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

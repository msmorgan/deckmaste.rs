---
needs: [english-structural-recovery-zero]
---
**AST grouping dedup and coordination shared-material elevation.** Sibling of
`english-predicate-generics` and `english-surface-fact-diet`; adjudicated from
an external review 2026-07-23, each item verified against the code.

- **Shared `TriggerHeader` struct** — `{ introducer: TriggerWord, event:
  TriggerEvent, intervening_condition: Option<DependentClause> }` is written
  out three times: `TriggeredAbility`, `ModalFrame::Triggered`, and
  `ChoiceTrigger` (whose doc admits it mirrors the frame). Extract once;
  the three sites hold the struct.
- **`Attachment<T>` scaffolding** — `ClauseAttachment { position, comma,
  kind }` and `DependentAttachment { position, comma, clause }` share their
  scaffolding; the narrower gerund type is deliberate precision (gerunds
  admit only dependent clauses), so unify as a generic
  `Attachment<T> { position, comma, payload: T }` rather than collapsing the
  payload types.
- **Nominal-coordination shared-determiner elevation** — verified on
  Disenchant: `CoordinatedNounPhrase.first` holds `determiner: Some(Target)`
  while `rest` members carry `determiner: None`, so the shared determiner is
  buried in the first conjunct (the same asymmetry the predicate-generics
  ticket fixes one layer up). Needs the same two-species split: coordination
  of head material UNDER one determiner (`target artifact or enchantment` —
  one selection) vs coordination of full nominals (`target artifact and
  target enchantment` — two). Survey `CoordinatedPredicateObject` for the
  same pattern while there.

Adjudication notes (no action here): typed activation-cost components
(replacing `Cost::Components(Vec<Phrase>)`, an 18-variant catch-all bag)
belong to the campaign's existing cost slice, not this ticket. The partitive
critique was partially wrong — `any number of X` currently recovers rather
than parsing as a PP complement; when that family lands, pick its shape
against the real three-way split: partitive selection (`each of X`) vs
measure (`the number of X`) vs container (`a deck of cards`, barely
oracle-relevant).

Representation-only throughout: recovery census byte-identical, round-trip
stays at zero. Standard constraints apply.

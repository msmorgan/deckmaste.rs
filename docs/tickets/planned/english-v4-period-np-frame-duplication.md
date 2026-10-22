---
needs: []
---
# Prepositions duplicate their `NP` frame as a `PeriodNP` frame

Split out of `english-v4-wave1-review-cleanups` (item 3), whose proposed fix
needs a feature value the owner's rulings forbid.

`english-v4-temporal-np-as-direct-object` added `SlotFill::PeriodNP`, which the
grammar maps to `PeriodNounPhrase`, so that a Period Noun Phrase (*that turn*,
*each end step*) is never a verb's Object. A Period Noun Phrase is admitted
only where a head selects it. As a result, `of`, `after`, `before` and
`during` in `prepositions.ron` each declare a `(complement: PeriodNP, ...)`
frame beside their `(complement: NP, ...)` frame, with the same hosts. *skip*
selects one through `Transitive(PeriodNP)` in `verbs.ron`.

## Needs an owner ruling

The fix proposed in the wave-1 ticket cannot land under the current rulings.

- **No `Period` value.** The proposal was to make Period Noun Phrases a
  `NounPhrase` with `NounKind: Period` and exclude that value on
  `NonNumericComplement`. `NounKind` (`english.ron`) has no `Period` value.
  Adding one breaks the round-2 ruling (no new `english.ron` features for the
  cost-audit defects) and the round-6 ruling (no new `english.ron` features or
  values).
- **The proposal's premise no longer holds.** It assumed that prepositions take
  a plain `NounPhrase` while verbs take `ComplementNounPhrase`. In fact the `NP`
  slot maps to `ComplementNounPhrase` (`slot_categories` in `english.ron`) for
  prepositions and verbs alike. An exclusion on `NonNumericComplement` would
  therefore also keep Period Noun Phrases out of every preposition's `NP` frame.
- **`Temporal` cannot stand in.** Excluding `NounKind: Temporal` on
  `NonNumericComplement` would also remove genuine temporal objects, such as
  *take an extra turn*. In those, an ordinary determinative makes a
  `Determined` Noun Phrase over a temporal noun.

The options:

- **(a)** Approve a `NounKind: Period` value, together with a way for
  prepositions, but not verbs, to take it through `NP`.
- **(b)** Let one preposition frame list several complements (for example
  `complement: [NP, PeriodNP]`). This is a lexicon-schema change in
  `deckmaste_lexical_source_v4`. It removes the duplicate lines but not the
  duplicated declaration.
- **(c)** Keep the duplicate frames and retire this item.

## Done when

- The owner has ruled on (a), (b) or (c), and the ruling is recorded here.
- Under (a) or (b): `of`, `after`, `before` and `during` no longer duplicate
  their `NP` and `PeriodNP` frames in `prepositions.ron`, and *this turn* and
  *each combat* are still never Objects of verbs that do not select them.
- No face is lost, and no unit gains a Reading.

Standard constraints apply.

1. **Census delta**: measured at landing.
2. **Judgment set**: measured at landing.

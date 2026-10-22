---
needs: []
---
# Prepositions duplicate their `NP` frame as a `PeriodNP` frame

**Retired (owner ruling, option c).** No grammar or lexicon change.

## Owner's ruling (round 7, item 5)

> Period-NP frame duplication: Option (c) — keep the explicit PeriodNP frames
> on of/after/before/during and retire the ticket.

## What stays

This item was split out of `english-v4-wave1-review-cleanups` (item 3).

`english-v4-temporal-np-as-direct-object` added `SlotFill::PeriodNP`, which the
grammar maps to `PeriodNounPhrase`. With it, a Period Noun Phrase (*that turn*,
*each end step*) is never a verb's Object and is admitted only where a head
selects one. `of`, `after`, `before` and `during` in `prepositions.ron` keep
their explicit `(complement: PeriodNP, ...)` frame beside their
`(complement: NP, ...)` frame, and *skip* keeps `Transitive(PeriodNP)` in
`verbs.ron`. Each preposition that takes a Period Noun Phrase declares it once,
in its own frame list.

## Why the other options were not taken

- **(a) A `NounKind: Period` value.** It would add a new `english.ron` feature
  value. The round-2 ruling (no new `english.ron` features for the cost-audit
  defects) and the round-6 ruling (no new `english.ron` features or values)
  rule that out. It also rested on a false premise. Prepositions do not take a
  plain `NounPhrase` while verbs take `ComplementNounPhrase`: the `NP` slot maps
  to `ComplementNounPhrase` (`slot_categories` in `english.ron`) for both. So an
  exclusion on `NonNumericComplement` would also have kept Period Noun Phrases
  out of every preposition's `NP` frame. The existing `NounKind: Temporal`
  cannot stand in either: excluding it would also remove genuine temporal
  Objects such as *take an extra turn*.
- **(b) Preposition frames listing several complements.** This would be a
  lexicon-schema change in `deckmaste_lexical_source_v4`. It would remove the
  duplicate lines but not the duplicated declaration.

Standard constraints apply.

Retired: no grammar change.

---
needs: []
---
`Cast this spell your turn.` selects on the 2026-09-05 coordinator line, but
Oracle English requires an overt temporal marker here (`during your turn`) or
a demonstrative duration (`this turn`). The selected trunk analysis is
`Transitive Cast(Object(this spell))` followed by
`PredicateAdjunct(Duration(FixedDurationPhrase(your turn)))` through
`predicate_adjunct_predicate`.

The completed `english-v2-fixed-duration-endpoint` correctly made
`FixedDurationPhrase` read the endpoint noun's declared temporal licence, but
that licence says that *turn* denotes a time; it does not distinguish which
Nominal Phrase shapes may stand as a marker-less Duration Phrase. Introduce
that general grammatical distinction without naming a noun, determiner, verb,
construction, or card, and keep focused and unfocused Predicate Adjuncts
feature-transparent to the same host decision.

Read the rewrite ADR's declared-property and attestation amendments, the
2026-09-05 non-iterating-focus amendment, the Oracle English glossary entries
for Duration Phrase, Adjunct, Determiner Phrase, and Nominal Phrase, and the
Oracle style guide's “Timing and duration” section. Acceptance includes the
sentence above rejecting while `Cast this spell during your turn.`, `Cast this
spell this turn.`, and their single-focus counterparts retain the same host
admissibility decisions. Standard constraints apply.

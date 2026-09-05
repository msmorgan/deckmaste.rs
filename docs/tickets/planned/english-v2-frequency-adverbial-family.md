---
needs: []
---
`once each turn` is not a Frequency Predicate Adjunct. `Draw a card once each
turn.` fails without any focus adverb, so the family is independent of
`english-v2-tail-restrictive-focus-adverb` (done, 2026-09-05), which measured
101 corpus units of the shape `Activate only once each turn.` /
`Do this only once each turn.` and left every one of them a parse failure with
its 2026-09-05 exclusion recorded.

Defect. `vocab FrequencyAdverb { Once, Twice }` and
`frequency_predicate_adjunct` (`crates/deckmaste_english_v2/src/constructions.rs`)
admit the bare adverb alone. The attested adverbial is the adverb qualified by
a distributive temporal phrase — `once each turn`, `twice each turn`,
`once during each of your turns` — and neither the qualification nor the
distributive complement has a derivation. `FrequencyReference` covers the
comparative shapes (`more than once`) but not this one.

Shape. Give the Frequency Predicate Adjunct a general qualified member whose
complement is the existing distributive/temporal machinery, never a member per
attested surface and never a form literal spelling `turn`. The restrictive
focus adverb already composes with any Predicate Adjunct member, so
`only once each turn` needs no focus work: the 101 units select as soon as the
unfocused adverbial derives.

Acceptance. `Draw a card once each turn.` and `Activate only once each turn.`
select; the coverage delta names the units gained from the 101-unit block;
no construction, `require`, or `checked by` names an adverb, a noun, or a card.
Standard constraints apply.

---
needs: []
---
# Merge the three CoordinationMember singular/plural pairs

**R-coord — Group R, small.** Authority: rewrite ADR "Plan 08 homogeneous-Number
amendment (2026-09-04)" and "Plan 08 per-feature sequence amendment (2026-09-04,
second amendment)".

Defect. `CoordinationMember` has six constructions in three singular/plural pairs
(`crates/deckmaste_english_v2/src/constructions.rs:2762-2830`): bare, modified,
and negative-modified. Each pair is otherwise identical — same element fields,
same form, same `concord_class` / `number` / `onset` / `possessive_ending`
derivations — and differs in exactly one line,
`derive head.number = Values::Singular` against `Values::Plural`. Number is a
feature; six public AST variants encode it as a category distinction.

Pinned shape. One construction per member shape, three in total, with Number
relayed upward from the members (`derive number = members.number`) or imposed
across them by the enclosing coordination, per the homogeneous-Number amendment.
Six AST variants become three. This is the same feature-collapse idiom as
`english-v2-number-feature-unification`; spell it against that landing.

Fences. Keeping a stored number tag or a form tag to stand in for the deleted
variants. Widening a `nominal_form` value list as a side effect — the possessive
and genitive splits are `english-v2-possessive-nominal-form-collapse`'s
(that landing's `Aquatic Alchemist // Bubble Up` misselection is the standing
warning). A `checked by` naming a construction.

Glossary: Coordination, Number, Concord Class, Nominal, Head. Record any gap.

Baseline, measured on change `oulzkkoqmvuv` (388 constructions, 17,052 / 32,641
covered) — re-measure at claim. Report every changed selected analysis. Standard
constraints apply.

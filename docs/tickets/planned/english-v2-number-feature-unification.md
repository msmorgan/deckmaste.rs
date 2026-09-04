---
needs: [english-v2-np-postmodifiers]
---
**Carry grammatical number as the `Number` feature and delete the parallel
singular/plural category hierarchies.** The rewrite decision
(`docs/decisions/english-v2-rewrite.md`, "Number is the noun recipe's feature
axis … Grammar may distinguish noun behavior only through declared grammatical
features") is already the ruling; the grammar violates it by duplicating the
axis as categories. Ten mirror pairs, twenty constructions, differ only in the
role's category and one `derive nominal_form = Values::…` value:
`noun_singular_head`/`noun_plural_head`; `bare_`/`modified_`/
`negative_modified_` × `SingularNominal`/`PluralNominal`; the same three ×
`SingularCoordinationMember`/`PluralCoordinationMember`; `_and_`/`_or_`/
`_and_or_` × `SingularNominalCoordination`/`PluralNominalCoordination`; and the
`Nominal` re-entry arms (`singular_nominal_value`/`plural_nominal_value`,
`{singular,plural}_coordination_nominal_value` and their `modified_` twins).

Pinned shape: one category per pair (`Head`, `Nominal`, `CoordinationMember`,
`NominalCoordination`), with number carried by `derive number = head.number`
upward, imposed downward by `derive role.number = Values::…`, and gated where a
parent needs one value by `require role.number is Singular|Plural`. Every
primitive is already in use in the file; `genitive_determiner_mass_reference`
(general `Nominal` role plus `require nominal.nominal_form is MassNoun`) is the
in-file precedent. `NominalForm` keeps its full domain. No compiler change.

Fences (STOP-and-report, never shipped): a new specificity tie resolved by a
dominance edge, exception entry, or narrowed form; a `checked by` Rust guard
substituting for a feature `require`; any construction deleted without its
mirror twin's coverage re-spelled against the unified shape.

Acceptance: the landing record's selection census (unique / specificity-
resolved) before and after, coverage unchanged or up with every newly covered
identity listed, both byte-exact laws green, and the construction count down by
the pairs collapsed. Standard constraints apply.

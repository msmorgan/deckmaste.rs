---
needs: [english-v2-grammar-family-breadth]
---
# Migrate extraction, relatives and clause Complements

Complete this family on the shared breadth interfaces under
[the lexical-analysis contract](../../decisions/english-lexical-analysis.md).

Replace `SubjectGapRelativeClause`/`ObjectGapRelativeClause` and their
auxiliary/perfect/negative/copular wrapper families with the common clause
body and explicit relative form/discharge. Track ordered exposed Gaps by
category and relation. That, who/which, zero, possessive-fronted, supplementary
and pied-piped forms use the reviewed carrier; zero cannot discharge a Subject.
Agreement of a possessive fronted phrase is independent of the modified noun.

Add declared finite that-Complements, wh-Complements and free-relative forms
over the same clause grammar. Use `Dependencies` and inhabited positive
witnesses to refine gap/form/boundary constraints and their exclusions. Adjuncts
and complex Subjects are extraction boundaries; ordinary coordination cannot
pretend to share a Gap. Closed complete candidates must have no undischarged
Gap. No raw-text fallback, copied antecedent tree or rule-resolution context.

Acceptance carries Absorbing Man and Titania's auxiliary object-gap relative
on `all damage`, Ashen-Skin Zubera's `that died this turn`, and Boldwyr
Heavyweights' `who searched`, with full identities in the register. Add
which/whose/pied-piping/supplementary positives and zero-subject, wrong-category,
wrong-agreement and adjunct-extraction negatives corresponding to
`DependencyInteractions`. Preserve all current relative tests by outcome.
Infectious Curse's two relative structures are exercised here with a declared
synthetic transitive head; its actual Target Verb and full re-coverage remain
owned by `english-v2-target-verb-subject-selection`.
Style-guide evidence: §5 “Names, self-reference, pronouns, and anaphora” and
§6 “Describing objects, players, and targets”.

Preserve the applicable [inherited witnesses](../../english-grammar-migration-obligations.md)
and check this family's structures under both roundtrip laws. Standard constraints
apply as amended by the lexical-analysis decision; targeted Lean changes belong
here only when needed to settle a changed grammatical constraint.

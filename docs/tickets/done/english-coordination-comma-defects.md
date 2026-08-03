---
needs: []
---
**Characterize the noun/nominal comma defects for derived coordination.**

Round `sfdiet` proved the member-count/conjunction-shape comma derivation exact
for five coordination structs but exposed 40 wrong noun/nominal trees. Removing
the spurious generic count-noun reading of `block` corrected 22; the remaining
18 are partitioned into wrong clause splits, per-conjunct PP attachment,
shared-determiner collapse, local power/toughness grouping, and two structures
requiring explicit `inspect` classification.

The partition, named witnesses, intended AST shapes, and failed legacy
list-flattening experiment are transferred to `english-coordination-derived`.
That replacement owns the structural fixes and deletion of
`NounPhraseCoordination.comma` and `NominalPhraseCoordination.comma`; no
handwritten grammar, reduction, lowering, or renderer stopgap lands here.

Standard constraints apply.

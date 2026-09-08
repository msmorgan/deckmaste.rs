---
needs: []
---
**`ActFacts.participle` and the declarations' `grammar: Verb(participle: …)`
disagree in both directions.** Left standing by
`facts-act-columns-are-lean-data`, which moved the `actFacts` table into
`lean/Semantics/Check/Words.lean` and carried both sets over as they were.

`participle` (8 rows) is NOT the declaration's English participle: five keyword
actions declare `grammar: Verb(participle: …)`, and the two sets disagree in
both directions (`cast` and `activate` declare one and carry no `ActFacts`
participle; `destroy`, `discard`, `mill`, `tap` and `untap` carry one and
declare none).

Decide whether the checker column is a second, deliberately different fact —
in which case say so where it is defined — or the declaration's own participle
read late, in which case the column comes from the declaration and the five
disagreements are each resolved by name. Standard constraints apply.

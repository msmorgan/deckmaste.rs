---
needs: []
---
**Model gaps behind the 87 keyword-ability declarations still bodyless
after `semantics-v2-macro-bodies-keyword-abilities`.** That ticket's
second landing record (`docs/tickets/done/`) holds the per-declaration
STOP text: 95 defined, 13 settled empty as rules machinery, 87 STOPs.
Pieces of the 87 are already owned elsewhere —
`lean-keyword-definition-regimes` (Offspring, Squad, Impending),
`semantics-v2-subject-param-kind`, `semantics-v2-macro-param-kinds`,
`semantics-v2-macro-capture-and-plurality` — but no ticket owns the list.
This one does: first landing is the census, every one of the 87 named
with its STOP cause and the ticket that owns it (or "this ticket"), then
the Lean-first route for the unowned remainder, each shape added to
`lean/Semantics` with its pins, mirrored, then the declaration bodied.
Standard constraints apply.

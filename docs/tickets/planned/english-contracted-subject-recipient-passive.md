---
needs: []
---
**Contracted-subject recipient passives (`you're dealt damage`) do not
parse.** Found 2026-07-25 (round kwverbs, E3-tail triage): round dealtdmg
landed the recipient passive (`PredicateFrame::recipient_passive` on
`Deal`, `PassivePredicate::retained_object`) for `you were dealt damage` /
`an opponent was dealt damage`, but the CONTRACTED-subject form (`you're
dealt damage`, presumably routed through `SimpleClauseContractedSubject`,
grammar/clause.rs) does not reach it. ~8 witness faces, all trigger
events: Sun Droplet, Living Artifact, Lich, Darien King of Kjeldor, Blood
Hound, Angelheart Vial, Swarmborn Giant (`you're dealt combat damage`),
Contested Game Ball. Diagnosis entry point: compare the contracted-subject
reduction path against the ordinary subject path for where the
recipient-passive frame routing (predicate_arguments_complete and the
passive-object gates fixed at dealtdmg) is consulted — likely one of those
gates lacks the same frame exception on the contracted path. Smells
repair-class.

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

**RESOLVED 2026-07-25 (round ctrsubj).** Diagnosis confirmed exactly:
the `SimpleClauseContractedSubject` features arm passed the raw child
`passive` to `predicate_arguments_complete` while the ordinary
`VerbPhraseAuxiliary` arm folded the Be+past-participle auxiliary into
the passive determination. Mechanism: a shared `fold_auxiliary_passive`
helper beside `predicate_arguments_complete` (grammar/clause.rs), used by
BOTH arms so the two auxiliary paths cannot drift again; it also carries
the retained-object rule (a passive keeps a direct object only as a
recipient passive's retained theme, and never keeps an explicit indirect
object), which tightened the contracted path as a side effect. All 8
witness faces now parse as `Predicate::Passive` with `retained_object`
and `first_auxiliary_contracted_with_subject: true`, rendering the
contracted surface. Six named tests pin the behavior, including a
direct-helper test for the non-recipient-object rejection (a
full-sentence synthetic is confounded by `'s`-as-`has`).

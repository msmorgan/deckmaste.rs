---
needs: [english-v2-np-postmodifiers, english-v2-closed-class-single-owner, english-v2-adjunct-licence-removal]
---
**Add `with` to `vocab Preposition` and retire the three fused
`with`-postmodifiers.** The inventory has fourteen members (`after` … `under`)
and no `with`, so `"with"` appears as a bare form literal or codec-tail literal
at about eleven sites and cannot head a `PrepositionalPhrase`.
`scalar_qualification` (`"with" measure comparison`),
`degree_scalar_qualification` (`"with" degree measure`) and
`granted_ability_qualified_reference` (`reference "with" granted`) exist only
because of that gap; the general `prepositional_qualified_reference`
postmodifier and its noun-side licence check already carry every other
preposition. This is the unclosed remainder of the Plan 09 taxonomy audit's
HIGH finding that `with/without/by/as/between` cannot form PPs.

Pinned method, per the 2026-09-02 attachment amendment in
`docs/decisions/english-v2-rewrite.md`: `With`'s `PrepositionAttachment` class
is corpus-measured, never asserted, and admissibility is the conjunction of
that class and the complement head's declared licence. `PrepositionalComplement`
needs a measured decision on its non-`Object` complements here (a keyword-line
item, a quoted ability, a scalar measure): add arms only where the census
shows them, and record the census in the landing record.

Fences: declaring `With` adjunct-capable by intuition; keeping a fused
`with` construction "for now" beside the PP; a `checked by` guard naming `with`.
Verb-selected `with` (`WithObjectVerb`, `ObjectWithObjectVerb`,
`EnterWithCountersVerb`) is frame data and stays out of scope; it belongs to
`english-v2-frame-selected-prepositions`.

Acceptance: `form_literal_vocab_overlaps` down by the retired `with` literals,
the three fused constructions deleted with their coverage re-spelled through
the PP postmodifier, selection census before/after, byte-exact laws green.
Standard constraints apply.

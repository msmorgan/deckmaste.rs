---
needs: [english-derived-family-inventory, english-derived-quantity-family, english-derived-noun-lexeme-family]
---
**Derive the remaining noun-phrase variants.**

Migrate P01 from `docs/english-derived-family-inventory.md`:
`noun_phrase_set_exception_bare`, `noun_phrase_set_exception_for`,
`noun_phrase_nominal`, `rules_object_noun_phrase`,
`noun_phrase_subject_pronoun`, `noun_phrase_object_pronoun`,
`noun_phrase_reciprocal`, `noun_phrase_quantity`, `noun_phrase_this_card`,
`noun_phrase_full_this_card`, `noun_phrase_possessive_this_card`,
`noun_phrase_demonstrative`, `noun_phrase_partitive`,
`noun_phrase_each_partitive`, `noun_phrase_any_number_of`,
`noun_phrase_minus`, `noun_phrase_half`, `noun_phrase_half_rounded_up`, and
`noun_phrase_half_rounded_down`.

Use scalar and identity holes and add agreement, pronoun-case,
coordination-domain, set-exception-host, notional-plurality, and rules-object
followup constraints. Preserve the listed dominance over arithmetic,
coordination, and object-case alternatives and the exact self-reference and
rounding witnesses.

Land declarations and generated parse/reduction/lowering, total render, and
validated build projections; the Nominal fragment plus every frame/view
consumer change; direct-AST/`inspect`/exactness/negative fixtures; all 19
registry flips; and deletion of handwritten mirrors and raw-construction
bypasses in one change. Update the inventory. No consumer or deletion work is
deferred.

Standard constraints apply.

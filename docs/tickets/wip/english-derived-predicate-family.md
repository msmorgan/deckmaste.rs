---
needs: [english-derived-family-inventory, english-derived-quantity-family, english-derived-noun-lexeme-family, english-derived-nominal-family]
---
**Derive the English predicate spine and valency constraints.**

Migrate V01 from `docs/english-derived-family-inventory.md`: `verb`,
`verb_phrase_base`, `verb_phrase_auxiliary`,
`verb_phrase_auxiliary_proform`, `verb_phrase_direct_object`,
`verb_phrase_indirect_object`, `verb_phrase_adjective`,
`verb_phrase_prepositional`,
`verb_phrase_passive_shared_determiner_prepositional`,
`verb_phrase_except_by`, `verb_phrase_infinitive`, `verb_phrase_adverb`,
`verb_phrase_preverb_adverb`, `verb_phrase_particle`,
`verb_phrase_coin_result`, `verb_phrase_frequency`,
`frequency_phrase_adverb`, `verb_phrase_ability`,
`verb_phrase_quoted_ability`, `verb_phrase_quoted_ability_coordination`,
`verb_phrase_ability_quoted_coordination`, `verb_phrase_oracle_symbol`,
`verb_phrase_symbol_sequence`, `mana_amount_symbol`,
`mana_amount_sequence`, `mana_amount_list_single`, `mana_amount_list_comma`,
`mana_amount_coordination`, `mana_amount_coordination_oxford`,
`verb_phrase_mana_amount_coordination`, `verb_phrase_power_toughness`,
`verb_phrase_quantity`, `verb_phrase_causative`, and `frequency_phrase`.

Use scalar, identity, subtree, and field-lens holes; add lexical valency,
predicate-form, voice, auxiliary agreement, direct/indirect object,
causative, selected-PP, and attachment constraints. Preserve pre/post-object
field slices, mixed ability-object order, named costs, and all four dominance
relations recorded in V01.

Land declarations and generated parse/reduction/lowering, total render, and
validated build projections; every Sentence/Cost/KeywordLine/Ability and
spelling/view consumer change; direct-AST and verbose `inspect` fixtures for
every valency/role; exactness and invalid-argument/list negatives; all 34
registry flips; and deletion of handwritten mirrors and raw construction paths
together. The deletion audit must remove CR's ordinary registrations and
substantive reduction/lowering arms, NR's sole
`verb_phrase_coin_result` registration in `add_coin_result_rules`, and NR's
five V01 registrations in `add_reduced_recipient_passive_rules`:
`verb_phrase_base`, `verb_phrase_direct_object`,
`verb_phrase_prepositional`, `verb_phrase_adverb`, and
`verb_phrase_frequency`. Update the inventory. No consumer or deletion work is
deferred.

Standard constraints apply.

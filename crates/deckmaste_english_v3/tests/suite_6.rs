// Register new tests/suite modules in exactly one binary root; see tests/README.
#[allow(dead_code, reason = "Each binary uses a subset of the shared test fixtures.")]
mod common;

#[path = "suite/attributive_scalar_comparative.rs"]
mod attributive_scalar_comparative;

#[path = "suite/amount_complements.rs"]
mod amount_complements;

#[path = "suite/bounded_quantity_mass_noun.rs"]
mod bounded_quantity_mass_noun;

#[path = "suite/central_adverb_position.rs"]
mod central_adverb_position;

#[path = "suite/correlative_finite_complements.rs"]
mod correlative_finite_complements;

#[path = "suite/data_batch_2.rs"]
mod data_batch_2;

#[path = "suite/derived_punctuation.rs"]
mod derived_punctuation;

#[path = "suite/enter_as_copy_except_clause.rs"]
mod enter_as_copy_except_clause;

#[path = "suite/gapped_coordination_shared_adjunct.rs"]
mod gapped_coordination_shared_adjunct;

#[path = "suite/head_primitives.rs"]
mod head_primitives;

#[path = "suite/italic_heads.rs"]
mod italic_heads;

#[path = "suite/lexical_compositions.rs"]
mod lexical_compositions;

#[path = "suite/name_lexeme_scope.rs"]
mod name_lexeme_scope;

#[path = "suite/negated_to_infinitival.rs"]
mod negated_to_infinitival;

#[path = "suite/negative_common_noun_premodifier.rs"]
mod negative_common_noun_premodifier;

#[path = "suite/preposition_function_licences.rs"]
mod preposition_function_licences;

#[path = "suite/remove_complements.rs"]
mod remove_complements;

#[path = "suite/scalar_cardinals.rs"]
mod scalar_cardinals;

#[path = "suite/selected_complement_clusters.rs"]
mod selected_complement_clusters;

#[path = "suite/shared_heads.rs"]
mod shared_heads;

#[path = "suite/singular_you.rs"]
mod singular_you;

#[path = "suite/slice.rs"]
mod slice;

#[path = "suite/targeting_projection.rs"]
mod targeting_projection;

#[path = "suite/preverbal_next.rs"]
mod preverbal_next;

#[path = "suite/focus_only_object.rs"]
mod focus_only_object;

#[path = "suite/do_so_and_able_to.rs"]
mod do_so_and_able_to;

#[path = "suite/named_modes.rs"]
mod named_modes;

#[path = "suite/play_with_absolute.rs"]
mod play_with_absolute;

#[path = "suite/remove_source_attachment.rs"]
mod remove_source_attachment;

#[path = "suite/champion_keyword_line.rs"]
mod champion_keyword_line;

#[path = "suite/noncombat_negative_noun.rs"]
mod noncombat_negative_noun;

#[path = "suite/modal_header_quantities.rs"]
mod modal_header_quantities;

#[path = "suite/polyhedral_die_nouns.rs"]
mod polyhedral_die_nouns;

#[path = "suite/progressive_object_gap_relative.rs"]
mod progressive_object_gap_relative;

#[path = "suite/devotion_to_colour.rs"]
mod devotion_to_colour;

#[path = "suite/partitive_quantity_phrases.rs"]
mod partitive_quantity_phrases;

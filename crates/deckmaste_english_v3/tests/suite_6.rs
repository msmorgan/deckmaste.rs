// Register new tests/suite modules in exactly one binary root; see tests/README.
#[allow(dead_code, reason = "Each binary uses a subset of the shared test fixtures.")]
mod common;

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

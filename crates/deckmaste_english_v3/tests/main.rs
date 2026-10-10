// Register new tests/suite modules in exactly one binary root; see tests/README.
#[allow(dead_code, reason = "Each binary uses a subset of the shared test fixtures.")]
mod common;

#[path = "suite/cause_catenative.rs"]
mod cause_catenative;

#[path = "suite/become_adjectival_passive.rs"]
mod become_adjectival_passive;

#[path = "suite/by_complement_functions.rs"]
mod by_complement_functions;

#[path = "suite/clause_final_predicative_as.rs"]
mod clause_final_predicative_as;

#[path = "suite/comparative_complements.rs"]
mod comparative_complements;

#[path = "suite/cost_scalar_complement.rs"]
mod cost_scalar_complement;

#[path = "suite/designation_categories.rs"]
mod designation_categories;

#[path = "suite/divided_as_you_choose.rs"]
mod divided_as_you_choose;

#[path = "suite/existential_there.rs"]
mod existential_there;

#[path = "suite/frequency.rs"]
mod frequency;

#[path = "suite/grammar.rs"]
mod grammar;

#[path = "suite/last_turn_temporal_adjunct.rs"]
mod last_turn_temporal_adjunct;

#[path = "suite/multiplier_predeterminer.rs"]
mod multiplier_predeterminer;

#[path = "suite/multiword_lexemes.rs"]
mod multiword_lexemes;

#[path = "suite/nominal_adjuncts.rs"]
mod nominal_adjuncts;

#[path = "suite/overgeneration_prune.rs"]
mod overgeneration_prune;

#[path = "suite/purpose_infinitival.rs"]
mod purpose_infinitival;

#[path = "suite/selected_frame_passives.rs"]
mod selected_frame_passives;

#[path = "suite/shared_preposition_heads.rs"]
mod shared_preposition_heads;

#[path = "suite/verbless_if_complement.rs"]
mod verbless_if_complement;

#[path = "suite/half_rounded.rs"]
mod half_rounded;

#[path = "suite/phase_in_out.rs"]
mod phase_in_out;

#[path = "suite/supplementary_temporal_adjunct.rs"]
mod supplementary_temporal_adjunct;

#[path = "suite/spelled_cardinal_scalar_measure.rs"]
mod spelled_cardinal_scalar_measure;

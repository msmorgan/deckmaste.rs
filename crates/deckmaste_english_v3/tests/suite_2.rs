// Register new tests/suite modules in exactly one binary root; see tests/README.
#[allow(dead_code, reason = "Each binary uses a subset of the shared test fixtures.")]
mod common;

#[path = "suite/arithmetic_nominal_operand.rs"]
mod arithmetic_nominal_operand;

#[path = "suite/as_long_as.rs"]
mod as_long_as;

#[path = "suite/at_least_cardinal.rs"]
mod at_least_cardinal;

#[path = "suite/bare_zone_noun_complement.rs"]
mod bare_zone_noun_complement;

#[path = "suite/card_noun_premodifier.rs"]
mod card_noun_premodifier;

#[path = "suite/combat_intervals.rs"]
mod combat_intervals;

#[path = "suite/fixed_cost_phrases.rs"]
mod fixed_cost_phrases;

#[path = "suite/from_anywhere.rs"]
mod from_anywhere;

#[path = "suite/get_object_frames.rs"]
mod get_object_frames;

#[path = "suite/landwalk_bound_suffix.rs"]
mod landwalk_bound_suffix;

#[path = "suite/library_positions.rs"]
mod library_positions;

#[path = "suite/once_frequency.rs"]
mod once_frequency;

#[path = "suite/permanent_noun_premodifier.rs"]
mod permanent_noun_premodifier;

#[path = "suite/phrase_coordination.rs"]
mod phrase_coordination;

#[path = "suite/relative_clause_adjuncts.rs"]
mod relative_clause_adjuncts;

#[path = "suite/signed_numeral_cost.rs"]
mod signed_numeral_cost;

#[path = "suite/transitive_turn_face.rs"]
mod transitive_turn_face;

#[path = "suite/typecycling_bound_suffix.rs"]
mod typecycling_bound_suffix;

#[path = "suite/vocabulary_batch_2.rs"]
mod vocabulary_batch_2;

#[path = "suite/where_variable_clause.rs"]
mod where_variable_clause;

#[path = "suite/analytic_not_after_be.rs"]
mod analytic_not_after_be;

#[path = "suite/coordinated_focused_adjuncts.rs"]
mod coordinated_focused_adjuncts;

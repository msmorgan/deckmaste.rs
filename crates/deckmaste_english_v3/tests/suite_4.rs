// Register new tests/suite modules in exactly one binary root; see tests/README.
#[allow(dead_code, reason = "Each binary uses a subset of the shared test fixtures.")]
mod common;

#[path = "suite/base_phrases.rs"]
mod base_phrases;

#[path = "suite/causative_have.rs"]
mod causative_have;

#[path = "suite/coordinated_property_values.rs"]
mod coordinated_property_values;

#[path = "suite/correlative_adjectives.rs"]
mod correlative_adjectives;

#[path = "suite/enter_as_copy_complement.rs"]
mod enter_as_copy_complement;

#[path = "suite/focusing_only.rs"]
mod focusing_only;

#[path = "suite/from_preposition_phrase_complement.rs"]
mod from_preposition_phrase_complement;

#[path = "suite/functions_coordination.rs"]
mod functions_coordination;

#[path = "suite/generic_predicates.rs"]
mod generic_predicates;

#[path = "suite/gerund_preposition_complements.rs"]
mod gerund_preposition_complements;

#[path = "suite/granted_ability_quotes.rs"]
mod granted_ability_quotes;

#[path = "suite/instead_replacement.rs"]
mod instead_replacement;

#[path = "suite/keyword_pp.rs"]
mod keyword_pp;

#[path = "suite/keyword_quality_phrases.rs"]
mod keyword_quality_phrases;

#[path = "suite/lexical_gaps.rs"]
mod lexical_gaps;

#[path = "suite/next_time_relative_adjunct.rs"]
mod next_time_relative_adjunct;

#[path = "suite/participial_selection.rs"]
mod participial_selection;

#[path = "suite/past_participial_premodifiers.rs"]
mod past_participial_premodifiers;

#[path = "suite/postpositive_comparative_property.rs"]
mod postpositive_comparative_property;

#[path = "suite/preposition_head_primitives.rs"]
mod preposition_head_primitives;

#[path = "suite/property_equal_to.rs"]
mod property_equal_to;

#[path = "suite/rather_than_complements.rs"]
mod rather_than_complements;

#[path = "suite/reading_support.rs"]
mod reading_support;

#[path = "suite/scalar_property_values.rs"]
mod scalar_property_values;

#[path = "suite/then_sequencing.rs"]
mod then_sequencing;

#[path = "suite/preposed_duration_for.rs"]
mod preposed_duration_for;

#[path = "suite/verbless_while_adjunct.rs"]
mod verbless_while_adjunct;

// Register new tests/suite modules in exactly one binary root; see tests/README.
#[allow(dead_code, reason = "Each binary uses a subset of the shared test fixtures.")]
mod common;

#[path = "suite/articles.rs"]
mod articles;

#[path = "suite/asyndetic_premodifier_coordination.rs"]
mod asyndetic_premodifier_coordination;

#[path = "suite/clitic_contractions.rs"]
mod clitic_contractions;

#[path = "suite/comparative_quantities.rs"]
mod comparative_quantities;

#[path = "suite/coordinated_subject_agreement.rs"]
mod coordinated_subject_agreement;

#[path = "suite/copular_scalar_location.rs"]
mod copular_scalar_location;

#[path = "suite/correlative_gaps.rs"]
mod correlative_gaps;

#[path = "suite/correlatives.rs"]
mod correlatives;

#[path = "suite/counter_compounds.rs"]
mod counter_compounds;

#[path = "suite/each_of_partitive.rs"]
mod each_of_partitive;

#[path = "suite/floated_each_quantifier.rs"]
mod floated_each_quantifier;

#[path = "suite/licence_hygiene.rs"]
mod licence_hygiene;

#[path = "suite/notation_coordination.rs"]
mod notation_coordination;

#[path = "suite/packed.rs"]
mod packed;

#[path = "suite/possessive_phrases.rs"]
mod possessive_phrases;

#[path = "suite/predicate_terms.rs"]
mod predicate_terms;

#[path = "suite/pronominal_other.rs"]
mod pronominal_other;

#[path = "suite/restricted_nominals.rs"]
mod restricted_nominals;

#[path = "suite/same_as_comparative.rs"]
mod same_as_comparative;

#[path = "suite/vocabulary_batch_1.rs"]
mod vocabulary_batch_1;

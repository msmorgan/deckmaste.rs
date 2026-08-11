//! Checked construction and projection API for the derived predicate family.
//!
//! Builders retain the declaration's private verb-phrase value while a
//! predicate is incomplete. Finishing projects it into the sealed public AST;
//! callers inspect the result through the semantic accessors on its syntax
//! types.

pub use crate::constructions::predicate::PredicateBuilder;
pub use crate::constructions::predicate::PredicateFrameChoice;
pub use crate::constructions::predicate::build_predicate_auxiliary;
pub use crate::constructions::predicate::build_predicate_direct_object;
pub use crate::constructions::predicate::build_predicate_element;
pub use crate::constructions::predicate::build_predicate_verb;
pub use crate::constructions::predicate::finish_object_gap_predicate;
pub use crate::constructions::predicate::finish_predicate;
pub use crate::grammar::VerbAnalysis;

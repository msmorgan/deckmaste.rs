#[allow(
    clippy::wildcard_imports,
    reason = "module uses generated imports and shared grammar aliases"
)]
use super::*;
use crate::syntax::AttachedPredicate;
use crate::syntax::AttachmentPosition;
#[cfg(test)]
use crate::syntax::ClauseAttachment;
#[cfg(test)]
use crate::syntax::ClauseAttachmentKind;
use crate::syntax::ClauseCoordination;
#[cfg(test)]
use crate::syntax::ComplexClause;
use crate::syntax::CoordinatedClauseMember;
use crate::syntax::CoordinatedIndependentClause;
#[cfg(test)]
use crate::syntax::CoordinatedPredicateObject;
use crate::syntax::Coordination;
use crate::syntax::CoordinationJunction;
use crate::syntax::DeonticPredicate;
#[cfg(test)]
use crate::syntax::DependentClause;
#[cfg(test)]
use crate::syntax::EllipticalClause;
use crate::syntax::IndependentClause;
#[cfg(test)]
use crate::syntax::Modal;
#[cfg(test)]
use crate::syntax::PassivePredicate;
use crate::syntax::Predicate;
use crate::syntax::PredicateAdjunct;
#[cfg(test)]
use crate::syntax::PredicateComplement;
#[cfg(test)]
use crate::syntax::PredicateElement;
use crate::syntax::PredicateExpression;
use crate::syntax::PredicateHead;
#[cfg(test)]
use crate::syntax::PredicateObjectCoordination;
#[cfg(test)]
use crate::syntax::RestrictionCoordination;
#[cfg(test)]
use crate::syntax::SubordinateBody;

mod lowering;
mod reduction;
mod rules;

pub(crate) use reduction::PredicateAttachment;
pub(crate) use reduction::auxiliary_form;
pub(crate) use reduction::extend_predicate_features;
pub(crate) use reduction::fold_auxiliary_passive;
pub(crate) use reduction::predicate_arguments_complete;
pub(crate) use reduction::predicate_object_gap_complete;
pub(crate) use reduction::reduce_relative_contracted_copular_coordinated_adjective_features;

#[cfg(test)]
pub(crate) mod tests;
pub(super) use lowering::coordinated_modifier_as_adjectives;
pub(super) use lowering::finish_infinitive;
pub(super) use lowering::finish_reduced_recipient_passive;
pub(crate) use lowering::finish_simple_clause;
pub(super) use lowering::lower_clause;
pub(crate) use lowering::lowered_nominal_adjunct_kind;
pub(super) use reduction::accepts_predicate_prefix;
pub(super) use reduction::reduce_clause;
pub(crate) use reduction::reduce_generated_recipient_passive_nominal_adjunct_features;
pub(super) use reduction::reduction_cost;
pub(super) use rules::add_rules;
#[cfg(test)]
pub(crate) use tests::fixture_catalogs;

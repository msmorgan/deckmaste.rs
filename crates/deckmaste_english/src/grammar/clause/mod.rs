#[allow(
    clippy::wildcard_imports,
    reason = "module uses generated imports and shared grammar aliases"
)]
use super::*;
#[cfg(test)]
use crate::syntax::AttachmentPosition;
#[cfg(test)]
use crate::syntax::ClauseAttachmentKind;
#[cfg(test)]
use crate::syntax::ClauseCoordination;
#[cfg(test)]
use crate::syntax::CoordinatedClauseMember;
#[cfg(test)]
use crate::syntax::Coordination;
#[cfg(test)]
use crate::syntax::CoordinationJunction;
#[cfg(test)]
use crate::syntax::DeonticPredicate;
#[cfg(test)]
use crate::syntax::DependentClause;
#[cfg(test)]
use crate::syntax::EllipticalClause;
#[cfg(test)]
use crate::syntax::IndependentClause;
#[cfg(test)]
use crate::syntax::Modal;
#[cfg(test)]
use crate::syntax::PassivePredicate;
use crate::syntax::Predicate;
#[cfg(test)]
use crate::syntax::PredicateAdjunct;
#[cfg(test)]
use crate::syntax::PredicateComplement;
#[cfg(test)]
use crate::syntax::PredicateElement;
#[cfg(test)]
use crate::syntax::PredicateExpression;
#[cfg(test)]
use crate::syntax::PredicateHead;
#[cfg(test)]
use crate::syntax::RestrictionCoordination;
#[cfg(test)]
use crate::syntax::SubordinateBody;

mod lowering;
mod reduction;

pub(crate) use reduction::PredicateAttachment;
pub(crate) use reduction::auxiliary_form;
pub(crate) use reduction::extend_predicate_features;
pub(crate) use reduction::fold_auxiliary_passive;
pub(crate) use reduction::predicate_arguments_complete;
pub(crate) use reduction::predicate_object_gap_complete;

#[cfg(test)]
pub(crate) mod tests;
pub(super) use lowering::coordinated_modifier_as_adjectives;
pub(super) use lowering::finish_infinitive;
pub(super) use lowering::finish_reduced_recipient_passive;
pub(crate) use lowering::lowered_nominal_adjunct_kind;
pub(crate) use reduction::reduce_generated_recipient_passive_nominal_adjunct_features;
#[cfg(test)]
pub(crate) use tests::fixture_catalogs;

pub(crate) use crate::constructions::clause::finish_simple_clause;

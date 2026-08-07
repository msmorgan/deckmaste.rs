#[allow(
    clippy::wildcard_imports,
    reason = "module uses generated imports and shared grammar aliases"
)]
use super::*;
use crate::syntax::AbilityObject;
use crate::syntax::AttachedPredicate;
use crate::syntax::AttachmentPosition;
use crate::syntax::ClauseAttachment;
use crate::syntax::ClauseAttachmentKind;
use crate::syntax::ClauseCoordination;
use crate::syntax::ComplexClause;
use crate::syntax::CoordinatedClauseMember;
use crate::syntax::CoordinatedIndependentClause;
use crate::syntax::CoordinatedPredicateObject;
use crate::syntax::Coordination;
use crate::syntax::CoordinationJunction;
use crate::syntax::DeonticPredicate;
use crate::syntax::DependentAttachment;
use crate::syntax::DependentClause;
use crate::syntax::EllipticalClause;
use crate::syntax::ExceptionConjunct;
use crate::syntax::ExceptionRider;
use crate::syntax::IndependentClause;
use crate::syntax::InfinitiveMarker;
use crate::syntax::Modal;
use crate::syntax::PassivePredicate;
use crate::syntax::Predicate;
use crate::syntax::PredicateAdjunct;
use crate::syntax::PredicateComplement;
use crate::syntax::PredicateElement;
use crate::syntax::PredicateExpression;
use crate::syntax::PredicateHead;
use crate::syntax::PredicateObject;
use crate::syntax::PredicateObjectCoordination;
use crate::syntax::PreverbModifier;
use crate::syntax::ProPredicate;
use crate::syntax::RelativeBody;
use crate::syntax::RelativeMarker;
use crate::syntax::RestrictionCoordination;
use crate::syntax::RestrictionRun;
use crate::syntax::SubordinateBody;

mod lowering;
mod reduction;
mod rules;

#[cfg(test)]
mod tests;

pub(super) use lowering::coordinated_modifier_as_adjectives;
pub(super) use lowering::finish_infinitive;
pub(super) use lowering::finish_reduced_recipient_passive;
pub(super) use lowering::finish_simple_clause;
pub(super) use lowering::lower_clause;
pub(super) use reduction::accepts_predicate_prefix;
pub(super) use reduction::reduce_clause;
pub(crate) use reduction::reduce_generated_recipient_passive_nominal_adjunct_features;
pub(super) use reduction::reduction_cost;
pub(super) use rules::add_rules;

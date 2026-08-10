//! Checked construction, projection, and rendering API for nonfinite clauses.
//!
//! Every builder validates the generated F01 declaration before returning a
//! sealed syntax value. Matching `parts_*` functions expose immutable inverse
//! projections without reopening the representation.

use deckmaste_construction_compiler::runtime::DeclarationViolation;

use crate::RenderError;
use crate::syntax::GerundClause;
use crate::syntax::InfinitiveClause;
use crate::syntax::Predicate;

fn staged(predicate: &Predicate) -> Result<crate::grammar::VerbPhrase, DeclarationViolation> {
    crate::constructions::predicate::inverse_public_predicate(predicate)
}

fn projected(predicate: crate::grammar::VerbPhrase) -> Predicate {
    let crate::constructions::predicate::FinishedPredicate {
        modal: None,
        predicate,
        ..
    } = crate::constructions::predicate::project_public_predicate(predicate)
        .expect("a generated nonfinite inverse remains a valid predicate")
    else {
        unreachable!("a generated nonfinite inverse has no finite modal")
    };
    predicate
}

/// Builds an unnegated `to`-infinitive from a complete infinitive predicate.
///
/// # Errors
///
/// Returns a declaration violation when the predicate is not a complete,
/// invertible infinitive form.
pub fn build_infinitive_to(
    predicate: &Predicate,
) -> Result<InfinitiveClause, DeclarationViolation> {
    crate::constructions::nonfinite::build_infinitive_to(staged(predicate)?)
}

/// Projects the predicate from an unnegated `to`-infinitive.
#[must_use]
pub fn parts_infinitive_to(value: &InfinitiveClause) -> Predicate {
    projected(crate::constructions::nonfinite::parts_infinitive_to(value))
}

/// Builds a negated `not to` infinitive from a complete infinitive predicate.
///
/// # Errors
///
/// Returns a declaration violation when the predicate is not a complete,
/// invertible infinitive form.
pub fn build_infinitive_not_to(
    predicate: &Predicate,
) -> Result<InfinitiveClause, DeclarationViolation> {
    crate::constructions::nonfinite::build_infinitive_not_to(staged(predicate)?)
}

/// Projects the predicate from a negated `not to` infinitive.
#[must_use]
pub fn parts_infinitive_not_to(value: &InfinitiveClause) -> Predicate {
    projected(crate::constructions::nonfinite::parts_infinitive_not_to(
        value,
    ))
}

/// Builds a base gerund clause from a complete present-participle predicate.
///
/// # Errors
///
/// Returns a declaration violation when the predicate is not a complete,
/// invertible present-participle form.
pub fn build_gerund_clause_base(
    predicate: &Predicate,
) -> Result<GerundClause, DeclarationViolation> {
    crate::constructions::nonfinite::build_gerund_clause_base(staged(predicate)?)
}

/// Projects the predicate from a base gerund clause.
#[must_use]
pub fn parts_gerund_clause_base(value: &GerundClause) -> Predicate {
    projected(crate::constructions::nonfinite::parts_gerund_clause_base(
        value,
    ))
}

/// Attaches a trailing `rather than` gerund alternative.
///
/// # Errors
///
/// Returns a declaration violation when either clause is outside the complete
/// generated gerund domain.
pub fn build_gerund_clause_subordinate_after(
    matrix: GerundClause,
    alternative: GerundClause,
) -> Result<GerundClause, DeclarationViolation> {
    crate::constructions::nonfinite::build_gerund_clause_subordinate_after(matrix, alternative)
}

/// Projects the matrix and alternative from a trailing `rather than` clause.
#[must_use]
pub fn parts_gerund_clause_subordinate_after(value: &GerundClause) -> (GerundClause, GerundClause) {
    crate::constructions::nonfinite::parts_gerund_clause_subordinate_after(value)
}

/// Renders a checked infinitive through its declaration-generated inverse.
///
/// # Errors
///
/// Returns an error when a nested lexical item has no surface form or the
/// value does not match exactly one admitted nonfinite construction.
pub fn render_infinitive(
    value: &InfinitiveClause,
    name: &str,
    is_legendary: bool,
) -> Result<String, RenderError> {
    crate::renderer::render_infinitive_clause(value, name, is_legendary)
}

/// Renders a checked gerund through its declaration-generated inverse.
///
/// # Errors
///
/// Returns an error when a nested lexical item has no surface form or the
/// value does not match exactly one admitted nonfinite construction.
pub fn render_gerund(
    value: &GerundClause,
    name: &str,
    is_legendary: bool,
) -> Result<String, RenderError> {
    crate::renderer::render_gerund_clause(value, name, is_legendary)
}

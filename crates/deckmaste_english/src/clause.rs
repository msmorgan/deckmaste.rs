//! Checked construction, projection, and rendering API for clauses.
//!
//! Every builder validates its generated declaration before returning a sealed
//! syntax value. Matching `parts_*` functions expose immutable inverse
//! projections without reopening the representation.

use deckmaste_construction_compiler::runtime::DeclarationViolation;

use crate::RenderError;
use crate::features::Conjunction;
use crate::syntax::AdjectivePhrase;
use crate::syntax::Clause;
use crate::syntax::CoordinatedAdjectivePhrase;
use crate::syntax::ExceptionRider;
use crate::syntax::ExceptionRiderList;
use crate::syntax::GerundClause;
use crate::syntax::InfinitiveClause;
use crate::syntax::NounPhrase;
use crate::syntax::ObjectGapPredicate;
use crate::syntax::Predicate;
use crate::syntax::PrepositionalPhrase;
use crate::syntax::RelativeClause;
use crate::syntax::RelativeMarker;
use crate::syntax::RestrictionCoordination;
use crate::syntax::RestrictionMember;
use crate::syntax::Subordinator;
use crate::word::AuxiliaryInstance;
use crate::word::Vocab;

fn contracted_relative_subject(
    auxiliary: AuxiliaryInstance,
) -> crate::grammar::ContractedSubjectAuxiliary {
    crate::grammar::ContractedSubjectAuxiliary {
        subject: crate::syntax::Subject(crate::syntax::NounPhrase::from_demonstrative_declaration(
            crate::syntax::Demonstrative::That,
        )),
        auxiliary,
    }
}

/// Builds a zero-marker relative whose predicate retains one object gap.
///
/// # Errors
///
/// Returns a declaration violation when the subject agreement or predicate
/// valency does not satisfy the object-relative construction.
pub fn build_relative_object(
    subject: NounPhrase,
    predicate: ObjectGapPredicate,
) -> Result<RelativeClause, DeclarationViolation> {
    crate::constructions::relative::checked_build_relative_object(subject, predicate)
}

/// Projects the subject and object-gap predicate of an object relative.
///
/// # Errors
///
/// Returns a declaration violation when `value` selects another relative form.
pub fn parts_relative_object(
    value: &RelativeClause,
) -> Result<(NounPhrase, ObjectGapPredicate), DeclarationViolation> {
    crate::constructions::relative::checked_parts_relative_object(value)
}

/// Builds a zero-marker object relative with a subject-contracted auxiliary.
///
/// # Errors
///
/// Returns a declaration violation when the auxiliary, subject agreement, or
/// object-gap predicate cannot form the contracted construction.
pub fn build_relative_object_contracted_subject(
    subject: NounPhrase,
    auxiliary: AuxiliaryInstance,
    predicate: ObjectGapPredicate,
) -> Result<RelativeClause, DeclarationViolation> {
    crate::constructions::relative::checked_build_relative_object_contracted_subject(
        crate::grammar::ContractedSubjectAuxiliary {
            subject: crate::syntax::Subject(subject),
            auxiliary,
        },
        predicate,
    )
}

/// Projects the subject, contracted auxiliary, and object-gap predicate.
///
/// # Errors
///
/// Returns a declaration violation when `value` selects another relative form.
pub fn parts_relative_object_contracted_subject(
    value: &RelativeClause,
) -> Result<(NounPhrase, AuxiliaryInstance, ObjectGapPredicate), DeclarationViolation> {
    let (subject_auxiliary, predicate) =
        crate::constructions::relative::checked_parts_relative_object_contracted_subject(value)?;
    Ok((
        subject_auxiliary.subject.0,
        subject_auxiliary.auxiliary,
        predicate,
    ))
}

/// Builds a `that` relative contracted with its predicate's first auxiliary.
///
/// # Errors
///
/// Returns a declaration violation when the auxiliary and predicate do not
/// form one complete agreement-preserving subject contraction.
pub fn build_relative_subject_contracted_auxiliary(
    auxiliary: AuxiliaryInstance,
    predicate: Predicate,
) -> Result<RelativeClause, DeclarationViolation> {
    crate::constructions::relative::checked_build_relative_subject_contracted_auxiliary(
        contracted_relative_subject(auxiliary),
        predicate,
    )
}

/// Projects the contracted auxiliary and remaining complete predicate.
///
/// # Errors
///
/// Returns a declaration violation when `value` selects another relative form.
pub fn parts_relative_subject_contracted_auxiliary(
    value: &RelativeClause,
) -> Result<(AuxiliaryInstance, Predicate), DeclarationViolation> {
    let (subject_auxiliary, predicate) =
        crate::constructions::relative::checked_parts_relative_subject_contracted_auxiliary(value)?;
    Ok((subject_auxiliary.auxiliary, predicate))
}

/// Builds an explicit-marker subject relative.
///
/// # Errors
///
/// Returns a declaration violation for a zero marker, incomplete predicate,
/// or predicate reserved for a more specific relative construction.
pub fn build_relative_subject(
    marker: RelativeMarker,
    predicate: Predicate,
) -> Result<RelativeClause, DeclarationViolation> {
    crate::constructions::relative::checked_build_relative_subject(marker, predicate)
}

/// Projects the explicit marker and predicate of a subject relative.
///
/// # Errors
///
/// Returns a declaration violation when `value` selects another relative form.
pub fn parts_relative_subject(
    value: &RelativeClause,
) -> Result<(RelativeMarker, Predicate), DeclarationViolation> {
    crate::constructions::relative::checked_parts_relative_subject(value)
}

/// Builds an explicit-marker plural subject relative with distributive `each`.
///
/// # Errors
///
/// Returns a declaration violation unless the predicate is complete, plural,
/// uncontracted, and does not already contain distributive `each`.
pub fn build_relative_subject_distributive_each(
    marker: RelativeMarker,
    predicate: Predicate,
) -> Result<RelativeClause, DeclarationViolation> {
    crate::constructions::relative::checked_build_relative_subject_distributive_each(
        marker, predicate,
    )
}

/// Projects the marker and predicate without its distributive `each` witness.
///
/// # Errors
///
/// Returns a declaration violation when `value` selects another relative form.
pub fn parts_relative_subject_distributive_each(
    value: &RelativeClause,
) -> Result<(RelativeMarker, Predicate), DeclarationViolation> {
    crate::constructions::relative::checked_parts_relative_subject_distributive_each(value)
}

macro_rules! contracted_copular_facade {
    ($build:ident, $checked:ident, $parts:ident, $checked_parts:ident, $ty:ty) => {
        /// Builds a `that` relative contracted with a copula and the typed
        /// complement.
        ///
        /// # Errors
        ///
        /// Returns a declaration violation when the auxiliary is not a
        /// contractible agreeing copula or the complement has the wrong
        /// relative-copular class.
        pub fn $build(
            auxiliary: AuxiliaryInstance,
            complement: $ty,
        ) -> Result<RelativeClause, DeclarationViolation> {
            crate::constructions::relative::$checked(
                contracted_relative_subject(auxiliary),
                complement,
            )
        }

        /// Projects the contracted copula and typed complement.
        ///
        /// # Errors
        ///
        /// Returns a declaration violation when `value` selects another
        /// relative form.
        pub fn $parts(
            value: &RelativeClause,
        ) -> Result<(AuxiliaryInstance, $ty), DeclarationViolation> {
            let (subject_auxiliary, complement) =
                crate::constructions::relative::$checked_parts(value)?;
            Ok((subject_auxiliary.auxiliary, complement))
        }
    };
}

contracted_copular_facade!(
    build_relative_contracted_copular_noun,
    checked_build_relative_contracted_copular_noun,
    parts_relative_contracted_copular_noun,
    checked_parts_relative_contracted_copular_noun,
    NounPhrase
);
contracted_copular_facade!(
    build_relative_contracted_copular_adjective,
    checked_build_relative_contracted_copular_adjective,
    parts_relative_contracted_copular_adjective,
    checked_parts_relative_contracted_copular_adjective,
    AdjectivePhrase
);
contracted_copular_facade!(
    build_relative_contracted_copular_prepositional,
    checked_build_relative_contracted_copular_prepositional,
    parts_relative_contracted_copular_prepositional,
    checked_parts_relative_contracted_copular_prepositional,
    PrepositionalPhrase
);
contracted_copular_facade!(
    build_relative_contracted_copular_coordinated_adjective,
    checked_build_relative_contracted_copular_coordinated_adjective,
    parts_relative_contracted_copular_coordinated_adjective,
    checked_parts_relative_contracted_copular_coordinated_adjective,
    CoordinatedAdjectivePhrase
);

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

/// Renders a sealed infinitive through its checked inverse path.
///
/// # Errors
///
/// Returns an error when a nested lexical item has no surface form or the
/// value is outside the admitted infinitive domain.
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

/// Builds a fronted ordinary adverb attachment.
///
/// # Errors
///
/// Returns a declaration violation when the supplied values do not satisfy
/// this construction.
pub fn build_clause_adverb_before(
    adverb: Vocab,
    host: Clause,
) -> Result<Clause, DeclarationViolation> {
    crate::constructions::attachment::build_clause_adverb_before(adverb, host)
}

/// Projects a fronted ordinary adverb attachment.
#[must_use]
pub fn parts_clause_adverb_before(value: &Clause) -> (Vocab, Clause) {
    crate::constructions::attachment::parts_clause_adverb_before(value)
}

/// Builds a comma-delimited fronted sentence adverbial.
///
/// # Errors
///
/// Returns a declaration violation when the supplied values do not satisfy
/// this construction.
pub fn build_clause_sentence_adverbial_before(
    adverb: Vocab,
    host: Clause,
) -> Result<Clause, DeclarationViolation> {
    crate::constructions::attachment::build_clause_sentence_adverbial_before(adverb, host)
}

/// Projects a fronted sentence adverbial.
#[must_use]
pub fn parts_clause_sentence_adverbial_before(value: &Clause) -> (Vocab, Clause) {
    crate::constructions::attachment::parts_clause_sentence_adverbial_before(value)
}

/// Builds a comma-delimited fronted prepositional attachment.
///
/// # Errors
///
/// Returns a declaration violation when the supplied values do not satisfy
/// this construction.
pub fn build_clause_prepositional_before(
    preposition: PrepositionalPhrase,
    host: Clause,
) -> Result<Clause, DeclarationViolation> {
    crate::constructions::attachment::build_clause_prepositional_before(preposition, host)
}

/// Projects a fronted prepositional attachment.
#[must_use]
pub fn parts_clause_prepositional_before(value: &Clause) -> (PrepositionalPhrase, Clause) {
    crate::constructions::attachment::parts_clause_prepositional_before(value)
}

/// Builds a comma-delimited fronted finite subordinate clause.
///
/// # Errors
///
/// Returns a declaration violation when the supplied values do not satisfy
/// this construction.
pub fn build_clause_subordinate_before(
    subordinator: Subordinator,
    condition: Clause,
    host: Clause,
) -> Result<Clause, DeclarationViolation> {
    crate::constructions::attachment::build_clause_subordinate_before(subordinator, condition, host)
}

/// Projects a fronted finite subordinate clause.
#[must_use]
pub fn parts_clause_subordinate_before(value: &Clause) -> (Subordinator, Clause, Clause) {
    crate::constructions::attachment::parts_clause_subordinate_before(value)
}

/// Builds a fronted `while` gerund clause.
///
/// # Errors
///
/// Returns a declaration violation when the supplied values do not satisfy
/// this construction.
pub fn build_clause_subordinate_gerund_before(
    gerund: GerundClause,
    host: Clause,
) -> Result<Clause, DeclarationViolation> {
    crate::constructions::attachment::build_clause_subordinate_gerund_before(gerund, host)
}

/// Projects a fronted `while` gerund clause.
#[must_use]
pub fn parts_clause_subordinate_gerund_before(value: &Clause) -> (GerundClause, Clause) {
    crate::constructions::attachment::parts_clause_subordinate_gerund_before(value)
}

/// Builds a trailing elliptical subordinate clause.
///
/// # Errors
///
/// Returns a declaration violation when the supplied values do not satisfy
/// this construction.
pub fn build_clause_subordinate_after_elliptical(
    host: Clause,
    subordinator: Subordinator,
    adjective: AdjectivePhrase,
) -> Result<Clause, DeclarationViolation> {
    crate::constructions::attachment::build_clause_subordinate_after_elliptical(
        host,
        subordinator,
        adjective,
    )
}

/// Projects a trailing elliptical subordinate clause.
#[must_use]
pub fn parts_clause_subordinate_after_elliptical(
    value: &Clause,
) -> (Clause, Subordinator, AdjectivePhrase) {
    crate::constructions::attachment::parts_clause_subordinate_after_elliptical(value)
}

/// Builds a trailing finite subordinate clause without a comma.
///
/// # Errors
///
/// Returns a declaration violation when the supplied values do not satisfy
/// this construction.
pub fn build_clause_subordinate_after(
    host: Clause,
    subordinator: Subordinator,
    condition: Clause,
) -> Result<Clause, DeclarationViolation> {
    crate::constructions::attachment::build_clause_subordinate_after(host, subordinator, condition)
}

/// Projects a trailing finite subordinate clause without a comma.
#[must_use]
pub fn parts_clause_subordinate_after(value: &Clause) -> (Clause, Subordinator, Clause) {
    crate::constructions::attachment::parts_clause_subordinate_after(value)
}

/// Builds a comma-delimited trailing finite subordinate clause.
///
/// # Errors
///
/// Returns a declaration violation when the supplied values do not satisfy
/// this construction.
pub fn build_clause_subordinate_after_comma(
    host: Clause,
    subordinator: Subordinator,
    condition: Clause,
) -> Result<Clause, DeclarationViolation> {
    crate::constructions::attachment::build_clause_subordinate_after_comma(
        host,
        subordinator,
        condition,
    )
}

/// Projects a comma-delimited trailing finite subordinate clause.
#[must_use]
pub fn parts_clause_subordinate_after_comma(value: &Clause) -> (Clause, Subordinator, Clause) {
    crate::constructions::attachment::parts_clause_subordinate_after_comma(value)
}

/// Builds a trailing `rather than` bare-infinitive clause.
///
/// # Errors
///
/// Returns a declaration violation when the supplied values do not satisfy
/// this construction.
pub fn build_clause_subordinate_after_infinitive(
    host: Clause,
    predicate: &Predicate,
) -> Result<Clause, DeclarationViolation> {
    crate::constructions::attachment::build_clause_subordinate_after_infinitive(
        host,
        staged(predicate)?,
    )
}

/// Projects a trailing `rather than` clause.
#[must_use]
pub fn parts_clause_subordinate_after_infinitive(value: &Clause) -> (Clause, Predicate) {
    let (host, predicate) =
        crate::constructions::attachment::parts_clause_subordinate_after_infinitive(value);
    (host, projected(predicate))
}

/// Builds a one-clause exception rider.
///
/// # Errors
///
/// Returns a declaration violation when the supplied values do not satisfy
/// this construction.
pub fn build_exception_rider_single(
    clause: Clause,
) -> Result<ExceptionRider, DeclarationViolation> {
    crate::constructions::attachment::build_exception_rider_single(clause)
}

/// Projects the clause from a one-clause exception rider.
#[must_use]
pub fn parts_exception_rider_single(value: &ExceptionRider) -> Clause {
    crate::constructions::attachment::parts_exception_rider_single(value)
}

/// Appends a bare-conjunction clause to an exception rider.
///
/// # Errors
///
/// Returns a declaration violation when the supplied values do not satisfy
/// this construction.
pub fn build_exception_rider_conjoined(
    rider: ExceptionRider,
    conjunction: Conjunction,
    clause: Clause,
) -> Result<ExceptionRider, DeclarationViolation> {
    crate::constructions::attachment::build_exception_rider_conjoined(rider, conjunction, clause)
}

/// Projects a bare-conjunction exception rider.
#[must_use]
pub fn parts_exception_rider_conjoined(
    value: &ExceptionRider,
) -> (ExceptionRider, Conjunction, Clause) {
    crate::constructions::attachment::parts_exception_rider_conjoined(value)
}

/// Builds or extends a comma-open exception rider.
///
/// # Errors
///
/// Returns a declaration violation when the supplied values do not satisfy
/// this construction.
pub fn build_exception_rider_comma(
    rider: Option<ExceptionRider>,
    list: Option<ExceptionRiderList>,
    clause: Clause,
) -> Result<ExceptionRiderList, DeclarationViolation> {
    crate::constructions::attachment::build_exception_rider_comma(rider, list, clause)
}

/// Projects one comma-open exception-rider step.
#[must_use]
pub fn parts_exception_rider_comma(
    value: &ExceptionRiderList,
) -> (Option<ExceptionRider>, Option<ExceptionRiderList>, Clause) {
    crate::constructions::attachment::parts_exception_rider_comma(value)
}

/// Closes a comma-open exception rider with an Oxford conjunction.
///
/// # Errors
///
/// Returns a declaration violation when the supplied values do not satisfy
/// this construction.
pub fn build_exception_rider_oxford(
    rider: ExceptionRiderList,
    conjunction: Conjunction,
    clause: Clause,
) -> Result<ExceptionRider, DeclarationViolation> {
    crate::constructions::attachment::build_exception_rider_oxford(rider, conjunction, clause)
}

/// Projects an Oxford exception-rider close.
#[must_use]
pub fn parts_exception_rider_oxford(
    value: &ExceptionRider,
) -> (ExceptionRiderList, Conjunction, Clause) {
    crate::constructions::attachment::parts_exception_rider_oxford(value)
}

/// Attaches a complete or comma-open exception rider to a host clause.
///
/// # Errors
///
/// Returns a declaration violation when the supplied values do not satisfy
/// this construction.
pub fn build_clause_excepted(
    host: Clause,
    rider: Option<ExceptionRider>,
    list: Option<ExceptionRiderList>,
) -> Result<Clause, DeclarationViolation> {
    crate::constructions::attachment::build_clause_excepted(host, rider, list)
}

/// Projects an exception rider from its host clause.
#[must_use]
pub fn parts_clause_excepted(
    value: &Clause,
) -> (Clause, Option<ExceptionRider>, Option<ExceptionRiderList>) {
    crate::constructions::attachment::parts_clause_excepted(value)
}

/// Builds one typed restriction member.
///
/// # Errors
///
/// Returns a declaration violation when the supplied values do not satisfy
/// this construction.
pub fn build_clause_restriction_member(
    preposition: Option<PrepositionalPhrase>,
    condition: Option<Clause>,
    temporal: Option<NounPhrase>,
) -> Result<RestrictionMember, DeclarationViolation> {
    crate::constructions::attachment::build_clause_restriction_member(
        preposition,
        condition,
        temporal,
    )
}

/// Projects one typed restriction member.
#[must_use]
pub fn parts_clause_restriction_member(
    value: &RestrictionMember,
) -> (
    Option<PrepositionalPhrase>,
    Option<Clause>,
    Option<NounPhrase>,
) {
    crate::constructions::attachment::parts_clause_restriction_member(value)
}

/// Creates one continuation in a validated restriction run.
#[must_use]
pub fn restriction_coordination(
    conjunction: Option<Conjunction>,
    member: RestrictionMember,
) -> RestrictionCoordination {
    RestrictionCoordination::from_declaration_parts(conjunction, member)
}

/// Builds a two-or-more-member trailing restriction run.
///
/// # Errors
///
/// Returns a declaration violation when the supplied values do not satisfy
/// this construction.
pub fn build_clause_restriction_run(
    host: Clause,
    first: RestrictionMember,
    rest: Vec<RestrictionCoordination>,
) -> Result<Clause, DeclarationViolation> {
    crate::constructions::attachment::build_clause_restriction_run(host, first, rest)
}

/// Projects a trailing restriction run.
#[must_use]
pub fn parts_clause_restriction_run(
    value: &Clause,
) -> (Clause, RestrictionMember, Vec<RestrictionCoordination>) {
    crate::constructions::attachment::parts_clause_restriction_run(value)
}

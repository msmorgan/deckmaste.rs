//! Checked construction, projection, and rendering API for clauses.
//!
//! Every builder validates its generated declaration before returning a sealed
//! syntax value. Immutable semantic accessors expose the resulting structure
//! without reopening construction-specific inverse projections.

use deckmaste_construction_compiler::runtime::DeclarationViolation;

use crate::RenderError;
pub use crate::constructions::clause::build_clause_existential;
use crate::features::Conjunction;
use crate::syntax::AdjectivePhrase;
use crate::syntax::Clause;
use crate::syntax::CoordinatedAdjectivePhrase;
use crate::syntax::ExceptionRider;
use crate::syntax::ExceptionRiderList;
use crate::syntax::GerundClause;
use crate::syntax::IndependentClause;
use crate::syntax::InfinitiveClause;
use crate::syntax::NounPhrase;
use crate::syntax::ObjectGapPredicate;
use crate::syntax::Predicate;
use crate::syntax::PredicateExpression;
use crate::syntax::PrepositionalPhrase;
use crate::syntax::RelativeClause;
use crate::syntax::RelativeMarker;
use crate::syntax::RestrictionCoordination;
use crate::syntax::RestrictionMember;
use crate::syntax::Subordinator;
use crate::word::AuxiliaryInstance;
use crate::word::Vocab;

fn simple_clause(
    construction: &'static str,
    value: &Clause,
) -> Result<crate::grammar::SimpleClause, DeclarationViolation> {
    let Clause::Independent(IndependentClause::Finite(finite)) = value else {
        return Err(DeclarationViolation {
            construction,
            requirement: "the continuation is one uncoordinated finite clause",
        });
    };
    let PredicateExpression::Simple(predicate) = finite.predicate() else {
        return Err(DeclarationViolation {
            construction,
            requirement: "the continuation is one uncoordinated finite clause",
        });
    };
    Ok(crate::grammar::SimpleClause {
        subject: finite.subject().cloned(),
        predicate: crate::constructions::predicate::inverse_public_predicate(predicate)?,
        attachment: None,
    })
}

/// Coordinates a clause with a second finite clause without a comma.
///
/// An explicit-subject continuation remains a complete clause member. A
/// standalone imperative may be adopted by an eligible addressee host; use
/// [`build_clause_coordination_shared_predicate`] for an ordinary finite
/// predicate that shares a third-person subject.
///
/// # Errors
///
/// Returns a declaration violation when the continuation is not a simple
/// finite clause or the two clauses cannot form the declared coordination.
pub fn build_clause_coordination(
    first: Clause,
    conjunction: Conjunction,
    next: &Clause,
) -> Result<Clause, DeclarationViolation> {
    let next = simple_clause("clause_coordination", next)?;
    crate::constructions::clause::build_clause_coordination(first, conjunction, next)
}

/// Coordinates a clause with a comma-delimited finite clause.
///
/// # Errors
///
/// Returns a declaration violation when the continuation is not a simple
/// finite clause or the requested grouping violates finite agreement and
/// subject-sharing constraints.
pub fn build_clause_coordination_comma(
    first: Clause,
    conjunction: Conjunction,
    next: &Clause,
) -> Result<Clause, DeclarationViolation> {
    let next = simple_clause("clause_coordination_comma", next)?;
    crate::constructions::clause::build_clause_coordination_comma(first, conjunction, next)
}

/// Appends an asyndetic comma-delimited finite clause.
///
/// # Errors
///
/// Returns a declaration violation when the continuation is not a simple
/// finite clause or cannot share the host's subject and agreement.
pub fn build_clause_coordination_asyndetic(
    first: Clause,
    next: &Clause,
) -> Result<Clause, DeclarationViolation> {
    let next = simple_clause("clause_coordination_asyndetic", next)?;
    crate::constructions::clause::build_clause_coordination_asyndetic(first, next)
}

fn shared_predicate(
    predicate: &Predicate,
) -> Result<crate::grammar::SimpleClause, DeclarationViolation> {
    Ok(crate::grammar::SimpleClause {
        subject: None,
        predicate: crate::constructions::predicate::inverse_public_predicate(predicate)?,
        attachment: None,
    })
}

/// Coordinates a finite host with an ordinary predicate sharing its subject.
///
/// # Errors
///
/// Returns a declaration violation when the predicate is incomplete or its
/// finite agreement is incompatible with the host.
pub fn build_clause_coordination_shared_predicate(
    first: Clause,
    conjunction: Conjunction,
    predicate: &Predicate,
) -> Result<Clause, DeclarationViolation> {
    crate::constructions::clause::build_clause_coordination(
        first,
        conjunction,
        shared_predicate(predicate)?,
    )
}

/// Coordinates a finite host with a comma-delimited predicate sharing its
/// subject.
///
/// # Errors
///
/// Returns a declaration violation when the predicate is incomplete or its
/// finite agreement is incompatible with the host.
pub fn build_clause_coordination_shared_predicate_comma(
    first: Clause,
    conjunction: Conjunction,
    predicate: &Predicate,
) -> Result<Clause, DeclarationViolation> {
    crate::constructions::clause::build_clause_coordination_comma(
        first,
        conjunction,
        shared_predicate(predicate)?,
    )
}

/// Appends an asyndetic predicate through the shared-subject declaration.
///
/// # Errors
///
/// Returns a declaration violation when the predicate is incomplete or the
/// host cannot license this asyndetic subject-sharing form.
pub fn build_clause_coordination_shared_predicate_asyndetic(
    first: Clause,
    predicate: &Predicate,
) -> Result<Clause, DeclarationViolation> {
    crate::constructions::clause::build_clause_coordination_asyndetic(
        first,
        shared_predicate(predicate)?,
    )
}

fn shared_copular_predicate(
    copula: AuxiliaryInstance,
    complement: NounPhrase,
    preposition: PrepositionalPhrase,
) -> Result<Predicate, DeclarationViolation> {
    crate::constructions::clause::build_shared_copular_predicate(copula, complement, preposition)
}

/// Appends the narrow shared-subject additive copular continuation.
///
/// # Errors
///
/// Returns a declaration violation unless the copula is indicative, the
/// complement is nominal, the adjunct is the declared additive `in` phrase,
/// and the host supplies compatible finite agreement.
pub fn build_clause_coordination_copular_noun_prepositional(
    first: Clause,
    conjunction: Conjunction,
    copula: AuxiliaryInstance,
    complement: NounPhrase,
    preposition: PrepositionalPhrase,
) -> Result<Clause, DeclarationViolation> {
    let predicate = shared_copular_predicate(copula, complement, preposition)?;
    crate::constructions::clause::build_clause_coordination_copular_noun_prepositional(
        first,
        conjunction,
        predicate,
    )
}

/// Appends a comma-delimited additive copular continuation.
///
/// # Errors
///
/// Returns a declaration violation unless the copula, complement, additive
/// adjunct, and host agreement satisfy the generated declaration.
pub fn build_clause_coordination_copular_noun_prepositional_comma(
    first: Clause,
    conjunction: Conjunction,
    copula: AuxiliaryInstance,
    complement: NounPhrase,
    preposition: PrepositionalPhrase,
) -> Result<Clause, DeclarationViolation> {
    let predicate = shared_copular_predicate(copula, complement, preposition)?;
    crate::constructions::clause::build_clause_coordination_copular_noun_prepositional_comma(
        first,
        conjunction,
        predicate,
    )
}

/// Appends an asyndetic additive copular continuation.
///
/// # Errors
///
/// Returns a declaration violation unless the copula, complement, additive
/// adjunct, and host agreement satisfy the generated declaration.
pub fn build_clause_coordination_copular_noun_prepositional_asyndetic(
    first: Clause,
    copula: AuxiliaryInstance,
    complement: NounPhrase,
    preposition: PrepositionalPhrase,
) -> Result<Clause, DeclarationViolation> {
    let predicate = shared_copular_predicate(copula, complement, preposition)?;
    crate::constructions::clause::build_clause_coordination_copular_noun_prepositional_asyndetic(
        first, predicate,
    )
}

fn contracted_relative_subject(
    auxiliary: AuxiliaryInstance,
) -> Result<crate::grammar::ContractedSubjectAuxiliary, DeclarationViolation> {
    Ok(crate::grammar::ContractedSubjectAuxiliary {
        subject: crate::syntax::Subject(
            crate::constructions::noun_phrase::build_noun_phrase_demonstrative(
                crate::syntax::Demonstrative::That,
            )?,
        ),
        auxiliary,
    })
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
        contracted_relative_subject(auxiliary)?,
        predicate,
    )
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

macro_rules! contracted_copular_facade {
    ($build:ident, $checked:ident, $ty:ty) => {
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
                contracted_relative_subject(auxiliary)?,
                complement,
            )
        }
    };
}

contracted_copular_facade!(
    build_relative_contracted_copular_noun,
    checked_build_relative_contracted_copular_noun,
    NounPhrase
);
contracted_copular_facade!(
    build_relative_contracted_copular_adjective,
    checked_build_relative_contracted_copular_adjective,
    AdjectivePhrase
);
contracted_copular_facade!(
    build_relative_contracted_copular_prepositional,
    checked_build_relative_contracted_copular_prepositional,
    PrepositionalPhrase
);
contracted_copular_facade!(
    build_relative_contracted_copular_coordinated_adjective,
    checked_build_relative_contracted_copular_coordinated_adjective,
    CoordinatedAdjectivePhrase
);

fn staged(predicate: &Predicate) -> Result<crate::grammar::VerbPhrase, DeclarationViolation> {
    crate::constructions::predicate::inverse_public_predicate(predicate)
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

/// Creates one continuation in a validated restriction run.
#[must_use]
pub fn restriction_coordination(
    conjunction: Option<Conjunction>,
    member: RestrictionMember,
) -> RestrictionCoordination {
    crate::constructions::attachment::restriction_coordination(conjunction, member)
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

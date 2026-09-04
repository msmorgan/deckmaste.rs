//! Compiler-derived finite, existential, and copular clause declarations.

#![allow(
    clippy::needless_pass_by_value,
    clippy::unnecessary_wraps,
    dead_code,
    reason = "declaration adapters own erased builder values and uniform fallible signatures"
)]

use deckmaste_construction_compiler::runtime::DeclarationViolation;
use deckmaste_construction_compiler::runtime::GroupData;

use crate::catalog::CatalogAtom;
use crate::constructions::predicate::FinishedPredicate;
use crate::features::Comma;
use crate::features::Conjunction;
use crate::features::Contraction;
use crate::features::Number;
use crate::features::Person;
use crate::features::PronounCase;
use crate::grammar::Agreement;
use crate::grammar::ContractedSubjectAuxiliary;
use crate::grammar::CopulaAgreement;
use crate::grammar::CopularRemainder;
use crate::grammar::Features;
use crate::grammar::PredicateForm;
use crate::grammar::SimpleClause;
use crate::grammar::VerbAnalysis;
use crate::grammar::VerbPhrase;
use crate::grammar::auxiliary_form;
use crate::grammar::fold_auxiliary_passive;
use crate::grammar::predicate_arguments_complete;
use crate::syntax::AbilityObject;
use crate::syntax::AdjectivePhrase;
use crate::syntax::AttachedPredicate;
use crate::syntax::AttachmentScope;
use crate::syntax::Clause;
use crate::syntax::ClauseAttachment;
use crate::syntax::ClauseCoordination;
use crate::syntax::ComplexClause;
use crate::syntax::CoordinatedAdjectivePhrase;
use crate::syntax::CoordinatedClauseMember;
use crate::syntax::CoordinatedIndependentClause;
use crate::syntax::Coordination;
use crate::syntax::CoordinationJunction;
use crate::syntax::Copula;
use crate::syntax::CopularComplement;
use crate::syntax::CopularPredicate;
use crate::syntax::DeonticPredicate;
use crate::syntax::DependentClause;
use crate::syntax::EllipticalClause;
use crate::syntax::ExistentialClause;
use crate::syntax::ExistentialForm;
use crate::syntax::FiniteClause;
use crate::syntax::IndependentClause;
use crate::syntax::Modal;
use crate::syntax::NounPhrase;
use crate::syntax::NumberLiteral;
use crate::syntax::PowerToughness;
use crate::syntax::Predicate;
use crate::syntax::PredicateAdjunct;
use crate::syntax::PredicateExpression;
use crate::syntax::PredicateHead;
use crate::syntax::PredicateObject;
use crate::syntax::PrepositionalPhrase;
use crate::syntax::Quantity;
use crate::syntax::QuantityKind;
use crate::syntax::QuotedAbility;
use crate::syntax::Subject;
use crate::syntax::SubordinateBody;
use crate::syntax::Subordinator;
use crate::word::Auxiliary;
use crate::word::AuxiliaryInflection;
use crate::word::AuxiliaryInstance;
use crate::word::Verb;
use crate::word::VerbSlot;
use crate::word::Vocab;

type SharedCopularPredicate = Predicate;
type CoordinatedPredicateAttachment = ClauseAttachment;
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SharedGrantPrefix(Clause);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SharedGrantBase(Clause);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SharedGrantComplement {
    Ability(CatalogAtom),
    Quoted(QuotedAbility),
}

impl SharedGrantComplement {
    fn into_object(self) -> PredicateObject {
        match self {
            Self::Ability(ability) => PredicateObject::Ability(AbilityObject {
                ability,
                argument: None,
            }),
            Self::Quoted(quoted) => PredicateObject::QuotedAbility(Box::new(quoted)),
        }
    }
}

fn violation(construction: &'static str, requirement: &'static str) -> DeclarationViolation {
    DeclarationViolation {
        construction,
        requirement,
    }
}

fn make_coordinated_predicate_attachment(
    subordinator: Subordinator,
    condition: Clause,
) -> Result<ClauseAttachment, DeclarationViolation> {
    make_coordinated_predicate_attachment_with_comma(
        "coordinated_predicate_attachment",
        subordinator,
        condition,
        Comma::Absent,
    )
}

fn make_coordinated_predicate_attachment_comma(
    subordinator: Subordinator,
    condition: Clause,
) -> Result<ClauseAttachment, DeclarationViolation> {
    make_coordinated_predicate_attachment_with_comma(
        "coordinated_predicate_attachment_comma",
        subordinator,
        condition,
        Comma::Present,
    )
}

fn make_coordinated_predicate_attachment_with_comma(
    construction: &'static str,
    subordinator: Subordinator,
    condition: Clause,
    comma: Comma,
) -> Result<ClauseAttachment, DeclarationViolation> {
    let Clause::Independent(condition) = condition else {
        return Err(violation(
            construction,
            "the postpositive condition is an independent finite clause",
        ));
    };
    Ok(ClauseAttachment::from_declaration_parts(
        crate::syntax::AttachmentPosition::AfterMatrix,
        comma,
        crate::syntax::ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
            subordinator,
            SubordinateBody::Finite(Box::new(condition)),
        )),
    ))
}

fn coordinated_predicate_attachment_parts(value: &ClauseAttachment) -> (Subordinator, Clause) {
    let crate::syntax::ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
        subordinator,
        SubordinateBody::Finite(condition),
    )) = value.payload()
    else {
        unreachable!("the coordinated-predicate attachment recognizer checks its payload")
    };
    (*subordinator, Clause::Independent((**condition).clone()))
}

fn is_coordinated_predicate_attachment(value: &ClauseAttachment) -> bool {
    postpositive_subordinator(value).is_some() && !value.comma().is_present()
}

fn is_coordinated_predicate_attachment_comma(value: &ClauseAttachment) -> bool {
    postpositive_subordinator(value).is_some() && value.comma().is_present()
}

fn make_coordinated_predicate_attachment_elliptical(
    subordinator: Subordinator,
    adjective: AdjectivePhrase,
) -> Result<ClauseAttachment, DeclarationViolation> {
    if subordinator != Subordinator::If {
        return Err(violation(
            "coordinated_predicate_attachment_elliptical",
            "the elliptical coordinated-predicate condition is introduced by `if`",
        ));
    }
    Ok(ClauseAttachment::from_declaration_parts(
        crate::syntax::AttachmentPosition::AfterMatrix,
        Comma::Absent,
        crate::syntax::ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
            subordinator,
            SubordinateBody::Elliptical(EllipticalClause::Adjective(adjective)),
        )),
    ))
}

fn coordinated_predicate_attachment_elliptical_parts(
    value: &ClauseAttachment,
) -> (Subordinator, AdjectivePhrase) {
    let crate::syntax::ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
        subordinator,
        SubordinateBody::Elliptical(EllipticalClause::Adjective(adjective)),
    )) = value.payload()
    else {
        unreachable!(
            "the elliptical coordinated-predicate attachment recognizer checks its payload"
        )
    };
    (*subordinator, adjective.clone())
}

fn is_coordinated_predicate_attachment_elliptical(value: &ClauseAttachment) -> bool {
    matches!(
        value,
        ClauseAttachment {
            position: crate::syntax::AttachmentPosition::AfterMatrix,
            comma: Comma::Absent,
            payload: crate::syntax::ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
                Subordinator::If,
                SubordinateBody::Elliptical(EllipticalClause::Adjective(_)),
            ),),
        }
    )
}

fn make_shared_grant_ability_complement(
    ability: CatalogAtom,
) -> Result<SharedGrantComplement, DeclarationViolation> {
    ability
        .is_keyword_ability()
        .then_some(SharedGrantComplement::Ability(ability))
        .ok_or_else(|| {
            violation(
                "shared_grant_ability_complement",
                "the shared grant complement is a keyword ability",
            )
        })
}

fn shared_grant_ability_complement_parts(value: &SharedGrantComplement) -> CatalogAtom {
    let SharedGrantComplement::Ability(ability) = value else {
        unreachable!("the shared-grant ability recognizer checks its variant")
    };
    ability.clone()
}

fn is_shared_grant_ability_complement(value: &SharedGrantComplement) -> bool {
    matches!(value, SharedGrantComplement::Ability(ability) if ability.is_keyword_ability())
}

fn make_shared_grant_quoted_complement(
    quoted: QuotedAbility,
) -> Result<SharedGrantComplement, DeclarationViolation> {
    Ok(SharedGrantComplement::Quoted(quoted))
}

fn shared_grant_quoted_complement_parts(value: &SharedGrantComplement) -> QuotedAbility {
    let SharedGrantComplement::Quoted(quoted) = value else {
        unreachable!("the shared-grant quote recognizer checks its variant")
    };
    quoted.clone()
}

fn is_shared_grant_quoted_complement(value: &SharedGrantComplement) -> bool {
    matches!(value, SharedGrantComplement::Quoted(_))
}

fn make_shared_grant_base(
    subject: NounPhrase,
    head: VerbAnalysis,
    complement: SharedGrantComplement,
    attachment: ClauseAttachment,
) -> Result<SharedGrantBase, DeclarationViolation> {
    if !matches!(head.instance().verb, Verb::Word(Vocab::Have)) {
        return Err(violation(
            "shared_grant_base",
            "the overt shared head is lexical have",
        ));
    }
    let predicate = crate::constructions::predicate::build_verb_phrase_base(head)?;
    let predicate = match complement {
        SharedGrantComplement::Ability(ability) => {
            crate::constructions::predicate::build_verb_phrase_ability(predicate, ability)?
        }
        SharedGrantComplement::Quoted(quoted) => {
            crate::constructions::predicate::build_verb_phrase_quoted_ability(predicate, quoted)?
        }
    };
    let host = finish_simple_clause(SimpleClause {
        subject: Some(Subject(subject)),
        predicate,
        attachment: None,
    })
    .ok_or_else(|| {
        violation(
            "shared_grant_base",
            "the overt have predicate agrees with its explicit subject",
        )
    })?;
    if postpositive_subordinator(&attachment).is_none() {
        return Err(violation(
            "shared_grant_base",
            "the first grant complement has a postpositive finite condition",
        ));
    }
    Ok(SharedGrantBase(Clause::Independent(
        IndependentClause::Complex(ComplexClause::from_declaration_parts(host, attachment)),
    )))
}

fn shared_grant_base_parts(
    value: &SharedGrantBase,
) -> (
    NounPhrase,
    VerbAnalysis,
    SharedGrantComplement,
    ClauseAttachment,
) {
    let Clause::Independent(IndependentClause::Complex(complex)) = &value.0 else {
        unreachable!("the shared-grant base recognizer checks its outer condition")
    };
    let IndependentClause::Finite(finite) = complex.host() else {
        unreachable!("the shared-grant base recognizer checks its finite host")
    };
    let subject = finite
        .subject()
        .expect("the shared-grant base recognizer checks its subject")
        .0
        .clone();
    let PredicateExpression::Simple(predicate) = finite.predicate() else {
        unreachable!("the shared-grant base recognizer checks its simple predicate")
    };
    let phrase = crate::constructions::predicate::inverse_public_predicate(predicate)
        .expect("the shared-grant base predicate has a generated inverse");
    let (phrase, complement) = if let Some((phrase, ability)) =
        crate::constructions::predicate::parts_verb_phrase_ability(&phrase)
    {
        (phrase, SharedGrantComplement::Ability(ability))
    } else {
        let (phrase, quoted) =
            crate::constructions::predicate::parts_verb_phrase_quoted_ability(&phrase)
                .expect("the shared-grant base recognizer checks its complement");
        (phrase, SharedGrantComplement::Quoted(quoted))
    };
    let head = crate::constructions::predicate::parts_verb_phrase_base(&phrase);
    (subject, head, complement, complex.attachment().clone())
}

fn is_shared_grant_base(value: &SharedGrantBase) -> bool {
    let Clause::Independent(IndependentClause::Complex(complex)) = &value.0 else {
        return false;
    };
    let IndependentClause::Finite(finite) = complex.host() else {
        return false;
    };
    let PredicateExpression::Simple(predicate) = finite.predicate() else {
        return false;
    };
    finite.subject().is_some()
        && postpositive_subordinator(complex.attachment()).is_some()
        && matches!(predicate, Predicate::Transitive(predicate)
            if matches!(predicate.head().verb().verb, Verb::Word(Vocab::Have))
                && matches!(predicate.object(), PredicateObject::Ability(AbilityObject { argument: None, .. }) | PredicateObject::QuotedAbility(_)))
}

fn checked_simple(
    construction: &'static str,
    simple: SimpleClause,
) -> Result<SimpleClause, DeclarationViolation> {
    finish_simple_clause(simple.clone())
        .is_some()
        .then_some(simple)
        .ok_or_else(|| {
            violation(
                construction,
                "the predicate is complete and projects to the declared finite clause shape",
            )
        })
}

/// Projects the generated `SimpleClause` carrier into its sealed semantic
/// clause. Keeping this adapter beside `clause_simple` makes the declaration
/// module the only place that constructs an ordinary finite clause.
pub(crate) fn finish_simple_clause(simple: SimpleClause) -> Option<IndependentClause> {
    let imperative = simple.subject.is_none()
        && simple.predicate.declaration_verb_slot() == VerbSlot::Imperative;
    let subject = simple.subject;
    let FinishedPredicate {
        modal,
        predicate,
        elided,
    } = crate::constructions::predicate::project_public_predicate(simple.predicate).ok()?;
    let has_modal = modal.is_some();
    let predicate = match (modal, elided) {
        (Some(modal), true) => Predicate::Deontic(DeonticPredicate { modal, inner: None }),
        (Some(modal), false) => Predicate::Deontic(DeonticPredicate {
            modal,
            inner: Some(Box::new(PredicateExpression::Simple(predicate))),
        }),
        (None, false) => predicate,
        (None, true) => return None,
    };
    match (subject, imperative, has_modal) {
        (None, true, false) => Some(finite_clause(None, predicate)),
        (Some(subject), false, _) => Some(finite_clause(Some(subject), predicate)),
        _ => None,
    }
}

fn finite_clause(subject: Option<Subject>, predicate: Predicate) -> IndependentClause {
    IndependentClause::Finite(FiniteClause::from_declaration_parts(
        subject,
        PredicateExpression::Simple(predicate),
    ))
}

/// Enters the generated Clause inverse from an already sealed predicate
/// expression without exposing a raw `FiniteClause` construction to callers.
pub(crate) fn linearize_predicate_expression_with<V>(
    subject: Option<&Subject>,
    expression: &PredicateExpression,
    visitor: &mut V,
) -> Result<(), deckmaste_construction_compiler::runtime::LinearizationError<V::Error>>
where
    V: deckmaste_construction_compiler::runtime::LinearizationVisitor,
{
    let clause = Clause::Independent(IndependentClause::Finite(
        FiniteClause::from_declaration_parts(subject.cloned(), expression.clone()),
    ));
    linearize_clause_clause_with(&clause, visitor)
}

fn make_simple_clause_subject(
    subject: NounPhrase,
    predicate: VerbPhrase,
) -> Result<SimpleClause, DeclarationViolation> {
    if predicate.declaration_has_distributive_each() {
        return Err(violation(
            "simple_clause_subject",
            "ordinary subject clauses do not carry floated distributive each",
        ));
    }
    checked_simple(
        "simple_clause_subject",
        SimpleClause {
            subject: Some(Subject(subject)),
            predicate,
            attachment: None,
        },
    )
}

fn simple_clause_subject_parts(value: &SimpleClause) -> (NounPhrase, VerbPhrase) {
    (
        value
            .subject
            .as_ref()
            .expect("subject construction has a subject")
            .0
            .clone(),
        value.predicate.clone(),
    )
}

fn is_simple_clause_subject(value: &SimpleClause) -> bool {
    value.attachment().is_none()
        && value.subject.is_some()
        && !value.predicate.declaration_has_distributive_each()
        && value
            .predicate
            .declaration_contracted_subject_auxiliary_parts()
            .is_none()
}

fn make_simple_clause_subject_distributive_each(
    subject: NounPhrase,
    predicate: VerbPhrase,
) -> Result<SimpleClause, DeclarationViolation> {
    let predicate = predicate
        .declaration_with_distributive_each()
        .ok_or_else(|| {
            violation(
                "simple_clause_subject_distributive_each",
                "distributive each is neither duplicated nor combined with contraction",
            )
        })?;
    checked_simple(
        "simple_clause_subject_distributive_each",
        SimpleClause {
            subject: Some(Subject(subject)),
            predicate,
            attachment: None,
        },
    )
}

fn simple_clause_subject_distributive_each_parts(value: &SimpleClause) -> (NounPhrase, VerbPhrase) {
    (
        value
            .subject
            .as_ref()
            .expect("distributive subject construction has a subject")
            .0
            .clone(),
        value
            .predicate
            .clone()
            .declaration_without_distributive_each()
            .expect("distributive inverse removes exactly one each marker"),
    )
}

fn is_simple_clause_subject_distributive_each(value: &SimpleClause) -> bool {
    value.attachment().is_none()
        && value.subject.is_some()
        && value.predicate.declaration_has_distributive_each()
}

fn make_simple_clause_contracted_subject(
    subject_auxiliary: ContractedSubjectAuxiliary,
    predicate: VerbPhrase,
) -> Result<SimpleClause, DeclarationViolation> {
    let predicate = predicate
        .declaration_with_contracted_subject_auxiliary(subject_auxiliary.auxiliary)
        .ok_or_else(|| {
            violation(
                "simple_clause_contracted_subject",
                "subject contraction is represented exactly once",
            )
        })?;
    checked_simple(
        "simple_clause_contracted_subject",
        SimpleClause {
            subject: Some(subject_auxiliary.subject),
            predicate,
            attachment: None,
        },
    )
}

fn simple_clause_contracted_subject_parts(
    value: &SimpleClause,
) -> (ContractedSubjectAuxiliary, VerbPhrase) {
    let (auxiliary, predicate) = value
        .predicate
        .declaration_contracted_subject_auxiliary_parts()
        .expect("contracted subject construction retains its first auxiliary");
    (
        ContractedSubjectAuxiliary {
            subject: value
                .subject
                .as_ref()
                .expect("contracted subject construction has a subject")
                .clone(),
            auxiliary,
        },
        predicate,
    )
}

fn is_simple_clause_contracted_subject(value: &SimpleClause) -> bool {
    value.attachment().is_none()
        && value.subject.is_some()
        && value
            .predicate
            .declaration_contracted_subject_auxiliary_parts()
            .is_some()
}

fn make_simple_clause_subjectless(
    predicate: VerbPhrase,
) -> Result<SimpleClause, DeclarationViolation> {
    // Subjectless simple clauses are also coordination intermediates. A
    // finite or infinitival predicate cannot finish as an independent clause
    // by itself, but it is well-formed here and receives its subject or host
    // inflection when the enclosing coordination lowers.
    Ok(SimpleClause {
        subject: None,
        predicate,
        attachment: None,
    })
}

fn simple_clause_subjectless_parts(value: &SimpleClause) -> VerbPhrase {
    value.predicate.clone()
}

const fn is_simple_clause_subjectless(value: &SimpleClause) -> bool {
    value.subject.is_none() && value.attachment().is_none()
}

fn make_simple_clause_subjectless_attached(
    predicate: VerbPhrase,
    attachment: ClauseAttachment,
) -> Result<SimpleClause, DeclarationViolation> {
    Ok(SimpleClause {
        subject: None,
        predicate,
        attachment: Some(attachment),
    })
}

fn simple_clause_subjectless_attached_parts(
    value: &SimpleClause,
) -> (VerbPhrase, ClauseAttachment) {
    (
        value.predicate.clone(),
        value
            .attachment()
            .cloned()
            .expect("the attached-member recognizer checks its attachment"),
    )
}

fn is_simple_clause_subjectless_attached(value: &SimpleClause) -> bool {
    value.subject.is_none() && value.attachment().is_some()
}

fn make_clause_simple(simple: SimpleClause) -> Result<Clause, DeclarationViolation> {
    finish_simple_clause(simple)
        .map(Clause::Independent)
        .ok_or_else(|| {
            violation(
                "clause_simple",
                "the simple clause projects to one independent clause",
            )
        })
}

fn inverse_simple_clause(value: &Clause) -> Option<SimpleClause> {
    let Clause::Independent(IndependentClause::Finite(value)) = value else {
        return None;
    };
    let PredicateExpression::Simple(predicate) = value.predicate() else {
        return None;
    };
    let predicate = crate::constructions::predicate::inverse_public_predicate(predicate).ok()?;
    Some(SimpleClause {
        subject: value.subject().cloned(),
        predicate,
        attachment: None,
    })
}

fn clause_simple_parts(value: &Clause) -> SimpleClause {
    inverse_simple_clause(value).expect("clause_simple admits its generated independent shapes")
}

fn is_clause_simple(value: &Clause) -> bool {
    inverse_simple_clause(value).is_some()
}

fn coordinate_clause(
    first: Clause,
    conjunction: Option<Conjunction>,
    comma: crate::features::Comma,
    next: SimpleClause,
) -> Option<Clause> {
    let Clause::Independent(first) = first else {
        return None;
    };
    if conjunction.is_none() && next.subject.is_some() {
        return None;
    }
    let junction = CoordinationJunction::from_declaration_parts(conjunction, comma);
    let coordinated = if next.subject.is_some() {
        if next.attachment.is_some() {
            return None;
        }
        let next = finish_simple_clause(next)?;
        append_complete_clause(first, junction, next)
    } else {
        let attachment = next.attachment;
        let FinishedPredicate {
            modal,
            mut predicate,
            elided,
        } = finish_predicate(next.predicate)?;
        if let Some(modal) = modal {
            predicate = Predicate::Deontic(DeonticPredicate {
                modal,
                inner: if elided {
                    None
                } else {
                    Some(Box::new(PredicateExpression::Simple(predicate)))
                },
            });
        } else if let Some(host_inflection) = finite_inflection_of_clause(&first) {
            if finite_inflection_of_predicate(&predicate).is_some_and(|predicate_inflection| {
                predicate_inflection.agreement() != host_inflection.agreement()
            }) {
                return None;
            }
            apply_finite_inflection(&mut predicate, host_inflection);
        }
        if let Some(attachment) = attachment {
            if !establishes_serial_postpositive_condition(&first, &attachment) {
                return None;
            }
            predicate = Predicate::Attached(AttachedPredicate {
                scope: AttachmentScope::from_declaration_parts(predicate, attachment),
            });
        }
        if starts_new_clause_group(&first, &junction) || !accepts_shared_predicate(&first) {
            append_subjectless_clause(first, &junction, predicate)?
        } else {
            append_shared_predicate(first, junction, predicate)?
        }
    };
    Some(Clause::Independent(coordinated))
}

fn coordinate_shared_predicate(
    first: Clause,
    conjunction: Option<Conjunction>,
    comma: crate::features::Comma,
    predicate: Predicate,
) -> Option<Clause> {
    let Clause::Independent(first) = first else {
        return None;
    };
    if let (Some(host), Some(member)) = (
        finite_inflection_of_clause(&first),
        finite_inflection_of_predicate(&predicate),
    ) && host.agreement() != member.agreement()
    {
        return None;
    }
    let coordinated = append_shared_predicate(
        first,
        CoordinationJunction::from_declaration_parts(conjunction, comma),
        predicate,
    )?;
    Some(Clause::Independent(coordinated))
}

#[derive(Clone, Copy)]
pub(super) enum FiniteInflection {
    Present(Agreement),
    Past(Agreement),
}

impl FiniteInflection {
    const fn agreement(self) -> Agreement {
        match self {
            Self::Present(agreement) | Self::Past(agreement) => agreement,
        }
    }

    const fn verb_slot(self) -> VerbSlot {
        match self {
            Self::Present(Agreement { person, number }) => VerbSlot::Present { person, number },
            Self::Past(Agreement { person, number }) => VerbSlot::Past { person, number },
        }
    }

    const fn auxiliary_inflection(self) -> AuxiliaryInflection {
        match self {
            Self::Present(Agreement { person, number }) => {
                AuxiliaryInflection::Present { person, number }
            }
            Self::Past(Agreement { person, number }) => {
                AuxiliaryInflection::Past { person, number }
            }
        }
    }
}

pub(super) fn finite_inflection_of_clause(clause: &IndependentClause) -> Option<FiniteInflection> {
    match clause {
        IndependentClause::Finite(finite) => finite
            .subject()
            .and_then(|_| finite_inflection_of_expression(finite.predicate())),
        IndependentClause::Existential(_) => None,
        IndependentClause::Complex(complex) => finite_inflection_of_clause(complex.host()),
        IndependentClause::Coordinated(coordination) => {
            coordination
                .rest
                .last()
                .and_then(|continuation| match &continuation.member {
                    CoordinatedClauseMember::Independent(clause) => {
                        finite_inflection_of_clause(clause)
                    }
                })
        }
    }
}

fn finite_inflection_of_expression(expression: &PredicateExpression) -> Option<FiniteInflection> {
    match expression {
        PredicateExpression::Simple(predicate) => finite_inflection_of_predicate(predicate),
        PredicateExpression::Coordinated(coordination) => coordination
            .conjuncts()
            .first()
            .and_then(finite_inflection_of_expression),
    }
}

pub(super) fn finite_inflection_of_predicate(predicate: &Predicate) -> Option<FiniteInflection> {
    match predicate {
        Predicate::Transitive(predicate) => finite_inflection_of_head(&predicate.head),
        Predicate::Intransitive(predicate) => finite_inflection_of_head(&predicate.head),
        Predicate::Passive(predicate) => finite_inflection_of_head(&predicate.head),
        Predicate::Copular(predicate) => finite_inflection_of_auxiliary(predicate.copula.auxiliary),
        Predicate::Proform(predicate) => finite_inflection_of_auxiliary(predicate.auxiliary),
        Predicate::Deontic(_) => None,
        Predicate::Attached(predicate) => finite_inflection_of_predicate(predicate.predicate()),
    }
}

pub(super) fn finite_inflection_of_head(head: &PredicateHead) -> Option<FiniteInflection> {
    if let Some(auxiliary) = head.auxiliaries.first().copied() {
        finite_inflection_of_auxiliary(auxiliary)
    } else {
        match head.verb.slot {
            VerbSlot::Present { person, number } => {
                Some(FiniteInflection::Present(Agreement { person, number }))
            }
            VerbSlot::Past { person, number } => {
                Some(FiniteInflection::Past(Agreement { person, number }))
            }
            _ => None,
        }
    }
}

pub(super) const fn finite_inflection_of_auxiliary(
    auxiliary: AuxiliaryInstance,
) -> Option<FiniteInflection> {
    match auxiliary.inflection {
        AuxiliaryInflection::Present { person, number } => {
            Some(FiniteInflection::Present(Agreement { person, number }))
        }
        AuxiliaryInflection::Past { person, number } => {
            Some(FiniteInflection::Past(Agreement { person, number }))
        }
        _ => None,
    }
}

pub(super) fn apply_finite_inflection(predicate: &mut Predicate, inflection: FiniteInflection) {
    match predicate {
        Predicate::Transitive(predicate) => {
            apply_finite_inflection_to_head(&mut predicate.head, inflection);
        }
        Predicate::Intransitive(predicate) => {
            apply_finite_inflection_to_head(&mut predicate.head, inflection);
        }
        Predicate::Passive(predicate) => {
            apply_finite_inflection_to_head(&mut predicate.head, inflection);
        }
        Predicate::Copular(predicate) => {
            apply_finite_inflection_to_auxiliary(&mut predicate.copula.auxiliary, inflection);
        }
        Predicate::Proform(predicate) => {
            apply_finite_inflection_to_auxiliary(&mut predicate.auxiliary, inflection);
        }
        // The modal itself is uninflected and licenses the inner infinitive.
        Predicate::Deontic(_) => {}
        Predicate::Attached(predicate) => {
            apply_finite_inflection(&mut predicate.scope.host, inflection);
        }
    }
}

pub(super) fn apply_finite_inflection_to_head(
    head: &mut PredicateHead,
    inflection: FiniteInflection,
) {
    if let Some(auxiliary) = head.auxiliaries.first_mut() {
        apply_finite_inflection_to_auxiliary(auxiliary, inflection);
    } else if head.verb.slot == VerbSlot::Infinitive {
        let vocabulary = crate::word::Vocabulary::new();
        let mut finite = head.verb.clone();
        finite.slot = inflection.verb_slot();
        let source_spelling = vocabulary.render_verb_instance(&head.verb);
        if source_spelling.is_some() && source_spelling == vocabulary.render_verb_instance(&finite)
        {
            head.verb = finite;
        }
    }
}

/// Records agreement only when the finite form is syncretic with the parsed
/// base form. The grammar intentionally admits some subjectless continuations
/// whose apparent predicate head is really a noun or an imperative; changing
/// their surface (`target` -> `targets`, `return` -> `returned`) would turn a
/// representation-only normalization into a parse-selection change.
pub(super) fn apply_finite_inflection_to_auxiliary(
    auxiliary: &mut AuxiliaryInstance,
    inflection: FiniteInflection,
) {
    if auxiliary.inflection == AuxiliaryInflection::Base {
        let vocabulary = crate::word::Vocabulary::new();
        let mut finite = *auxiliary;
        finite.inflection = inflection.auxiliary_inflection();
        let source_spelling = vocabulary.render_auxiliary(*auxiliary);
        if source_spelling.is_some() && source_spelling == vocabulary.render_auxiliary(finite) {
            *auxiliary = finite;
        }
    }
}

/// Adds a subjectless predicate to the rightmost finite clause that supplies
/// its subject. This turns mixed `S₁ P₁ and S₂ P₂ and P₃` into coordination of
/// two complete clauses, with `P₂ and P₃` coordinated under `S₂`.
pub(super) fn append_shared_predicate(
    clause: IndependentClause,
    junction: CoordinationJunction,
    predicate: Predicate,
) -> Option<IndependentClause> {
    match clause {
        IndependentClause::Finite(mut finite) => {
            finite.predicate = push_predicate_expression(finite.predicate, junction, predicate);
            Some(IndependentClause::Finite(finite))
        }
        IndependentClause::Complex(complex) => {
            if complex.attachment().position() == crate::syntax::AttachmentPosition::AfterMatrix
                && let Some(scoped) = append_shared_predicate_after_attachment(
                    complex.host().clone(),
                    complex.attachment().clone(),
                    junction.clone(),
                    predicate.clone(),
                )
            {
                return Some(scoped);
            }
            let host = append_shared_predicate(complex.host().clone(), junction, predicate)?;
            Some(IndependentClause::Complex(
                ComplexClause::from_declaration_parts(host, complex.attachment().clone()),
            ))
        }
        IndependentClause::Coordinated(mut coordinated) => {
            let CoordinatedClauseMember::Independent(last) =
                &mut coordinated.rest.last_mut()?.member;
            if !accepts_shared_predicate(last) {
                return None;
            }
            let replacement = append_shared_predicate((**last).clone(), junction, predicate)?;
            **last = replacement;
            Some(IndependentClause::Coordinated(coordinated))
        }
        IndependentClause::Existential(_) => None,
    }
}

/// Moves a trailing clause edge onto the first predicate before adding a
/// shared-subject continuation. Fronted edges stay outside the resulting
/// coordination, so `If C, P unless U and Q` retains the surface scope
/// `(P unless U) and Q` rather than widening `unless U` over both predicates.
fn append_shared_predicate_after_attachment(
    clause: IndependentClause,
    attachment: ClauseAttachment,
    junction: CoordinationJunction,
    predicate: Predicate,
) -> Option<IndependentClause> {
    match clause {
        IndependentClause::Finite(finite) => {
            let attached = attach_to_rightmost_predicate(finite.predicate, attachment)?;
            Some(IndependentClause::Finite(
                FiniteClause::from_declaration_parts(
                    finite.subject,
                    push_predicate_expression(attached, junction, predicate),
                ),
            ))
        }
        IndependentClause::Complex(complex)
            if complex.attachment().position()
                == crate::syntax::AttachmentPosition::BeforeMatrix =>
        {
            let host = append_shared_predicate_after_attachment(
                complex.host().clone(),
                attachment,
                junction,
                predicate,
            )?;
            Some(IndependentClause::Complex(
                ComplexClause::from_declaration_parts(host, complex.attachment().clone()),
            ))
        }
        IndependentClause::Complex(_)
        | IndependentClause::Coordinated(_)
        | IndependentClause::Existential(_) => None,
    }
}

fn attach_to_rightmost_predicate(
    expression: PredicateExpression,
    attachment: ClauseAttachment,
) -> Option<PredicateExpression> {
    match expression {
        PredicateExpression::Simple(predicate) => Some(PredicateExpression::Simple(
            Predicate::Attached(AttachedPredicate {
                scope: AttachmentScope::from_declaration_parts(predicate, attachment),
            }),
        )),
        PredicateExpression::Coordinated(coordination) => {
            let (mut conjuncts, junctions) = coordination.into_declaration_parts();
            let last = conjuncts.pop()?;
            conjuncts.push(attach_to_rightmost_predicate(last, attachment)?);
            Some(PredicateExpression::Coordinated(
                Coordination::from_declaration_parts(conjuncts, junctions)?,
            ))
        }
    }
}

fn postpositive_subordinator(attachment: &ClauseAttachment) -> Option<Subordinator> {
    if attachment.position() != crate::syntax::AttachmentPosition::AfterMatrix {
        return None;
    }
    let crate::syntax::ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
        subordinator,
        SubordinateBody::Finite(_),
    )) = attachment.payload()
    else {
        return None;
    };
    Some(*subordinator)
}

fn predicate_postpositive_subordinator(expression: &PredicateExpression) -> Option<Subordinator> {
    let PredicateExpression::Simple(Predicate::Attached(predicate)) = expression else {
        return None;
    };
    postpositive_subordinator(predicate.attachment())
}

fn establishes_serial_postpositive_condition(
    host: &IndependentClause,
    attachment: &ClauseAttachment,
) -> bool {
    let Some(subordinator) = postpositive_subordinator(attachment) else {
        return false;
    };
    match host {
        IndependentClause::Complex(complex)
            if postpositive_subordinator(complex.attachment()) == Some(subordinator) =>
        {
            true
        }
        IndependentClause::Complex(complex)
            if complex.attachment().position()
                == crate::syntax::AttachmentPosition::BeforeMatrix =>
        {
            establishes_serial_postpositive_condition(complex.host(), attachment)
        }
        IndependentClause::Finite(finite) => match finite.predicate() {
            PredicateExpression::Simple(member) => {
                postpositive_subordinator(match member {
                    Predicate::Attached(attached) => attached.attachment(),
                    _ => return false,
                }) == Some(subordinator)
            }
            PredicateExpression::Coordinated(coordination) => {
                coordination.conjuncts().len() >= 2
                    && coordination.conjuncts().iter().all(|member| {
                        predicate_postpositive_subordinator(member) == Some(subordinator)
                    })
            }
        },
        IndependentClause::Coordinated(coordination) => {
            coordination
                .rest()
                .last()
                .is_some_and(|continuation| match continuation.member() {
                    CoordinatedClauseMember::Independent(clause) => {
                        establishes_serial_postpositive_condition(clause, attachment)
                    }
                })
        }
        IndependentClause::Complex(_) | IndependentClause::Existential(_) => false,
    }
}

fn rightmost_predicate(clause: &IndependentClause) -> Option<&Predicate> {
    match clause {
        IndependentClause::Finite(finite) => match finite.predicate() {
            PredicateExpression::Simple(predicate) => Some(predicate),
            PredicateExpression::Coordinated(coordination) => {
                let PredicateExpression::Simple(predicate) = coordination.conjuncts().last()?
                else {
                    return None;
                };
                Some(predicate)
            }
        },
        IndependentClause::Complex(complex) => rightmost_predicate(complex.host()),
        IndependentClause::Coordinated(coordination) => {
            let CoordinatedClauseMember::Independent(clause) = coordination.rest().last()?.member();
            rightmost_predicate(clause)
        }
        IndependentClause::Existential(_) => None,
    }
}

fn shared_grant_predicate(
    clause: &IndependentClause,
) -> Option<&crate::syntax::TransitivePredicate> {
    let predicate = match rightmost_predicate(clause)? {
        Predicate::Attached(attached) => attached.predicate(),
        predicate => predicate,
    };
    let Predicate::Transitive(predicate) = predicate else {
        return None;
    };
    (matches!(predicate.head().verb().verb, Verb::Word(Vocab::Have))
        && matches!(
            predicate.object(),
            PredicateObject::Ability(AbilityObject { argument: None, .. })
                | PredicateObject::QuotedAbility(_)
        ))
    .then_some(predicate)
}

fn same_shared_grant_shape(
    left: &crate::syntax::TransitivePredicate,
    right: &crate::syntax::TransitivePredicate,
) -> bool {
    left.head == right.head
        && left.kind.pre_object_elements == right.kind.pre_object_elements
        && left.elements == right.elements
}

fn coordinate_shared_grant_complement(
    first: Clause,
    conjunction: Option<Conjunction>,
    comma: Comma,
    complement: SharedGrantComplement,
    attachment: ClauseAttachment,
) -> Option<Clause> {
    let Clause::Independent(first) = first else {
        return None;
    };
    if !establishes_serial_postpositive_condition(&first, &attachment) {
        return None;
    }
    let mut predicate = shared_grant_predicate(&first)?.clone();
    predicate.kind.object = complement.into_object();
    let predicate = Predicate::Attached(AttachedPredicate {
        scope: AttachmentScope::from_declaration_parts(
            Predicate::Transitive(predicate),
            attachment,
        ),
    });
    let coordinated = append_shared_predicate(
        first,
        CoordinationJunction::from_shared_grant_parts(conjunction, comma),
        predicate,
    )?;
    Some(Clause::Independent(coordinated))
}

fn checked_shared_grant_coordination(
    construction: &'static str,
    first: Clause,
    conjunction: Option<Conjunction>,
    comma: Comma,
    complement: SharedGrantComplement,
    attachment: ClauseAttachment,
) -> Result<Clause, DeclarationViolation> {
    coordinate_shared_grant_complement(first, conjunction, comma, complement, attachment)
        .ok_or_else(|| {
            violation(
                construction,
                "the host is a finite has-ability predicate with the same repeated condition pattern",
            )
        })
}

fn make_clause_coordination_shared_grant(
    first: Clause,
    conjunction: Conjunction,
    complement: SharedGrantComplement,
    attachment: ClauseAttachment,
) -> Result<Clause, DeclarationViolation> {
    checked_shared_grant_coordination(
        "clause_coordination_shared_grant",
        first,
        Some(conjunction),
        Comma::Absent,
        complement,
        attachment,
    )
}

fn make_clause_coordination_shared_grant_comma(
    first: SharedGrantPrefix,
    conjunction: Conjunction,
    complement: SharedGrantComplement,
    attachment: ClauseAttachment,
) -> Result<Clause, DeclarationViolation> {
    checked_shared_grant_coordination(
        "clause_coordination_shared_grant_comma",
        first.0,
        Some(conjunction),
        Comma::Present,
        complement,
        attachment,
    )
}

fn make_clause_coordination_shared_grant_asyndetic(
    first: Clause,
    complement: SharedGrantComplement,
    attachment: ClauseAttachment,
) -> Result<Clause, DeclarationViolation> {
    checked_shared_grant_coordination(
        "clause_coordination_shared_grant_asyndetic",
        first,
        None,
        Comma::Present,
        complement,
        attachment,
    )
}

fn make_shared_grant_prefix_start(
    first: SharedGrantBase,
    complement: SharedGrantComplement,
    attachment: ClauseAttachment,
) -> Result<SharedGrantPrefix, DeclarationViolation> {
    checked_shared_grant_coordination(
        "shared_grant_prefix_start",
        first.0,
        None,
        Comma::Present,
        complement,
        attachment,
    )
    .map(SharedGrantPrefix)
}

fn make_shared_grant_prefix_continue(
    first: SharedGrantPrefix,
    complement: SharedGrantComplement,
    attachment: ClauseAttachment,
) -> Result<SharedGrantPrefix, DeclarationViolation> {
    checked_shared_grant_coordination(
        "shared_grant_prefix_continue",
        first.0,
        None,
        Comma::Present,
        complement,
        attachment,
    )
    .map(SharedGrantPrefix)
}

pub(crate) fn attach_serial_postpositive_condition(
    host: IndependentClause,
    attachment: ClauseAttachment,
) -> Option<IndependentClause> {
    let subordinator = postpositive_subordinator(&attachment)?;
    match host {
        IndependentClause::Finite(finite) => {
            let PredicateExpression::Coordinated(coordination) = &finite.predicate else {
                return None;
            };
            let conjuncts = coordination.conjuncts();
            if conjuncts.len() < 3
                || predicate_postpositive_subordinator(conjuncts.last()?).is_some()
                || !conjuncts[..conjuncts.len() - 1]
                    .iter()
                    .all(|member| predicate_postpositive_subordinator(member) == Some(subordinator))
            {
                return None;
            }
            Some(IndependentClause::Finite(
                FiniteClause::from_declaration_parts(
                    finite.subject,
                    attach_to_rightmost_predicate(finite.predicate, attachment)?,
                ),
            ))
        }
        IndependentClause::Complex(complex)
            if complex.attachment().position()
                == crate::syntax::AttachmentPosition::BeforeMatrix =>
        {
            let host = attach_serial_postpositive_condition(complex.host().clone(), attachment)?;
            Some(IndependentClause::Complex(
                ComplexClause::from_declaration_parts(host, complex.attachment().clone()),
            ))
        }
        IndependentClause::Existential(_)
        | IndependentClause::Complex(_)
        | IndependentClause::Coordinated(_) => None,
    }
}

/// Adds one predicate without flattening a changed connective into the
/// existing run. Predicate expressions are recursive specifically so
/// `(attack or block) and has ...` keeps the `or` constituent as the first
/// member of the outer `and` group. Asyndetic Oxford members remain in the
/// open run until its overt closing connective arrives.
fn push_predicate_expression(
    expression: PredicateExpression,
    junction: CoordinationJunction,
    predicate: Predicate,
) -> PredicateExpression {
    match expression {
        PredicateExpression::Simple(Predicate::Deontic(mut deontic))
            if !matches!(predicate, Predicate::Deontic(_)) && deontic.inner.is_some() =>
        {
            deontic.inner = deontic
                .inner
                .map(|inner| Box::new(push_predicate_expression(*inner, junction, predicate)));
            PredicateExpression::Simple(Predicate::Deontic(deontic))
        }
        PredicateExpression::Coordinated(mut coordinated)
            if !connective_changes(
                coordinated
                    .junctions()
                    .iter()
                    .rev()
                    .find_map(|junction| junction.conjunction),
                junction.conjunction,
            ) =>
        {
            coordinated.push(junction, PredicateExpression::Simple(predicate));
            PredicateExpression::Coordinated(coordinated)
        }
        expression => PredicateExpression::Coordinated(Coordination::new(
            expression,
            junction,
            PredicateExpression::Simple(predicate),
        )),
    }
}

/// Keeps a subjectless imperative as a complete clause member when there is no
/// finite subject to share, or when a changed overt connective starts a new
/// clause-level group (`C1 and C2, or P3`). The latter preserves ordinary
/// `and`/`or` grouping instead of incorrectly making `P3` a predicate of C2's
/// subject.
pub(super) fn append_subjectless_clause(
    clause: IndependentClause,
    junction: &CoordinationJunction,
    predicate: Predicate,
) -> Option<IndependentClause> {
    if matches!(predicate, Predicate::Deontic(_)) {
        return None;
    }
    Some(append_complete_clause(
        clause,
        junction.clone(),
        IndependentClause::Finite(FiniteClause::from_declaration_parts(
            None,
            PredicateExpression::Simple(predicate),
        )),
    ))
}

/// Adds a complete clause while preserving punctuation-signaled grouping.
/// A changed connective after a comma starts a new outer group (`A and B, or
/// C`). Without that comma it joins the rightmost clause (`A, or B and C`),
/// which is also where an immediately following subjectless predicate finds
/// its finite subject.
fn append_complete_clause(
    first: IndependentClause,
    junction: CoordinationJunction,
    next: IndependentClause,
) -> IndependentClause {
    let continuation = ClauseCoordination::from_declaration_parts(
        junction.conjunction(),
        junction.comma(),
        CoordinatedClauseMember::Independent(Box::new(next)),
    );
    match first {
        IndependentClause::Coordinated(mut coordinated) => {
            let changed = connective_changes(
                coordinated
                    .rest
                    .iter()
                    .rev()
                    .find_map(|member| member.conjunction),
                junction.conjunction,
            );
            if changed && junction.comma.is_present() {
                return IndependentClause::Coordinated(
                    CoordinatedIndependentClause::from_declaration_parts(
                        Box::new(IndependentClause::Coordinated(coordinated)),
                        vec![continuation],
                    ),
                );
            }
            if changed {
                let CoordinatedClauseMember::Independent(last) = &mut coordinated
                    .rest
                    .last_mut()
                    .expect("a coordinated clause has a continuation")
                    .member;
                let CoordinatedClauseMember::Independent(next) = continuation.member;
                **last = append_complete_clause((**last).clone(), junction, *next);
            } else {
                coordinated.rest.push(continuation);
            }
            IndependentClause::Coordinated(coordinated)
        }
        first => {
            IndependentClause::Coordinated(CoordinatedIndependentClause::from_declaration_parts(
                Box::new(first),
                vec![continuation],
            ))
        }
    }
}

fn connective_changes(previous: Option<Conjunction>, next: Option<Conjunction>) -> bool {
    matches!((previous, next), (Some(previous), Some(next)) if previous != next)
}

pub(super) fn starts_new_clause_group(
    clause: &IndependentClause,
    junction: &CoordinationJunction,
) -> bool {
    let IndependentClause::Coordinated(coordinated) = clause else {
        return false;
    };
    junction.comma.is_present()
        && coordinated.rest.last().is_some_and(|previous| {
            matches!(
                (previous.conjunction, junction.conjunction),
                (Some(previous), Some(next)) if previous != next
            )
        })
}

pub(super) fn accepts_shared_predicate(clause: &IndependentClause) -> bool {
    match clause {
        IndependentClause::Finite(..) => true,
        IndependentClause::Complex(complex) => accepts_shared_predicate(complex.host()),
        IndependentClause::Coordinated(coordination) => {
            coordination
                .rest
                .last()
                .is_some_and(|continuation| match &continuation.member {
                    CoordinatedClauseMember::Independent(clause) => {
                        accepts_shared_predicate(clause)
                    }
                })
        }
        IndependentClause::Existential(_) => false,
    }
}

fn finish_predicate(phrase: VerbPhrase) -> Option<FinishedPredicate> {
    crate::constructions::predicate::project_public_predicate(phrase).ok()
}

fn checked_clause_coordination(
    construction: &'static str,
    first: Clause,
    conjunction: Option<Conjunction>,
    comma: Comma,
    next: SimpleClause,
) -> Result<Clause, DeclarationViolation> {
    coordinate_clause(first, conjunction, comma, next).ok_or_else(|| {
        violation(
            construction,
            "the continuation is a complete clause or a compatible shared-subject predicate",
        )
    })
}

fn make_clause_coordination(
    first: Clause,
    conjunction: Conjunction,
    next: SimpleClause,
) -> Result<Clause, DeclarationViolation> {
    checked_clause_coordination(
        "clause_coordination",
        first,
        Some(conjunction),
        Comma::Absent,
        next,
    )
}

fn make_clause_coordination_comma(
    first: Clause,
    conjunction: Conjunction,
    next: SimpleClause,
) -> Result<Clause, DeclarationViolation> {
    checked_clause_coordination(
        "clause_coordination_comma",
        first,
        Some(conjunction),
        Comma::Present,
        next,
    )
}

fn make_clause_coordination_asyndetic(
    first: Clause,
    next: SimpleClause,
) -> Result<Clause, DeclarationViolation> {
    checked_clause_coordination(
        "clause_coordination_asyndetic",
        first,
        None,
        Comma::Present,
        next,
    )
}

fn peel_last_predicate(
    clause: IndependentClause,
) -> Option<(IndependentClause, CoordinationJunction, Predicate)> {
    match clause {
        IndependentClause::Finite(finite) => {
            let (remaining, junction, last_predicate) =
                peel_last_predicate_expression(finite.predicate)?;
            Some((
                IndependentClause::Finite(FiniteClause::from_declaration_parts(
                    finite.subject,
                    remaining,
                )),
                junction,
                last_predicate,
            ))
        }
        // A fronted or trailing attachment owns the outer clause wrapper.
        // Coordination projects from that owner's host instead of tunneling
        // through the wrapper and manufacturing a rival narrower scope.
        IndependentClause::Coordinated(mut coordinated) => {
            let last = coordinated.rest.last_mut()?;
            let CoordinatedClauseMember::Independent(member) = &mut last.member;
            let (prior, junction, predicate) = peel_last_predicate((**member).clone())?;
            **member = prior;
            Some((
                IndependentClause::Coordinated(coordinated),
                junction,
                predicate,
            ))
        }
        IndependentClause::Complex(_) | IndependentClause::Existential(_) => None,
    }
}

fn peel_last_predicate_expression(
    expression: PredicateExpression,
) -> Option<(PredicateExpression, CoordinationJunction, Predicate)> {
    match expression {
        PredicateExpression::Coordinated(coordination) => {
            let (mut conjuncts, mut junctions) = coordination.into_declaration_parts();
            let PredicateExpression::Simple(last_predicate) = conjuncts.pop()? else {
                return None;
            };
            let junction = junctions.pop()?;
            let remaining = if conjuncts.len() == 1 {
                conjuncts.pop()?
            } else {
                PredicateExpression::Coordinated(Coordination::from_declaration_parts(
                    conjuncts, junctions,
                )?)
            };
            Some((remaining, junction, last_predicate))
        }
        PredicateExpression::Simple(Predicate::Deontic(mut deontic)) => {
            let (remaining, junction, last_predicate) =
                peel_last_predicate_expression(*deontic.inner.take()?)?;
            deontic.inner = Some(Box::new(remaining));
            Some((
                PredicateExpression::Simple(Predicate::Deontic(deontic)),
                junction,
                last_predicate,
            ))
        }
        PredicateExpression::Simple(_) => None,
    }
}

fn peel_last_complete_clause(
    clause: IndependentClause,
) -> Option<(IndependentClause, CoordinationJunction, IndependentClause)> {
    let IndependentClause::Coordinated(mut coordinated) = clause else {
        return None;
    };
    let last = coordinated.rest.last_mut()?;
    let CoordinatedClauseMember::Independent(member) = &mut last.member;
    if matches!(member.as_ref(), IndependentClause::Coordinated(_))
        && let Some((prior, junction, next)) = peel_last_complete_clause((**member).clone())
    {
        **member = prior;
        return Some((IndependentClause::Coordinated(coordinated), junction, next));
    }
    let continuation = coordinated.rest.pop()?;
    let first = if coordinated.rest.is_empty() {
        *coordinated.first
    } else {
        IndependentClause::Coordinated(coordinated)
    };
    let CoordinatedClauseMember::Independent(next) = continuation.member;
    Some((
        first,
        CoordinationJunction::from_declaration_parts(continuation.conjunction, continuation.comma),
        *next,
    ))
}

fn clause_coordination_parts(
    value: &Clause,
) -> Option<(Clause, CoordinationJunction, SimpleClause)> {
    let Clause::Independent(value) = value else {
        return None;
    };
    if let Some((first, junction, predicate)) = peel_last_predicate(value.clone())
        && junction.head_realization() == crate::syntax::CoordinationHeadRealization::Overt
    {
        let (predicate, attachment) = match predicate {
            Predicate::Attached(attached) => (
                crate::constructions::predicate::inverse_public_predicate(attached.predicate())
                    .ok()?,
                Some(attached.attachment().clone()),
            ),
            predicate => (
                crate::constructions::predicate::inverse_public_predicate(&predicate).ok()?,
                None,
            ),
        };
        return Some((
            Clause::Independent(first),
            junction,
            SimpleClause {
                subject: None,
                predicate,
                attachment,
            },
        ));
    }
    let (first, junction, next) = peel_last_complete_clause(value.clone())?;
    let next = inverse_simple_clause(&Clause::Independent(next))?;
    Some((Clause::Independent(first), junction, next))
}

fn clause_coordination_conjoined_parts(value: &Clause) -> (Clause, Conjunction, SimpleClause) {
    let (first, junction, next) =
        clause_coordination_parts(value).expect("the coordination recognizer checks its shape");
    (
        first,
        junction
            .conjunction()
            .expect("the conjoined coordination has a conjunction"),
        next,
    )
}

fn clause_coordination_asyndetic_parts(value: &Clause) -> (Clause, SimpleClause) {
    let (first, _, next) =
        clause_coordination_parts(value).expect("the asyndetic recognizer checks its shape");
    (first, next)
}

fn is_clause_coordination(value: &Clause) -> bool {
    clause_coordination_parts(value).is_some_and(|(_, junction, _)| {
        !junction.comma().is_present() && junction.conjunction().is_some()
    })
}

fn is_clause_coordination_comma(value: &Clause) -> bool {
    clause_coordination_parts(value).is_some_and(|(_, junction, _)| {
        junction.comma().is_present() && junction.conjunction().is_some()
    })
}

fn is_clause_coordination_asyndetic(value: &Clause) -> bool {
    clause_coordination_parts(value).is_some_and(|(_, junction, _)| {
        junction.comma().is_present() && junction.conjunction().is_none()
    })
}

fn shared_grant_coordination_parts(
    value: &Clause,
) -> Option<(
    Clause,
    CoordinationJunction,
    SharedGrantComplement,
    ClauseAttachment,
)> {
    let Clause::Independent(value) = value else {
        return None;
    };
    let (first, junction, predicate) = peel_last_predicate(value.clone())?;
    if junction.head_realization() != crate::syntax::CoordinationHeadRealization::SharedGrantElided
    {
        return None;
    }
    let Predicate::Attached(attached) = predicate else {
        return None;
    };
    let Predicate::Transitive(predicate) = attached.predicate() else {
        return None;
    };
    let template = shared_grant_predicate(&first)?;
    if !same_shared_grant_shape(template, predicate) {
        return None;
    }
    let complement = match predicate.object() {
        PredicateObject::Ability(AbilityObject {
            ability,
            argument: None,
        }) if ability.is_keyword_ability() => SharedGrantComplement::Ability(ability.clone()),
        PredicateObject::QuotedAbility(quoted) => SharedGrantComplement::Quoted((**quoted).clone()),
        _ => return None,
    };
    let first = Clause::Independent(first);
    let first = restore_single_member_attachment(first.clone()).unwrap_or(first);
    Some((first, junction, complement, attached.attachment().clone()))
}

fn shared_grant_coordination_conjoined_parts(
    value: &Clause,
) -> (Clause, Conjunction, SharedGrantComplement, ClauseAttachment) {
    let (first, junction, complement, attachment) = shared_grant_coordination_parts(value)
        .expect("the shared-grant coordination recognizer checks its shape");
    (
        first,
        junction
            .conjunction()
            .expect("the conjoined shared-grant coordination has a conjunction"),
        complement,
        attachment,
    )
}

fn shared_grant_coordination_comma_parts(
    value: &Clause,
) -> (
    SharedGrantPrefix,
    Conjunction,
    SharedGrantComplement,
    ClauseAttachment,
) {
    let (first, junction, complement, attachment) = shared_grant_coordination_parts(value)
        .expect("the shared-grant comma recognizer checks its shape");
    (
        SharedGrantPrefix(first),
        junction
            .conjunction()
            .expect("the shared-grant comma coordination has a conjunction"),
        complement,
        attachment,
    )
}

fn shared_grant_coordination_asyndetic_parts(
    value: &Clause,
) -> (Clause, SharedGrantComplement, ClauseAttachment) {
    let (first, _, complement, attachment) = shared_grant_coordination_parts(value)
        .expect("the asyndetic shared-grant coordination recognizer checks its shape");
    (first, complement, attachment)
}

fn is_clause_coordination_shared_grant(value: &Clause) -> bool {
    shared_grant_coordination_parts(value).is_some_and(|(_, junction, _, _)| {
        !junction.comma().is_present() && junction.conjunction().is_some()
    })
}

fn is_clause_coordination_shared_grant_comma(value: &Clause) -> bool {
    shared_grant_coordination_parts(value).is_some_and(|(_, junction, _, _)| {
        junction.comma().is_present() && junction.conjunction().is_some()
    })
}

fn is_clause_coordination_shared_grant_asyndetic(value: &Clause) -> bool {
    shared_grant_coordination_parts(value).is_some_and(|(_, junction, _, _)| {
        junction.comma().is_present() && junction.conjunction().is_none()
    })
}

fn shared_grant_prefix_len(value: &Clause) -> Option<usize> {
    let Clause::Independent(clause) = value else {
        return None;
    };
    let finite = match clause {
        IndependentClause::Finite(finite) => finite,
        IndependentClause::Complex(complex)
            if complex.attachment().position()
                == crate::syntax::AttachmentPosition::BeforeMatrix =>
        {
            let IndependentClause::Finite(finite) = complex.host() else {
                return None;
            };
            finite
        }
        _ => return None,
    };
    let PredicateExpression::Coordinated(coordination) = finite.predicate() else {
        return None;
    };
    coordination
        .junctions()
        .iter()
        .all(|junction| {
            junction.head_realization()
                == crate::syntax::CoordinationHeadRealization::SharedGrantElided
                && junction.conjunction().is_none()
                && junction.comma().is_present()
        })
        .then_some(coordination.conjuncts().len())
}

fn restore_single_member_attachment(value: Clause) -> Option<Clause> {
    let Clause::Independent(value) = value else {
        return None;
    };
    match value {
        IndependentClause::Finite(finite) => {
            let PredicateExpression::Simple(Predicate::Attached(attached)) = finite.predicate
            else {
                return None;
            };
            let AttachmentScope { host, attachment } = attached.scope;
            let host = IndependentClause::Finite(FiniteClause::from_declaration_parts(
                finite.subject,
                PredicateExpression::Simple(*host),
            ));
            Some(Clause::Independent(IndependentClause::Complex(
                ComplexClause::from_declaration_parts(host, *attachment),
            )))
        }
        IndependentClause::Complex(complex)
            if complex.attachment().position()
                == crate::syntax::AttachmentPosition::BeforeMatrix =>
        {
            let restored =
                restore_single_member_attachment(Clause::Independent(complex.host().clone()))?;
            let Clause::Independent(restored) = restored else { unreachable!() };
            Some(Clause::Independent(IndependentClause::Complex(
                ComplexClause::from_declaration_parts(restored, complex.attachment().clone()),
            )))
        }
        _ => None,
    }
}

fn shared_grant_prefix_start_parts(
    value: &SharedGrantPrefix,
) -> (SharedGrantBase, SharedGrantComplement, ClauseAttachment) {
    let (first, complement, attachment) = shared_grant_coordination_asyndetic_parts(&value.0);
    (SharedGrantBase(first), complement, attachment)
}

fn shared_grant_prefix_continue_parts(
    value: &SharedGrantPrefix,
) -> (SharedGrantPrefix, SharedGrantComplement, ClauseAttachment) {
    let (first, complement, attachment) = shared_grant_coordination_asyndetic_parts(&value.0);
    (SharedGrantPrefix(first), complement, attachment)
}

fn is_shared_grant_prefix_start(value: &SharedGrantPrefix) -> bool {
    shared_grant_prefix_len(&value.0) == Some(2)
        && is_clause_coordination_shared_grant_asyndetic(&value.0)
}

fn is_shared_grant_prefix_continue(value: &SharedGrantPrefix) -> bool {
    shared_grant_prefix_len(&value.0).is_some_and(|len| len >= 3)
        && is_clause_coordination_shared_grant_asyndetic(&value.0)
}

fn make_shared_copular_predicate(
    copula: AuxiliaryInstance,
    complement: NounPhrase,
    preposition: PrepositionalPhrase,
) -> Result<Predicate, DeclarationViolation> {
    let valid_copula = matches!(
        copula,
        AuxiliaryInstance {
            auxiliary: Auxiliary::Be,
            inflection: AuxiliaryInflection::Present { .. } | AuxiliaryInflection::Past { .. },
            ..
        }
    );
    if !valid_copula || preposition.head().preposition != crate::syntax::Preposition::In {
        return Err(violation(
            "shared_copular_predicate",
            "the indicative copula has a nominal complement and additive in-phrase",
        ));
    }
    Ok(Predicate::Copular(CopularPredicate {
        copula: Copula {
            auxiliary: copula,
            contracted_with_subject: Contraction::Full,
        },
        negated: false,
        distributive_each: false,
        precomplement_adverbs: Vec::new(),
        complement: CopularComplement::NounPhrase(complement),
        adjuncts: vec![PredicateAdjunct::Prepositional(preposition)],
    }))
}

fn shared_copular_predicate_parts(
    value: &Predicate,
) -> (AuxiliaryInstance, NounPhrase, PrepositionalPhrase) {
    let Predicate::Copular(value) = value else {
        unreachable!("the shared-copular recognizer checks its predicate kind")
    };
    let CopularComplement::NounPhrase(complement) = value.complement() else {
        unreachable!("the shared-copular recognizer checks its complement")
    };
    let [PredicateAdjunct::Prepositional(preposition)] = value.adjuncts() else {
        unreachable!("the shared-copular recognizer checks its additive adjunct")
    };
    (
        value.copula().auxiliary(),
        complement.clone(),
        preposition.clone(),
    )
}

fn is_shared_copular_predicate(value: &Predicate) -> bool {
    let Predicate::Copular(value) = value else {
        return false;
    };
    let CopularComplement::NounPhrase(_) = value.complement() else {
        return false;
    };
    matches!(
        value.copula().auxiliary(),
        AuxiliaryInstance {
            auxiliary: Auxiliary::Be,
            inflection: AuxiliaryInflection::Present { .. } | AuxiliaryInflection::Past { .. },
            ..
        }
    ) && !value.negated()
        && !value.distributive_each()
        && value.precomplement_adverbs().is_empty()
        && matches!(
            value.adjuncts(),
            [PredicateAdjunct::Prepositional(preposition)]
                if preposition.head().preposition == crate::syntax::Preposition::In
        )
}

fn checked_shared_copular_coordination(
    construction: &'static str,
    first: Clause,
    conjunction: Option<Conjunction>,
    comma: Comma,
    predicate: Predicate,
) -> Result<Clause, DeclarationViolation> {
    if !is_shared_copular_predicate(&predicate) {
        return Err(violation(
            construction,
            "the continuation is the declared additive copular predicate",
        ));
    }
    coordinate_shared_predicate(first, conjunction, comma, predicate).ok_or_else(|| {
        violation(
            construction,
            "the finite host supplies the copular predicate subject and agreement",
        )
    })
}

fn make_clause_coordination_copular_noun_prepositional(
    first: Clause,
    conjunction: Conjunction,
    predicate: Predicate,
) -> Result<Clause, DeclarationViolation> {
    checked_shared_copular_coordination(
        "clause_coordination_copular_noun_prepositional",
        first,
        Some(conjunction),
        Comma::Absent,
        predicate,
    )
}

fn make_clause_coordination_copular_noun_prepositional_comma(
    first: Clause,
    conjunction: Conjunction,
    predicate: Predicate,
) -> Result<Clause, DeclarationViolation> {
    checked_shared_copular_coordination(
        "clause_coordination_copular_noun_prepositional_comma",
        first,
        Some(conjunction),
        Comma::Present,
        predicate,
    )
}

fn make_clause_coordination_copular_noun_prepositional_asyndetic(
    first: Clause,
    predicate: Predicate,
) -> Result<Clause, DeclarationViolation> {
    checked_shared_copular_coordination(
        "clause_coordination_copular_noun_prepositional_asyndetic",
        first,
        None,
        Comma::Present,
        predicate,
    )
}

fn shared_copular_coordination_parts(
    value: &Clause,
) -> Option<(Clause, CoordinationJunction, Predicate)> {
    let Clause::Independent(value) = value else {
        return None;
    };
    let (first, junction, predicate) = peel_last_predicate(value.clone())?;
    is_shared_copular_predicate(&predicate).then_some((
        Clause::Independent(first),
        junction,
        predicate,
    ))
}

fn shared_copular_coordination_conjoined_parts(value: &Clause) -> (Clause, Conjunction, Predicate) {
    let (first, junction, predicate) = shared_copular_coordination_parts(value)
        .expect("the shared-copular coordination recognizer checks its shape");
    (
        first,
        junction
            .conjunction()
            .expect("the conjoined coordination has a conjunction"),
        predicate,
    )
}

fn shared_copular_coordination_asyndetic_parts(value: &Clause) -> (Clause, Predicate) {
    let (first, _, predicate) = shared_copular_coordination_parts(value)
        .expect("the asyndetic shared-copular recognizer checks its shape");
    (first, predicate)
}

fn is_clause_coordination_copular_noun_prepositional(value: &Clause) -> bool {
    shared_copular_coordination_parts(value).is_some_and(|(_, junction, _)| {
        !junction.comma().is_present() && junction.conjunction() == Some(Conjunction::And)
    })
}

fn is_clause_coordination_copular_noun_prepositional_comma(value: &Clause) -> bool {
    shared_copular_coordination_parts(value).is_some_and(|(_, junction, _)| {
        junction.comma().is_present() && junction.conjunction() == Some(Conjunction::And)
    })
}

fn is_clause_coordination_copular_noun_prepositional_asyndetic(value: &Clause) -> bool {
    shared_copular_coordination_parts(value).is_some_and(|(_, junction, _)| {
        junction.comma().is_present() && junction.conjunction().is_none()
    })
}

fn make_clause_elliptical(
    adjective: AdjectivePhrase,
) -> Result<EllipticalClause, DeclarationViolation> {
    Ok(EllipticalClause::Adjective(adjective))
}

fn clause_elliptical_parts(value: &EllipticalClause) -> AdjectivePhrase {
    match value {
        EllipticalClause::Adjective(adjective) => adjective.clone(),
    }
}

const fn is_clause_elliptical(_: &EllipticalClause) -> bool {
    true
}

fn make_clause_existential(
    form: ExistentialForm,
    pivot: NounPhrase,
) -> Result<Clause, DeclarationViolation> {
    if !existential_pivot_agrees(form, &pivot) {
        return Err(violation(
            "clause_existential",
            "the existential form agrees in number with its pivot",
        ));
    }
    Ok(Clause::Independent(IndependentClause::Existential(
        ExistentialClause::from_declaration_parts(form, pivot),
    )))
}

fn existential_pivot_agrees(form: ExistentialForm, pivot: &NounPhrase) -> bool {
    crate::constructions::noun_phrase::relative_subject_feature_candidates(pivot)
        .iter()
        .any(|features| {
            matches!(
                features,
                Features::NounPhrase {
                    agreement: Some(agreement),
                    ..
                } if agreement.number() == form.number()
            )
        })
}

fn clause_existential_parts(value: &Clause) -> (ExistentialForm, NounPhrase) {
    let Clause::Independent(IndependentClause::Existential(value)) = value else {
        unreachable!("clause_existential admits only existential clauses")
    };
    (value.form(), value.pivot().clone())
}

fn is_clause_existential(value: &Clause) -> bool {
    let Clause::Independent(IndependentClause::Existential(value)) = value else {
        return false;
    };
    existential_pivot_agrees(value.form(), value.pivot())
}

fn clean_remainder(complement: CopularComplement) -> CopularRemainder {
    CopularRemainder {
        negated: false,
        distributive_each: false,
        precomplement_adverbs: Vec::new(),
        complement,
        adjuncts: Vec::new(),
    }
}

macro_rules! remainder_adapter {
    ($make:ident, $parts:ident, $recognizer:ident, $variant:ident, $ty:ty) => {
        fn $make(value: $ty) -> Result<CopularRemainder, DeclarationViolation> {
            Ok(clean_remainder(CopularComplement::$variant(value)))
        }

        fn $parts(value: &CopularRemainder) -> $ty {
            let CopularComplement::$variant(value) = &value.complement else {
                unreachable!(concat!(
                    stringify!($recognizer),
                    " admits one complement class"
                ))
            };
            value.clone()
        }

        fn $recognizer(value: &CopularRemainder) -> bool {
            !value.negated
                && !value.distributive_each
                && value.precomplement_adverbs.is_empty()
                && value.adjuncts.is_empty()
                && matches!(value.complement, CopularComplement::$variant(_))
        }
    };
}

remainder_adapter!(
    make_copular_remainder_noun,
    copular_remainder_noun_parts,
    is_copular_remainder_noun,
    NounPhrase,
    NounPhrase
);
remainder_adapter!(
    make_copular_remainder_adjective,
    copular_remainder_adjective_parts,
    is_copular_remainder_adjective,
    Adjective,
    AdjectivePhrase
);
remainder_adapter!(
    make_copular_remainder_coordinated_adjective,
    copular_remainder_coordinated_adjective_parts,
    is_copular_remainder_coordinated_adjective,
    CoordinatedAdjective,
    CoordinatedAdjectivePhrase
);
remainder_adapter!(
    make_copular_remainder_prepositional,
    copular_remainder_prepositional_parts,
    is_copular_remainder_prepositional,
    Prepositional,
    PrepositionalPhrase
);
remainder_adapter!(
    make_copular_remainder_power_toughness,
    copular_remainder_power_toughness_parts,
    is_copular_remainder_power_toughness,
    PowerToughness,
    PowerToughness
);
remainder_adapter!(
    make_copular_remainder_catalog_atom,
    copular_remainder_catalog_atom_parts,
    is_copular_remainder_catalog_atom,
    CatalogAtom,
    CatalogAtom
);

fn make_copular_remainder_prepositional_adjunct(
    mut remainder: CopularRemainder,
    adjunct: PrepositionalPhrase,
) -> Result<CopularRemainder, DeclarationViolation> {
    remainder
        .adjuncts
        .push(PredicateAdjunct::Prepositional(adjunct));
    Ok(remainder)
}

fn copular_remainder_prepositional_adjunct_parts(
    value: &CopularRemainder,
) -> (CopularRemainder, PrepositionalPhrase) {
    let mut remainder = value.clone();
    let Some(PredicateAdjunct::Prepositional(adjunct)) = remainder.adjuncts.pop() else {
        unreachable!("copular adjunct inverse admits a trailing preposition")
    };
    (remainder, adjunct)
}

fn is_copular_remainder_prepositional_adjunct(value: &CopularRemainder) -> bool {
    matches!(
        value.adjuncts.last(),
        Some(PredicateAdjunct::Prepositional(_))
    )
}

fn make_copular_remainder_adverb(
    adverb: Vocab,
    mut remainder: CopularRemainder,
) -> Result<CopularRemainder, DeclarationViolation> {
    remainder.precomplement_adverbs.insert(0, adverb);
    Ok(remainder)
}

fn copular_remainder_adverb_parts(value: &CopularRemainder) -> (Vocab, CopularRemainder) {
    let mut remainder = value.clone();
    let adverb = remainder.precomplement_adverbs.remove(0);
    (adverb, remainder)
}

fn is_copular_remainder_adverb(value: &CopularRemainder) -> bool {
    value.adjuncts.is_empty() && !value.negated && !value.precomplement_adverbs.is_empty()
}

fn make_copular_remainder_negated(
    mut remainder: CopularRemainder,
) -> Result<CopularRemainder, DeclarationViolation> {
    if remainder.negated {
        return Err(violation(
            "copular_remainder_negated",
            "copular free-standing negation occurs exactly once",
        ));
    }
    remainder.negated = true;
    Ok(remainder)
}

fn copular_remainder_negated_parts(value: &CopularRemainder) -> CopularRemainder {
    let mut remainder = value.clone();
    remainder.negated = false;
    remainder
}

fn is_copular_remainder_negated(value: &CopularRemainder) -> bool {
    value.adjuncts.is_empty() && value.negated
}

fn make_copular_remainder_distributive_each(
    adjective: AdjectivePhrase,
    standard: PrepositionalPhrase,
) -> Result<CopularRemainder, DeclarationViolation> {
    let adjective = adjective
        .try_attach_declared_prepositional(standard)
        .ok_or_else(|| {
            violation(
                "copular_remainder_distributive_each",
                "the distributive adjective accepts its selected prepositional standard",
            )
        })?;
    Ok(CopularRemainder {
        distributive_each: true,
        ..clean_remainder(CopularComplement::Adjective(adjective))
    })
}

fn copular_remainder_distributive_each_parts(
    value: &CopularRemainder,
) -> (AdjectivePhrase, PrepositionalPhrase) {
    let CopularComplement::Adjective(adjective) = &value.complement else {
        unreachable!("distributive copular remainder has an adjective complement")
    };
    adjective
        .clone()
        .try_split_declared_prepositional()
        .expect("distributive copular adjective retains its standard")
}

fn is_copular_remainder_distributive_each(value: &CopularRemainder) -> bool {
    value.distributive_each
        && !value.negated
        && value.precomplement_adverbs.is_empty()
        && value.adjuncts.is_empty()
        && matches!(
            &value.complement,
            CopularComplement::Adjective(adjective)
                if adjective.clone().try_split_declared_prepositional().is_some()
        )
}

fn make_clause_copular(
    subject: NounPhrase,
    auxiliary: AuxiliaryInstance,
    remainder: CopularRemainder,
) -> Result<Clause, DeclarationViolation> {
    if auxiliary.auxiliary != Auxiliary::Be {
        return Err(violation(
            "clause_copular",
            "the finite copula is a form of be",
        ));
    }
    Ok(Clause::Independent(IndependentClause::Finite(
        FiniteClause::from_declaration_parts(
            Some(Subject(subject)),
            PredicateExpression::Simple(Predicate::Copular(CopularPredicate {
                copula: Copula {
                    auxiliary,
                    contracted_with_subject: Contraction::Full,
                },
                negated: remainder.negated,
                distributive_each: remainder.distributive_each,
                precomplement_adverbs: remainder.precomplement_adverbs,
                complement: remainder.complement,
                adjuncts: remainder.adjuncts,
            })),
        ),
    )))
}

fn clause_copular_parts(value: &Clause) -> (NounPhrase, AuxiliaryInstance, CopularRemainder) {
    let Clause::Independent(IndependentClause::Finite(finite)) = value else {
        unreachable!("clause_copular admits only copular clauses")
    };
    let Some(subject) = finite.subject() else {
        unreachable!("clause_copular retains its subject")
    };
    let PredicateExpression::Simple(Predicate::Copular(predicate)) = finite.predicate() else {
        unreachable!("clause_copular retains its copular predicate")
    };
    (
        subject.0.clone(),
        predicate.copula.auxiliary,
        CopularRemainder {
            negated: predicate.negated,
            distributive_each: predicate.distributive_each,
            precomplement_adverbs: predicate.precomplement_adverbs.clone(),
            complement: predicate.complement.clone(),
            adjuncts: predicate.adjuncts.clone(),
        },
    )
}

fn is_clause_copular(value: &Clause) -> bool {
    matches!(
        value,
        Clause::Independent(IndependentClause::Finite(FiniteClause {
            subject: Some(_),
            predicate: PredicateExpression::Simple(Predicate::Copular(CopularPredicate {
                copula: Copula {
                    contracted_with_subject: Contraction::Full,
                    ..
                },
                ..
            })),
        }))
    )
}

fn make_clause_contracted_copular(
    subject_auxiliary: ContractedSubjectAuxiliary,
    remainder: CopularRemainder,
) -> Result<Clause, DeclarationViolation> {
    if subject_auxiliary.auxiliary.auxiliary != Auxiliary::Be {
        return Err(violation(
            "clause_contracted_copular",
            "the contracted subject auxiliary is a form of be",
        ));
    }
    Ok(Clause::Independent(IndependentClause::Finite(
        FiniteClause::from_declaration_parts(
            Some(subject_auxiliary.subject),
            PredicateExpression::Simple(Predicate::Copular(CopularPredicate {
                copula: Copula {
                    auxiliary: subject_auxiliary.auxiliary,
                    contracted_with_subject: Contraction::Contracted,
                },
                negated: remainder.negated,
                distributive_each: remainder.distributive_each,
                precomplement_adverbs: remainder.precomplement_adverbs,
                complement: remainder.complement,
                adjuncts: remainder.adjuncts,
            })),
        ),
    )))
}

fn clause_contracted_copular_parts(
    value: &Clause,
) -> (ContractedSubjectAuxiliary, CopularRemainder) {
    let Clause::Independent(IndependentClause::Finite(finite)) = value else {
        unreachable!("clause_contracted_copular admits only copular clauses")
    };
    let Some(subject) = finite.subject() else {
        unreachable!("clause_contracted_copular retains its subject")
    };
    let PredicateExpression::Simple(Predicate::Copular(predicate)) = finite.predicate() else {
        unreachable!("clause_contracted_copular retains its copular predicate")
    };
    (
        ContractedSubjectAuxiliary {
            subject: subject.clone(),
            auxiliary: predicate.copula.auxiliary,
        },
        CopularRemainder {
            negated: predicate.negated,
            distributive_each: predicate.distributive_each,
            precomplement_adverbs: predicate.precomplement_adverbs.clone(),
            complement: predicate.complement.clone(),
            adjuncts: predicate.adjuncts.clone(),
        },
    )
}

fn is_clause_contracted_copular(value: &Clause) -> bool {
    matches!(
        value,
        Clause::Independent(IndependentClause::Finite(FiniteClause {
            subject: Some(_),
            predicate: PredicateExpression::Simple(Predicate::Copular(CopularPredicate {
                copula: Copula {
                    contracted_with_subject: Contraction::Contracted,
                    ..
                },
                ..
            })),
        }))
    )
}

fn make_clause_variable_value_constraint(
    subject: Quantity,
    modal: AuxiliaryInstance,
    copula: AuxiliaryInstance,
    number: NumberLiteral,
) -> Result<Clause, DeclarationViolation> {
    if subject != Quantity::unchecked_x() {
        return Err(violation(
            "clause_variable_value_constraint",
            "the constrained subject is the variable X",
        ));
    }
    if !crate::constructions::predicate::is_modal(modal.auxiliary) {
        return Err(violation(
            "clause_variable_value_constraint",
            "the first auxiliary is modal",
        ));
    }
    if copula.auxiliary != Auxiliary::Be || copula.inflection != AuxiliaryInflection::Base {
        return Err(violation(
            "clause_variable_value_constraint",
            "the complement copula is base-form be",
        ));
    }
    Ok(Clause::Independent(IndependentClause::Finite(
        FiniteClause::from_declaration_parts(
            Some(Subject(NounPhrase::from_quantity_declaration(subject))),
            PredicateExpression::Simple(Predicate::Deontic(crate::syntax::DeonticPredicate {
                modal: Modal { auxiliary: modal },
                inner: Some(Box::new(PredicateExpression::Simple(Predicate::Copular(
                    CopularPredicate {
                        copula: Copula {
                            auxiliary: copula,
                            contracted_with_subject: Contraction::Full,
                        },
                        negated: false,
                        distributive_each: false,
                        precomplement_adverbs: Vec::new(),
                        complement: CopularComplement::NounPhrase(
                            NounPhrase::from_quantity_declaration(Quantity::unchecked_exact(
                                number,
                            )),
                        ),
                        adjuncts: Vec::new(),
                    },
                )))),
            })),
        ),
    )))
}

fn variable_value_constraint_parts(
    value: &Clause,
) -> (
    Quantity,
    AuxiliaryInstance,
    AuxiliaryInstance,
    NumberLiteral,
) {
    let Clause::Independent(IndependentClause::Finite(finite)) = value else {
        unreachable!("variable value constraint admits one modal copular shape")
    };
    let Some(Subject(subject_phrase)) = finite.subject() else {
        unreachable!("variable value constraint retains its subject")
    };
    let PredicateExpression::Simple(Predicate::Deontic(deontic)) = finite.predicate() else {
        unreachable!("variable value constraint retains its modal")
    };
    let Modal { auxiliary: modal } = deontic.modal();
    let Some(PredicateExpression::Simple(Predicate::Copular(predicate))) = deontic.inner() else {
        unreachable!("variable value constraint retains its copular predicate")
    };
    let crate::syntax::NounPhraseKind::Quantity(subject) = subject_phrase.kind() else {
        unreachable!("variable value constraint retains one numeric subject")
    };
    let CopularComplement::NounPhrase(quantity_phrase) = &predicate.complement else {
        unreachable!("variable value constraint retains one numeric complement")
    };
    let crate::syntax::NounPhraseKind::Quantity(quantity) = quantity_phrase.kind() else {
        unreachable!("variable value constraint retains one numeric complement")
    };
    let QuantityKind::Exact(number) = quantity.kind() else {
        unreachable!("variable value constraint retains one numeric complement")
    };
    (*subject, *modal, predicate.copula.auxiliary, number)
}

fn is_clause_variable_value_constraint(value: &Clause) -> bool {
    let Clause::Independent(IndependentClause::Finite(FiniteClause {
        subject: Some(Subject(subject_phrase)),
        predicate:
            PredicateExpression::Simple(Predicate::Deontic(crate::syntax::DeonticPredicate {
                modal: Modal { auxiliary: modal },
                inner: Some(inner),
            })),
    })) = value
    else {
        return false;
    };
    let PredicateExpression::Simple(Predicate::Copular(CopularPredicate {
        copula:
            Copula {
                auxiliary: copula,
                contracted_with_subject: Contraction::Full,
            },
        negated: false,
        distributive_each: false,
        precomplement_adverbs,
        complement: CopularComplement::NounPhrase(complement_phrase),
        adjuncts,
    })) = inner.as_ref()
    else {
        return false;
    };
    let crate::syntax::NounPhraseKind::Quantity(subject) = subject_phrase.kind() else {
        return false;
    };
    let crate::syntax::NounPhraseKind::Quantity(complement) = complement_phrase.kind() else {
        return false;
    };
    subject.kind() == QuantityKind::X
        && matches!(complement.kind(), QuantityKind::Exact(_))
        && crate::constructions::predicate::is_modal(modal.auxiliary)
        && copula.auxiliary == Auxiliary::Be
        && copula.inflection == AuxiliaryInflection::Base
        && precomplement_adverbs.is_empty()
        && adjuncts.is_empty()
}

#[allow(
    clippy::fn_params_excessive_bools,
    reason = "the fields are the exact SimpleClause chart-feature projection"
)]
const fn simple_clause_features(
    agreement: Option<crate::grammar::Agreement>,
    has_subject: bool,
    standalone: bool,
    has_direct_object: bool,
    host_addressee_subject: bool,
    host_modal: bool,
    subjunctive: bool,
) -> Features {
    Features::SimpleClause {
        agreement,
        has_subject,
        standalone,
        has_direct_object,
        host_addressee_subject,
        host_modal,
        subjunctive,
    }
}

fn reduce_simple_clause_subject_features(
    subject: &Features,
    predicate: &Features,
) -> Option<Features> {
    let Features::NounPhrase {
        agreement: Some(subject_agreement),
        pronoun_case,
        ..
    } = subject
    else {
        return None;
    };
    if *pronoun_case == Some(PronounCase::Object) {
        return None;
    }
    let Features::VerbPhrase {
        form: PredicateForm::Finite(predicate_agreement),
        passive,
        object,
        indirect_object,
        selected_preposition,
        frame,
        subjunctive,
        ..
    } = predicate
    else {
        return None;
    };
    if predicate_agreement.is_some_and(|agreement| agreement != *subject_agreement)
        || !predicate_arguments_complete(
            *frame,
            *passive,
            *object,
            *indirect_object,
            *selected_preposition,
        )
    {
        return None;
    }
    Some(simple_clause_features(
        Some(*subject_agreement),
        true,
        true,
        object.has_direct_object(),
        *pronoun_case == Some(PronounCase::Subject) && subject_agreement.person() == Person::Second,
        predicate_agreement.is_none(),
        *subjunctive,
    ))
}

fn reduce_simple_clause_subject_distributive_each_features(
    subject: &Features,
    predicate: &Features,
) -> Option<Features> {
    let Features::NounPhrase {
        agreement: Some(subject_agreement),
        pronoun_case,
        ..
    } = subject
    else {
        return None;
    };
    if *pronoun_case == Some(PronounCase::Object)
        || (subject_agreement.number() != Number::Plural
            && subject_agreement.person() != Person::Second)
    {
        return None;
    }
    let Features::VerbPhrase {
        form: PredicateForm::Finite(Some(predicate_agreement)),
        passive,
        object,
        indirect_object,
        selected_preposition,
        frame,
        subjunctive,
        ..
    } = predicate
    else {
        return None;
    };
    if *predicate_agreement != *subject_agreement
        || !predicate_arguments_complete(
            *frame,
            *passive,
            *object,
            *indirect_object,
            *selected_preposition,
        )
    {
        return None;
    }
    Some(simple_clause_features(
        Some(*subject_agreement),
        true,
        true,
        object.has_direct_object(),
        *pronoun_case == Some(PronounCase::Subject) && subject_agreement.person() == Person::Second,
        false,
        *subjunctive,
    ))
}

fn reduce_simple_clause_contracted_subject_features(
    subject_auxiliary: &Features,
    predicate: &Features,
) -> Option<Features> {
    let Features::SubjectAuxiliary {
        agreement: subject_agreement,
        auxiliary,
        ..
    } = subject_auxiliary
    else {
        return None;
    };
    let Features::VerbPhrase {
        form: child_form,
        passive: child_passive,
        object,
        indirect_object,
        selected_preposition,
        frame,
        subjunctive,
        ..
    } = predicate
    else {
        return None;
    };
    let PredicateForm::Finite(Some(predicate_agreement)) = auxiliary_form(*auxiliary, *child_form)?
    else {
        return None;
    };
    if predicate_agreement != *subject_agreement {
        return None;
    }
    let passive = fold_auxiliary_passive(
        *auxiliary,
        *child_form,
        *child_passive,
        *object,
        *indirect_object,
        *frame,
    )?;
    if !predicate_arguments_complete(
        *frame,
        passive,
        *object,
        *indirect_object,
        *selected_preposition,
    ) {
        return None;
    }
    Some(simple_clause_features(
        Some(*subject_agreement),
        true,
        true,
        object.has_direct_object(),
        subject_agreement.person() == Person::Second,
        false,
        *subjunctive,
    ))
}

fn reduce_simple_clause_subjectless_features(predicate: &Features) -> Option<Features> {
    let Features::VerbPhrase {
        form,
        passive,
        object,
        indirect_object,
        selected_preposition,
        frame,
        subjunctive,
        ..
    } = predicate
    else {
        return None;
    };
    if !predicate_arguments_complete(
        *frame,
        *passive,
        *object,
        *indirect_object,
        *selected_preposition,
    ) {
        return None;
    }
    let (agreement, standalone, host_modal) = match form {
        PredicateForm::Imperative => (None, true, false),
        PredicateForm::Finite(agreement) => (*agreement, false, agreement.is_none()),
        PredicateForm::Infinitive => (None, false, false),
        PredicateForm::PresentParticiple | PredicateForm::PastParticiple => return None,
    };
    Some(simple_clause_features(
        agreement,
        false,
        standalone,
        object.has_direct_object(),
        false,
        host_modal,
        *subjunctive,
    ))
}

fn reduce_simple_clause_subjectless_attached_features(
    predicate: &Features,
    attachment: &Features,
) -> Option<Features> {
    matches!(attachment, Features::None)
        .then(|| reduce_simple_clause_subjectless_features(predicate))?
}

fn reduce_clause_simple_features(simple: &Features) -> Option<Features> {
    let Features::SimpleClause {
        agreement,
        has_subject,
        standalone,
        host_addressee_subject,
        host_modal,
        subjunctive,
        ..
    } = simple
    else {
        return None;
    };
    Some(Features::Clause {
        agreement: *agreement,
        standalone: *standalone,
        finite: *has_subject,
        host_addressee_subject: *host_addressee_subject,
        host_modal: *host_modal,
        subjunctive: *subjunctive,
    })
}

fn reduce_coordinated_predicate_attachment_features(
    subordinator: &Features,
    condition: &Features,
) -> Option<Features> {
    let Features::Subordinator(_) = subordinator else {
        return None;
    };
    matches!(
        condition,
        Features::Clause {
            standalone: true,
            finite: true,
            subjunctive: false,
            ..
        }
    )
    .then_some(Features::None)
}

fn reduce_coordinated_predicate_attachment_elliptical_features(
    subordinator: &Features,
    adjective: &Features,
) -> Option<Features> {
    matches!(subordinator, Features::Subordinator(Subordinator::If))
        .then_some(())
        .and_then(|()| matches!(adjective, Features::Adjective { .. }).then_some(Features::None))
}

fn reduce_shared_grant_complement_features(value: &Features) -> Option<Features> {
    matches!(value, Features::None).then_some(Features::None)
}

fn reduce_shared_grant_base_features(
    subject: &Features,
    head: &Features,
    complement: &Features,
    attachment: &Features,
) -> Option<Features> {
    let Features::NounPhrase {
        agreement: Some(subject_agreement),
        ..
    } = subject
    else {
        return None;
    };
    let Features::Verb { slot, .. } = head else {
        return None;
    };
    let predicate_agreement = match slot {
        VerbSlot::Present { person, number } | VerbSlot::Past { person, number } => Agreement {
            person: *person,
            number: *number,
        },
        _ => return None,
    };
    if predicate_agreement != *subject_agreement
        || !matches!(complement, Features::None)
        || !matches!(attachment, Features::None)
    {
        return None;
    }
    Some(Features::Clause {
        agreement: Some(*subject_agreement),
        standalone: true,
        finite: true,
        host_addressee_subject: subject_agreement.person() == Person::Second,
        host_modal: false,
        subjunctive: false,
    })
}

fn reduce_shared_grant_coordination_features(
    first: &Features,
    conjunction: Option<&Features>,
    complement: &Features,
    attachment: &Features,
) -> Option<Features> {
    if let Some(Features::Conjunction(Conjunction::And)) = conjunction {
    } else if conjunction.is_some() {
        return None;
    }
    if !matches!(complement, Features::None) || !matches!(attachment, Features::None) {
        return None;
    }
    let Features::Clause {
        agreement: Some(agreement),
        standalone: true,
        finite: true,
        host_addressee_subject,
        host_modal,
        subjunctive: false,
    } = first
    else {
        return None;
    };
    Some(Features::Clause {
        agreement: Some(*agreement),
        standalone: true,
        finite: true,
        host_addressee_subject: *host_addressee_subject,
        host_modal: *host_modal,
        subjunctive: false,
    })
}

fn reduce_shared_grant_coordination_conjoined_features(
    first: &Features,
    conjunction: &Features,
    complement: &Features,
    attachment: &Features,
) -> Option<Features> {
    reduce_shared_grant_coordination_features(first, Some(conjunction), complement, attachment)
}

fn reduce_shared_grant_coordination_asyndetic_features(
    first: &Features,
    complement: &Features,
    attachment: &Features,
) -> Option<Features> {
    reduce_shared_grant_coordination_features(first, None, complement, attachment)
}

fn coordination_agrees(first: &Features, next: &Features) -> bool {
    let Features::Clause {
        agreement: first_agreement,
        finite: first_finite,
        host_addressee_subject,
        host_modal,
        ..
    } = first
    else {
        return false;
    };
    let Features::SimpleClause {
        agreement: next_agreement,
        has_subject,
        standalone,
        host_modal: next_modal,
        ..
    } = next
    else {
        return false;
    };
    let host_adopts_imperative = *host_addressee_subject || *host_modal;
    let shared_finite = matches!(
        (first_agreement, next_agreement, standalone),
        (Some(left), Some(right), false) if left == right
    );
    let imperative_sequence = !*first_finite
        && matches!(
            (first_agreement, next_agreement, standalone),
            (None, None, true)
        );
    let subjectless_modal = *next_modal
        && matches!(
            (first_agreement, next_agreement, standalone),
            (Some(_), None, false)
        );
    let hosted_imperative = host_adopts_imperative && next_agreement.is_none() && *standalone;
    *has_subject || shared_finite || imperative_sequence || subjectless_modal || hosted_imperative
}

fn reduce_clause_coordination_features(
    first: &Features,
    conjunction: Option<&Features>,
    next: &Features,
    asyndetic: bool,
) -> Option<Features> {
    if let Some(Features::Conjunction(
        Conjunction::And | Conjunction::Or | Conjunction::Then | Conjunction::AndOr,
    )) = conjunction
    {
    } else if conjunction.is_some() {
        return None;
    }
    let Features::Clause {
        agreement,
        standalone: true,
        finite,
        host_addressee_subject,
        host_modal,
        subjunctive: false,
    } = first
    else {
        return None;
    };
    let Features::SimpleClause {
        has_subject,
        subjunctive: false,
        ..
    } = next
    else {
        return None;
    };
    if asyndetic && *has_subject {
        return None;
    }
    coordination_agrees(first, next).then_some(Features::Clause {
        agreement: *agreement,
        standalone: true,
        finite: *finite,
        host_addressee_subject: *host_addressee_subject,
        host_modal: *host_modal,
        subjunctive: false,
    })
}

fn reduce_clause_coordination_conjoined_features(
    first: &Features,
    conjunction: &Features,
    next: &Features,
) -> Option<Features> {
    reduce_clause_coordination_features(first, Some(conjunction), next, false)
}

fn reduce_clause_coordination_asyndetic_features(
    first: &Features,
    next: &Features,
) -> Option<Features> {
    reduce_clause_coordination_features(first, None, next, true)
}

fn reduce_shared_copular_predicate_features(
    copula: &Features,
    complement: &Features,
    preposition: &Features,
) -> Option<Features> {
    let Features::Copula(CopulaAgreement::Indicative(agreement)) = copula else {
        return None;
    };
    if !matches!(complement, Features::NounPhrase { .. })
        || !matches!(
            preposition,
            Features::PrepositionalPhrase {
                preposition: crate::syntax::Preposition::In,
                ..
            }
        )
    {
        return None;
    }
    Some(simple_clause_features(
        Some(*agreement),
        false,
        false,
        false,
        false,
        false,
        false,
    ))
}

fn reduce_shared_copular_coordination_features(
    first: &Features,
    conjunction: Option<&Features>,
    predicate: &Features,
) -> Option<Features> {
    if let Some(features) = conjunction
        && !matches!(features, Features::Conjunction(Conjunction::And))
    {
        return None;
    }
    let Features::Clause {
        agreement: Some(host_agreement),
        standalone: true,
        finite: true,
        host_addressee_subject,
        host_modal,
        subjunctive: false,
    } = first
    else {
        return None;
    };
    let Features::SimpleClause {
        agreement: Some(predicate_agreement),
        has_subject: false,
        standalone: false,
        subjunctive: false,
        ..
    } = predicate
    else {
        return None;
    };
    (host_agreement == predicate_agreement).then_some(Features::Clause {
        agreement: Some(*host_agreement),
        standalone: true,
        finite: true,
        host_addressee_subject: *host_addressee_subject,
        host_modal: *host_modal,
        subjunctive: false,
    })
}

fn reduce_shared_copular_coordination_conjoined_features(
    first: &Features,
    conjunction: &Features,
    predicate: &Features,
) -> Option<Features> {
    reduce_shared_copular_coordination_features(first, Some(conjunction), predicate)
}

fn reduce_shared_copular_coordination_asyndetic_features(
    first: &Features,
    predicate: &Features,
) -> Option<Features> {
    reduce_shared_copular_coordination_features(first, None, predicate)
}

const fn reduce_clause_elliptical_features(_: &Features) -> Option<Features> {
    Some(Features::Clause {
        agreement: None,
        standalone: false,
        finite: false,
        host_addressee_subject: false,
        host_modal: false,
        subjunctive: false,
    })
}

fn reduce_clause_existential_features(
    existential: &Features,
    pivot: &Features,
) -> Option<Features> {
    let Features::Existential {
        number: expected_number,
    } = existential
    else {
        return None;
    };
    let Features::NounPhrase {
        agreement: Some(agreement),
        ..
    } = pivot
    else {
        return None;
    };
    (agreement.number() == *expected_number).then_some(Features::Clause {
        agreement: None,
        standalone: true,
        finite: true,
        host_addressee_subject: false,
        host_modal: false,
        subjunctive: false,
    })
}

const fn reduce_copular_remainder_features(_: &Features) -> Option<Features> {
    Some(Features::None)
}

const fn reduce_copular_remainder_binary_features(_: &Features, _: &Features) -> Option<Features> {
    Some(Features::None)
}

fn reduce_clause_copular_features(
    subject: &Features,
    copula: &Features,
    _: &Features,
) -> Option<Features> {
    let Features::NounPhrase {
        agreement: Some(subject_agreement),
        pronoun_case,
        ..
    } = subject
    else {
        return None;
    };
    if *pronoun_case == Some(PronounCase::Object) {
        return None;
    }
    let Features::Copula(copula) = copula else {
        return None;
    };
    let (agreement, subjunctive) = match copula {
        CopulaAgreement::Indicative(copula_agreement) => {
            if *subject_agreement != *copula_agreement {
                return None;
            }
            (*subject_agreement, false)
        }
        CopulaAgreement::PastSubjunctive => (*subject_agreement, true),
    };
    Some(Features::Clause {
        agreement: Some(agreement),
        standalone: true,
        finite: true,
        host_addressee_subject: false,
        host_modal: false,
        subjunctive,
    })
}

fn reduce_clause_contracted_copular_features(
    subject_auxiliary: &Features,
    _: &Features,
) -> Option<Features> {
    let Features::SubjectAuxiliary {
        agreement,
        auxiliary,
        ..
    } = subject_auxiliary
    else {
        return None;
    };
    (auxiliary.auxiliary() == Auxiliary::Be).then_some(Features::Clause {
        agreement: Some(*agreement),
        standalone: true,
        finite: true,
        host_addressee_subject: false,
        host_modal: false,
        subjunctive: false,
    })
}

fn reduce_clause_variable_value_constraint_features(
    quantity: &Features,
    modal: &Features,
    copula: &Features,
    number: &Features,
) -> Option<Features> {
    let Features::Quantity(_) = quantity else {
        return None;
    };
    let Features::Auxiliary(modal) = modal else {
        return None;
    };
    let Features::Auxiliary(copula) = copula else {
        return None;
    };
    let Features::Number { .. } = number else {
        return None;
    };
    (crate::constructions::predicate::is_modal(modal.auxiliary())
        && copula.auxiliary() == Auxiliary::Be
        && copula.inflection() == AuxiliaryInflection::Base)
        .then_some(Features::Clause {
            agreement: None,
            standalone: true,
            finite: true,
            host_addressee_subject: false,
            host_modal: false,
            subjunctive: false,
        })
}

deckmaste_constructions_macro::constructions! {
    group clause;

    construction simple_clause_subject: SimpleClause {
        bind SimpleClause via make_simple_clause_subject, simple_clause_subject_parts {
            subject: hole NounPhrase,
            predicate: hole VerbPhrase,
        }
        derive features: Features = reduce_simple_clause_subject_features(subject, predicate);
        evidence feature "finite subject-predicate agreement" from category;
        form only @ 0 inverse check(is_simple_clause_subject) = subject predicate;
        selection unique;
    }

    construction simple_clause_subject_distributive_each: SimpleClause {
        bind SimpleClause via make_simple_clause_subject_distributive_each, simple_clause_subject_distributive_each_parts {
            subject: hole NounPhrase,
            predicate: hole VerbPhrase,
        }
        derive features: Features = reduce_simple_clause_subject_distributive_each_features(subject, predicate);
        evidence feature "plural distributive subject agreement" from category;
        form only @ 0 inverse check(is_simple_clause_subject_distributive_each) = subject "each" predicate;
        selection unique;
    }

    construction simple_clause_contracted_subject: SimpleClause {
        bind SimpleClause via make_simple_clause_contracted_subject, simple_clause_contracted_subject_parts {
            subject_auxiliary: identity ContractedSubjectAuxiliary via SubjectAuxiliary,
            predicate: hole VerbPhrase,
        }
        derive features: Features = reduce_simple_clause_contracted_subject_features(subject_auxiliary, predicate);
        evidence feature "contracted subject-auxiliary agreement" from category;
        form only @ 0 inverse check(is_simple_clause_contracted_subject) = identity(subject_auxiliary) predicate;
        selection unique;
    }

    construction simple_clause_subjectless: SimpleClause {
        bind SimpleClause via make_simple_clause_subjectless, simple_clause_subjectless_parts {
            predicate: hole VerbPhrase,
        }
        derive features: Features = reduce_simple_clause_subjectless_features(predicate);
        evidence feature "subjectless predicate form" from category;
        form only @ 0 inverse check(is_simple_clause_subjectless) = predicate;
        selection unique;
    }

    internal construction simple_clause_subjectless_attached: SimpleClause {
        bind SimpleClause via make_simple_clause_subjectless_attached, simple_clause_subjectless_attached_parts {
            predicate: hole VerbPhrase,
            attachment: hole CoordinatedPredicateAttachment,
        }
        derive features: Features = reduce_simple_clause_subjectless_attached_features(predicate, attachment);
        form only @ 0 inverse check(is_simple_clause_subjectless_attached) = predicate attachment;
        selection unique;
    }

    construction clause_simple: Clause {
        bind Clause via make_clause_simple, clause_simple_parts {
            simple: hole SimpleClause,
        }
        derive features: Features = reduce_clause_simple_features(simple);
        form only @ 0 inverse check(is_clause_simple) = simple;
        dominates clause_copular;
        selection unique;
    }

    internal construction coordinated_predicate_attachment: CoordinatedPredicateAttachment {
        bind ClauseAttachment via make_coordinated_predicate_attachment, coordinated_predicate_attachment_parts {
            subordinator: identity Subordinator via Subordinator,
            condition: hole Clause,
        }
        derive features: Features = reduce_coordinated_predicate_attachment_features(subordinator, condition);
        form only @ 0 inverse check(is_coordinated_predicate_attachment) = identity(subordinator) condition;
        selection unique;
    }

    internal construction coordinated_predicate_attachment_comma: CoordinatedPredicateAttachment {
        bind ClauseAttachment via make_coordinated_predicate_attachment_comma, coordinated_predicate_attachment_parts {
            subordinator: identity Subordinator via Subordinator,
            condition: hole Clause,
        }
        derive features: Features = reduce_coordinated_predicate_attachment_features(subordinator, condition);
        form only @ 0 inverse check(is_coordinated_predicate_attachment_comma) = "," identity(subordinator) condition;
        selection unique;
    }

    internal construction coordinated_predicate_attachment_elliptical: CoordinatedPredicateAttachment {
        bind ClauseAttachment via make_coordinated_predicate_attachment_elliptical, coordinated_predicate_attachment_elliptical_parts {
            subordinator: identity Subordinator via Subordinator,
            adjective: hole AdjectivePhrase,
        }
        derive features: Features = reduce_coordinated_predicate_attachment_elliptical_features(subordinator, adjective);
        form only @ 0 inverse check(is_coordinated_predicate_attachment_elliptical) = identity(subordinator) adjective;
        selection unique;
    }

    internal construction shared_grant_ability_complement: SharedGrantComplement {
        bind SharedGrantComplement via make_shared_grant_ability_complement, shared_grant_ability_complement_parts {
            ability: identity CatalogAtom via AbilityItem,
        }
        derive features: Features = reduce_shared_grant_complement_features(ability);
        form only @ 0 inverse check(is_shared_grant_ability_complement) = identity(ability);
        selection unique;
    }

    internal construction shared_grant_quoted_complement: SharedGrantComplement {
        bind SharedGrantComplement via make_shared_grant_quoted_complement, shared_grant_quoted_complement_parts {
            quoted: identity QuotedAbility via QuotedAbility,
        }
        derive features: Features = reduce_shared_grant_complement_features(quoted);
        form only @ 0 inverse check(is_shared_grant_quoted_complement) = identity(quoted);
        selection unique;
    }

    internal construction shared_grant_base: SharedGrantBase {
        bind SharedGrantBase via make_shared_grant_base, shared_grant_base_parts {
            subject: hole NounPhrase,
            head: identity VerbAnalysis via LexicalVerb,
            complement: hole SharedGrantComplement,
            attachment: hole CoordinatedPredicateAttachment,
        }
        derive features: Features = reduce_shared_grant_base_features(subject, head, complement, attachment);
        form only @ 0 inverse check(is_shared_grant_base) = subject identity(head) complement attachment;
        selection unique;
    }

    internal construction shared_grant_prefix_start: SharedGrantPrefix {
        bind SharedGrantPrefix via make_shared_grant_prefix_start, shared_grant_prefix_start_parts {
            first: hole SharedGrantBase,
            complement: hole SharedGrantComplement,
            attachment: hole CoordinatedPredicateAttachment,
        }
        derive features: Features = reduce_shared_grant_coordination_asyndetic_features(first, complement, attachment);
        form only @ 0 inverse check(is_shared_grant_prefix_start) = first "," complement attachment;
        selection unique;
    }

    internal construction shared_grant_prefix_continue: SharedGrantPrefix {
        bind SharedGrantPrefix via make_shared_grant_prefix_continue, shared_grant_prefix_continue_parts {
            first: hole SharedGrantPrefix,
            complement: hole SharedGrantComplement,
            attachment: hole CoordinatedPredicateAttachment,
        }
        derive features: Features = reduce_shared_grant_coordination_asyndetic_features(first, complement, attachment);
        form only @ 0 inverse check(is_shared_grant_prefix_continue) = first "," complement attachment;
        selection unique;
    }

    construction clause_coordination_shared_grant: Clause {
        bind Clause via make_clause_coordination_shared_grant, shared_grant_coordination_conjoined_parts {
            first: hole Clause,
            conjunction: lex Conjunction,
            complement: hole SharedGrantComplement,
            attachment: hole CoordinatedPredicateAttachment,
        }
        require conjunction in [And];
        derive features: Features = reduce_shared_grant_coordination_conjoined_features(first, conjunction, complement, attachment);
        form only @ 0 inverse check(is_clause_coordination_shared_grant) = first lex(conjunction) complement attachment;
        selection unique;
    }

    construction clause_coordination_shared_grant_comma: Clause {
        bind Clause via make_clause_coordination_shared_grant_comma, shared_grant_coordination_comma_parts {
            first: hole SharedGrantPrefix,
            conjunction: lex Conjunction,
            complement: hole SharedGrantComplement,
            attachment: hole CoordinatedPredicateAttachment,
        }
        require conjunction in [And];
        derive features: Features = reduce_shared_grant_coordination_conjoined_features(first, conjunction, complement, attachment);
        form only @ 0 inverse check(is_clause_coordination_shared_grant_comma) = first "," lex(conjunction) complement attachment;
        selection unique;
    }

    construction clause_coordination_shared_grant_asyndetic: Clause {
        bind Clause via make_clause_coordination_shared_grant_asyndetic, shared_grant_coordination_asyndetic_parts {
            first: hole Clause,
            complement: hole SharedGrantComplement,
            attachment: hole CoordinatedPredicateAttachment,
        }
        derive features: Features = reduce_shared_grant_coordination_asyndetic_features(first, complement, attachment);
        form only @ 0 inverse check(is_clause_coordination_shared_grant_asyndetic) = first "," complement attachment;
        selection unique;
    }

    internal construction shared_copular_predicate: SharedCopularPredicate {
        bind Predicate via make_shared_copular_predicate, shared_copular_predicate_parts {
            copula: identity AuxiliaryInstance via Copula,
            complement: hole NounPhrase,
            preposition: hole PrepositionalPhrase,
        }
        derive features: Features = reduce_shared_copular_predicate_features(copula, complement, preposition);
        form only @ 0 inverse check(is_shared_copular_predicate) = identity(copula) complement preposition;
        selection unique;
    }

    construction clause_coordination: Clause {
        bind Clause via make_clause_coordination, clause_coordination_conjoined_parts {
            first: hole Clause,
            conjunction: lex Conjunction,
            next: hole SimpleClause,
        }
        require conjunction in [And, Or, Then, AndOr];
        derive features: Features = reduce_clause_coordination_conjoined_features(first, conjunction, next);
        form only @ 0 inverse check(is_clause_coordination) = first lex(conjunction) next;
        selection unique;
    }

    construction clause_coordination_comma: Clause {
        bind Clause via make_clause_coordination_comma, clause_coordination_conjoined_parts {
            first: hole Clause,
            conjunction: lex Conjunction,
            next: hole SimpleClause,
        }
        require conjunction in [And, Or, Then, AndOr];
        derive features: Features = reduce_clause_coordination_conjoined_features(first, conjunction, next);
        form only @ 0 inverse check(is_clause_coordination_comma) = first "," lex(conjunction) next;
        selection unique;
    }

    construction clause_coordination_asyndetic: Clause {
        bind Clause via make_clause_coordination_asyndetic, clause_coordination_asyndetic_parts {
            first: hole Clause,
            next: hole SimpleClause,
        }
        derive features: Features = reduce_clause_coordination_asyndetic_features(first, next);
        form only @ 0 inverse check(is_clause_coordination_asyndetic) = first "," next;
        selection unique;
    }

    construction clause_coordination_copular_noun_prepositional: Clause {
        bind Clause via make_clause_coordination_copular_noun_prepositional, shared_copular_coordination_conjoined_parts {
            first: hole Clause,
            conjunction: lex Conjunction,
            predicate: hole SharedCopularPredicate,
        }
        require conjunction in [And];
        derive features: Features = reduce_shared_copular_coordination_conjoined_features(first, conjunction, predicate);
        form only @ 0 inverse check(is_clause_coordination_copular_noun_prepositional) = first lex(conjunction) predicate;
        selection unique;
    }

    construction clause_coordination_copular_noun_prepositional_comma: Clause {
        bind Clause via make_clause_coordination_copular_noun_prepositional_comma, shared_copular_coordination_conjoined_parts {
            first: hole Clause,
            conjunction: lex Conjunction,
            predicate: hole SharedCopularPredicate,
        }
        require conjunction in [And];
        derive features: Features = reduce_shared_copular_coordination_conjoined_features(first, conjunction, predicate);
        form only @ 0 inverse check(is_clause_coordination_copular_noun_prepositional_comma) = first "," lex(conjunction) predicate;
        selection unique;
    }

    construction clause_coordination_copular_noun_prepositional_asyndetic: Clause {
        bind Clause via make_clause_coordination_copular_noun_prepositional_asyndetic, shared_copular_coordination_asyndetic_parts {
            first: hole Clause,
            predicate: hole SharedCopularPredicate,
        }
        derive features: Features = reduce_shared_copular_coordination_asyndetic_features(first, predicate);
        form only @ 0 inverse check(is_clause_coordination_copular_noun_prepositional_asyndetic) = first "," predicate;
        selection unique;
    }

    construction clause_elliptical: EllipticalClause {
        bind EllipticalClause via make_clause_elliptical, clause_elliptical_parts {
            adjective: hole AdjectivePhrase,
        }
        derive features: Features = reduce_clause_elliptical_features(adjective);
        form only @ 0 inverse check(is_clause_elliptical) = adjective;
        selection unique;
    }

    construction clause_existential: Clause {
        bind Clause via make_clause_existential, clause_existential_parts {
            existential: identity ExistentialForm via Existential,
            pivot: hole NounPhrase,
        }
        derive features: Features = reduce_clause_existential_features(existential, pivot);
        evidence feature "existential pivot number agreement" from category;
        form only @ 0 inverse check(is_clause_existential) = identity(existential) pivot;
        selection unique;
    }

    construction copular_remainder_noun: CopularRemainder {
        bind CopularRemainder via make_copular_remainder_noun, copular_remainder_noun_parts {
            complement: hole NounPhrase,
        }
        derive features: Features = reduce_copular_remainder_features(complement);
        derive base_precedence: Features = reduce_copular_remainder_features(complement);
        form only @ 0 inverse check(is_copular_remainder_noun) = complement;
        selection unique;
    }

    construction copular_remainder_adjective: CopularRemainder {
        bind CopularRemainder via make_copular_remainder_adjective, copular_remainder_adjective_parts {
            complement: hole AdjectivePhrase,
        }
        derive features: Features = reduce_copular_remainder_features(complement);
        form only @ 0 inverse check(is_copular_remainder_adjective) = complement;
        selection unique;
    }

    construction copular_remainder_coordinated_adjective: CopularRemainder {
        bind CopularRemainder via make_copular_remainder_coordinated_adjective, copular_remainder_coordinated_adjective_parts {
            complement: hole CoordinatedAdjectivePhrase,
        }
        derive features: Features = reduce_copular_remainder_features(complement);
        form only @ 0 inverse check(is_copular_remainder_coordinated_adjective) = complement;
        selection unique;
    }

    construction copular_remainder_prepositional: CopularRemainder {
        bind CopularRemainder via make_copular_remainder_prepositional, copular_remainder_prepositional_parts {
            complement: hole PrepositionalPhrase,
        }
        derive features: Features = reduce_copular_remainder_features(complement);
        form only @ 0 inverse check(is_copular_remainder_prepositional) = complement;
        selection unique;
    }

    construction copular_remainder_power_toughness: CopularRemainder {
        bind CopularRemainder via make_copular_remainder_power_toughness, copular_remainder_power_toughness_parts {
            complement: lex PowerToughness via PowerToughness,
        }
        derive features: Features = reduce_copular_remainder_features(complement);
        form only @ 0 inverse check(is_copular_remainder_power_toughness) = lex(complement);
        selection unique;
    }

    construction copular_remainder_catalog_atom: CopularRemainder {
        bind CopularRemainder via make_copular_remainder_catalog_atom, copular_remainder_catalog_atom_parts {
            complement: identity CatalogAtom via AbilityItem,
        }
        derive features: Features = reduce_copular_remainder_features(complement);
        form only @ 0 inverse check(is_copular_remainder_catalog_atom) = identity(complement);
        selection unique;
    }

    construction copular_remainder_prepositional_adjunct: CopularRemainder {
        bind CopularRemainder via make_copular_remainder_prepositional_adjunct, copular_remainder_prepositional_adjunct_parts {
            remainder: hole CopularRemainder,
            adjunct: hole PrepositionalPhrase,
        }
        derive features: Features = reduce_copular_remainder_binary_features(remainder, adjunct);
        derive base_precedence_8: Features = reduce_copular_remainder_binary_features(remainder, adjunct);
        evidence role "copular adjunct attachment" from category;
        form only @ 0 inverse check(is_copular_remainder_prepositional_adjunct) = remainder adjunct;
        selection unique;
    }

    construction copular_remainder_adverb: CopularRemainder {
        bind CopularRemainder via make_copular_remainder_adverb, copular_remainder_adverb_parts {
            adverb: identity Vocab via Adverb,
            remainder: hole CopularRemainder,
        }
        derive features: Features = reduce_copular_remainder_binary_features(adverb, remainder);
        form only @ 0 inverse check(is_copular_remainder_adverb) = identity(adverb) remainder;
        selection unique;
    }

    construction copular_remainder_negated: CopularRemainder {
        bind CopularRemainder via make_copular_remainder_negated, copular_remainder_negated_parts {
            remainder: hole CopularRemainder,
        }
        derive features: Features = reduce_copular_remainder_features(remainder);
        form only @ 0 inverse check(is_copular_remainder_negated) = "not" remainder;
        selection unique;
    }

    construction copular_remainder_distributive_each: CopularRemainder {
        bind CopularRemainder via make_copular_remainder_distributive_each, copular_remainder_distributive_each_parts {
            adjective: hole AdjectivePhrase,
            standard: hole PrepositionalPhrase,
        }
        derive features: Features = reduce_copular_remainder_binary_features(adjective, standard);
        evidence feature "distributive copular standard" from category;
        form only @ 0 inverse check(is_copular_remainder_distributive_each) = "each" adjective standard;
        selection unique;
    }

    construction clause_copular: Clause {
        bind Clause via make_clause_copular, clause_copular_parts {
            subject: hole NounPhrase,
            copula: identity AuxiliaryInstance via Copula,
            remainder: hole CopularRemainder,
        }
        derive features: Features = reduce_clause_copular_features(subject, copula, remainder);
        evidence feature "copular subject agreement and complement class" from category;
        form only @ 0 inverse check(is_clause_copular) = subject identity(copula) remainder;
        selection unique;
    }

    construction clause_contracted_copular: Clause {
        bind Clause via make_clause_contracted_copular, clause_contracted_copular_parts {
            subject_auxiliary: identity ContractedSubjectAuxiliary via SubjectAuxiliary,
            remainder: hole CopularRemainder,
        }
        derive features: Features = reduce_clause_contracted_copular_features(subject_auxiliary, remainder);
        evidence feature "contracted copular subject agreement" from category;
        form only @ 0 inverse check(is_clause_contracted_copular) = identity(subject_auxiliary) remainder;
        selection unique;
    }

    construction clause_variable_value_constraint: Clause {
        bind Clause via make_clause_variable_value_constraint, variable_value_constraint_parts {
            subject: hole Quantity,
            modal: identity AuxiliaryInstance via Auxiliary,
            copula: identity AuxiliaryInstance via Auxiliary,
            number: lex NumberLiteral via Numeral,
        }
        derive features: Features = reduce_clause_variable_value_constraint_features(subject, modal, copula, number);
        evidence feature "modal variable-value numeric constraint" from category;
        form only @ 0 inverse check(is_clause_variable_value_constraint) = subject identity(modal) identity(copula) lex(number);
        selection unique;
    }
}

pub(crate) static GROUPS: &[&GroupData] = &[&CLAUSE_DECLARATION];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Catalogs;
    use crate::catalog::CatalogKind;
    use crate::catalog::CatalogSlot;
    use crate::catalog::CatalogValue;
    use crate::grammar::GeneratedActivation;
    use crate::grammar::Nonterminal;
    use crate::syntax::ClauseAttachmentKind;

    const IDS: [&str; 39] = [
        "simple_clause_subject",
        "simple_clause_subject_distributive_each",
        "simple_clause_contracted_subject",
        "simple_clause_subjectless",
        "simple_clause_subjectless_attached",
        "clause_simple",
        "coordinated_predicate_attachment",
        "coordinated_predicate_attachment_comma",
        "coordinated_predicate_attachment_elliptical",
        "shared_grant_ability_complement",
        "shared_grant_quoted_complement",
        "shared_grant_base",
        "shared_grant_prefix_start",
        "shared_grant_prefix_continue",
        "clause_coordination_shared_grant",
        "clause_coordination_shared_grant_comma",
        "clause_coordination_shared_grant_asyndetic",
        "shared_copular_predicate",
        "clause_coordination",
        "clause_coordination_comma",
        "clause_coordination_asyndetic",
        "clause_coordination_copular_noun_prepositional",
        "clause_coordination_copular_noun_prepositional_comma",
        "clause_coordination_copular_noun_prepositional_asyndetic",
        "clause_elliptical",
        "clause_existential",
        "copular_remainder_noun",
        "copular_remainder_adjective",
        "copular_remainder_coordinated_adjective",
        "copular_remainder_prepositional",
        "copular_remainder_power_toughness",
        "copular_remainder_catalog_atom",
        "copular_remainder_prepositional_adjunct",
        "copular_remainder_adverb",
        "copular_remainder_negated",
        "copular_remainder_distributive_each",
        "clause_copular",
        "clause_contracted_copular",
        "clause_variable_value_constraint",
    ];

    #[test]
    fn clause_declaration_has_the_expected_id_census_and_dominance() {
        let declaration = GROUPS[0];
        assert_eq!(declaration.name, "clause");
        assert_eq!(
            declaration
                .constructions
                .iter()
                .map(|construction| construction.id)
                .collect::<Vec<_>>(),
            IDS
        );
        let edges = declaration
            .constructions
            .iter()
            .flat_map(|construction| {
                construction
                    .dominates
                    .iter()
                    .map(move |subordinate| (construction.id, *subordinate))
            })
            .collect::<Vec<_>>();
        assert_eq!(edges, [("clause_simple", "clause_copular"),]);
    }

    #[test]
    fn production_fixtures_lower_render_and_report_generated_clause_owners() {
        let catalogs = crate::grammar::fixture_catalogs();
        for (source, required) in [
            (
                "you draw a card",
                &["simple_clause_subject", "clause_simple"][..],
            ),
            (
                "draw a card",
                &["simple_clause_subjectless", "clause_simple"][..],
            ),
            (
                "it's a creature",
                &["copular_remainder_noun", "clause_contracted_copular"][..],
            ),
            ("there are two creatures", &["clause_existential"][..]),
            ("X can't be 0", &["clause_variable_value_constraint"][..]),
        ] {
            let parsed = crate::grammar::parse_nonterminal_with_activation(
                source,
                &catalogs,
                Nonterminal::Clause,
                GeneratedActivation::Production,
            )
            .unwrap_or_else(|error| panic!("failed to parse {source:?}: {error:?}"));
            let clause = parsed.clause().expect("Clause root lowers to Clause");
            assert_eq!(
                crate::renderer::render_generated_clause_law(clause)
                    .unwrap_or_else(|error| panic!("failed to linearize {source:?}: {error:?}")),
                source
            );
            for id in required {
                let decision = parsed
                    .construction_decisions()
                    .iter()
                    .find(|decision| decision.selected().as_str() == *id)
                    .unwrap_or_else(|| panic!("{source:?} did not select {id}"));
                assert_eq!(decision.backend(), crate::ConstructionBackend::Chart);
            }
        }
    }

    #[test]
    fn coordination_inverse_stops_at_an_outer_attachment_scope() {
        let source = "instead flip two coins and ignore one";
        let parsed = crate::grammar::parse_nonterminal_with_activation(
            source,
            &crate::grammar::fixture_catalogs(),
            Nonterminal::Clause,
            GeneratedActivation::Production,
        )
        .unwrap_or_else(|error| panic!("failed to parse {source:?}: {error:?}"));
        let clause = parsed.clause().expect("Clause root lowers to Clause");
        let Clause::Independent(IndependentClause::Complex(complex)) = clause else {
            panic!("the fronted adverb owns the outer clause: {clause:#?}")
        };
        assert!(matches!(
            complex.attachment().payload(),
            ClauseAttachmentKind::Adjunct(PredicateAdjunct::Adverb(Vocab::Instead))
        ));
        assert!(
            !is_clause_coordination(clause),
            "coordination must not project through the outer adverb scope"
        );
        assert!(
            is_clause_coordination(&Clause::Independent(complex.host().clone())),
            "the attachment host retains the coordinated matrix owner"
        );
    }

    #[test]
    fn shared_grant_builder_bootstraps_from_one_conditioned_member() {
        let catalogs = Catalogs::default()
            .with_catalog(CatalogKind::KeywordAbility, ["Trample", "Haste"])
            .with_catalog(CatalogKind::CreatureType, ["Beast", "Goblin"]);
        let parse_clause = |source| {
            let parsed = crate::grammar::parse_nonterminal_with_activation(
                source,
                &catalogs,
                Nonterminal::Clause,
                GeneratedActivation::Production,
            )
            .unwrap_or_else(|error| panic!("failed to parse {source:?}: {error:?}"));
            parsed
                .clause()
                .expect("the source has a clause root")
                .clone()
        };
        let first = parse_clause("it has trample as long as you control a Beast");
        let condition = parse_clause("you control a Goblin");
        let CatalogValue::Atom(haste) = catalogs.matches("haste", CatalogSlot::AbilityItem)[0]
            .value
            .clone()
        else {
            panic!("the ability-item match is a catalog atom")
        };
        let attachment = ClauseAttachment::from_declaration_parts(
            crate::syntax::AttachmentPosition::AfterMatrix,
            Comma::Absent,
            crate::syntax::ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
                Subordinator::AsLongAs,
                SubordinateBody::Finite(Box::new(match condition {
                    Clause::Independent(condition) => condition,
                    Clause::Dependent(_) => panic!("the condition is independent"),
                })),
            )),
        );
        let coordinated = build_clause_coordination_shared_grant_asyndetic(
            first,
            SharedGrantComplement::Ability(haste),
            attachment,
        )
        .expect("one explicit member-local condition bootstraps the elided shared head");
        assert!(is_clause_coordination_shared_grant_asyndetic(&coordinated));
    }

    #[test]
    fn clause_syntax_and_selection_are_registration_order_neutral() {
        let catalogs = crate::grammar::fixture_catalogs();
        for source in [
            "you draw a card",
            "draw a card",
            "it's a creature",
            "there are two creatures",
            "X can't be 0",
        ] {
            let parses = crate::grammar::parse_nonterminal_with_activation_in_both_orders(
                source,
                &catalogs,
                Nonterminal::Clause,
                &crate::identity::SelfReference::default(),
                GeneratedActivation::Production,
            );
            assert_eq!(parses[0].clause(), parses[1].clause(), "{source:?}");
            let selected = |parsed: &crate::grammar::ParsedNonterminal| {
                parsed
                    .construction_decisions()
                    .iter()
                    .filter(|decision| IDS.contains(&decision.selected().as_str()))
                    .map(|decision| {
                        (
                            decision.selected().as_str(),
                            decision.selected_production_ordinal(),
                        )
                    })
                    .collect::<Vec<_>>()
            };
            assert_eq!(selected(&parses[0]), selected(&parses[1]), "{source:?}");
        }
    }

    #[test]
    fn variable_value_builder_rejects_each_closed_domain_violation() {
        let x = Quantity::unchecked_x();
        let one = Quantity::try_exact(NumberLiteral {
            value: 1,
            numeral: crate::numeral::Numeral::Arabic(false),
        })
        .unwrap();
        let modal = AuxiliaryInstance {
            auxiliary: Auxiliary::Can,
            inflection: AuxiliaryInflection::Base,
            contracted_negation: Contraction::Full,
        };
        let be = AuxiliaryInstance {
            auxiliary: Auxiliary::Be,
            inflection: AuxiliaryInflection::Base,
            contracted_negation: Contraction::Full,
        };
        let number = NumberLiteral {
            value: 0,
            numeral: crate::numeral::Numeral::Arabic(false),
        };
        assert!(build_clause_variable_value_constraint(one, modal, be, number).is_err());
        assert!(
            build_clause_variable_value_constraint(x, be, be, number).is_err(),
            "the first auxiliary must be modal"
        );
        assert!(
            build_clause_variable_value_constraint(x, modal, modal, number).is_err(),
            "the complement auxiliary must be base-form be"
        );
    }

    #[test]
    fn elliptical_builder_parts_and_generated_inverse_are_exact() {
        let parsed = crate::grammar::parse_nonterminal_with_activation(
            "red",
            &crate::grammar::fixture_catalogs(),
            Nonterminal::AdjectivePhrase,
            GeneratedActivation::Production,
        )
        .expect("the adjective fixture parses");
        let adjective = parsed
            .adjective_phrase()
            .expect("the requested root lowers as an adjective")
            .clone();
        let clause = build_clause_elliptical(adjective.clone()).expect("the adjective builds");
        assert_eq!(parts_clause_elliptical(&clause), adjective);
        assert_eq!(
            crate::renderer::render_generated_elliptical_clause_law(&clause).unwrap(),
            "red"
        );
    }
}

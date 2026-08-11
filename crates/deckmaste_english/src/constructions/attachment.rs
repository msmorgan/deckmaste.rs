//! Compiler-derived clause attachment, exception-rider, and restriction
//! declarations.

#![allow(
    clippy::needless_pass_by_value,
    clippy::too_many_lines,
    reason = "declaration adapters own erased values and one form dispatcher per F03 family"
)]

use deckmaste_construction_compiler::runtime::DeclarationViolation;
use deckmaste_construction_compiler::runtime::GroupData;

use crate::features::Comma;
use crate::features::Conjunction;
use crate::grammar::Features;
use crate::grammar::PredicateForm;
use crate::grammar::VerbPhrase;
use crate::syntax::AdjectivePhrase;
use crate::syntax::AttachmentPosition;
use crate::syntax::Clause;
use crate::syntax::ClauseAttachment;
use crate::syntax::ClauseAttachmentKind;
use crate::syntax::ComplexClause;
use crate::syntax::DependentClause;
use crate::syntax::EllipticalClause;
use crate::syntax::ExceptionConjunct;
use crate::syntax::ExceptionRider;
use crate::syntax::ExceptionRiderList;
use crate::syntax::GerundClause;
use crate::syntax::IndependentClause;
use crate::syntax::InfinitiveClause;
use crate::syntax::NounPhrase;
use crate::syntax::PredicateAdjunct;
use crate::syntax::PrepositionalPhrase;
use crate::syntax::RestrictionCoordination;
use crate::syntax::RestrictionMember;
use crate::syntax::RestrictionRun;
use crate::syntax::SubordinateBody;
use crate::syntax::Subordinator;
use crate::word::Vocab;

fn violation(construction: &'static str, requirement: &'static str) -> DeclarationViolation {
    DeclarationViolation {
        construction,
        requirement,
    }
}

fn independent(
    construction: &'static str,
    clause: Clause,
) -> Result<IndependentClause, DeclarationViolation> {
    let Clause::Independent(clause) = clause else {
        return Err(violation(construction, "the clause is an independent host"));
    };
    Ok(clause)
}

fn with_attachment(
    construction: &'static str,
    host: Clause,
    attachment: ClauseAttachment,
) -> Result<Clause, DeclarationViolation> {
    let host = independent(construction, host)?;
    Ok(Clause::Independent(IndependentClause::Complex(
        ComplexClause::from_declaration_parts(host, attachment),
    )))
}

fn remove_attachment(
    value: &Clause,
    position: AttachmentPosition,
    predicate: impl FnOnce(&ClauseAttachmentKind) -> bool,
) -> Option<(Clause, ClauseAttachment)> {
    let Clause::Independent(IndependentClause::Complex(complex)) = value else {
        return None;
    };
    let attachment = complex.attachment();
    if attachment.position() != position || !predicate(attachment.payload()) {
        return None;
    }
    Some((
        Clause::Independent(complex.host().clone()),
        attachment.clone(),
    ))
}

fn clause_features(features: &Features) -> Option<Features> {
    let Features::Clause {
        agreement,
        standalone: true,
        finite,
        host_addressee_subject,
        host_modal,
        subjunctive: false,
    } = features
    else {
        return None;
    };
    Some(Features::Clause {
        agreement: *agreement,
        standalone: true,
        finite: *finite,
        host_addressee_subject: *host_addressee_subject,
        host_modal: *host_modal,
        subjunctive: false,
    })
}

fn conditional_features(
    subordinator: &Features,
    condition: &Features,
    host: &Features,
) -> Option<Features> {
    let Features::Subordinator(subordinator) = subordinator else {
        return None;
    };
    let Features::Clause {
        standalone: true,
        finite: true,
        subjunctive,
        ..
    } = condition
    else {
        return None;
    };
    if *subjunctive && *subordinator != Subordinator::AsThough {
        return None;
    }
    clause_features(host)
}

fn make_clause_adverb_before(adverb: Vocab, host: Clause) -> Result<Clause, DeclarationViolation> {
    with_attachment(
        "clause_adverb_before",
        host,
        ClauseAttachment::from_declaration_parts(
            AttachmentPosition::BeforeMatrix,
            Comma::Absent,
            ClauseAttachmentKind::Adjunct(PredicateAdjunct::Adverb(adverb)),
        ),
    )
}

fn clause_adverb_before_parts(value: &Clause) -> (Vocab, Clause) {
    let (host, attachment) = remove_attachment(value, AttachmentPosition::BeforeMatrix, |kind| {
        matches!(
            kind,
            ClauseAttachmentKind::Adjunct(PredicateAdjunct::Adverb(_))
        )
    })
    .expect("the adverb-before recognizer admits one front attachment");
    let ClauseAttachmentKind::Adjunct(PredicateAdjunct::Adverb(adverb)) = attachment.payload else {
        unreachable!()
    };
    (adverb, host)
}

fn is_clause_adverb_before(value: &Clause) -> bool {
    remove_attachment(value, AttachmentPosition::BeforeMatrix, |kind| {
        matches!(
            kind,
            ClauseAttachmentKind::Adjunct(PredicateAdjunct::Adverb(_))
        )
    })
    .is_some_and(|(_, attachment)| attachment.comma == Comma::Absent)
}

fn make_clause_sentence_adverbial_before(
    adverb: Vocab,
    host: Clause,
) -> Result<Clause, DeclarationViolation> {
    with_attachment(
        "clause_sentence_adverbial_before",
        host,
        ClauseAttachment::from_declaration_parts(
            AttachmentPosition::BeforeMatrix,
            Comma::Present,
            ClauseAttachmentKind::Adjunct(PredicateAdjunct::Adverb(adverb)),
        ),
    )
}

fn clause_sentence_adverbial_before_parts(value: &Clause) -> (Vocab, Clause) {
    clause_adverb_before_parts(value)
}

fn is_clause_sentence_adverbial_before(value: &Clause) -> bool {
    remove_attachment(value, AttachmentPosition::BeforeMatrix, |kind| {
        matches!(
            kind,
            ClauseAttachmentKind::Adjunct(PredicateAdjunct::Adverb(_))
        )
    })
    .is_some_and(|(_, attachment)| attachment.comma == Comma::Present)
}

fn make_clause_prepositional_before(
    preposition: PrepositionalPhrase,
    host: Clause,
) -> Result<Clause, DeclarationViolation> {
    with_attachment(
        "clause_prepositional_before",
        host,
        ClauseAttachment::from_declaration_parts(
            AttachmentPosition::BeforeMatrix,
            Comma::Present,
            ClauseAttachmentKind::Adjunct(PredicateAdjunct::Prepositional(preposition)),
        ),
    )
}

fn clause_prepositional_before_parts(value: &Clause) -> (PrepositionalPhrase, Clause) {
    let (host, attachment) = remove_attachment(value, AttachmentPosition::BeforeMatrix, |kind| {
        matches!(
            kind,
            ClauseAttachmentKind::Adjunct(PredicateAdjunct::Prepositional(_))
        )
    })
    .expect("the PP-before recognizer admits one front attachment");
    let ClauseAttachmentKind::Adjunct(PredicateAdjunct::Prepositional(preposition)) =
        attachment.payload
    else {
        unreachable!()
    };
    (preposition, host)
}

fn is_clause_prepositional_before(value: &Clause) -> bool {
    remove_attachment(value, AttachmentPosition::BeforeMatrix, |kind| {
        matches!(
            kind,
            ClauseAttachmentKind::Adjunct(PredicateAdjunct::Prepositional(_))
        )
    })
    .is_some_and(|(_, attachment)| attachment.comma == Comma::Present)
}

fn make_subordinate_attachment(
    construction: &'static str,
    subordinator: Subordinator,
    body: SubordinateBody,
    host: Clause,
    position: AttachmentPosition,
    comma: Comma,
) -> Result<Clause, DeclarationViolation> {
    with_attachment(
        construction,
        host,
        ClauseAttachment::from_declaration_parts(
            position,
            comma,
            ClauseAttachmentKind::Dependent(DependentClause::Subordinate(subordinator, body)),
        ),
    )
}

fn finite_body(
    construction: &'static str,
    condition: Clause,
) -> Result<SubordinateBody, DeclarationViolation> {
    Ok(SubordinateBody::Finite(Box::new(independent(
        construction,
        condition,
    )?)))
}

fn make_clause_subordinate_before(
    subordinator: Subordinator,
    condition: Clause,
    host: Clause,
) -> Result<Clause, DeclarationViolation> {
    let body = finite_body("clause_subordinate_before", condition)?;
    make_subordinate_attachment(
        "clause_subordinate_before",
        subordinator,
        body,
        host,
        AttachmentPosition::BeforeMatrix,
        Comma::Present,
    )
}

fn finite_subordinate_parts(
    value: &Clause,
    position: AttachmentPosition,
) -> (Subordinator, Clause, Clause) {
    let (host, attachment) = remove_attachment(value, position, |kind| {
        matches!(
            kind,
            ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
                _,
                SubordinateBody::Finite(_)
            ))
        )
    })
    .expect("the finite subordinate recognizer admits one attachment");
    let ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
        subordinator,
        SubordinateBody::Finite(condition),
    )) = attachment.payload
    else {
        unreachable!()
    };
    (subordinator, Clause::Independent(*condition), host)
}

fn clause_subordinate_before_parts(value: &Clause) -> (Subordinator, Clause, Clause) {
    finite_subordinate_parts(value, AttachmentPosition::BeforeMatrix)
}

fn is_clause_subordinate_before(value: &Clause) -> bool {
    remove_attachment(value, AttachmentPosition::BeforeMatrix, |kind| {
        matches!(
            kind,
            ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
                _,
                SubordinateBody::Finite(_)
            ))
        )
    })
    .is_some_and(|(_, attachment)| attachment.comma == Comma::Present)
}

fn make_clause_subordinate_gerund_before(
    gerund: GerundClause,
    host: Clause,
) -> Result<Clause, DeclarationViolation> {
    make_subordinate_attachment(
        "clause_subordinate_gerund_before",
        Subordinator::While,
        SubordinateBody::Gerund(gerund),
        host,
        AttachmentPosition::BeforeMatrix,
        Comma::Present,
    )
}

fn clause_subordinate_gerund_before_parts(value: &Clause) -> (GerundClause, Clause) {
    let (host, attachment) = remove_attachment(value, AttachmentPosition::BeforeMatrix, |kind| {
        matches!(
            kind,
            ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
                Subordinator::While,
                SubordinateBody::Gerund(_)
            ))
        )
    })
    .expect("the while-gerund recognizer admits one front attachment");
    let ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
        Subordinator::While,
        SubordinateBody::Gerund(gerund),
    )) = attachment.payload
    else {
        unreachable!()
    };
    (gerund, host)
}

fn is_clause_subordinate_gerund_before(value: &Clause) -> bool {
    remove_attachment(value, AttachmentPosition::BeforeMatrix, |kind| {
        matches!(
            kind,
            ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
                Subordinator::While,
                SubordinateBody::Gerund(_)
            ))
        )
    })
    .is_some_and(|(_, attachment)| attachment.comma == Comma::Present)
}

fn make_clause_subordinate_after_elliptical(
    host: Clause,
    subordinator: Subordinator,
    adjective: AdjectivePhrase,
) -> Result<Clause, DeclarationViolation> {
    make_subordinate_attachment(
        "clause_subordinate_after_elliptical",
        subordinator,
        SubordinateBody::Elliptical(EllipticalClause::Adjective(adjective)),
        host,
        AttachmentPosition::AfterMatrix,
        Comma::Absent,
    )
}

fn clause_subordinate_after_elliptical_parts(
    value: &Clause,
) -> (Clause, Subordinator, AdjectivePhrase) {
    let (host, attachment) = remove_attachment(value, AttachmentPosition::AfterMatrix, |kind| {
        matches!(
            kind,
            ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
                _,
                SubordinateBody::Elliptical(_)
            ))
        )
    })
    .expect("the elliptical recognizer admits one trailing attachment");
    let ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
        subordinator,
        SubordinateBody::Elliptical(EllipticalClause::Adjective(adjective)),
    )) = attachment.payload
    else {
        unreachable!()
    };
    (host, subordinator, adjective)
}

fn is_clause_subordinate_after_elliptical(value: &Clause) -> bool {
    remove_attachment(value, AttachmentPosition::AfterMatrix, |kind| {
        matches!(
            kind,
            ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
                _,
                SubordinateBody::Elliptical(_)
            ))
        )
    })
    .is_some_and(|(_, attachment)| attachment.comma == Comma::Absent)
}

fn make_clause_subordinate_after(
    host: Clause,
    subordinator: Subordinator,
    condition: Clause,
) -> Result<Clause, DeclarationViolation> {
    let body = finite_body("clause_subordinate_after", condition)?;
    make_subordinate_attachment(
        "clause_subordinate_after",
        subordinator,
        body,
        host,
        AttachmentPosition::AfterMatrix,
        Comma::Absent,
    )
}

fn clause_subordinate_after_parts(value: &Clause) -> (Clause, Subordinator, Clause) {
    let (subordinator, condition, host) =
        finite_subordinate_parts(value, AttachmentPosition::AfterMatrix);
    (host, subordinator, condition)
}

fn is_clause_subordinate_after(value: &Clause) -> bool {
    remove_attachment(value, AttachmentPosition::AfterMatrix, |kind| {
        matches!(
            kind,
            ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
                _,
                SubordinateBody::Finite(_)
            ))
        )
    })
    .is_some_and(|(_, attachment)| attachment.comma == Comma::Absent)
}

fn make_clause_subordinate_after_comma(
    host: Clause,
    subordinator: Subordinator,
    condition: Clause,
) -> Result<Clause, DeclarationViolation> {
    let body = finite_body("clause_subordinate_after_comma", condition)?;
    make_subordinate_attachment(
        "clause_subordinate_after_comma",
        subordinator,
        body,
        host,
        AttachmentPosition::AfterMatrix,
        Comma::Present,
    )
}

fn clause_subordinate_after_comma_parts(value: &Clause) -> (Clause, Subordinator, Clause) {
    clause_subordinate_after_parts(value)
}

fn is_clause_subordinate_after_comma(value: &Clause) -> bool {
    remove_attachment(value, AttachmentPosition::AfterMatrix, |kind| {
        matches!(
            kind,
            ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
                _,
                SubordinateBody::Finite(_)
            ))
        )
    })
    .is_some_and(|(_, attachment)| attachment.comma == Comma::Present)
}

fn checked_bare_infinitive(
    predicate: VerbPhrase,
) -> Result<InfinitiveClause, DeclarationViolation> {
    if !matches!(
        predicate.declaration_core_features(),
        Some(Features::VerbPhrase {
            form: PredicateForm::Infinitive,
            ..
        })
    ) || !predicate.declaration_core_arguments_complete()
    {
        return Err(violation(
            "clause_subordinate_after_infinitive",
            "the rather-than predicate is a complete bare infinitive",
        ));
    }
    let crate::constructions::predicate::FinishedPredicate {
        modal: None,
        predicate,
        ..
    } = crate::constructions::predicate::project_public_predicate(predicate)?
    else {
        return Err(violation(
            "clause_subordinate_after_infinitive",
            "the rather-than infinitive has no finite modal",
        ));
    };
    Ok(InfinitiveClause::declaration_bare(predicate))
}

fn make_clause_subordinate_after_infinitive(
    host: Clause,
    predicate: VerbPhrase,
) -> Result<Clause, DeclarationViolation> {
    let infinitive = checked_bare_infinitive(predicate)?;
    make_subordinate_attachment(
        "clause_subordinate_after_infinitive",
        Subordinator::RatherThan,
        SubordinateBody::Infinitive(infinitive),
        host,
        AttachmentPosition::AfterMatrix,
        Comma::Absent,
    )
}

fn clause_subordinate_after_infinitive_parts(value: &Clause) -> (Clause, VerbPhrase) {
    let (host, attachment) = remove_attachment(value, AttachmentPosition::AfterMatrix, |kind| {
        matches!(
            kind,
            ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
                Subordinator::RatherThan,
                SubordinateBody::Infinitive(_)
            ))
        )
    })
    .expect("the rather-than recognizer admits one trailing infinitive");
    let ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
        Subordinator::RatherThan,
        SubordinateBody::Infinitive(infinitive),
    )) = attachment.payload
    else {
        unreachable!()
    };
    let predicate =
        crate::constructions::predicate::inverse_public_predicate(infinitive.predicate())
            .expect("the bare infinitive recognizer admits an invertible predicate");
    (host, predicate)
}

fn is_clause_subordinate_after_infinitive(value: &Clause) -> bool {
    remove_attachment(value, AttachmentPosition::AfterMatrix, |kind| {
        matches!(
            kind,
            ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
                Subordinator::RatherThan,
                SubordinateBody::Infinitive(_)
            ))
        )
    })
    .is_some_and(|(_, attachment)| attachment.comma == Comma::Absent)
}

fn reduce_infinitive_attachment_features(
    host: &Features,
    predicate: &Features,
) -> Option<Features> {
    let Features::VerbPhrase {
        form: PredicateForm::Infinitive,
        passive,
        object,
        indirect_object,
        selected_preposition,
        frame,
        ..
    } = predicate
    else {
        return None;
    };
    crate::grammar::predicate_arguments_complete(
        *frame,
        *passive,
        *object,
        *indirect_object,
        *selected_preposition,
    )
    .then(|| clause_features(host))?
}

fn make_exception_rider_single(clause: Clause) -> Result<ExceptionRider, DeclarationViolation> {
    Ok(ExceptionRider::from_declaration_parts(
        independent("exception_rider_single", clause)?,
        Vec::new(),
    ))
}

fn exception_rider_single_parts(value: &ExceptionRider) -> Clause {
    Clause::Independent(value.first.as_ref().clone())
}

fn is_exception_rider_single(value: &ExceptionRider) -> bool {
    value.rest.is_empty()
}

fn append_exception(
    construction: &'static str,
    mut rider: ExceptionRider,
    conjunction: Option<Conjunction>,
    clause: Clause,
) -> Result<ExceptionRider, DeclarationViolation> {
    if conjunction.is_some_and(|value| {
        !matches!(
            value,
            Conjunction::And | Conjunction::Or | Conjunction::Then
        )
    }) {
        return Err(violation(
            construction,
            "the exception conjunction is and, or, or then",
        ));
    }
    rider.rest.push(ExceptionConjunct::from_declaration_parts(
        conjunction,
        independent(construction, clause)?,
    ));
    Ok(rider)
}

fn make_exception_rider_conjoined(
    rider: ExceptionRider,
    conjunction: Conjunction,
    clause: Clause,
) -> Result<ExceptionRider, DeclarationViolation> {
    if !rider.rest.is_empty() {
        return Err(violation(
            "exception_rider_conjoined",
            "the bare conjunction closes a two-member rider",
        ));
    }
    append_exception(
        "exception_rider_conjoined",
        rider,
        Some(conjunction),
        clause,
    )
}

fn exception_rider_conjoined_parts(
    value: &ExceptionRider,
) -> (ExceptionRider, Conjunction, Clause) {
    let mut rider = value.clone();
    let last = rider
        .rest
        .pop()
        .expect("conjoined rider has a continuation");
    (
        rider,
        last.conjunction.expect("conjoined rider has a conjunction"),
        Clause::Independent(last.clause),
    )
}

fn is_exception_rider_conjoined(value: &ExceptionRider) -> bool {
    value.rest.len() == 1 && value.rest[0].conjunction.is_some()
}

fn make_exception_rider_comma(
    rider: Option<ExceptionRider>,
    list: Option<ExceptionRiderList>,
    clause: Clause,
) -> Result<ExceptionRiderList, DeclarationViolation> {
    let rider = match (rider, list) {
        (Some(rider), None) if rider.rest.is_empty() => rider,
        (None, Some(list)) => list.into_rider(),
        _ => {
            return Err(violation(
                "exception_rider_comma",
                "a comma extends a single or comma-open rider",
            ));
        }
    };
    if rider
        .rest
        .last()
        .is_some_and(|member| member.conjunction.is_some())
    {
        return Err(violation(
            "exception_rider_comma",
            "a comma extends a single or comma-open rider",
        ));
    }
    append_exception("exception_rider_comma", rider, None, clause).map(ExceptionRiderList)
}

fn exception_rider_comma_parts(
    value: &ExceptionRiderList,
) -> (Option<ExceptionRider>, Option<ExceptionRiderList>, Clause) {
    let mut rider = value.rider().clone();
    let last = rider.rest.pop().expect("comma rider has a continuation");
    if rider.rest.is_empty() {
        (Some(rider), None, Clause::Independent(last.clause))
    } else {
        (
            None,
            Some(ExceptionRiderList(rider)),
            Clause::Independent(last.clause),
        )
    }
}

fn is_exception_rider_comma(value: &ExceptionRiderList) -> bool {
    value
        .rider()
        .rest
        .last()
        .is_some_and(|member| member.conjunction.is_none())
}

fn make_exception_rider_oxford(
    rider: ExceptionRiderList,
    conjunction: Conjunction,
    clause: Clause,
) -> Result<ExceptionRider, DeclarationViolation> {
    let rider = rider.into_rider();
    if rider.rest.is_empty()
        || rider
            .rest
            .last()
            .is_some_and(|member| member.conjunction.is_some())
    {
        return Err(violation(
            "exception_rider_oxford",
            "an Oxford close follows at least one comma-open member",
        ));
    }
    append_exception("exception_rider_oxford", rider, Some(conjunction), clause)
}

fn exception_rider_oxford_parts(
    value: &ExceptionRider,
) -> (ExceptionRiderList, Conjunction, Clause) {
    let (rider, conjunction, clause) = exception_rider_conjoined_parts(value);
    (ExceptionRiderList(rider), conjunction, clause)
}

fn is_exception_rider_oxford(value: &ExceptionRider) -> bool {
    value.rest.len() >= 2
        && value
            .rest
            .last()
            .is_some_and(|member| member.conjunction.is_some())
}

fn reduce_exception_single_features(clause: &Features) -> Option<Features> {
    matches!(
        clause,
        Features::Clause {
            standalone: true,
            ..
        }
    )
    .then_some(Features::ExceptionRider)
}

fn reduce_exception_append_features(rider: &Features, clause: &Features) -> Option<Features> {
    (matches!(rider, Features::ExceptionRider)
        && matches!(
            clause,
            Features::Clause {
                standalone: true,
                ..
            }
        ))
    .then_some(Features::ExceptionRider)
}

fn reduce_exception_comma_features(
    rider: Option<&Features>,
    list: Option<&Features>,
    clause: &Features,
) -> Option<Features> {
    let source_ok = matches!(
        (rider, list),
        (Some(Features::ExceptionRider), None) | (None, Some(Features::ExceptionRider))
    );
    (source_ok
        && matches!(
            clause,
            Features::Clause {
                standalone: true,
                ..
            }
        ))
    .then_some(Features::ExceptionRider)
}

fn reduce_exception_conjoined_features(
    rider: &Features,
    conjunction: &Features,
    clause: &Features,
) -> Option<Features> {
    (matches!(
        conjunction,
        Features::Conjunction(Conjunction::And | Conjunction::Or | Conjunction::Then)
    ))
    .then(|| reduce_exception_append_features(rider, clause))?
}

fn reduce_open_exception_dispreference(
    rider: Option<&Features>,
    list: Option<&Features>,
) -> Option<Features> {
    matches!((rider, list), (None, Some(Features::ExceptionRider))).then_some(Features::None)
}

fn make_clause_excepted(
    host: Clause,
    rider: Option<ExceptionRider>,
    list: Option<ExceptionRiderList>,
) -> Result<Clause, DeclarationViolation> {
    let rider = match (rider, list) {
        (Some(rider), None) => rider,
        (None, Some(list)) => list.into_rider(),
        _ => {
            return Err(violation(
                "clause_excepted",
                "exactly one complete or comma-open rider is present",
            ));
        }
    };
    with_attachment(
        "clause_excepted",
        host,
        ClauseAttachment::from_declaration_parts(
            AttachmentPosition::AfterMatrix,
            Comma::Present,
            ClauseAttachmentKind::Exception(rider),
        ),
    )
}

fn clause_excepted_parts(
    value: &Clause,
) -> (Clause, Option<ExceptionRider>, Option<ExceptionRiderList>) {
    let (host, attachment) = remove_attachment(value, AttachmentPosition::AfterMatrix, |kind| {
        matches!(kind, ClauseAttachmentKind::Exception(_))
    })
    .expect("the excepted recognizer admits one trailing rider");
    let ClauseAttachmentKind::Exception(rider) = attachment.payload else {
        unreachable!()
    };
    if rider
        .rest
        .last()
        .is_some_and(|member| member.conjunction.is_none())
    {
        (host, None, Some(ExceptionRiderList(rider)))
    } else {
        (host, Some(rider), None)
    }
}

fn is_clause_excepted(value: &Clause) -> bool {
    remove_attachment(value, AttachmentPosition::AfterMatrix, |kind| {
        matches!(kind, ClauseAttachmentKind::Exception(_))
    })
    .is_some_and(|(_, attachment)| attachment.comma == Comma::Present)
}

fn make_clause_restriction_member(
    preposition: Option<PrepositionalPhrase>,
    condition: Option<Clause>,
    temporal: Option<NounPhrase>,
) -> Result<RestrictionMember, DeclarationViolation> {
    let adjuncts = match (preposition, condition, temporal) {
        (Some(preposition), None, None) => vec![PredicateAdjunct::Prepositional(preposition)],
        (None, Some(condition), None) => vec![PredicateAdjunct::Dependent(Box::new(
            DependentClause::Subordinate(
                Subordinator::If,
                SubordinateBody::Finite(Box::new(independent(
                    "clause_restriction_member",
                    condition,
                )?)),
            ),
        ))],
        (None, None, None) => vec![PredicateAdjunct::Adverb(
            Vocab::from_spelling("once").expect("once is a registered adverb"),
        )],
        (None, None, Some(temporal)) => vec![
            PredicateAdjunct::Adverb(
                Vocab::from_spelling("once").expect("once is a registered adverb"),
            ),
            PredicateAdjunct::Temporal(temporal),
        ],
        _ => {
            return Err(violation(
                "clause_restriction_member",
                "exactly one declared only-member form is present",
            ));
        }
    };
    RestrictionMember::from_declaration_parts(adjuncts).ok_or_else(|| {
        violation(
            "clause_restriction_member",
            "the restriction member is non-empty",
        )
    })
}

fn clause_restriction_member_parts(
    value: &RestrictionMember,
) -> (
    Option<PrepositionalPhrase>,
    Option<Clause>,
    Option<NounPhrase>,
) {
    match value.adjuncts() {
        [PredicateAdjunct::Prepositional(value)] => (Some(value.clone()), None, None),
        [PredicateAdjunct::Dependent(value)] => {
            let DependentClause::Subordinate(Subordinator::If, SubordinateBody::Finite(condition)) =
                value.as_ref()
            else {
                return (None, None, None);
            };
            (
                None,
                Some(Clause::Independent(condition.as_ref().clone())),
                None,
            )
        }
        [PredicateAdjunct::Adverb(word)] if word.spelling() == "once" => (None, None, None),
        [
            PredicateAdjunct::Adverb(word),
            PredicateAdjunct::Temporal(value),
        ] if word.spelling() == "once" => (None, None, Some(value.clone())),
        _ => (None, None, None),
    }
}

fn is_restriction_preposition(value: &RestrictionMember) -> bool {
    matches!(value.adjuncts(), [PredicateAdjunct::Prepositional(_)])
}

fn is_restriction_condition(value: &RestrictionMember) -> bool {
    matches!(
        value.adjuncts(),
        [PredicateAdjunct::Dependent(value)] if matches!(
            value.as_ref(),
            DependentClause::Subordinate(Subordinator::If, SubordinateBody::Finite(_))
        )
    )
}

fn is_restriction_once(value: &RestrictionMember) -> bool {
    matches!(value.adjuncts(), [PredicateAdjunct::Adverb(word)] if word.spelling() == "once")
}

fn is_restriction_once_temporal(value: &RestrictionMember) -> bool {
    matches!(
        value.adjuncts(),
        [PredicateAdjunct::Adverb(word), PredicateAdjunct::Temporal(_)] if word.spelling() == "once"
    )
}

fn reduce_restriction_member_features(
    preposition: Option<&Features>,
    condition: Option<&Features>,
    temporal: Option<&Features>,
) -> Option<Features> {
    match (preposition, condition, temporal) {
        (Some(Features::PrepositionalPhrase { .. }) | None, None, None)
        | (
            None,
            Some(Features::Clause {
                standalone: true, ..
            }),
            None,
        )
        | (None, None, Some(Features::NounPhrase { .. })) => Some(Features::RestrictionMember),
        _ => None,
    }
}

fn restriction_parts(value: &Clause) -> Option<(Clause, RestrictionRun)> {
    let (host, attachment) = remove_attachment(value, AttachmentPosition::AfterMatrix, |kind| {
        matches!(kind, ClauseAttachmentKind::Restriction(_))
    })?;
    if attachment.comma != Comma::Absent {
        return None;
    }
    let ClauseAttachmentKind::Restriction(run) = attachment.payload else {
        unreachable!()
    };
    Some((host, run))
}

fn attach_restriction(host: Clause, run: RestrictionRun) -> Result<Clause, DeclarationViolation> {
    with_attachment(
        "clause_restriction_run",
        host,
        ClauseAttachment::from_declaration_parts(
            AttachmentPosition::AfterMatrix,
            Comma::Absent,
            ClauseAttachmentKind::Restriction(run),
        ),
    )
}

fn make_clause_restriction_run(
    host: Clause,
    first: RestrictionMember,
    rest: Vec<RestrictionCoordination>,
) -> Result<Clause, DeclarationViolation> {
    if rest.is_empty() {
        return Err(violation(
            "clause_restriction_run",
            "a restriction run has at least two members",
        ));
    }
    if rest
        .iter()
        .take(rest.len().saturating_sub(1))
        .any(|member| member.conjunction.is_some())
        || rest
            .last()
            .is_none_or(|member| member.conjunction != Some(Conjunction::And))
    {
        return Err(violation(
            "clause_restriction_run",
            "only the final restriction member carries and",
        ));
    }
    attach_restriction(host, RestrictionRun::from_declaration_parts(first, rest))
}

fn clause_restriction_run_parts(
    value: &Clause,
) -> (Clause, RestrictionMember, Vec<RestrictionCoordination>) {
    let (host, run) =
        restriction_parts(value).expect("a restriction form has one trailing declared run");
    (host, run.first, run.rest)
}

fn restriction_run(value: &Clause) -> Option<&RestrictionRun> {
    let Clause::Independent(IndependentClause::Complex(complex)) = value else {
        return None;
    };
    let attachment = complex.attachment();
    if attachment.position() != AttachmentPosition::AfterMatrix
        || attachment.comma() != Comma::Absent
    {
        return None;
    }
    let ClauseAttachmentKind::Restriction(run) = attachment.payload() else {
        return None;
    };
    Some(run)
}

fn is_clause_restriction_run(value: &Clause) -> bool {
    restriction_run(value).is_some_and(|run| {
        !run.rest.is_empty()
            && run
                .rest
                .iter()
                .take(run.rest.len().saturating_sub(1))
                .all(|member| member.conjunction.is_none())
            && run
                .rest
                .last()
                .is_some_and(|member| member.conjunction == Some(Conjunction::And))
    })
}

fn reduce_restriction_run_features(
    host: &Features,
    first: &Features,
    rest: &Features,
) -> Option<Features> {
    if !matches!(
        host,
        Features::Clause {
            standalone: true,
            subjunctive: false,
            ..
        }
    ) || !matches!(first, Features::RestrictionMember)
        || !matches!(rest, Features::GeneratedSequence { .. })
    {
        return None;
    }
    clause_features(host)
}

deckmaste_constructions_macro::constructions! {
    group attachment;

    element restriction_run_member bind RestrictionCoordination {
        comma: surface lex Comma,
        conjunction: opt lex Conjunction,
        member: hole RestrictionMember,
    }

    construction clause_adverb_before: Clause {
        bind Clause via make_clause_adverb_before, clause_adverb_before_parts {
            adverb: identity Vocab via Adverb,
            host: hole Clause,
        }
        derive features: Features = clause_features(host);
        form only @ 0 inverse check(is_clause_adverb_before) = identity(adverb) host;
        selection unique;
    }

    construction clause_sentence_adverbial_before: Clause {
        bind Clause via make_clause_sentence_adverbial_before, clause_sentence_adverbial_before_parts {
            adverb: identity Vocab via SentenceAdverbial,
            host: hole Clause,
        }
        derive features: Features = clause_features(host);
        form only @ 0 inverse check(is_clause_sentence_adverbial_before) = identity(adverb) "," host;
        selection unique;
    }

    construction clause_prepositional_before: Clause {
        bind Clause via make_clause_prepositional_before, clause_prepositional_before_parts {
            preposition: hole PrepositionalPhrase,
            host: hole Clause,
        }
        derive features: Features = clause_features(host);
        form only @ 0 inverse check(is_clause_prepositional_before) = preposition "," host;
        selection unique;
    }

    construction clause_subordinate_before: Clause {
        bind Clause via make_clause_subordinate_before, clause_subordinate_before_parts {
            subordinator: identity Subordinator via Subordinator,
            condition: hole Clause,
            host: hole Clause,
        }
        derive features: Features = conditional_features(subordinator, condition, host);
        evidence feature "finite subordinate selection and host eligibility" from category;
        form only @ 0 inverse check(is_clause_subordinate_before) = identity(subordinator) condition "," host;
        selection unique;
    }

    construction clause_subordinate_gerund_before: Clause {
        bind Clause via make_clause_subordinate_gerund_before, clause_subordinate_gerund_before_parts {
            gerund: hole GerundClause,
            host: hole Clause,
        }
        derive features: Features = clause_features(host);
        evidence feature "while-only complete gerund attachment" from category;
        form only @ 0 inverse check(is_clause_subordinate_gerund_before) = "while" gerund "," host;
        selection unique;
    }

    construction clause_subordinate_after_elliptical: Clause {
        bind Clause via make_clause_subordinate_after_elliptical, clause_subordinate_after_elliptical_parts {
            host: hole Clause,
            subordinator: identity Subordinator via Subordinator,
            adjective: hole AdjectivePhrase,
        }
        derive features: Features = clause_features(host);
        form only @ 0 inverse check(is_clause_subordinate_after_elliptical) = host identity(subordinator) adjective;
        selection unique;
    }

    construction clause_subordinate_after: Clause {
        bind Clause via make_clause_subordinate_after, clause_subordinate_after_parts {
            host: hole Clause,
            subordinator: identity Subordinator via Subordinator,
            condition: hole Clause,
        }
        derive features: Features = conditional_features(subordinator, condition, host);
        evidence feature "finite subordinate selection and host eligibility" from category;
        form only @ 0 inverse check(is_clause_subordinate_after) = host identity(subordinator) condition;
        selection unique;
    }

    construction clause_subordinate_after_comma: Clause {
        bind Clause via make_clause_subordinate_after_comma, clause_subordinate_after_comma_parts {
            host: hole Clause,
            subordinator: identity Subordinator via Subordinator,
            condition: hole Clause,
        }
        derive features: Features = conditional_features(subordinator, condition, host);
        evidence feature "finite subordinate selection and host eligibility" from category;
        form only @ 0 inverse check(is_clause_subordinate_after_comma) = host "," identity(subordinator) condition;
        selection unique;
    }

    construction clause_subordinate_after_infinitive: Clause {
        bind Clause via make_clause_subordinate_after_infinitive, clause_subordinate_after_infinitive_parts {
            host: hole Clause,
            predicate: hole VerbPhrase,
        }
        derive features: Features = reduce_infinitive_attachment_features(host, predicate);
        evidence feature "complete bare rather-than infinitive" from category;
        form only @ 0 inverse check(is_clause_subordinate_after_infinitive) = host "rather" "than" predicate;
        selection unique;
    }

    construction exception_rider_single: ExceptionRider {
        bind ExceptionRider via make_exception_rider_single, exception_rider_single_parts {
            clause: hole Clause,
        }
        derive features: Features = reduce_exception_single_features(clause);
        form only @ 0 inverse check(is_exception_rider_single) = "except" clause;
        selection unique;
    }

    construction exception_rider_conjoined: ExceptionRider {
        bind ExceptionRider via make_exception_rider_conjoined, exception_rider_conjoined_parts {
            rider: hole ExceptionRider,
            conjunction: lex Conjunction,
            clause: hole Clause,
        }
        derive features: Features = reduce_exception_conjoined_features(rider, conjunction, clause);
        form only @ 0 inverse check(is_exception_rider_conjoined) = rider lex(conjunction) clause;
        selection unique;
    }

    construction exception_rider_comma: ExceptionRiderList {
        bind ExceptionRiderList via make_exception_rider_comma, exception_rider_comma_parts {
            rider: opt hole ExceptionRider,
            list: opt hole ExceptionRiderList,
            clause: hole Clause,
        }
        require any(
            all(rider.is_some(), list.is_none()),
            all(rider.is_none(), list.is_some())
        );
        derive features: Features = reduce_exception_comma_features(rider, list, clause);
        form seed @ 0 when all(rider.is_some(), list.is_none()) inverse check(is_exception_rider_comma) = rider "," clause;
        form extend @ 1 when all(rider.is_none(), list.is_some()) inverse check(is_exception_rider_comma) = list "," clause;
        selection unique;
    }

    construction exception_rider_oxford: ExceptionRider {
        bind ExceptionRider via make_exception_rider_oxford, exception_rider_oxford_parts {
            rider: hole ExceptionRiderList,
            conjunction: lex Conjunction,
            clause: hole Clause,
        }
        derive features: Features = reduce_exception_conjoined_features(rider, conjunction, clause);
        form only @ 0 inverse check(is_exception_rider_oxford) = rider "," lex(conjunction) clause;
        selection unique;
    }

    construction clause_excepted: Clause {
        bind Clause via make_clause_excepted, clause_excepted_parts {
            host: hole Clause,
            rider: opt hole ExceptionRider,
            list: opt hole ExceptionRiderList,
        }
        require any(
            all(rider.is_some(), list.is_none()),
            all(rider.is_none(), list.is_some())
        );
        derive features: Features = clause_features(host);
        derive reading_dispreference: Features = reduce_open_exception_dispreference(rider, list);
        form complete @ 0 when all(rider.is_some(), list.is_none()) inverse check(is_clause_excepted) = host "," rider;
        form open @ 1 when all(rider.is_none(), list.is_some()) inverse check(is_clause_excepted) = host "," list;
        selection unique;
    }

    construction clause_restriction_member: RestrictionMember {
        bind RestrictionMember via make_clause_restriction_member, clause_restriction_member_parts {
            preposition: opt hole PrepositionalPhrase,
            condition: opt hole Clause,
            temporal: opt hole NounPhrase,
        }
        require any(
            all(preposition.is_some(), condition.is_none(), temporal.is_none()),
            all(preposition.is_none(), condition.is_some(), temporal.is_none()),
            all(preposition.is_none(), condition.is_none())
        );
        derive features: Features = reduce_restriction_member_features(preposition, condition, temporal);
        form preposition @ 0 when all(preposition.is_some(), condition.is_none(), temporal.is_none()) inverse check(is_restriction_preposition) = "only" preposition;
        form condition @ 1 when all(preposition.is_none(), condition.is_some(), temporal.is_none()) inverse check(is_restriction_condition) = "only" "if" condition;
        form once @ 2 when all(preposition.is_none(), condition.is_none(), temporal.is_none()) inverse check(is_restriction_once) = "only" "once";
        form once_temporal @ 3 when all(preposition.is_none(), condition.is_none(), temporal.is_some()) inverse check(is_restriction_once_temporal) = "only" "once" temporal;
        selection unique;
    }

    construction clause_restriction_run: Clause {
        bind Clause via make_clause_restriction_run, clause_restriction_run_parts {
            host: hole Clause,
            first: hole RestrictionMember,
            rest: seq restriction_run_member,
        }
        require rest.len() >= 1;
        require rest.nonfinal.conjunction.is_none();
        require rest.last.conjunction.is_some();
        require rest.last.conjunction in [And];
        derive features: Features = reduce_restriction_run_features(host, first, rest);
        form only @ 0 inverse check(is_clause_restriction_run) = host first rest;
        selection unique;
    }
}

pub(crate) static GROUPS: &[&GroupData] = &[&ATTACHMENT_DECLARATION];

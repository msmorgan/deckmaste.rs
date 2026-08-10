//! Compiler-derived infinitive and gerund clause declarations.

#![allow(
    clippy::needless_pass_by_value,
    dead_code,
    reason = "declaration adapters own erased builder values and uniform fallible signatures"
)]

use deckmaste_construction_compiler::runtime::DeclarationViolation;
use deckmaste_construction_compiler::runtime::GroupData;

use crate::features::Comma;
use crate::grammar::Features;
use crate::grammar::PredicateForm;
use crate::grammar::VerbPhrase;
use crate::syntax::AttachmentPosition;
use crate::syntax::DependentAttachment;
use crate::syntax::DependentClause;
use crate::syntax::GerundClause;
use crate::syntax::InfinitiveClause;
use crate::syntax::InfinitiveMarker;
use crate::syntax::Predicate;
use crate::syntax::SubordinateBody;
use crate::syntax::Subordinator;

fn violation(construction: &'static str, requirement: &'static str) -> DeclarationViolation {
    DeclarationViolation {
        construction,
        requirement,
    }
}

fn predicate_has_complete_form(predicate: &Predicate, required: PredicateForm) -> bool {
    let Ok(predicate) = crate::constructions::predicate::inverse_public_predicate(predicate) else {
        return false;
    };
    matches!(
        predicate.declaration_core_features(),
        Some(Features::VerbPhrase { form, .. }) if form == required
    ) && predicate.declaration_core_arguments_complete()
}

fn staged_predicate_has_complete_form(predicate: &VerbPhrase, required: PredicateForm) -> bool {
    matches!(
        predicate.declaration_core_features(),
        Some(Features::VerbPhrase { form, .. }) if form == required
    ) && predicate.declaration_core_arguments_complete()
}

fn checked_predicate(
    construction: &'static str,
    predicate: VerbPhrase,
    required: PredicateForm,
) -> Result<Predicate, DeclarationViolation> {
    if !staged_predicate_has_complete_form(&predicate, required) {
        return Err(violation(
            construction,
            "the predicate has the declared nonfinite form and complete lexical valency",
        ));
    }
    let crate::constructions::predicate::FinishedPredicate {
        modal: None,
        predicate,
        ..
    } = crate::constructions::predicate::project_public_predicate(predicate)?
    else {
        return Err(violation(
            construction,
            "a nonfinite predicate has no finite modal projection",
        ));
    };
    Ok(predicate)
}

fn make_infinitive_to(predicate: VerbPhrase) -> Result<InfinitiveClause, DeclarationViolation> {
    let predicate = checked_predicate("infinitive_to", predicate, PredicateForm::Infinitive)?;
    Ok(InfinitiveClause::from_declaration_parts(
        false,
        InfinitiveMarker::To,
        predicate,
    ))
}

fn infinitive_to_parts(value: &InfinitiveClause) -> VerbPhrase {
    crate::constructions::predicate::inverse_public_predicate(value.predicate())
        .expect("the infinitive recognizer admits an invertible predicate")
}

fn is_infinitive_to(value: &InfinitiveClause) -> bool {
    !value.negated()
        && value.marker() == InfinitiveMarker::To
        && predicate_has_complete_form(value.predicate(), PredicateForm::Infinitive)
}

fn make_infinitive_not_to(predicate: VerbPhrase) -> Result<InfinitiveClause, DeclarationViolation> {
    let predicate = checked_predicate("infinitive_not_to", predicate, PredicateForm::Infinitive)?;
    Ok(InfinitiveClause::from_declaration_parts(
        true,
        InfinitiveMarker::To,
        predicate,
    ))
}

fn infinitive_not_to_parts(value: &InfinitiveClause) -> VerbPhrase {
    crate::constructions::predicate::inverse_public_predicate(value.predicate())
        .expect("the infinitive recognizer admits an invertible predicate")
}

fn is_infinitive_not_to(value: &InfinitiveClause) -> bool {
    value.negated()
        && value.marker() == InfinitiveMarker::To
        && predicate_has_complete_form(value.predicate(), PredicateForm::Infinitive)
}

pub(crate) fn is_valid_infinitive(value: &InfinitiveClause) -> bool {
    matches!(
        (value.negated(), value.marker()),
        (false, InfinitiveMarker::Bare)
            | (false, InfinitiveMarker::To)
            | (true, InfinitiveMarker::To)
    ) && predicate_has_complete_form(value.predicate(), PredicateForm::Infinitive)
}

fn valid_gerund(value: &GerundClause) -> bool {
    predicate_has_complete_form(value.predicate(), PredicateForm::PresentParticiple)
        && value.attachments().iter().all(|attachment| {
            matches!(
                attachment,
                DependentAttachment {
                    position: AttachmentPosition::AfterMatrix,
                    comma: Comma::Absent,
                    payload: DependentClause::Subordinate(
                        Subordinator::RatherThan,
                        SubordinateBody::Gerund(alternative),
                    ),
                } if valid_gerund(alternative)
            )
        })
}

fn make_gerund_clause_base(predicate: VerbPhrase) -> Result<GerundClause, DeclarationViolation> {
    let predicate = checked_predicate(
        "gerund_clause_base",
        predicate,
        PredicateForm::PresentParticiple,
    )?;
    Ok(GerundClause::from_declaration_parts(predicate, Vec::new()))
}

fn gerund_clause_base_parts(value: &GerundClause) -> VerbPhrase {
    crate::constructions::predicate::inverse_public_predicate(value.predicate())
        .expect("the gerund recognizer admits an invertible predicate")
}

fn is_gerund_clause_base(value: &GerundClause) -> bool {
    value.attachments().is_empty() && valid_gerund(value)
}

fn make_gerund_clause_subordinate_after(
    matrix: GerundClause,
    alternative: GerundClause,
) -> Result<GerundClause, DeclarationViolation> {
    if !valid_gerund(&matrix) || !valid_gerund(&alternative) {
        return Err(violation(
            "gerund_clause_subordinate_after",
            "both sides are complete generated gerund clauses",
        ));
    }
    let (predicate, mut attachments) = matrix.into_declaration_parts();
    attachments.push(DependentAttachment {
        position: AttachmentPosition::AfterMatrix,
        comma: Comma::Absent,
        payload: DependentClause::Subordinate(
            Subordinator::RatherThan,
            SubordinateBody::Gerund(alternative),
        ),
    });
    Ok(GerundClause::from_declaration_parts(predicate, attachments))
}

fn gerund_clause_subordinate_after_parts(value: &GerundClause) -> (GerundClause, GerundClause) {
    let (predicate, mut attachments) = value.clone().into_declaration_parts();
    let DependentAttachment {
        payload:
            DependentClause::Subordinate(Subordinator::RatherThan, SubordinateBody::Gerund(alternative)),
        ..
    } = attachments
        .pop()
        .expect("the subordinate recognizer admits one trailing attachment")
    else {
        unreachable!("the subordinate recognizer admits a rather-than gerund")
    };
    (
        GerundClause::from_declaration_parts(predicate, attachments),
        alternative,
    )
}

fn is_gerund_clause_subordinate_after(value: &GerundClause) -> bool {
    valid_gerund(value)
        && matches!(
            value.attachments().last(),
            Some(DependentAttachment {
                position: AttachmentPosition::AfterMatrix,
                comma: Comma::Absent,
                payload: DependentClause::Subordinate(
                    Subordinator::RatherThan,
                    SubordinateBody::Gerund(_),
                ),
            })
        )
}

fn reduce_infinitive_features(predicate: &Features) -> Option<Features> {
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
    .then_some(Features::InfinitiveClause)
}

fn reduce_gerund_features(predicate: &Features) -> Option<Features> {
    let Features::VerbPhrase {
        form: PredicateForm::PresentParticiple,
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
    .then_some(Features::GerundClause)
}

fn reduce_gerund_attachment_features(
    matrix: &Features,
    alternative: &Features,
) -> Option<Features> {
    (matches!(matrix, Features::GerundClause) && matches!(alternative, Features::GerundClause))
        .then_some(Features::GerundClause)
}

deckmaste_constructions_macro::constructions! {
    group nonfinite;

    construction infinitive_to: InfinitiveClause {
        bind InfinitiveClause via make_infinitive_to, infinitive_to_parts {
            predicate: hole VerbPhrase,
        }
        derive features: Features = reduce_infinitive_features(predicate);
        evidence feature "complete infinitive predicate form and valency" from category;
        form only @ 0 inverse check(is_infinitive_to) = "to" predicate;
        selection unique;
    }

    construction infinitive_not_to: InfinitiveClause {
        bind InfinitiveClause via make_infinitive_not_to, infinitive_not_to_parts {
            predicate: hole VerbPhrase,
        }
        derive features: Features = reduce_infinitive_features(predicate);
        evidence feature "complete infinitive predicate form and valency" from category;
        form only @ 0 inverse check(is_infinitive_not_to) = "not" "to" predicate;
        selection unique;
    }

    construction gerund_clause_base: GerundClause {
        bind GerundClause via make_gerund_clause_base, gerund_clause_base_parts {
            predicate: hole VerbPhrase,
        }
        derive features: Features = reduce_gerund_features(predicate);
        evidence feature "complete present-participle predicate form and valency" from category;
        form only @ 0 inverse check(is_gerund_clause_base) = predicate;
        selection unique;
    }

    construction gerund_clause_subordinate_after: GerundClause {
        bind GerundClause via make_gerund_clause_subordinate_after, gerund_clause_subordinate_after_parts {
            matrix: hole GerundClause,
            alternative: hole GerundClause,
        }
        derive features: Features = reduce_gerund_attachment_features(matrix, alternative);
        evidence feature "typed trailing rather-than gerund attachment" from category;
        form only @ 0 inverse check(is_gerund_clause_subordinate_after) = matrix "rather" "than" alternative;
        selection unique;
    }
}

pub(crate) static GROUPS: &[&GroupData] = &[&NONFINITE_DECLARATION];

//! Compiler-derived declarations for the R01 relative-clause family.

#![allow(
    dead_code,
    clippy::unnecessary_wraps,
    reason = "declaration adapters are reached through generated function pointers and retain the shared checked-builder signature"
)]

use deckmaste_construction_compiler::runtime::DeclarationViolation;
use deckmaste_construction_compiler::runtime::GroupData;

use crate::features::GapState;
use crate::grammar::ContractedSubjectAuxiliary;
use crate::grammar::Features;
use crate::grammar::PredicateForm;
use crate::grammar::PredicateObjectState;
use crate::grammar::VerbPhrase;
use crate::grammar::auxiliary_form;
use crate::grammar::predicate_arguments_complete;
use crate::grammar::predicate_object_gap_complete;
use crate::syntax::AdjectivePhrase;
use crate::syntax::Copula;
use crate::syntax::CopularComplement;
use crate::syntax::CopularPredicate;
use crate::syntax::Demonstrative;
use crate::syntax::DeonticPredicate;
use crate::syntax::NounPhrase;
use crate::syntax::NounPhraseKind;
use crate::syntax::ObjectGapPredicate;
use crate::syntax::Predicate;
use crate::syntax::PrepositionalPhrase;
use crate::syntax::RelativeBody;
use crate::syntax::RelativeClause;
use crate::syntax::RelativeMarker;
use crate::syntax::Subject;
use crate::word::Auxiliary;
use crate::word::Number;

fn violation(construction: &'static str, requirement: &'static str) -> DeclarationViolation {
    DeclarationViolation {
        construction,
        requirement,
    }
}

pub(crate) fn project_predicate_hole(
    phrase: VerbPhrase,
) -> Result<Predicate, DeclarationViolation> {
    if !phrase.declaration_core_arguments_complete() {
        return Err(violation(
            "relative_predicate_hole",
            "the predicate is complete for its selected valency",
        ));
    }
    let finished = crate::constructions::predicate::project_public_predicate(phrase)?;
    Ok(match finished.modal {
        Some(modal) => Predicate::Deontic(DeonticPredicate {
            modal,
            inner: (!finished.elided).then(|| Box::new(finished.predicate)),
        }),
        None => finished.predicate,
    })
}

pub(crate) fn project_object_gap_predicate_hole(
    phrase: VerbPhrase,
) -> Result<ObjectGapPredicate, DeclarationViolation> {
    let finished = crate::constructions::predicate::project_public_predicate(phrase)?;
    if finished.modal.is_some() {
        return Err(violation(
            "relative_object_gap_predicate_hole",
            "an object-gap predicate has no detached modal",
        ));
    }
    let Predicate::Intransitive(predicate) = finished.predicate else {
        return Err(violation(
            "relative_object_gap_predicate_hole",
            "the omitted direct object projects as an object gap",
        ));
    };
    Ok(ObjectGapPredicate {
        head: predicate.head,
        kind: crate::syntax::ObjectGap,
        elements: predicate.elements,
    })
}

fn object_gap_parts(predicate: &ObjectGapPredicate) -> Result<VerbPhrase, DeclarationViolation> {
    crate::constructions::predicate::inverse_public_object_gap_predicate(predicate)
}

fn predicate_parts(predicate: &Predicate) -> Result<VerbPhrase, DeclarationViolation> {
    crate::constructions::predicate::inverse_public_predicate(predicate)
}

fn make_relative_object(
    subject: NounPhrase,
    predicate: ObjectGapPredicate,
) -> Result<RelativeClause, DeclarationViolation> {
    Ok(RelativeClause {
        marker: RelativeMarker::Zero,
        gap: GapState::Object,
        body: RelativeBody::ObjectGap {
            subject: Subject(subject),
            predicate,
        },
    })
}

fn relative_object_parts(value: &RelativeClause) -> (NounPhrase, ObjectGapPredicate) {
    let RelativeBody::ObjectGap { subject, predicate } = &value.body else {
        unreachable!("relative_object recognizer admits only object gaps")
    };
    (subject.0.clone(), predicate.clone())
}

fn is_relative_object(value: &RelativeClause) -> bool {
    value.marker == RelativeMarker::Zero
        && value.gap == GapState::Object
        && matches!(value.body, RelativeBody::ObjectGap { .. })
        && !object_gap_contracted(value)
}

fn make_relative_object_contracted_subject(
    subject_auxiliary: ContractedSubjectAuxiliary,
    predicate: ObjectGapPredicate,
) -> Result<RelativeClause, DeclarationViolation> {
    let predicate = object_gap_parts(&predicate)?
        .declaration_with_contracted_subject_auxiliary(subject_auxiliary.auxiliary)
        .ok_or_else(|| {
            violation(
                "relative_object_contracted_subject",
                "subject contraction is represented exactly once",
            )
        })?;
    Ok(RelativeClause {
        marker: RelativeMarker::Zero,
        gap: GapState::Object,
        body: RelativeBody::ObjectGap {
            subject: subject_auxiliary.subject,
            predicate: project_object_gap_predicate_hole(predicate)?,
        },
    })
}

fn relative_object_contracted_subject_parts(
    value: &RelativeClause,
) -> (ContractedSubjectAuxiliary, ObjectGapPredicate) {
    let RelativeBody::ObjectGap { subject, predicate } = &value.body else {
        unreachable!("contracted object-relative recognizer admits only object gaps")
    };
    let (auxiliary, predicate) = object_gap_parts(predicate)
        .and_then(|predicate| {
            predicate
                .declaration_contracted_subject_auxiliary_parts()
                .ok_or_else(|| {
                    violation(
                        "relative_object_contracted_subject",
                        "the object-gap predicate retains its contracted auxiliary",
                    )
                })
        })
        .expect("recognizer retains a contracted object-gap auxiliary");
    (
        ContractedSubjectAuxiliary {
            subject: subject.clone(),
            auxiliary,
        },
        project_object_gap_predicate_hole(predicate)
            .expect("stripping contraction leaves an object-gap predicate"),
    )
}

fn object_gap_contracted(value: &RelativeClause) -> bool {
    let RelativeBody::ObjectGap { predicate, .. } = &value.body else {
        return false;
    };
    object_gap_parts(predicate).is_ok_and(|predicate| {
        predicate
            .declaration_contracted_subject_auxiliary_parts()
            .is_some()
    })
}

fn is_relative_object_contracted_subject(value: &RelativeClause) -> bool {
    value.marker == RelativeMarker::Zero
        && value.gap == GapState::Object
        && object_gap_contracted(value)
}

fn make_relative_subject_contracted_auxiliary(
    subject_auxiliary: ContractedSubjectAuxiliary,
    predicate: Predicate,
) -> Result<RelativeClause, DeclarationViolation> {
    if !subject_is_demonstrative_that(&subject_auxiliary.subject) {
        return Err(violation(
            "relative_subject_contracted_auxiliary",
            "the contracted relative subject is demonstrative that",
        ));
    }
    let predicate = predicate_parts(&predicate)?
        .declaration_with_contracted_subject_auxiliary(subject_auxiliary.auxiliary)
        .ok_or_else(|| {
            violation(
                "relative_subject_contracted_auxiliary",
                "subject contraction is represented exactly once",
            )
        })?;
    Ok(subject_relative(
        RelativeMarker::That,
        project_predicate_hole(predicate)?,
    ))
}

fn relative_subject_contracted_auxiliary_parts(
    value: &RelativeClause,
) -> (ContractedSubjectAuxiliary, Predicate) {
    let RelativeBody::SubjectGap(predicate) = &value.body else {
        unreachable!("contracted subject-relative recognizer admits only subject gaps")
    };
    let (auxiliary, predicate) = predicate_parts(predicate)
        .and_then(|predicate| {
            predicate
                .declaration_contracted_subject_auxiliary_parts()
                .ok_or_else(|| {
                    violation(
                        "relative_subject_contracted_auxiliary",
                        "the subject-gap predicate retains its contracted auxiliary",
                    )
                })
        })
        .expect("recognizer retains a contracted subject-gap auxiliary");
    (
        ContractedSubjectAuxiliary {
            subject: Subject(NounPhrase::from_demonstrative_declaration(
                Demonstrative::That,
            )),
            auxiliary,
        },
        project_predicate_hole(predicate)
            .expect("stripping contraction leaves a complete predicate"),
    )
}

fn is_relative_subject_contracted_auxiliary(value: &RelativeClause) -> bool {
    value.marker == RelativeMarker::That
        && value.gap == GapState::Subject
        && matches!(&value.body, RelativeBody::SubjectGap(predicate)
            if predicate_parts(predicate).is_ok_and(|predicate| predicate
                .declaration_contracted_subject_auxiliary_parts().is_some()))
}

fn make_relative_subject(
    marker: RelativeMarker,
    predicate: Predicate,
) -> Result<RelativeClause, DeclarationViolation> {
    if marker == RelativeMarker::Zero {
        return Err(violation(
            "relative_subject",
            "a subject-gap relative has an explicit marker",
        ));
    }
    Ok(subject_relative(marker, predicate))
}

fn relative_subject_parts(value: &RelativeClause) -> (RelativeMarker, Predicate) {
    let RelativeBody::SubjectGap(predicate) = &value.body else {
        unreachable!("subject-relative recognizer admits only subject gaps")
    };
    (value.marker, predicate.clone())
}

fn is_relative_subject(value: &RelativeClause) -> bool {
    value.marker != RelativeMarker::Zero
        && value.gap == GapState::Subject
        && matches!(&value.body, RelativeBody::SubjectGap(predicate)
        if predicate_parts(predicate).is_ok_and(|predicate| {
            !predicate.declaration_has_distributive_each()
                && predicate.declaration_contracted_subject_auxiliary_parts().is_none()
        }))
}

fn make_relative_subject_distributive_each(
    marker: RelativeMarker,
    predicate: Predicate,
) -> Result<RelativeClause, DeclarationViolation> {
    if marker == RelativeMarker::Zero {
        return Err(violation(
            "relative_subject_distributive_each",
            "a distributive subject relative has an explicit marker",
        ));
    }
    let predicate = predicate_parts(&predicate)?
        .declaration_with_distributive_each()
        .ok_or_else(|| {
            violation(
                "relative_subject_distributive_each",
                "distributive each is represented exactly once",
            )
        })?;
    Ok(subject_relative(marker, project_predicate_hole(predicate)?))
}

fn relative_subject_distributive_each_parts(value: &RelativeClause) -> (RelativeMarker, Predicate) {
    let RelativeBody::SubjectGap(predicate) = &value.body else {
        unreachable!("distributive subject-relative recognizer admits only subject gaps")
    };
    let predicate = predicate_parts(predicate)
        .and_then(|predicate| {
            predicate
                .declaration_without_distributive_each()
                .ok_or_else(|| {
                    violation(
                        "relative_subject_distributive_each",
                        "the predicate retains distributive each",
                    )
                })
        })
        .and_then(project_predicate_hole)
        .expect("recognizer retains one distributive each marker");
    (value.marker, predicate)
}

fn is_relative_subject_distributive_each(value: &RelativeClause) -> bool {
    value.marker != RelativeMarker::Zero
        && value.gap == GapState::Subject
        && matches!(&value.body, RelativeBody::SubjectGap(predicate)
            if predicate_parts(predicate).is_ok_and(|predicate| predicate.declaration_has_distributive_each()))
}

fn subject_relative(marker: RelativeMarker, predicate: Predicate) -> RelativeClause {
    RelativeClause {
        marker,
        gap: GapState::Subject,
        body: RelativeBody::SubjectGap(predicate),
    }
}

fn subject_is_demonstrative_that(subject: &Subject) -> bool {
    matches!(
        subject.0.kind(),
        NounPhraseKind::Demonstrative(Demonstrative::That)
    )
}

fn make_contracted_copular(
    construction: &'static str,
    subject_auxiliary: ContractedSubjectAuxiliary,
    complement: CopularComplement,
) -> Result<RelativeClause, DeclarationViolation> {
    if !subject_is_demonstrative_that(&subject_auxiliary.subject)
        || subject_auxiliary.auxiliary.auxiliary != Auxiliary::Be
    {
        return Err(violation(
            construction,
            "contracted demonstrative that agrees with a be copula",
        ));
    }
    Ok(subject_relative(
        RelativeMarker::That,
        Predicate::Copular(CopularPredicate {
            negated: false,
            copula: Copula {
                auxiliary: subject_auxiliary.auxiliary,
                contracted_with_subject: crate::features::Contraction::Contracted,
            },
            distributive_each: false,
            precomplement_adverbs: vec![],
            complement,
            adjuncts: vec![],
        }),
    ))
}

fn contracted_copular_parts(
    value: &RelativeClause,
) -> (ContractedSubjectAuxiliary, &CopularComplement) {
    let RelativeBody::SubjectGap(Predicate::Copular(predicate)) = &value.body else {
        unreachable!("contracted copular recognizer admits only copular subject gaps")
    };
    (
        ContractedSubjectAuxiliary {
            subject: Subject(NounPhrase::from_demonstrative_declaration(
                Demonstrative::That,
            )),
            auxiliary: predicate.copula.auxiliary,
        },
        &predicate.complement,
    )
}

fn is_contracted_copular(value: &RelativeClause) -> bool {
    value.marker == RelativeMarker::That
        && value.gap == GapState::Subject
        && matches!(&value.body, RelativeBody::SubjectGap(Predicate::Copular(predicate))
            if !predicate.negated
                && predicate.copula.auxiliary.auxiliary == Auxiliary::Be
                && predicate.copula.contracted_with_subject.is_contracted()
                && !predicate.distributive_each
                && predicate.precomplement_adverbs.is_empty()
                && predicate.adjuncts.is_empty())
}

macro_rules! contracted_copular_adapter {
    ($make:ident, $parts:ident, $is:ident, $variant:ident, $ty:ty, $id:literal) => {
        fn $make(
            subject_auxiliary: ContractedSubjectAuxiliary,
            complement: $ty,
        ) -> Result<RelativeClause, DeclarationViolation> {
            make_contracted_copular(
                $id,
                subject_auxiliary,
                CopularComplement::$variant(complement),
            )
        }

        fn $parts(value: &RelativeClause) -> (ContractedSubjectAuxiliary, $ty) {
            let (subject_auxiliary, complement) = contracted_copular_parts(value);
            let CopularComplement::$variant(complement) = complement else {
                unreachable!("contracted copular recognizer checks its complement class")
            };
            (subject_auxiliary, complement.clone())
        }

        fn $is(value: &RelativeClause) -> bool {
            is_contracted_copular(value)
                && matches!(
                    contracted_copular_parts(value).1,
                    CopularComplement::$variant(_)
                )
        }
    };
}

contracted_copular_adapter!(
    make_relative_contracted_copular_noun,
    relative_contracted_copular_noun_parts,
    is_relative_contracted_copular_noun,
    NounPhrase,
    NounPhrase,
    "relative_contracted_copular_noun"
);
contracted_copular_adapter!(
    make_relative_contracted_copular_adjective,
    relative_contracted_copular_adjective_parts,
    is_relative_contracted_copular_adjective,
    Adjective,
    AdjectivePhrase,
    "relative_contracted_copular_adjective"
);
contracted_copular_adapter!(
    make_relative_contracted_copular_prepositional,
    relative_contracted_copular_prepositional_parts,
    is_relative_contracted_copular_prepositional,
    Prepositional,
    PrepositionalPhrase,
    "relative_contracted_copular_prepositional"
);

fn relative_features(
    gap: GapState,
    marker: RelativeMarker,
    antecedent_agreement: Option<crate::grammar::Agreement>,
    object_gap_requires_rules_object: bool,
    bare_copular_tail: bool,
) -> Features {
    Features::RelativeClause {
        gap,
        marker,
        antecedent_agreement,
        object_gap_requires_rules_object,
        bare_copular_tail,
    }
}

fn reduce_relative_object_features(subject: &Features, predicate: &Features) -> Option<Features> {
    let Features::NounPhrase {
        agreement: Some(subject_agreement),
        pronoun_case,
        ..
    } = subject
    else {
        return None;
    };
    if *pronoun_case == Some(crate::word::PronounCase::Object) {
        return None;
    }
    let Features::VerbPhrase {
        form: PredicateForm::Finite(predicate_agreement),
        object: PredicateObjectState::None,
        indirect_object,
        selected_preposition,
        frame,
        bare,
        object_gap_requires_rules_object,
        ..
    } = predicate
    else {
        return None;
    };
    ((!(*bare && frame.is_proform()))
        && predicate_object_gap_complete(*frame, *indirect_object, *selected_preposition)
        && predicate_agreement.is_none_or(|agreement| agreement == *subject_agreement))
    .then(|| {
        relative_features(
            GapState::Object,
            RelativeMarker::Zero,
            None,
            *object_gap_requires_rules_object,
            false,
        )
    })
}

fn reduce_relative_object_contracted_subject_features(
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
        form,
        object: PredicateObjectState::None,
        indirect_object,
        selected_preposition,
        frame,
        bare,
        object_gap_requires_rules_object,
        ..
    } = predicate
    else {
        return None;
    };
    let PredicateForm::Finite(Some(predicate_agreement)) = auxiliary_form(*auxiliary, *form)?
    else {
        return None;
    };
    ((!(*bare && frame.is_proform()))
        && predicate_object_gap_complete(*frame, *indirect_object, *selected_preposition)
        && predicate_agreement == *subject_agreement)
        .then(|| {
            relative_features(
                GapState::Object,
                RelativeMarker::Zero,
                None,
                *object_gap_requires_rules_object,
                false,
            )
        })
}

fn reduce_relative_subject_contracted_auxiliary_features(
    subject_auxiliary: &Features,
    predicate: &Features,
) -> Option<Features> {
    let Features::SubjectAuxiliary {
        subject: crate::grammar::ContractedSubjectKey::Demonstrative(Demonstrative::That),
        agreement,
        auxiliary,
    } = subject_auxiliary
    else {
        return None;
    };
    let Features::VerbPhrase { form, .. } = predicate else {
        return None;
    };
    let PredicateForm::Finite(Some(predicate_agreement)) = auxiliary_form(*auxiliary, *form)?
    else {
        return None;
    };
    (predicate_agreement == *agreement).then(|| {
        relative_features(
            GapState::Subject,
            RelativeMarker::That,
            Some(predicate_agreement),
            false,
            false,
        )
    })
}

fn explicit_marker(marker: &Features) -> Option<RelativeMarker> {
    let Features::RelativeMarker(marker) = marker else {
        return None;
    };
    (*marker != RelativeMarker::Zero).then_some(*marker)
}

fn reduce_subject_predicate(
    marker: RelativeMarker,
    predicate: &Features,
    distributive_each: bool,
) -> Option<Features> {
    let Features::VerbPhrase {
        form: PredicateForm::Finite(antecedent_agreement),
        passive,
        object,
        indirect_object,
        selected_preposition,
        frame,
        bare,
        head_is_copular,
        ..
    } = predicate
    else {
        return None;
    };
    if distributive_each
        && !matches!(antecedent_agreement, Some(agreement) if agreement.number == Number::Plural)
    {
        return None;
    }
    predicate_arguments_complete(
        *frame,
        *passive,
        *object,
        *indirect_object,
        *selected_preposition,
    )
    .then(|| {
        relative_features(
            GapState::Subject,
            marker,
            *antecedent_agreement,
            false,
            *bare && *head_is_copular,
        )
    })
}

fn reduce_relative_subject_features(marker: &Features, predicate: &Features) -> Option<Features> {
    reduce_subject_predicate(explicit_marker(marker)?, predicate, false)
}

fn reduce_relative_subject_distributive_each_features(
    marker: &Features,
    predicate: &Features,
) -> Option<Features> {
    reduce_subject_predicate(explicit_marker(marker)?, predicate, true)
}

fn reduce_contracted_copular_features(
    subject_auxiliary: &Features,
    _complement: &Features,
) -> Option<Features> {
    let Features::SubjectAuxiliary {
        subject: crate::grammar::ContractedSubjectKey::Demonstrative(Demonstrative::That),
        agreement,
        auxiliary,
    } = subject_auxiliary
    else {
        return None;
    };
    (auxiliary.auxiliary() == Auxiliary::Be).then(|| {
        relative_features(
            GapState::Subject,
            RelativeMarker::That,
            Some(*agreement),
            false,
            false,
        )
    })
}

deckmaste_constructions_macro::constructions! {
    group relative;

    construction relative_object: RelativeClause {
        bind RelativeClause via make_relative_object, relative_object_parts {
            subject: hole NounPhrase,
            predicate: hole ObjectGapPredicate,
        }
        derive features: Features = reduce_relative_object_features(subject, predicate);
        evidence feature "relative gap, marker, and agreement" from output relative_signature;
        form only @ 0 inverse check(is_relative_object) = subject predicate;
        selection unique;
    }

    construction relative_object_contracted_subject: RelativeClause {
        bind RelativeClause via make_relative_object_contracted_subject, relative_object_contracted_subject_parts {
            subject_auxiliary: identity ContractedSubjectAuxiliary via SubjectAuxiliary,
            predicate: hole ObjectGapPredicate,
        }
        derive features: Features = reduce_relative_object_contracted_subject_features(subject_auxiliary, predicate);
        evidence feature "relative gap, marker, and agreement" from output relative_signature;
        form only @ 0 inverse check(is_relative_object_contracted_subject) = identity(subject_auxiliary) predicate;
        selection unique;
    }

    construction relative_subject_contracted_auxiliary: RelativeClause {
        bind RelativeClause via make_relative_subject_contracted_auxiliary, relative_subject_contracted_auxiliary_parts {
            subject_auxiliary: identity ContractedSubjectAuxiliary via SubjectAuxiliary,
            predicate: hole Predicate,
        }
        derive features: Features = reduce_relative_subject_contracted_auxiliary_features(subject_auxiliary, predicate);
        evidence feature "relative gap, marker, and agreement" from output relative_signature;
        form only @ 0 inverse check(is_relative_subject_contracted_auxiliary) = identity(subject_auxiliary) predicate;
        selection unique;
    }

    construction relative_subject: RelativeClause {
        bind RelativeClause via make_relative_subject, relative_subject_parts {
            marker: identity RelativeMarker via RelativeMarker,
            predicate: hole Predicate,
        }
        derive features: Features = reduce_relative_subject_features(marker, predicate);
        evidence feature "relative gap, marker, and agreement" from output relative_signature;
        form only @ 0 inverse check(is_relative_subject) = identity(marker) predicate;
        dominates relative_object;
        selection unique;
    }

    construction relative_subject_distributive_each: RelativeClause {
        bind RelativeClause via make_relative_subject_distributive_each, relative_subject_distributive_each_parts {
            marker: identity RelativeMarker via RelativeMarker,
            predicate: hole Predicate,
        }
        derive features: Features = reduce_relative_subject_distributive_each_features(marker, predicate);
        evidence feature "relative gap, marker, and agreement" from output relative_signature;
        form only @ 0 inverse check(is_relative_subject_distributive_each) = identity(marker) "each" predicate;
        selection unique;
    }

    construction relative_contracted_copular_noun: RelativeClause {
        bind RelativeClause via make_relative_contracted_copular_noun, relative_contracted_copular_noun_parts {
            subject_auxiliary: identity ContractedSubjectAuxiliary via SubjectAuxiliary,
            complement: hole NounPhrase,
        }
        derive features: Features = reduce_contracted_copular_features(subject_auxiliary, complement);
        evidence feature "relative gap, marker, and agreement" from output relative_signature;
        form only @ 0 inverse check(is_relative_contracted_copular_noun) = identity(subject_auxiliary) complement;
        selection unique;
    }

    construction relative_contracted_copular_adjective: RelativeClause {
        bind RelativeClause via make_relative_contracted_copular_adjective, relative_contracted_copular_adjective_parts {
            subject_auxiliary: identity ContractedSubjectAuxiliary via SubjectAuxiliary,
            complement: hole AdjectivePhrase,
        }
        derive features: Features = reduce_contracted_copular_features(subject_auxiliary, complement);
        evidence feature "relative gap, marker, and agreement" from output relative_signature;
        form only @ 0 inverse check(is_relative_contracted_copular_adjective) = identity(subject_auxiliary) complement;
        selection unique;
    }

    construction relative_contracted_copular_prepositional: RelativeClause {
        bind RelativeClause via make_relative_contracted_copular_prepositional, relative_contracted_copular_prepositional_parts {
            subject_auxiliary: identity ContractedSubjectAuxiliary via SubjectAuxiliary,
            complement: hole PrepositionalPhrase,
        }
        derive features: Features = reduce_contracted_copular_features(subject_auxiliary, complement);
        evidence feature "relative gap, marker, and agreement" from output relative_signature;
        form only @ 0 inverse check(is_relative_contracted_copular_prepositional) = identity(subject_auxiliary) complement;
        selection unique;
    }
}

pub(crate) static GROUPS: &[&GroupData] = &[&RELATIVE_DECLARATION];

//! Compiler-derived finite, existential, and copular clause declarations.

#![allow(
    clippy::needless_pass_by_value,
    clippy::unnecessary_wraps,
    dead_code,
    reason = "declaration adapters own erased builder values and uniform fallible signatures"
)]

use deckmaste_construction_compiler::runtime::DeclarationViolation;
use deckmaste_construction_compiler::runtime::GroupData;

use crate::features::Contraction;
use crate::features::Number;
use crate::features::Person;
use crate::features::PronounCase;
use crate::grammar::ContractedSubjectAuxiliary;
use crate::grammar::CopulaAgreement;
use crate::grammar::CopularRemainder;
use crate::grammar::Features;
use crate::grammar::PredicateForm;
use crate::grammar::SimpleClause;
use crate::grammar::VerbPhrase;
use crate::grammar::auxiliary_form;
use crate::grammar::finish_simple_clause;
use crate::grammar::fold_auxiliary_passive;
use crate::grammar::predicate_arguments_complete;
use crate::syntax::AdjectiveComplement;
use crate::syntax::AdjectivePhrase;
use crate::syntax::Clause;
use crate::syntax::Copula;
use crate::syntax::CopularComplement;
use crate::syntax::CopularPredicate;
use crate::syntax::EllipticalClause;
use crate::syntax::ExistentialClause;
use crate::syntax::ExistentialForm;
use crate::syntax::IndependentClause;
use crate::syntax::Modal;
use crate::syntax::NounPhrase;
use crate::syntax::NumberLiteral;
use crate::syntax::PowerToughness;
use crate::syntax::Predicate;
use crate::syntax::PredicateAdjunct;
use crate::syntax::PrepositionalPhrase;
use crate::syntax::Quantity;
use crate::syntax::QuantityKind;
use crate::syntax::Subject;
use crate::word::Auxiliary;
use crate::word::AuxiliaryInflection;
use crate::word::AuxiliaryInstance;
use crate::word::Vocab;

fn violation(construction: &'static str, requirement: &'static str) -> DeclarationViolation {
    DeclarationViolation {
        construction,
        requirement,
    }
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
    value.subject.is_some()
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
    value.subject.is_some() && value.predicate.declaration_has_distributive_each()
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
    value.subject.is_some()
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
    })
}

fn simple_clause_subjectless_parts(value: &SimpleClause) -> VerbPhrase {
    value.predicate.clone()
}

const fn is_simple_clause_subjectless(value: &SimpleClause) -> bool {
    value.subject.is_none()
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
    let Clause::Independent(value) = value else {
        return None;
    };
    let (subject, predicate) = match value {
        IndependentClause::Transitive(subject, predicate) => (
            Some(subject.clone()),
            Predicate::Transitive(predicate.clone()),
        ),
        IndependentClause::Intransitive(subject, predicate) => (
            Some(subject.clone()),
            Predicate::Intransitive(predicate.clone()),
        ),
        IndependentClause::Passive(subject, predicate) => {
            (Some(subject.clone()), Predicate::Passive(predicate.clone()))
        }
        IndependentClause::Proform(subject, predicate) => {
            (Some(subject.clone()), Predicate::Proform(*predicate))
        }
        IndependentClause::Imperative(predicate) => (None, predicate.clone()),
        IndependentClause::Deontic(subject, modal, predicate) => (
            Some(subject.clone()),
            Predicate::Deontic(crate::syntax::DeonticPredicate {
                modal: *modal,
                inner: predicate.clone().map(Box::new),
            }),
        ),
        IndependentClause::Copular(..)
        | IndependentClause::Predicated(..)
        | IndependentClause::Existential(..)
        | IndependentClause::Complex(..)
        | IndependentClause::Coordinated(..) => return None,
    };
    let predicate = crate::constructions::predicate::inverse_public_predicate(&predicate).ok()?;
    Some(SimpleClause { subject, predicate })
}

fn clause_simple_parts(value: &Clause) -> SimpleClause {
    inverse_simple_clause(value).expect("clause_simple admits its generated independent shapes")
}

fn is_clause_simple(value: &Clause) -> bool {
    inverse_simple_clause(value).is_some()
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
    Ok(Clause::Independent(IndependentClause::Existential(
        ExistentialClause {
            form,
            pivot,
            adjuncts: Vec::new(),
        },
    )))
}

fn clause_existential_parts(value: &Clause) -> (ExistentialForm, NounPhrase) {
    let Clause::Independent(IndependentClause::Existential(value)) = value else {
        unreachable!("clause_existential admits only existential clauses")
    };
    (value.form, value.pivot.clone())
}

fn is_clause_existential(value: &Clause) -> bool {
    matches!(
        value,
        Clause::Independent(IndependentClause::Existential(ExistentialClause {
            adjuncts,
            ..
        })) if adjuncts.is_empty()
    )
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
        .try_attach_compatibility_complement(AdjectiveComplement::Prepositional(standard))
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
        .try_split_trailing_prepositional()
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
                if adjective.clone().try_split_trailing_prepositional().is_some()
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
    Ok(Clause::Independent(IndependentClause::Copular(
        Subject(subject),
        CopularPredicate {
            copula: Copula {
                auxiliary,
                contracted_with_subject: Contraction::Full,
            },
            negated: remainder.negated,
            distributive_each: remainder.distributive_each,
            precomplement_adverbs: remainder.precomplement_adverbs,
            complement: remainder.complement,
            adjuncts: remainder.adjuncts,
        },
    )))
}

fn clause_copular_parts(value: &Clause) -> (NounPhrase, AuxiliaryInstance, CopularRemainder) {
    let Clause::Independent(IndependentClause::Copular(subject, predicate)) = value else {
        unreachable!("clause_copular admits only copular clauses")
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
        Clause::Independent(IndependentClause::Copular(
            _,
            CopularPredicate {
                copula: Copula {
                    contracted_with_subject: Contraction::Full,
                    ..
                },
                ..
            }
        ))
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
    Ok(Clause::Independent(IndependentClause::Copular(
        subject_auxiliary.subject,
        CopularPredicate {
            copula: Copula {
                auxiliary: subject_auxiliary.auxiliary,
                contracted_with_subject: Contraction::Contracted,
            },
            negated: remainder.negated,
            distributive_each: remainder.distributive_each,
            precomplement_adverbs: remainder.precomplement_adverbs,
            complement: remainder.complement,
            adjuncts: remainder.adjuncts,
        },
    )))
}

fn clause_contracted_copular_parts(
    value: &Clause,
) -> (ContractedSubjectAuxiliary, CopularRemainder) {
    let Clause::Independent(IndependentClause::Copular(subject, predicate)) = value else {
        unreachable!("clause_contracted_copular admits only copular clauses")
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
        Clause::Independent(IndependentClause::Copular(
            _,
            CopularPredicate {
                copula: Copula {
                    contracted_with_subject: Contraction::Contracted,
                    ..
                },
                ..
            }
        ))
    )
}

fn make_clause_variable_value_constraint(
    subject: Quantity,
    modal: AuxiliaryInstance,
    copula: AuxiliaryInstance,
    number: NumberLiteral,
) -> Result<Clause, DeclarationViolation> {
    if subject != Quantity::X {
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
    Ok(Clause::Independent(IndependentClause::Deontic(
        Subject(NounPhrase::from_quantity_declaration(subject)),
        Modal { auxiliary: modal },
        Some(Predicate::Copular(CopularPredicate {
            copula: Copula {
                auxiliary: copula,
                contracted_with_subject: Contraction::Full,
            },
            negated: false,
            distributive_each: false,
            precomplement_adverbs: Vec::new(),
            complement: CopularComplement::NounPhrase(NounPhrase::from_quantity_declaration(
                Quantity::Exact(number),
            )),
            adjuncts: Vec::new(),
        })),
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
    let Clause::Independent(IndependentClause::Deontic(
        Subject(subject_phrase),
        Modal { auxiliary: modal },
        Some(Predicate::Copular(predicate)),
    )) = value
    else {
        unreachable!("variable value constraint admits one modal copular shape")
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
    (subject, *modal, predicate.copula.auxiliary, number)
}

fn is_clause_variable_value_constraint(value: &Clause) -> bool {
    let Clause::Independent(IndependentClause::Deontic(
        Subject(subject_phrase),
        Modal { auxiliary: modal },
        Some(Predicate::Copular(CopularPredicate {
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
        })),
    )) = value
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

    construction clause_simple: Clause {
        bind Clause via make_clause_simple, clause_simple_parts {
            simple: hole SimpleClause,
        }
        derive features: Features = reduce_clause_simple_features(simple);
        form only @ 0 inverse check(is_clause_simple) = simple;
        dominates clause_copular;
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
    use crate::construction::ConstructionOwner;
    use crate::grammar::GeneratedActivation;
    use crate::grammar::Nonterminal;

    const IDS: [&str; 18] = [
        "simple_clause_subject",
        "simple_clause_subject_distributive_each",
        "simple_clause_contracted_subject",
        "simple_clause_subjectless",
        "clause_simple",
        "clause_elliptical",
        "clause_existential",
        "copular_remainder_noun",
        "copular_remainder_adjective",
        "copular_remainder_prepositional",
        "copular_remainder_power_toughness",
        "copular_remainder_prepositional_adjunct",
        "copular_remainder_adverb",
        "copular_remainder_negated",
        "copular_remainder_distributive_each",
        "clause_copular",
        "clause_contracted_copular",
        "clause_variable_value_constraint",
    ];

    #[test]
    fn finite_clause_family_has_the_exact_eighteen_id_census_and_dominance() {
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
        assert_eq!(edges, [("clause_simple", "clause_copular")]);
    }

    #[test]
    fn production_fixtures_lower_render_and_report_generated_f02_owners() {
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
                assert_eq!(decision.owner(), ConstructionOwner::Generated);
            }
        }
    }

    #[test]
    fn f02_clause_syntax_and_selection_are_registration_order_neutral() {
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
        let x = Quantity::X;
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

//! Compiler-derived declarations for the English noun-phrase spine.

#![allow(
    dead_code,
    clippy::unnecessary_wraps,
    reason = "declaration adapters share the generated checked-builder ABI"
)]

use deckmaste_construction_compiler::runtime::DeclarationViolation;
use deckmaste_construction_compiler::runtime::GroupData;

use super::nominal::RulesObjectFollowupNominal;
use crate::features::Comma;
use crate::features::Number;
use crate::features::Person;
use crate::grammar::Agreement;
use crate::grammar::CoordinationDomain;
use crate::grammar::Features;
use crate::grammar::NounForm;
use crate::grammar::NounPhraseCoordinationState;
use crate::grammar::QuantityFeatures;
use crate::grammar::SetExceptionState;
use crate::syntax::ArithmeticValue;
use crate::syntax::Demonstrative;
use crate::syntax::DeterminerKind;
use crate::syntax::NominalComplement;
use crate::syntax::NominalPhrase;
use crate::syntax::NounPhrase;
use crate::syntax::PartitiveHead;
use crate::syntax::PartitiveNounPhrase;
use crate::syntax::Phrase;
use crate::syntax::PossessorKind;
use crate::syntax::Preposition;
use crate::syntax::PrepositionalPhrase;
use crate::syntax::Quantity;
use crate::syntax::Rounding;
use crate::syntax::SetExceptionMarker;
use crate::syntax::SetExceptionNounPhrase;
use crate::syntax::ThisCardForm;
use crate::word::Noun;
use crate::word::NounInstance;
use crate::word::NounInstanceKind;
use crate::word::Pronoun;
use crate::word::PronounCase;
use crate::word::Vocab;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RulesObjectNounPhrase(NounPhrase);

impl RulesObjectNounPhrase {
    pub(crate) fn into_noun_phrase(self) -> NounPhrase {
        self.0
    }
}

fn violation(construction: &'static str, requirement: &'static str) -> DeclarationViolation {
    DeclarationViolation {
        construction,
        requirement,
    }
}

fn make_nominal(nominal: NominalPhrase) -> Result<NounPhrase, DeclarationViolation> {
    Ok(NounPhrase::Nominal(nominal))
}

fn make_rules_object(
    nominal: RulesObjectFollowupNominal,
) -> Result<RulesObjectNounPhrase, DeclarationViolation> {
    Ok(RulesObjectNounPhrase(NounPhrase::Nominal(
        nominal.into_nominal(),
    )))
}

fn make_pronoun(pronoun: Pronoun, case: PronounCase) -> Result<NounPhrase, DeclarationViolation> {
    Ok(NounPhrase::Pronoun { pronoun, case })
}

fn make_quantity(quantity: Quantity) -> Result<NounPhrase, DeclarationViolation> {
    Ok(NounPhrase::Quantity(quantity))
}

fn make_this_card(form: ThisCardForm) -> Result<NounPhrase, DeclarationViolation> {
    Ok(NounPhrase::ThisCard(form))
}

fn make_possessive_this_card(form: ThisCardForm) -> Result<NounPhrase, DeclarationViolation> {
    let determiner = crate::constructions::determiner::build_determiner_possessive_this_card(form)?;
    let DeterminerKind::Possessive(possessor) = determiner.kind() else {
        unreachable!("the self-reference determiner builder returns a possessive")
    };
    Ok(NounPhrase::Possessive(possessor.clone()))
}

fn make_demonstrative(value: Demonstrative) -> Result<NounPhrase, DeclarationViolation> {
    Ok(NounPhrase::Demonstrative(value))
}

fn make_partitive(
    head: PartitiveHead,
    whole: NounPhrase,
) -> Result<NounPhrase, DeclarationViolation> {
    Ok(NounPhrase::Partitive(PartitiveNounPhrase {
        head,
        whole: Box::new(whole),
    }))
}

fn make_any_number_of(whole: NounPhrase) -> Result<NounPhrase, DeclarationViolation> {
    let determiner = crate::constructions::determiner::build_determiner_closed(
        crate::syntax::ClosedDeterminer::Any,
    )?;
    let head = NounInstance::Singular(Noun::Word(Vocab::Number));
    let nominal = crate::constructions::nominal::build_nominal_determiner(
        determiner,
        NominalPhrase::try_from_noun(head)?,
    )?;
    let nominal = crate::constructions::nominal::build_nominal_prepositional(
        nominal,
        PrepositionalPhrase::simple(Preposition::Of, Phrase::NounPhrase(Box::new(whole))),
    )?;
    Ok(NounPhrase::Nominal(nominal))
}

fn make_minus(left: NounPhrase, right: NounPhrase) -> Result<NounPhrase, DeclarationViolation> {
    Ok(NounPhrase::Arithmetic(ArithmeticValue::Minus {
        left: Box::new(left),
        right: Box::new(right),
    }))
}

fn make_half(
    value: NounPhrase,
    rounding: Option<Rounding>,
) -> Result<NounPhrase, DeclarationViolation> {
    Ok(NounPhrase::Arithmetic(ArithmeticValue::Half {
        value: Box::new(value),
        rounding,
    }))
}

fn make_set_exception(
    construction: &'static str,
    included: NounPhrase,
    marker: SetExceptionMarker,
    comma: Comma,
    excluded: NounPhrase,
) -> Result<NounPhrase, DeclarationViolation> {
    let expected = match construction {
        "noun_phrase_set_exception_bare" => SetExceptionMarker::Bare,
        "noun_phrase_set_exception_for" => SetExceptionMarker::For,
        _ => {
            return Err(violation(
                construction,
                "construction is a declared set-exception family",
            ));
        }
    };
    if marker != expected {
        return Err(violation(
            construction,
            "marker matches the declared set-exception surface",
        ));
    }
    Ok(NounPhrase::SetException(SetExceptionNounPhrase {
        included: Box::new(included),
        marker,
        comma,
        excluded: Box::new(excluded),
    }))
}

fn nominal_parts(value: &NounPhrase) -> NominalPhrase {
    let NounPhrase::Nominal(nominal) = value else {
        unreachable!("noun_phrase_nominal admits only Nominal")
    };
    nominal.clone()
}

fn rules_object_parts(value: &RulesObjectNounPhrase) -> RulesObjectFollowupNominal {
    let NounPhrase::Nominal(nominal) = &value.0 else {
        unreachable!("rules_object_noun_phrase stores a nominal noun phrase")
    };
    RulesObjectFollowupNominal::from_nominal(nominal.clone())
}

fn pronoun_parts(value: &NounPhrase) -> Pronoun {
    let NounPhrase::Pronoun { pronoun, .. } = value else {
        unreachable!("pronoun families admit only Pronoun")
    };
    *pronoun
}

fn quantity_parts(value: &NounPhrase) -> Quantity {
    let NounPhrase::Quantity(quantity) = value else {
        unreachable!("noun_phrase_quantity admits only Quantity")
    };
    *quantity
}

fn this_card_parts(value: &NounPhrase) -> ThisCardForm {
    let NounPhrase::ThisCard(form) = value else {
        unreachable!("self-reference families admit only ThisCard")
    };
    *form
}

fn possessive_this_card_parts(value: &NounPhrase) -> ThisCardForm {
    let NounPhrase::Possessive(possessor) = value else {
        unreachable!("possessive self-reference admits only Possessive")
    };
    let PossessorKind::NounPhrase(NounPhrase::ThisCard(form)) = possessor.kind() else {
        unreachable!("possessive self-reference stores ThisCard")
    };
    *form
}

fn demonstrative_parts(value: &NounPhrase) -> Demonstrative {
    let NounPhrase::Demonstrative(value) = value else {
        unreachable!("noun_phrase_demonstrative admits only Demonstrative")
    };
    *value
}

fn partitive_parts(value: &NounPhrase) -> (PartitiveHead, NounPhrase) {
    let NounPhrase::Partitive(partitive) = value else {
        unreachable!("partitive families admit only Partitive")
    };
    (partitive.head, partitive.whole.as_ref().clone())
}

fn quantity_partitive_parts(value: &NounPhrase) -> (Quantity, NounPhrase) {
    let (PartitiveHead::Quantity(quantity), whole) = partitive_parts(value) else {
        unreachable!("noun_phrase_partitive admits only counted partitives")
    };
    (quantity, whole)
}

fn each_partitive_parts(value: &NounPhrase) -> (PartitiveHead, NounPhrase) {
    partitive_parts(value)
}

fn any_number_of_whole(value: &NounPhrase) -> Option<&NounPhrase> {
    let NounPhrase::Nominal(nominal) = value else {
        return None;
    };
    if !matches!(
        nominal.determiner().map(|value| value.kind()),
        Some(DeterminerKind::Any)
    ) || !matches!(
        nominal.head().kind(),
        NounInstanceKind::Singular(Noun::Word(Vocab::Number))
    ) {
        return None;
    }
    let [NominalComplement::Prepositional(preposition)] = nominal.complements() else {
        return None;
    };
    let simple = preposition.as_simple()?;
    let Phrase::NounPhrase(whole) = simple.object.as_ref() else {
        return None;
    };
    (simple.preposition == Preposition::Of).then_some(whole.as_ref())
}

fn any_number_of_parts(value: &NounPhrase) -> NounPhrase {
    any_number_of_whole(value)
        .expect("noun_phrase_any_number_of admits only its nominal spine")
        .clone()
}

fn minus_parts(value: &NounPhrase) -> (NounPhrase, NounPhrase) {
    let NounPhrase::Arithmetic(ArithmeticValue::Minus { left, right }) = value else {
        unreachable!("noun_phrase_minus admits only Minus")
    };
    (left.as_ref().clone(), right.as_ref().clone())
}

fn half_parts(value: &NounPhrase) -> (NounPhrase, Option<Rounding>) {
    let NounPhrase::Arithmetic(ArithmeticValue::Half { value, rounding }) = value else {
        unreachable!("half families admit only Half")
    };
    (value.as_ref().clone(), *rounding)
}

fn half_plain_parts(value: &NounPhrase) -> NounPhrase {
    let (value, None) = half_parts(value) else {
        unreachable!("noun_phrase_half admits only unrounded Half")
    };
    value
}

fn half_rounded_parts(value: &NounPhrase) -> (NounPhrase, Rounding) {
    let (value, Some(rounding)) = half_parts(value) else {
        unreachable!("rounded-half families admit only rounded Half")
    };
    (value, rounding)
}

fn set_exception_parts(
    value: &NounPhrase,
) -> (NounPhrase, SetExceptionMarker, Option<Comma>, NounPhrase) {
    let NounPhrase::SetException(exception) = value else {
        unreachable!("set-exception families admit only SetException")
    };
    (
        exception.included.as_ref().clone(),
        exception.marker,
        (exception.comma == Comma::Present).then_some(Comma::Present),
        exception.excluded.as_ref().clone(),
    )
}

fn make_subject_pronoun(pronoun: Pronoun) -> Result<NounPhrase, DeclarationViolation> {
    make_pronoun(pronoun, PronounCase::Subject)
}

fn make_object_pronoun(pronoun: Pronoun) -> Result<NounPhrase, DeclarationViolation> {
    make_pronoun(pronoun, PronounCase::Object)
}

fn make_reciprocal(pronoun: Pronoun) -> Result<NounPhrase, DeclarationViolation> {
    if pronoun != Pronoun::EachOther {
        return Err(violation(
            "noun_phrase_reciprocal",
            "pronoun is the reciprocal each other",
        ));
    }
    make_pronoun(pronoun, PronounCase::Object)
}

fn make_quantity_partitive(
    quantity: Quantity,
    whole: NounPhrase,
) -> Result<NounPhrase, DeclarationViolation> {
    make_partitive(PartitiveHead::Quantity(quantity), whole)
}

fn make_each_partitive(
    head: PartitiveHead,
    whole: NounPhrase,
) -> Result<NounPhrase, DeclarationViolation> {
    if head != PartitiveHead::Each {
        return Err(violation(
            "noun_phrase_each_partitive",
            "head is distributive each",
        ));
    }
    make_partitive(head, whole)
}

fn make_half_plain(value: NounPhrase) -> Result<NounPhrase, DeclarationViolation> {
    make_half(value, None)
}

fn make_half_rounded_up(
    value: NounPhrase,
    rounding: Rounding,
) -> Result<NounPhrase, DeclarationViolation> {
    if rounding != Rounding::Up {
        return Err(violation(
            "noun_phrase_half_rounded_up",
            "rounding direction is up",
        ));
    }
    make_half(value, Some(rounding))
}

fn make_half_rounded_down(
    value: NounPhrase,
    rounding: Rounding,
) -> Result<NounPhrase, DeclarationViolation> {
    if rounding != Rounding::Down {
        return Err(violation(
            "noun_phrase_half_rounded_down",
            "rounding direction is down",
        ));
    }
    make_half(value, Some(rounding))
}

fn make_set_exception_bare(
    included: NounPhrase,
    marker: SetExceptionMarker,
    comma: Option<Comma>,
    excluded: NounPhrase,
) -> Result<NounPhrase, DeclarationViolation> {
    make_set_exception(
        "noun_phrase_set_exception_bare",
        included,
        marker,
        comma.unwrap_or(Comma::Absent),
        excluded,
    )
}

fn make_set_exception_for(
    included: NounPhrase,
    marker: SetExceptionMarker,
    comma: Option<Comma>,
    excluded: NounPhrase,
) -> Result<NounPhrase, DeclarationViolation> {
    make_set_exception(
        "noun_phrase_set_exception_for",
        included,
        marker,
        comma.unwrap_or(Comma::Absent),
        excluded,
    )
}

fn is_nominal(value: &NounPhrase) -> bool {
    matches!(value, NounPhrase::Nominal(_)) && any_number_of_whole(value).is_none()
}

fn is_subject_pronoun(value: &NounPhrase) -> bool {
    matches!(
        value,
        NounPhrase::Pronoun {
            case: PronounCase::Subject,
            ..
        }
    )
}

fn is_object_pronoun(value: &NounPhrase) -> bool {
    matches!(
        value,
        NounPhrase::Pronoun {
            pronoun,
            case: PronounCase::Object,
        } if *pronoun != Pronoun::EachOther
    )
}

fn is_reciprocal(value: &NounPhrase) -> bool {
    matches!(
        value,
        NounPhrase::Pronoun {
            pronoun: Pronoun::EachOther,
            case: PronounCase::Object,
        }
    )
}

fn is_quantity(value: &NounPhrase) -> bool {
    matches!(value, NounPhrase::Quantity(_))
}

fn is_abbreviated_this_card(value: &NounPhrase) -> bool {
    matches!(value, NounPhrase::ThisCard(ThisCardForm::AbbreviatedName))
}

fn is_full_this_card(value: &NounPhrase) -> bool {
    matches!(value, NounPhrase::ThisCard(ThisCardForm::FullName))
}

fn is_possessive_this_card(value: &NounPhrase) -> bool {
    matches!(
        value,
        NounPhrase::Possessive(possessor)
            if matches!(possessor.kind(), PossessorKind::NounPhrase(NounPhrase::ThisCard(_)))
    )
}

fn is_demonstrative(value: &NounPhrase) -> bool {
    matches!(value, NounPhrase::Demonstrative(_))
}

fn is_quantity_partitive(value: &NounPhrase) -> bool {
    matches!(
        value,
        NounPhrase::Partitive(PartitiveNounPhrase {
            head: PartitiveHead::Quantity(_),
            ..
        })
    )
}

fn is_each_partitive(value: &NounPhrase) -> bool {
    matches!(
        value,
        NounPhrase::Partitive(PartitiveNounPhrase {
            head: PartitiveHead::Each,
            ..
        })
    )
}

fn is_any_number_of(value: &NounPhrase) -> bool {
    any_number_of_whole(value).is_some()
}

fn is_minus(value: &NounPhrase) -> bool {
    matches!(value, NounPhrase::Arithmetic(ArithmeticValue::Minus { .. }))
}

fn is_half(value: &NounPhrase) -> bool {
    matches!(
        value,
        NounPhrase::Arithmetic(ArithmeticValue::Half { rounding: None, .. })
    )
}

fn is_half_rounded_up(value: &NounPhrase) -> bool {
    matches!(
        value,
        NounPhrase::Arithmetic(ArithmeticValue::Half {
            rounding: Some(Rounding::Up),
            ..
        })
    )
}

fn is_half_rounded_down(value: &NounPhrase) -> bool {
    matches!(
        value,
        NounPhrase::Arithmetic(ArithmeticValue::Half {
            rounding: Some(Rounding::Down),
            ..
        })
    )
}

fn is_set_exception_bare(value: &NounPhrase) -> bool {
    matches!(value, NounPhrase::SetException(value) if value.marker == SetExceptionMarker::Bare)
}

fn is_set_exception_for(value: &NounPhrase) -> bool {
    matches!(value, NounPhrase::SetException(value) if value.marker == SetExceptionMarker::For)
}

fn noun_phrase(
    agreement: Option<Agreement>,
    coordination_domain: Option<CoordinationDomain>,
    pronoun_case: Option<PronounCase>,
    adjunct: Option<crate::word::BareNominalAdjunct>,
    set_exception: SetExceptionState,
    rules_object_followup: bool,
) -> Features {
    Features::NounPhrase {
        agreement,
        coordination_domain,
        pronoun_case,
        adjunct,
        set_exception,
        coordination: NounPhraseCoordinationState::None,
        recipient_passive_theme: false,
        rules_object_followup,
    }
}

fn reduce_nominal(noun: &Features, rules_object_followup: bool) -> Option<Features> {
    let Features::Nominal {
        coordination_domain,
        form,
        determined,
        modified,
        adjunct,
        set_exception_host,
        recipient_passive_theme,
        ..
    } = noun
    else {
        return None;
    };
    let mut features = noun_phrase(
        Some(Agreement {
            person: Person::Third,
            number: match form {
                NounForm::Plural => Number::Plural,
                NounForm::Singular | NounForm::Mass => Number::Singular,
            },
        }),
        *coordination_domain,
        None,
        if *determined || *modified { *adjunct } else { None },
        if *set_exception_host {
            SetExceptionState::Host
        } else {
            SetExceptionState::Ineligible
        },
        rules_object_followup,
    );
    let Features::NounPhrase {
        recipient_passive_theme: output_theme,
        ..
    } = &mut features
    else {
        unreachable!()
    };
    *output_theme = *recipient_passive_theme;
    Some(features)
}

fn reduce_noun_phrase_nominal(nominal: &Features) -> Option<Features> {
    reduce_nominal(nominal, false)
}

fn reduce_rules_object_noun_phrase(nominal: &Features) -> Option<Features> {
    reduce_nominal(nominal, true)
}

fn reduce_pronoun(pronoun: &Features) -> Option<Features> {
    matches!(pronoun, Features::NounPhrase { .. }).then(|| pronoun.clone())
}

fn reduce_quantity(quantity: &Features) -> Option<Features> {
    let Features::Quantity(QuantityFeatures {
        standalone_number,
        is_one,
        ..
    }) = quantity
    else {
        return None;
    };
    Some(noun_phrase(
        Some(Agreement {
            person: Person::Third,
            number: *standalone_number,
        }),
        Some(if *is_one {
            CoordinationDomain::SelectionContinuation
        } else {
            CoordinationDomain::NonEntity
        }),
        None,
        None,
        SetExceptionState::Ineligible,
        false,
    ))
}

fn reduce_this_card(value: &Features) -> Option<Features> {
    matches!(value, Features::NounPhrase { .. }).then(|| value.clone())
}

fn reduce_possessive_this_card(value: &Features) -> Option<Features> {
    let Features::PossessiveThisCard { agreement } = value else {
        return None;
    };
    Some(noun_phrase(
        Some(*agreement),
        Some(CoordinationDomain::Entity),
        None,
        None,
        SetExceptionState::Ineligible,
        false,
    ))
}

fn reduce_partitive(head: &Features) -> Option<Features> {
    let Features::Quantity(QuantityFeatures {
        standalone_number, ..
    }) = head
    else {
        return None;
    };
    Some(noun_phrase(
        Some(Agreement {
            person: Person::Third,
            number: *standalone_number,
        }),
        Some(CoordinationDomain::SelectionHost),
        None,
        None,
        SetExceptionState::Ineligible,
        false,
    ))
}

fn reduce_each_partitive(head: &Features) -> Option<Features> {
    matches!(head, Features::Determiner { .. }).then(|| {
        noun_phrase(
            Some(Agreement {
                person: Person::Third,
                number: Number::Singular,
            }),
            Some(CoordinationDomain::SelectionHost),
            None,
            None,
            SetExceptionState::Ineligible,
            false,
        )
    })
}

fn reduce_any_number_of(whole: &Features) -> Option<Features> {
    let Features::NounPhrase {
        agreement: Some(Agreement {
            number: Number::Plural,
            ..
        }),
        coordination_domain,
        ..
    } = whole
    else {
        return None;
    };
    Some(noun_phrase(
        Some(Agreement {
            person: Person::Third,
            number: Number::Plural,
        }),
        *coordination_domain,
        None,
        None,
        SetExceptionState::Ineligible,
        false,
    ))
}

fn reduce_arithmetic(value: &Features) -> Option<Features> {
    matches!(value, Features::NounPhrase { .. }).then(|| {
        noun_phrase(
            Some(Agreement {
                person: Person::Third,
                number: Number::Singular,
            }),
            Some(CoordinationDomain::NonEntity),
            None,
            None,
            SetExceptionState::Ineligible,
            false,
        )
    })
}

fn reduce_minus(left: &Features, right: &Features) -> Option<Features> {
    matches!(right, Features::NounPhrase { .. })
        .then(|| reduce_arithmetic(left))
        .flatten()
}

fn reduce_set_exception(included: &Features, excluded: &Features) -> Option<Features> {
    if !noun_phrase_accepts_set_exception(included)
        || !matches!(excluded, Features::NounPhrase { .. })
    {
        return None;
    }
    let Features::NounPhrase {
        agreement,
        coordination_domain,
        pronoun_case,
        adjunct,
        ..
    } = included
    else {
        return None;
    };
    Some(noun_phrase(
        *agreement,
        *coordination_domain,
        *pronoun_case,
        *adjunct,
        SetExceptionState::Closed,
        false,
    ))
}

fn reduce_set_exception_prefix(included: &Features) -> Option<Features> {
    noun_phrase_accepts_set_exception(included).then(|| included.clone())
}

fn noun_phrase_accepts_set_exception(features: &Features) -> bool {
    matches!(
        features,
        Features::NounPhrase {
            set_exception: SetExceptionState::Host,
            ..
        }
    )
}

fn mark_generated_cost(feature: &Features) -> Option<Features> {
    Some(feature.clone())
}

deckmaste_constructions_macro::constructions! {
    group noun_phrase;

    construction noun_phrase_set_exception_bare: NounPhrase {
        bind NounPhrase via make_set_exception_bare, set_exception_parts {
            included: hole NounPhrase,
            marker: identity SetExceptionMarker via SetExceptionMarker,
            comma: opt lex Comma,
            excluded: hole NounPhrase,
        }
        derive features: Features = reduce_set_exception(included, excluded);
        derive prefix_admission: Features = reduce_set_exception_prefix(included);
        form plain @ 0 when comma.is_none() inverse check(is_set_exception_bare) = included identity(marker) excluded;
        form comma @ 1 otherwise = included lex(comma) identity(marker) excluded;
        selection unique;
    }

    construction noun_phrase_set_exception_for: NounPhrase {
        bind NounPhrase via make_set_exception_for, set_exception_parts {
            included: hole NounPhrase,
            marker: identity SetExceptionMarker via SetExceptionMarker,
            comma: opt lex Comma,
            excluded: hole NounPhrase,
        }
        derive features: Features = reduce_set_exception(included, excluded);
        derive prefix_admission: Features = reduce_set_exception_prefix(included);
        form plain @ 0 when comma.is_none() inverse check(is_set_exception_for) = included identity(marker) excluded;
        form comma @ 1 otherwise = included lex(comma) identity(marker) excluded;
        selection unique;
    }

    construction noun_phrase_nominal: NounPhrase {
        bind NounPhrase via make_nominal, nominal_parts {
            nominal: hole NominalPhrase,
        }
        derive features: Features = reduce_noun_phrase_nominal(nominal);
        form only @ 0 inverse check(is_nominal) = nominal;
        dominates noun_phrase_minus;
        dominates noun_phrase_coordination;
        selection unique;
    }

    construction rules_object_noun_phrase: RulesObjectNounPhrase {
        bind RulesObjectNounPhrase via make_rules_object, rules_object_parts {
            nominal: hole RulesObjectFollowupNominal,
        }
        derive features: Features = reduce_rules_object_noun_phrase(nominal);
        evidence role "rules-object attachment role" from category;
        form only @ 0 = nominal;
        selection unique;
    }

    construction noun_phrase_subject_pronoun: NounPhrase {
        bind NounPhrase via make_subject_pronoun, pronoun_parts {
            pronoun: identity Pronoun via SubjectPronoun,
        }
        derive features: Features = reduce_pronoun(pronoun);
        form only @ 0 inverse check(is_subject_pronoun) = identity(pronoun);
        dominates noun_phrase_object_pronoun;
        selection unique;
    }

    construction noun_phrase_object_pronoun: NounPhrase {
        bind NounPhrase via make_object_pronoun, pronoun_parts {
            pronoun: identity Pronoun via ObjectPronoun,
        }
        derive features: Features = reduce_pronoun(pronoun);
        form only @ 0 inverse check(is_object_pronoun) = identity(pronoun);
        selection unique;
    }

    construction noun_phrase_reciprocal: NounPhrase {
        bind NounPhrase via make_reciprocal, pronoun_parts {
            pronoun: identity Pronoun via Reciprocal,
        }
        derive features: Features = reduce_pronoun(pronoun);
        form only @ 0 inverse check(is_reciprocal) = identity(pronoun);
        selection unique;
    }

    construction noun_phrase_quantity: NounPhrase {
        bind NounPhrase via make_quantity, quantity_parts {
            quantity: hole Quantity,
        }
        derive features: Features = reduce_quantity(quantity);
        form only @ 0 inverse check(is_quantity) = quantity;
        selection unique;
    }

    construction noun_phrase_this_card: NounPhrase {
        bind NounPhrase via make_this_card, this_card_parts {
            form: identity ThisCardForm via ThisCard,
        }
        derive features: Features = reduce_this_card(form);
        form only @ 0 inverse check(is_abbreviated_this_card) = identity(form);
        selection unique;
    }

    construction noun_phrase_full_this_card: NounPhrase {
        bind NounPhrase via make_this_card, this_card_parts {
            form: identity ThisCardForm via FullThisCard,
        }
        derive features: Features = reduce_this_card(form);
        form only @ 0 inverse check(is_full_this_card) = identity(form);
        selection unique;
    }

    construction noun_phrase_possessive_this_card: NounPhrase {
        bind NounPhrase via make_possessive_this_card, possessive_this_card_parts {
            form: identity ThisCardForm via PossessiveThisCard,
        }
        derive features: Features = reduce_possessive_this_card(form);
        form only @ 0 inverse check(is_possessive_this_card) = identity(form);
        selection unique;
    }

    construction noun_phrase_demonstrative: NounPhrase {
        bind NounPhrase via make_demonstrative, demonstrative_parts {
            value: identity Demonstrative via Demonstrative,
        }
        derive features: Features = reduce_this_card(value);
        form only @ 0 inverse check(is_demonstrative) = identity(value);
        selection unique;
    }

    construction noun_phrase_partitive: NounPhrase {
        bind NounPhrase via make_quantity_partitive, quantity_partitive_parts {
            head: hole Quantity,
            whole: hole NounPhrase,
        }
        derive features: Features = reduce_partitive(head);
        form only @ 0 inverse check(is_quantity_partitive) = head "of" whole;
        selection unique;
    }

    construction noun_phrase_each_partitive: NounPhrase {
        bind NounPhrase via make_each_partitive, each_partitive_parts {
            head: identity PartitiveHead via PartitiveEach,
            whole: hole NounPhrase,
        }
        derive features: Features = reduce_each_partitive(head);
        form only @ 0 inverse check(is_each_partitive) = identity(head) "of" whole;
        selection unique;
    }

    construction noun_phrase_any_number_of: NounPhrase {
        bind NounPhrase via make_any_number_of, any_number_of_parts {
            whole: hole NounPhrase,
        }
        derive features: Features = reduce_any_number_of(whole);
        derive base_precedence: Features = mark_generated_cost(whole);
        form only @ 0 inverse check(is_any_number_of) = "any" "number" "of" whole;
        selection unique;
    }

    construction noun_phrase_minus: NounPhrase {
        bind NounPhrase via make_minus, minus_parts {
            left: hole NounPhrase,
            right: hole NounPhrase,
        }
        derive features: Features = reduce_minus(left, right);
        derive base_precedence: Features = mark_generated_cost(left);
        form only @ 0 inverse check(is_minus) = left "minus" right;
        selection unique;
    }

    construction noun_phrase_half: NounPhrase {
        bind NounPhrase via make_half_plain, half_plain_parts {
            value: hole NounPhrase,
        }
        derive features: Features = reduce_arithmetic(value);
        form only @ 0 inverse check(is_half) = "half" value;
        selection unique;
    }

    construction noun_phrase_half_rounded_up: NounPhrase {
        bind NounPhrase via make_half_rounded_up, half_rounded_parts {
            value: hole NounPhrase,
            rounding: identity Rounding via RoundingUp,
        }
        derive features: Features = reduce_arithmetic(value);
        form only @ 0 inverse check(is_half_rounded_up) = "half" value "," "rounded" identity(rounding);
        selection unique;
    }

    construction noun_phrase_half_rounded_down: NounPhrase {
        bind NounPhrase via make_half_rounded_down, half_rounded_parts {
            value: hole NounPhrase,
            rounding: identity Rounding via RoundingDown,
        }
        derive features: Features = reduce_arithmetic(value);
        form only @ 0 inverse check(is_half_rounded_down) = "half" value "," "rounded" identity(rounding);
        selection unique;
    }
}

pub(crate) static GROUPS: &[&GroupData] = &[&NOUN_PHRASE_DECLARATION];

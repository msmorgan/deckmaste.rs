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
use crate::syntax::NounPhraseKind;
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

/// A validated noun phrase eligible for rules-object attachment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RulesObjectNounPhrase(NounPhrase);

impl RulesObjectNounPhrase {
    /// Returns the validated noun phrase carried by this attachment-role view.
    #[must_use]
    pub const fn as_noun_phrase(&self) -> &NounPhrase {
        &self.0
    }

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
    Ok(NounPhrase::from_nominal_declaration(nominal))
}

fn make_rules_object(
    nominal: RulesObjectFollowupNominal,
) -> Result<RulesObjectNounPhrase, DeclarationViolation> {
    Ok(RulesObjectNounPhrase(NounPhrase::from_nominal_declaration(
        nominal.into_nominal(),
    )))
}

fn make_pronoun(pronoun: Pronoun, case: PronounCase) -> Result<NounPhrase, DeclarationViolation> {
    Ok(NounPhrase::from_pronoun_declaration(pronoun, case))
}

fn make_quantity(quantity: Quantity) -> Result<NounPhrase, DeclarationViolation> {
    Ok(NounPhrase::from_quantity_declaration(quantity))
}

fn make_this_card(form: ThisCardForm) -> Result<NounPhrase, DeclarationViolation> {
    Ok(NounPhrase::from_this_card_declaration(form))
}

fn make_possessive_this_card(form: ThisCardForm) -> Result<NounPhrase, DeclarationViolation> {
    let determiner = crate::constructions::determiner::build_determiner_possessive_this_card(form)?;
    let DeterminerKind::Possessive(possessor) = determiner.kind() else {
        unreachable!("the self-reference determiner builder returns a possessive")
    };
    Ok(NounPhrase::from_possessive_declaration(possessor.clone()))
}

fn make_demonstrative(value: Demonstrative) -> Result<NounPhrase, DeclarationViolation> {
    Ok(NounPhrase::from_demonstrative_declaration(value))
}

fn make_partitive(
    head: PartitiveHead,
    whole: NounPhrase,
) -> Result<NounPhrase, DeclarationViolation> {
    Ok(NounPhrase::from_partitive_declaration(
        PartitiveNounPhrase {
            head,
            whole: Box::new(whole),
        },
    ))
}

fn make_any_number_of(whole: NounPhrase) -> Result<NounPhrase, DeclarationViolation> {
    if !noun_phrase_is_plural(&whole) {
        return Err(violation(
            "noun_phrase_any_number_of",
            "whole has plural agreement",
        ));
    }
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
    Ok(NounPhrase::from_nominal_declaration(nominal))
}

fn make_minus(left: NounPhrase, right: NounPhrase) -> Result<NounPhrase, DeclarationViolation> {
    Ok(NounPhrase::from_arithmetic_declaration(
        ArithmeticValue::Minus {
            left: Box::new(left),
            right: Box::new(right),
        },
    ))
}

fn make_half(
    value: NounPhrase,
    rounding: Option<Rounding>,
) -> Result<NounPhrase, DeclarationViolation> {
    Ok(NounPhrase::from_arithmetic_declaration(
        ArithmeticValue::Half {
            value: Box::new(value),
            rounding,
        },
    ))
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
    if !noun_phrase_is_set_exception_host(&included) {
        return Err(violation(
            construction,
            "included noun phrase is an open set-exception host",
        ));
    }
    Ok(NounPhrase::from_set_exception_declaration(
        SetExceptionNounPhrase {
            included: Box::new(included),
            marker,
            comma,
            excluded: Box::new(excluded),
        },
    ))
}

fn noun_phrase_is_plural(value: &NounPhrase) -> bool {
    match value.kind() {
        NounPhraseKind::Nominal(nominal) => {
            matches!(nominal.head().kind(), NounInstanceKind::Plural(_))
                || any_number_of_whole(value).is_some()
        }
        NounPhraseKind::Pronoun { pronoun, .. } => pronoun == Pronoun::They,
        NounPhraseKind::Demonstrative(value) => {
            matches!(value, Demonstrative::These | Demonstrative::Those)
        }
        NounPhraseKind::CoordinatedNominal(_) | NounPhraseKind::Coordinated(_) => true,
        NounPhraseKind::SetException(exception) => noun_phrase_is_plural(&exception.included),
        NounPhraseKind::Possessive(_)
        | NounPhraseKind::Quantity(_)
        | NounPhraseKind::ThisCard(_)
        | NounPhraseKind::Partitive(_)
        | NounPhraseKind::Arithmetic(_) => false,
    }
}

fn noun_phrase_is_set_exception_host(value: &NounPhrase) -> bool {
    match value.kind() {
        NounPhraseKind::Nominal(nominal) => matches!(
            nominal.determiner().map(crate::syntax::Determiner::kind),
            Some(DeterminerKind::All | DeterminerKind::Each)
        ),
        NounPhraseKind::CoordinatedNominal(coordinated) => matches!(
            coordinated.determiner().kind(),
            DeterminerKind::All | DeterminerKind::Each
        ),
        NounPhraseKind::Coordinated(coordinated) => {
            noun_phrase_is_set_exception_host(coordinated.first())
        }
        NounPhraseKind::Pronoun { .. }
        | NounPhraseKind::Possessive(_)
        | NounPhraseKind::Demonstrative(_)
        | NounPhraseKind::Quantity(_)
        | NounPhraseKind::ThisCard(_)
        | NounPhraseKind::Partitive(_)
        | NounPhraseKind::SetException(_)
        | NounPhraseKind::Arithmetic(_) => false,
    }
}

fn nominal_parts(value: &NounPhrase) -> NominalPhrase {
    let NounPhraseKind::Nominal(nominal) = value.kind() else {
        unreachable!("noun_phrase_nominal admits only Nominal")
    };
    nominal.clone()
}

fn rules_object_parts(value: &RulesObjectNounPhrase) -> RulesObjectFollowupNominal {
    let NounPhraseKind::Nominal(nominal) = value.0.kind() else {
        unreachable!("rules_object_noun_phrase stores a nominal noun phrase")
    };
    RulesObjectFollowupNominal::from_nominal(nominal.clone())
}

fn pronoun_parts(value: &NounPhrase) -> Pronoun {
    let NounPhraseKind::Pronoun { pronoun, .. } = value.kind() else {
        unreachable!("pronoun families admit only Pronoun")
    };
    pronoun
}

fn quantity_parts(value: &NounPhrase) -> Quantity {
    let NounPhraseKind::Quantity(quantity) = value.kind() else {
        unreachable!("noun_phrase_quantity admits only Quantity")
    };
    quantity
}

fn this_card_parts(value: &NounPhrase) -> ThisCardForm {
    let NounPhraseKind::ThisCard(form) = value.kind() else {
        unreachable!("self-reference families admit only ThisCard")
    };
    form
}

fn possessive_this_card_parts(value: &NounPhrase) -> ThisCardForm {
    let NounPhraseKind::Possessive(possessor) = value.kind() else {
        unreachable!("possessive self-reference admits only Possessive")
    };
    let PossessorKind::NounPhrase(noun_phrase) = possessor.kind() else {
        unreachable!("possessive self-reference stores ThisCard")
    };
    let NounPhraseKind::ThisCard(form) = noun_phrase.kind() else {
        unreachable!("possessive self-reference stores ThisCard")
    };
    form
}

fn demonstrative_parts(value: &NounPhrase) -> Demonstrative {
    let NounPhraseKind::Demonstrative(value) = value.kind() else {
        unreachable!("noun_phrase_demonstrative admits only Demonstrative")
    };
    value
}

fn partitive_parts(value: &NounPhrase) -> (PartitiveHead, NounPhrase) {
    let NounPhraseKind::Partitive(partitive) = value.kind() else {
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
    let NounPhraseKind::Nominal(nominal) = value.kind() else {
        return None;
    };
    if !matches!(
        nominal.determiner().map(|value| value.kind()),
        Some(DeterminerKind::Any)
    ) || !nominal.modifiers().is_empty()
        || !matches!(
            nominal.head().kind(),
            NounInstanceKind::Singular(Noun::Word(Vocab::Number))
        )
    {
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
    let NounPhraseKind::Arithmetic(ArithmeticValue::Minus { left, right }) = value.kind() else {
        unreachable!("noun_phrase_minus admits only Minus")
    };
    (left.as_ref().clone(), right.as_ref().clone())
}

fn half_parts(value: &NounPhrase) -> (NounPhrase, Option<Rounding>) {
    let NounPhraseKind::Arithmetic(ArithmeticValue::Half { value, rounding }) = value.kind() else {
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
    let NounPhraseKind::SetException(exception) = value.kind() else {
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
    if matches!(
        pronoun,
        Pronoun::EachOther | Pronoun::Itself | Pronoun::Himself | Pronoun::YoursAbsolute
    ) {
        return Err(violation(
            "noun_phrase_subject_pronoun",
            "pronoun has a subject-case surface form",
        ));
    }
    make_pronoun(pronoun, PronounCase::Subject)
}

fn make_object_pronoun(pronoun: Pronoun) -> Result<NounPhrase, DeclarationViolation> {
    if pronoun == Pronoun::EachOther {
        return Err(violation(
            "noun_phrase_object_pronoun",
            "pronoun is not the dedicated reciprocal",
        ));
    }
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
    matches!(value.kind(), NounPhraseKind::Nominal(_)) && any_number_of_whole(value).is_none()
}

fn is_subject_pronoun(value: &NounPhrase) -> bool {
    matches!(
        value.kind(),
        NounPhraseKind::Pronoun {
            case: PronounCase::Subject,
            ..
        }
    )
}

fn is_object_pronoun(value: &NounPhrase) -> bool {
    matches!(
        value.kind(),
        NounPhraseKind::Pronoun {
            pronoun,
            case: PronounCase::Object,
        } if pronoun != Pronoun::EachOther
    )
}

fn is_reciprocal(value: &NounPhrase) -> bool {
    matches!(
        value.kind(),
        NounPhraseKind::Pronoun {
            pronoun: Pronoun::EachOther,
            case: PronounCase::Object,
        }
    )
}

fn is_quantity(value: &NounPhrase) -> bool {
    matches!(value.kind(), NounPhraseKind::Quantity(_))
}

fn is_abbreviated_this_card(value: &NounPhrase) -> bool {
    matches!(
        value.kind(),
        NounPhraseKind::ThisCard(ThisCardForm::AbbreviatedName)
    )
}

fn is_full_this_card(value: &NounPhrase) -> bool {
    matches!(
        value.kind(),
        NounPhraseKind::ThisCard(ThisCardForm::FullName)
    )
}

fn is_possessive_this_card(value: &NounPhrase) -> bool {
    matches!(
        value.kind(),
        NounPhraseKind::Possessive(possessor)
            if matches!(
                possessor.kind(),
                PossessorKind::NounPhrase(noun_phrase)
                    if matches!(noun_phrase.kind(), NounPhraseKind::ThisCard(_))
            )
    )
}

fn is_demonstrative(value: &NounPhrase) -> bool {
    matches!(value.kind(), NounPhraseKind::Demonstrative(_))
}

fn is_quantity_partitive(value: &NounPhrase) -> bool {
    matches!(
        value.kind(),
        NounPhraseKind::Partitive(PartitiveNounPhrase {
            head: PartitiveHead::Quantity(_),
            ..
        })
    )
}

fn is_each_partitive(value: &NounPhrase) -> bool {
    matches!(
        value.kind(),
        NounPhraseKind::Partitive(PartitiveNounPhrase {
            head: PartitiveHead::Each,
            ..
        })
    )
}

fn is_any_number_of(value: &NounPhrase) -> bool {
    any_number_of_whole(value).is_some()
}

fn is_minus(value: &NounPhrase) -> bool {
    matches!(
        value.kind(),
        NounPhraseKind::Arithmetic(ArithmeticValue::Minus { .. })
    )
}

fn is_half(value: &NounPhrase) -> bool {
    matches!(
        value.kind(),
        NounPhraseKind::Arithmetic(ArithmeticValue::Half { rounding: None, .. })
    )
}

fn is_half_rounded_up(value: &NounPhrase) -> bool {
    matches!(
        value.kind(),
        NounPhraseKind::Arithmetic(ArithmeticValue::Half {
            rounding: Some(Rounding::Up),
            ..
        })
    )
}

fn is_half_rounded_down(value: &NounPhrase) -> bool {
    matches!(
        value.kind(),
        NounPhraseKind::Arithmetic(ArithmeticValue::Half {
            rounding: Some(Rounding::Down),
            ..
        })
    )
}

fn is_set_exception_bare(value: &NounPhrase) -> bool {
    matches!(value.kind(), NounPhraseKind::SetException(value) if value.marker == SetExceptionMarker::Bare)
}

fn is_set_exception_for(value: &NounPhrase) -> bool {
    matches!(value.kind(), NounPhraseKind::SetException(value) if value.marker == SetExceptionMarker::For)
}

pub(crate) fn partitive_head_spelling(value: PartitiveHead) -> &'static str {
    match value {
        PartitiveHead::Each => "each",
        PartitiveHead::Quantity(_) => {
            unreachable!("quantity partitive heads linearize through their Quantity hole")
        }
    }
}

pub(crate) const fn set_exception_marker_spelling(value: SetExceptionMarker) -> &'static str {
    match value {
        SetExceptionMarker::Bare => "except",
        SetExceptionMarker::For => "except for",
    }
}

pub(crate) const fn rounding_spelling(value: Rounding) -> &'static str {
    match value {
        Rounding::Up => "up",
        Rounding::Down => "down",
    }
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
        evidence feature "set-exception host eligibility" from category;
        form plain @ 0 when comma.is_none() inverse check(is_set_exception_bare) = included identity(marker) excluded;
        form comma @ 1 inverse check(is_set_exception_bare) otherwise = included lex(comma) identity(marker) excluded;
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
        evidence feature "set-exception host eligibility" from category;
        form plain @ 0 when comma.is_none() inverse check(is_set_exception_for) = included identity(marker) excluded;
        form comma @ 1 inverse check(is_set_exception_for) otherwise = included lex(comma) identity(marker) excluded;
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
        evidence feature "pronoun case" from category;
        form only @ 0 inverse check(is_subject_pronoun) = identity(pronoun);
        dominates noun_phrase_object_pronoun;
        selection unique;
    }

    construction noun_phrase_object_pronoun: NounPhrase {
        bind NounPhrase via make_object_pronoun, pronoun_parts {
            pronoun: identity Pronoun via ObjectPronoun,
        }
        derive features: Features = reduce_pronoun(pronoun);
        evidence feature "pronoun case" from category;
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
        evidence feature "notional plural agreement" from category;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct ConstructionRecorder {
        construction: Option<&'static str>,
    }

    impl deckmaste_construction_compiler::runtime::LinearizationVisitor for ConstructionRecorder {
        type Error = std::convert::Infallible;

        fn begin_form(
            &mut self,
            construction: &'static str,
            _form: &'static str,
            _ordinal: u16,
        ) -> Result<(), Self::Error> {
            self.construction = Some(construction);
            Ok(())
        }

        fn literal(&mut self, _literal: &'static str) -> Result<(), Self::Error> {
            Ok(())
        }

        fn subtree<T: std::any::Any>(
            &mut self,
            _category: &'static str,
            _value: &T,
        ) -> Result<(), Self::Error> {
            Ok(())
        }

        fn scalar<T: std::any::Any>(
            &mut self,
            _codec: &'static str,
            _value: &T,
        ) -> Result<(), Self::Error> {
            Ok(())
        }

        fn identity<T: std::any::Any>(
            &mut self,
            _provider: &'static str,
            _value_type: &'static str,
            _value: &T,
        ) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    #[test]
    fn modified_any_number_nominal_keeps_the_ordinary_inverse_and_modifier() {
        let parsed = crate::parse_fragment(
            "any large number of players",
            &crate::Catalogs::default(),
            crate::FragmentKind::Nominal,
            "Test Card",
            false,
        );
        let Some(crate::Fragment::Nominal(value)) = parsed.into_fragment() else {
            panic!("expected a nominal noun phrase")
        };
        let NounPhraseKind::Nominal(nominal) = value.kind() else { unreachable!() };
        assert_eq!(nominal.modifiers().len(), 1);
        assert!(
            any_number_of_whole(&value).is_none(),
            "only the unmodified literal spine belongs to noun_phrase_any_number_of"
        );

        let mut ordinary = ConstructionRecorder::default();
        linearize_noun_phrase_nominal_with(&value, &mut ordinary)
            .expect("the modified nominal has the ordinary P01 inverse");
        assert_eq!(ordinary.construction, Some("noun_phrase_nominal"));

        let surface = crate::renderer::render_nominal_construction_form(
            &value,
            0,
            |value, ordinal, visitor| {
                linearize_noun_phrase_nominal_form_with(value, ordinal, visitor)
            },
        )
        .expect("the ordinary nominal inverse preserves every modifier");
        assert_eq!(surface, "any large number of players");
    }

    #[test]
    fn generated_renderer_covers_the_rules_object_role_wrapper() {
        let nominal = NominalPhrase::try_from_noun(NounInstance::Singular(Noun::Word(Vocab::Card)))
            .expect("card is a singular count noun");
        let value =
            build_rules_object_noun_phrase(RulesObjectFollowupNominal::from_nominal(nominal))
                .expect("the rules-object wrapper accepts its typed nominal role");
        assert_eq!(
            crate::renderer::render_generated_rules_object_noun_phrase_law(&value)
                .expect("the P01 rules-object construction linearizes"),
            "card",
        );
    }
}

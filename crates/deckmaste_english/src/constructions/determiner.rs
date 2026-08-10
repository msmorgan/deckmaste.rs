//! Compiler-derived English determiner and possession declarations.

#![allow(
    clippy::unnecessary_wraps,
    dead_code,
    reason = "the declaration ABI uses fallible builders and optional feature combinators uniformly"
)]

use deckmaste_construction_compiler::runtime::DeclarationViolation;
use deckmaste_construction_compiler::runtime::GroupData;

use super::DeterminerRepr;
use super::NounPhrase;
use super::Possessor;
use super::PossessorRepr;
use crate::features::NounCardinality;
use crate::grammar::AdjectiveComparisonState;
use crate::grammar::Features;
use crate::grammar::NounForm;
use crate::syntax::AdjectivePhrase;
use crate::syntax::ClosedDeterminer;
use crate::syntax::Determiner;
use crate::syntax::NominalPhrase;
use crate::syntax::Quantity;
use crate::syntax::ThisCardForm;
use crate::word::NounInstance;

type PossessiveNominal = NominalPhrase;

fn violation(construction: &'static str, requirement: &'static str) -> DeclarationViolation {
    DeclarationViolation {
        construction,
        requirement,
    }
}

fn is_closed(value: &Determiner) -> bool {
    matches!(
        value.repr,
        DeterminerRepr::The
            | DeterminerRepr::Each
            | DeterminerRepr::Another
            | DeterminerRepr::Indefinite
            | DeterminerRepr::Demonstrative(_)
            | DeterminerRepr::Possessive(Possessor {
                repr: PossessorRepr::Pronoun(_)
            })
            | DeterminerRepr::All
            | DeterminerRepr::Any
            | DeterminerRepr::No
    )
}

fn make_closed(identity: ClosedDeterminer) -> Result<Determiner, DeclarationViolation> {
    if matches!(identity, ClosedDeterminer::PossessivePronoun(pronoun) if pronoun.possessive_spelling().is_none())
    {
        return Err(violation(
            "determiner_closed",
            "possessive pronoun has a determiner form",
        ));
    }
    let repr = match identity {
        ClosedDeterminer::The => DeterminerRepr::The,
        ClosedDeterminer::Each => DeterminerRepr::Each,
        ClosedDeterminer::Another => DeterminerRepr::Another,
        ClosedDeterminer::Indefinite => DeterminerRepr::Indefinite,
        ClosedDeterminer::Demonstrative(value) => DeterminerRepr::Demonstrative(value),
        ClosedDeterminer::PossessivePronoun(value) => DeterminerRepr::Possessive(Possessor {
            repr: PossessorRepr::Pronoun(value),
        }),
        ClosedDeterminer::All => DeterminerRepr::All,
        ClosedDeterminer::Any => DeterminerRepr::Any,
        ClosedDeterminer::No => DeterminerRepr::No,
    };
    Ok(Determiner { repr })
}

fn closed_parts(value: &Determiner) -> ClosedDeterminer {
    match &value.repr {
        DeterminerRepr::The => ClosedDeterminer::The,
        DeterminerRepr::Each => ClosedDeterminer::Each,
        DeterminerRepr::Another => ClosedDeterminer::Another,
        DeterminerRepr::Indefinite => ClosedDeterminer::Indefinite,
        DeterminerRepr::Demonstrative(value) => ClosedDeterminer::Demonstrative(*value),
        DeterminerRepr::Possessive(Possessor {
            repr: PossessorRepr::Pronoun(value),
        }) => ClosedDeterminer::PossessivePronoun(*value),
        DeterminerRepr::All => ClosedDeterminer::All,
        DeterminerRepr::Any => ClosedDeterminer::Any,
        DeterminerRepr::No => ClosedDeterminer::No,
        DeterminerRepr::Target(_)
        | DeterminerRepr::Quantity(_)
        | DeterminerRepr::Possessive(Possessor {
            repr: PossessorRepr::NounPhrase(_),
        }) => unreachable!("determiner_closed admits only closed identities"),
    }
}

fn make_target() -> Result<Determiner, DeclarationViolation> {
    Ok(Determiner {
        repr: DeterminerRepr::Target(None),
    })
}

fn no_parts(_: &Determiner) {}

fn is_target(value: &Determiner) -> bool {
    matches!(value.repr, DeterminerRepr::Target(None))
}

fn make_quantified_target(quantity: Quantity) -> Result<Determiner, DeclarationViolation> {
    Ok(Determiner {
        repr: DeterminerRepr::Target(Some(quantity)),
    })
}

fn quantified_target_parts(value: &Determiner) -> Quantity {
    let DeterminerRepr::Target(Some(quantity)) = value.repr else {
        unreachable!("determiner_quantified_target admits only quantified target")
    };
    quantity
}

fn is_quantified_target(value: &Determiner) -> bool {
    matches!(value.repr, DeterminerRepr::Target(Some(_)))
}

fn make_quantity(quantity: Quantity) -> Result<Determiner, DeclarationViolation> {
    Ok(Determiner {
        repr: DeterminerRepr::Quantity(quantity),
    })
}

fn quantity_parts(value: &Determiner) -> Quantity {
    let DeterminerRepr::Quantity(quantity) = value.repr else {
        unreachable!("determiner_quantity admits only direct quantity")
    };
    quantity
}

fn is_quantity(value: &Determiner) -> bool {
    matches!(value.repr, DeterminerRepr::Quantity(_))
}

fn make_possessive_this_card(form: ThisCardForm) -> Result<Determiner, DeclarationViolation> {
    Ok(Determiner {
        repr: DeterminerRepr::Possessive(Possessor {
            repr: PossessorRepr::NounPhrase(Box::new(NounPhrase::from_this_card_declaration(form))),
        }),
    })
}

fn possessive_this_card_parts(value: &Determiner) -> ThisCardForm {
    let DeterminerRepr::Possessive(Possessor {
        repr: PossessorRepr::NounPhrase(noun),
    }) = &value.repr
    else {
        unreachable!("determiner_possessive_this_card admits only self-reference possessors")
    };
    let crate::syntax::NounPhraseKind::ThisCard(form) = noun.kind() else {
        unreachable!("determiner_possessive_this_card admits only self-reference possessors")
    };
    form
}

fn is_possessive_this_card(value: &Determiner) -> bool {
    matches!(
        &value.repr,
        DeterminerRepr::Possessive(Possessor {
            repr: PossessorRepr::NounPhrase(noun)
        })
            if matches!(noun.kind(), crate::syntax::NounPhraseKind::ThisCard(_))
    )
}

fn make_possessive_noun_base(
    head: NounInstance,
) -> Result<PossessiveNominal, DeclarationViolation> {
    NominalPhrase::try_from_noun(head)
}

fn possessive_noun_base_parts(value: &PossessiveNominal) -> NounInstance {
    value.head().clone()
}

fn is_possessive_noun_base(value: &PossessiveNominal) -> bool {
    value.determiner().is_none() && value.modifiers().is_empty() && value.complements().is_empty()
}

fn possessive_noun_determined_remainder(value: &PossessiveNominal) -> Option<PossessiveNominal> {
    if value.determiner().is_none() || !value.complements().is_empty() {
        return None;
    }
    let (_, modifiers, head, complements) = value.clone().into_projection_parts();
    Some(NominalPhrase::from_projection_parts(
        None,
        modifiers,
        head,
        complements,
    ))
}

fn possessive_noun_adjective_remainder(value: &PossessiveNominal) -> Option<PossessiveNominal> {
    if value.determiner().is_some() || !value.complements().is_empty() {
        return None;
    }
    let Some(crate::syntax::NominalModifier::Adjective {
        polarity: crate::syntax::Polarity::Positive,
        phrase,
    }) = value.modifiers().first()
    else {
        return None;
    };
    if phrase.complements().iter().any(|complement| {
        matches!(
            complement,
            crate::syntax::AdjectiveComplement::PostnominalComparison(_)
        )
    }) {
        return None;
    }
    let (determiner, mut modifiers, head, complements) = value.clone().into_projection_parts();
    modifiers.remove(0);
    Some(NominalPhrase::from_projection_parts(
        determiner,
        modifiers,
        head,
        complements,
    ))
}

fn is_valid_possessive_nominal(value: &PossessiveNominal) -> bool {
    let mut inverse_count = u8::from(is_possessive_noun_base(value));

    if let Some(possessor) = possessive_noun_determined_remainder(value) {
        inverse_count += u8::from(is_valid_possessive_nominal(&possessor));
    }

    if let Some(possessor) = possessive_noun_adjective_remainder(value) {
        inverse_count += u8::from(is_valid_possessive_nominal(&possessor));
    }

    inverse_count == 1
}

fn make_possessive_noun_determined(
    determiner: Determiner,
    possessor: PossessiveNominal,
) -> Result<PossessiveNominal, DeclarationViolation> {
    if !is_valid_possessive_nominal(&possessor) {
        return Err(violation(
            "possessive_noun_determined",
            "possessor has exactly one D01 inverse",
        ));
    }
    crate::constructions::nominal::build_nominal_determiner(determiner, possessor)
}

fn possessive_noun_determined_parts(value: &PossessiveNominal) -> (Determiner, PossessiveNominal) {
    let determiner = value
        .determiner()
        .cloned()
        .expect("possessive_noun_determined admits one determiner");
    let (_, modifiers, head, complements) = value.clone().into_projection_parts();
    (
        determiner,
        NominalPhrase::from_projection_parts(None, modifiers, head, complements),
    )
}

fn is_possessive_noun_determined(value: &PossessiveNominal) -> bool {
    possessive_noun_determined_remainder(value)
        .is_some_and(|possessor| is_valid_possessive_nominal(&possessor))
}

fn make_determiner_possessive_noun(
    possessor: PossessiveNominal,
) -> Result<Determiner, DeclarationViolation> {
    if !is_valid_possessive_nominal(&possessor) {
        return Err(violation(
            "determiner_possessive_noun",
            "possessor has exactly one D01 inverse",
        ));
    }
    Ok(Determiner {
        repr: DeterminerRepr::Possessive(Possessor {
            repr: PossessorRepr::NounPhrase(Box::new(NounPhrase::from_nominal_declaration(
                possessor,
            ))),
        }),
    })
}

fn determiner_possessive_noun_parts(value: &Determiner) -> PossessiveNominal {
    let DeterminerRepr::Possessive(Possessor {
        repr: PossessorRepr::NounPhrase(noun),
    }) = &value.repr
    else {
        unreachable!("determiner_possessive_noun admits only noun possessors")
    };
    let crate::syntax::NounPhraseKind::Nominal(nominal) = noun.kind() else {
        unreachable!("determiner_possessive_noun admits only noun possessors")
    };
    nominal.clone()
}

fn is_determiner_possessive_noun(value: &Determiner) -> bool {
    matches!(
        &value.repr,
        DeterminerRepr::Possessive(Possessor {
            repr: PossessorRepr::NounPhrase(noun)
        })
            if matches!(noun.kind(), crate::syntax::NounPhraseKind::Nominal(nominal) if is_valid_possessive_nominal(nominal))
    )
}

fn make_possessive_noun_adjective(
    adjective: AdjectivePhrase,
    mut possessor: PossessiveNominal,
) -> Result<PossessiveNominal, DeclarationViolation> {
    if !is_valid_possessive_nominal(&possessor) {
        return Err(violation(
            "possessive_noun_adjective",
            "possessor has exactly one D01 inverse",
        ));
    }
    if possessor.determiner().is_some() {
        return Err(violation(
            "possessive_noun_adjective",
            "possessor has no determiner",
        ));
    }
    if matches!(
        adjective.head(),
        crate::word::Adjective::Participle(_, crate::word::Verb::Word(crate::word::Vocab::Name))
    ) {
        possessor.open_name_interior();
    }
    crate::constructions::nominal::build_nominal_adjective(adjective, possessor)
}

fn possessive_noun_adjective_parts(
    value: &PossessiveNominal,
) -> (AdjectivePhrase, PossessiveNominal) {
    let Some(crate::syntax::NominalModifier::Adjective { phrase, .. }) = value.modifiers().first()
    else {
        unreachable!("possessive_noun_adjective admits an adjective-prefixed possessor")
    };
    let (determiner, mut modifiers, head, complements) = value.clone().into_projection_parts();
    modifiers.remove(0);
    (
        phrase.clone(),
        NominalPhrase::from_projection_parts(determiner, modifiers, head, complements),
    )
}

fn is_possessive_noun_adjective(value: &PossessiveNominal) -> bool {
    possessive_noun_adjective_remainder(value)
        .is_some_and(|possessor| is_valid_possessive_nominal(&possessor))
}

fn closed_features(identity: &Features) -> Option<Features> {
    matches!(identity, Features::Determiner { .. }).then(|| identity.clone())
}

const fn target_features() -> Option<Features> {
    Some(determiner_features(NounCardinality::SingularCount))
}

fn quantified_target_features(quantity: &Features) -> Option<Features> {
    let Features::Quantity(quantity) = quantity else { return None };
    Some(determiner_features(target_cardinality(
        quantity.cardinality,
    )))
}

fn quantity_features(quantity: &Features) -> Option<Features> {
    let Features::Quantity(quantity) = quantity else { return None };
    Some(determiner_features(quantity.cardinality))
}

const fn possessive_features() -> Option<Features> {
    Some(determiner_features(NounCardinality::Unconstrained))
}

fn possessive_noun_base_features(head: &Features) -> Option<Features> {
    let Features::Noun {
        form,
        initial_sound,
        ..
    } = head
    else {
        return None;
    };
    Some(Features::PossessiveNounPhrase {
        form: *form,
        initial_sound: *initial_sound,
        determined: false,
    })
}

fn possessive_noun_determined_features(
    determiner: &Features,
    possessor: &Features,
) -> Option<Features> {
    let Features::Determiner {
        cardinality,
        article,
        ..
    } = determiner
    else {
        return None;
    };
    let Features::PossessiveNounPhrase {
        form,
        initial_sound,
        determined,
    } = possessor
    else {
        return None;
    };
    (!*determined
        && cardinality_accepts(*cardinality, *form)
        && article_accepts(*article, *initial_sound))
    .then_some(Features::PossessiveNounPhrase {
        form: *form,
        initial_sound: *initial_sound,
        determined: true,
    })
}

fn possessive_noun_adjective_features(
    adjective: &Features,
    possessor: &Features,
) -> Option<Features> {
    let Features::Adjective {
        initial_sound,
        comparison,
        card_orientation: false,
        ..
    } = adjective
    else {
        return None;
    };
    let Features::PossessiveNounPhrase {
        form,
        determined: false,
        ..
    } = possessor
    else {
        return None;
    };
    matches!(
        comparison,
        AdjectiveComparisonState::NotComparative
            | AdjectiveComparisonState::Pending(_)
            | AdjectiveComparisonState::Complete
    )
    .then_some(Features::PossessiveNounPhrase {
        form: *form,
        initial_sound: *initial_sound,
        determined: false,
    })
}

const fn determiner_features(cardinality: NounCardinality) -> Features {
    Features::Determiner {
        cardinality,
        article: None,
        demonstrative_this: false,
        set_exception_host: false,
    }
}

const fn target_cardinality(cardinality: NounCardinality) -> NounCardinality {
    match cardinality {
        NounCardinality::SingularOrMass => NounCardinality::SingularCount,
        NounCardinality::PluralOrMass => NounCardinality::PluralCount,
        other => other,
    }
}

const fn cardinality_accepts(cardinality: NounCardinality, form: NounForm) -> bool {
    matches!(
        (cardinality, form),
        (NounCardinality::Unconstrained, _)
            | (NounCardinality::SingularCount, NounForm::Singular)
            | (NounCardinality::PluralCount, NounForm::Plural)
            | (NounCardinality::Mass, NounForm::Mass)
            | (
                NounCardinality::SingularOrMass,
                NounForm::Singular | NounForm::Mass
            )
            | (
                NounCardinality::PluralOrMass,
                NounForm::Plural | NounForm::Mass
            )
    )
}

const fn article_accepts(
    article: Option<crate::syntax::IndefiniteArticle>,
    onset: crate::features::Onset,
) -> bool {
    matches!(
        (article, onset),
        (None, _)
            | (
                Some(crate::syntax::IndefiniteArticle::A),
                crate::features::Onset::Consonant
            )
            | (
                Some(crate::syntax::IndefiniteArticle::An),
                crate::features::Onset::Vowel
            )
    )
}

deckmaste_constructions_macro::constructions! {
    group determiner;

    construction determiner_closed: Determiner {
        bind Determiner via make_closed, closed_parts {
            identity: identity ClosedDeterminer via Determiner,
        }
        derive features: Features = closed_features(identity);
        evidence feature "determiner cardinality and article agreement" from category;
        form only @ 0 inverse check(is_closed) = identity(identity);
        selection unique;
    }

    construction determiner_target: Determiner {
        bind Determiner via make_target, no_parts {}
        derive features: Features = target_features();
        evidence feature "singular target cardinality" from category;
        form only @ 0 inverse check(is_target) = "target";
        selection unique;
    }

    construction determiner_quantified_target: Determiner {
        bind Determiner via make_quantified_target, quantified_target_parts {
            quantity: hole Quantity,
        }
        derive features: Features = quantified_target_features(quantity);
        evidence feature "quantified target cardinality" from category;
        form only @ 0 inverse check(is_quantified_target) = quantity "target";
        selection unique;
    }

    construction determiner_quantity: Determiner {
        bind Determiner via make_quantity, quantity_parts {
            quantity: hole Quantity,
        }
        derive features: Features = quantity_features(quantity);
        evidence feature "quantity cardinality" from category;
        form only @ 0 inverse check(is_quantity) = quantity;
        selection unique;
    }

    construction determiner_possessive_this_card: Determiner {
        bind Determiner via make_possessive_this_card, possessive_this_card_parts {
            form: identity ThisCardForm via PossessiveThisCard,
        }
        derive features: Features = possessive_features();
        evidence role "self-reference possessor identity" from category;
        form only @ 0 inverse check(is_possessive_this_card) = identity(form);
        selection unique;
    }

    construction possessive_noun_base: PossessiveNominal {
        bind PossessiveNominal via make_possessive_noun_base, possessive_noun_base_parts {
            head: identity NounInstance via PossessiveNoun,
        }
        derive features: Features = possessive_noun_base_features(head);
        evidence feature "noun possessor number and onset" from category;
        form only @ 0 inverse check(is_possessive_noun_base) = identity(head);
        selection unique;
    }

    construction possessive_noun_determined: PossessiveNominal {
        bind PossessiveNominal via make_possessive_noun_determined, possessive_noun_determined_parts {
            determiner: hole Determiner,
            possessor: hole PossessiveNominal,
        }
        derive features: Features = possessive_noun_determined_features(determiner, possessor);
        evidence feature "possessor determination and agreement" from category;
        form only @ 0 inverse check(is_possessive_noun_determined) = determiner possessor;
        selection unique;
    }

    construction determiner_possessive_noun: Determiner {
        bind Determiner via make_determiner_possessive_noun, determiner_possessive_noun_parts {
            possessor: hole PossessiveNominal,
        }
        derive features: Features = possessive_features();
        evidence role "noun possessor determiner" from category;
        form only @ 0 inverse check(is_determiner_possessive_noun) = possessor;
        selection unique;
    }

    construction possessive_noun_adjective: PossessiveNominal {
        bind PossessiveNominal via make_possessive_noun_adjective, possessive_noun_adjective_parts {
            adjective: hole AdjectivePhrase,
            possessor: hole PossessiveNominal,
        }
        derive features: Features = possessive_noun_adjective_features(adjective, possessor);
        evidence feature "adjective-prefixed possessor state" from category;
        form only @ 0 inverse check(is_possessive_noun_adjective) = adjective possessor;
        selection unique;
    }
}

pub(crate) static GROUPS: &[&GroupData] = &[&DETERMINER_DECLARATION];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::Gender;
    use crate::word::Pronoun;

    const D01_IDS: &[&str] = &[
        "determiner_closed",
        "determiner_target",
        "determiner_quantified_target",
        "determiner_quantity",
        "determiner_possessive_this_card",
        "possessive_noun_base",
        "possessive_noun_determined",
        "determiner_possessive_noun",
        "possessive_noun_adjective",
    ];

    #[test]
    fn declaration_ids_are_the_complete_d01_family_in_stable_order() {
        let actual = GROUPS[0]
            .constructions
            .iter()
            .map(|construction| construction.id)
            .collect::<Vec<_>>();
        assert_eq!(actual, D01_IDS);
    }

    #[test]
    fn closed_possessive_pronoun_identity_accepts_exactly_the_five_renderable_classes() {
        let fixtures = [
            (Pronoun::You, Some("your")),
            (Pronoun::It(Gender::Neuter), Some("its")),
            (Pronoun::They, Some("their")),
            (Pronoun::It(Gender::Masculine), Some("his")),
            (Pronoun::It(Gender::Feminine), Some("her")),
            (Pronoun::EachOther, None),
            (Pronoun::Itself, None),
            (Pronoun::Himself, None),
            (Pronoun::YoursAbsolute, None),
        ];
        assert_eq!(fixtures.map(|(pronoun, _)| pronoun), Pronoun::ALL);
        for (pronoun, expected_spelling) in fixtures {
            let built = build_determiner_closed(ClosedDeterminer::PossessivePronoun(pronoun));
            match expected_spelling {
                Some(expected) => {
                    assert_eq!(built.unwrap().render().unwrap(), expected, "{pronoun:?}");
                }
                None => assert!(
                    built.is_err(),
                    "{pronoun:?} has no possessive determiner form"
                ),
            }
        }
    }
}

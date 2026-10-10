use crate::common;
use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{FeatureBundle, LexicalReading, LexicalValue, SurfaceCase, WordForm};
use std::collections::BTreeSet;

fn word(id: &str, frame: Option<usize>) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: id.into(),
            form: WordForm::Invariant,
            features: FeatureBundle::default(),
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame,
        countability: None,
    }
}

fn pro_form() -> Reading {
    Reading::ProFormComplement {
        form: 0,
        head: word("vocab:Adverb/So", None),
    }
}

fn witness(card: &str, text: &str, infinitival: &str) {
    let values = common::readings_on_card(card, text, Category::Document);
    assert!(!values.is_empty(), "missing {card}");
    let expected = Reading::InfinitivalAdjective {
        form: 0,
        head: word("vocab:Adjective/Able", Some(0)),
        complement: Box::new(
            common::readings(infinitival, Category::InfinitiveComplement)
                .into_iter()
                .next()
                .unwrap(),
        ),
    };
    for value in values {
        let mut pro_forms = Vec::new();
        let mut postmodifiers = Vec::new();
        value
            .visit(&mut |node| {
                if let Reading::BareAuxiliaryPredicate {
                    head, complement, ..
                } = node
                {
                    let LexicalReading::Word(lexical) = &head.value else {
                        panic!("auxiliary head must be a Word");
                    };
                    assert_eq!(lexical.lexeme.as_str(), "core-verb:Do");
                    pro_forms.push(complement.as_ref().clone());
                }
                if let Reading::PostpositiveNominal {
                    category: Category::Nominal,
                    modifier,
                    ..
                } = node
                {
                    postmodifiers.push(modifier.as_ref().clone());
                }
            })
            .unwrap();
        assert_eq!(pro_forms, vec![pro_form()]);
        assert_eq!(postmodifiers, vec![expected.clone()]);
    }
}

#[test]
fn card_witnesses_keep_both_complement_roles_and_nominal_modification() {
    for (card, text, infinitival) in [
        (
            "Lure",
            "All creatures able to block enchanted creature do so.",
            "to block enchanted creature",
        ),
        (
            "Mortipede",
            "{2}{G}: All creatures able to block this creature this turn do so.",
            "to block this creature this turn",
        ),
        (
            "Gift of the Deity",
            "As long as enchanted creature is green, it gets +1/+1 and all creatures able to block it do so.",
            "to block it",
        ),
        (
            "Marble Priest",
            "All Walls able to block this creature do so.",
            "to block this creature",
        ),
        (
            "Talruum Piper",
            "All creatures with flying able to block this creature do so.",
            "to block this creature",
        ),
    ] {
        witness(card, text, infinitival);
    }
}

#[test]
fn attested_pro_form_predicates_select_the_complement() {
    // Lure and Camouflage respectively attest these finite constituents.
    for text in ["do so", "does so"] {
        let values = common::readings(text, Category::FinitePredicate);
        assert!(!values.is_empty());
        for value in values {
            let Reading::BareAuxiliaryPredicate { complement, .. } = value else {
                panic!("pro-form predicate must use the auxiliary route");
            };
            assert_eq!(*complement, pro_form());
        }
    }
}

#[test]
fn independently_constructed_pro_form_obeys_both_roundtrip_laws() {
    let value = pro_form();
    value.admit(common::lexicon()).unwrap();
    assert_eq!(value.realize(common::lexicon()).unwrap(), " so");
    assert_eq!(
        common::readings(" so", Category::BareComplement),
        BTreeSet::from([value])
    );
}

#[test]
fn constructed_adjective_selects_the_infinitival_complement() {
    // Marble Priest's infinitival constituent, constructed under the selecting adjective.
    let complements = common::readings("to block this creature", Category::InfinitiveComplement);
    assert!(!complements.is_empty());
    let expected: BTreeSet<_> = complements
        .into_iter()
        .map(|complement| Reading::InfinitivalAdjective {
            form: 0,
            head: word("vocab:Adjective/Able", Some(0)),
            complement: Box::new(complement),
        })
        .collect();
    for value in &expected {
        value.admit(common::lexicon()).unwrap();
        assert_eq!(
            value.realize(common::lexicon()).unwrap(),
            "able to block this creature"
        );
    }
    assert_eq!(
        common::readings("able to block this creature", Category::AdjectivePhrase),
        expected
    );
}

#[test]
fn every_declared_pro_form_licence_is_required() {
    for (id, feature) in [
        ("core-verb:Do", "AuxiliaryEllipsisLicence"),
        ("core-verb:Do", "AuxiliaryProFormLicence"),
        ("vocab:Adverb/So", "ProFormComplementUse"),
    ] {
        let mut lexemes: Vec<_> = common::lexicon().lexemes().values().cloned().collect();
        lexemes
            .iter_mut()
            .find(|lexeme| lexeme.id.as_str() == id)
            .unwrap()
            .properties
            .features
            .insert(feature.into(), "No".into());
        let lexicon = deckmaste_lexical::Lexicon::new(lexemes).unwrap();
        assert!(
            common::readings_with_lexicon(&lexicon, "do so", Category::FinitePredicate).is_empty()
        );
    }
}

#[test]
fn bare_adjectives_and_nonagreeing_or_unlicensed_heads_gain_no_route() {
    for text in [
        "creatures able",
        "creatures able to",
        "creatures able block this creature",
    ] {
        assert!(
            common::readings(text, Category::Nominal).is_empty(),
            "{text}"
        );
    }
    for text in [
        "If you did so, draw a card.",
        "That player do so.",
        "All creatures does so.",
        "You can't so.",
        "You can't all so.",
        "You do and can't so.",
    ] {
        assert!(
            common::readings(text, Category::Document).is_empty(),
            "{text}"
        );
    }
    assert!(common::readings("doing so", Category::SecondaryVerbPhrase).is_empty());
    // A bare passive already licenses this nominal; no adjectival postmodifier is added.
    let values = common::readings("creatures tapped", Category::Nominal);
    assert!(!values.is_empty());
    for value in values {
        let Reading::ParticipialPostmodifiedNominal { modifier, .. } = value else {
            panic!("tapped must remain a verbal postmodifier");
        };
        assert_eq!(
            BTreeSet::from([*modifier]),
            common::readings("tapped", Category::SecondaryVerbPhrase)
        );
    }
}

#[test]
fn existing_ellipsis_and_verbless_conditional_roles_are_preserved() {
    for text in ["If you do, draw a card.", "If a player does, draw a card."] {
        let values = common::readings(text, Category::Document);
        assert!(!values.is_empty());
        for value in values {
            let mut omissions = Vec::new();
            value
                .visit(&mut |node| {
                    if let Reading::BareAuxiliaryPredicate { complement, .. } = node {
                        omissions.push(complement.as_ref().clone());
                    }
                })
                .unwrap();
            assert_eq!(
                omissions,
                vec![Reading::BareEllipsis {
                    form: 0,
                    omission: Box::new(Reading::OmittedPlain { form: 0 }),
                }]
            );
        }
    }
    for text in [
        "Each creature blocks this creature if able.",
        "Target creature attacks this turn if able.",
    ] {
        let values = common::readings(text, Category::Document);
        assert!(!values.is_empty());
        for value in values {
            let mut predicates = Vec::new();
            value
                .visit(&mut |node| {
                    if let Reading::VerblessClause { predicate, .. } = node {
                        predicates.push(predicate.clone());
                    }
                })
                .unwrap();
            assert_eq!(predicates, vec![word("vocab:Adjective/Able", Some(0))]);
        }
    }
}

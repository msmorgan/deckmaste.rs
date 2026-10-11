use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{
    FeatureBundle, LexicalReading, LexicalValue, Number, SurfaceCase, WordForm,
};

use crate::common;

fn word(owner: &str) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form: WordForm::Invariant,
            features: FeatureBundle::default(),
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    }
}

fn degree() -> Reading {
    Reading::DegreeDeterminativePhrase {
        form: 0,
        arithmetic_modifier: None,
        modifier: word("vocab:SingularDemonstrative/That"),
        head: word("vocab:Determinative/Many"),
    }
}

fn bounded(initial: bool) -> Reading {
    let mut up = word("vocab:Preposition/Up");
    if initial {
        let LexicalReading::Word(ref mut lexical) = up.value else { unreachable!() };
        lexical.capitalization = SurfaceCase::Initial;
    }
    Reading::PrepositionDeterminer {
        form: 1,
        value: Box::new(Reading::QuantitativePrepositionPhrase {
            category: Category::QuantitativeDegreePrepositionPhrase,
            form: 0,
            head: up,
            complement: Box::new(Reading::QuantitativePrepositionPhrase {
                category: Category::DegreePrepositionPhrase,
                form: 0,
                head: word("vocab:Preposition/To"),
                complement: Box::new(degree()),
            }),
        }),
    }
}

fn noun(owner: &str, number: Number) -> Word {
    let mut head = word(owner);
    let LexicalReading::Word(ref mut lexical) = head.value else { unreachable!() };
    lexical.form = if number == Number::Plural { WordForm::Plural } else { WordForm::Singular };
    lexical.features.number = Some(number);
    head.countability = Some(true);
    head
}

#[test]
fn witnesses_use_a_bounded_degree_determiner_in_each_nominal_position() {
    for (card, text, noun_phrase, nominal, initial) in [
        (
            "Cephalid Constable",
            "Whenever this creature deals combat damage to a player, return up to that many target permanents that player controls to their owner's hand.",
            "up to that many target permanents that player controls",
            "target permanents that player controls",
            false,
        ),
        (
            "Heroic Feast",
            "Whenever you gain life, choose up to that many target creatures you control.",
            "up to that many target creatures you control",
            "target creatures you control",
            false,
        ),
        (
            "Swashbuckler Extraordinaire",
            "When you do, up to that many target creatures gain double strike until end of turn.",
            "up to that many target creatures",
            "target creatures",
            false,
        ),
        (
            "Scapeshift",
            "Search your library for up to that many land cards, put them onto the battlefield tapped, then shuffle.",
            "up to that many land cards",
            "land cards",
            false,
        ),
        (
            "Swashbuckler Extraordinaire (main clause)",
            "Up to that many target creatures gain double strike until end of turn.",
            "Up to that many target creatures",
            "target creatures",
            true,
        ),
    ] {
        common::assert_constituents(
            text,
            Category::Sentence,
            &[
                (Category::NounPhrase, noun_phrase),
                (Category::Nominal, nominal),
                (Category::QuantitativeDeterminer, "that many"),
                (Category::DegreePrepositionPhrase, "to that many"),
                (
                    Category::QuantitativeDegreePrepositionPhrase,
                    if initial { "Up to that many" } else { "up to that many" },
                ),
            ],
        );
        let values = common::readings(text, Category::Sentence);
        assert!(!values.is_empty(), "{card}");
        for value in values {
            let mut quantities = Vec::new();
            value
                .visit(&mut |node| {
                    if let Reading::QuantifiedNounPhrase { quantity, .. } = node
                        && quantity.realize(common::lexicon()).unwrap()
                            == if initial { "Up to that many" } else { "up to that many" }
                    {
                        quantities.push((**quantity).clone());
                    }
                })
                .unwrap();
            assert_eq!(quantities, vec![bounded(initial)], "{card}");
        }
    }
}

#[test]
fn independent_bounded_degree_values_preserve_both_roundtrip_laws_and_traversal() {
    // Attested constituents of Heroic Feast and Scapeshift.
    let creatures = Reading::TargetedNominal {
        form: 0,
        marker: word("vocab:TargetingMarker/Target"),
        head: Box::new(Reading::Noun {
            form: 0,
            head: noun("lexeme:type/creature", Number::Plural),
        }),
    };
    let cards = Reading::NounPremodifiedNominal {
        form: 0,
        modifier: Box::new(Reading::NounPremodifier {
            form: 0,
            head: noun("lexeme:type/land", Number::Singular),
        }),
        head: Box::new(Reading::Noun {
            form: 0,
            head: noun("lexeme:CommonNoun/Card", Number::Plural),
        }),
    };
    for (head, text) in [
        (creatures, "up to that many target creatures"),
        (cards, "up to that many land cards"),
    ] {
        let value = Reading::QuantifiedNounPhrase {
            form: 0,
            quantity: Box::new(bounded(false)),
            head: Box::new(head),
        };
        value.admit(common::lexicon()).unwrap();
        assert_eq!(value.realize(common::lexicon()).unwrap(), text);
        let parsed = common::readings(text, Category::NounPhrase);
        assert_eq!(parsed, BTreeSet::from([value.clone()]));
        let observed = parsed.get(&value).unwrap();
        let mut expected_nodes = Vec::new();
        let mut actual_nodes = Vec::new();
        value
            .visit(&mut |node| expected_nodes.push(node.clone()))
            .unwrap();
        observed
            .visit(&mut |node| actual_nodes.push(node.clone()))
            .unwrap();
        assert_eq!(actual_nodes, expected_nodes);
        let mut expected_words = Vec::new();
        let mut actual_words = Vec::new();
        value
            .visit_words(&mut |word| expected_words.push(word.clone()))
            .unwrap();
        observed
            .visit_words(&mut |word| actual_words.push(word.clone()))
            .unwrap();
        assert_eq!(actual_words, expected_words);
    }
}

#[test]
fn bounded_degree_has_no_goal_pp_or_demonstrative_determiner_route() {
    let text = "up to that many target creatures";
    assert!(common::readings(text, Category::PrepositionPhrase).is_empty());
    assert!(common::readings("many target creatures", Category::Nominal).is_empty());
    let values = common::readings(text, Category::NounPhrase);
    assert!(!values.is_empty());
    for value in values {
        let Reading::QuantifiedNounPhrase { quantity, .. } = value else {
            panic!("the bounded phrase must determine the Nominal")
        };
        assert_eq!(*quantity, bounded(false));
    }
}

#[test]
fn bounded_degree_preserves_head_selection_and_subject_agreement() {
    // Altered constituents and agreement of the pinned witnesses.
    for text in [
        "up to that many target creature",
        "up to that many life",
        "up to those many target creatures",
        "up to that all target creatures",
        "up to up to that many target creatures",
        "to that many target creatures",
    ] {
        assert!(
            common::readings(text, Category::NounPhrase).is_empty(),
            "{text}"
        );
    }
    assert!(
        common::readings(
            "Up to that many target creatures gains double strike until end of turn.",
            Category::Sentence,
        )
        .is_empty()
    );
    let invalid = Reading::QuantifiedNounPhrase {
        form: 0,
        quantity: Box::new(bounded(false)),
        head: Box::new(Reading::Noun {
            form: 0,
            head: noun("lexeme:type/creature", Number::Singular),
        }),
    };
    assert!(invalid.admit(common::lexicon()).is_err());
    assert!(invalid.realize(common::lexicon()).is_err());
}

#[test]
fn unbounded_degree_and_cardinal_controls_keep_their_constituent_roles() {
    for (text, quantity) in [
        (
            "Return that many target permanents that player controls to their owner's hand.",
            "that many",
        ),
        (
            "Return up to two target permanents that player controls to their owner's hand.",
            "up to two",
        ),
        ("Search your library for that many land cards.", "that many"),
        ("Search your library for up to two land cards.", "up to two"),
        (
            "That many target creatures gain double strike until end of turn.",
            "That many",
        ),
    ] {
        common::assert_constituents(
            text,
            Category::Sentence,
            &[(Category::QuantitativeDeterminer, quantity)],
        );
    }
}

use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, FrameValue, Reading, Word};
use deckmaste_lexical::{
    FeatureBundle, Finiteness, LexicalReading, LexicalValue, Number, Numeral, Person, SurfaceCase,
    Tense, WordForm,
};

use crate::common::{lexicon, readings, readings_on_card};

const OWNER: &str = "lexeme:keyword_action/waterbend";

fn waterbend() -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: OWNER.into(),
            form: WordForm::Plain,
            features: FeatureBundle::default(),
            variant: 0,
            capitalization: SurfaceCase::Initial,
        }),
        frame: Some(1),
        countability: None,
    }
}

fn symbols() -> Reading {
    Reading::SymbolSequence {
        category: Category::CostSymbols,
        form: 0,
        first: Box::new(Reading::NumericCostSymbol {
            form: 0,
            number: Word {
                value: LexicalReading::Numeral {
                    value: 3,
                    notation: Numeral::Arabic(false),
                    capitalization: SurfaceCase::Declared,
                },
                frame: None,
                countability: None,
            },
        }),
        rest: vec![],
    }
}

#[test]
fn independently_constructed_action_cost_has_a_cost_symbol_complement() {
    // Giant Koi's cost constituent, independently constructed.
    let expected = Reading::Cost {
        form: 0,
        first: Box::new(Reading::ActionCost {
            form: 0,
            action: Box::new(Reading::BarePredicate {
                form: 0,
                head: Box::new(Reading::SelectedPredicate {
                    category: Category::SecondaryVerbPhrase,
                    form: 0,
                    head: waterbend(),
                    complements: vec![FrameValue::Argument(Box::new(symbols()))],
                }),
            }),
        }),
        rest: vec![],
    };
    expected.admit(lexicon()).unwrap();
    assert_eq!(expected.realize(lexicon()).unwrap(), "Waterbend {3}");
    assert_eq!(
        readings("Waterbend {3}", Category::Cost),
        BTreeSet::from([expected])
    );
}

#[test]
fn oracle_cost_and_predicate_uses_keep_the_selected_complement() {
    for (card, text, complement, action_cost) in [
        (
            "Giant Koi",
            "Waterbend {3}: This creature can't be blocked this turn.",
            "{3}",
            true,
        ),
        (
            "Katara, Bending Prodigy",
            "Waterbend {6}: Draw a card.",
            "{6}",
            true,
        ),
        (
            "Water Whip",
            "As an additional cost to cast this spell, waterbend {5}.",
            "{5}",
            false,
        ),
        (
            "Spirit Water Revival",
            "As an additional cost to cast this spell, you may waterbend {6}.",
            "{6}",
            false,
        ),
        (
            "Waterbending Lesson",
            "Draw three cards. Then discard a card unless you waterbend {2}.",
            "{2}",
            false,
        ),
        (
            "The Unagi of Kyoshi Island",
            "Ward—Waterbend {4}.",
            "{4}",
            true,
        ),
        (
            "Hama, the Bloodbender",
            "For as long as you control Hama, you may cast the exiled card during your turn by waterbending {X} rather than paying its mana cost, where X is its mana value.",
            "{X}",
            false,
        ),
    ] {
        let values = readings_on_card(card, text, Category::Document);
        assert!(!values.is_empty(), "{card}");
        let expected = readings(complement, Category::CostSymbols);
        for value in values {
            let mut complements = vec![];
            let mut costs = vec![];
            value
                .visit(&mut |node| {
                    if let Reading::SelectedPredicate {
                        head,
                        complements: arguments,
                        ..
                    } = node
                        && let LexicalReading::Word(word) = &head.value
                        && word.lexeme.as_str() == OWNER
                    {
                        let [FrameValue::Argument(argument)] = arguments.as_slice() else {
                            panic!("{card}: waterbend requires its selected symbols");
                        };
                        complements.push(argument.as_ref().clone());
                    }
                    if let Reading::ActionCost { action, .. } = node
                        && action
                            .realize(lexicon())
                            .unwrap()
                            .to_lowercase()
                            .starts_with("waterbend ")
                    {
                        costs.push(action.as_ref().clone());
                    }
                })
                .unwrap();
            assert!(!complements.is_empty(), "{card}: missing verb head");
            for actual in complements {
                assert_eq!(actual.category(), Category::CostSymbols);
                assert!(expected.contains(&actual), "{card}: wrong Complement");
            }
            assert_eq!(
                !costs.is_empty(),
                action_cost,
                "{card}: Cost component role"
            );
        }
    }
}

#[test]
fn ordinary_paradigm_is_declared_and_roundtrips() {
    // Owner ruling 2026-10-10: retain the whole regular paradigm, including
    // the row's regular waterbended past forms, independently of attestation.
    for (surface, form, features) in [
        ("waterbend", WordForm::Plain, FeatureBundle::default()),
        (
            "waterbends",
            WordForm::Present,
            FeatureBundle {
                number: Some(Number::Singular),
                person: Some(Person::Third),
                tense: Some(Tense::Present),
                finiteness: Some(Finiteness::Finite),
                ..FeatureBundle::default()
            },
        ),
        (
            "waterbending",
            WordForm::GerundParticiple,
            FeatureBundle {
                finiteness: Some(Finiteness::Nonfinite),
                ..FeatureBundle::default()
            },
        ),
        (
            "waterbended",
            WordForm::Preterite,
            FeatureBundle {
                number: Some(Number::Singular),
                person: Some(Person::Third),
                tense: Some(Tense::Past),
                finiteness: Some(Finiteness::Finite),
                ..FeatureBundle::default()
            },
        ),
        (
            "waterbended",
            WordForm::PastParticiple,
            FeatureBundle {
                finiteness: Some(Finiteness::Nonfinite),
                ..FeatureBundle::default()
            },
        ),
    ] {
        let expected = LexicalReading::Word(LexicalValue {
            lexeme: OWNER.into(),
            form,
            features,
            variant: 0,
            capitalization: SurfaceCase::Declared,
        });
        assert_eq!(lexicon().realize(&expected).unwrap(), surface);
        assert!(
            lexicon()
                .analyze(surface)
                .matches
                .iter()
                .any(|m| m.reading == expected)
        );
    }
}

#[test]
fn bare_frame_is_retained_and_the_fixed_term_route_is_retired() {
    // Avatar Aang attests the bare verb in its coordinated trigger.
    let mut head = waterbend();
    head.frame = Some(0);
    let LexicalReading::Word(word) = &mut head.value else { unreachable!() };
    word.capitalization = SurfaceCase::Declared;
    let expected = Reading::SelectedPredicate {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head,
        complements: vec![],
    };
    expected.admit(lexicon()).unwrap();
    assert_eq!(expected.realize(lexicon()).unwrap(), "waterbend");
    assert_eq!(
        readings("waterbend", Category::SecondaryVerbPhrase),
        BTreeSet::from([expected])
    );
    assert!(readings("waterbend", Category::NounPhrase).is_empty());
    assert!(readings("Waterbend a card", Category::BarePredicate).is_empty());
}

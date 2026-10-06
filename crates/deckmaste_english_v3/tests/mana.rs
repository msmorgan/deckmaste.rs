mod common;

use std::collections::BTreeSet;

use common::LEXICON;
use common::readings;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::Finiteness;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::Tense;
use deckmaste_lexical::WordForm;

fn word(owner: &str, form: WordForm, features: FeatureBundle) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form,
            features,
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    }
}

fn symbol(name: &str) -> Reading {
    Reading::NamedManaSymbol {
        form: 0,
        symbol: word(
            &format!("vocab:FixedCostSymbol/{name}"),
            WordForm::Invariant,
            FeatureBundle::default(),
        ),
    }
}

fn mana(first: &str, rest: &[&str]) -> Reading {
    Reading::ManaPhrase {
        form: 0,
        first: Box::new(symbol(first)),
        rest: rest.iter().map(|name| symbol(name)).collect(),
    }
}

fn finite_head(name: &str) -> Word {
    let mut head = word(
        &format!("core-verb:{name}"),
        WordForm::Present,
        FeatureBundle {
            number: Some(Number::Singular),
            person: Some(Person::Third),
            tense: Some(Tense::Present),
            finiteness: Some(Finiteness::Finite),
            case: None,
        },
    );
    head.frame = Some(1);
    head
}

#[test]
fn independent_mana_values_preserve_symbol_sequence() {
    for (text, expected) in [
        ("{W}", mana("White", &[])),
        (
            "{U}{B}{R}{G}{C}",
            mana("Blue", &["Black", "Red", "Green", "Colorless"]),
        ),
    ] {
        assert_eq!(expected.realize(&LEXICON).unwrap(), text);
        assert_eq!(
            readings(text, Category::ManaPhrase),
            BTreeSet::from([expected])
        );
    }
    for text in [
        "", "{T}", "{Q}", "{E}", "{H}", "{X}", "{S}", "{2}", "{W}{T}", "{W/U}", "{W} {U}",
    ] {
        assert_eq!(readings(text, Category::ManaPhrase).len(), 0, "{text}");
    }
    for name in ["Tap", "Untap", "E", "H", "Variable", "Snow"] {
        assert!(mana(name, &[]).admit(&LEXICON).is_err(), "{name}");
    }
    assert_eq!(readings("{T}", Category::CostSymbols).len(), 1);
}

#[test]
fn independent_finite_and_secondary_complements_use_declared_frame() {
    for name in ["Add", "Pay"] {
        let expected = Reading::SelectedPredicate {
            category: Category::FinitePredicate,
            form: 0,
            head: finite_head(name),
            complements: vec![deckmaste_english_v3::grammar::FrameValue::Argument(
                Box::new(mana("Green", &[])),
            )],
        };
        let text = format!("{}s {{G}}", name.to_lowercase());
        assert_eq!(expected.realize(&LEXICON).unwrap(), text);
        assert!(readings(&text, Category::FinitePredicate).contains(&expected));
        let mut head = word(
            &format!("core-verb:{name}"),
            WordForm::Plain,
            FeatureBundle {
                finiteness: None,
                ..Default::default()
            },
        );
        head.frame = Some(1);
        let expected = Reading::SelectedPredicate {
            category: Category::SecondaryVerbPhrase,
            form: 0,
            head,
            complements: vec![deckmaste_english_v3::grammar::FrameValue::Argument(
                Box::new(mana("Green", &[])),
            )],
        };
        let text = format!("{} {{G}}", name.to_lowercase());
        assert_eq!(expected.realize(&LEXICON).unwrap(), text);
        assert!(readings(&text, Category::SecondaryVerbPhrase).contains(&expected));
    }
    let mut wrong = finite_head("Add");
    wrong.frame = Some(0);
    assert!(
        Reading::SelectedPredicate {
            category: Category::FinitePredicate,
            form: 0,
            head: wrong,
            complements: vec![deckmaste_english_v3::grammar::FrameValue::Argument(
                Box::new(mana("Green", &[]))
            )]
        }
        .admit(&LEXICON)
        .is_err()
    );
    assert_eq!(readings("draws {G}", Category::FinitePredicate).len(), 0);
    assert_eq!(readings("add {T}", Category::SecondaryVerbPhrase).len(), 0);
}

#[test]
fn authentic_basic_mana_abilities_have_structured_complements() {
    // Ancient Den, Mox Jet, Llanowar Elves, Sol Ring, and Wastes.
    for text in [
        "{T}: Add {W}.",
        "{T}: Add {B}.",
        "{T}: Add {G}.",
        "{T}: Add {C}{C}.",
        "{T}: Add {C}.",
    ] {
        let values = readings(text, Category::Document);
        assert_eq!(values.len(), 1, "{text}");
        assert!(values.iter().all(|reading| {
            let mut found = false;
            reading
                .visit(&mut |node| {
                    found |= matches!(
                        node,
                        Reading::SelectedPredicate {
                            category: Category::SecondaryVerbPhrase,
                            form: 0,
                            complements, ..
                        } if matches!(complements.as_slice(), [deckmaste_english_v3::grammar::FrameValue::Argument(child)] if child.category() == Category::ManaPhrase)
                    );
                })
                .unwrap();
            found
        }));
    }
}

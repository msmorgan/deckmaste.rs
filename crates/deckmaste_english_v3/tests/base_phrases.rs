mod common;

use common::lexicon;
use common::readings;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::Numeral;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

fn word(id: &str, form: WordForm, number: Option<Number>) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: id.into(),
            form,
            features: FeatureBundle {
                number,
                ..FeatureBundle::default()
            },
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        countability: None,
        frame: None,
    }
}
fn numeral(value: i32, notation: Numeral) -> Word {
    Word {
        value: LexicalReading::Numeral {
            value,
            notation,
            capitalization: SurfaceCase::Declared,
        },
        countability: None,
        frame: None,
    }
}
fn assert_laws(value: &Reading, text: &str, category: Category) {
    value.admit(lexicon()).unwrap();
    assert_eq!(value.realize(lexicon()).unwrap(), text);
    let parsed = readings(text, category);
    assert!(
        parsed.contains(value),
        "independent value missing for {text:?}"
    );
    let reparsed = parsed.get(value).unwrap();
    let mut nodes = Vec::new();
    value.visit(&mut |node| nodes.push(node.clone())).unwrap();
    let mut parsed_nodes = Vec::new();
    reparsed
        .visit(&mut |node| parsed_nodes.push(node.clone()))
        .unwrap();
    assert_eq!(nodes, parsed_nodes);
    let mut words = Vec::new();
    value
        .visit_words(&mut |word| words.push(word.clone()))
        .unwrap();
    let mut parsed_words = Vec::new();
    reparsed
        .visit_words(&mut |word| parsed_words.push(word.clone()))
        .unwrap();
    assert_eq!(words, parsed_words);
}

#[test]
fn independent_uncoordinated_phrase_primitives_roundtrip() {
    let mut creature = word(
        "lexeme:type/creature",
        WordForm::Plural,
        Some(Number::Plural),
    );
    creature.countability = Some(true);
    let nominal = Reading::Noun {
        form: 0,
        head: creature,
    };
    let np = Reading::BarePlural {
        form: 0,
        head: Box::new(nominal.clone()),
    };
    let pp = Reading::PrepositionPhrase {
        form: 0,
        head: word("vocab:Preposition/Among", WordForm::Invariant, None),
        complement: Box::new(Reading::CasePhrase {
            category: Category::AccusativePhrase,
            form: 0,
            head: Box::new(np.clone()),
        }),
    };
    let cardinal = Reading::Cardinal {
        form: 0,
        head: numeral(3, Numeral::Cardinal),
    };
    let amount = Reading::ScalarAmount {
        form: 0,
        value: Box::new(Reading::SmallUnsignedScalar {
            form: 0,
            head: numeral(3, Numeral::Arabic(false)),
        }),
    };
    let measure = Reading::UngroupedScalarNumeral {
        form: 0,
        head: numeral(3, Numeral::Arabic(false)),
    };
    let mana = Reading::ManaPhrase {
        form: 0,
        first: Box::new(Reading::NamedManaSymbol {
            form: 0,
            symbol: word("vocab:FixedCostSymbol/Green", WordForm::Invariant, None),
        }),
        rest: vec![],
    };
    let keyword = Reading::BareKeyword {
        form: 0,
        head: word("lexeme:keyword_ability/flying", WordForm::Invariant, None),
    };
    let ap = Reading::Adjective {
        form: 0,
        head: word("vocab:ColorWord/Green", WordForm::Invariant, None),
    };
    let adverb = Reading::Adverb {
        form: 0,
        head: word("vocab:Adverb/Again", WordForm::Invariant, None),
    };
    for (value, text, category) in [
        (nominal, "creatures", Category::Nominal),
        (np, "creatures", Category::NounPhrase),
        (ap, "green", Category::AdjectivePhrase),
        (pp, "among creatures", Category::PrepositionPhrase),
        (cardinal, "three", Category::Cardinal),
        (amount, "3", Category::Amount),
        (measure, "3", Category::MeasurePhrase),
        (mana, "{G}", Category::ManaPhrase),
        (keyword, "flying", Category::KeywordPhrase),
        (adverb, "again", Category::AdverbPhrase),
    ] {
        assert_laws(&value, text, category);
    }
}

#[test]
fn selected_infinitive_is_a_whole_bare_predicate_and_not_a_sentence() {
    let mut attack = word("core-verb:Attack", WordForm::Plain, None);
    attack.frame = Some(0);
    let infinitive = Reading::ToInfinitive {
        form: 0,
        marker: word("vocab:InfinitivalMarker/To", WordForm::Invariant, None),
        predicate: Box::new(Reading::BarePredicate {
            form: 0,
            head: Box::new(Reading::IntransitivePredicate {
                category: Category::SecondaryVerbPhrase,
                form: 0,
                head: attack,
            }),
        }),
    };
    assert_laws(
        &infinitive.clone(),
        "to attack",
        Category::InfinitiveComplement,
    );
    let mut choose = word("core-verb:Choose", WordForm::Plain, None);
    choose.frame = Some(2);
    let predicate = Reading::InfinitivePredicate {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head: choose.clone(),
        complement: Box::new(infinitive.clone()),
    };
    assert_laws(
        &predicate,
        "choose to attack",
        Category::SecondaryVerbPhrase,
    );
    choose.frame = Some(0);
    assert!(
        Reading::InfinitivePredicate {
            category: Category::SecondaryVerbPhrase,
            form: 0,
            head: choose,
            complement: Box::new(infinitive)
        }
        .admit(lexicon())
        .is_err()
    );
    for text in ["To attack.", "Choose to attacks.", "Choose from attack."] {
        assert!(
            readings(text, Category::Document).is_empty(),
            "invalid fragment for {text:?}"
        );
    }
}

#[test]
fn oracle_adverb_constituents_keep_declared_positions() {
    // Jadelight Ranger, Raging Kronch, and Repeat Offender constituents.
    for text in [
        "it explores again",
        "this creature can't attack alone",
        "Otherwise, suspect it",
        "Then draw a card",
    ] {
        assert!(
            !readings(text, Category::Clause).is_empty(),
            "no Reading for {text:?}"
        );
    }
}

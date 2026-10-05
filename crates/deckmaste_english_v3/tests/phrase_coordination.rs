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
fn assert_laws(value: Reading, text: &str, category: Category) {
    value.admit(lexicon()).unwrap();
    assert_eq!(value.realize(lexicon()).unwrap(), text);
    let parsed = readings(text, category);
    assert!(
        parsed.contains(&value),
        "independent value missing for {text:?}"
    );
    let reparsed = parsed.get(&value).unwrap();
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

macro_rules! check_family {
    ($binary:ident, $end:ident, $serial:ident, $series:ident, $base:expr, $text:expr, $category:expr) => {{
        let base = $base;
        let coordinator = word("vocab:Coordinator/And", WordForm::Invariant, None);
        let binary = Reading::$binary {
            category: $category,
            form: 0,
            left: Box::new(base.clone()),
            coordinator: coordinator.clone(),
            right: Box::new(base.clone()),
        };
        assert_laws(binary, &format!("{} and {}", $text, $text), $category);
        let end = Reading::$end {
            category: Category::$series,
            form: 0,
            left: Box::new(base.clone()),
            coordinator,
            right: Box::new(base.clone()),
        };
        let serial = Reading::$serial {
            category: $category,
            form: 0,
            left: Box::new(base),
            rest: Box::new(end),
        };
        assert_laws(
            serial,
            &format!("{}, {}, and {}", $text, $text, $text),
            $category,
        );
        assert!(readings(&format!("{}, {} and {}", $text, $text, $text), $category).is_empty());
    }};
}

#[test]
fn independent_phrase_coordinations_preserve_structure_and_oxford_commas() {
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
    let pp = Reading::PrepositionPhrase {
        form: 0,
        head: word("vocab:Preposition/Among", WordForm::Invariant, None),
        complement: Box::new(Reading::CasePhrase {
            category: Category::AccusativePhrase,
            form: 0,
            head: Box::new(Reading::BarePlural {
                form: 0,
                head: Box::new(nominal.clone()),
            }),
        }),
    };
    let ap = Reading::Adjective {
        form: 0,
        head: word("vocab:ColorWord/Green", WordForm::Invariant, None),
    };
    let adv = Reading::Adverb {
        form: 0,
        head: word("vocab:Adverb/Again", WordForm::Invariant, None),
    };
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
    let mut time = word(
        "lexeme:CommonNoun/Time",
        WordForm::Plural,
        Some(Number::Plural),
    );
    time.countability = Some(true);
    let frequency = Reading::CountedFrequency {
        form: 0,
        quantity: Box::new(Reading::Cardinal {
            form: 0,
            head: numeral(3, Numeral::Cardinal),
        }),
        head: time,
    };
    check_family!(
        Coordination,
        CoordinationSeriesEnd,
        SerialCoordination,
        NominalSeries,
        nominal,
        "creatures",
        Category::Nominal
    );
    check_family!(
        Coordination,
        CoordinationSeriesEnd,
        SerialCoordination,
        AdjectivePhraseSeries,
        ap,
        "green",
        Category::AdjectivePhrase
    );
    check_family!(
        Coordination,
        CoordinationSeriesEnd,
        SerialCoordination,
        PrepositionPhraseSeries,
        pp,
        "among creatures",
        Category::PrepositionPhrase
    );
    check_family!(
        Coordination,
        CoordinationSeriesEnd,
        SerialCoordination,
        AdverbPhraseSeries,
        adv,
        "again",
        Category::AdverbPhrase
    );
    check_family!(
        Coordination,
        CoordinationSeriesEnd,
        SerialCoordination,
        InfinitiveComplementSeries,
        infinitive,
        "to attack",
        Category::InfinitiveComplement
    );
    check_family!(
        Coordination,
        CoordinationSeriesEnd,
        SerialCoordination,
        FrequencyPhraseSeries,
        frequency,
        "three times",
        Category::FrequencyPhrase
    );
}

#[test]
fn authentic_shared_modifiers_and_phrase_positions_compose() {
    // Revoke Existence and a Wallop constituent, followed by synthetic
    // combinations that exercise independently available PP/infinitive bases.
    for (text, category) in [
        ("Exile target artifact or enchantment.", Category::Document),
        ("target blue or black creature", Category::NounPhrase),
        ("from them or from you", Category::PrepositionPhrase),
        (
            "choose to attack or to block",
            Category::SecondaryVerbPhrase,
        ),
    ] {
        assert!(
            !readings(text, category).is_empty(),
            "no Reading for {text:?}"
        );
    }
    // Every coordinate must license the surrounding adjunct function.
    assert!(
        !readings(
            "of creatures and among creatures",
            Category::PrepositionPhrase
        )
        .is_empty()
    );
    assert!(
        readings(
            "attack of creatures and among creatures",
            Category::SecondaryVerbPhrase
        )
        .is_empty()
    );
    assert!(readings("creature or creatures", Category::Nominal).is_empty());
    assert!(readings("to attacks or to block", Category::InfinitiveComplement).is_empty());
}

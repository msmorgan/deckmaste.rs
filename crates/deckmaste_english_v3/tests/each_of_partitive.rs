mod common;

use std::collections::BTreeSet;

use common::assert_constituents;
use common::lexicon;
use common::readings;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::FrameValue;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::Case;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::Finiteness;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::Numeral;
use deckmaste_lexical::Person;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::Tense;
use deckmaste_lexical::WordForm;

fn witness(card: &str, text: &str, partitive: &str, oblique: &str) {
    assert!(!readings(text, Category::Document).is_empty(), "{card}");
    assert_constituents(
        text,
        Category::Document,
        &[
            (Category::NounPhrase, partitive),
            (Category::PrepositionPhrase, &format!("of {oblique}")),
            (Category::AccusativePhrase, oblique),
        ],
    );
}

macro_rules! witnesses {
    ($($name:ident: ($card:literal, $text:literal, $partitive:literal, $oblique:literal)),* $(,)?) => {
        $(#[test] fn $name() { witness($card, $text, $partitive, $oblique); })*
    };
}

witnesses! {
    succumb_to_the_cold: ("Succumb to the Cold", "Put a stun counter on each of them.", "each of them", "them"),
    involuntary_cooldown: ("Involuntary Cooldown", "Put two stun counters on each of them.", "each of them", "them"),
    reap_what_is_sown: ("Reap What Is Sown", "Put a +1/+1 counter on each of up to three target creatures.", "each of up to three target creatures", "up to three target creatures"),
    felidar_savior: ("Felidar Savior", "When this creature enters, put a +1/+1 counter on each of up to two other target creatures you control.", "each of up to two other target creatures you control", "up to two other target creatures you control"),
    exploration: ("Exploration", "You may play an additional land on each of your turns.", "each of your turns", "your turns"),
    absolute_virtue: ("Absolute Virtue", "You have protection from each of your opponents.", "each of your opponents", "your opponents"),
}

fn word(id: &str) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: id.into(),
            form: WordForm::Invariant,
            features: FeatureBundle::default(),
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    }
}

fn partitive(capitalization: SurfaceCase) -> Reading {
    let mut head = word("vocab:FloatedQuantifier/Each");
    let LexicalReading::Word(value) = &mut head.value else { unreachable!() };
    value.capitalization = capitalization;
    let mut them = word("vocab:ObjectPronoun/Them");
    let LexicalReading::Word(value) = &mut them.value else { unreachable!() };
    value.features = FeatureBundle {
        number: Some(Number::Plural),
        person: Some(Person::Third),
        case: Some(Case::Accusative),
        ..Default::default()
    };
    Reading::DeterminerHeadPartitiveNounPhrase {
        form: 0,
        head,
        complement: Box::new(Reading::PrepositionPhrase {
            form: 0,
            head: word("vocab:Preposition/Of"),
            complement: Box::new(Reading::CasePhrase {
                form: 0,
                category: Category::AccusativePhrase,
                head: Box::new(Reading::AccusativePronoun {
                    form: 0,
                    head: them,
                }),
            }),
        }),
        modifiers: vec![],
    }
}

fn assert_laws(value: &Reading, text: &str, category: Category) {
    value.admit(lexicon()).unwrap();
    assert_eq!(value.realize(lexicon()).unwrap(), text);
    let parsed = readings(text, category);
    assert_eq!(parsed, BTreeSet::from([value.clone()]));
    let observed = parsed.get(value).unwrap();
    let mut nodes = vec![];
    let mut observed_nodes = vec![];
    value.visit(&mut |node| nodes.push(node.clone())).unwrap();
    observed
        .visit(&mut |node| observed_nodes.push(node.clone()))
        .unwrap();
    assert_eq!(observed_nodes, nodes);
    let mut words = vec![];
    let mut observed_words = vec![];
    value
        .visit_words(&mut |word| words.push(word.clone()))
        .unwrap();
    observed
        .visit_words(&mut |word| observed_words.push(word.clone()))
        .unwrap();
    assert_eq!(observed_words, words);
}

#[test]
fn independent_fused_head_has_one_reading_and_exact_lexical_owners() {
    // Succumb to the Cold and Hope and Glory attest these constituents.
    for (capitalization, text) in [
        (SurfaceCase::Declared, "each of them"),
        (SurfaceCase::Initial, "Each of them"),
    ] {
        let value = partitive(capitalization);
        assert_laws(&value, text, Category::NounPhrase);
        let mut owners = vec![];
        value
            .visit_words(&mut |word| {
                let LexicalReading::Word(value) = &word.value else { unreachable!() };
                owners.push(value.lexeme.to_string());
            })
            .unwrap();
        assert_eq!(
            owners,
            [
                "vocab:FloatedQuantifier/Each",
                "vocab:Preposition/Of",
                "vocab:ObjectPronoun/Them"
            ]
        );
    }
}

fn agreement_clause(number: Number) -> Reading {
    let mut head = word("core-verb:Get");
    head.frame = Some(0);
    let LexicalReading::Word(value) = &mut head.value else { unreachable!() };
    value.form = WordForm::Present;
    value.features = FeatureBundle {
        number: Some(number),
        person: Some(Person::Third),
        tense: Some(Tense::Present),
        finiteness: Some(Finiteness::Finite),
        ..Default::default()
    };
    let component = || Reading::PositiveScalar {
        form: 0,
        value: Box::new(Reading::SmallUnsignedScalar {
            form: 0,
            head: Word {
                value: LexicalReading::Numeral {
                    value: 1,
                    notation: Numeral::Arabic(false),
                    capitalization: SurfaceCase::Declared,
                },
                frame: None,
                countability: None,
            },
        }),
    };
    Reading::Declarative {
        form: 0,
        clause: Box::new(Reading::FiniteClause {
            form: 0,
            subject: Box::new(Reading::CasePhrase {
                form: 0,
                category: Category::NominativePhrase,
                head: Box::new(partitive(SurfaceCase::Initial)),
            }),
            predicate: Box::new(Reading::SelectedPredicate {
                form: 0,
                category: Category::FinitePredicate,
                head,
                complements: vec![FrameValue::Argument(Box::new(Reading::SlashPair {
                    form: 0,
                    left: Box::new(component()),
                    right: Box::new(component()),
                }))],
            }),
        }),
    }
}

#[test]
fn fused_head_controls_singular_agreement_independently_of_plural_oblique() {
    // The initial Clause of Hope and Glory's second sentence.
    assert_laws(
        &agreement_clause(Number::Singular),
        "Each of them gets +1/+1",
        Category::Clause,
    );
    let plural = agreement_clause(Number::Plural);
    assert!(plural.admit(lexicon()).is_err());
    assert!(plural.realize(lexicon()).is_err());
    assert!(readings("Each of them get +1/+1", Category::Clause).is_empty());
}

#[test]
fn unlicensed_fused_heads_markers_and_singular_obliques_are_rejected() {
    for text in [
        "every of them",
        "the of them",
        "each among them",
        "each of it",
        "each of a creature",
        "each of target creature",
    ] {
        assert!(readings(text, Category::NounPhrase).is_empty(), "{text}");
    }
    for text in ["each creature", "one of them"] {
        assert_eq!(readings(text, Category::NounPhrase).len(), 1, "{text}");
    }
}

#[test]
fn attested_object_and_preposition_positions_share_the_fused_head_structure() {
    // The War in Heaven's third chapter.
    witness(
        "The War in Heaven",
        "Return each of them to the battlefield with a necrodermis counter on it.",
        "each of them",
        "them",
    );
    // Shower of Coals, Blessing of the Nephilim, Absolute Virtue, Stall for Time.
    for (text, partitive, oblique) in [
        (
            "to each of up to three targets",
            "each of up to three targets",
            "up to three targets",
        ),
        ("for each of its colors", "each of its colors", "its colors"),
        (
            "from each of your opponents",
            "each of your opponents",
            "your opponents",
        ),
        (
            "on each of those creatures",
            "each of those creatures",
            "those creatures",
        ),
    ] {
        assert_constituents(
            text,
            Category::PrepositionPhrase,
            &[
                (Category::NounPhrase, partitive),
                (Category::PrepositionPhrase, &format!("of {oblique}")),
                (Category::AccusativePhrase, oblique),
            ],
        );
    }
}

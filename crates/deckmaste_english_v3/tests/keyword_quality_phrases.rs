mod common;

use std::collections::BTreeSet;

use common::LEXICON;
use common::readings;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::Case;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

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
fn noun(owner: &str, number: Number) -> Reading {
    let mut head = word(owner);
    head.countability = Some(true);
    if let LexicalReading::Word(value) = &mut head.value {
        value.form = if number == Number::Plural { WordForm::Plural } else { WordForm::Singular };
        value.features.number = Some(number);
    }
    Reading::Noun { form: 0, head }
}
fn color_of_choice() -> Reading {
    let mut possessor = word("vocab:PossessiveDeterminerPronoun/Your");
    if let LexicalReading::Word(value) = &mut possessor.value {
        value.features = FeatureBundle {
            number: Some(Number::Singular),
            person: Some(Person::Second),
            case: Some(Case::Genitive),
            ..Default::default()
        };
    }
    Reading::DeterminedNounPhrase {
        form: 0,
        determiner: word("vocab:DefiniteMarker/The"),
        head: Box::new(Reading::PostmodifiedNominal {
            form: 0,
            head: Box::new(noun("lexeme:CommonNoun/Color", Number::Singular)),
            modifier: Box::new(Reading::PrepositionPhrase {
                form: 0,
                head: word("vocab:Preposition/Of"),
                complement: Box::new(Reading::CasePhrase {
                    form: 0,
                    category: Category::AccusativePhrase,
                    head: Box::new(Reading::PossessiveNounPhrase {
                        form: 0,
                        possessor,
                        head: Box::new(noun("lexeme:CommonNoun/Choice", Number::Singular)),
                    }),
                }),
            }),
        }),
    }
}
fn marked(quality: Reading) -> Reading {
    Reading::KeywordQualityPreposition {
        form: 0,
        head: word("vocab:Preposition/From"),
        quality: Box::new(quality),
    }
}
fn protection(left: Reading) -> Reading {
    let mut head = word("lexeme:keyword_ability/protection");
    if let LexicalReading::Word(value) = &mut head.value {
        value.capitalization = SurfaceCase::Initial;
    }
    Reading::QualityKeyword {
        form: 0,
        head,
        quality: Box::new(Reading::Coordination {
            form: 0,
            category: Category::KeywordQualityPreposition,
            left: Box::new(marked(left)),
            coordinator: word("vocab:Coordinator/Or"),
            right: Box::new(marked(Reading::NounPhraseKeywordQuality {
                form: 0,
                phrase: Box::new(color_of_choice()),
            })),
        }),
    }
}
fn exact(text: &str, expected: BTreeSet<Reading>) {
    assert_eq!(readings(text, Category::KeywordPhrase), expected);
    for value in expected {
        value.admit(&LEXICON).unwrap();
        assert_eq!(value.realize(&LEXICON).unwrap(), text);
        let mut words = vec![];
        value
            .visit_words(&mut |word| words.push(word.clone()))
            .unwrap();
        assert_eq!(words.len(), 10);
        assert_eq!(words[0].value, protection_head());
        assert_eq!(words[3].value, word("vocab:Coordinator/Or").value);
        assert_eq!(
            words[9].value,
            if let Reading::Noun { head, .. } = noun("lexeme:CommonNoun/Choice", Number::Singular) {
                head.value
            } else {
                unreachable!()
            }
        );
    }
}
fn protection_head() -> LexicalReading {
    let mut head = word("lexeme:keyword_ability/protection");
    if let LexicalReading::Word(value) = &mut head.value {
        value.capitalization = SurfaceCase::Initial;
    }
    head.value
}
#[test]
fn authentic_coordinated_qualities_preserve_determined_noun_phrase_structure() {
    // Jeweled Spirit, with both legitimate bare-plural constituent analyses
    // retained.
    let artifacts = noun("lexeme:type/artifact", Number::Plural);
    exact(
        "Protection from artifacts or from the color of your choice",
        BTreeSet::from([
            protection(Reading::NominalKeywordQuality {
                form: 0,
                phrase: Box::new(artifacts.clone()),
            }),
            protection(Reading::NounPhraseKeywordQuality {
                form: 0,
                phrase: Box::new(Reading::BarePlural {
                    form: 0,
                    head: Box::new(artifacts),
                }),
            }),
        ]),
    );
    // Giver of Runes.
    exact(
        "Protection from colorless or from the color of your choice",
        BTreeSet::from([protection(Reading::AdjectivalKeywordQuality {
            form: 0,
            phrase: Box::new(Reading::Adjective {
                form: 0,
                head: word("vocab:ColorWord/Colorless"),
            }),
        })]),
    );
}
#[test]
fn keyword_quality_rejects_targeting_including_mixed_coordinands() {
    for text in [
        "Protection from target creatures",
        "Protection from target creatures and artifacts",
        "Protection from artifacts and target creatures",
    ] {
        assert!(readings(text, Category::KeywordPhrase).is_empty(), "{text}");
    }
}

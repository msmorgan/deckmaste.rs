mod common;

use std::collections::BTreeSet;

use common::LEXICON;
use common::readings;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

fn invariant(owner: &str, capitalization: SurfaceCase) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form: WordForm::Invariant,
            features: FeatureBundle::default(),
            variant: 0,
            capitalization,
        }),
        frame: None,
        countability: None,
    }
}

fn noun(owner: &str) -> Reading {
    Reading::Noun {
        form: 0,
        head: Word {
            value: LexicalReading::Word(LexicalValue {
                lexeme: owner.into(),
                form: WordForm::Singular,
                features: FeatureBundle {
                    number: Some(Number::Singular),
                    ..Default::default()
                },
                variant: 0,
                capitalization: SurfaceCase::Declared,
            }),
            frame: None,
            countability: Some(true),
        },
    }
}

fn exact(text: &str, category: Category, expected: &Reading) {
    expected.admit(&LEXICON).unwrap();
    assert_eq!(expected.realize(&LEXICON).unwrap(), text);
    let actual = readings(text, category);
    assert_eq!(actual, BTreeSet::from([expected.clone()]));
    let mut expected_words = Vec::new();
    expected
        .visit_words(&mut |word| expected_words.push(word.clone()))
        .unwrap();
    let mut actual_words = Vec::new();
    actual
        .first()
        .unwrap()
        .visit_words(&mut |word| actual_words.push(word.clone()))
        .unwrap();
    assert_eq!(actual_words, expected_words);
}

#[test]
fn authentic_boundary_pp_keeps_its_bare_interval_structure() {
    // Giant Growth: "Target creature gets +3/+3 until end of turn."
    exact(
        "until end of turn",
        Category::PrepositionPhrase,
        &Reading::BareTemporalPreposition {
            form: 0,
            head: invariant("vocab:Preposition/Until", SurfaceCase::Declared),
            complement: Box::new(Reading::BareBoundaryNominal {
                form: 0,
                head: match noun("lexeme:CommonNoun/End") {
                    Reading::Noun { head, .. } => head,
                    _ => unreachable!(),
                },
                marker: invariant("vocab:Preposition/Of", SurfaceCase::Declared),
                complement: Box::new(Reading::BareBoundaryComplement {
                    form: 0,
                    phrase: Box::new(Reading::BareIntervalNominal {
                        form: 0,
                        head: match noun("lexeme:CommonNoun/Turn") {
                            Reading::Noun { head, .. } => head,
                            _ => unreachable!(),
                        },
                    }),
                }),
            }),
        },
    );
}
#[test]
fn authentic_temporal_and_manner_adjuncts_keep_their_noun_identity() {
    // Ashen-Skin Zubera: "...for each Zubera that died this turn."
    // Boldwyr Heavyweights: "Then each player who searched their library this
    // way shuffles."
    for (text, owner) in [
        ("this turn", "lexeme:CommonNoun/Turn"),
        ("this way", "lexeme:CommonNoun/Way"),
    ] {
        exact(
            text,
            Category::NominalAdjunctPhrase,
            &Reading::NominalAdjunctPhrase {
                form: 0,
                determiner: invariant("vocab:SingularDemonstrative/This", SurfaceCase::Declared),
                head: Box::new(noun(owner)),
            },
        );
    }
    for text in ["this creature", "each way", "this turns"] {
        assert!(
            readings(text, Category::NominalAdjunctPhrase).is_empty(),
            "{text}"
        );
    }
}
#[test]
fn authentic_preterite_host_preserves_temporal_adjunct_attachment() {
    // Ashen-Skin Zubera's relative contains the predicate "died this turn".
    use deckmaste_lexical::Finiteness;
    use deckmaste_lexical::Person;
    use deckmaste_lexical::Tense;
    let mut head = invariant("core-verb:Die", SurfaceCase::Declared);
    head.frame = Some(0);
    if let LexicalReading::Word(value) = &mut head.value {
        value.form = WordForm::Preterite;
        value.features = FeatureBundle {
            number: Some(Number::Singular),
            person: Some(Person::Third),
            tense: Some(Tense::Past),
            finiteness: Some(Finiteness::Finite),
            case: None,
        };
    }
    let expected = Reading::NominalAdjunctPredicate {
        category: Category::FinitePredicate,
        form: 0,
        head: Box::new(Reading::SelectedPredicate {
            category: Category::FinitePredicate,
            form: 0,
            head,
            complements: vec![],
        }),
        modifier: Box::new(Reading::NominalAdjunctPhrase {
            form: 0,
            determiner: invariant("vocab:SingularDemonstrative/This", SurfaceCase::Declared),
            head: Box::new(noun("lexeme:CommonNoun/Turn")),
        }),
    };
    expected.admit(&LEXICON).unwrap();
    assert_eq!(expected.realize(&LEXICON).unwrap(), "died this turn");
    // An isolated finite predicate retains all compatible person/number
    // bundles.
    let mut alternatives = BTreeSet::new();
    for number in [Number::Singular, Number::Plural] {
        for person in [Person::First, Person::Second, Person::Third] {
            let mut value = expected.clone();
            let Reading::NominalAdjunctPredicate { head, .. } = &mut value else {
                unreachable!()
            };
            let Reading::SelectedPredicate { head, .. } = head.as_mut() else {
                unreachable!()
            };
            let LexicalReading::Word(head) = &mut head.value else { unreachable!() };
            head.features.number = Some(number);
            head.features.person = Some(person);
            value.admit(&LEXICON).unwrap();
            assert_eq!(value.realize(&LEXICON).unwrap(), "died this turn");
            alternatives.insert(value);
        }
    }
    assert_eq!(
        readings("died this turn", Category::FinitePredicate),
        alternatives
    );
}
